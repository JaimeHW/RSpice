#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_363(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94210_e144371, assign94210_e144371_d_n0, assign94210_e144371_d_n2, assign94210_e144371_d_n4, assign94210_e144371_d_n5, assign94210_e144371_d_n6, assign94210_e144371_d_n7, assign94210_e144371_d_n8, assign94210_e144371_d_n9, assign94210_e144371_d_n10, assign94210_e144371_d_n11, assign94210_e144371_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2181 == 0.0)) {
        let assign94210_e144366: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94210_e144367: f64 = (assign94210_e144366).sqrt();
        let assign94210_e144369: f64 = (assign94210_e144367 * p.p432);
        (assign94210_e144369, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94210_e144367)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94210_e144367)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94210_e144371;
        locals.var_wjunc0_dn0 = assign94210_e144371_d_n0;
        locals.var_wjunc0_dn2 = assign94210_e144371_d_n2;
        locals.var_wjunc0_dn4 = assign94210_e144371_d_n4;
        locals.var_wjunc0_dn5 = assign94210_e144371_d_n5;
        locals.var_wjunc0_dn6 = assign94210_e144371_d_n6;
        locals.var_wjunc0_dn7 = assign94210_e144371_d_n7;
        locals.var_wjunc0_dn8 = assign94210_e144371_d_n8;
        locals.var_wjunc0_dn9 = assign94210_e144371_d_n9;
        locals.var_wjunc0_dn10 = assign94210_e144371_d_n10;
        locals.var_wjunc0_dn11 = assign94210_e144371_d_n11;
        locals.var_wjunc0_dn14 = assign94210_e144371_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign94220_e144387, assign94220_e144387_d_n0, assign94220_e144387_d_n2, assign94220_e144387_d_n4, assign94220_e144387_d_n5, assign94220_e144387_d_n6, assign94220_e144387_d_n7, assign94220_e144387_d_n8, assign94220_e144387_d_n9, assign94220_e144387_d_n10, assign94220_e144387_d_n11, assign94220_e144387_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2181 == 0.0)) {
        let assign94220_e144385: f64 = (p.p334 - locals.var_wjunc0);
        (assign94220_e144385, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94220_e144387;
        locals.var_t2_dn0 = assign94220_e144387_d_n0;
        locals.var_t2_dn2 = assign94220_e144387_d_n2;
        locals.var_t2_dn4 = assign94220_e144387_d_n4;
        locals.var_t2_dn5 = assign94220_e144387_d_n5;
        locals.var_t2_dn6 = assign94220_e144387_d_n6;
        locals.var_t2_dn7 = assign94220_e144387_d_n7;
        locals.var_t2_dn8 = assign94220_e144387_d_n8;
        locals.var_t2_dn9 = assign94220_e144387_d_n9;
        locals.var_t2_dn10 = assign94220_e144387_d_n10;
        locals.var_t2_dn11 = assign94220_e144387_d_n11;
        locals.var_t2_dn14 = assign94220_e144387_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94230_e144411, assign94230_e144411_d_n0, assign94230_e144411_d_n2, assign94230_e144411_d_n4, assign94230_e144411_d_n5, assign94230_e144411_d_n6, assign94230_e144411_d_n7, assign94230_e144411_d_n8, assign94230_e144411_d_n9, assign94230_e144411_d_n10, assign94230_e144411_d_n11, assign94230_e144411_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94230_e144398: f64 = (locals.var_t2 * locals.var_t2);
        let assign94230_e144402: f64 = (p.p334 * 0.01);
        let assign94230_e144403: f64 = (4.0 * assign94230_e144402);
        let assign94230_e144406: f64 = (p.p334 * 0.01);
        let assign94230_e144407: f64 = (assign94230_e144403 * assign94230_e144406);
        let assign94230_e144408: f64 = (assign94230_e144398 + assign94230_e144407);
        let assign94230_e144409: f64 = (assign94230_e144408).sqrt();
        (assign94230_e144409, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign94230_e144409)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign94230_e144409)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94230_e144411;
        locals.var_tmf2_dn0 = assign94230_e144411_d_n0;
        locals.var_tmf2_dn2 = assign94230_e144411_d_n2;
        locals.var_tmf2_dn4 = assign94230_e144411_d_n4;
        locals.var_tmf2_dn5 = assign94230_e144411_d_n5;
        locals.var_tmf2_dn6 = assign94230_e144411_d_n6;
        locals.var_tmf2_dn7 = assign94230_e144411_d_n7;
        locals.var_tmf2_dn8 = assign94230_e144411_d_n8;
        locals.var_tmf2_dn9 = assign94230_e144411_d_n9;
        locals.var_tmf2_dn10 = assign94230_e144411_d_n10;
        locals.var_tmf2_dn11 = assign94230_e144411_d_n11;
        locals.var_tmf2_dn14 = assign94230_e144411_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94240_e144428, assign94240_e144428_d_n0, assign94240_e144428_d_n2, assign94240_e144428_d_n4, assign94240_e144428_d_n5, assign94240_e144428_d_n6, assign94240_e144428_d_n7, assign94240_e144428_d_n8, assign94240_e144428_d_n9, assign94240_e144428_d_n10, assign94240_e144428_d_n11, assign94240_e144428_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94240_e144424: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94240_e144425: f64 = (1.0 + assign94240_e144424);
        let assign94240_e144426: f64 = (0.5 * assign94240_e144425);
        (assign94240_e144426, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94240_e144428;
        locals.var_t9_dn0 = assign94240_e144428_d_n0;
        locals.var_t9_dn2 = assign94240_e144428_d_n2;
        locals.var_t9_dn4 = assign94240_e144428_d_n4;
        locals.var_t9_dn5 = assign94240_e144428_d_n5;
        locals.var_t9_dn6 = assign94240_e144428_d_n6;
        locals.var_t9_dn7 = assign94240_e144428_d_n7;
        locals.var_t9_dn8 = assign94240_e144428_d_n8;
        locals.var_t9_dn9 = assign94240_e144428_d_n9;
        locals.var_t9_dn10 = assign94240_e144428_d_n10;
        locals.var_t9_dn11 = assign94240_e144428_d_n11;
        locals.var_t9_dn14 = assign94240_e144428_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94250_e144443, assign94250_e144443_d_n0, assign94250_e144443_d_n2, assign94250_e144443_d_n4, assign94250_e144443_d_n5, assign94250_e144443_d_n6, assign94250_e144443_d_n7, assign94250_e144443_d_n8, assign94250_e144443_d_n9, assign94250_e144443_d_n10, assign94250_e144443_d_n11, assign94250_e144443_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94250_e144440: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94250_e144441: f64 = (0.5 * assign94250_e144440);
        (assign94250_e144441, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94250_e144443;
        locals.var_t2_dn0 = assign94250_e144443_d_n0;
        locals.var_t2_dn2 = assign94250_e144443_d_n2;
        locals.var_t2_dn4 = assign94250_e144443_d_n4;
        locals.var_t2_dn5 = assign94250_e144443_d_n5;
        locals.var_t2_dn6 = assign94250_e144443_d_n6;
        locals.var_t2_dn7 = assign94250_e144443_d_n7;
        locals.var_t2_dn8 = assign94250_e144443_d_n8;
        locals.var_t2_dn9 = assign94250_e144443_d_n9;
        locals.var_t2_dn10 = assign94250_e144443_d_n10;
        locals.var_t2_dn11 = assign94250_e144443_d_n11;
        locals.var_t2_dn14 = assign94250_e144443_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94260_e144446: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2183 = assign94260_e144446;
        locals.var_guard2183_rv = 0.0;

        let (assign94270_e144459, assign94270_e144459_d_n0, assign94270_e144459_d_n2, assign94270_e144459_d_n4, assign94270_e144459_d_n5, assign94270_e144459_d_n6, assign94270_e144459_d_n7, assign94270_e144459_d_n8, assign94270_e144459_d_n9, assign94270_e144459_d_n10, assign94270_e144459_d_n11, assign94270_e144459_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94270_e144459;
        locals.var_t2_dn0 = assign94270_e144459_d_n0;
        locals.var_t2_dn2 = assign94270_e144459_d_n2;
        locals.var_t2_dn4 = assign94270_e144459_d_n4;
        locals.var_t2_dn5 = assign94270_e144459_d_n5;
        locals.var_t2_dn6 = assign94270_e144459_d_n6;
        locals.var_t2_dn7 = assign94270_e144459_d_n7;
        locals.var_t2_dn8 = assign94270_e144459_d_n8;
        locals.var_t2_dn9 = assign94270_e144459_d_n9;
        locals.var_t2_dn10 = assign94270_e144459_d_n10;
        locals.var_t2_dn11 = assign94270_e144459_d_n11;
        locals.var_t2_dn14 = assign94270_e144459_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94280_e144472, assign94280_e144472_d_n0, assign94280_e144472_d_n2, assign94280_e144472_d_n4, assign94280_e144472_d_n5, assign94280_e144472_d_n6, assign94280_e144472_d_n7, assign94280_e144472_d_n8, assign94280_e144472_d_n9, assign94280_e144472_d_n10, assign94280_e144472_d_n11, assign94280_e144472_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94280_e144472;
        locals.var_t9_dn0 = assign94280_e144472_d_n0;
        locals.var_t9_dn2 = assign94280_e144472_d_n2;
        locals.var_t9_dn4 = assign94280_e144472_d_n4;
        locals.var_t9_dn5 = assign94280_e144472_d_n5;
        locals.var_t9_dn6 = assign94280_e144472_d_n6;
        locals.var_t9_dn7 = assign94280_e144472_d_n7;
        locals.var_t9_dn8 = assign94280_e144472_d_n8;
        locals.var_t9_dn9 = assign94280_e144472_d_n9;
        locals.var_t9_dn10 = assign94280_e144472_d_n10;
        locals.var_t9_dn11 = assign94280_e144472_d_n11;
        locals.var_t9_dn14 = assign94280_e144472_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94290_e144483, assign94290_e144483_d_n0, assign94290_e144483_d_n2, assign94290_e144483_d_n4, assign94290_e144483_d_n5, assign94290_e144483_d_n6, assign94290_e144483_d_n7, assign94290_e144483_d_n8, assign94290_e144483_d_n9, assign94290_e144483_d_n10, assign94290_e144483_d_n11, assign94290_e144483_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign94290_e144483;
        locals.var_ddriftldc_dn0 = assign94290_e144483_d_n0;
        locals.var_ddriftldc_dn2 = assign94290_e144483_d_n2;
        locals.var_ddriftldc_dn4 = assign94290_e144483_d_n4;
        locals.var_ddriftldc_dn5 = assign94290_e144483_d_n5;
        locals.var_ddriftldc_dn6 = assign94290_e144483_d_n6;
        locals.var_ddriftldc_dn7 = assign94290_e144483_d_n7;
        locals.var_ddriftldc_dn8 = assign94290_e144483_d_n8;
        locals.var_ddriftldc_dn9 = assign94290_e144483_d_n9;
        locals.var_ddriftldc_dn10 = assign94290_e144483_d_n10;
        locals.var_ddriftldc_dn11 = assign94290_e144483_d_n11;
        locals.var_ddriftldc_dn14 = assign94290_e144483_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94300_e144502, assign94300_e144502_d_n0, assign94300_e144502_d_n2, assign94300_e144502_d_n4, assign94300_e144502_d_n5, assign94300_e144502_d_n6, assign94300_e144502_d_n7, assign94300_e144502_d_n8, assign94300_e144502_d_n9, assign94300_e144502_d_n10, assign94300_e144502_d_n11, assign94300_e144502_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94300_e144494: f64 = (locals.var_q_nsubld__blk2117 * locals.var_ddriftldc);
        let assign94300_e144496: f64 = (assign94300_e144494 * locals.var_ddriftldc);
        let assign94300_e144498: f64 = (assign94300_e144496 / 2.0);
        let assign94300_e144500: f64 = (assign94300_e144498 / 1.034943e-10);
        (assign94300_e144500, (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign94300_e144494 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign94300_e144502;
        locals.var_dphi_sb_dn0 = assign94300_e144502_d_n0;
        locals.var_dphi_sb_dn2 = assign94300_e144502_d_n2;
        locals.var_dphi_sb_dn4 = assign94300_e144502_d_n4;
        locals.var_dphi_sb_dn5 = assign94300_e144502_d_n5;
        locals.var_dphi_sb_dn6 = assign94300_e144502_d_n6;
        locals.var_dphi_sb_dn7 = assign94300_e144502_d_n7;
        locals.var_dphi_sb_dn8 = assign94300_e144502_d_n8;
        locals.var_dphi_sb_dn9 = assign94300_e144502_d_n9;
        locals.var_dphi_sb_dn10 = assign94300_e144502_d_n10;
        locals.var_dphi_sb_dn11 = assign94300_e144502_d_n11;
        locals.var_dphi_sb_dn14 = assign94300_e144502_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94310_e144518, assign94310_e144518_d_n0, assign94310_e144518_d_n2, assign94310_e144518_d_n4, assign94310_e144518_d_n5, assign94310_e144518_d_n6, assign94310_e144518_d_n7, assign94310_e144518_d_n8, assign94310_e144518_d_n9, assign94310_e144518_d_n10, assign94310_e144518_d_n11, assign94310_e144518_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94310_e144513: f64 = (2.0 * locals.var_beta);
        let assign94310_e144515: f64 = (assign94310_e144513 * locals.var_dphi_sb);
        let assign94310_e144516: f64 = (assign94310_e144515).sqrt();
        (assign94310_e144516, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn0)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn2)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn4)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn5)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn6)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn7)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn8)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn9)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn10)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn11)) / (2.0 * assign94310_e144516)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign94310_e144513 * locals.var_dphi_sb_dn14)) / (2.0 * assign94310_e144516)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94310_e144518;
        locals.var_t0_dn0 = assign94310_e144518_d_n0;
        locals.var_t0_dn2 = assign94310_e144518_d_n2;
        locals.var_t0_dn4 = assign94310_e144518_d_n4;
        locals.var_t0_dn5 = assign94310_e144518_d_n5;
        locals.var_t0_dn6 = assign94310_e144518_d_n6;
        locals.var_t0_dn7 = assign94310_e144518_d_n7;
        locals.var_t0_dn8 = assign94310_e144518_d_n8;
        locals.var_t0_dn9 = assign94310_e144518_d_n9;
        locals.var_t0_dn10 = assign94310_e144518_d_n10;
        locals.var_t0_dn11 = assign94310_e144518_d_n11;
        locals.var_t0_dn14 = assign94310_e144518_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94320_e144536, assign94320_e144536_d_n0, assign94320_e144536_d_n2, assign94320_e144536_d_n4, assign94320_e144536_d_n5, assign94320_e144536_d_n6, assign94320_e144536_d_n7, assign94320_e144536_d_n8, assign94320_e144536_d_n9, assign94320_e144536_d_n10, assign94320_e144536_d_n11, assign94320_e144536_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94320_e144528: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94320_e144530: f64 = (-locals.var_t0);
        let assign94320_e144531: f64 = { let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94320_e144532: f64 = (assign94320_e144528 + assign94320_e144531);
        let assign94320_e144534: f64 = (assign94320_e144532 / 2.0);
        (assign94320_e144534, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign94320_e144530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94320_e144536;
        locals.var_t1_dn0 = assign94320_e144536_d_n0;
        locals.var_t1_dn2 = assign94320_e144536_d_n2;
        locals.var_t1_dn4 = assign94320_e144536_d_n4;
        locals.var_t1_dn5 = assign94320_e144536_d_n5;
        locals.var_t1_dn6 = assign94320_e144536_d_n6;
        locals.var_t1_dn7 = assign94320_e144536_d_n7;
        locals.var_t1_dn8 = assign94320_e144536_d_n8;
        locals.var_t1_dn9 = assign94320_e144536_d_n9;
        locals.var_t1_dn10 = assign94320_e144536_d_n10;
        locals.var_t1_dn11 = assign94320_e144536_d_n11;
        locals.var_t1_dn14 = assign94320_e144536_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94330_e144550, assign94330_e144550_d_n0, assign94330_e144550_d_n2, assign94330_e144550_d_n4, assign94330_e144550_d_n5, assign94330_e144550_d_n6, assign94330_e144550_d_n7, assign94330_e144550_d_n8, assign94330_e144550_d_n9, assign94330_e144550_d_n10, assign94330_e144550_d_n11, assign94330_e144550_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94330_e144546: f64 = (locals.var_t1).ln();
        let assign94330_e144548: f64 = (assign94330_e144546 / locals.var_dphi_sb);
        (assign94330_e144548, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign94330_e144546 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign94330_e144550;
        locals.var_c_sb_dn0 = assign94330_e144550_d_n0;
        locals.var_c_sb_dn2 = assign94330_e144550_d_n2;
        locals.var_c_sb_dn4 = assign94330_e144550_d_n4;
        locals.var_c_sb_dn5 = assign94330_e144550_d_n5;
        locals.var_c_sb_dn6 = assign94330_e144550_d_n6;
        locals.var_c_sb_dn7 = assign94330_e144550_d_n7;
        locals.var_c_sb_dn8 = assign94330_e144550_d_n8;
        locals.var_c_sb_dn9 = assign94330_e144550_d_n9;
        locals.var_c_sb_dn10 = assign94330_e144550_d_n10;
        locals.var_c_sb_dn11 = assign94330_e144550_d_n11;
        locals.var_c_sb_dn14 = assign94330_e144550_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign94340_e144561,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94340_e144561;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_364(
        locals: &mut StampLocals,
    ) {
        let mut assign94350_loop_guard: usize = 0;
        while {
            let assign94350_cond_e144573: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94350_cond_e144575: f64 = if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_lp_s0 <= assign94350_cond_e144573)) { 1.0 } else { 0.0 };
            assign94350_cond_e144575 != 0.0
        } {
            assign94350_loop_guard += 1;
            assert!(assign94350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94350_body3_e144617, assign94350_body3_e144617_d_n0, assign94350_body3_e144617_d_n2, assign94350_body3_e144617_d_n4, assign94350_body3_e144617_d_n5, assign94350_body3_e144617_d_n6, assign94350_body3_e144617_d_n7, assign94350_body3_e144617_d_n8, assign94350_body3_e144617_d_n9, assign94350_body3_e144617_d_n10, assign94350_body3_e144617_d_n11, assign94350_body3_e144617_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body3_e144615: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94350_body3_e144615, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign94350_body3_e144617;
            locals.var_ps0ld_vxb_dn0 = assign94350_body3_e144617_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94350_body3_e144617_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94350_body3_e144617_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94350_body3_e144617_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94350_body3_e144617_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94350_body3_e144617_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94350_body3_e144617_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94350_body3_e144617_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94350_body3_e144617_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign94350_body3_e144617_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign94350_body3_e144617_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94350_body4_e144630, assign94350_body4_e144630_d_n0, assign94350_body4_e144630_d_n2, assign94350_body4_e144630_d_n4, assign94350_body4_e144630_d_n5, assign94350_body4_e144630_d_n6, assign94350_body4_e144630_d_n7, assign94350_body4_e144630_d_n8, assign94350_body4_e144630_d_n9, assign94350_body4_e144630_d_n10, assign94350_body4_e144630_d_n11, assign94350_body4_e144630_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body4_e144628: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94350_body4_e144628, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign94350_body4_e144630;
            locals.var_chi_dn0 = assign94350_body4_e144630_d_n0;
            locals.var_chi_dn2 = assign94350_body4_e144630_d_n2;
            locals.var_chi_dn4 = assign94350_body4_e144630_d_n4;
            locals.var_chi_dn5 = assign94350_body4_e144630_d_n5;
            locals.var_chi_dn6 = assign94350_body4_e144630_d_n6;
            locals.var_chi_dn7 = assign94350_body4_e144630_d_n7;
            locals.var_chi_dn8 = assign94350_body4_e144630_d_n8;
            locals.var_chi_dn9 = assign94350_body4_e144630_d_n9;
            locals.var_chi_dn10 = assign94350_body4_e144630_d_n10;
            locals.var_chi_dn11 = assign94350_body4_e144630_d_n11;
            locals.var_chi_dn14 = assign94350_body4_e144630_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign94350_body5_e144645, assign94350_body5_e144645_d_n0, assign94350_body5_e144645_d_n2, assign94350_body5_e144645_d_n4, assign94350_body5_e144645_d_n5, assign94350_body5_e144645_d_n6, assign94350_body5_e144645_d_n7, assign94350_body5_e144645_d_n8, assign94350_body5_e144645_d_n9, assign94350_body5_e144645_d_n10, assign94350_body5_e144645_d_n11, assign94350_body5_e144645_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body5_e144642: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94350_body5_e144643: f64 = (locals.var_c_sb * assign94350_body5_e144642);
        (assign94350_body5_e144643, ((locals.var_c_sb_dn0 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign94350_body5_e144642) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign94350_body5_e144645;
            locals.var_ty_dn0 = assign94350_body5_e144645_d_n0;
            locals.var_ty_dn2 = assign94350_body5_e144645_d_n2;
            locals.var_ty_dn4 = assign94350_body5_e144645_d_n4;
            locals.var_ty_dn5 = assign94350_body5_e144645_d_n5;
            locals.var_ty_dn6 = assign94350_body5_e144645_d_n6;
            locals.var_ty_dn7 = assign94350_body5_e144645_d_n7;
            locals.var_ty_dn8 = assign94350_body5_e144645_d_n8;
            locals.var_ty_dn9 = assign94350_body5_e144645_d_n9;
            locals.var_ty_dn10 = assign94350_body5_e144645_d_n10;
            locals.var_ty_dn11 = assign94350_body5_e144645_d_n11;
            locals.var_ty_dn14 = assign94350_body5_e144645_d_n14;
            locals.var_ty_rv = 0.0;
            let assign94350_body6_e144648: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2185 = assign94350_body6_e144648;
            locals.var_guard2185_rv = 0.0;
            let (assign94350_body7_e144662, assign94350_body7_e144662_d_n0, assign94350_body7_e144662_d_n2, assign94350_body7_e144662_d_n4, assign94350_body7_e144662_d_n5, assign94350_body7_e144662_d_n6, assign94350_body7_e144662_d_n7, assign94350_body7_e144662_d_n8, assign94350_body7_e144662_d_n9, assign94350_body7_e144662_d_n10, assign94350_body7_e144662_d_n11, assign94350_body7_e144662_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94350_body7_e144660: f64 = (locals.var_ty).exp();
        (assign94350_body7_e144660, (assign94350_body7_e144660 * locals.var_ty_dn0), (assign94350_body7_e144660 * locals.var_ty_dn2), (assign94350_body7_e144660 * locals.var_ty_dn4), (assign94350_body7_e144660 * locals.var_ty_dn5), (assign94350_body7_e144660 * locals.var_ty_dn6), (assign94350_body7_e144660 * locals.var_ty_dn7), (assign94350_body7_e144660 * locals.var_ty_dn8), (assign94350_body7_e144660 * locals.var_ty_dn9), (assign94350_body7_e144660 * locals.var_ty_dn10), (assign94350_body7_e144660 * locals.var_ty_dn11), (assign94350_body7_e144660 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94350_body7_e144662;
            locals.var_t1_dn0 = assign94350_body7_e144662_d_n0;
            locals.var_t1_dn2 = assign94350_body7_e144662_d_n2;
            locals.var_t1_dn4 = assign94350_body7_e144662_d_n4;
            locals.var_t1_dn5 = assign94350_body7_e144662_d_n5;
            locals.var_t1_dn6 = assign94350_body7_e144662_d_n6;
            locals.var_t1_dn7 = assign94350_body7_e144662_d_n7;
            locals.var_t1_dn8 = assign94350_body7_e144662_d_n8;
            locals.var_t1_dn9 = assign94350_body7_e144662_d_n9;
            locals.var_t1_dn10 = assign94350_body7_e144662_d_n10;
            locals.var_t1_dn11 = assign94350_body7_e144662_d_n11;
            locals.var_t1_dn14 = assign94350_body7_e144662_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94350_body8_e144679, assign94350_body8_e144679_d_n0, assign94350_body8_e144679_d_n2, assign94350_body8_e144679_d_n4, assign94350_body8_e144679_d_n5, assign94350_body8_e144679_d_n6, assign94350_body8_e144679_d_n7, assign94350_body8_e144679_d_n8, assign94350_body8_e144679_d_n9, assign94350_body8_e144679_d_n10, assign94350_body8_e144679_d_n11, assign94350_body8_e144679_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94350_body8_e144674: f64 = (-locals.var_c_sb);
        let assign94350_body8_e144676: f64 = (assign94350_body8_e144674 * locals.var_dphi_sb);
        let assign94350_body8_e144677: f64 = (assign94350_body8_e144676).exp();
        (assign94350_body8_e144677, (assign94350_body8_e144677 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn0))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn2))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn4))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn5))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn6))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn7))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn8))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn9))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn10))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn11))), (assign94350_body8_e144677 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign94350_body8_e144674 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94350_body8_e144679;
            locals.var_t0_dn0 = assign94350_body8_e144679_d_n0;
            locals.var_t0_dn2 = assign94350_body8_e144679_d_n2;
            locals.var_t0_dn4 = assign94350_body8_e144679_d_n4;
            locals.var_t0_dn5 = assign94350_body8_e144679_d_n5;
            locals.var_t0_dn6 = assign94350_body8_e144679_d_n6;
            locals.var_t0_dn7 = assign94350_body8_e144679_d_n7;
            locals.var_t0_dn8 = assign94350_body8_e144679_d_n8;
            locals.var_t0_dn9 = assign94350_body8_e144679_d_n9;
            locals.var_t0_dn10 = assign94350_body8_e144679_d_n10;
            locals.var_t0_dn11 = assign94350_body8_e144679_d_n11;
            locals.var_t0_dn14 = assign94350_body8_e144679_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94350_body9_e144694, assign94350_body9_e144694_d_n0, assign94350_body9_e144694_d_n2, assign94350_body9_e144694_d_n4, assign94350_body9_e144694_d_n5, assign94350_body9_e144694_d_n6, assign94350_body9_e144694_d_n7, assign94350_body9_e144694_d_n8, assign94350_body9_e144694_d_n9, assign94350_body9_e144694_d_n10, assign94350_body9_e144694_d_n11, assign94350_body9_e144694_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94350_body9_e144692: f64 = (locals.var_t1 - locals.var_t0);
        (assign94350_body9_e144692, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94350_body9_e144694;
            locals.var_t2_dn0 = assign94350_body9_e144694_d_n0;
            locals.var_t2_dn2 = assign94350_body9_e144694_d_n2;
            locals.var_t2_dn4 = assign94350_body9_e144694_d_n4;
            locals.var_t2_dn5 = assign94350_body9_e144694_d_n5;
            locals.var_t2_dn6 = assign94350_body9_e144694_d_n6;
            locals.var_t2_dn7 = assign94350_body9_e144694_d_n7;
            locals.var_t2_dn8 = assign94350_body9_e144694_d_n8;
            locals.var_t2_dn9 = assign94350_body9_e144694_d_n9;
            locals.var_t2_dn10 = assign94350_body9_e144694_d_n10;
            locals.var_t2_dn11 = assign94350_body9_e144694_d_n11;
            locals.var_t2_dn14 = assign94350_body9_e144694_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94350_body10_e144712, assign94350_body10_e144712_d_n0, assign94350_body10_e144712_d_n2, assign94350_body10_e144712_d_n4, assign94350_body10_e144712_d_n5, assign94350_body10_e144712_d_n6, assign94350_body10_e144712_d_n7, assign94350_body10_e144712_d_n8, assign94350_body10_e144712_d_n9, assign94350_body10_e144712_d_n10, assign94350_body10_e144712_d_n11, assign94350_body10_e144712_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94350_body10_e144707: f64 = (1.0 + locals.var_t2);
        let assign94350_body10_e144708: f64 = (assign94350_body10_e144707).ln();
        let assign94350_body10_e144710: f64 = (assign94350_body10_e144708 / locals.var_c_sb);
        (assign94350_body10_e144710, ((((locals.var_t2_dn0 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign94350_body10_e144707) * locals.var_c_sb) - (assign94350_body10_e144708 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94350_body10_e144712;
            locals.var_phi_b_dn0 = assign94350_body10_e144712_d_n0;
            locals.var_phi_b_dn2 = assign94350_body10_e144712_d_n2;
            locals.var_phi_b_dn4 = assign94350_body10_e144712_d_n4;
            locals.var_phi_b_dn5 = assign94350_body10_e144712_d_n5;
            locals.var_phi_b_dn6 = assign94350_body10_e144712_d_n6;
            locals.var_phi_b_dn7 = assign94350_body10_e144712_d_n7;
            locals.var_phi_b_dn8 = assign94350_body10_e144712_d_n8;
            locals.var_phi_b_dn9 = assign94350_body10_e144712_d_n9;
            locals.var_phi_b_dn10 = assign94350_body10_e144712_d_n10;
            locals.var_phi_b_dn11 = assign94350_body10_e144712_d_n11;
            locals.var_phi_b_dn14 = assign94350_body10_e144712_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94350_body11_e144729, assign94350_body11_e144729_d_n0, assign94350_body11_e144729_d_n2, assign94350_body11_e144729_d_n4, assign94350_body11_e144729_d_n5, assign94350_body11_e144729_d_n6, assign94350_body11_e144729_d_n7, assign94350_body11_e144729_d_n8, assign94350_body11_e144729_d_n9, assign94350_body11_e144729_d_n10, assign94350_body11_e144729_d_n11, assign94350_body11_e144729_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94350_body11_e144726: f64 = (1.0 + locals.var_t2);
        let assign94350_body11_e144727: f64 = (locals.var_t1 / assign94350_body11_e144726);
        (assign94350_body11_e144727, (((locals.var_t1_dn0 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn2 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn4 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn5 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn6 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn7 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn8 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn9 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn10 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn11 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn11)) / (assign94350_body11_e144726 * assign94350_body11_e144726)), (((locals.var_t1_dn14 * assign94350_body11_e144726) - (locals.var_t1 * locals.var_t2_dn14)) / (assign94350_body11_e144726 * assign94350_body11_e144726)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94350_body11_e144729;
            locals.var_phi_b_dpss_dn0 = assign94350_body11_e144729_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94350_body11_e144729_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94350_body11_e144729_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94350_body11_e144729_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94350_body11_e144729_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94350_body11_e144729_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94350_body11_e144729_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94350_body11_e144729_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94350_body11_e144729_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94350_body11_e144729_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94350_body11_e144729_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94350_body13_e144761, assign94350_body13_e144761_d_n0, assign94350_body13_e144761_d_n2, assign94350_body13_e144761_d_n4, assign94350_body13_e144761_d_n5, assign94350_body13_e144761_d_n6, assign94350_body13_e144761_d_n7, assign94350_body13_e144761_d_n8, assign94350_body13_e144761_d_n9, assign94350_body13_e144761_d_n10, assign94350_body13_e144761_d_n11, assign94350_body13_e144761_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 == 0.0)) {
        let assign94350_body13_e144759: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94350_body13_e144759, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94350_body13_e144761;
            locals.var_phi_b_dn0 = assign94350_body13_e144761_d_n0;
            locals.var_phi_b_dn2 = assign94350_body13_e144761_d_n2;
            locals.var_phi_b_dn4 = assign94350_body13_e144761_d_n4;
            locals.var_phi_b_dn5 = assign94350_body13_e144761_d_n5;
            locals.var_phi_b_dn6 = assign94350_body13_e144761_d_n6;
            locals.var_phi_b_dn7 = assign94350_body13_e144761_d_n7;
            locals.var_phi_b_dn8 = assign94350_body13_e144761_d_n8;
            locals.var_phi_b_dn9 = assign94350_body13_e144761_d_n9;
            locals.var_phi_b_dn10 = assign94350_body13_e144761_d_n10;
            locals.var_phi_b_dn11 = assign94350_body13_e144761_d_n11;
            locals.var_phi_b_dn14 = assign94350_body13_e144761_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94350_body14_e144775, assign94350_body14_e144775_d_n0, assign94350_body14_e144775_d_n2, assign94350_body14_e144775_d_n4, assign94350_body14_e144775_d_n5, assign94350_body14_e144775_d_n6, assign94350_body14_e144775_d_n7, assign94350_body14_e144775_d_n8, assign94350_body14_e144775_d_n9, assign94350_body14_e144775_d_n10, assign94350_body14_e144775_d_n11, assign94350_body14_e144775_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2185 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94350_body14_e144775;
            locals.var_phi_b_dpss_dn0 = assign94350_body14_e144775_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94350_body14_e144775_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94350_body14_e144775_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94350_body14_e144775_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94350_body14_e144775_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94350_body14_e144775_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94350_body14_e144775_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94350_body14_e144775_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94350_body14_e144775_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94350_body14_e144775_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94350_body14_e144775_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94350_body15_e144788, assign94350_body15_e144788_d_n0, assign94350_body15_e144788_d_n2, assign94350_body15_e144788_d_n4, assign94350_body15_e144788_d_n5, assign94350_body15_e144788_d_n6, assign94350_body15_e144788_d_n7, assign94350_body15_e144788_d_n8, assign94350_body15_e144788_d_n9, assign94350_body15_e144788_d_n10, assign94350_body15_e144788_d_n11, assign94350_body15_e144788_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body15_e144786: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94350_body15_e144786, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign94350_body15_e144788;
            locals.var_chib_dn0 = assign94350_body15_e144788_d_n0;
            locals.var_chib_dn2 = assign94350_body15_e144788_d_n2;
            locals.var_chib_dn4 = assign94350_body15_e144788_d_n4;
            locals.var_chib_dn5 = assign94350_body15_e144788_d_n5;
            locals.var_chib_dn6 = assign94350_body15_e144788_d_n6;
            locals.var_chib_dn7 = assign94350_body15_e144788_d_n7;
            locals.var_chib_dn8 = assign94350_body15_e144788_d_n8;
            locals.var_chib_dn9 = assign94350_body15_e144788_d_n9;
            locals.var_chib_dn10 = assign94350_body15_e144788_d_n10;
            locals.var_chib_dn11 = assign94350_body15_e144788_d_n11;
            locals.var_chib_dn14 = assign94350_body15_e144788_d_n14;
            locals.var_chib_rv = 0.0;
            let assign94350_body16_e144791: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2186 = assign94350_body16_e144791;
            locals.var_guard2186_rv = 0.0;
            let (assign94350_body18_e144820, assign94350_body18_e144820_d_n0, assign94350_body18_e144820_d_n2, assign94350_body18_e144820_d_n4, assign94350_body18_e144820_d_n5, assign94350_body18_e144820_d_n6, assign94350_body18_e144820_d_n7, assign94350_body18_e144820_d_n8, assign94350_body18_e144820_d_n9, assign94350_body18_e144820_d_n10, assign94350_body18_e144820_d_n11, assign94350_body18_e144820_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 != 0.0)) {
        let assign94350_body18_e144818: f64 = (-0.7071067811865475);
        (assign94350_body18_e144818, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94350_body18_e144820;
            locals.var_t0_dn0 = assign94350_body18_e144820_d_n0;
            locals.var_t0_dn2 = assign94350_body18_e144820_d_n2;
            locals.var_t0_dn4 = assign94350_body18_e144820_d_n4;
            locals.var_t0_dn5 = assign94350_body18_e144820_d_n5;
            locals.var_t0_dn6 = assign94350_body18_e144820_d_n6;
            locals.var_t0_dn7 = assign94350_body18_e144820_d_n7;
            locals.var_t0_dn8 = assign94350_body18_e144820_d_n8;
            locals.var_t0_dn9 = assign94350_body18_e144820_d_n9;
            locals.var_t0_dn10 = assign94350_body18_e144820_d_n10;
            locals.var_t0_dn11 = assign94350_body18_e144820_d_n11;
            locals.var_t0_dn14 = assign94350_body18_e144820_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94350_body19_e144835, assign94350_body19_e144835_d_n0, assign94350_body19_e144835_d_n2, assign94350_body19_e144835_d_n4, assign94350_body19_e144835_d_n5, assign94350_body19_e144835_d_n6, assign94350_body19_e144835_d_n7, assign94350_body19_e144835_d_n8, assign94350_body19_e144835_d_n9, assign94350_body19_e144835_d_n10, assign94350_body19_e144835_d_n11, assign94350_body19_e144835_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 != 0.0)) {
        let assign94350_body19_e144833: f64 = (locals.var_chi * locals.var_t0);
        (assign94350_body19_e144833, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94350_body19_e144835;
            locals.var_fb_dn0 = assign94350_body19_e144835_d_n0;
            locals.var_fb_dn2 = assign94350_body19_e144835_d_n2;
            locals.var_fb_dn4 = assign94350_body19_e144835_d_n4;
            locals.var_fb_dn5 = assign94350_body19_e144835_d_n5;
            locals.var_fb_dn6 = assign94350_body19_e144835_d_n6;
            locals.var_fb_dn7 = assign94350_body19_e144835_d_n7;
            locals.var_fb_dn8 = assign94350_body19_e144835_d_n8;
            locals.var_fb_dn9 = assign94350_body19_e144835_d_n9;
            locals.var_fb_dn10 = assign94350_body19_e144835_d_n10;
            locals.var_fb_dn11 = assign94350_body19_e144835_d_n11;
            locals.var_fb_dn14 = assign94350_body19_e144835_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94350_body20_e144850, assign94350_body20_e144850_d_n0, assign94350_body20_e144850_d_n2, assign94350_body20_e144850_d_n4, assign94350_body20_e144850_d_n5, assign94350_body20_e144850_d_n6, assign94350_body20_e144850_d_n7, assign94350_body20_e144850_d_n8, assign94350_body20_e144850_d_n9, assign94350_body20_e144850_d_n10, assign94350_body20_e144850_d_n11, assign94350_body20_e144850_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 != 0.0)) {
        let assign94350_body20_e144848: f64 = (locals.var_beta * locals.var_t0);
        (assign94350_body20_e144848, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94350_body20_e144850;
            locals.var_fb_dpss_dn0 = assign94350_body20_e144850_d_n0;
            locals.var_fb_dpss_dn2 = assign94350_body20_e144850_d_n2;
            locals.var_fb_dpss_dn4 = assign94350_body20_e144850_d_n4;
            locals.var_fb_dpss_dn5 = assign94350_body20_e144850_d_n5;
            locals.var_fb_dpss_dn6 = assign94350_body20_e144850_d_n6;
            locals.var_fb_dpss_dn7 = assign94350_body20_e144850_d_n7;
            locals.var_fb_dpss_dn8 = assign94350_body20_e144850_d_n8;
            locals.var_fb_dpss_dn9 = assign94350_body20_e144850_d_n9;
            locals.var_fb_dpss_dn10 = assign94350_body20_e144850_d_n10;
            locals.var_fb_dpss_dn11 = assign94350_body20_e144850_d_n11;
            locals.var_fb_dpss_dn14 = assign94350_body20_e144850_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign94350_body21_e144853: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2187 = assign94350_body21_e144853;
            locals.var_guard2187_rv = 0.0;
            let (assign94350_body23_e144909, assign94350_body23_e144909_d_n0, assign94350_body23_e144909_d_n2, assign94350_body23_e144909_d_n4, assign94350_body23_e144909_d_n5, assign94350_body23_e144909_d_n6, assign94350_body23_e144909_d_n7, assign94350_body23_e144909_d_n8, assign94350_body23_e144909_d_n9, assign94350_body23_e144909_d_n10, assign94350_body23_e144909_d_n11, assign94350_body23_e144909_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94350_body23_e144887: f64 = (locals.var_chi * locals.var_chi);
        let assign94350_body23_e144889: f64 = (assign94350_body23_e144887 / 2.0);
        let assign94350_body23_e144893: f64 = (locals.var_chi / 3.0);
        let assign94350_body23_e144897: f64 = (locals.var_chi / 4.0);
        let assign94350_body23_e144901: f64 = (locals.var_chi / 5.0);
        let assign94350_body23_e144902: f64 = (1.0 - assign94350_body23_e144901);
        let assign94350_body23_e144903: f64 = (assign94350_body23_e144897 * assign94350_body23_e144902);
        let assign94350_body23_e144904: f64 = (1.0 - assign94350_body23_e144903);
        let assign94350_body23_e144905: f64 = (assign94350_body23_e144893 * assign94350_body23_e144904);
        let assign94350_body23_e144906: f64 = (1.0 - assign94350_body23_e144905);
        let assign94350_body23_e144907: f64 = (assign94350_body23_e144889 * assign94350_body23_e144906);
        (assign94350_body23_e144907, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn0 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn0 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn2 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn2 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn4 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn4 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn5 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn5 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn6 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn6 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn7 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn7 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn8 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn8 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn9 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn9 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn10 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn10 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn11 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn11 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94350_body23_e144906) + (assign94350_body23_e144889 * (-(((locals.var_chi_dn14 / 3.0) * assign94350_body23_e144904) + (assign94350_body23_e144893 * (-(((locals.var_chi_dn14 / 4.0) * assign94350_body23_e144902) + (assign94350_body23_e144897 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94350_body23_e144909;
            locals.var_t0_dn0 = assign94350_body23_e144909_d_n0;
            locals.var_t0_dn2 = assign94350_body23_e144909_d_n2;
            locals.var_t0_dn4 = assign94350_body23_e144909_d_n4;
            locals.var_t0_dn5 = assign94350_body23_e144909_d_n5;
            locals.var_t0_dn6 = assign94350_body23_e144909_d_n6;
            locals.var_t0_dn7 = assign94350_body23_e144909_d_n7;
            locals.var_t0_dn8 = assign94350_body23_e144909_d_n8;
            locals.var_t0_dn9 = assign94350_body23_e144909_d_n9;
            locals.var_t0_dn10 = assign94350_body23_e144909_d_n10;
            locals.var_t0_dn11 = assign94350_body23_e144909_d_n11;
            locals.var_t0_dn14 = assign94350_body23_e144909_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94350_body24_e144943, assign94350_body24_e144943_d_n0, assign94350_body24_e144943_d_n2, assign94350_body24_e144943_d_n4, assign94350_body24_e144943_d_n5, assign94350_body24_e144943_d_n6, assign94350_body24_e144943_d_n7, assign94350_body24_e144943_d_n8, assign94350_body24_e144943_d_n9, assign94350_body24_e144943_d_n10, assign94350_body24_e144943_d_n11, assign94350_body24_e144943_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94350_body24_e144927: f64 = (locals.var_chi / 2.0);
        let assign94350_body24_e144931: f64 = (locals.var_chi / 3.0);
        let assign94350_body24_e144935: f64 = (locals.var_chi / 4.0);
        let assign94350_body24_e144936: f64 = (1.0 - assign94350_body24_e144935);
        let assign94350_body24_e144937: f64 = (assign94350_body24_e144931 * assign94350_body24_e144936);
        let assign94350_body24_e144938: f64 = (1.0 - assign94350_body24_e144937);
        let assign94350_body24_e144939: f64 = (assign94350_body24_e144927 * assign94350_body24_e144938);
        let assign94350_body24_e144940: f64 = (1.0 - assign94350_body24_e144939);
        let assign94350_body24_e144941: f64 = (locals.var_chi * assign94350_body24_e144940);
        (assign94350_body24_e144941, ((locals.var_chi_dn0 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn0 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn2 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn4 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn5 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn6 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn7 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn8 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn9 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn10 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn11 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign94350_body24_e144940) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign94350_body24_e144938) + (assign94350_body24_e144927 * (-(((locals.var_chi_dn14 / 3.0) * assign94350_body24_e144936) + (assign94350_body24_e144931 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94350_body24_e144943;
            locals.var_t1_dn0 = assign94350_body24_e144943_d_n0;
            locals.var_t1_dn2 = assign94350_body24_e144943_d_n2;
            locals.var_t1_dn4 = assign94350_body24_e144943_d_n4;
            locals.var_t1_dn5 = assign94350_body24_e144943_d_n5;
            locals.var_t1_dn6 = assign94350_body24_e144943_d_n6;
            locals.var_t1_dn7 = assign94350_body24_e144943_d_n7;
            locals.var_t1_dn8 = assign94350_body24_e144943_d_n8;
            locals.var_t1_dn9 = assign94350_body24_e144943_d_n9;
            locals.var_t1_dn10 = assign94350_body24_e144943_d_n10;
            locals.var_t1_dn11 = assign94350_body24_e144943_d_n11;
            locals.var_t1_dn14 = assign94350_body24_e144943_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94350_body25_e144981, assign94350_body25_e144981_d_n0, assign94350_body25_e144981_d_n2, assign94350_body25_e144981_d_n4, assign94350_body25_e144981_d_n5, assign94350_body25_e144981_d_n6, assign94350_body25_e144981_d_n7, assign94350_body25_e144981_d_n8, assign94350_body25_e144981_d_n9, assign94350_body25_e144981_d_n10, assign94350_body25_e144981_d_n11, assign94350_body25_e144981_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94350_body25_e144959: f64 = (locals.var_chib * locals.var_chib);
        let assign94350_body25_e144961: f64 = (assign94350_body25_e144959 / 2.0);
        let assign94350_body25_e144965: f64 = (locals.var_chib / 3.0);
        let assign94350_body25_e144969: f64 = (locals.var_chib / 4.0);
        let assign94350_body25_e144973: f64 = (locals.var_chib / 5.0);
        let assign94350_body25_e144974: f64 = (1.0 - assign94350_body25_e144973);
        let assign94350_body25_e144975: f64 = (assign94350_body25_e144969 * assign94350_body25_e144974);
        let assign94350_body25_e144976: f64 = (1.0 - assign94350_body25_e144975);
        let assign94350_body25_e144977: f64 = (assign94350_body25_e144965 * assign94350_body25_e144976);
        let assign94350_body25_e144978: f64 = (1.0 - assign94350_body25_e144977);
        let assign94350_body25_e144979: f64 = (assign94350_body25_e144961 * assign94350_body25_e144978);
        (assign94350_body25_e144979, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn0 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn0 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn2 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn2 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn4 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn4 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn5 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn5 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn6 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn6 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn7 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn7 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn8 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn8 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn9 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn9 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn10 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn10 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn11 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn11 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign94350_body25_e144978) + (assign94350_body25_e144961 * (-(((locals.var_chib_dn14 / 3.0) * assign94350_body25_e144976) + (assign94350_body25_e144965 * (-(((locals.var_chib_dn14 / 4.0) * assign94350_body25_e144974) + (assign94350_body25_e144969 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94350_body25_e144981;
            locals.var_t2_dn0 = assign94350_body25_e144981_d_n0;
            locals.var_t2_dn2 = assign94350_body25_e144981_d_n2;
            locals.var_t2_dn4 = assign94350_body25_e144981_d_n4;
            locals.var_t2_dn5 = assign94350_body25_e144981_d_n5;
            locals.var_t2_dn6 = assign94350_body25_e144981_d_n6;
            locals.var_t2_dn7 = assign94350_body25_e144981_d_n7;
            locals.var_t2_dn8 = assign94350_body25_e144981_d_n8;
            locals.var_t2_dn9 = assign94350_body25_e144981_d_n9;
            locals.var_t2_dn10 = assign94350_body25_e144981_d_n10;
            locals.var_t2_dn11 = assign94350_body25_e144981_d_n11;
            locals.var_t2_dn14 = assign94350_body25_e144981_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94350_body26_e145015, assign94350_body26_e145015_d_n0, assign94350_body26_e145015_d_n2, assign94350_body26_e145015_d_n4, assign94350_body26_e145015_d_n5, assign94350_body26_e145015_d_n6, assign94350_body26_e145015_d_n7, assign94350_body26_e145015_d_n8, assign94350_body26_e145015_d_n9, assign94350_body26_e145015_d_n10, assign94350_body26_e145015_d_n11, assign94350_body26_e145015_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94350_body26_e144999: f64 = (locals.var_chib / 2.0);
        let assign94350_body26_e145003: f64 = (locals.var_chib / 3.0);
        let assign94350_body26_e145007: f64 = (locals.var_chib / 4.0);
        let assign94350_body26_e145008: f64 = (1.0 - assign94350_body26_e145007);
        let assign94350_body26_e145009: f64 = (assign94350_body26_e145003 * assign94350_body26_e145008);
        let assign94350_body26_e145010: f64 = (1.0 - assign94350_body26_e145009);
        let assign94350_body26_e145011: f64 = (assign94350_body26_e144999 * assign94350_body26_e145010);
        let assign94350_body26_e145012: f64 = (1.0 - assign94350_body26_e145011);
        let assign94350_body26_e145013: f64 = (locals.var_chib * assign94350_body26_e145012);
        (assign94350_body26_e145013, ((locals.var_chib_dn0 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn0 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn2 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn4 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn5 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn6 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn7 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn8 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn9 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn10 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn11 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign94350_body26_e145012) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign94350_body26_e145010) + (assign94350_body26_e144999 * (-(((locals.var_chib_dn14 / 3.0) * assign94350_body26_e145008) + (assign94350_body26_e145003 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign94350_body26_e145015;
            locals.var_t3_dn0 = assign94350_body26_e145015_d_n0;
            locals.var_t3_dn2 = assign94350_body26_e145015_d_n2;
            locals.var_t3_dn4 = assign94350_body26_e145015_d_n4;
            locals.var_t3_dn5 = assign94350_body26_e145015_d_n5;
            locals.var_t3_dn6 = assign94350_body26_e145015_d_n6;
            locals.var_t3_dn7 = assign94350_body26_e145015_d_n7;
            locals.var_t3_dn8 = assign94350_body26_e145015_d_n8;
            locals.var_t3_dn9 = assign94350_body26_e145015_d_n9;
            locals.var_t3_dn10 = assign94350_body26_e145015_d_n10;
            locals.var_t3_dn11 = assign94350_body26_e145015_d_n11;
            locals.var_t3_dn14 = assign94350_body26_e145015_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign94350_body27_e145033, assign94350_body27_e145033_d_n0, assign94350_body27_e145033_d_n2, assign94350_body27_e145033_d_n4, assign94350_body27_e145033_d_n5, assign94350_body27_e145033_d_n6, assign94350_body27_e145033_d_n7, assign94350_body27_e145033_d_n8, assign94350_body27_e145033_d_n9, assign94350_body27_e145033_d_n10, assign94350_body27_e145033_d_n11, assign94350_body27_e145033_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94350_body27_e145031: f64 = (locals.var_t0 - locals.var_t2);
        (assign94350_body27_e145031, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign94350_body27_e145033;
            locals.var_t4_dn0 = assign94350_body27_e145033_d_n0;
            locals.var_t4_dn2 = assign94350_body27_e145033_d_n2;
            locals.var_t4_dn4 = assign94350_body27_e145033_d_n4;
            locals.var_t4_dn5 = assign94350_body27_e145033_d_n5;
            locals.var_t4_dn6 = assign94350_body27_e145033_d_n6;
            locals.var_t4_dn7 = assign94350_body27_e145033_d_n7;
            locals.var_t4_dn8 = assign94350_body27_e145033_d_n8;
            locals.var_t4_dn9 = assign94350_body27_e145033_d_n9;
            locals.var_t4_dn10 = assign94350_body27_e145033_d_n10;
            locals.var_t4_dn11 = assign94350_body27_e145033_d_n11;
            locals.var_t4_dn14 = assign94350_body27_e145033_d_n14;
            locals.var_t4_rv = 0.0;
            let assign94350_body28_e145036: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2188 = assign94350_body28_e145036;
            locals.var_guard2188_rv = 0.0;
            let (assign94350_body29_e145055, assign94350_body29_e145055_d_n0, assign94350_body29_e145055_d_n2, assign94350_body29_e145055_d_n4, assign94350_body29_e145055_d_n5, assign94350_body29_e145055_d_n6, assign94350_body29_e145055_d_n7, assign94350_body29_e145055_d_n8, assign94350_body29_e145055_d_n9, assign94350_body29_e145055_d_n10, assign94350_body29_e145055_d_n11, assign94350_body29_e145055_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94350_body29_e145053: f64 = (locals.var_t4).sqrt();
        (assign94350_body29_e145053, (locals.var_t4_dn0 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn2 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn4 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn5 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn6 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn7 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn8 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn9 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn10 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn11 / (2.0 * assign94350_body29_e145053)), (locals.var_t4_dn14 / (2.0 * assign94350_body29_e145053)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94350_body29_e145055;
            locals.var_fb_dn0 = assign94350_body29_e145055_d_n0;
            locals.var_fb_dn2 = assign94350_body29_e145055_d_n2;
            locals.var_fb_dn4 = assign94350_body29_e145055_d_n4;
            locals.var_fb_dn5 = assign94350_body29_e145055_d_n5;
            locals.var_fb_dn6 = assign94350_body29_e145055_d_n6;
            locals.var_fb_dn7 = assign94350_body29_e145055_d_n7;
            locals.var_fb_dn8 = assign94350_body29_e145055_d_n8;
            locals.var_fb_dn9 = assign94350_body29_e145055_d_n9;
            locals.var_fb_dn10 = assign94350_body29_e145055_d_n10;
            locals.var_fb_dn11 = assign94350_body29_e145055_d_n11;
            locals.var_fb_dn14 = assign94350_body29_e145055_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94350_body30_e145083, assign94350_body30_e145083_d_n0, assign94350_body30_e145083_d_n2, assign94350_body30_e145083_d_n4, assign94350_body30_e145083_d_n5, assign94350_body30_e145083_d_n6, assign94350_body30_e145083_d_n7, assign94350_body30_e145083_d_n8, assign94350_body30_e145083_d_n9, assign94350_body30_e145083_d_n10, assign94350_body30_e145083_d_n11, assign94350_body30_e145083_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94350_body30_e145073: f64 = (locals.var_beta * 0.5);
        let assign94350_body30_e145077: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94350_body30_e145078: f64 = (locals.var_t1 - assign94350_body30_e145077);
        let assign94350_body30_e145079: f64 = (assign94350_body30_e145073 * assign94350_body30_e145078);
        let assign94350_body30_e145081: f64 = (assign94350_body30_e145079 / locals.var_fb);
        (assign94350_body30_e145081, ((((((locals.var_beta_dn0 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign94350_body30_e145078) + (assign94350_body30_e145073 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))) * locals.var_fb) - (assign94350_body30_e145079 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94350_body30_e145083;
            locals.var_fb_dpss_dn0 = assign94350_body30_e145083_d_n0;
            locals.var_fb_dpss_dn2 = assign94350_body30_e145083_d_n2;
            locals.var_fb_dpss_dn4 = assign94350_body30_e145083_d_n4;
            locals.var_fb_dpss_dn5 = assign94350_body30_e145083_d_n5;
            locals.var_fb_dpss_dn6 = assign94350_body30_e145083_d_n6;
            locals.var_fb_dpss_dn7 = assign94350_body30_e145083_d_n7;
            locals.var_fb_dpss_dn8 = assign94350_body30_e145083_d_n8;
            locals.var_fb_dpss_dn9 = assign94350_body30_e145083_d_n9;
            locals.var_fb_dpss_dn10 = assign94350_body30_e145083_d_n10;
            locals.var_fb_dpss_dn11 = assign94350_body30_e145083_d_n11;
            locals.var_fb_dpss_dn14 = assign94350_body30_e145083_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94350_body32_e145123, assign94350_body32_e145123_d_n0, assign94350_body32_e145123_d_n2, assign94350_body32_e145123_d_n4, assign94350_body32_e145123_d_n5, assign94350_body32_e145123_d_n6, assign94350_body32_e145123_d_n7, assign94350_body32_e145123_d_n8, assign94350_body32_e145123_d_n9, assign94350_body32_e145123_d_n10, assign94350_body32_e145123_d_n11, assign94350_body32_e145123_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) && (locals.var_guard2188 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94350_body32_e145123;
            locals.var_fb_dn0 = assign94350_body32_e145123_d_n0;
            locals.var_fb_dn2 = assign94350_body32_e145123_d_n2;
            locals.var_fb_dn4 = assign94350_body32_e145123_d_n4;
            locals.var_fb_dn5 = assign94350_body32_e145123_d_n5;
            locals.var_fb_dn6 = assign94350_body32_e145123_d_n6;
            locals.var_fb_dn7 = assign94350_body32_e145123_d_n7;
            locals.var_fb_dn8 = assign94350_body32_e145123_d_n8;
            locals.var_fb_dn9 = assign94350_body32_e145123_d_n9;
            locals.var_fb_dn10 = assign94350_body32_e145123_d_n10;
            locals.var_fb_dn11 = assign94350_body32_e145123_d_n11;
            locals.var_fb_dn14 = assign94350_body32_e145123_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94350_body33_e145142, assign94350_body33_e145142_d_n0, assign94350_body33_e145142_d_n2, assign94350_body33_e145142_d_n4, assign94350_body33_e145142_d_n5, assign94350_body33_e145142_d_n6, assign94350_body33_e145142_d_n7, assign94350_body33_e145142_d_n8, assign94350_body33_e145142_d_n9, assign94350_body33_e145142_d_n10, assign94350_body33_e145142_d_n11, assign94350_body33_e145142_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 != 0.0)) && (locals.var_guard2188 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94350_body33_e145142;
            locals.var_fb_dpss_dn0 = assign94350_body33_e145142_d_n0;
            locals.var_fb_dpss_dn2 = assign94350_body33_e145142_d_n2;
            locals.var_fb_dpss_dn4 = assign94350_body33_e145142_d_n4;
            locals.var_fb_dpss_dn5 = assign94350_body33_e145142_d_n5;
            locals.var_fb_dpss_dn6 = assign94350_body33_e145142_d_n6;
            locals.var_fb_dpss_dn7 = assign94350_body33_e145142_d_n7;
            locals.var_fb_dpss_dn8 = assign94350_body33_e145142_d_n8;
            locals.var_fb_dpss_dn9 = assign94350_body33_e145142_d_n9;
            locals.var_fb_dpss_dn10 = assign94350_body33_e145142_d_n10;
            locals.var_fb_dpss_dn11 = assign94350_body33_e145142_d_n11;
            locals.var_fb_dpss_dn14 = assign94350_body33_e145142_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94350_body34_e145161, assign94350_body34_e145161_d_n0, assign94350_body34_e145161_d_n2, assign94350_body34_e145161_d_n4, assign94350_body34_e145161_d_n5, assign94350_body34_e145161_d_n6, assign94350_body34_e145161_d_n7, assign94350_body34_e145161_d_n8, assign94350_body34_e145161_d_n9, assign94350_body34_e145161_d_n10, assign94350_body34_e145161_d_n11, assign94350_body34_e145161_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) {
        let assign94350_body34_e145158: f64 = (-locals.var_chi);
        let assign94350_body34_e145159: f64 = (assign94350_body34_e145158).exp();
        (assign94350_body34_e145159, (assign94350_body34_e145159 * (-locals.var_chi_dn0)), (assign94350_body34_e145159 * (-locals.var_chi_dn2)), (assign94350_body34_e145159 * (-locals.var_chi_dn4)), (assign94350_body34_e145159 * (-locals.var_chi_dn5)), (assign94350_body34_e145159 * (-locals.var_chi_dn6)), (assign94350_body34_e145159 * (-locals.var_chi_dn7)), (assign94350_body34_e145159 * (-locals.var_chi_dn8)), (assign94350_body34_e145159 * (-locals.var_chi_dn9)), (assign94350_body34_e145159 * (-locals.var_chi_dn10)), (assign94350_body34_e145159 * (-locals.var_chi_dn11)), (assign94350_body34_e145159 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94350_body34_e145161;
            locals.var_t0_dn0 = assign94350_body34_e145161_d_n0;
            locals.var_t0_dn2 = assign94350_body34_e145161_d_n2;
            locals.var_t0_dn4 = assign94350_body34_e145161_d_n4;
            locals.var_t0_dn5 = assign94350_body34_e145161_d_n5;
            locals.var_t0_dn6 = assign94350_body34_e145161_d_n6;
            locals.var_t0_dn7 = assign94350_body34_e145161_d_n7;
            locals.var_t0_dn8 = assign94350_body34_e145161_d_n8;
            locals.var_t0_dn9 = assign94350_body34_e145161_d_n9;
            locals.var_t0_dn10 = assign94350_body34_e145161_d_n10;
            locals.var_t0_dn11 = assign94350_body34_e145161_d_n11;
            locals.var_t0_dn14 = assign94350_body34_e145161_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94350_body35_e145180, assign94350_body35_e145180_d_n0, assign94350_body35_e145180_d_n2, assign94350_body35_e145180_d_n4, assign94350_body35_e145180_d_n5, assign94350_body35_e145180_d_n6, assign94350_body35_e145180_d_n7, assign94350_body35_e145180_d_n8, assign94350_body35_e145180_d_n9, assign94350_body35_e145180_d_n10, assign94350_body35_e145180_d_n11, assign94350_body35_e145180_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) {
        let assign94350_body35_e145177: f64 = (-locals.var_chib);
        let assign94350_body35_e145178: f64 = (assign94350_body35_e145177).exp();
        (assign94350_body35_e145178, (assign94350_body35_e145178 * (-locals.var_chib_dn0)), (assign94350_body35_e145178 * (-locals.var_chib_dn2)), (assign94350_body35_e145178 * (-locals.var_chib_dn4)), (assign94350_body35_e145178 * (-locals.var_chib_dn5)), (assign94350_body35_e145178 * (-locals.var_chib_dn6)), (assign94350_body35_e145178 * (-locals.var_chib_dn7)), (assign94350_body35_e145178 * (-locals.var_chib_dn8)), (assign94350_body35_e145178 * (-locals.var_chib_dn9)), (assign94350_body35_e145178 * (-locals.var_chib_dn10)), (assign94350_body35_e145178 * (-locals.var_chib_dn11)), (assign94350_body35_e145178 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94350_body35_e145180;
            locals.var_t1_dn0 = assign94350_body35_e145180_d_n0;
            locals.var_t1_dn2 = assign94350_body35_e145180_d_n2;
            locals.var_t1_dn4 = assign94350_body35_e145180_d_n4;
            locals.var_t1_dn5 = assign94350_body35_e145180_d_n5;
            locals.var_t1_dn6 = assign94350_body35_e145180_d_n6;
            locals.var_t1_dn7 = assign94350_body35_e145180_d_n7;
            locals.var_t1_dn8 = assign94350_body35_e145180_d_n8;
            locals.var_t1_dn9 = assign94350_body35_e145180_d_n9;
            locals.var_t1_dn10 = assign94350_body35_e145180_d_n10;
            locals.var_t1_dn11 = assign94350_body35_e145180_d_n11;
            locals.var_t1_dn14 = assign94350_body35_e145180_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94350_body36_e145203, assign94350_body36_e145203_d_n0, assign94350_body36_e145203_d_n2, assign94350_body36_e145203_d_n4, assign94350_body36_e145203_d_n5, assign94350_body36_e145203_d_n6, assign94350_body36_e145203_d_n7, assign94350_body36_e145203_d_n8, assign94350_body36_e145203_d_n9, assign94350_body36_e145203_d_n10, assign94350_body36_e145203_d_n11, assign94350_body36_e145203_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) {
        let assign94350_body36_e145197: f64 = (locals.var_chi - locals.var_chib);
        let assign94350_body36_e145200: f64 = (locals.var_t0 - locals.var_t1);
        let assign94350_body36_e145201: f64 = (assign94350_body36_e145197 + assign94350_body36_e145200);
        (assign94350_body36_e145201, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign94350_body36_e145203;
            locals.var_t4_dn0 = assign94350_body36_e145203_d_n0;
            locals.var_t4_dn2 = assign94350_body36_e145203_d_n2;
            locals.var_t4_dn4 = assign94350_body36_e145203_d_n4;
            locals.var_t4_dn5 = assign94350_body36_e145203_d_n5;
            locals.var_t4_dn6 = assign94350_body36_e145203_d_n6;
            locals.var_t4_dn7 = assign94350_body36_e145203_d_n7;
            locals.var_t4_dn8 = assign94350_body36_e145203_d_n8;
            locals.var_t4_dn9 = assign94350_body36_e145203_d_n9;
            locals.var_t4_dn10 = assign94350_body36_e145203_d_n10;
            locals.var_t4_dn11 = assign94350_body36_e145203_d_n11;
            locals.var_t4_dn14 = assign94350_body36_e145203_d_n14;
            locals.var_t4_rv = 0.0;
            let assign94350_body37_e145206: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2189 = assign94350_body37_e145206;
            locals.var_guard2189_rv = 0.0;
            let (assign94350_body38_e145226, assign94350_body38_e145226_d_n0, assign94350_body38_e145226_d_n2, assign94350_body38_e145226_d_n4, assign94350_body38_e145226_d_n5, assign94350_body38_e145226_d_n6, assign94350_body38_e145226_d_n7, assign94350_body38_e145226_d_n8, assign94350_body38_e145226_d_n9, assign94350_body38_e145226_d_n10, assign94350_body38_e145226_d_n11, assign94350_body38_e145226_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94350_body38_e145224: f64 = (locals.var_t4).sqrt();
        (assign94350_body38_e145224, (locals.var_t4_dn0 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn2 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn4 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn5 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn6 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn7 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn8 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn9 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn10 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn11 / (2.0 * assign94350_body38_e145224)), (locals.var_t4_dn14 / (2.0 * assign94350_body38_e145224)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94350_body38_e145226;
            locals.var_fb_dn0 = assign94350_body38_e145226_d_n0;
            locals.var_fb_dn2 = assign94350_body38_e145226_d_n2;
            locals.var_fb_dn4 = assign94350_body38_e145226_d_n4;
            locals.var_fb_dn5 = assign94350_body38_e145226_d_n5;
            locals.var_fb_dn6 = assign94350_body38_e145226_d_n6;
            locals.var_fb_dn7 = assign94350_body38_e145226_d_n7;
            locals.var_fb_dn8 = assign94350_body38_e145226_d_n8;
            locals.var_fb_dn9 = assign94350_body38_e145226_d_n9;
            locals.var_fb_dn10 = assign94350_body38_e145226_d_n10;
            locals.var_fb_dn11 = assign94350_body38_e145226_d_n11;
            locals.var_fb_dn14 = assign94350_body38_e145226_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94350_body39_e145259, assign94350_body39_e145259_d_n0, assign94350_body39_e145259_d_n2, assign94350_body39_e145259_d_n4, assign94350_body39_e145259_d_n5, assign94350_body39_e145259_d_n6, assign94350_body39_e145259_d_n7, assign94350_body39_e145259_d_n8, assign94350_body39_e145259_d_n9, assign94350_body39_e145259_d_n10, assign94350_body39_e145259_d_n11, assign94350_body39_e145259_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94350_body39_e145245: f64 = (locals.var_beta * 0.5);
        let assign94350_body39_e145248: f64 = (1.0 - locals.var_t0);
        let assign94350_body39_e145252: f64 = (1.0 - locals.var_t1);
        let assign94350_body39_e145253: f64 = (locals.var_phi_b_dpss * assign94350_body39_e145252);
        let assign94350_body39_e145254: f64 = (assign94350_body39_e145248 - assign94350_body39_e145253);
        let assign94350_body39_e145255: f64 = (assign94350_body39_e145245 * assign94350_body39_e145254);
        let assign94350_body39_e145257: f64 = (assign94350_body39_e145255 / locals.var_fb);
        (assign94350_body39_e145257, ((((((locals.var_beta_dn0 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign94350_body39_e145254) + (assign94350_body39_e145245 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign94350_body39_e145252) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign94350_body39_e145255 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94350_body39_e145259;
            locals.var_fb_dpss_dn0 = assign94350_body39_e145259_d_n0;
            locals.var_fb_dpss_dn2 = assign94350_body39_e145259_d_n2;
            locals.var_fb_dpss_dn4 = assign94350_body39_e145259_d_n4;
            locals.var_fb_dpss_dn5 = assign94350_body39_e145259_d_n5;
            locals.var_fb_dpss_dn6 = assign94350_body39_e145259_d_n6;
            locals.var_fb_dpss_dn7 = assign94350_body39_e145259_d_n7;
            locals.var_fb_dpss_dn8 = assign94350_body39_e145259_d_n8;
            locals.var_fb_dpss_dn9 = assign94350_body39_e145259_d_n9;
            locals.var_fb_dpss_dn10 = assign94350_body39_e145259_d_n10;
            locals.var_fb_dpss_dn11 = assign94350_body39_e145259_d_n11;
            locals.var_fb_dpss_dn14 = assign94350_body39_e145259_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94350_body41_e145301, assign94350_body41_e145301_d_n0, assign94350_body41_e145301_d_n2, assign94350_body41_e145301_d_n4, assign94350_body41_e145301_d_n5, assign94350_body41_e145301_d_n6, assign94350_body41_e145301_d_n7, assign94350_body41_e145301_d_n8, assign94350_body41_e145301_d_n9, assign94350_body41_e145301_d_n10, assign94350_body41_e145301_d_n11, assign94350_body41_e145301_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94350_body41_e145301;
            locals.var_fb_dn0 = assign94350_body41_e145301_d_n0;
            locals.var_fb_dn2 = assign94350_body41_e145301_d_n2;
            locals.var_fb_dn4 = assign94350_body41_e145301_d_n4;
            locals.var_fb_dn5 = assign94350_body41_e145301_d_n5;
            locals.var_fb_dn6 = assign94350_body41_e145301_d_n6;
            locals.var_fb_dn7 = assign94350_body41_e145301_d_n7;
            locals.var_fb_dn8 = assign94350_body41_e145301_d_n8;
            locals.var_fb_dn9 = assign94350_body41_e145301_d_n9;
            locals.var_fb_dn10 = assign94350_body41_e145301_d_n10;
            locals.var_fb_dn11 = assign94350_body41_e145301_d_n11;
            locals.var_fb_dn14 = assign94350_body41_e145301_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94350_body42_e145321, assign94350_body42_e145321_d_n0, assign94350_body42_e145321_d_n2, assign94350_body42_e145321_d_n4, assign94350_body42_e145321_d_n5, assign94350_body42_e145321_d_n6, assign94350_body42_e145321_d_n7, assign94350_body42_e145321_d_n8, assign94350_body42_e145321_d_n9, assign94350_body42_e145321_d_n10, assign94350_body42_e145321_d_n11, assign94350_body42_e145321_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2186 == 0.0)) && (locals.var_guard2187 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94350_body42_e145321;
            locals.var_fb_dpss_dn0 = assign94350_body42_e145321_d_n0;
            locals.var_fb_dpss_dn2 = assign94350_body42_e145321_d_n2;
            locals.var_fb_dpss_dn4 = assign94350_body42_e145321_d_n4;
            locals.var_fb_dpss_dn5 = assign94350_body42_e145321_d_n5;
            locals.var_fb_dpss_dn6 = assign94350_body42_e145321_d_n6;
            locals.var_fb_dpss_dn7 = assign94350_body42_e145321_d_n7;
            locals.var_fb_dpss_dn8 = assign94350_body42_e145321_d_n8;
            locals.var_fb_dpss_dn9 = assign94350_body42_e145321_d_n9;
            locals.var_fb_dpss_dn10 = assign94350_body42_e145321_d_n10;
            locals.var_fb_dpss_dn11 = assign94350_body42_e145321_d_n11;
            locals.var_fb_dpss_dn14 = assign94350_body42_e145321_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign94350_body43_e145324: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2190 = assign94350_body43_e145324;
            locals.var_guard2190_rv = 0.0;
            let (assign94350_body45_e145352, assign94350_body45_e145352_d_n0, assign94350_body45_e145352_d_n2, assign94350_body45_e145352_d_n4, assign94350_body45_e145352_d_n5, assign94350_body45_e145352_d_n6, assign94350_body45_e145352_d_n7, assign94350_body45_e145352_d_n8, assign94350_body45_e145352_d_n9, assign94350_body45_e145352_d_n10, assign94350_body45_e145352_d_n11, assign94350_body45_e145352_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94350_body45_e145352;
            locals.var_fs01_dn0 = assign94350_body45_e145352_d_n0;
            locals.var_fs01_dn2 = assign94350_body45_e145352_d_n2;
            locals.var_fs01_dn4 = assign94350_body45_e145352_d_n4;
            locals.var_fs01_dn5 = assign94350_body45_e145352_d_n5;
            locals.var_fs01_dn6 = assign94350_body45_e145352_d_n6;
            locals.var_fs01_dn7 = assign94350_body45_e145352_d_n7;
            locals.var_fs01_dn8 = assign94350_body45_e145352_d_n8;
            locals.var_fs01_dn9 = assign94350_body45_e145352_d_n9;
            locals.var_fs01_dn10 = assign94350_body45_e145352_d_n10;
            locals.var_fs01_dn11 = assign94350_body45_e145352_d_n11;
            locals.var_fs01_dn14 = assign94350_body45_e145352_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94350_body46_e145365, assign94350_body46_e145365_d_n0, assign94350_body46_e145365_d_n2, assign94350_body46_e145365_d_n4, assign94350_body46_e145365_d_n5, assign94350_body46_e145365_d_n6, assign94350_body46_e145365_d_n7, assign94350_body46_e145365_d_n8, assign94350_body46_e145365_d_n9, assign94350_body46_e145365_d_n10, assign94350_body46_e145365_d_n11, assign94350_body46_e145365_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94350_body46_e145365;
            locals.var_fs01_dps0_dn0 = assign94350_body46_e145365_d_n0;
            locals.var_fs01_dps0_dn2 = assign94350_body46_e145365_d_n2;
            locals.var_fs01_dps0_dn4 = assign94350_body46_e145365_d_n4;
            locals.var_fs01_dps0_dn5 = assign94350_body46_e145365_d_n5;
            locals.var_fs01_dps0_dn6 = assign94350_body46_e145365_d_n6;
            locals.var_fs01_dps0_dn7 = assign94350_body46_e145365_d_n7;
            locals.var_fs01_dps0_dn8 = assign94350_body46_e145365_d_n8;
            locals.var_fs01_dps0_dn9 = assign94350_body46_e145365_d_n9;
            locals.var_fs01_dps0_dn10 = assign94350_body46_e145365_d_n10;
            locals.var_fs01_dps0_dn11 = assign94350_body46_e145365_d_n11;
            locals.var_fs01_dps0_dn14 = assign94350_body46_e145365_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94350_body47_e145379, assign94350_body47_e145379_d_n0, assign94350_body47_e145379_d_n2, assign94350_body47_e145379_d_n4, assign94350_body47_e145379_d_n5, assign94350_body47_e145379_d_n6, assign94350_body47_e145379_d_n7, assign94350_body47_e145379_d_n8, assign94350_body47_e145379_d_n9, assign94350_body47_e145379_d_n10, assign94350_body47_e145379_d_n11, assign94350_body47_e145379_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94350_body47_e145377: f64 = (-locals.var_fb);
        (assign94350_body47_e145377, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94350_body47_e145379;
            locals.var_fs02_dn0 = assign94350_body47_e145379_d_n0;
            locals.var_fs02_dn2 = assign94350_body47_e145379_d_n2;
            locals.var_fs02_dn4 = assign94350_body47_e145379_d_n4;
            locals.var_fs02_dn5 = assign94350_body47_e145379_d_n5;
            locals.var_fs02_dn6 = assign94350_body47_e145379_d_n6;
            locals.var_fs02_dn7 = assign94350_body47_e145379_d_n7;
            locals.var_fs02_dn8 = assign94350_body47_e145379_d_n8;
            locals.var_fs02_dn9 = assign94350_body47_e145379_d_n9;
            locals.var_fs02_dn10 = assign94350_body47_e145379_d_n10;
            locals.var_fs02_dn11 = assign94350_body47_e145379_d_n11;
            locals.var_fs02_dn14 = assign94350_body47_e145379_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94350_body48_e145393, assign94350_body48_e145393_d_n0, assign94350_body48_e145393_d_n2, assign94350_body48_e145393_d_n4, assign94350_body48_e145393_d_n5, assign94350_body48_e145393_d_n6, assign94350_body48_e145393_d_n7, assign94350_body48_e145393_d_n8, assign94350_body48_e145393_d_n9, assign94350_body48_e145393_d_n10, assign94350_body48_e145393_d_n11, assign94350_body48_e145393_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94350_body48_e145391: f64 = (-locals.var_fb_dpss);
        (assign94350_body48_e145391, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94350_body48_e145393;
            locals.var_fs02_dps0_dn0 = assign94350_body48_e145393_d_n0;
            locals.var_fs02_dps0_dn2 = assign94350_body48_e145393_d_n2;
            locals.var_fs02_dps0_dn4 = assign94350_body48_e145393_d_n4;
            locals.var_fs02_dps0_dn5 = assign94350_body48_e145393_d_n5;
            locals.var_fs02_dps0_dn6 = assign94350_body48_e145393_d_n6;
            locals.var_fs02_dps0_dn7 = assign94350_body48_e145393_d_n7;
            locals.var_fs02_dps0_dn8 = assign94350_body48_e145393_d_n8;
            locals.var_fs02_dps0_dn9 = assign94350_body48_e145393_d_n9;
            locals.var_fs02_dps0_dn10 = assign94350_body48_e145393_d_n10;
            locals.var_fs02_dps0_dn11 = assign94350_body48_e145393_d_n11;
            locals.var_fs02_dps0_dn14 = assign94350_body48_e145393_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94350_body49_e145396: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2191 = assign94350_body49_e145396;
            locals.var_guard2191_rv = 0.0;
            let assign94350_body50_e145399: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2192 = assign94350_body50_e145399;
            locals.var_guard2192_rv = 0.0;
            let (assign94350_body51_e145439, assign94350_body51_e145439_d_n0, assign94350_body51_e145439_d_n2, assign94350_body51_e145439_d_n4, assign94350_body51_e145439_d_n5, assign94350_body51_e145439_d_n6, assign94350_body51_e145439_d_n7, assign94350_body51_e145439_d_n8, assign94350_body51_e145439_d_n9, assign94350_body51_e145439_d_n10, assign94350_body51_e145439_d_n11, assign94350_body51_e145439_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94350_body51_e145417: f64 = (locals.var_chi * locals.var_chi);
        let assign94350_body51_e145419: f64 = (assign94350_body51_e145417 / 2.0);
        let assign94350_body51_e145423: f64 = (locals.var_chi / 3.0);
        let assign94350_body51_e145427: f64 = (locals.var_chi / 4.0);
        let assign94350_body51_e145431: f64 = (locals.var_chi / 5.0);
        let assign94350_body51_e145432: f64 = (1.0 + assign94350_body51_e145431);
        let assign94350_body51_e145433: f64 = (assign94350_body51_e145427 * assign94350_body51_e145432);
        let assign94350_body51_e145434: f64 = (1.0 + assign94350_body51_e145433);
        let assign94350_body51_e145435: f64 = (assign94350_body51_e145423 * assign94350_body51_e145434);
        let assign94350_body51_e145436: f64 = (1.0 + assign94350_body51_e145435);
        let assign94350_body51_e145437: f64 = (assign94350_body51_e145419 * assign94350_body51_e145436);
        (assign94350_body51_e145437, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn0 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn0 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn2 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn2 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn4 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn4 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn5 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn5 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn6 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn6 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn7 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn7 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn8 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn8 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn9 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn9 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn10 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn10 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn11 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn11 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94350_body51_e145436) + (assign94350_body51_e145419 * (((locals.var_chi_dn14 / 3.0) * assign94350_body51_e145434) + (assign94350_body51_e145423 * (((locals.var_chi_dn14 / 4.0) * assign94350_body51_e145432) + (assign94350_body51_e145427 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94350_body51_e145439;
            locals.var_t0_dn0 = assign94350_body51_e145439_d_n0;
            locals.var_t0_dn2 = assign94350_body51_e145439_d_n2;
            locals.var_t0_dn4 = assign94350_body51_e145439_d_n4;
            locals.var_t0_dn5 = assign94350_body51_e145439_d_n5;
            locals.var_t0_dn6 = assign94350_body51_e145439_d_n6;
            locals.var_t0_dn7 = assign94350_body51_e145439_d_n7;
            locals.var_t0_dn8 = assign94350_body51_e145439_d_n8;
            locals.var_t0_dn9 = assign94350_body51_e145439_d_n9;
            locals.var_t0_dn10 = assign94350_body51_e145439_d_n10;
            locals.var_t0_dn11 = assign94350_body51_e145439_d_n11;
            locals.var_t0_dn14 = assign94350_body51_e145439_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94350_body52_e145475, assign94350_body52_e145475_d_n0, assign94350_body52_e145475_d_n2, assign94350_body52_e145475_d_n4, assign94350_body52_e145475_d_n5, assign94350_body52_e145475_d_n6, assign94350_body52_e145475_d_n7, assign94350_body52_e145475_d_n8, assign94350_body52_e145475_d_n9, assign94350_body52_e145475_d_n10, assign94350_body52_e145475_d_n11, assign94350_body52_e145475_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94350_body52_e145459: f64 = (locals.var_chi / 2.0);
        let assign94350_body52_e145463: f64 = (locals.var_chi / 3.0);
        let assign94350_body52_e145467: f64 = (locals.var_chi / 4.0);
        let assign94350_body52_e145468: f64 = (1.0 + assign94350_body52_e145467);
        let assign94350_body52_e145469: f64 = (assign94350_body52_e145463 * assign94350_body52_e145468);
        let assign94350_body52_e145470: f64 = (1.0 + assign94350_body52_e145469);
        let assign94350_body52_e145471: f64 = (assign94350_body52_e145459 * assign94350_body52_e145470);
        let assign94350_body52_e145472: f64 = (1.0 + assign94350_body52_e145471);
        let assign94350_body52_e145473: f64 = (locals.var_chi * assign94350_body52_e145472);
        (assign94350_body52_e145473, ((locals.var_chi_dn0 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn0 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn2 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn4 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn5 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn6 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn7 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn8 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn9 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn10 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn11 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign94350_body52_e145472) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign94350_body52_e145470) + (assign94350_body52_e145459 * (((locals.var_chi_dn14 / 3.0) * assign94350_body52_e145468) + (assign94350_body52_e145463 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94350_body52_e145475;
            locals.var_t1_dn0 = assign94350_body52_e145475_d_n0;
            locals.var_t1_dn2 = assign94350_body52_e145475_d_n2;
            locals.var_t1_dn4 = assign94350_body52_e145475_d_n4;
            locals.var_t1_dn5 = assign94350_body52_e145475_d_n5;
            locals.var_t1_dn6 = assign94350_body52_e145475_d_n6;
            locals.var_t1_dn7 = assign94350_body52_e145475_d_n7;
            locals.var_t1_dn8 = assign94350_body52_e145475_d_n8;
            locals.var_t1_dn9 = assign94350_body52_e145475_d_n9;
            locals.var_t1_dn10 = assign94350_body52_e145475_d_n10;
            locals.var_t1_dn11 = assign94350_body52_e145475_d_n11;
            locals.var_t1_dn14 = assign94350_body52_e145475_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94350_body53_e145495, assign94350_body53_e145495_d_n0, assign94350_body53_e145495_d_n2, assign94350_body53_e145495_d_n4, assign94350_body53_e145495_d_n5, assign94350_body53_e145495_d_n6, assign94350_body53_e145495_d_n7, assign94350_body53_e145495_d_n8, assign94350_body53_e145495_d_n9, assign94350_body53_e145495_d_n10, assign94350_body53_e145495_d_n11, assign94350_body53_e145495_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94350_body53_e145493: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94350_body53_e145493, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94350_body53_e145495;
            locals.var_fs01_dn0 = assign94350_body53_e145495_d_n0;
            locals.var_fs01_dn2 = assign94350_body53_e145495_d_n2;
            locals.var_fs01_dn4 = assign94350_body53_e145495_d_n4;
            locals.var_fs01_dn5 = assign94350_body53_e145495_d_n5;
            locals.var_fs01_dn6 = assign94350_body53_e145495_d_n6;
            locals.var_fs01_dn7 = assign94350_body53_e145495_d_n7;
            locals.var_fs01_dn8 = assign94350_body53_e145495_d_n8;
            locals.var_fs01_dn9 = assign94350_body53_e145495_d_n9;
            locals.var_fs01_dn10 = assign94350_body53_e145495_d_n10;
            locals.var_fs01_dn11 = assign94350_body53_e145495_d_n11;
            locals.var_fs01_dn14 = assign94350_body53_e145495_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94350_body54_e145517, assign94350_body54_e145517_d_n0, assign94350_body54_e145517_d_n2, assign94350_body54_e145517_d_n4, assign94350_body54_e145517_d_n5, assign94350_body54_e145517_d_n6, assign94350_body54_e145517_d_n7, assign94350_body54_e145517_d_n8, assign94350_body54_e145517_d_n9, assign94350_body54_e145517_d_n10, assign94350_body54_e145517_d_n11, assign94350_body54_e145517_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94350_body54_e145513: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94350_body54_e145515: f64 = (assign94350_body54_e145513 * locals.var_beta);
        (assign94350_body54_e145515, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign94350_body54_e145513 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94350_body54_e145517;
            locals.var_fs01_dps0_dn0 = assign94350_body54_e145517_d_n0;
            locals.var_fs01_dps0_dn2 = assign94350_body54_e145517_d_n2;
            locals.var_fs01_dps0_dn4 = assign94350_body54_e145517_d_n4;
            locals.var_fs01_dps0_dn5 = assign94350_body54_e145517_d_n5;
            locals.var_fs01_dps0_dn6 = assign94350_body54_e145517_d_n6;
            locals.var_fs01_dps0_dn7 = assign94350_body54_e145517_d_n7;
            locals.var_fs01_dps0_dn8 = assign94350_body54_e145517_d_n8;
            locals.var_fs01_dps0_dn9 = assign94350_body54_e145517_d_n9;
            locals.var_fs01_dps0_dn10 = assign94350_body54_e145517_d_n10;
            locals.var_fs01_dps0_dn11 = assign94350_body54_e145517_d_n11;
            locals.var_fs01_dps0_dn14 = assign94350_body54_e145517_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94350_body55_e145537, assign94350_body55_e145537_d_n0, assign94350_body55_e145537_d_n2, assign94350_body55_e145537_d_n4, assign94350_body55_e145537_d_n5, assign94350_body55_e145537_d_n6, assign94350_body55_e145537_d_n7, assign94350_body55_e145537_d_n8, assign94350_body55_e145537_d_n9, assign94350_body55_e145537_d_n10, assign94350_body55_e145537_d_n11, assign94350_body55_e145537_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94350_body55_e145535: f64 = (locals.var_chi).exp();
        (assign94350_body55_e145535, (assign94350_body55_e145535 * locals.var_chi_dn0), (assign94350_body55_e145535 * locals.var_chi_dn2), (assign94350_body55_e145535 * locals.var_chi_dn4), (assign94350_body55_e145535 * locals.var_chi_dn5), (assign94350_body55_e145535 * locals.var_chi_dn6), (assign94350_body55_e145535 * locals.var_chi_dn7), (assign94350_body55_e145535 * locals.var_chi_dn8), (assign94350_body55_e145535 * locals.var_chi_dn9), (assign94350_body55_e145535 * locals.var_chi_dn10), (assign94350_body55_e145535 * locals.var_chi_dn11), (assign94350_body55_e145535 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign94350_body55_e145537;
            locals.var_exp_chi_dn0 = assign94350_body55_e145537_d_n0;
            locals.var_exp_chi_dn2 = assign94350_body55_e145537_d_n2;
            locals.var_exp_chi_dn4 = assign94350_body55_e145537_d_n4;
            locals.var_exp_chi_dn5 = assign94350_body55_e145537_d_n5;
            locals.var_exp_chi_dn6 = assign94350_body55_e145537_d_n6;
            locals.var_exp_chi_dn7 = assign94350_body55_e145537_d_n7;
            locals.var_exp_chi_dn8 = assign94350_body55_e145537_d_n8;
            locals.var_exp_chi_dn9 = assign94350_body55_e145537_d_n9;
            locals.var_exp_chi_dn10 = assign94350_body55_e145537_d_n10;
            locals.var_exp_chi_dn11 = assign94350_body55_e145537_d_n11;
            locals.var_exp_chi_dn14 = assign94350_body55_e145537_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign94350_body56_e145558, assign94350_body56_e145558_d_n0, assign94350_body56_e145558_d_n2, assign94350_body56_e145558_d_n4, assign94350_body56_e145558_d_n5, assign94350_body56_e145558_d_n6, assign94350_body56_e145558_d_n7, assign94350_body56_e145558_d_n8, assign94350_body56_e145558_d_n9, assign94350_body56_e145558_d_n10, assign94350_body56_e145558_d_n11, assign94350_body56_e145558_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94350_body56_e145556: f64 = (locals.var_exp_chi - 1.0);
        (assign94350_body56_e145556, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94350_body56_e145558;
            locals.var_t1_dn0 = assign94350_body56_e145558_d_n0;
            locals.var_t1_dn2 = assign94350_body56_e145558_d_n2;
            locals.var_t1_dn4 = assign94350_body56_e145558_d_n4;
            locals.var_t1_dn5 = assign94350_body56_e145558_d_n5;
            locals.var_t1_dn6 = assign94350_body56_e145558_d_n6;
            locals.var_t1_dn7 = assign94350_body56_e145558_d_n7;
            locals.var_t1_dn8 = assign94350_body56_e145558_d_n8;
            locals.var_t1_dn9 = assign94350_body56_e145558_d_n9;
            locals.var_t1_dn10 = assign94350_body56_e145558_d_n10;
            locals.var_t1_dn11 = assign94350_body56_e145558_d_n11;
            locals.var_t1_dn14 = assign94350_body56_e145558_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94350_body57_e145581, assign94350_body57_e145581_d_n0, assign94350_body57_e145581_d_n2, assign94350_body57_e145581_d_n4, assign94350_body57_e145581_d_n5, assign94350_body57_e145581_d_n6, assign94350_body57_e145581_d_n7, assign94350_body57_e145581_d_n8, assign94350_body57_e145581_d_n9, assign94350_body57_e145581_d_n10, assign94350_body57_e145581_d_n11, assign94350_body57_e145581_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94350_body57_e145578: f64 = (locals.var_t1 - locals.var_chi);
        let assign94350_body57_e145579: f64 = (locals.var_cfs1 * assign94350_body57_e145578);
        (assign94350_body57_e145579, ((locals.var_cfs1_dn0 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign94350_body57_e145578) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94350_body57_e145581;
            locals.var_fs01_dn0 = assign94350_body57_e145581_d_n0;
            locals.var_fs01_dn2 = assign94350_body57_e145581_d_n2;
            locals.var_fs01_dn4 = assign94350_body57_e145581_d_n4;
            locals.var_fs01_dn5 = assign94350_body57_e145581_d_n5;
            locals.var_fs01_dn6 = assign94350_body57_e145581_d_n6;
            locals.var_fs01_dn7 = assign94350_body57_e145581_d_n7;
            locals.var_fs01_dn8 = assign94350_body57_e145581_d_n8;
            locals.var_fs01_dn9 = assign94350_body57_e145581_d_n9;
            locals.var_fs01_dn10 = assign94350_body57_e145581_d_n10;
            locals.var_fs01_dn11 = assign94350_body57_e145581_d_n11;
            locals.var_fs01_dn14 = assign94350_body57_e145581_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94350_body58_e145604, assign94350_body58_e145604_d_n0, assign94350_body58_e145604_d_n2, assign94350_body58_e145604_d_n4, assign94350_body58_e145604_d_n5, assign94350_body58_e145604_d_n6, assign94350_body58_e145604_d_n7, assign94350_body58_e145604_d_n8, assign94350_body58_e145604_d_n9, assign94350_body58_e145604_d_n10, assign94350_body58_e145604_d_n11, assign94350_body58_e145604_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94350_body58_e145600: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94350_body58_e145602: f64 = (assign94350_body58_e145600 * locals.var_t1);
        (assign94350_body58_e145602, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign94350_body58_e145600 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94350_body58_e145604;
            locals.var_fs01_dps0_dn0 = assign94350_body58_e145604_d_n0;
            locals.var_fs01_dps0_dn2 = assign94350_body58_e145604_d_n2;
            locals.var_fs01_dps0_dn4 = assign94350_body58_e145604_d_n4;
            locals.var_fs01_dps0_dn5 = assign94350_body58_e145604_d_n5;
            locals.var_fs01_dps0_dn6 = assign94350_body58_e145604_d_n6;
            locals.var_fs01_dps0_dn7 = assign94350_body58_e145604_d_n7;
            locals.var_fs01_dps0_dn8 = assign94350_body58_e145604_d_n8;
            locals.var_fs01_dps0_dn9 = assign94350_body58_e145604_d_n9;
            locals.var_fs01_dps0_dn10 = assign94350_body58_e145604_d_n10;
            locals.var_fs01_dps0_dn11 = assign94350_body58_e145604_d_n11;
            locals.var_fs01_dps0_dn14 = assign94350_body58_e145604_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94350_body60_e145643, assign94350_body60_e145643_d_n0, assign94350_body60_e145643_d_n2, assign94350_body60_e145643_d_n4, assign94350_body60_e145643_d_n5, assign94350_body60_e145643_d_n6, assign94350_body60_e145643_d_n7, assign94350_body60_e145643_d_n8, assign94350_body60_e145643_d_n9, assign94350_body60_e145643_d_n10, assign94350_body60_e145643_d_n11, assign94350_body60_e145643_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        let assign94350_body60_e145640: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94350_body60_e145641: f64 = (assign94350_body60_e145640).exp();
        (assign94350_body60_e145641, (assign94350_body60_e145641 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94350_body60_e145641 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94350_body60_e145641 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94350_body60_e145641 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94350_body60_e145641 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94350_body60_e145641 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94350_body60_e145641 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94350_body60_e145641 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94350_body60_e145641 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94350_body60_e145641 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign94350_body60_e145641 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign94350_body60_e145643;
            locals.var_exp_bps0_dn0 = assign94350_body60_e145643_d_n0;
            locals.var_exp_bps0_dn2 = assign94350_body60_e145643_d_n2;
            locals.var_exp_bps0_dn4 = assign94350_body60_e145643_d_n4;
            locals.var_exp_bps0_dn5 = assign94350_body60_e145643_d_n5;
            locals.var_exp_bps0_dn6 = assign94350_body60_e145643_d_n6;
            locals.var_exp_bps0_dn7 = assign94350_body60_e145643_d_n7;
            locals.var_exp_bps0_dn8 = assign94350_body60_e145643_d_n8;
            locals.var_exp_bps0_dn9 = assign94350_body60_e145643_d_n9;
            locals.var_exp_bps0_dn10 = assign94350_body60_e145643_d_n10;
            locals.var_exp_bps0_dn11 = assign94350_body60_e145643_d_n11;
            locals.var_exp_bps0_dn14 = assign94350_body60_e145643_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94350_body61_e145668, assign94350_body61_e145668_d_n0, assign94350_body61_e145668_d_n2, assign94350_body61_e145668_d_n4, assign94350_body61_e145668_d_n5, assign94350_body61_e145668_d_n6, assign94350_body61_e145668_d_n7, assign94350_body61_e145668_d_n8, assign94350_body61_e145668_d_n9, assign94350_body61_e145668_d_n10, assign94350_body61_e145668_d_n11, assign94350_body61_e145668_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        let assign94350_body61_e145663: f64 = (locals.var_chi + 1.0);
        let assign94350_body61_e145664: f64 = (locals.var_exp_bvbs * assign94350_body61_e145663);
        let assign94350_body61_e145665: f64 = (locals.var_exp_bps0 - assign94350_body61_e145664);
        let assign94350_body61_e145666: f64 = (locals.var_cnst1over * assign94350_body61_e145665);
        (assign94350_body61_e145666, ((locals.var_cnst1over_dn0 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign94350_body61_e145665) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign94350_body61_e145663) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94350_body61_e145668;
            locals.var_fs01_dn0 = assign94350_body61_e145668_d_n0;
            locals.var_fs01_dn2 = assign94350_body61_e145668_d_n2;
            locals.var_fs01_dn4 = assign94350_body61_e145668_d_n4;
            locals.var_fs01_dn5 = assign94350_body61_e145668_d_n5;
            locals.var_fs01_dn6 = assign94350_body61_e145668_d_n6;
            locals.var_fs01_dn7 = assign94350_body61_e145668_d_n7;
            locals.var_fs01_dn8 = assign94350_body61_e145668_d_n8;
            locals.var_fs01_dn9 = assign94350_body61_e145668_d_n9;
            locals.var_fs01_dn10 = assign94350_body61_e145668_d_n10;
            locals.var_fs01_dn11 = assign94350_body61_e145668_d_n11;
            locals.var_fs01_dn14 = assign94350_body61_e145668_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94350_body62_e145691, assign94350_body62_e145691_d_n0, assign94350_body62_e145691_d_n2, assign94350_body62_e145691_d_n4, assign94350_body62_e145691_d_n5, assign94350_body62_e145691_d_n6, assign94350_body62_e145691_d_n7, assign94350_body62_e145691_d_n8, assign94350_body62_e145691_d_n9, assign94350_body62_e145691_d_n10, assign94350_body62_e145691_d_n11, assign94350_body62_e145691_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        let assign94350_body62_e145685: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94350_body62_e145688: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94350_body62_e145689: f64 = (assign94350_body62_e145685 * assign94350_body62_e145688);
        (assign94350_body62_e145689, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign94350_body62_e145688) + (assign94350_body62_e145685 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94350_body62_e145691;
            locals.var_fs01_dps0_dn0 = assign94350_body62_e145691_d_n0;
            locals.var_fs01_dps0_dn2 = assign94350_body62_e145691_d_n2;
            locals.var_fs01_dps0_dn4 = assign94350_body62_e145691_d_n4;
            locals.var_fs01_dps0_dn5 = assign94350_body62_e145691_d_n5;
            locals.var_fs01_dps0_dn6 = assign94350_body62_e145691_d_n6;
            locals.var_fs01_dps0_dn7 = assign94350_body62_e145691_d_n7;
            locals.var_fs01_dps0_dn8 = assign94350_body62_e145691_d_n8;
            locals.var_fs01_dps0_dn9 = assign94350_body62_e145691_d_n9;
            locals.var_fs01_dps0_dn10 = assign94350_body62_e145691_d_n10;
            locals.var_fs01_dps0_dn11 = assign94350_body62_e145691_d_n11;
            locals.var_fs01_dps0_dn14 = assign94350_body62_e145691_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94350_body63_e145694: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2193 = assign94350_body63_e145694;
            locals.var_guard2193_rv = 0.0;
            let (assign94350_body64_e145715, assign94350_body64_e145715_d_n0, assign94350_body64_e145715_d_n2, assign94350_body64_e145715_d_n4, assign94350_body64_e145715_d_n5, assign94350_body64_e145715_d_n6, assign94350_body64_e145715_d_n7, assign94350_body64_e145715_d_n8, assign94350_body64_e145715_d_n9, assign94350_body64_e145715_d_n10, assign94350_body64_e145715_d_n11, assign94350_body64_e145715_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2193 != 0.0)) {
        let assign94350_body64_e145710: f64 = (locals.var_fb * locals.var_fb);
        let assign94350_body64_e145712: f64 = (assign94350_body64_e145710 + locals.var_fs01);
        let assign94350_body64_e145713: f64 = (assign94350_body64_e145712).sqrt();
        (assign94350_body64_e145713, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign94350_body64_e145713)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign94350_body64_e145713)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94350_body64_e145715;
            locals.var_fs02_dn0 = assign94350_body64_e145715_d_n0;
            locals.var_fs02_dn2 = assign94350_body64_e145715_d_n2;
            locals.var_fs02_dn4 = assign94350_body64_e145715_d_n4;
            locals.var_fs02_dn5 = assign94350_body64_e145715_d_n5;
            locals.var_fs02_dn6 = assign94350_body64_e145715_d_n6;
            locals.var_fs02_dn7 = assign94350_body64_e145715_d_n7;
            locals.var_fs02_dn8 = assign94350_body64_e145715_d_n8;
            locals.var_fs02_dn9 = assign94350_body64_e145715_d_n9;
            locals.var_fs02_dn10 = assign94350_body64_e145715_d_n10;
            locals.var_fs02_dn11 = assign94350_body64_e145715_d_n11;
            locals.var_fs02_dn14 = assign94350_body64_e145715_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94350_body65_e145741, assign94350_body65_e145741_d_n0, assign94350_body65_e145741_d_n2, assign94350_body65_e145741_d_n4, assign94350_body65_e145741_d_n5, assign94350_body65_e145741_d_n6, assign94350_body65_e145741_d_n7, assign94350_body65_e145741_d_n8, assign94350_body65_e145741_d_n9, assign94350_body65_e145741_d_n10, assign94350_body65_e145741_d_n11, assign94350_body65_e145741_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2193 != 0.0)) {
        let assign94350_body65_e145732: f64 = (2.0 * locals.var_fb_dpss);
        let assign94350_body65_e145734: f64 = (assign94350_body65_e145732 * locals.var_fb);
        let assign94350_body65_e145736: f64 = (assign94350_body65_e145734 + locals.var_fs01_dps0);
        let assign94350_body65_e145737: f64 = (0.5 * assign94350_body65_e145736);
        let assign94350_body65_e145739: f64 = (assign94350_body65_e145737 / locals.var_fs02);
        (assign94350_body65_e145739, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn14) * locals.var_fb) + (assign94350_body65_e145732 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign94350_body65_e145737 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94350_body65_e145741;
            locals.var_fs02_dps0_dn0 = assign94350_body65_e145741_d_n0;
            locals.var_fs02_dps0_dn2 = assign94350_body65_e145741_d_n2;
            locals.var_fs02_dps0_dn4 = assign94350_body65_e145741_d_n4;
            locals.var_fs02_dps0_dn5 = assign94350_body65_e145741_d_n5;
            locals.var_fs02_dps0_dn6 = assign94350_body65_e145741_d_n6;
            locals.var_fs02_dps0_dn7 = assign94350_body65_e145741_d_n7;
            locals.var_fs02_dps0_dn8 = assign94350_body65_e145741_d_n8;
            locals.var_fs02_dps0_dn9 = assign94350_body65_e145741_d_n9;
            locals.var_fs02_dps0_dn10 = assign94350_body65_e145741_d_n10;
            locals.var_fs02_dps0_dn11 = assign94350_body65_e145741_d_n11;
            locals.var_fs02_dps0_dn14 = assign94350_body65_e145741_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94350_body67_e145777, assign94350_body67_e145777_d_n0, assign94350_body67_e145777_d_n2, assign94350_body67_e145777_d_n4, assign94350_body67_e145777_d_n5, assign94350_body67_e145777_d_n6, assign94350_body67_e145777_d_n7, assign94350_body67_e145777_d_n8, assign94350_body67_e145777_d_n9, assign94350_body67_e145777_d_n10, assign94350_body67_e145777_d_n11, assign94350_body67_e145777_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2193 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94350_body67_e145777;
            locals.var_fs02_dn0 = assign94350_body67_e145777_d_n0;
            locals.var_fs02_dn2 = assign94350_body67_e145777_d_n2;
            locals.var_fs02_dn4 = assign94350_body67_e145777_d_n4;
            locals.var_fs02_dn5 = assign94350_body67_e145777_d_n5;
            locals.var_fs02_dn6 = assign94350_body67_e145777_d_n6;
            locals.var_fs02_dn7 = assign94350_body67_e145777_d_n7;
            locals.var_fs02_dn8 = assign94350_body67_e145777_d_n8;
            locals.var_fs02_dn9 = assign94350_body67_e145777_d_n9;
            locals.var_fs02_dn10 = assign94350_body67_e145777_d_n10;
            locals.var_fs02_dn11 = assign94350_body67_e145777_d_n11;
            locals.var_fs02_dn14 = assign94350_body67_e145777_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94350_body68_e145794, assign94350_body68_e145794_d_n0, assign94350_body68_e145794_d_n2, assign94350_body68_e145794_d_n4, assign94350_body68_e145794_d_n5, assign94350_body68_e145794_d_n6, assign94350_body68_e145794_d_n7, assign94350_body68_e145794_d_n8, assign94350_body68_e145794_d_n9, assign94350_body68_e145794_d_n10, assign94350_body68_e145794_d_n11, assign94350_body68_e145794_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2190 == 0.0)) && (locals.var_guard2193 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94350_body68_e145794;
            locals.var_fs02_dps0_dn0 = assign94350_body68_e145794_d_n0;
            locals.var_fs02_dps0_dn2 = assign94350_body68_e145794_d_n2;
            locals.var_fs02_dps0_dn4 = assign94350_body68_e145794_d_n4;
            locals.var_fs02_dps0_dn5 = assign94350_body68_e145794_d_n5;
            locals.var_fs02_dps0_dn6 = assign94350_body68_e145794_d_n6;
            locals.var_fs02_dps0_dn7 = assign94350_body68_e145794_d_n7;
            locals.var_fs02_dps0_dn8 = assign94350_body68_e145794_d_n8;
            locals.var_fs02_dps0_dn9 = assign94350_body68_e145794_d_n9;
            locals.var_fs02_dps0_dn10 = assign94350_body68_e145794_d_n10;
            locals.var_fs02_dps0_dn11 = assign94350_body68_e145794_d_n11;
            locals.var_fs02_dps0_dn14 = assign94350_body68_e145794_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94350_body69_e145812, assign94350_body69_e145812_d_n0, assign94350_body69_e145812_d_n2, assign94350_body69_e145812_d_n4, assign94350_body69_e145812_d_n5, assign94350_body69_e145812_d_n6, assign94350_body69_e145812_d_n7, assign94350_body69_e145812_d_n8, assign94350_body69_e145812_d_n9, assign94350_body69_e145812_d_n10, assign94350_body69_e145812_d_n11, assign94350_body69_e145812_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body69_e145804: f64 = (-locals.var_vgpld);
        let assign94350_body69_e145806: f64 = (assign94350_body69_e145804 + locals.var_ps0ld);
        let assign94350_body69_e145809: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94350_body69_e145810: f64 = (assign94350_body69_e145806 + assign94350_body69_e145809);
        (assign94350_body69_e145810, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign94350_body69_e145812;
            locals.var_fs0_dn0 = assign94350_body69_e145812_d_n0;
            locals.var_fs0_dn2 = assign94350_body69_e145812_d_n2;
            locals.var_fs0_dn4 = assign94350_body69_e145812_d_n4;
            locals.var_fs0_dn5 = assign94350_body69_e145812_d_n5;
            locals.var_fs0_dn6 = assign94350_body69_e145812_d_n6;
            locals.var_fs0_dn7 = assign94350_body69_e145812_d_n7;
            locals.var_fs0_dn8 = assign94350_body69_e145812_d_n8;
            locals.var_fs0_dn9 = assign94350_body69_e145812_d_n9;
            locals.var_fs0_dn10 = assign94350_body69_e145812_d_n10;
            locals.var_fs0_dn11 = assign94350_body69_e145812_d_n11;
            locals.var_fs0_dn14 = assign94350_body69_e145812_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign94350_body70_e145827, assign94350_body70_e145827_d_n0, assign94350_body70_e145827_d_n2, assign94350_body70_e145827_d_n4, assign94350_body70_e145827_d_n5, assign94350_body70_e145827_d_n6, assign94350_body70_e145827_d_n7, assign94350_body70_e145827_d_n8, assign94350_body70_e145827_d_n9, assign94350_body70_e145827_d_n10, assign94350_body70_e145827_d_n11, assign94350_body70_e145827_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body70_e145824: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94350_body70_e145825: f64 = (1.0 + assign94350_body70_e145824);
        (assign94350_body70_e145825, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign94350_body70_e145827;
            locals.var_fs0_dps0_dn0 = assign94350_body70_e145827_d_n0;
            locals.var_fs0_dps0_dn2 = assign94350_body70_e145827_d_n2;
            locals.var_fs0_dps0_dn4 = assign94350_body70_e145827_d_n4;
            locals.var_fs0_dps0_dn5 = assign94350_body70_e145827_d_n5;
            locals.var_fs0_dps0_dn6 = assign94350_body70_e145827_d_n6;
            locals.var_fs0_dps0_dn7 = assign94350_body70_e145827_d_n7;
            locals.var_fs0_dps0_dn8 = assign94350_body70_e145827_d_n8;
            locals.var_fs0_dps0_dn9 = assign94350_body70_e145827_d_n9;
            locals.var_fs0_dps0_dn10 = assign94350_body70_e145827_d_n10;
            locals.var_fs0_dps0_dn11 = assign94350_body70_e145827_d_n11;
            locals.var_fs0_dps0_dn14 = assign94350_body70_e145827_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94350_body71_e145830: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2194 = assign94350_body71_e145830;
            locals.var_guard2194_rv = 0.0;
            let (assign94350_body72_e145845,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 != 0.0)) {
        let assign94350_body72_e145843: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94350_body72_e145843,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94350_body72_e145845;
            locals.var_lp_s0_rv = 0.0;
            let (assign94350_body73_e145862, assign94350_body73_e145862_d_n0, assign94350_body73_e145862_d_n2, assign94350_body73_e145862_d_n4, assign94350_body73_e145862_d_n5, assign94350_body73_e145862_d_n6, assign94350_body73_e145862_d_n7, assign94350_body73_e145862_d_n8, assign94350_body73_e145862_d_n9, assign94350_body73_e145862_d_n10, assign94350_body73_e145862_d_n11, assign94350_body73_e145862_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94350_body73_e145858: f64 = (-locals.var_fs0);
        let assign94350_body73_e145860: f64 = (assign94350_body73_e145858 / locals.var_fs0_dps0);
        (assign94350_body73_e145860, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign94350_body73_e145858 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94350_body73_e145862;
            locals.var_dps0_dn0 = assign94350_body73_e145862_d_n0;
            locals.var_dps0_dn2 = assign94350_body73_e145862_d_n2;
            locals.var_dps0_dn4 = assign94350_body73_e145862_d_n4;
            locals.var_dps0_dn5 = assign94350_body73_e145862_d_n5;
            locals.var_dps0_dn6 = assign94350_body73_e145862_d_n6;
            locals.var_dps0_dn7 = assign94350_body73_e145862_d_n7;
            locals.var_dps0_dn8 = assign94350_body73_e145862_d_n8;
            locals.var_dps0_dn9 = assign94350_body73_e145862_d_n9;
            locals.var_dps0_dn10 = assign94350_body73_e145862_d_n10;
            locals.var_dps0_dn11 = assign94350_body73_e145862_d_n11;
            locals.var_dps0_dn14 = assign94350_body73_e145862_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94350_body74_e145889, assign94350_body74_e145889_d_n0, assign94350_body74_e145889_d_n2, assign94350_body74_e145889_d_n4, assign94350_body74_e145889_d_n5, assign94350_body74_e145889_d_n6, assign94350_body74_e145889_d_n7, assign94350_body74_e145889_d_n8, assign94350_body74_e145889_d_n9, assign94350_body74_e145889_d_n10, assign94350_body74_e145889_d_n11, assign94350_body74_e145889_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94350_body74_e145876: f64 = (0.5 * 0.1);
        let assign94350_body74_e145880: f64 = (locals.var_ps0ld).abs();
        let (assign94350_body74_e145885, assign94350_body74_e145885_d_n0, assign94350_body74_e145885_d_n2, assign94350_body74_e145885_d_n4, assign94350_body74_e145885_d_n5, assign94350_body74_e145885_d_n6, assign94350_body74_e145885_d_n7, assign94350_body74_e145885_d_n8, assign94350_body74_e145885_d_n9, assign94350_body74_e145885_d_n10, assign94350_body74_e145885_d_n11, assign94350_body74_e145885_d_n14,) = {
            if (1.0 >= assign94350_body74_e145880) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94350_body74_e145884: f64 = (locals.var_ps0ld).abs();
                (assign94350_body74_e145884, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign94350_body74_e145886: f64 = (1.0 + assign94350_body74_e145885);
        let assign94350_body74_e145887: f64 = (assign94350_body74_e145876 * assign94350_body74_e145886);
        (assign94350_body74_e145887, (assign94350_body74_e145876 * assign94350_body74_e145885_d_n0), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n2), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n4), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n5), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n6), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n7), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n8), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n9), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n10), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n11), (assign94350_body74_e145876 * assign94350_body74_e145885_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign94350_body74_e145889;
            locals.var_dplim_dn0 = assign94350_body74_e145889_d_n0;
            locals.var_dplim_dn2 = assign94350_body74_e145889_d_n2;
            locals.var_dplim_dn4 = assign94350_body74_e145889_d_n4;
            locals.var_dplim_dn5 = assign94350_body74_e145889_d_n5;
            locals.var_dplim_dn6 = assign94350_body74_e145889_d_n6;
            locals.var_dplim_dn7 = assign94350_body74_e145889_d_n7;
            locals.var_dplim_dn8 = assign94350_body74_e145889_d_n8;
            locals.var_dplim_dn9 = assign94350_body74_e145889_d_n9;
            locals.var_dplim_dn10 = assign94350_body74_e145889_d_n10;
            locals.var_dplim_dn11 = assign94350_body74_e145889_d_n11;
            locals.var_dplim_dn14 = assign94350_body74_e145889_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign94350_body75_e145891: f64 = (locals.var_dps0).abs();
            let assign94350_body75_e145893: f64 = if assign94350_body75_e145891 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2195 = assign94350_body75_e145893;
            locals.var_guard2195_rv = 0.0;
            let (assign94350_body76_e145917, assign94350_body76_e145917_d_n0, assign94350_body76_e145917_d_n2, assign94350_body76_e145917_d_n4, assign94350_body76_e145917_d_n5, assign94350_body76_e145917_d_n6, assign94350_body76_e145917_d_n7, assign94350_body76_e145917_d_n8, assign94350_body76_e145917_d_n9, assign94350_body76_e145917_d_n10, assign94350_body76_e145917_d_n11, assign94350_body76_e145917_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 == 0.0)) && (locals.var_guard2195 != 0.0)) {
        let (assign94350_body76_e145914,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94350_body76_e145913: f64 = (-1.0);
                (assign94350_body76_e145913,)
            }
        };
        let assign94350_body76_e145915: f64 = (locals.var_dplim * assign94350_body76_e145914);
        (assign94350_body76_e145915, (locals.var_dplim_dn0 * assign94350_body76_e145914), (locals.var_dplim_dn2 * assign94350_body76_e145914), (locals.var_dplim_dn4 * assign94350_body76_e145914), (locals.var_dplim_dn5 * assign94350_body76_e145914), (locals.var_dplim_dn6 * assign94350_body76_e145914), (locals.var_dplim_dn7 * assign94350_body76_e145914), (locals.var_dplim_dn8 * assign94350_body76_e145914), (locals.var_dplim_dn9 * assign94350_body76_e145914), (locals.var_dplim_dn10 * assign94350_body76_e145914), (locals.var_dplim_dn11 * assign94350_body76_e145914), (locals.var_dplim_dn14 * assign94350_body76_e145914),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94350_body76_e145917;
            locals.var_dps0_dn0 = assign94350_body76_e145917_d_n0;
            locals.var_dps0_dn2 = assign94350_body76_e145917_d_n2;
            locals.var_dps0_dn4 = assign94350_body76_e145917_d_n4;
            locals.var_dps0_dn5 = assign94350_body76_e145917_d_n5;
            locals.var_dps0_dn6 = assign94350_body76_e145917_d_n6;
            locals.var_dps0_dn7 = assign94350_body76_e145917_d_n7;
            locals.var_dps0_dn8 = assign94350_body76_e145917_d_n8;
            locals.var_dps0_dn9 = assign94350_body76_e145917_d_n9;
            locals.var_dps0_dn10 = assign94350_body76_e145917_d_n10;
            locals.var_dps0_dn11 = assign94350_body76_e145917_d_n11;
            locals.var_dps0_dn14 = assign94350_body76_e145917_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94350_body77_e145933, assign94350_body77_e145933_d_n0, assign94350_body77_e145933_d_n2, assign94350_body77_e145933_d_n4, assign94350_body77_e145933_d_n5, assign94350_body77_e145933_d_n6, assign94350_body77_e145933_d_n7, assign94350_body77_e145933_d_n8, assign94350_body77_e145933_d_n9, assign94350_body77_e145933_d_n10, assign94350_body77_e145933_d_n11, assign94350_body77_e145933_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94350_body77_e145931: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94350_body77_e145931, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign94350_body77_e145933;
            locals.var_ps0ld_dn0 = assign94350_body77_e145933_d_n0;
            locals.var_ps0ld_dn2 = assign94350_body77_e145933_d_n2;
            locals.var_ps0ld_dn4 = assign94350_body77_e145933_d_n4;
            locals.var_ps0ld_dn5 = assign94350_body77_e145933_d_n5;
            locals.var_ps0ld_dn6 = assign94350_body77_e145933_d_n6;
            locals.var_ps0ld_dn7 = assign94350_body77_e145933_d_n7;
            locals.var_ps0ld_dn8 = assign94350_body77_e145933_d_n8;
            locals.var_ps0ld_dn9 = assign94350_body77_e145933_d_n9;
            locals.var_ps0ld_dn10 = assign94350_body77_e145933_d_n10;
            locals.var_ps0ld_dn11 = assign94350_body77_e145933_d_n11;
            locals.var_ps0ld_dn14 = assign94350_body77_e145933_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign94350_body78_e145935: f64 = (locals.var_dps0).abs();
            let assign94350_body78_e145939: f64 = (locals.var_fs0).abs();
            let assign94350_body78_e145942: f64 = if ((assign94350_body78_e145935 <= 1e-12) && (assign94350_body78_e145939 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2196 = assign94350_body78_e145942;
            locals.var_guard2196_rv = 0.0;
            let (assign94350_body79_e145958,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) && (locals.var_guard2194 == 0.0)) && (locals.var_guard2196 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94350_body79_e145958;
            locals.var_flg_conv_rv = 0.0;
            let (assign94350_body80_e145971,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94350_body80_e145969: f64 = (locals.var_lp_s0 + 1.0);
        (assign94350_body80_e145969,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94350_body80_e145971;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94370_e145987, assign94370_e145987_d_n0, assign94370_e145987_d_n2, assign94370_e145987_d_n4, assign94370_e145987_d_n5, assign94370_e145987_d_n6, assign94370_e145987_d_n7, assign94370_e145987_d_n8, assign94370_e145987_d_n9, assign94370_e145987_d_n10, assign94370_e145987_d_n11, assign94370_e145987_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94370_e145985: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94370_e145985, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2115, locals.var_wdld__blk2115_dn0, locals.var_wdld__blk2115_dn2, locals.var_wdld__blk2115_dn4, locals.var_wdld__blk2115_dn5, locals.var_wdld__blk2115_dn6, locals.var_wdld__blk2115_dn7, locals.var_wdld__blk2115_dn8, locals.var_wdld__blk2115_dn9, locals.var_wdld__blk2115_dn10, locals.var_wdld__blk2115_dn11, locals.var_wdld__blk2115_dn14,)
    }
};
        locals.var_wdld__blk2115 = assign94370_e145987;
        locals.var_wdld__blk2115_dn0 = assign94370_e145987_d_n0;
        locals.var_wdld__blk2115_dn2 = assign94370_e145987_d_n2;
        locals.var_wdld__blk2115_dn4 = assign94370_e145987_d_n4;
        locals.var_wdld__blk2115_dn5 = assign94370_e145987_d_n5;
        locals.var_wdld__blk2115_dn6 = assign94370_e145987_d_n6;
        locals.var_wdld__blk2115_dn7 = assign94370_e145987_d_n7;
        locals.var_wdld__blk2115_dn8 = assign94370_e145987_d_n8;
        locals.var_wdld__blk2115_dn9 = assign94370_e145987_d_n9;
        locals.var_wdld__blk2115_dn10 = assign94370_e145987_d_n10;
        locals.var_wdld__blk2115_dn11 = assign94370_e145987_d_n11;
        locals.var_wdld__blk2115_dn14 = assign94370_e145987_d_n14;
        locals.var_wdld__blk2115_rv = 0.0;

        let (assign94380_e146000, assign94380_e146000_d_n0, assign94380_e146000_d_n2, assign94380_e146000_d_n4, assign94380_e146000_d_n5, assign94380_e146000_d_n6, assign94380_e146000_d_n7, assign94380_e146000_d_n8, assign94380_e146000_d_n9, assign94380_e146000_d_n10, assign94380_e146000_d_n11, assign94380_e146000_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94380_e145998: f64 = (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115);
        (assign94380_e145998, (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn0), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn2), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn4), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn5), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn6), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn7), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn8), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn9), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn10), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn11), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2116, locals.var_q_dep_ld__blk2116_dn0, locals.var_q_dep_ld__blk2116_dn2, locals.var_q_dep_ld__blk2116_dn4, locals.var_q_dep_ld__blk2116_dn5, locals.var_q_dep_ld__blk2116_dn6, locals.var_q_dep_ld__blk2116_dn7, locals.var_q_dep_ld__blk2116_dn8, locals.var_q_dep_ld__blk2116_dn9, locals.var_q_dep_ld__blk2116_dn10, locals.var_q_dep_ld__blk2116_dn11, locals.var_q_dep_ld__blk2116_dn14,)
    }
};
        locals.var_q_dep_ld__blk2116 = assign94380_e146000;
        locals.var_q_dep_ld__blk2116_dn0 = assign94380_e146000_d_n0;
        locals.var_q_dep_ld__blk2116_dn2 = assign94380_e146000_d_n2;
        locals.var_q_dep_ld__blk2116_dn4 = assign94380_e146000_d_n4;
        locals.var_q_dep_ld__blk2116_dn5 = assign94380_e146000_d_n5;
        locals.var_q_dep_ld__blk2116_dn6 = assign94380_e146000_d_n6;
        locals.var_q_dep_ld__blk2116_dn7 = assign94380_e146000_d_n7;
        locals.var_q_dep_ld__blk2116_dn8 = assign94380_e146000_d_n8;
        locals.var_q_dep_ld__blk2116_dn9 = assign94380_e146000_d_n9;
        locals.var_q_dep_ld__blk2116_dn10 = assign94380_e146000_d_n10;
        locals.var_q_dep_ld__blk2116_dn11 = assign94380_e146000_d_n11;
        locals.var_q_dep_ld__blk2116_dn14 = assign94380_e146000_d_n14;
        locals.var_q_dep_ld__blk2116_rv = 0.0;

        let (assign94390_e146017, assign94390_e146017_d_n0, assign94390_e146017_d_n2, assign94390_e146017_d_n4, assign94390_e146017_d_n5, assign94390_e146017_d_n6, assign94390_e146017_d_n7, assign94390_e146017_d_n8, assign94390_e146017_d_n9, assign94390_e146017_d_n10, assign94390_e146017_d_n11, assign94390_e146017_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94390_e146011: f64 = (locals.var_q_dep_ld__blk2116 / locals.var_cnst0over_func);
        let assign94390_e146014: f64 = (10.0 * 2.220446049250313e-16);
        let assign94390_e146015: f64 = (assign94390_e146011 + assign94390_e146014);
        (assign94390_e146015, (((locals.var_q_dep_ld__blk2116_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign94390_e146017;
        locals.var_xi0p12_dn0 = assign94390_e146017_d_n0;
        locals.var_xi0p12_dn2 = assign94390_e146017_d_n2;
        locals.var_xi0p12_dn4 = assign94390_e146017_d_n4;
        locals.var_xi0p12_dn5 = assign94390_e146017_d_n5;
        locals.var_xi0p12_dn6 = assign94390_e146017_d_n6;
        locals.var_xi0p12_dn7 = assign94390_e146017_d_n7;
        locals.var_xi0p12_dn8 = assign94390_e146017_d_n8;
        locals.var_xi0p12_dn9 = assign94390_e146017_d_n9;
        locals.var_xi0p12_dn10 = assign94390_e146017_d_n10;
        locals.var_xi0p12_dn11 = assign94390_e146017_d_n11;
        locals.var_xi0p12_dn14 = assign94390_e146017_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign94400_e146030, assign94400_e146030_d_n0, assign94400_e146030_d_n2, assign94400_e146030_d_n4, assign94400_e146030_d_n5, assign94400_e146030_d_n6, assign94400_e146030_d_n7, assign94400_e146030_d_n8, assign94400_e146030_d_n9, assign94400_e146030_d_n10, assign94400_e146030_d_n11, assign94400_e146030_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94400_e146028: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94400_e146028, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign94400_e146030;
        locals.var_qbuld_dn0 = assign94400_e146030_d_n0;
        locals.var_qbuld_dn2 = assign94400_e146030_d_n2;
        locals.var_qbuld_dn4 = assign94400_e146030_d_n4;
        locals.var_qbuld_dn5 = assign94400_e146030_d_n5;
        locals.var_qbuld_dn6 = assign94400_e146030_d_n6;
        locals.var_qbuld_dn7 = assign94400_e146030_d_n7;
        locals.var_qbuld_dn8 = assign94400_e146030_d_n8;
        locals.var_qbuld_dn9 = assign94400_e146030_d_n9;
        locals.var_qbuld_dn10 = assign94400_e146030_d_n10;
        locals.var_qbuld_dn11 = assign94400_e146030_d_n11;
        locals.var_qbuld_dn14 = assign94400_e146030_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign94410_e146045, assign94410_e146045_d_n0, assign94410_e146045_d_n2, assign94410_e146045_d_n4, assign94410_e146045_d_n5, assign94410_e146045_d_n6, assign94410_e146045_d_n7, assign94410_e146045_d_n8, assign94410_e146045_d_n9, assign94410_e146045_d_n10, assign94410_e146045_d_n11, assign94410_e146045_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94410_e146042: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94410_e146043: f64 = (1.0 / assign94410_e146042);
        (assign94410_e146043, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign94410_e146042 * assign94410_e146042))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign94410_e146042 * assign94410_e146042))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94410_e146045;
        locals.var_t1_dn0 = assign94410_e146045_d_n0;
        locals.var_t1_dn2 = assign94410_e146045_d_n2;
        locals.var_t1_dn4 = assign94410_e146045_d_n4;
        locals.var_t1_dn5 = assign94410_e146045_d_n5;
        locals.var_t1_dn6 = assign94410_e146045_d_n6;
        locals.var_t1_dn7 = assign94410_e146045_d_n7;
        locals.var_t1_dn8 = assign94410_e146045_d_n8;
        locals.var_t1_dn9 = assign94410_e146045_d_n9;
        locals.var_t1_dn10 = assign94410_e146045_d_n10;
        locals.var_t1_dn11 = assign94410_e146045_d_n11;
        locals.var_t1_dn14 = assign94410_e146045_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94420_e146060, assign94420_e146060_d_n0, assign94420_e146060_d_n2, assign94420_e146060_d_n4, assign94420_e146060_d_n5, assign94420_e146060_d_n6, assign94420_e146060_d_n7, assign94420_e146060_d_n8, assign94420_e146060_d_n9, assign94420_e146060_d_n10, assign94420_e146060_d_n11, assign94420_e146060_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94420_e146056: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94420_e146058: f64 = (assign94420_e146056 * locals.var_t1);
        (assign94420_e146058, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign94420_e146056 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94420_e146060;
        locals.var_qiuld_dn0 = assign94420_e146060_d_n0;
        locals.var_qiuld_dn2 = assign94420_e146060_d_n2;
        locals.var_qiuld_dn4 = assign94420_e146060_d_n4;
        locals.var_qiuld_dn5 = assign94420_e146060_d_n5;
        locals.var_qiuld_dn6 = assign94420_e146060_d_n6;
        locals.var_qiuld_dn7 = assign94420_e146060_d_n7;
        locals.var_qiuld_dn8 = assign94420_e146060_d_n8;
        locals.var_qiuld_dn9 = assign94420_e146060_d_n9;
        locals.var_qiuld_dn10 = assign94420_e146060_d_n10;
        locals.var_qiuld_dn11 = assign94420_e146060_d_n11;
        locals.var_qiuld_dn14 = assign94420_e146060_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign94430_e146073, assign94430_e146073_d_n0, assign94430_e146073_d_n2, assign94430_e146073_d_n4, assign94430_e146073_d_n5, assign94430_e146073_d_n6, assign94430_e146073_d_n7, assign94430_e146073_d_n8, assign94430_e146073_d_n9, assign94430_e146073_d_n10, assign94430_e146073_d_n11, assign94430_e146073_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        let assign94430_e146071: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94430_e146071, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign94430_e146073;
        locals.var_qsuld_dn0 = assign94430_e146073_d_n0;
        locals.var_qsuld_dn2 = assign94430_e146073_d_n2;
        locals.var_qsuld_dn4 = assign94430_e146073_d_n4;
        locals.var_qsuld_dn5 = assign94430_e146073_d_n5;
        locals.var_qsuld_dn6 = assign94430_e146073_d_n6;
        locals.var_qsuld_dn7 = assign94430_e146073_d_n7;
        locals.var_qsuld_dn8 = assign94430_e146073_d_n8;
        locals.var_qsuld_dn9 = assign94430_e146073_d_n9;
        locals.var_qsuld_dn10 = assign94430_e146073_d_n10;
        locals.var_qsuld_dn11 = assign94430_e146073_d_n11;
        locals.var_qsuld_dn14 = assign94430_e146073_d_n14;
        locals.var_qsuld_rv = 0.0;

        let assign94440_e146076: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2198 = assign94440_e146076;
        locals.var_guard2198_rv = 0.0;

        let (assign94450_e146088, assign94450_e146088_d_n0, assign94450_e146088_d_n2, assign94450_e146088_d_n4, assign94450_e146088_d_n5, assign94450_e146088_d_n6, assign94450_e146088_d_n7, assign94450_e146088_d_n8, assign94450_e146088_d_n9, assign94450_e146088_d_n10, assign94450_e146088_d_n11, assign94450_e146088_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94450_e146084: f64 = (-locals.var_vxbgmtcl);
        let assign94450_e146085: f64 = (locals.var_beta * assign94450_e146084);
        let assign94450_e146086: f64 = (assign94450_e146085).exp();
        (assign94450_e146086, (assign94450_e146086 * ((locals.var_beta_dn0 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign94450_e146086 * ((locals.var_beta_dn2 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign94450_e146086 * ((locals.var_beta_dn4 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign94450_e146086 * ((locals.var_beta_dn5 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign94450_e146086 * ((locals.var_beta_dn6 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign94450_e146086 * ((locals.var_beta_dn7 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign94450_e146086 * ((locals.var_beta_dn8 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign94450_e146086 * ((locals.var_beta_dn9 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign94450_e146086 * ((locals.var_beta_dn10 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign94450_e146086 * ((locals.var_beta_dn11 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (assign94450_e146086 * ((locals.var_beta_dn14 * assign94450_e146084) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign94450_e146088;
        locals.var_exp_bvbs_dn0 = assign94450_e146088_d_n0;
        locals.var_exp_bvbs_dn2 = assign94450_e146088_d_n2;
        locals.var_exp_bvbs_dn4 = assign94450_e146088_d_n4;
        locals.var_exp_bvbs_dn5 = assign94450_e146088_d_n5;
        locals.var_exp_bvbs_dn6 = assign94450_e146088_d_n6;
        locals.var_exp_bvbs_dn7 = assign94450_e146088_d_n7;
        locals.var_exp_bvbs_dn8 = assign94450_e146088_d_n8;
        locals.var_exp_bvbs_dn9 = assign94450_e146088_d_n9;
        locals.var_exp_bvbs_dn10 = assign94450_e146088_d_n10;
        locals.var_exp_bvbs_dn11 = assign94450_e146088_d_n11;
        locals.var_exp_bvbs_dn14 = assign94450_e146088_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign94460_e146098, assign94460_e146098_d_n0, assign94460_e146098_d_n2, assign94460_e146098_d_n4, assign94460_e146098_d_n5, assign94460_e146098_d_n6, assign94460_e146098_d_n7, assign94460_e146098_d_n8, assign94460_e146098_d_n9, assign94460_e146098_d_n10, assign94460_e146098_d_n11, assign94460_e146098_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94460_e146096: f64 = (locals.var_nin / locals.var_nover_func);
        (assign94460_e146096, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94460_e146098;
        locals.var_t0_dn0 = assign94460_e146098_d_n0;
        locals.var_t0_dn2 = assign94460_e146098_d_n2;
        locals.var_t0_dn4 = assign94460_e146098_d_n4;
        locals.var_t0_dn5 = assign94460_e146098_d_n5;
        locals.var_t0_dn6 = assign94460_e146098_d_n6;
        locals.var_t0_dn7 = assign94460_e146098_d_n7;
        locals.var_t0_dn8 = assign94460_e146098_d_n8;
        locals.var_t0_dn9 = assign94460_e146098_d_n9;
        locals.var_t0_dn10 = assign94460_e146098_d_n10;
        locals.var_t0_dn11 = assign94460_e146098_d_n11;
        locals.var_t0_dn14 = assign94460_e146098_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94470_e146108, assign94470_e146108_d_n0, assign94470_e146108_d_n2, assign94470_e146108_d_n4, assign94470_e146108_d_n5, assign94470_e146108_d_n6, assign94470_e146108_d_n7, assign94470_e146108_d_n8, assign94470_e146108_d_n9, assign94470_e146108_d_n10, assign94470_e146108_d_n11, assign94470_e146108_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94470_e146106: f64 = (locals.var_t0 * locals.var_t0);
        (assign94470_e146106, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign94470_e146108;
        locals.var_cnst1over_dn0 = assign94470_e146108_d_n0;
        locals.var_cnst1over_dn2 = assign94470_e146108_d_n2;
        locals.var_cnst1over_dn4 = assign94470_e146108_d_n4;
        locals.var_cnst1over_dn5 = assign94470_e146108_d_n5;
        locals.var_cnst1over_dn6 = assign94470_e146108_d_n6;
        locals.var_cnst1over_dn7 = assign94470_e146108_d_n7;
        locals.var_cnst1over_dn8 = assign94470_e146108_d_n8;
        locals.var_cnst1over_dn9 = assign94470_e146108_d_n9;
        locals.var_cnst1over_dn10 = assign94470_e146108_d_n10;
        locals.var_cnst1over_dn11 = assign94470_e146108_d_n11;
        locals.var_cnst1over_dn14 = assign94470_e146108_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let (assign94480_e146118, assign94480_e146118_d_n0, assign94480_e146118_d_n2, assign94480_e146118_d_n4, assign94480_e146118_d_n5, assign94480_e146118_d_n6, assign94480_e146118_d_n7, assign94480_e146118_d_n8, assign94480_e146118_d_n9, assign94480_e146118_d_n10, assign94480_e146118_d_n11, assign94480_e146118_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94480_e146116: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign94480_e146116, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign94480_e146118;
        locals.var_cfs1_dn0 = assign94480_e146118_d_n0;
        locals.var_cfs1_dn2 = assign94480_e146118_d_n2;
        locals.var_cfs1_dn4 = assign94480_e146118_d_n4;
        locals.var_cfs1_dn5 = assign94480_e146118_d_n5;
        locals.var_cfs1_dn6 = assign94480_e146118_d_n6;
        locals.var_cfs1_dn7 = assign94480_e146118_d_n7;
        locals.var_cfs1_dn8 = assign94480_e146118_d_n8;
        locals.var_cfs1_dn9 = assign94480_e146118_d_n9;
        locals.var_cfs1_dn10 = assign94480_e146118_d_n10;
        locals.var_cfs1_dn11 = assign94480_e146118_d_n11;
        locals.var_cfs1_dn14 = assign94480_e146118_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign94490_e146126, assign94490_e146126_d_n0, assign94490_e146126_d_n2, assign94490_e146126_d_n4, assign94490_e146126_d_n5, assign94490_e146126_d_n6, assign94490_e146126_d_n7, assign94490_e146126_d_n8, assign94490_e146126_d_n9, assign94490_e146126_d_n10, assign94490_e146126_d_n11, assign94490_e146126_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        (locals.var_ps0ld_ini__blk2124, locals.var_ps0ld_ini__blk2124_dn0, locals.var_ps0ld_ini__blk2124_dn2, locals.var_ps0ld_ini__blk2124_dn4, locals.var_ps0ld_ini__blk2124_dn5, locals.var_ps0ld_ini__blk2124_dn6, locals.var_ps0ld_ini__blk2124_dn7, locals.var_ps0ld_ini__blk2124_dn8, locals.var_ps0ld_ini__blk2124_dn9, locals.var_ps0ld_ini__blk2124_dn10, locals.var_ps0ld_ini__blk2124_dn11, locals.var_ps0ld_ini__blk2124_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign94490_e146126;
        locals.var_ps0ld_dn0 = assign94490_e146126_d_n0;
        locals.var_ps0ld_dn2 = assign94490_e146126_d_n2;
        locals.var_ps0ld_dn4 = assign94490_e146126_d_n4;
        locals.var_ps0ld_dn5 = assign94490_e146126_d_n5;
        locals.var_ps0ld_dn6 = assign94490_e146126_d_n6;
        locals.var_ps0ld_dn7 = assign94490_e146126_d_n7;
        locals.var_ps0ld_dn8 = assign94490_e146126_d_n8;
        locals.var_ps0ld_dn9 = assign94490_e146126_d_n9;
        locals.var_ps0ld_dn10 = assign94490_e146126_d_n10;
        locals.var_ps0ld_dn11 = assign94490_e146126_d_n11;
        locals.var_ps0ld_dn14 = assign94490_e146126_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign94500_e146134,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign94500_e146134;
        locals.var_flg_conv_rv = 0.0;

        let (assign94510_e146149, assign94510_e146149_d_n0, assign94510_e146149_d_n2, assign94510_e146149_d_n4, assign94510_e146149_d_n5, assign94510_e146149_d_n6, assign94510_e146149_d_n7, assign94510_e146149_d_n8, assign94510_e146149_d_n9, assign94510_e146149_d_n10, assign94510_e146149_d_n11, assign94510_e146149_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94510_e146143: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2117);
        let assign94510_e146145: f64 = (assign94510_e146143 * locals.var_beta_inv);
        let assign94510_e146146: f64 = (2.0 * assign94510_e146145);
        let assign94510_e146147: f64 = (assign94510_e146146).sqrt();
        (assign94510_e146147, ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn0)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn2)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn4)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn5)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn6)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn7)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn8)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn9)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn10)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn11)) / (2.0 * assign94510_e146147)), ((2.0 * (assign94510_e146143 * locals.var_beta_inv_dn14)) / (2.0 * assign94510_e146147)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign94510_e146149;
        locals.var_c_w_ld_dn0 = assign94510_e146149_d_n0;
        locals.var_c_w_ld_dn2 = assign94510_e146149_d_n2;
        locals.var_c_w_ld_dn4 = assign94510_e146149_d_n4;
        locals.var_c_w_ld_dn5 = assign94510_e146149_d_n5;
        locals.var_c_w_ld_dn6 = assign94510_e146149_d_n6;
        locals.var_c_w_ld_dn7 = assign94510_e146149_d_n7;
        locals.var_c_w_ld_dn8 = assign94510_e146149_d_n8;
        locals.var_c_w_ld_dn9 = assign94510_e146149_d_n9;
        locals.var_c_w_ld_dn10 = assign94510_e146149_d_n10;
        locals.var_c_w_ld_dn11 = assign94510_e146149_d_n11;
        locals.var_c_w_ld_dn14 = assign94510_e146149_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign94520_e146152: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2199 = assign94520_e146152;
        locals.var_guard2199_rv = 0.0;

        let (assign94530_e146164, assign94530_e146164_d_n0, assign94530_e146164_d_n2, assign94530_e146164_d_n4, assign94530_e146164_d_n5, assign94530_e146164_d_n6, assign94530_e146164_d_n7, assign94530_e146164_d_n8, assign94530_e146164_d_n9, assign94530_e146164_d_n10, assign94530_e146164_d_n11, assign94530_e146164_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 != 0.0)) {
        let assign94530_e146162: f64 = (p.p334 - locals.var_wdep_func);
        (assign94530_e146162, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94530_e146164;
        locals.var_t2_dn0 = assign94530_e146164_d_n0;
        locals.var_t2_dn2 = assign94530_e146164_d_n2;
        locals.var_t2_dn4 = assign94530_e146164_d_n4;
        locals.var_t2_dn5 = assign94530_e146164_d_n5;
        locals.var_t2_dn6 = assign94530_e146164_d_n6;
        locals.var_t2_dn7 = assign94530_e146164_d_n7;
        locals.var_t2_dn8 = assign94530_e146164_d_n8;
        locals.var_t2_dn9 = assign94530_e146164_d_n9;
        locals.var_t2_dn10 = assign94530_e146164_d_n10;
        locals.var_t2_dn11 = assign94530_e146164_d_n11;
        locals.var_t2_dn14 = assign94530_e146164_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94540_e146188, assign94540_e146188_d_n0, assign94540_e146188_d_n2, assign94540_e146188_d_n4, assign94540_e146188_d_n5, assign94540_e146188_d_n6, assign94540_e146188_d_n7, assign94540_e146188_d_n8, assign94540_e146188_d_n9, assign94540_e146188_d_n10, assign94540_e146188_d_n11, assign94540_e146188_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) {
        let assign94540_e146175: f64 = (locals.var_vdsi + p.p137);
        let assign94540_e146178: f64 = (locals.var_vdsi + p.p137);
        let assign94540_e146179: f64 = (assign94540_e146175 * assign94540_e146178);
        let assign94540_e146182: f64 = (4.0 * 0.1);
        let assign94540_e146184: f64 = (assign94540_e146182 * 0.1);
        let assign94540_e146185: f64 = (assign94540_e146179 + assign94540_e146184);
        let assign94540_e146186: f64 = (assign94540_e146185).sqrt();
        (assign94540_e146186, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign94540_e146178) + (assign94540_e146175 * locals.var_vdsi_dn6)) / (2.0 * assign94540_e146186)), 0.0, (((locals.var_vdsi_dn8 * assign94540_e146178) + (assign94540_e146175 * locals.var_vdsi_dn8)) / (2.0 * assign94540_e146186)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94540_e146188;
        locals.var_tmf2_dn0 = assign94540_e146188_d_n0;
        locals.var_tmf2_dn2 = assign94540_e146188_d_n2;
        locals.var_tmf2_dn4 = assign94540_e146188_d_n4;
        locals.var_tmf2_dn5 = assign94540_e146188_d_n5;
        locals.var_tmf2_dn6 = assign94540_e146188_d_n6;
        locals.var_tmf2_dn7 = assign94540_e146188_d_n7;
        locals.var_tmf2_dn8 = assign94540_e146188_d_n8;
        locals.var_tmf2_dn9 = assign94540_e146188_d_n9;
        locals.var_tmf2_dn10 = assign94540_e146188_d_n10;
        locals.var_tmf2_dn11 = assign94540_e146188_d_n11;
        locals.var_tmf2_dn14 = assign94540_e146188_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94550_e146207, assign94550_e146207_d_n0, assign94550_e146207_d_n2, assign94550_e146207_d_n4, assign94550_e146207_d_n5, assign94550_e146207_d_n6, assign94550_e146207_d_n7, assign94550_e146207_d_n8, assign94550_e146207_d_n9, assign94550_e146207_d_n10, assign94550_e146207_d_n11, assign94550_e146207_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) {
        let assign94550_e146201: f64 = (locals.var_vdsi + p.p137);
        let assign94550_e146203: f64 = (assign94550_e146201 / locals.var_tmf2);
        let assign94550_e146204: f64 = (1.0 + assign94550_e146203);
        let assign94550_e146205: f64 = (0.5 * assign94550_e146204);
        (assign94550_e146205, (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign94550_e146201 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign94550_e146201 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94550_e146201 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94550_e146207;
        locals.var_t9_dn0 = assign94550_e146207_d_n0;
        locals.var_t9_dn2 = assign94550_e146207_d_n2;
        locals.var_t9_dn4 = assign94550_e146207_d_n4;
        locals.var_t9_dn5 = assign94550_e146207_d_n5;
        locals.var_t9_dn6 = assign94550_e146207_d_n6;
        locals.var_t9_dn7 = assign94550_e146207_d_n7;
        locals.var_t9_dn8 = assign94550_e146207_d_n8;
        locals.var_t9_dn9 = assign94550_e146207_d_n9;
        locals.var_t9_dn10 = assign94550_e146207_d_n10;
        locals.var_t9_dn11 = assign94550_e146207_d_n11;
        locals.var_t9_dn14 = assign94550_e146207_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94560_e146224, assign94560_e146224_d_n0, assign94560_e146224_d_n2, assign94560_e146224_d_n4, assign94560_e146224_d_n5, assign94560_e146224_d_n6, assign94560_e146224_d_n7, assign94560_e146224_d_n8, assign94560_e146224_d_n9, assign94560_e146224_d_n10, assign94560_e146224_d_n11, assign94560_e146224_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) {
        let assign94560_e146219: f64 = (locals.var_vdsi + p.p137);
        let assign94560_e146221: f64 = (assign94560_e146219 + locals.var_tmf2);
        let assign94560_e146222: f64 = (0.5 * assign94560_e146221);
        (assign94560_e146222, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94560_e146224;
        locals.var_t2_dn0 = assign94560_e146224_d_n0;
        locals.var_t2_dn2 = assign94560_e146224_d_n2;
        locals.var_t2_dn4 = assign94560_e146224_d_n4;
        locals.var_t2_dn5 = assign94560_e146224_d_n5;
        locals.var_t2_dn6 = assign94560_e146224_d_n6;
        locals.var_t2_dn7 = assign94560_e146224_d_n7;
        locals.var_t2_dn8 = assign94560_e146224_d_n8;
        locals.var_t2_dn9 = assign94560_e146224_d_n9;
        locals.var_t2_dn10 = assign94560_e146224_d_n10;
        locals.var_t2_dn11 = assign94560_e146224_d_n11;
        locals.var_t2_dn14 = assign94560_e146224_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94570_e146227: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2200 = assign94570_e146227;
        locals.var_guard2200_rv = 0.0;

        let (assign94580_e146240, assign94580_e146240_d_n0, assign94580_e146240_d_n2, assign94580_e146240_d_n4, assign94580_e146240_d_n5, assign94580_e146240_d_n6, assign94580_e146240_d_n7, assign94580_e146240_d_n8, assign94580_e146240_d_n9, assign94580_e146240_d_n10, assign94580_e146240_d_n11, assign94580_e146240_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) && (locals.var_guard2200 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94580_e146240;
        locals.var_t2_dn0 = assign94580_e146240_d_n0;
        locals.var_t2_dn2 = assign94580_e146240_d_n2;
        locals.var_t2_dn4 = assign94580_e146240_d_n4;
        locals.var_t2_dn5 = assign94580_e146240_d_n5;
        locals.var_t2_dn6 = assign94580_e146240_d_n6;
        locals.var_t2_dn7 = assign94580_e146240_d_n7;
        locals.var_t2_dn8 = assign94580_e146240_d_n8;
        locals.var_t2_dn9 = assign94580_e146240_d_n9;
        locals.var_t2_dn10 = assign94580_e146240_d_n10;
        locals.var_t2_dn11 = assign94580_e146240_d_n11;
        locals.var_t2_dn14 = assign94580_e146240_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94590_e146253, assign94590_e146253_d_n0, assign94590_e146253_d_n2, assign94590_e146253_d_n4, assign94590_e146253_d_n5, assign94590_e146253_d_n6, assign94590_e146253_d_n7, assign94590_e146253_d_n8, assign94590_e146253_d_n9, assign94590_e146253_d_n10, assign94590_e146253_d_n11, assign94590_e146253_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) && (locals.var_guard2200 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94590_e146253;
        locals.var_t9_dn0 = assign94590_e146253_d_n0;
        locals.var_t9_dn2 = assign94590_e146253_d_n2;
        locals.var_t9_dn4 = assign94590_e146253_d_n4;
        locals.var_t9_dn5 = assign94590_e146253_d_n5;
        locals.var_t9_dn6 = assign94590_e146253_d_n6;
        locals.var_t9_dn7 = assign94590_e146253_d_n7;
        locals.var_t9_dn8 = assign94590_e146253_d_n8;
        locals.var_t9_dn9 = assign94590_e146253_d_n9;
        locals.var_t9_dn10 = assign94590_e146253_d_n10;
        locals.var_t9_dn11 = assign94590_e146253_d_n11;
        locals.var_t9_dn14 = assign94590_e146253_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94600_e146269, assign94600_e146269_d_n0, assign94600_e146269_d_n2, assign94600_e146269_d_n4, assign94600_e146269_d_n5, assign94600_e146269_d_n6, assign94600_e146269_d_n7, assign94600_e146269_d_n8, assign94600_e146269_d_n9, assign94600_e146269_d_n10, assign94600_e146269_d_n11, assign94600_e146269_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) {
        let assign94600_e146264: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94600_e146265: f64 = (assign94600_e146264).sqrt();
        let assign94600_e146267: f64 = (assign94600_e146265 * p.p432);
        (assign94600_e146267, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94600_e146265)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94600_e146265)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94600_e146269;
        locals.var_wjunc0_dn0 = assign94600_e146269_d_n0;
        locals.var_wjunc0_dn2 = assign94600_e146269_d_n2;
        locals.var_wjunc0_dn4 = assign94600_e146269_d_n4;
        locals.var_wjunc0_dn5 = assign94600_e146269_d_n5;
        locals.var_wjunc0_dn6 = assign94600_e146269_d_n6;
        locals.var_wjunc0_dn7 = assign94600_e146269_d_n7;
        locals.var_wjunc0_dn8 = assign94600_e146269_d_n8;
        locals.var_wjunc0_dn9 = assign94600_e146269_d_n9;
        locals.var_wjunc0_dn10 = assign94600_e146269_d_n10;
        locals.var_wjunc0_dn11 = assign94600_e146269_d_n11;
        locals.var_wjunc0_dn14 = assign94600_e146269_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign94610_e146282, assign94610_e146282_d_n0, assign94610_e146282_d_n2, assign94610_e146282_d_n4, assign94610_e146282_d_n5, assign94610_e146282_d_n6, assign94610_e146282_d_n7, assign94610_e146282_d_n8, assign94610_e146282_d_n9, assign94610_e146282_d_n10, assign94610_e146282_d_n11, assign94610_e146282_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2199 == 0.0)) {
        let assign94610_e146280: f64 = (p.p334 - locals.var_wjunc0);
        (assign94610_e146280, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94610_e146282;
        locals.var_t2_dn0 = assign94610_e146282_d_n0;
        locals.var_t2_dn2 = assign94610_e146282_d_n2;
        locals.var_t2_dn4 = assign94610_e146282_d_n4;
        locals.var_t2_dn5 = assign94610_e146282_d_n5;
        locals.var_t2_dn6 = assign94610_e146282_d_n6;
        locals.var_t2_dn7 = assign94610_e146282_d_n7;
        locals.var_t2_dn8 = assign94610_e146282_d_n8;
        locals.var_t2_dn9 = assign94610_e146282_d_n9;
        locals.var_t2_dn10 = assign94610_e146282_d_n10;
        locals.var_t2_dn11 = assign94610_e146282_d_n11;
        locals.var_t2_dn14 = assign94610_e146282_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_366(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94620_e146303, assign94620_e146303_d_n0, assign94620_e146303_d_n2, assign94620_e146303_d_n4, assign94620_e146303_d_n5, assign94620_e146303_d_n6, assign94620_e146303_d_n7, assign94620_e146303_d_n8, assign94620_e146303_d_n9, assign94620_e146303_d_n10, assign94620_e146303_d_n11, assign94620_e146303_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94620_e146290: f64 = (locals.var_t2 * locals.var_t2);
        let assign94620_e146294: f64 = (p.p334 * 0.01);
        let assign94620_e146295: f64 = (4.0 * assign94620_e146294);
        let assign94620_e146298: f64 = (p.p334 * 0.01);
        let assign94620_e146299: f64 = (assign94620_e146295 * assign94620_e146298);
        let assign94620_e146300: f64 = (assign94620_e146290 + assign94620_e146299);
        let assign94620_e146301: f64 = (assign94620_e146300).sqrt();
        (assign94620_e146301, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign94620_e146301)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign94620_e146301)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94620_e146303;
        locals.var_tmf2_dn0 = assign94620_e146303_d_n0;
        locals.var_tmf2_dn2 = assign94620_e146303_d_n2;
        locals.var_tmf2_dn4 = assign94620_e146303_d_n4;
        locals.var_tmf2_dn5 = assign94620_e146303_d_n5;
        locals.var_tmf2_dn6 = assign94620_e146303_d_n6;
        locals.var_tmf2_dn7 = assign94620_e146303_d_n7;
        locals.var_tmf2_dn8 = assign94620_e146303_d_n8;
        locals.var_tmf2_dn9 = assign94620_e146303_d_n9;
        locals.var_tmf2_dn10 = assign94620_e146303_d_n10;
        locals.var_tmf2_dn11 = assign94620_e146303_d_n11;
        locals.var_tmf2_dn14 = assign94620_e146303_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94630_e146317, assign94630_e146317_d_n0, assign94630_e146317_d_n2, assign94630_e146317_d_n4, assign94630_e146317_d_n5, assign94630_e146317_d_n6, assign94630_e146317_d_n7, assign94630_e146317_d_n8, assign94630_e146317_d_n9, assign94630_e146317_d_n10, assign94630_e146317_d_n11, assign94630_e146317_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94630_e146313: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94630_e146314: f64 = (1.0 + assign94630_e146313);
        let assign94630_e146315: f64 = (0.5 * assign94630_e146314);
        (assign94630_e146315, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94630_e146317;
        locals.var_t9_dn0 = assign94630_e146317_d_n0;
        locals.var_t9_dn2 = assign94630_e146317_d_n2;
        locals.var_t9_dn4 = assign94630_e146317_d_n4;
        locals.var_t9_dn5 = assign94630_e146317_d_n5;
        locals.var_t9_dn6 = assign94630_e146317_d_n6;
        locals.var_t9_dn7 = assign94630_e146317_d_n7;
        locals.var_t9_dn8 = assign94630_e146317_d_n8;
        locals.var_t9_dn9 = assign94630_e146317_d_n9;
        locals.var_t9_dn10 = assign94630_e146317_d_n10;
        locals.var_t9_dn11 = assign94630_e146317_d_n11;
        locals.var_t9_dn14 = assign94630_e146317_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94640_e146329, assign94640_e146329_d_n0, assign94640_e146329_d_n2, assign94640_e146329_d_n4, assign94640_e146329_d_n5, assign94640_e146329_d_n6, assign94640_e146329_d_n7, assign94640_e146329_d_n8, assign94640_e146329_d_n9, assign94640_e146329_d_n10, assign94640_e146329_d_n11, assign94640_e146329_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94640_e146326: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94640_e146327: f64 = (0.5 * assign94640_e146326);
        (assign94640_e146327, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94640_e146329;
        locals.var_t2_dn0 = assign94640_e146329_d_n0;
        locals.var_t2_dn2 = assign94640_e146329_d_n2;
        locals.var_t2_dn4 = assign94640_e146329_d_n4;
        locals.var_t2_dn5 = assign94640_e146329_d_n5;
        locals.var_t2_dn6 = assign94640_e146329_d_n6;
        locals.var_t2_dn7 = assign94640_e146329_d_n7;
        locals.var_t2_dn8 = assign94640_e146329_d_n8;
        locals.var_t2_dn9 = assign94640_e146329_d_n9;
        locals.var_t2_dn10 = assign94640_e146329_d_n10;
        locals.var_t2_dn11 = assign94640_e146329_d_n11;
        locals.var_t2_dn14 = assign94640_e146329_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94650_e146332: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2201 = assign94650_e146332;
        locals.var_guard2201_rv = 0.0;

        let (assign94660_e146342, assign94660_e146342_d_n0, assign94660_e146342_d_n2, assign94660_e146342_d_n4, assign94660_e146342_d_n5, assign94660_e146342_d_n6, assign94660_e146342_d_n7, assign94660_e146342_d_n8, assign94660_e146342_d_n9, assign94660_e146342_d_n10, assign94660_e146342_d_n11, assign94660_e146342_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94660_e146342;
        locals.var_t2_dn0 = assign94660_e146342_d_n0;
        locals.var_t2_dn2 = assign94660_e146342_d_n2;
        locals.var_t2_dn4 = assign94660_e146342_d_n4;
        locals.var_t2_dn5 = assign94660_e146342_d_n5;
        locals.var_t2_dn6 = assign94660_e146342_d_n6;
        locals.var_t2_dn7 = assign94660_e146342_d_n7;
        locals.var_t2_dn8 = assign94660_e146342_d_n8;
        locals.var_t2_dn9 = assign94660_e146342_d_n9;
        locals.var_t2_dn10 = assign94660_e146342_d_n10;
        locals.var_t2_dn11 = assign94660_e146342_d_n11;
        locals.var_t2_dn14 = assign94660_e146342_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94670_e146352, assign94670_e146352_d_n0, assign94670_e146352_d_n2, assign94670_e146352_d_n4, assign94670_e146352_d_n5, assign94670_e146352_d_n6, assign94670_e146352_d_n7, assign94670_e146352_d_n8, assign94670_e146352_d_n9, assign94670_e146352_d_n10, assign94670_e146352_d_n11, assign94670_e146352_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94670_e146352;
        locals.var_t9_dn0 = assign94670_e146352_d_n0;
        locals.var_t9_dn2 = assign94670_e146352_d_n2;
        locals.var_t9_dn4 = assign94670_e146352_d_n4;
        locals.var_t9_dn5 = assign94670_e146352_d_n5;
        locals.var_t9_dn6 = assign94670_e146352_d_n6;
        locals.var_t9_dn7 = assign94670_e146352_d_n7;
        locals.var_t9_dn8 = assign94670_e146352_d_n8;
        locals.var_t9_dn9 = assign94670_e146352_d_n9;
        locals.var_t9_dn10 = assign94670_e146352_d_n10;
        locals.var_t9_dn11 = assign94670_e146352_d_n11;
        locals.var_t9_dn14 = assign94670_e146352_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94680_e146360, assign94680_e146360_d_n0, assign94680_e146360_d_n2, assign94680_e146360_d_n4, assign94680_e146360_d_n5, assign94680_e146360_d_n6, assign94680_e146360_d_n7, assign94680_e146360_d_n8, assign94680_e146360_d_n9, assign94680_e146360_d_n10, assign94680_e146360_d_n11, assign94680_e146360_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign94680_e146360;
        locals.var_ddriftldc_dn0 = assign94680_e146360_d_n0;
        locals.var_ddriftldc_dn2 = assign94680_e146360_d_n2;
        locals.var_ddriftldc_dn4 = assign94680_e146360_d_n4;
        locals.var_ddriftldc_dn5 = assign94680_e146360_d_n5;
        locals.var_ddriftldc_dn6 = assign94680_e146360_d_n6;
        locals.var_ddriftldc_dn7 = assign94680_e146360_d_n7;
        locals.var_ddriftldc_dn8 = assign94680_e146360_d_n8;
        locals.var_ddriftldc_dn9 = assign94680_e146360_d_n9;
        locals.var_ddriftldc_dn10 = assign94680_e146360_d_n10;
        locals.var_ddriftldc_dn11 = assign94680_e146360_d_n11;
        locals.var_ddriftldc_dn14 = assign94680_e146360_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94690_e146376, assign94690_e146376_d_n0, assign94690_e146376_d_n2, assign94690_e146376_d_n4, assign94690_e146376_d_n5, assign94690_e146376_d_n6, assign94690_e146376_d_n7, assign94690_e146376_d_n8, assign94690_e146376_d_n9, assign94690_e146376_d_n10, assign94690_e146376_d_n11, assign94690_e146376_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94690_e146368: f64 = (locals.var_q_nsubld__blk2117 * locals.var_ddriftldc);
        let assign94690_e146370: f64 = (assign94690_e146368 * locals.var_ddriftldc);
        let assign94690_e146372: f64 = (assign94690_e146370 / 2.0);
        let assign94690_e146374: f64 = (assign94690_e146372 / 1.034943e-10);
        (assign94690_e146374, (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign94690_e146368 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign94690_e146376;
        locals.var_dphi_sb_dn0 = assign94690_e146376_d_n0;
        locals.var_dphi_sb_dn2 = assign94690_e146376_d_n2;
        locals.var_dphi_sb_dn4 = assign94690_e146376_d_n4;
        locals.var_dphi_sb_dn5 = assign94690_e146376_d_n5;
        locals.var_dphi_sb_dn6 = assign94690_e146376_d_n6;
        locals.var_dphi_sb_dn7 = assign94690_e146376_d_n7;
        locals.var_dphi_sb_dn8 = assign94690_e146376_d_n8;
        locals.var_dphi_sb_dn9 = assign94690_e146376_d_n9;
        locals.var_dphi_sb_dn10 = assign94690_e146376_d_n10;
        locals.var_dphi_sb_dn11 = assign94690_e146376_d_n11;
        locals.var_dphi_sb_dn14 = assign94690_e146376_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94700_e146389, assign94700_e146389_d_n0, assign94700_e146389_d_n2, assign94700_e146389_d_n4, assign94700_e146389_d_n5, assign94700_e146389_d_n6, assign94700_e146389_d_n7, assign94700_e146389_d_n8, assign94700_e146389_d_n9, assign94700_e146389_d_n10, assign94700_e146389_d_n11, assign94700_e146389_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94700_e146384: f64 = (2.0 * locals.var_beta);
        let assign94700_e146386: f64 = (assign94700_e146384 * locals.var_dphi_sb);
        let assign94700_e146387: f64 = (assign94700_e146386).sqrt();
        (assign94700_e146387, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn0)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn2)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn4)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn5)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn6)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn7)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn8)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn9)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn10)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn11)) / (2.0 * assign94700_e146387)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign94700_e146384 * locals.var_dphi_sb_dn14)) / (2.0 * assign94700_e146387)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94700_e146389;
        locals.var_t0_dn0 = assign94700_e146389_d_n0;
        locals.var_t0_dn2 = assign94700_e146389_d_n2;
        locals.var_t0_dn4 = assign94700_e146389_d_n4;
        locals.var_t0_dn5 = assign94700_e146389_d_n5;
        locals.var_t0_dn6 = assign94700_e146389_d_n6;
        locals.var_t0_dn7 = assign94700_e146389_d_n7;
        locals.var_t0_dn8 = assign94700_e146389_d_n8;
        locals.var_t0_dn9 = assign94700_e146389_d_n9;
        locals.var_t0_dn10 = assign94700_e146389_d_n10;
        locals.var_t0_dn11 = assign94700_e146389_d_n11;
        locals.var_t0_dn14 = assign94700_e146389_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94710_e146404, assign94710_e146404_d_n0, assign94710_e146404_d_n2, assign94710_e146404_d_n4, assign94710_e146404_d_n5, assign94710_e146404_d_n6, assign94710_e146404_d_n7, assign94710_e146404_d_n8, assign94710_e146404_d_n9, assign94710_e146404_d_n10, assign94710_e146404_d_n11, assign94710_e146404_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94710_e146396: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94710_e146398: f64 = (-locals.var_t0);
        let assign94710_e146399: f64 = { let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94710_e146400: f64 = (assign94710_e146396 + assign94710_e146399);
        let assign94710_e146402: f64 = (assign94710_e146400 / 2.0);
        (assign94710_e146402, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign94710_e146398; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94710_e146404;
        locals.var_t1_dn0 = assign94710_e146404_d_n0;
        locals.var_t1_dn2 = assign94710_e146404_d_n2;
        locals.var_t1_dn4 = assign94710_e146404_d_n4;
        locals.var_t1_dn5 = assign94710_e146404_d_n5;
        locals.var_t1_dn6 = assign94710_e146404_d_n6;
        locals.var_t1_dn7 = assign94710_e146404_d_n7;
        locals.var_t1_dn8 = assign94710_e146404_d_n8;
        locals.var_t1_dn9 = assign94710_e146404_d_n9;
        locals.var_t1_dn10 = assign94710_e146404_d_n10;
        locals.var_t1_dn11 = assign94710_e146404_d_n11;
        locals.var_t1_dn14 = assign94710_e146404_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94720_e146415, assign94720_e146415_d_n0, assign94720_e146415_d_n2, assign94720_e146415_d_n4, assign94720_e146415_d_n5, assign94720_e146415_d_n6, assign94720_e146415_d_n7, assign94720_e146415_d_n8, assign94720_e146415_d_n9, assign94720_e146415_d_n10, assign94720_e146415_d_n11, assign94720_e146415_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94720_e146411: f64 = (locals.var_t1).ln();
        let assign94720_e146413: f64 = (assign94720_e146411 / locals.var_dphi_sb);
        (assign94720_e146413, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign94720_e146411 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign94720_e146415;
        locals.var_c_sb_dn0 = assign94720_e146415_d_n0;
        locals.var_c_sb_dn2 = assign94720_e146415_d_n2;
        locals.var_c_sb_dn4 = assign94720_e146415_d_n4;
        locals.var_c_sb_dn5 = assign94720_e146415_d_n5;
        locals.var_c_sb_dn6 = assign94720_e146415_d_n6;
        locals.var_c_sb_dn7 = assign94720_e146415_d_n7;
        locals.var_c_sb_dn8 = assign94720_e146415_d_n8;
        locals.var_c_sb_dn9 = assign94720_e146415_d_n9;
        locals.var_c_sb_dn10 = assign94720_e146415_d_n10;
        locals.var_c_sb_dn11 = assign94720_e146415_d_n11;
        locals.var_c_sb_dn14 = assign94720_e146415_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign94730_e146423,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94730_e146423;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_367(
        locals: &mut StampLocals,
    ) {
        let mut assign94740_loop_guard: usize = 0;
        while {
            let assign94740_cond_e146432: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94740_cond_e146434: f64 = if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_lp_s0 <= assign94740_cond_e146432)) { 1.0 } else { 0.0 };
            assign94740_cond_e146434 != 0.0
        } {
            assign94740_loop_guard += 1;
            assert!(assign94740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94740_body3_e146467, assign94740_body3_e146467_d_n0, assign94740_body3_e146467_d_n2, assign94740_body3_e146467_d_n4, assign94740_body3_e146467_d_n5, assign94740_body3_e146467_d_n6, assign94740_body3_e146467_d_n7, assign94740_body3_e146467_d_n8, assign94740_body3_e146467_d_n9, assign94740_body3_e146467_d_n10, assign94740_body3_e146467_d_n11, assign94740_body3_e146467_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body3_e146465: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94740_body3_e146465, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign94740_body3_e146467;
            locals.var_ps0ld_vxb_dn0 = assign94740_body3_e146467_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94740_body3_e146467_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94740_body3_e146467_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94740_body3_e146467_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94740_body3_e146467_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94740_body3_e146467_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94740_body3_e146467_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94740_body3_e146467_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94740_body3_e146467_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign94740_body3_e146467_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign94740_body3_e146467_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94740_body4_e146477, assign94740_body4_e146477_d_n0, assign94740_body4_e146477_d_n2, assign94740_body4_e146477_d_n4, assign94740_body4_e146477_d_n5, assign94740_body4_e146477_d_n6, assign94740_body4_e146477_d_n7, assign94740_body4_e146477_d_n8, assign94740_body4_e146477_d_n9, assign94740_body4_e146477_d_n10, assign94740_body4_e146477_d_n11, assign94740_body4_e146477_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body4_e146475: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94740_body4_e146475, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign94740_body4_e146477;
            locals.var_chi_dn0 = assign94740_body4_e146477_d_n0;
            locals.var_chi_dn2 = assign94740_body4_e146477_d_n2;
            locals.var_chi_dn4 = assign94740_body4_e146477_d_n4;
            locals.var_chi_dn5 = assign94740_body4_e146477_d_n5;
            locals.var_chi_dn6 = assign94740_body4_e146477_d_n6;
            locals.var_chi_dn7 = assign94740_body4_e146477_d_n7;
            locals.var_chi_dn8 = assign94740_body4_e146477_d_n8;
            locals.var_chi_dn9 = assign94740_body4_e146477_d_n9;
            locals.var_chi_dn10 = assign94740_body4_e146477_d_n10;
            locals.var_chi_dn11 = assign94740_body4_e146477_d_n11;
            locals.var_chi_dn14 = assign94740_body4_e146477_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign94740_body5_e146489, assign94740_body5_e146489_d_n0, assign94740_body5_e146489_d_n2, assign94740_body5_e146489_d_n4, assign94740_body5_e146489_d_n5, assign94740_body5_e146489_d_n6, assign94740_body5_e146489_d_n7, assign94740_body5_e146489_d_n8, assign94740_body5_e146489_d_n9, assign94740_body5_e146489_d_n10, assign94740_body5_e146489_d_n11, assign94740_body5_e146489_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body5_e146486: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94740_body5_e146487: f64 = (locals.var_c_sb * assign94740_body5_e146486);
        (assign94740_body5_e146487, ((locals.var_c_sb_dn0 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign94740_body5_e146486) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign94740_body5_e146489;
            locals.var_ty_dn0 = assign94740_body5_e146489_d_n0;
            locals.var_ty_dn2 = assign94740_body5_e146489_d_n2;
            locals.var_ty_dn4 = assign94740_body5_e146489_d_n4;
            locals.var_ty_dn5 = assign94740_body5_e146489_d_n5;
            locals.var_ty_dn6 = assign94740_body5_e146489_d_n6;
            locals.var_ty_dn7 = assign94740_body5_e146489_d_n7;
            locals.var_ty_dn8 = assign94740_body5_e146489_d_n8;
            locals.var_ty_dn9 = assign94740_body5_e146489_d_n9;
            locals.var_ty_dn10 = assign94740_body5_e146489_d_n10;
            locals.var_ty_dn11 = assign94740_body5_e146489_d_n11;
            locals.var_ty_dn14 = assign94740_body5_e146489_d_n14;
            locals.var_ty_rv = 0.0;
            let assign94740_body6_e146492: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2203 = assign94740_body6_e146492;
            locals.var_guard2203_rv = 0.0;
            let (assign94740_body7_e146503, assign94740_body7_e146503_d_n0, assign94740_body7_e146503_d_n2, assign94740_body7_e146503_d_n4, assign94740_body7_e146503_d_n5, assign94740_body7_e146503_d_n6, assign94740_body7_e146503_d_n7, assign94740_body7_e146503_d_n8, assign94740_body7_e146503_d_n9, assign94740_body7_e146503_d_n10, assign94740_body7_e146503_d_n11, assign94740_body7_e146503_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94740_body7_e146501: f64 = (locals.var_ty).exp();
        (assign94740_body7_e146501, (assign94740_body7_e146501 * locals.var_ty_dn0), (assign94740_body7_e146501 * locals.var_ty_dn2), (assign94740_body7_e146501 * locals.var_ty_dn4), (assign94740_body7_e146501 * locals.var_ty_dn5), (assign94740_body7_e146501 * locals.var_ty_dn6), (assign94740_body7_e146501 * locals.var_ty_dn7), (assign94740_body7_e146501 * locals.var_ty_dn8), (assign94740_body7_e146501 * locals.var_ty_dn9), (assign94740_body7_e146501 * locals.var_ty_dn10), (assign94740_body7_e146501 * locals.var_ty_dn11), (assign94740_body7_e146501 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94740_body7_e146503;
            locals.var_t1_dn0 = assign94740_body7_e146503_d_n0;
            locals.var_t1_dn2 = assign94740_body7_e146503_d_n2;
            locals.var_t1_dn4 = assign94740_body7_e146503_d_n4;
            locals.var_t1_dn5 = assign94740_body7_e146503_d_n5;
            locals.var_t1_dn6 = assign94740_body7_e146503_d_n6;
            locals.var_t1_dn7 = assign94740_body7_e146503_d_n7;
            locals.var_t1_dn8 = assign94740_body7_e146503_d_n8;
            locals.var_t1_dn9 = assign94740_body7_e146503_d_n9;
            locals.var_t1_dn10 = assign94740_body7_e146503_d_n10;
            locals.var_t1_dn11 = assign94740_body7_e146503_d_n11;
            locals.var_t1_dn14 = assign94740_body7_e146503_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94740_body8_e146517, assign94740_body8_e146517_d_n0, assign94740_body8_e146517_d_n2, assign94740_body8_e146517_d_n4, assign94740_body8_e146517_d_n5, assign94740_body8_e146517_d_n6, assign94740_body8_e146517_d_n7, assign94740_body8_e146517_d_n8, assign94740_body8_e146517_d_n9, assign94740_body8_e146517_d_n10, assign94740_body8_e146517_d_n11, assign94740_body8_e146517_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94740_body8_e146512: f64 = (-locals.var_c_sb);
        let assign94740_body8_e146514: f64 = (assign94740_body8_e146512 * locals.var_dphi_sb);
        let assign94740_body8_e146515: f64 = (assign94740_body8_e146514).exp();
        (assign94740_body8_e146515, (assign94740_body8_e146515 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn0))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn2))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn4))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn5))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn6))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn7))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn8))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn9))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn10))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn11))), (assign94740_body8_e146515 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign94740_body8_e146512 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94740_body8_e146517;
            locals.var_t0_dn0 = assign94740_body8_e146517_d_n0;
            locals.var_t0_dn2 = assign94740_body8_e146517_d_n2;
            locals.var_t0_dn4 = assign94740_body8_e146517_d_n4;
            locals.var_t0_dn5 = assign94740_body8_e146517_d_n5;
            locals.var_t0_dn6 = assign94740_body8_e146517_d_n6;
            locals.var_t0_dn7 = assign94740_body8_e146517_d_n7;
            locals.var_t0_dn8 = assign94740_body8_e146517_d_n8;
            locals.var_t0_dn9 = assign94740_body8_e146517_d_n9;
            locals.var_t0_dn10 = assign94740_body8_e146517_d_n10;
            locals.var_t0_dn11 = assign94740_body8_e146517_d_n11;
            locals.var_t0_dn14 = assign94740_body8_e146517_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94740_body9_e146529, assign94740_body9_e146529_d_n0, assign94740_body9_e146529_d_n2, assign94740_body9_e146529_d_n4, assign94740_body9_e146529_d_n5, assign94740_body9_e146529_d_n6, assign94740_body9_e146529_d_n7, assign94740_body9_e146529_d_n8, assign94740_body9_e146529_d_n9, assign94740_body9_e146529_d_n10, assign94740_body9_e146529_d_n11, assign94740_body9_e146529_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94740_body9_e146527: f64 = (locals.var_t1 - locals.var_t0);
        (assign94740_body9_e146527, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94740_body9_e146529;
            locals.var_t2_dn0 = assign94740_body9_e146529_d_n0;
            locals.var_t2_dn2 = assign94740_body9_e146529_d_n2;
            locals.var_t2_dn4 = assign94740_body9_e146529_d_n4;
            locals.var_t2_dn5 = assign94740_body9_e146529_d_n5;
            locals.var_t2_dn6 = assign94740_body9_e146529_d_n6;
            locals.var_t2_dn7 = assign94740_body9_e146529_d_n7;
            locals.var_t2_dn8 = assign94740_body9_e146529_d_n8;
            locals.var_t2_dn9 = assign94740_body9_e146529_d_n9;
            locals.var_t2_dn10 = assign94740_body9_e146529_d_n10;
            locals.var_t2_dn11 = assign94740_body9_e146529_d_n11;
            locals.var_t2_dn14 = assign94740_body9_e146529_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94740_body10_e146544, assign94740_body10_e146544_d_n0, assign94740_body10_e146544_d_n2, assign94740_body10_e146544_d_n4, assign94740_body10_e146544_d_n5, assign94740_body10_e146544_d_n6, assign94740_body10_e146544_d_n7, assign94740_body10_e146544_d_n8, assign94740_body10_e146544_d_n9, assign94740_body10_e146544_d_n10, assign94740_body10_e146544_d_n11, assign94740_body10_e146544_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94740_body10_e146539: f64 = (1.0 + locals.var_t2);
        let assign94740_body10_e146540: f64 = (assign94740_body10_e146539).ln();
        let assign94740_body10_e146542: f64 = (assign94740_body10_e146540 / locals.var_c_sb);
        (assign94740_body10_e146542, ((((locals.var_t2_dn0 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign94740_body10_e146539) * locals.var_c_sb) - (assign94740_body10_e146540 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94740_body10_e146544;
            locals.var_phi_b_dn0 = assign94740_body10_e146544_d_n0;
            locals.var_phi_b_dn2 = assign94740_body10_e146544_d_n2;
            locals.var_phi_b_dn4 = assign94740_body10_e146544_d_n4;
            locals.var_phi_b_dn5 = assign94740_body10_e146544_d_n5;
            locals.var_phi_b_dn6 = assign94740_body10_e146544_d_n6;
            locals.var_phi_b_dn7 = assign94740_body10_e146544_d_n7;
            locals.var_phi_b_dn8 = assign94740_body10_e146544_d_n8;
            locals.var_phi_b_dn9 = assign94740_body10_e146544_d_n9;
            locals.var_phi_b_dn10 = assign94740_body10_e146544_d_n10;
            locals.var_phi_b_dn11 = assign94740_body10_e146544_d_n11;
            locals.var_phi_b_dn14 = assign94740_body10_e146544_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94740_body11_e146558, assign94740_body11_e146558_d_n0, assign94740_body11_e146558_d_n2, assign94740_body11_e146558_d_n4, assign94740_body11_e146558_d_n5, assign94740_body11_e146558_d_n6, assign94740_body11_e146558_d_n7, assign94740_body11_e146558_d_n8, assign94740_body11_e146558_d_n9, assign94740_body11_e146558_d_n10, assign94740_body11_e146558_d_n11, assign94740_body11_e146558_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94740_body11_e146555: f64 = (1.0 + locals.var_t2);
        let assign94740_body11_e146556: f64 = (locals.var_t1 / assign94740_body11_e146555);
        (assign94740_body11_e146556, (((locals.var_t1_dn0 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn2 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn4 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn5 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn6 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn7 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn8 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn9 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn10 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn11 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn11)) / (assign94740_body11_e146555 * assign94740_body11_e146555)), (((locals.var_t1_dn14 * assign94740_body11_e146555) - (locals.var_t1 * locals.var_t2_dn14)) / (assign94740_body11_e146555 * assign94740_body11_e146555)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94740_body11_e146558;
            locals.var_phi_b_dpss_dn0 = assign94740_body11_e146558_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94740_body11_e146558_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94740_body11_e146558_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94740_body11_e146558_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94740_body11_e146558_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94740_body11_e146558_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94740_body11_e146558_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94740_body11_e146558_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94740_body11_e146558_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94740_body11_e146558_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94740_body11_e146558_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94740_body12_e146571, assign94740_body12_e146571_d_n0, assign94740_body12_e146571_d_n2, assign94740_body12_e146571_d_n4, assign94740_body12_e146571_d_n5, assign94740_body12_e146571_d_n6, assign94740_body12_e146571_d_n7, assign94740_body12_e146571_d_n8, assign94740_body12_e146571_d_n9, assign94740_body12_e146571_d_n10, assign94740_body12_e146571_d_n11, assign94740_body12_e146571_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 == 0.0)) {
        let assign94740_body12_e146569: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94740_body12_e146569, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94740_body12_e146571;
            locals.var_phi_b_dn0 = assign94740_body12_e146571_d_n0;
            locals.var_phi_b_dn2 = assign94740_body12_e146571_d_n2;
            locals.var_phi_b_dn4 = assign94740_body12_e146571_d_n4;
            locals.var_phi_b_dn5 = assign94740_body12_e146571_d_n5;
            locals.var_phi_b_dn6 = assign94740_body12_e146571_d_n6;
            locals.var_phi_b_dn7 = assign94740_body12_e146571_d_n7;
            locals.var_phi_b_dn8 = assign94740_body12_e146571_d_n8;
            locals.var_phi_b_dn9 = assign94740_body12_e146571_d_n9;
            locals.var_phi_b_dn10 = assign94740_body12_e146571_d_n10;
            locals.var_phi_b_dn11 = assign94740_body12_e146571_d_n11;
            locals.var_phi_b_dn14 = assign94740_body12_e146571_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94740_body13_e146582, assign94740_body13_e146582_d_n0, assign94740_body13_e146582_d_n2, assign94740_body13_e146582_d_n4, assign94740_body13_e146582_d_n5, assign94740_body13_e146582_d_n6, assign94740_body13_e146582_d_n7, assign94740_body13_e146582_d_n8, assign94740_body13_e146582_d_n9, assign94740_body13_e146582_d_n10, assign94740_body13_e146582_d_n11, assign94740_body13_e146582_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2203 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94740_body13_e146582;
            locals.var_phi_b_dpss_dn0 = assign94740_body13_e146582_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94740_body13_e146582_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94740_body13_e146582_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94740_body13_e146582_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94740_body13_e146582_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94740_body13_e146582_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94740_body13_e146582_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94740_body13_e146582_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94740_body13_e146582_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94740_body13_e146582_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94740_body13_e146582_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94740_body14_e146592, assign94740_body14_e146592_d_n0, assign94740_body14_e146592_d_n2, assign94740_body14_e146592_d_n4, assign94740_body14_e146592_d_n5, assign94740_body14_e146592_d_n6, assign94740_body14_e146592_d_n7, assign94740_body14_e146592_d_n8, assign94740_body14_e146592_d_n9, assign94740_body14_e146592_d_n10, assign94740_body14_e146592_d_n11, assign94740_body14_e146592_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body14_e146590: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94740_body14_e146590, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign94740_body14_e146592;
            locals.var_chib_dn0 = assign94740_body14_e146592_d_n0;
            locals.var_chib_dn2 = assign94740_body14_e146592_d_n2;
            locals.var_chib_dn4 = assign94740_body14_e146592_d_n4;
            locals.var_chib_dn5 = assign94740_body14_e146592_d_n5;
            locals.var_chib_dn6 = assign94740_body14_e146592_d_n6;
            locals.var_chib_dn7 = assign94740_body14_e146592_d_n7;
            locals.var_chib_dn8 = assign94740_body14_e146592_d_n8;
            locals.var_chib_dn9 = assign94740_body14_e146592_d_n9;
            locals.var_chib_dn10 = assign94740_body14_e146592_d_n10;
            locals.var_chib_dn11 = assign94740_body14_e146592_d_n11;
            locals.var_chib_dn14 = assign94740_body14_e146592_d_n14;
            locals.var_chib_rv = 0.0;
            let assign94740_body15_e146594: f64 = (locals.var_chi).abs();
            let assign94740_body15_e146596: f64 = if assign94740_body15_e146594 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2204 = assign94740_body15_e146596;
            locals.var_guard2204_rv = 0.0;
            let (assign94740_body17_e146646, assign94740_body17_e146646_d_n0, assign94740_body17_e146646_d_n2, assign94740_body17_e146646_d_n4, assign94740_body17_e146646_d_n5, assign94740_body17_e146646_d_n6, assign94740_body17_e146646_d_n7, assign94740_body17_e146646_d_n8, assign94740_body17_e146646_d_n9, assign94740_body17_e146646_d_n10, assign94740_body17_e146646_d_n11, assign94740_body17_e146646_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body17_e146624: f64 = (locals.var_chi * locals.var_chi);
        let assign94740_body17_e146626: f64 = (assign94740_body17_e146624 / 2.0);
        let assign94740_body17_e146630: f64 = (locals.var_chi / 3.0);
        let assign94740_body17_e146634: f64 = (locals.var_chi / 4.0);
        let assign94740_body17_e146638: f64 = (locals.var_chi / 5.0);
        let assign94740_body17_e146639: f64 = (1.0 - assign94740_body17_e146638);
        let assign94740_body17_e146640: f64 = (assign94740_body17_e146634 * assign94740_body17_e146639);
        let assign94740_body17_e146641: f64 = (1.0 - assign94740_body17_e146640);
        let assign94740_body17_e146642: f64 = (assign94740_body17_e146630 * assign94740_body17_e146641);
        let assign94740_body17_e146643: f64 = (1.0 - assign94740_body17_e146642);
        let assign94740_body17_e146644: f64 = (assign94740_body17_e146626 * assign94740_body17_e146643);
        (assign94740_body17_e146644, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn0 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn0 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn2 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn2 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn4 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn4 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn5 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn5 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn6 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn6 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn7 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn7 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn8 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn8 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn9 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn9 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn10 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn10 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn11 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn11 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94740_body17_e146643) + (assign94740_body17_e146626 * (-(((locals.var_chi_dn14 / 3.0) * assign94740_body17_e146641) + (assign94740_body17_e146630 * (-(((locals.var_chi_dn14 / 4.0) * assign94740_body17_e146639) + (assign94740_body17_e146634 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94740_body17_e146646;
            locals.var_t0_dn0 = assign94740_body17_e146646_d_n0;
            locals.var_t0_dn2 = assign94740_body17_e146646_d_n2;
            locals.var_t0_dn4 = assign94740_body17_e146646_d_n4;
            locals.var_t0_dn5 = assign94740_body17_e146646_d_n5;
            locals.var_t0_dn6 = assign94740_body17_e146646_d_n6;
            locals.var_t0_dn7 = assign94740_body17_e146646_d_n7;
            locals.var_t0_dn8 = assign94740_body17_e146646_d_n8;
            locals.var_t0_dn9 = assign94740_body17_e146646_d_n9;
            locals.var_t0_dn10 = assign94740_body17_e146646_d_n10;
            locals.var_t0_dn11 = assign94740_body17_e146646_d_n11;
            locals.var_t0_dn14 = assign94740_body17_e146646_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94740_body18_e146674, assign94740_body18_e146674_d_n0, assign94740_body18_e146674_d_n2, assign94740_body18_e146674_d_n4, assign94740_body18_e146674_d_n5, assign94740_body18_e146674_d_n6, assign94740_body18_e146674_d_n7, assign94740_body18_e146674_d_n8, assign94740_body18_e146674_d_n9, assign94740_body18_e146674_d_n10, assign94740_body18_e146674_d_n11, assign94740_body18_e146674_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body18_e146658: f64 = (locals.var_chi / 2.0);
        let assign94740_body18_e146662: f64 = (locals.var_chi / 3.0);
        let assign94740_body18_e146666: f64 = (locals.var_chi / 4.0);
        let assign94740_body18_e146667: f64 = (1.0 - assign94740_body18_e146666);
        let assign94740_body18_e146668: f64 = (assign94740_body18_e146662 * assign94740_body18_e146667);
        let assign94740_body18_e146669: f64 = (1.0 - assign94740_body18_e146668);
        let assign94740_body18_e146670: f64 = (assign94740_body18_e146658 * assign94740_body18_e146669);
        let assign94740_body18_e146671: f64 = (1.0 - assign94740_body18_e146670);
        let assign94740_body18_e146672: f64 = (locals.var_chi * assign94740_body18_e146671);
        (assign94740_body18_e146672, ((locals.var_chi_dn0 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn0 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn2 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn4 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn5 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn6 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn7 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn8 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn9 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn10 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn11 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign94740_body18_e146671) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign94740_body18_e146669) + (assign94740_body18_e146658 * (-(((locals.var_chi_dn14 / 3.0) * assign94740_body18_e146667) + (assign94740_body18_e146662 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94740_body18_e146674;
            locals.var_t1_dn0 = assign94740_body18_e146674_d_n0;
            locals.var_t1_dn2 = assign94740_body18_e146674_d_n2;
            locals.var_t1_dn4 = assign94740_body18_e146674_d_n4;
            locals.var_t1_dn5 = assign94740_body18_e146674_d_n5;
            locals.var_t1_dn6 = assign94740_body18_e146674_d_n6;
            locals.var_t1_dn7 = assign94740_body18_e146674_d_n7;
            locals.var_t1_dn8 = assign94740_body18_e146674_d_n8;
            locals.var_t1_dn9 = assign94740_body18_e146674_d_n9;
            locals.var_t1_dn10 = assign94740_body18_e146674_d_n10;
            locals.var_t1_dn11 = assign94740_body18_e146674_d_n11;
            locals.var_t1_dn14 = assign94740_body18_e146674_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94740_body19_e146706, assign94740_body19_e146706_d_n0, assign94740_body19_e146706_d_n2, assign94740_body19_e146706_d_n4, assign94740_body19_e146706_d_n5, assign94740_body19_e146706_d_n6, assign94740_body19_e146706_d_n7, assign94740_body19_e146706_d_n8, assign94740_body19_e146706_d_n9, assign94740_body19_e146706_d_n10, assign94740_body19_e146706_d_n11, assign94740_body19_e146706_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body19_e146684: f64 = (locals.var_chib * locals.var_chib);
        let assign94740_body19_e146686: f64 = (assign94740_body19_e146684 / 2.0);
        let assign94740_body19_e146690: f64 = (locals.var_chib / 3.0);
        let assign94740_body19_e146694: f64 = (locals.var_chib / 4.0);
        let assign94740_body19_e146698: f64 = (locals.var_chib / 5.0);
        let assign94740_body19_e146699: f64 = (1.0 - assign94740_body19_e146698);
        let assign94740_body19_e146700: f64 = (assign94740_body19_e146694 * assign94740_body19_e146699);
        let assign94740_body19_e146701: f64 = (1.0 - assign94740_body19_e146700);
        let assign94740_body19_e146702: f64 = (assign94740_body19_e146690 * assign94740_body19_e146701);
        let assign94740_body19_e146703: f64 = (1.0 - assign94740_body19_e146702);
        let assign94740_body19_e146704: f64 = (assign94740_body19_e146686 * assign94740_body19_e146703);
        (assign94740_body19_e146704, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn0 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn0 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn2 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn2 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn4 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn4 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn5 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn5 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn6 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn6 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn7 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn7 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn8 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn8 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn9 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn9 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn10 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn10 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn11 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn11 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign94740_body19_e146703) + (assign94740_body19_e146686 * (-(((locals.var_chib_dn14 / 3.0) * assign94740_body19_e146701) + (assign94740_body19_e146690 * (-(((locals.var_chib_dn14 / 4.0) * assign94740_body19_e146699) + (assign94740_body19_e146694 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94740_body19_e146706;
            locals.var_t2_dn0 = assign94740_body19_e146706_d_n0;
            locals.var_t2_dn2 = assign94740_body19_e146706_d_n2;
            locals.var_t2_dn4 = assign94740_body19_e146706_d_n4;
            locals.var_t2_dn5 = assign94740_body19_e146706_d_n5;
            locals.var_t2_dn6 = assign94740_body19_e146706_d_n6;
            locals.var_t2_dn7 = assign94740_body19_e146706_d_n7;
            locals.var_t2_dn8 = assign94740_body19_e146706_d_n8;
            locals.var_t2_dn9 = assign94740_body19_e146706_d_n9;
            locals.var_t2_dn10 = assign94740_body19_e146706_d_n10;
            locals.var_t2_dn11 = assign94740_body19_e146706_d_n11;
            locals.var_t2_dn14 = assign94740_body19_e146706_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94740_body20_e146734, assign94740_body20_e146734_d_n0, assign94740_body20_e146734_d_n2, assign94740_body20_e146734_d_n4, assign94740_body20_e146734_d_n5, assign94740_body20_e146734_d_n6, assign94740_body20_e146734_d_n7, assign94740_body20_e146734_d_n8, assign94740_body20_e146734_d_n9, assign94740_body20_e146734_d_n10, assign94740_body20_e146734_d_n11, assign94740_body20_e146734_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body20_e146718: f64 = (locals.var_chib / 2.0);
        let assign94740_body20_e146722: f64 = (locals.var_chib / 3.0);
        let assign94740_body20_e146726: f64 = (locals.var_chib / 4.0);
        let assign94740_body20_e146727: f64 = (1.0 - assign94740_body20_e146726);
        let assign94740_body20_e146728: f64 = (assign94740_body20_e146722 * assign94740_body20_e146727);
        let assign94740_body20_e146729: f64 = (1.0 - assign94740_body20_e146728);
        let assign94740_body20_e146730: f64 = (assign94740_body20_e146718 * assign94740_body20_e146729);
        let assign94740_body20_e146731: f64 = (1.0 - assign94740_body20_e146730);
        let assign94740_body20_e146732: f64 = (locals.var_chib * assign94740_body20_e146731);
        (assign94740_body20_e146732, ((locals.var_chib_dn0 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn0 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn2 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn4 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn5 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn6 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn7 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn8 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn9 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn10 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn11 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign94740_body20_e146731) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign94740_body20_e146729) + (assign94740_body20_e146718 * (-(((locals.var_chib_dn14 / 3.0) * assign94740_body20_e146727) + (assign94740_body20_e146722 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign94740_body20_e146734;
            locals.var_t3_dn0 = assign94740_body20_e146734_d_n0;
            locals.var_t3_dn2 = assign94740_body20_e146734_d_n2;
            locals.var_t3_dn4 = assign94740_body20_e146734_d_n4;
            locals.var_t3_dn5 = assign94740_body20_e146734_d_n5;
            locals.var_t3_dn6 = assign94740_body20_e146734_d_n6;
            locals.var_t3_dn7 = assign94740_body20_e146734_d_n7;
            locals.var_t3_dn8 = assign94740_body20_e146734_d_n8;
            locals.var_t3_dn9 = assign94740_body20_e146734_d_n9;
            locals.var_t3_dn10 = assign94740_body20_e146734_d_n10;
            locals.var_t3_dn11 = assign94740_body20_e146734_d_n11;
            locals.var_t3_dn14 = assign94740_body20_e146734_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign94740_body21_e146746, assign94740_body21_e146746_d_n0, assign94740_body21_e146746_d_n2, assign94740_body21_e146746_d_n4, assign94740_body21_e146746_d_n5, assign94740_body21_e146746_d_n6, assign94740_body21_e146746_d_n7, assign94740_body21_e146746_d_n8, assign94740_body21_e146746_d_n9, assign94740_body21_e146746_d_n10, assign94740_body21_e146746_d_n11, assign94740_body21_e146746_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body21_e146744: f64 = (locals.var_t0 - locals.var_t2);
        (assign94740_body21_e146744, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_fbsq__blk2125, locals.var_fbsq__blk2125_dn0, locals.var_fbsq__blk2125_dn2, locals.var_fbsq__blk2125_dn4, locals.var_fbsq__blk2125_dn5, locals.var_fbsq__blk2125_dn6, locals.var_fbsq__blk2125_dn7, locals.var_fbsq__blk2125_dn8, locals.var_fbsq__blk2125_dn9, locals.var_fbsq__blk2125_dn10, locals.var_fbsq__blk2125_dn11, locals.var_fbsq__blk2125_dn14,)
    }
};
            locals.var_fbsq__blk2125 = assign94740_body21_e146746;
            locals.var_fbsq__blk2125_dn0 = assign94740_body21_e146746_d_n0;
            locals.var_fbsq__blk2125_dn2 = assign94740_body21_e146746_d_n2;
            locals.var_fbsq__blk2125_dn4 = assign94740_body21_e146746_d_n4;
            locals.var_fbsq__blk2125_dn5 = assign94740_body21_e146746_d_n5;
            locals.var_fbsq__blk2125_dn6 = assign94740_body21_e146746_d_n6;
            locals.var_fbsq__blk2125_dn7 = assign94740_body21_e146746_d_n7;
            locals.var_fbsq__blk2125_dn8 = assign94740_body21_e146746_d_n8;
            locals.var_fbsq__blk2125_dn9 = assign94740_body21_e146746_d_n9;
            locals.var_fbsq__blk2125_dn10 = assign94740_body21_e146746_d_n10;
            locals.var_fbsq__blk2125_dn11 = assign94740_body21_e146746_d_n11;
            locals.var_fbsq__blk2125_dn14 = assign94740_body21_e146746_d_n14;
            locals.var_fbsq__blk2125_rv = 0.0;
            let (assign94740_body22_e146762, assign94740_body22_e146762_d_n0, assign94740_body22_e146762_d_n2, assign94740_body22_e146762_d_n4, assign94740_body22_e146762_d_n5, assign94740_body22_e146762_d_n6, assign94740_body22_e146762_d_n7, assign94740_body22_e146762_d_n8, assign94740_body22_e146762_d_n9, assign94740_body22_e146762_d_n10, assign94740_body22_e146762_d_n11, assign94740_body22_e146762_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94740_body22_e146758: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94740_body22_e146759: f64 = (locals.var_t1 - assign94740_body22_e146758);
        let assign94740_body22_e146760: f64 = (locals.var_beta * assign94740_body22_e146759);
        (assign94740_body22_e146760, ((locals.var_beta_dn0 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn11 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))), ((locals.var_beta_dn14 * assign94740_body22_e146759) + (locals.var_beta * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))),)
    } else {
        (locals.var_fbsq_dpss__blk2126, locals.var_fbsq_dpss__blk2126_dn0, locals.var_fbsq_dpss__blk2126_dn2, locals.var_fbsq_dpss__blk2126_dn4, locals.var_fbsq_dpss__blk2126_dn5, locals.var_fbsq_dpss__blk2126_dn6, locals.var_fbsq_dpss__blk2126_dn7, locals.var_fbsq_dpss__blk2126_dn8, locals.var_fbsq_dpss__blk2126_dn9, locals.var_fbsq_dpss__blk2126_dn10, locals.var_fbsq_dpss__blk2126_dn11, locals.var_fbsq_dpss__blk2126_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2126 = assign94740_body22_e146762;
            locals.var_fbsq_dpss__blk2126_dn0 = assign94740_body22_e146762_d_n0;
            locals.var_fbsq_dpss__blk2126_dn2 = assign94740_body22_e146762_d_n2;
            locals.var_fbsq_dpss__blk2126_dn4 = assign94740_body22_e146762_d_n4;
            locals.var_fbsq_dpss__blk2126_dn5 = assign94740_body22_e146762_d_n5;
            locals.var_fbsq_dpss__blk2126_dn6 = assign94740_body22_e146762_d_n6;
            locals.var_fbsq_dpss__blk2126_dn7 = assign94740_body22_e146762_d_n7;
            locals.var_fbsq_dpss__blk2126_dn8 = assign94740_body22_e146762_d_n8;
            locals.var_fbsq_dpss__blk2126_dn9 = assign94740_body22_e146762_d_n9;
            locals.var_fbsq_dpss__blk2126_dn10 = assign94740_body22_e146762_d_n10;
            locals.var_fbsq_dpss__blk2126_dn11 = assign94740_body22_e146762_d_n11;
            locals.var_fbsq_dpss__blk2126_dn14 = assign94740_body22_e146762_d_n14;
            locals.var_fbsq_dpss__blk2126_rv = 0.0;
            let (assign94740_body24_e146794, assign94740_body24_e146794_d_n0, assign94740_body24_e146794_d_n2, assign94740_body24_e146794_d_n4, assign94740_body24_e146794_d_n5, assign94740_body24_e146794_d_n6, assign94740_body24_e146794_d_n7, assign94740_body24_e146794_d_n8, assign94740_body24_e146794_d_n9, assign94740_body24_e146794_d_n10, assign94740_body24_e146794_d_n11, assign94740_body24_e146794_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94740_body24_e146791: f64 = (-locals.var_chi);
        let assign94740_body24_e146792: f64 = (assign94740_body24_e146791).exp();
        (assign94740_body24_e146792, (assign94740_body24_e146792 * (-locals.var_chi_dn0)), (assign94740_body24_e146792 * (-locals.var_chi_dn2)), (assign94740_body24_e146792 * (-locals.var_chi_dn4)), (assign94740_body24_e146792 * (-locals.var_chi_dn5)), (assign94740_body24_e146792 * (-locals.var_chi_dn6)), (assign94740_body24_e146792 * (-locals.var_chi_dn7)), (assign94740_body24_e146792 * (-locals.var_chi_dn8)), (assign94740_body24_e146792 * (-locals.var_chi_dn9)), (assign94740_body24_e146792 * (-locals.var_chi_dn10)), (assign94740_body24_e146792 * (-locals.var_chi_dn11)), (assign94740_body24_e146792 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94740_body24_e146794;
            locals.var_t0_dn0 = assign94740_body24_e146794_d_n0;
            locals.var_t0_dn2 = assign94740_body24_e146794_d_n2;
            locals.var_t0_dn4 = assign94740_body24_e146794_d_n4;
            locals.var_t0_dn5 = assign94740_body24_e146794_d_n5;
            locals.var_t0_dn6 = assign94740_body24_e146794_d_n6;
            locals.var_t0_dn7 = assign94740_body24_e146794_d_n7;
            locals.var_t0_dn8 = assign94740_body24_e146794_d_n8;
            locals.var_t0_dn9 = assign94740_body24_e146794_d_n9;
            locals.var_t0_dn10 = assign94740_body24_e146794_d_n10;
            locals.var_t0_dn11 = assign94740_body24_e146794_d_n11;
            locals.var_t0_dn14 = assign94740_body24_e146794_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94740_body25_e146807, assign94740_body25_e146807_d_n0, assign94740_body25_e146807_d_n2, assign94740_body25_e146807_d_n4, assign94740_body25_e146807_d_n5, assign94740_body25_e146807_d_n6, assign94740_body25_e146807_d_n7, assign94740_body25_e146807_d_n8, assign94740_body25_e146807_d_n9, assign94740_body25_e146807_d_n10, assign94740_body25_e146807_d_n11, assign94740_body25_e146807_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94740_body25_e146804: f64 = (-locals.var_chib);
        let assign94740_body25_e146805: f64 = (assign94740_body25_e146804).exp();
        (assign94740_body25_e146805, (assign94740_body25_e146805 * (-locals.var_chib_dn0)), (assign94740_body25_e146805 * (-locals.var_chib_dn2)), (assign94740_body25_e146805 * (-locals.var_chib_dn4)), (assign94740_body25_e146805 * (-locals.var_chib_dn5)), (assign94740_body25_e146805 * (-locals.var_chib_dn6)), (assign94740_body25_e146805 * (-locals.var_chib_dn7)), (assign94740_body25_e146805 * (-locals.var_chib_dn8)), (assign94740_body25_e146805 * (-locals.var_chib_dn9)), (assign94740_body25_e146805 * (-locals.var_chib_dn10)), (assign94740_body25_e146805 * (-locals.var_chib_dn11)), (assign94740_body25_e146805 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94740_body25_e146807;
            locals.var_t1_dn0 = assign94740_body25_e146807_d_n0;
            locals.var_t1_dn2 = assign94740_body25_e146807_d_n2;
            locals.var_t1_dn4 = assign94740_body25_e146807_d_n4;
            locals.var_t1_dn5 = assign94740_body25_e146807_d_n5;
            locals.var_t1_dn6 = assign94740_body25_e146807_d_n6;
            locals.var_t1_dn7 = assign94740_body25_e146807_d_n7;
            locals.var_t1_dn8 = assign94740_body25_e146807_d_n8;
            locals.var_t1_dn9 = assign94740_body25_e146807_d_n9;
            locals.var_t1_dn10 = assign94740_body25_e146807_d_n10;
            locals.var_t1_dn11 = assign94740_body25_e146807_d_n11;
            locals.var_t1_dn14 = assign94740_body25_e146807_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94740_body26_e146824, assign94740_body26_e146824_d_n0, assign94740_body26_e146824_d_n2, assign94740_body26_e146824_d_n4, assign94740_body26_e146824_d_n5, assign94740_body26_e146824_d_n6, assign94740_body26_e146824_d_n7, assign94740_body26_e146824_d_n8, assign94740_body26_e146824_d_n9, assign94740_body26_e146824_d_n10, assign94740_body26_e146824_d_n11, assign94740_body26_e146824_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94740_body26_e146818: f64 = (locals.var_chi - locals.var_chib);
        let assign94740_body26_e146821: f64 = (locals.var_t0 - locals.var_t1);
        let assign94740_body26_e146822: f64 = (assign94740_body26_e146818 + assign94740_body26_e146821);
        (assign94740_body26_e146822, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_fbsq__blk2125, locals.var_fbsq__blk2125_dn0, locals.var_fbsq__blk2125_dn2, locals.var_fbsq__blk2125_dn4, locals.var_fbsq__blk2125_dn5, locals.var_fbsq__blk2125_dn6, locals.var_fbsq__blk2125_dn7, locals.var_fbsq__blk2125_dn8, locals.var_fbsq__blk2125_dn9, locals.var_fbsq__blk2125_dn10, locals.var_fbsq__blk2125_dn11, locals.var_fbsq__blk2125_dn14,)
    }
};
            locals.var_fbsq__blk2125 = assign94740_body26_e146824;
            locals.var_fbsq__blk2125_dn0 = assign94740_body26_e146824_d_n0;
            locals.var_fbsq__blk2125_dn2 = assign94740_body26_e146824_d_n2;
            locals.var_fbsq__blk2125_dn4 = assign94740_body26_e146824_d_n4;
            locals.var_fbsq__blk2125_dn5 = assign94740_body26_e146824_d_n5;
            locals.var_fbsq__blk2125_dn6 = assign94740_body26_e146824_d_n6;
            locals.var_fbsq__blk2125_dn7 = assign94740_body26_e146824_d_n7;
            locals.var_fbsq__blk2125_dn8 = assign94740_body26_e146824_d_n8;
            locals.var_fbsq__blk2125_dn9 = assign94740_body26_e146824_d_n9;
            locals.var_fbsq__blk2125_dn10 = assign94740_body26_e146824_d_n10;
            locals.var_fbsq__blk2125_dn11 = assign94740_body26_e146824_d_n11;
            locals.var_fbsq__blk2125_dn14 = assign94740_body26_e146824_d_n14;
            locals.var_fbsq__blk2125_rv = 0.0;
            let (assign94740_body27_e146845, assign94740_body27_e146845_d_n0, assign94740_body27_e146845_d_n2, assign94740_body27_e146845_d_n4, assign94740_body27_e146845_d_n5, assign94740_body27_e146845_d_n6, assign94740_body27_e146845_d_n7, assign94740_body27_e146845_d_n8, assign94740_body27_e146845_d_n9, assign94740_body27_e146845_d_n10, assign94740_body27_e146845_d_n11, assign94740_body27_e146845_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94740_body27_e146836: f64 = (1.0 - locals.var_t0);
        let assign94740_body27_e146840: f64 = (1.0 - locals.var_t1);
        let assign94740_body27_e146841: f64 = (locals.var_phi_b_dpss * assign94740_body27_e146840);
        let assign94740_body27_e146842: f64 = (assign94740_body27_e146836 - assign94740_body27_e146841);
        let assign94740_body27_e146843: f64 = (locals.var_beta * assign94740_body27_e146842);
        (assign94740_body27_e146843, ((locals.var_beta_dn0 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn11 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))), ((locals.var_beta_dn14 * assign94740_body27_e146842) + (locals.var_beta * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign94740_body27_e146840) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2126, locals.var_fbsq_dpss__blk2126_dn0, locals.var_fbsq_dpss__blk2126_dn2, locals.var_fbsq_dpss__blk2126_dn4, locals.var_fbsq_dpss__blk2126_dn5, locals.var_fbsq_dpss__blk2126_dn6, locals.var_fbsq_dpss__blk2126_dn7, locals.var_fbsq_dpss__blk2126_dn8, locals.var_fbsq_dpss__blk2126_dn9, locals.var_fbsq_dpss__blk2126_dn10, locals.var_fbsq_dpss__blk2126_dn11, locals.var_fbsq_dpss__blk2126_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2126 = assign94740_body27_e146845;
            locals.var_fbsq_dpss__blk2126_dn0 = assign94740_body27_e146845_d_n0;
            locals.var_fbsq_dpss__blk2126_dn2 = assign94740_body27_e146845_d_n2;
            locals.var_fbsq_dpss__blk2126_dn4 = assign94740_body27_e146845_d_n4;
            locals.var_fbsq_dpss__blk2126_dn5 = assign94740_body27_e146845_d_n5;
            locals.var_fbsq_dpss__blk2126_dn6 = assign94740_body27_e146845_d_n6;
            locals.var_fbsq_dpss__blk2126_dn7 = assign94740_body27_e146845_d_n7;
            locals.var_fbsq_dpss__blk2126_dn8 = assign94740_body27_e146845_d_n8;
            locals.var_fbsq_dpss__blk2126_dn9 = assign94740_body27_e146845_d_n9;
            locals.var_fbsq_dpss__blk2126_dn10 = assign94740_body27_e146845_d_n10;
            locals.var_fbsq_dpss__blk2126_dn11 = assign94740_body27_e146845_d_n11;
            locals.var_fbsq_dpss__blk2126_dn14 = assign94740_body27_e146845_d_n14;
            locals.var_fbsq_dpss__blk2126_rv = 0.0;
            let assign94740_body28_e146847: f64 = (locals.var_chi).abs();
            let assign94740_body28_e146849: f64 = if assign94740_body28_e146847 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2205 = assign94740_body28_e146849;
            locals.var_guard2205_rv = 0.0;
            let (assign94740_body29_e146881, assign94740_body29_e146881_d_n0, assign94740_body29_e146881_d_n2, assign94740_body29_e146881_d_n4, assign94740_body29_e146881_d_n5, assign94740_body29_e146881_d_n6, assign94740_body29_e146881_d_n7, assign94740_body29_e146881_d_n8, assign94740_body29_e146881_d_n9, assign94740_body29_e146881_d_n10, assign94740_body29_e146881_d_n11, assign94740_body29_e146881_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94740_body29_e146859: f64 = (locals.var_chi * locals.var_chi);
        let assign94740_body29_e146861: f64 = (assign94740_body29_e146859 / 2.0);
        let assign94740_body29_e146865: f64 = (locals.var_chi / 3.0);
        let assign94740_body29_e146869: f64 = (locals.var_chi / 4.0);
        let assign94740_body29_e146873: f64 = (locals.var_chi / 5.0);
        let assign94740_body29_e146874: f64 = (1.0 + assign94740_body29_e146873);
        let assign94740_body29_e146875: f64 = (assign94740_body29_e146869 * assign94740_body29_e146874);
        let assign94740_body29_e146876: f64 = (1.0 + assign94740_body29_e146875);
        let assign94740_body29_e146877: f64 = (assign94740_body29_e146865 * assign94740_body29_e146876);
        let assign94740_body29_e146878: f64 = (1.0 + assign94740_body29_e146877);
        let assign94740_body29_e146879: f64 = (assign94740_body29_e146861 * assign94740_body29_e146878);
        (assign94740_body29_e146879, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn0 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn0 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn2 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn2 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn4 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn4 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn5 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn5 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn6 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn6 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn7 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn7 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn8 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn8 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn9 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn9 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn10 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn10 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn11 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn11 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94740_body29_e146878) + (assign94740_body29_e146861 * (((locals.var_chi_dn14 / 3.0) * assign94740_body29_e146876) + (assign94740_body29_e146865 * (((locals.var_chi_dn14 / 4.0) * assign94740_body29_e146874) + (assign94740_body29_e146869 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94740_body29_e146881;
            locals.var_t0_dn0 = assign94740_body29_e146881_d_n0;
            locals.var_t0_dn2 = assign94740_body29_e146881_d_n2;
            locals.var_t0_dn4 = assign94740_body29_e146881_d_n4;
            locals.var_t0_dn5 = assign94740_body29_e146881_d_n5;
            locals.var_t0_dn6 = assign94740_body29_e146881_d_n6;
            locals.var_t0_dn7 = assign94740_body29_e146881_d_n7;
            locals.var_t0_dn8 = assign94740_body29_e146881_d_n8;
            locals.var_t0_dn9 = assign94740_body29_e146881_d_n9;
            locals.var_t0_dn10 = assign94740_body29_e146881_d_n10;
            locals.var_t0_dn11 = assign94740_body29_e146881_d_n11;
            locals.var_t0_dn14 = assign94740_body29_e146881_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94740_body30_e146909, assign94740_body30_e146909_d_n0, assign94740_body30_e146909_d_n2, assign94740_body30_e146909_d_n4, assign94740_body30_e146909_d_n5, assign94740_body30_e146909_d_n6, assign94740_body30_e146909_d_n7, assign94740_body30_e146909_d_n8, assign94740_body30_e146909_d_n9, assign94740_body30_e146909_d_n10, assign94740_body30_e146909_d_n11, assign94740_body30_e146909_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94740_body30_e146893: f64 = (locals.var_chi / 2.0);
        let assign94740_body30_e146897: f64 = (locals.var_chi / 3.0);
        let assign94740_body30_e146901: f64 = (locals.var_chi / 4.0);
        let assign94740_body30_e146902: f64 = (1.0 + assign94740_body30_e146901);
        let assign94740_body30_e146903: f64 = (assign94740_body30_e146897 * assign94740_body30_e146902);
        let assign94740_body30_e146904: f64 = (1.0 + assign94740_body30_e146903);
        let assign94740_body30_e146905: f64 = (assign94740_body30_e146893 * assign94740_body30_e146904);
        let assign94740_body30_e146906: f64 = (1.0 + assign94740_body30_e146905);
        let assign94740_body30_e146907: f64 = (locals.var_chi * assign94740_body30_e146906);
        (assign94740_body30_e146907, ((locals.var_chi_dn0 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn0 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn2 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn4 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn5 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn6 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn7 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn8 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn9 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn10 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn11 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign94740_body30_e146906) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign94740_body30_e146904) + (assign94740_body30_e146893 * (((locals.var_chi_dn14 / 3.0) * assign94740_body30_e146902) + (assign94740_body30_e146897 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94740_body30_e146909;
            locals.var_t1_dn0 = assign94740_body30_e146909_d_n0;
            locals.var_t1_dn2 = assign94740_body30_e146909_d_n2;
            locals.var_t1_dn4 = assign94740_body30_e146909_d_n4;
            locals.var_t1_dn5 = assign94740_body30_e146909_d_n5;
            locals.var_t1_dn6 = assign94740_body30_e146909_d_n6;
            locals.var_t1_dn7 = assign94740_body30_e146909_d_n7;
            locals.var_t1_dn8 = assign94740_body30_e146909_d_n8;
            locals.var_t1_dn9 = assign94740_body30_e146909_d_n9;
            locals.var_t1_dn10 = assign94740_body30_e146909_d_n10;
            locals.var_t1_dn11 = assign94740_body30_e146909_d_n11;
            locals.var_t1_dn14 = assign94740_body30_e146909_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94740_body31_e146921, assign94740_body31_e146921_d_n0, assign94740_body31_e146921_d_n2, assign94740_body31_e146921_d_n4, assign94740_body31_e146921_d_n5, assign94740_body31_e146921_d_n6, assign94740_body31_e146921_d_n7, assign94740_body31_e146921_d_n8, assign94740_body31_e146921_d_n9, assign94740_body31_e146921_d_n10, assign94740_body31_e146921_d_n11, assign94740_body31_e146921_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94740_body31_e146919: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94740_body31_e146919, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94740_body31_e146921;
            locals.var_fs01_dn0 = assign94740_body31_e146921_d_n0;
            locals.var_fs01_dn2 = assign94740_body31_e146921_d_n2;
            locals.var_fs01_dn4 = assign94740_body31_e146921_d_n4;
            locals.var_fs01_dn5 = assign94740_body31_e146921_d_n5;
            locals.var_fs01_dn6 = assign94740_body31_e146921_d_n6;
            locals.var_fs01_dn7 = assign94740_body31_e146921_d_n7;
            locals.var_fs01_dn8 = assign94740_body31_e146921_d_n8;
            locals.var_fs01_dn9 = assign94740_body31_e146921_d_n9;
            locals.var_fs01_dn10 = assign94740_body31_e146921_d_n10;
            locals.var_fs01_dn11 = assign94740_body31_e146921_d_n11;
            locals.var_fs01_dn14 = assign94740_body31_e146921_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94740_body32_e146935, assign94740_body32_e146935_d_n0, assign94740_body32_e146935_d_n2, assign94740_body32_e146935_d_n4, assign94740_body32_e146935_d_n5, assign94740_body32_e146935_d_n6, assign94740_body32_e146935_d_n7, assign94740_body32_e146935_d_n8, assign94740_body32_e146935_d_n9, assign94740_body32_e146935_d_n10, assign94740_body32_e146935_d_n11, assign94740_body32_e146935_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94740_body32_e146931: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94740_body32_e146933: f64 = (assign94740_body32_e146931 * locals.var_beta);
        (assign94740_body32_e146933, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign94740_body32_e146931 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94740_body32_e146935;
            locals.var_fs01_dps0_dn0 = assign94740_body32_e146935_d_n0;
            locals.var_fs01_dps0_dn2 = assign94740_body32_e146935_d_n2;
            locals.var_fs01_dps0_dn4 = assign94740_body32_e146935_d_n4;
            locals.var_fs01_dps0_dn5 = assign94740_body32_e146935_d_n5;
            locals.var_fs01_dps0_dn6 = assign94740_body32_e146935_d_n6;
            locals.var_fs01_dps0_dn7 = assign94740_body32_e146935_d_n7;
            locals.var_fs01_dps0_dn8 = assign94740_body32_e146935_d_n8;
            locals.var_fs01_dps0_dn9 = assign94740_body32_e146935_d_n9;
            locals.var_fs01_dps0_dn10 = assign94740_body32_e146935_d_n10;
            locals.var_fs01_dps0_dn11 = assign94740_body32_e146935_d_n11;
            locals.var_fs01_dps0_dn14 = assign94740_body32_e146935_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94740_body33_e146937: f64 = (locals.var_chi).abs();
            let assign94740_body33_e146939: f64 = if assign94740_body33_e146937 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2206 = assign94740_body33_e146939;
            locals.var_guard2206_rv = 0.0;
            let (assign94740_body35_e146974, assign94740_body35_e146974_d_n0, assign94740_body35_e146974_d_n2, assign94740_body35_e146974_d_n4, assign94740_body35_e146974_d_n5, assign94740_body35_e146974_d_n6, assign94740_body35_e146974_d_n7, assign94740_body35_e146974_d_n8, assign94740_body35_e146974_d_n9, assign94740_body35_e146974_d_n10, assign94740_body35_e146974_d_n11, assign94740_body35_e146974_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94740_body35_e146972: f64 = (locals.var_chi).exp();
        (assign94740_body35_e146972, (assign94740_body35_e146972 * locals.var_chi_dn0), (assign94740_body35_e146972 * locals.var_chi_dn2), (assign94740_body35_e146972 * locals.var_chi_dn4), (assign94740_body35_e146972 * locals.var_chi_dn5), (assign94740_body35_e146972 * locals.var_chi_dn6), (assign94740_body35_e146972 * locals.var_chi_dn7), (assign94740_body35_e146972 * locals.var_chi_dn8), (assign94740_body35_e146972 * locals.var_chi_dn9), (assign94740_body35_e146972 * locals.var_chi_dn10), (assign94740_body35_e146972 * locals.var_chi_dn11), (assign94740_body35_e146972 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign94740_body35_e146974;
            locals.var_exp_chi_dn0 = assign94740_body35_e146974_d_n0;
            locals.var_exp_chi_dn2 = assign94740_body35_e146974_d_n2;
            locals.var_exp_chi_dn4 = assign94740_body35_e146974_d_n4;
            locals.var_exp_chi_dn5 = assign94740_body35_e146974_d_n5;
            locals.var_exp_chi_dn6 = assign94740_body35_e146974_d_n6;
            locals.var_exp_chi_dn7 = assign94740_body35_e146974_d_n7;
            locals.var_exp_chi_dn8 = assign94740_body35_e146974_d_n8;
            locals.var_exp_chi_dn9 = assign94740_body35_e146974_d_n9;
            locals.var_exp_chi_dn10 = assign94740_body35_e146974_d_n10;
            locals.var_exp_chi_dn11 = assign94740_body35_e146974_d_n11;
            locals.var_exp_chi_dn14 = assign94740_body35_e146974_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign94740_body36_e146989, assign94740_body36_e146989_d_n0, assign94740_body36_e146989_d_n2, assign94740_body36_e146989_d_n4, assign94740_body36_e146989_d_n5, assign94740_body36_e146989_d_n6, assign94740_body36_e146989_d_n7, assign94740_body36_e146989_d_n8, assign94740_body36_e146989_d_n9, assign94740_body36_e146989_d_n10, assign94740_body36_e146989_d_n11, assign94740_body36_e146989_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94740_body36_e146987: f64 = (locals.var_exp_chi - 1.0);
        (assign94740_body36_e146987, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94740_body36_e146989;
            locals.var_t1_dn0 = assign94740_body36_e146989_d_n0;
            locals.var_t1_dn2 = assign94740_body36_e146989_d_n2;
            locals.var_t1_dn4 = assign94740_body36_e146989_d_n4;
            locals.var_t1_dn5 = assign94740_body36_e146989_d_n5;
            locals.var_t1_dn6 = assign94740_body36_e146989_d_n6;
            locals.var_t1_dn7 = assign94740_body36_e146989_d_n7;
            locals.var_t1_dn8 = assign94740_body36_e146989_d_n8;
            locals.var_t1_dn9 = assign94740_body36_e146989_d_n9;
            locals.var_t1_dn10 = assign94740_body36_e146989_d_n10;
            locals.var_t1_dn11 = assign94740_body36_e146989_d_n11;
            locals.var_t1_dn14 = assign94740_body36_e146989_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94740_body37_e147006, assign94740_body37_e147006_d_n0, assign94740_body37_e147006_d_n2, assign94740_body37_e147006_d_n4, assign94740_body37_e147006_d_n5, assign94740_body37_e147006_d_n6, assign94740_body37_e147006_d_n7, assign94740_body37_e147006_d_n8, assign94740_body37_e147006_d_n9, assign94740_body37_e147006_d_n10, assign94740_body37_e147006_d_n11, assign94740_body37_e147006_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94740_body37_e147003: f64 = (locals.var_t1 - locals.var_chi);
        let assign94740_body37_e147004: f64 = (locals.var_cfs1 * assign94740_body37_e147003);
        (assign94740_body37_e147004, ((locals.var_cfs1_dn0 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign94740_body37_e147003) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94740_body37_e147006;
            locals.var_fs01_dn0 = assign94740_body37_e147006_d_n0;
            locals.var_fs01_dn2 = assign94740_body37_e147006_d_n2;
            locals.var_fs01_dn4 = assign94740_body37_e147006_d_n4;
            locals.var_fs01_dn5 = assign94740_body37_e147006_d_n5;
            locals.var_fs01_dn6 = assign94740_body37_e147006_d_n6;
            locals.var_fs01_dn7 = assign94740_body37_e147006_d_n7;
            locals.var_fs01_dn8 = assign94740_body37_e147006_d_n8;
            locals.var_fs01_dn9 = assign94740_body37_e147006_d_n9;
            locals.var_fs01_dn10 = assign94740_body37_e147006_d_n10;
            locals.var_fs01_dn11 = assign94740_body37_e147006_d_n11;
            locals.var_fs01_dn14 = assign94740_body37_e147006_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94740_body38_e147023, assign94740_body38_e147023_d_n0, assign94740_body38_e147023_d_n2, assign94740_body38_e147023_d_n4, assign94740_body38_e147023_d_n5, assign94740_body38_e147023_d_n6, assign94740_body38_e147023_d_n7, assign94740_body38_e147023_d_n8, assign94740_body38_e147023_d_n9, assign94740_body38_e147023_d_n10, assign94740_body38_e147023_d_n11, assign94740_body38_e147023_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94740_body38_e147019: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94740_body38_e147021: f64 = (assign94740_body38_e147019 * locals.var_t1);
        (assign94740_body38_e147021, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign94740_body38_e147019 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94740_body38_e147023;
            locals.var_fs01_dps0_dn0 = assign94740_body38_e147023_d_n0;
            locals.var_fs01_dps0_dn2 = assign94740_body38_e147023_d_n2;
            locals.var_fs01_dps0_dn4 = assign94740_body38_e147023_d_n4;
            locals.var_fs01_dps0_dn5 = assign94740_body38_e147023_d_n5;
            locals.var_fs01_dps0_dn6 = assign94740_body38_e147023_d_n6;
            locals.var_fs01_dps0_dn7 = assign94740_body38_e147023_d_n7;
            locals.var_fs01_dps0_dn8 = assign94740_body38_e147023_d_n8;
            locals.var_fs01_dps0_dn9 = assign94740_body38_e147023_d_n9;
            locals.var_fs01_dps0_dn10 = assign94740_body38_e147023_d_n10;
            locals.var_fs01_dps0_dn11 = assign94740_body38_e147023_d_n11;
            locals.var_fs01_dps0_dn14 = assign94740_body38_e147023_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94740_body40_e147062, assign94740_body40_e147062_d_n0, assign94740_body40_e147062_d_n2, assign94740_body40_e147062_d_n4, assign94740_body40_e147062_d_n5, assign94740_body40_e147062_d_n6, assign94740_body40_e147062_d_n7, assign94740_body40_e147062_d_n8, assign94740_body40_e147062_d_n9, assign94740_body40_e147062_d_n10, assign94740_body40_e147062_d_n11, assign94740_body40_e147062_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94740_body40_e147059: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94740_body40_e147060: f64 = (assign94740_body40_e147059).exp();
        (assign94740_body40_e147060, (assign94740_body40_e147060 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94740_body40_e147060 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94740_body40_e147060 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94740_body40_e147060 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94740_body40_e147060 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94740_body40_e147060 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94740_body40_e147060 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94740_body40_e147060 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94740_body40_e147060 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94740_body40_e147060 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign94740_body40_e147060 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign94740_body40_e147062;
            locals.var_exp_bps0_dn0 = assign94740_body40_e147062_d_n0;
            locals.var_exp_bps0_dn2 = assign94740_body40_e147062_d_n2;
            locals.var_exp_bps0_dn4 = assign94740_body40_e147062_d_n4;
            locals.var_exp_bps0_dn5 = assign94740_body40_e147062_d_n5;
            locals.var_exp_bps0_dn6 = assign94740_body40_e147062_d_n6;
            locals.var_exp_bps0_dn7 = assign94740_body40_e147062_d_n7;
            locals.var_exp_bps0_dn8 = assign94740_body40_e147062_d_n8;
            locals.var_exp_bps0_dn9 = assign94740_body40_e147062_d_n9;
            locals.var_exp_bps0_dn10 = assign94740_body40_e147062_d_n10;
            locals.var_exp_bps0_dn11 = assign94740_body40_e147062_d_n11;
            locals.var_exp_bps0_dn14 = assign94740_body40_e147062_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94740_body41_e147084, assign94740_body41_e147084_d_n0, assign94740_body41_e147084_d_n2, assign94740_body41_e147084_d_n4, assign94740_body41_e147084_d_n5, assign94740_body41_e147084_d_n6, assign94740_body41_e147084_d_n7, assign94740_body41_e147084_d_n8, assign94740_body41_e147084_d_n9, assign94740_body41_e147084_d_n10, assign94740_body41_e147084_d_n11, assign94740_body41_e147084_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94740_body41_e147079: f64 = (locals.var_chi + 1.0);
        let assign94740_body41_e147080: f64 = (locals.var_exp_bvbs * assign94740_body41_e147079);
        let assign94740_body41_e147081: f64 = (locals.var_exp_bps0 - assign94740_body41_e147080);
        let assign94740_body41_e147082: f64 = (locals.var_cnst1over * assign94740_body41_e147081);
        (assign94740_body41_e147082, ((locals.var_cnst1over_dn0 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign94740_body41_e147081) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign94740_body41_e147079) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94740_body41_e147084;
            locals.var_fs01_dn0 = assign94740_body41_e147084_d_n0;
            locals.var_fs01_dn2 = assign94740_body41_e147084_d_n2;
            locals.var_fs01_dn4 = assign94740_body41_e147084_d_n4;
            locals.var_fs01_dn5 = assign94740_body41_e147084_d_n5;
            locals.var_fs01_dn6 = assign94740_body41_e147084_d_n6;
            locals.var_fs01_dn7 = assign94740_body41_e147084_d_n7;
            locals.var_fs01_dn8 = assign94740_body41_e147084_d_n8;
            locals.var_fs01_dn9 = assign94740_body41_e147084_d_n9;
            locals.var_fs01_dn10 = assign94740_body41_e147084_d_n10;
            locals.var_fs01_dn11 = assign94740_body41_e147084_d_n11;
            locals.var_fs01_dn14 = assign94740_body41_e147084_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94740_body42_e147104, assign94740_body42_e147104_d_n0, assign94740_body42_e147104_d_n2, assign94740_body42_e147104_d_n4, assign94740_body42_e147104_d_n5, assign94740_body42_e147104_d_n6, assign94740_body42_e147104_d_n7, assign94740_body42_e147104_d_n8, assign94740_body42_e147104_d_n9, assign94740_body42_e147104_d_n10, assign94740_body42_e147104_d_n11, assign94740_body42_e147104_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94740_body42_e147098: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94740_body42_e147101: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94740_body42_e147102: f64 = (assign94740_body42_e147098 * assign94740_body42_e147101);
        (assign94740_body42_e147102, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign94740_body42_e147101) + (assign94740_body42_e147098 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94740_body42_e147104;
            locals.var_fs01_dps0_dn0 = assign94740_body42_e147104_d_n0;
            locals.var_fs01_dps0_dn2 = assign94740_body42_e147104_d_n2;
            locals.var_fs01_dps0_dn4 = assign94740_body42_e147104_d_n4;
            locals.var_fs01_dps0_dn5 = assign94740_body42_e147104_d_n5;
            locals.var_fs01_dps0_dn6 = assign94740_body42_e147104_d_n6;
            locals.var_fs01_dps0_dn7 = assign94740_body42_e147104_d_n7;
            locals.var_fs01_dps0_dn8 = assign94740_body42_e147104_d_n8;
            locals.var_fs01_dps0_dn9 = assign94740_body42_e147104_d_n9;
            locals.var_fs01_dps0_dn10 = assign94740_body42_e147104_d_n10;
            locals.var_fs01_dps0_dn11 = assign94740_body42_e147104_d_n11;
            locals.var_fs01_dps0_dn14 = assign94740_body42_e147104_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94740_body43_e147107: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2207 = assign94740_body43_e147107;
            locals.var_guard2207_rv = 0.0;
            let (assign94740_body44_e147120, assign94740_body44_e147120_d_n0, assign94740_body44_e147120_d_n2, assign94740_body44_e147120_d_n4, assign94740_body44_e147120_d_n5, assign94740_body44_e147120_d_n6, assign94740_body44_e147120_d_n7, assign94740_body44_e147120_d_n8, assign94740_body44_e147120_d_n9, assign94740_body44_e147120_d_n10, assign94740_body44_e147120_d_n11, assign94740_body44_e147120_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94740_body44_e147117: f64 = (locals.var_fbsq__blk2125 + locals.var_fs01);
        let assign94740_body44_e147118: f64 = (assign94740_body44_e147117).sqrt();
        (assign94740_body44_e147118, ((locals.var_fbsq__blk2125_dn0 + locals.var_fs01_dn0) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn2 + locals.var_fs01_dn2) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn4 + locals.var_fs01_dn4) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn5 + locals.var_fs01_dn5) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn6 + locals.var_fs01_dn6) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn7 + locals.var_fs01_dn7) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn8 + locals.var_fs01_dn8) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn9 + locals.var_fs01_dn9) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn10 + locals.var_fs01_dn10) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn11 + locals.var_fs01_dn11) / (2.0 * assign94740_body44_e147118)), ((locals.var_fbsq__blk2125_dn14 + locals.var_fs01_dn14) / (2.0 * assign94740_body44_e147118)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94740_body44_e147120;
            locals.var_fs02_dn0 = assign94740_body44_e147120_d_n0;
            locals.var_fs02_dn2 = assign94740_body44_e147120_d_n2;
            locals.var_fs02_dn4 = assign94740_body44_e147120_d_n4;
            locals.var_fs02_dn5 = assign94740_body44_e147120_d_n5;
            locals.var_fs02_dn6 = assign94740_body44_e147120_d_n6;
            locals.var_fs02_dn7 = assign94740_body44_e147120_d_n7;
            locals.var_fs02_dn8 = assign94740_body44_e147120_d_n8;
            locals.var_fs02_dn9 = assign94740_body44_e147120_d_n9;
            locals.var_fs02_dn10 = assign94740_body44_e147120_d_n10;
            locals.var_fs02_dn11 = assign94740_body44_e147120_d_n11;
            locals.var_fs02_dn14 = assign94740_body44_e147120_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94740_body45_e147136, assign94740_body45_e147136_d_n0, assign94740_body45_e147136_d_n2, assign94740_body45_e147136_d_n4, assign94740_body45_e147136_d_n5, assign94740_body45_e147136_d_n6, assign94740_body45_e147136_d_n7, assign94740_body45_e147136_d_n8, assign94740_body45_e147136_d_n9, assign94740_body45_e147136_d_n10, assign94740_body45_e147136_d_n11, assign94740_body45_e147136_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94740_body45_e147131: f64 = (locals.var_fbsq_dpss__blk2126 + locals.var_fs01_dps0);
        let assign94740_body45_e147132: f64 = (0.5 * assign94740_body45_e147131);
        let assign94740_body45_e147134: f64 = (assign94740_body45_e147132 / locals.var_fs02);
        (assign94740_body45_e147134, ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn11 + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2126_dn14 + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign94740_body45_e147132 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94740_body45_e147136;
            locals.var_fs02_dps0_dn0 = assign94740_body45_e147136_d_n0;
            locals.var_fs02_dps0_dn2 = assign94740_body45_e147136_d_n2;
            locals.var_fs02_dps0_dn4 = assign94740_body45_e147136_d_n4;
            locals.var_fs02_dps0_dn5 = assign94740_body45_e147136_d_n5;
            locals.var_fs02_dps0_dn6 = assign94740_body45_e147136_d_n6;
            locals.var_fs02_dps0_dn7 = assign94740_body45_e147136_d_n7;
            locals.var_fs02_dps0_dn8 = assign94740_body45_e147136_d_n8;
            locals.var_fs02_dps0_dn9 = assign94740_body45_e147136_d_n9;
            locals.var_fs02_dps0_dn10 = assign94740_body45_e147136_d_n10;
            locals.var_fs02_dps0_dn11 = assign94740_body45_e147136_d_n11;
            locals.var_fs02_dps0_dn14 = assign94740_body45_e147136_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94740_body46_e147139: f64 = if locals.var_fbsq__blk2125 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2208 = assign94740_body46_e147139;
            locals.var_guard2208_rv = 0.0;
            let (assign94740_body47_e147153, assign94740_body47_e147153_d_n0, assign94740_body47_e147153_d_n2, assign94740_body47_e147153_d_n4, assign94740_body47_e147153_d_n5, assign94740_body47_e147153_d_n6, assign94740_body47_e147153_d_n7, assign94740_body47_e147153_d_n8, assign94740_body47_e147153_d_n9, assign94740_body47_e147153_d_n10, assign94740_body47_e147153_d_n11, assign94740_body47_e147153_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94740_body47_e147151: f64 = (locals.var_fbsq__blk2125).sqrt();
        (assign94740_body47_e147151, (locals.var_fbsq__blk2125_dn0 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn2 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn4 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn5 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn6 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn7 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn8 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn9 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn10 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn11 / (2.0 * assign94740_body47_e147151)), (locals.var_fbsq__blk2125_dn14 / (2.0 * assign94740_body47_e147151)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94740_body47_e147153;
            locals.var_fs02_dn0 = assign94740_body47_e147153_d_n0;
            locals.var_fs02_dn2 = assign94740_body47_e147153_d_n2;
            locals.var_fs02_dn4 = assign94740_body47_e147153_d_n4;
            locals.var_fs02_dn5 = assign94740_body47_e147153_d_n5;
            locals.var_fs02_dn6 = assign94740_body47_e147153_d_n6;
            locals.var_fs02_dn7 = assign94740_body47_e147153_d_n7;
            locals.var_fs02_dn8 = assign94740_body47_e147153_d_n8;
            locals.var_fs02_dn9 = assign94740_body47_e147153_d_n9;
            locals.var_fs02_dn10 = assign94740_body47_e147153_d_n10;
            locals.var_fs02_dn11 = assign94740_body47_e147153_d_n11;
            locals.var_fs02_dn14 = assign94740_body47_e147153_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94740_body48_e147170, assign94740_body48_e147170_d_n0, assign94740_body48_e147170_d_n2, assign94740_body48_e147170_d_n4, assign94740_body48_e147170_d_n5, assign94740_body48_e147170_d_n6, assign94740_body48_e147170_d_n7, assign94740_body48_e147170_d_n8, assign94740_body48_e147170_d_n9, assign94740_body48_e147170_d_n10, assign94740_body48_e147170_d_n11, assign94740_body48_e147170_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94740_body48_e147166: f64 = (0.5 * locals.var_fbsq_dpss__blk2126);
        let assign94740_body48_e147168: f64 = (assign94740_body48_e147166 / locals.var_fs02);
        (assign94740_body48_e147168, ((((0.5 * locals.var_fbsq_dpss__blk2126_dn0) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn2) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn4) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn5) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn6) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn7) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn8) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn9) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn10) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn11) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2126_dn14) * locals.var_fs02) - (assign94740_body48_e147166 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94740_body48_e147170;
            locals.var_fs02_dps0_dn0 = assign94740_body48_e147170_d_n0;
            locals.var_fs02_dps0_dn2 = assign94740_body48_e147170_d_n2;
            locals.var_fs02_dps0_dn4 = assign94740_body48_e147170_d_n4;
            locals.var_fs02_dps0_dn5 = assign94740_body48_e147170_d_n5;
            locals.var_fs02_dps0_dn6 = assign94740_body48_e147170_d_n6;
            locals.var_fs02_dps0_dn7 = assign94740_body48_e147170_d_n7;
            locals.var_fs02_dps0_dn8 = assign94740_body48_e147170_d_n8;
            locals.var_fs02_dps0_dn9 = assign94740_body48_e147170_d_n9;
            locals.var_fs02_dps0_dn10 = assign94740_body48_e147170_d_n10;
            locals.var_fs02_dps0_dn11 = assign94740_body48_e147170_d_n11;
            locals.var_fs02_dps0_dn14 = assign94740_body48_e147170_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94740_body49_e147184, assign94740_body49_e147184_d_n0, assign94740_body49_e147184_d_n2, assign94740_body49_e147184_d_n4, assign94740_body49_e147184_d_n5, assign94740_body49_e147184_d_n6, assign94740_body49_e147184_d_n7, assign94740_body49_e147184_d_n8, assign94740_body49_e147184_d_n9, assign94740_body49_e147184_d_n10, assign94740_body49_e147184_d_n11, assign94740_body49_e147184_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94740_body49_e147184;
            locals.var_fs02_dn0 = assign94740_body49_e147184_d_n0;
            locals.var_fs02_dn2 = assign94740_body49_e147184_d_n2;
            locals.var_fs02_dn4 = assign94740_body49_e147184_d_n4;
            locals.var_fs02_dn5 = assign94740_body49_e147184_d_n5;
            locals.var_fs02_dn6 = assign94740_body49_e147184_d_n6;
            locals.var_fs02_dn7 = assign94740_body49_e147184_d_n7;
            locals.var_fs02_dn8 = assign94740_body49_e147184_d_n8;
            locals.var_fs02_dn9 = assign94740_body49_e147184_d_n9;
            locals.var_fs02_dn10 = assign94740_body49_e147184_d_n10;
            locals.var_fs02_dn11 = assign94740_body49_e147184_d_n11;
            locals.var_fs02_dn14 = assign94740_body49_e147184_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94740_body50_e147198, assign94740_body50_e147198_d_n0, assign94740_body50_e147198_d_n2, assign94740_body50_e147198_d_n4, assign94740_body50_e147198_d_n5, assign94740_body50_e147198_d_n6, assign94740_body50_e147198_d_n7, assign94740_body50_e147198_d_n8, assign94740_body50_e147198_d_n9, assign94740_body50_e147198_d_n10, assign94740_body50_e147198_d_n11, assign94740_body50_e147198_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94740_body50_e147198;
            locals.var_fs02_dps0_dn0 = assign94740_body50_e147198_d_n0;
            locals.var_fs02_dps0_dn2 = assign94740_body50_e147198_d_n2;
            locals.var_fs02_dps0_dn4 = assign94740_body50_e147198_d_n4;
            locals.var_fs02_dps0_dn5 = assign94740_body50_e147198_d_n5;
            locals.var_fs02_dps0_dn6 = assign94740_body50_e147198_d_n6;
            locals.var_fs02_dps0_dn7 = assign94740_body50_e147198_d_n7;
            locals.var_fs02_dps0_dn8 = assign94740_body50_e147198_d_n8;
            locals.var_fs02_dps0_dn9 = assign94740_body50_e147198_d_n9;
            locals.var_fs02_dps0_dn10 = assign94740_body50_e147198_d_n10;
            locals.var_fs02_dps0_dn11 = assign94740_body50_e147198_d_n11;
            locals.var_fs02_dps0_dn14 = assign94740_body50_e147198_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94740_body51_e147214, assign94740_body51_e147214_d_n0, assign94740_body51_e147214_d_n2, assign94740_body51_e147214_d_n4, assign94740_body51_e147214_d_n5, assign94740_body51_e147214_d_n6, assign94740_body51_e147214_d_n7, assign94740_body51_e147214_d_n8, assign94740_body51_e147214_d_n9, assign94740_body51_e147214_d_n10, assign94740_body51_e147214_d_n11, assign94740_body51_e147214_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let (assign94740_body51_e147210,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94740_body51_e147209: f64 = (-1.0);
                (assign94740_body51_e147209,)
            }
        };
        let assign94740_body51_e147212: f64 = (assign94740_body51_e147210 * locals.var_fs02);
        (assign94740_body51_e147212, (assign94740_body51_e147210 * locals.var_fs02_dn0), (assign94740_body51_e147210 * locals.var_fs02_dn2), (assign94740_body51_e147210 * locals.var_fs02_dn4), (assign94740_body51_e147210 * locals.var_fs02_dn5), (assign94740_body51_e147210 * locals.var_fs02_dn6), (assign94740_body51_e147210 * locals.var_fs02_dn7), (assign94740_body51_e147210 * locals.var_fs02_dn8), (assign94740_body51_e147210 * locals.var_fs02_dn9), (assign94740_body51_e147210 * locals.var_fs02_dn10), (assign94740_body51_e147210 * locals.var_fs02_dn11), (assign94740_body51_e147210 * locals.var_fs02_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94740_body51_e147214;
            locals.var_fs02_dn0 = assign94740_body51_e147214_d_n0;
            locals.var_fs02_dn2 = assign94740_body51_e147214_d_n2;
            locals.var_fs02_dn4 = assign94740_body51_e147214_d_n4;
            locals.var_fs02_dn5 = assign94740_body51_e147214_d_n5;
            locals.var_fs02_dn6 = assign94740_body51_e147214_d_n6;
            locals.var_fs02_dn7 = assign94740_body51_e147214_d_n7;
            locals.var_fs02_dn8 = assign94740_body51_e147214_d_n8;
            locals.var_fs02_dn9 = assign94740_body51_e147214_d_n9;
            locals.var_fs02_dn10 = assign94740_body51_e147214_d_n10;
            locals.var_fs02_dn11 = assign94740_body51_e147214_d_n11;
            locals.var_fs02_dn14 = assign94740_body51_e147214_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94740_body52_e147230, assign94740_body52_e147230_d_n0, assign94740_body52_e147230_d_n2, assign94740_body52_e147230_d_n4, assign94740_body52_e147230_d_n5, assign94740_body52_e147230_d_n6, assign94740_body52_e147230_d_n7, assign94740_body52_e147230_d_n8, assign94740_body52_e147230_d_n9, assign94740_body52_e147230_d_n10, assign94740_body52_e147230_d_n11, assign94740_body52_e147230_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let (assign94740_body52_e147226,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94740_body52_e147225: f64 = (-1.0);
                (assign94740_body52_e147225,)
            }
        };
        let assign94740_body52_e147228: f64 = (assign94740_body52_e147226 * locals.var_fs02_dps0);
        (assign94740_body52_e147228, (assign94740_body52_e147226 * locals.var_fs02_dps0_dn0), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn2), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn4), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn5), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn6), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn7), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn8), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn9), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn10), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn11), (assign94740_body52_e147226 * locals.var_fs02_dps0_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94740_body52_e147230;
            locals.var_fs02_dps0_dn0 = assign94740_body52_e147230_d_n0;
            locals.var_fs02_dps0_dn2 = assign94740_body52_e147230_d_n2;
            locals.var_fs02_dps0_dn4 = assign94740_body52_e147230_d_n4;
            locals.var_fs02_dps0_dn5 = assign94740_body52_e147230_d_n5;
            locals.var_fs02_dps0_dn6 = assign94740_body52_e147230_d_n6;
            locals.var_fs02_dps0_dn7 = assign94740_body52_e147230_d_n7;
            locals.var_fs02_dps0_dn8 = assign94740_body52_e147230_d_n8;
            locals.var_fs02_dps0_dn9 = assign94740_body52_e147230_d_n9;
            locals.var_fs02_dps0_dn10 = assign94740_body52_e147230_d_n10;
            locals.var_fs02_dps0_dn11 = assign94740_body52_e147230_d_n11;
            locals.var_fs02_dps0_dn14 = assign94740_body52_e147230_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94740_body53_e147245, assign94740_body53_e147245_d_n0, assign94740_body53_e147245_d_n2, assign94740_body53_e147245_d_n4, assign94740_body53_e147245_d_n5, assign94740_body53_e147245_d_n6, assign94740_body53_e147245_d_n7, assign94740_body53_e147245_d_n8, assign94740_body53_e147245_d_n9, assign94740_body53_e147245_d_n10, assign94740_body53_e147245_d_n11, assign94740_body53_e147245_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body53_e147237: f64 = (-locals.var_vgpld);
        let assign94740_body53_e147239: f64 = (assign94740_body53_e147237 + locals.var_ps0ld);
        let assign94740_body53_e147242: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94740_body53_e147243: f64 = (assign94740_body53_e147239 + assign94740_body53_e147242);
        (assign94740_body53_e147243, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign94740_body53_e147245;
            locals.var_fs0_dn0 = assign94740_body53_e147245_d_n0;
            locals.var_fs0_dn2 = assign94740_body53_e147245_d_n2;
            locals.var_fs0_dn4 = assign94740_body53_e147245_d_n4;
            locals.var_fs0_dn5 = assign94740_body53_e147245_d_n5;
            locals.var_fs0_dn6 = assign94740_body53_e147245_d_n6;
            locals.var_fs0_dn7 = assign94740_body53_e147245_d_n7;
            locals.var_fs0_dn8 = assign94740_body53_e147245_d_n8;
            locals.var_fs0_dn9 = assign94740_body53_e147245_d_n9;
            locals.var_fs0_dn10 = assign94740_body53_e147245_d_n10;
            locals.var_fs0_dn11 = assign94740_body53_e147245_d_n11;
            locals.var_fs0_dn14 = assign94740_body53_e147245_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign94740_body54_e147257, assign94740_body54_e147257_d_n0, assign94740_body54_e147257_d_n2, assign94740_body54_e147257_d_n4, assign94740_body54_e147257_d_n5, assign94740_body54_e147257_d_n6, assign94740_body54_e147257_d_n7, assign94740_body54_e147257_d_n8, assign94740_body54_e147257_d_n9, assign94740_body54_e147257_d_n10, assign94740_body54_e147257_d_n11, assign94740_body54_e147257_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body54_e147254: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94740_body54_e147255: f64 = (1.0 + assign94740_body54_e147254);
        (assign94740_body54_e147255, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign94740_body54_e147257;
            locals.var_fs0_dps0_dn0 = assign94740_body54_e147257_d_n0;
            locals.var_fs0_dps0_dn2 = assign94740_body54_e147257_d_n2;
            locals.var_fs0_dps0_dn4 = assign94740_body54_e147257_d_n4;
            locals.var_fs0_dps0_dn5 = assign94740_body54_e147257_d_n5;
            locals.var_fs0_dps0_dn6 = assign94740_body54_e147257_d_n6;
            locals.var_fs0_dps0_dn7 = assign94740_body54_e147257_d_n7;
            locals.var_fs0_dps0_dn8 = assign94740_body54_e147257_d_n8;
            locals.var_fs0_dps0_dn9 = assign94740_body54_e147257_d_n9;
            locals.var_fs0_dps0_dn10 = assign94740_body54_e147257_d_n10;
            locals.var_fs0_dps0_dn11 = assign94740_body54_e147257_d_n11;
            locals.var_fs0_dps0_dn14 = assign94740_body54_e147257_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94740_body55_e147260: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2209 = assign94740_body55_e147260;
            locals.var_guard2209_rv = 0.0;
            let (assign94740_body56_e147272,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 != 0.0)) {
        let assign94740_body56_e147270: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94740_body56_e147270,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94740_body56_e147272;
            locals.var_lp_s0_rv = 0.0;
            let (assign94740_body57_e147286, assign94740_body57_e147286_d_n0, assign94740_body57_e147286_d_n2, assign94740_body57_e147286_d_n4, assign94740_body57_e147286_d_n5, assign94740_body57_e147286_d_n6, assign94740_body57_e147286_d_n7, assign94740_body57_e147286_d_n8, assign94740_body57_e147286_d_n9, assign94740_body57_e147286_d_n10, assign94740_body57_e147286_d_n11, assign94740_body57_e147286_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 == 0.0)) {
        let assign94740_body57_e147282: f64 = (-locals.var_fs0);
        let assign94740_body57_e147284: f64 = (assign94740_body57_e147282 / locals.var_fs0_dps0);
        (assign94740_body57_e147284, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign94740_body57_e147282 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94740_body57_e147286;
            locals.var_dps0_dn0 = assign94740_body57_e147286_d_n0;
            locals.var_dps0_dn2 = assign94740_body57_e147286_d_n2;
            locals.var_dps0_dn4 = assign94740_body57_e147286_d_n4;
            locals.var_dps0_dn5 = assign94740_body57_e147286_d_n5;
            locals.var_dps0_dn6 = assign94740_body57_e147286_d_n6;
            locals.var_dps0_dn7 = assign94740_body57_e147286_d_n7;
            locals.var_dps0_dn8 = assign94740_body57_e147286_d_n8;
            locals.var_dps0_dn9 = assign94740_body57_e147286_d_n9;
            locals.var_dps0_dn10 = assign94740_body57_e147286_d_n10;
            locals.var_dps0_dn11 = assign94740_body57_e147286_d_n11;
            locals.var_dps0_dn14 = assign94740_body57_e147286_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94740_body58_e147310, assign94740_body58_e147310_d_n0, assign94740_body58_e147310_d_n2, assign94740_body58_e147310_d_n4, assign94740_body58_e147310_d_n5, assign94740_body58_e147310_d_n6, assign94740_body58_e147310_d_n7, assign94740_body58_e147310_d_n8, assign94740_body58_e147310_d_n9, assign94740_body58_e147310_d_n10, assign94740_body58_e147310_d_n11, assign94740_body58_e147310_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 == 0.0)) {
        let assign94740_body58_e147297: f64 = (0.5 * 0.1);
        let assign94740_body58_e147301: f64 = (locals.var_ps0ld).abs();
        let (assign94740_body58_e147306, assign94740_body58_e147306_d_n0, assign94740_body58_e147306_d_n2, assign94740_body58_e147306_d_n4, assign94740_body58_e147306_d_n5, assign94740_body58_e147306_d_n6, assign94740_body58_e147306_d_n7, assign94740_body58_e147306_d_n8, assign94740_body58_e147306_d_n9, assign94740_body58_e147306_d_n10, assign94740_body58_e147306_d_n11, assign94740_body58_e147306_d_n14,) = {
            if (1.0 >= assign94740_body58_e147301) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94740_body58_e147305: f64 = (locals.var_ps0ld).abs();
                (assign94740_body58_e147305, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign94740_body58_e147307: f64 = (1.0 + assign94740_body58_e147306);
        let assign94740_body58_e147308: f64 = (assign94740_body58_e147297 * assign94740_body58_e147307);
        (assign94740_body58_e147308, (assign94740_body58_e147297 * assign94740_body58_e147306_d_n0), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n2), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n4), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n5), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n6), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n7), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n8), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n9), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n10), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n11), (assign94740_body58_e147297 * assign94740_body58_e147306_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign94740_body58_e147310;
            locals.var_dplim_dn0 = assign94740_body58_e147310_d_n0;
            locals.var_dplim_dn2 = assign94740_body58_e147310_d_n2;
            locals.var_dplim_dn4 = assign94740_body58_e147310_d_n4;
            locals.var_dplim_dn5 = assign94740_body58_e147310_d_n5;
            locals.var_dplim_dn6 = assign94740_body58_e147310_d_n6;
            locals.var_dplim_dn7 = assign94740_body58_e147310_d_n7;
            locals.var_dplim_dn8 = assign94740_body58_e147310_d_n8;
            locals.var_dplim_dn9 = assign94740_body58_e147310_d_n9;
            locals.var_dplim_dn10 = assign94740_body58_e147310_d_n10;
            locals.var_dplim_dn11 = assign94740_body58_e147310_d_n11;
            locals.var_dplim_dn14 = assign94740_body58_e147310_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign94740_body59_e147312: f64 = (locals.var_dps0).abs();
            let assign94740_body59_e147314: f64 = if assign94740_body59_e147312 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2210 = assign94740_body59_e147314;
            locals.var_guard2210_rv = 0.0;
            let (assign94740_body60_e147335, assign94740_body60_e147335_d_n0, assign94740_body60_e147335_d_n2, assign94740_body60_e147335_d_n4, assign94740_body60_e147335_d_n5, assign94740_body60_e147335_d_n6, assign94740_body60_e147335_d_n7, assign94740_body60_e147335_d_n8, assign94740_body60_e147335_d_n9, assign94740_body60_e147335_d_n10, assign94740_body60_e147335_d_n11, assign94740_body60_e147335_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2210 != 0.0)) {
        let (assign94740_body60_e147332,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94740_body60_e147331: f64 = (-1.0);
                (assign94740_body60_e147331,)
            }
        };
        let assign94740_body60_e147333: f64 = (locals.var_dplim * assign94740_body60_e147332);
        (assign94740_body60_e147333, (locals.var_dplim_dn0 * assign94740_body60_e147332), (locals.var_dplim_dn2 * assign94740_body60_e147332), (locals.var_dplim_dn4 * assign94740_body60_e147332), (locals.var_dplim_dn5 * assign94740_body60_e147332), (locals.var_dplim_dn6 * assign94740_body60_e147332), (locals.var_dplim_dn7 * assign94740_body60_e147332), (locals.var_dplim_dn8 * assign94740_body60_e147332), (locals.var_dplim_dn9 * assign94740_body60_e147332), (locals.var_dplim_dn10 * assign94740_body60_e147332), (locals.var_dplim_dn11 * assign94740_body60_e147332), (locals.var_dplim_dn14 * assign94740_body60_e147332),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94740_body60_e147335;
            locals.var_dps0_dn0 = assign94740_body60_e147335_d_n0;
            locals.var_dps0_dn2 = assign94740_body60_e147335_d_n2;
            locals.var_dps0_dn4 = assign94740_body60_e147335_d_n4;
            locals.var_dps0_dn5 = assign94740_body60_e147335_d_n5;
            locals.var_dps0_dn6 = assign94740_body60_e147335_d_n6;
            locals.var_dps0_dn7 = assign94740_body60_e147335_d_n7;
            locals.var_dps0_dn8 = assign94740_body60_e147335_d_n8;
            locals.var_dps0_dn9 = assign94740_body60_e147335_d_n9;
            locals.var_dps0_dn10 = assign94740_body60_e147335_d_n10;
            locals.var_dps0_dn11 = assign94740_body60_e147335_d_n11;
            locals.var_dps0_dn14 = assign94740_body60_e147335_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94740_body61_e147348, assign94740_body61_e147348_d_n0, assign94740_body61_e147348_d_n2, assign94740_body61_e147348_d_n4, assign94740_body61_e147348_d_n5, assign94740_body61_e147348_d_n6, assign94740_body61_e147348_d_n7, assign94740_body61_e147348_d_n8, assign94740_body61_e147348_d_n9, assign94740_body61_e147348_d_n10, assign94740_body61_e147348_d_n11, assign94740_body61_e147348_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 == 0.0)) {
        let assign94740_body61_e147346: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94740_body61_e147346, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign94740_body61_e147348;
            locals.var_ps0ld_dn0 = assign94740_body61_e147348_d_n0;
            locals.var_ps0ld_dn2 = assign94740_body61_e147348_d_n2;
            locals.var_ps0ld_dn4 = assign94740_body61_e147348_d_n4;
            locals.var_ps0ld_dn5 = assign94740_body61_e147348_d_n5;
            locals.var_ps0ld_dn6 = assign94740_body61_e147348_d_n6;
            locals.var_ps0ld_dn7 = assign94740_body61_e147348_d_n7;
            locals.var_ps0ld_dn8 = assign94740_body61_e147348_d_n8;
            locals.var_ps0ld_dn9 = assign94740_body61_e147348_d_n9;
            locals.var_ps0ld_dn10 = assign94740_body61_e147348_d_n10;
            locals.var_ps0ld_dn11 = assign94740_body61_e147348_d_n11;
            locals.var_ps0ld_dn14 = assign94740_body61_e147348_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign94740_body62_e147350: f64 = (locals.var_dps0).abs();
            let assign94740_body62_e147354: f64 = (locals.var_fs0).abs();
            let assign94740_body62_e147357: f64 = if ((assign94740_body62_e147350 <= 1e-12) && (assign94740_body62_e147354 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2211 = assign94740_body62_e147357;
            locals.var_guard2211_rv = 0.0;
            let (assign94740_body63_e147372,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2211 != 0.0)) {
        let assign94740_body63_e147370: f64 = (locals.var_flg_conv + 2.0);
        (assign94740_body63_e147370,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94740_body63_e147372;
            locals.var_flg_conv_rv = 0.0;
            let (assign94740_body64_e147382,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94740_body64_e147380: f64 = (locals.var_lp_s0 + 1.0);
        (assign94740_body64_e147380,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94740_body64_e147382;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_368(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94760_e147407, assign94760_e147407_d_n0, assign94760_e147407_d_n2, assign94760_e147407_d_n4, assign94760_e147407_d_n5, assign94760_e147407_d_n6, assign94760_e147407_d_n7, assign94760_e147407_d_n8, assign94760_e147407_d_n9, assign94760_e147407_d_n10, assign94760_e147407_d_n11, assign94760_e147407_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let (assign94760_e147405, assign94760_e147405_d_n0, assign94760_e147405_d_n2, assign94760_e147405_d_n4, assign94760_e147405_d_n5, assign94760_e147405_d_n6, assign94760_e147405_d_n7, assign94760_e147405_d_n8, assign94760_e147405_d_n9, assign94760_e147405_d_n10, assign94760_e147405_d_n11, assign94760_e147405_d_n14,) = {
            if (locals.var_fbsq__blk2125 >= 0.0) {
                let (assign94760_e147400,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign94760_e147399: f64 = (-1.0);
                        (assign94760_e147399,)
                    }
                };
                let assign94760_e147402: f64 = (locals.var_fbsq__blk2125).sqrt();
                let assign94760_e147403: f64 = (assign94760_e147400 * assign94760_e147402);
                (assign94760_e147403, (assign94760_e147400 * (locals.var_fbsq__blk2125_dn0 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn2 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn4 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn5 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn6 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn7 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn8 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn9 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn10 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn11 / (2.0 * assign94760_e147402))), (assign94760_e147400 * (locals.var_fbsq__blk2125_dn14 / (2.0 * assign94760_e147402))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign94760_e147405, assign94760_e147405_d_n0, assign94760_e147405_d_n2, assign94760_e147405_d_n4, assign94760_e147405_d_n5, assign94760_e147405_d_n6, assign94760_e147405_d_n7, assign94760_e147405_d_n8, assign94760_e147405_d_n9, assign94760_e147405_d_n10, assign94760_e147405_d_n11, assign94760_e147405_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign94760_e147407;
        locals.var_fb_dn0 = assign94760_e147407_d_n0;
        locals.var_fb_dn2 = assign94760_e147407_d_n2;
        locals.var_fb_dn4 = assign94760_e147407_d_n4;
        locals.var_fb_dn5 = assign94760_e147407_d_n5;
        locals.var_fb_dn6 = assign94760_e147407_d_n6;
        locals.var_fb_dn7 = assign94760_e147407_d_n7;
        locals.var_fb_dn8 = assign94760_e147407_d_n8;
        locals.var_fb_dn9 = assign94760_e147407_d_n9;
        locals.var_fb_dn10 = assign94760_e147407_d_n10;
        locals.var_fb_dn11 = assign94760_e147407_d_n11;
        locals.var_fb_dn14 = assign94760_e147407_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign94770_e147417, assign94770_e147417_d_n0, assign94770_e147417_d_n2, assign94770_e147417_d_n4, assign94770_e147417_d_n5, assign94770_e147417_d_n6, assign94770_e147417_d_n7, assign94770_e147417_d_n8, assign94770_e147417_d_n9, assign94770_e147417_d_n10, assign94770_e147417_d_n11, assign94770_e147417_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94770_e147415: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94770_e147415, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2115, locals.var_wdld__blk2115_dn0, locals.var_wdld__blk2115_dn2, locals.var_wdld__blk2115_dn4, locals.var_wdld__blk2115_dn5, locals.var_wdld__blk2115_dn6, locals.var_wdld__blk2115_dn7, locals.var_wdld__blk2115_dn8, locals.var_wdld__blk2115_dn9, locals.var_wdld__blk2115_dn10, locals.var_wdld__blk2115_dn11, locals.var_wdld__blk2115_dn14,)
    }
};
        locals.var_wdld__blk2115 = assign94770_e147417;
        locals.var_wdld__blk2115_dn0 = assign94770_e147417_d_n0;
        locals.var_wdld__blk2115_dn2 = assign94770_e147417_d_n2;
        locals.var_wdld__blk2115_dn4 = assign94770_e147417_d_n4;
        locals.var_wdld__blk2115_dn5 = assign94770_e147417_d_n5;
        locals.var_wdld__blk2115_dn6 = assign94770_e147417_d_n6;
        locals.var_wdld__blk2115_dn7 = assign94770_e147417_d_n7;
        locals.var_wdld__blk2115_dn8 = assign94770_e147417_d_n8;
        locals.var_wdld__blk2115_dn9 = assign94770_e147417_d_n9;
        locals.var_wdld__blk2115_dn10 = assign94770_e147417_d_n10;
        locals.var_wdld__blk2115_dn11 = assign94770_e147417_d_n11;
        locals.var_wdld__blk2115_dn14 = assign94770_e147417_d_n14;
        locals.var_wdld__blk2115_rv = 0.0;

        let (assign94780_e147427, assign94780_e147427_d_n0, assign94780_e147427_d_n2, assign94780_e147427_d_n4, assign94780_e147427_d_n5, assign94780_e147427_d_n6, assign94780_e147427_d_n7, assign94780_e147427_d_n8, assign94780_e147427_d_n9, assign94780_e147427_d_n10, assign94780_e147427_d_n11, assign94780_e147427_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94780_e147425: f64 = (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115);
        (assign94780_e147425, (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn0), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn2), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn4), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn5), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn6), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn7), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn8), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn9), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn10), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn11), (locals.var_q_nsubld__blk2117 * locals.var_wdld__blk2115_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2116, locals.var_q_dep_ld__blk2116_dn0, locals.var_q_dep_ld__blk2116_dn2, locals.var_q_dep_ld__blk2116_dn4, locals.var_q_dep_ld__blk2116_dn5, locals.var_q_dep_ld__blk2116_dn6, locals.var_q_dep_ld__blk2116_dn7, locals.var_q_dep_ld__blk2116_dn8, locals.var_q_dep_ld__blk2116_dn9, locals.var_q_dep_ld__blk2116_dn10, locals.var_q_dep_ld__blk2116_dn11, locals.var_q_dep_ld__blk2116_dn14,)
    }
};
        locals.var_q_dep_ld__blk2116 = assign94780_e147427;
        locals.var_q_dep_ld__blk2116_dn0 = assign94780_e147427_d_n0;
        locals.var_q_dep_ld__blk2116_dn2 = assign94780_e147427_d_n2;
        locals.var_q_dep_ld__blk2116_dn4 = assign94780_e147427_d_n4;
        locals.var_q_dep_ld__blk2116_dn5 = assign94780_e147427_d_n5;
        locals.var_q_dep_ld__blk2116_dn6 = assign94780_e147427_d_n6;
        locals.var_q_dep_ld__blk2116_dn7 = assign94780_e147427_d_n7;
        locals.var_q_dep_ld__blk2116_dn8 = assign94780_e147427_d_n8;
        locals.var_q_dep_ld__blk2116_dn9 = assign94780_e147427_d_n9;
        locals.var_q_dep_ld__blk2116_dn10 = assign94780_e147427_d_n10;
        locals.var_q_dep_ld__blk2116_dn11 = assign94780_e147427_d_n11;
        locals.var_q_dep_ld__blk2116_dn14 = assign94780_e147427_d_n14;
        locals.var_q_dep_ld__blk2116_rv = 0.0;

        let (assign94790_e147441, assign94790_e147441_d_n0, assign94790_e147441_d_n2, assign94790_e147441_d_n4, assign94790_e147441_d_n5, assign94790_e147441_d_n6, assign94790_e147441_d_n7, assign94790_e147441_d_n8, assign94790_e147441_d_n9, assign94790_e147441_d_n10, assign94790_e147441_d_n11, assign94790_e147441_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94790_e147435: f64 = (locals.var_q_dep_ld__blk2116 / locals.var_cnst0over_func);
        let assign94790_e147438: f64 = (10.0 * 2.220446049250313e-16);
        let assign94790_e147439: f64 = (assign94790_e147435 + assign94790_e147438);
        (assign94790_e147439, (((locals.var_q_dep_ld__blk2116_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2116_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2116 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign94790_e147441;
        locals.var_xi0p12_dn0 = assign94790_e147441_d_n0;
        locals.var_xi0p12_dn2 = assign94790_e147441_d_n2;
        locals.var_xi0p12_dn4 = assign94790_e147441_d_n4;
        locals.var_xi0p12_dn5 = assign94790_e147441_d_n5;
        locals.var_xi0p12_dn6 = assign94790_e147441_d_n6;
        locals.var_xi0p12_dn7 = assign94790_e147441_d_n7;
        locals.var_xi0p12_dn8 = assign94790_e147441_d_n8;
        locals.var_xi0p12_dn9 = assign94790_e147441_d_n9;
        locals.var_xi0p12_dn10 = assign94790_e147441_d_n10;
        locals.var_xi0p12_dn11 = assign94790_e147441_d_n11;
        locals.var_xi0p12_dn14 = assign94790_e147441_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign94800_e147451, assign94800_e147451_d_n0, assign94800_e147451_d_n2, assign94800_e147451_d_n4, assign94800_e147451_d_n5, assign94800_e147451_d_n6, assign94800_e147451_d_n7, assign94800_e147451_d_n8, assign94800_e147451_d_n9, assign94800_e147451_d_n10, assign94800_e147451_d_n11, assign94800_e147451_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94800_e147449: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94800_e147449, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign94800_e147451;
        locals.var_qbuld_dn0 = assign94800_e147451_d_n0;
        locals.var_qbuld_dn2 = assign94800_e147451_d_n2;
        locals.var_qbuld_dn4 = assign94800_e147451_d_n4;
        locals.var_qbuld_dn5 = assign94800_e147451_d_n5;
        locals.var_qbuld_dn6 = assign94800_e147451_d_n6;
        locals.var_qbuld_dn7 = assign94800_e147451_d_n7;
        locals.var_qbuld_dn8 = assign94800_e147451_d_n8;
        locals.var_qbuld_dn9 = assign94800_e147451_d_n9;
        locals.var_qbuld_dn10 = assign94800_e147451_d_n10;
        locals.var_qbuld_dn11 = assign94800_e147451_d_n11;
        locals.var_qbuld_dn14 = assign94800_e147451_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign94810_e147463, assign94810_e147463_d_n0, assign94810_e147463_d_n2, assign94810_e147463_d_n4, assign94810_e147463_d_n5, assign94810_e147463_d_n6, assign94810_e147463_d_n7, assign94810_e147463_d_n8, assign94810_e147463_d_n9, assign94810_e147463_d_n10, assign94810_e147463_d_n11, assign94810_e147463_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94810_e147460: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94810_e147461: f64 = (1.0 / assign94810_e147460);
        (assign94810_e147461, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign94810_e147460 * assign94810_e147460))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign94810_e147460 * assign94810_e147460))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94810_e147463;
        locals.var_t1_dn0 = assign94810_e147463_d_n0;
        locals.var_t1_dn2 = assign94810_e147463_d_n2;
        locals.var_t1_dn4 = assign94810_e147463_d_n4;
        locals.var_t1_dn5 = assign94810_e147463_d_n5;
        locals.var_t1_dn6 = assign94810_e147463_d_n6;
        locals.var_t1_dn7 = assign94810_e147463_d_n7;
        locals.var_t1_dn8 = assign94810_e147463_d_n8;
        locals.var_t1_dn9 = assign94810_e147463_d_n9;
        locals.var_t1_dn10 = assign94810_e147463_d_n10;
        locals.var_t1_dn11 = assign94810_e147463_d_n11;
        locals.var_t1_dn14 = assign94810_e147463_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94820_e147475, assign94820_e147475_d_n0, assign94820_e147475_d_n2, assign94820_e147475_d_n4, assign94820_e147475_d_n5, assign94820_e147475_d_n6, assign94820_e147475_d_n7, assign94820_e147475_d_n8, assign94820_e147475_d_n9, assign94820_e147475_d_n10, assign94820_e147475_d_n11, assign94820_e147475_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94820_e147471: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94820_e147473: f64 = (assign94820_e147471 * locals.var_t1);
        (assign94820_e147473, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign94820_e147471 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94820_e147475;
        locals.var_qiuld_dn0 = assign94820_e147475_d_n0;
        locals.var_qiuld_dn2 = assign94820_e147475_d_n2;
        locals.var_qiuld_dn4 = assign94820_e147475_d_n4;
        locals.var_qiuld_dn5 = assign94820_e147475_d_n5;
        locals.var_qiuld_dn6 = assign94820_e147475_d_n6;
        locals.var_qiuld_dn7 = assign94820_e147475_d_n7;
        locals.var_qiuld_dn8 = assign94820_e147475_d_n8;
        locals.var_qiuld_dn9 = assign94820_e147475_d_n9;
        locals.var_qiuld_dn10 = assign94820_e147475_d_n10;
        locals.var_qiuld_dn11 = assign94820_e147475_d_n11;
        locals.var_qiuld_dn14 = assign94820_e147475_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign94830_e147485, assign94830_e147485_d_n0, assign94830_e147485_d_n2, assign94830_e147485_d_n4, assign94830_e147485_d_n5, assign94830_e147485_d_n6, assign94830_e147485_d_n7, assign94830_e147485_d_n8, assign94830_e147485_d_n9, assign94830_e147485_d_n10, assign94830_e147485_d_n11, assign94830_e147485_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2198 != 0.0)) {
        let assign94830_e147483: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94830_e147483, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign94830_e147485;
        locals.var_qsuld_dn0 = assign94830_e147485_d_n0;
        locals.var_qsuld_dn2 = assign94830_e147485_d_n2;
        locals.var_qsuld_dn4 = assign94830_e147485_d_n4;
        locals.var_qsuld_dn5 = assign94830_e147485_d_n5;
        locals.var_qsuld_dn6 = assign94830_e147485_d_n6;
        locals.var_qsuld_dn7 = assign94830_e147485_d_n7;
        locals.var_qsuld_dn8 = assign94830_e147485_d_n8;
        locals.var_qsuld_dn9 = assign94830_e147485_d_n9;
        locals.var_qsuld_dn10 = assign94830_e147485_d_n10;
        locals.var_qsuld_dn11 = assign94830_e147485_d_n11;
        locals.var_qsuld_dn14 = assign94830_e147485_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign94840_e147493, assign94840_e147493_d_n0, assign94840_e147493_d_n2, assign94840_e147493_d_n4, assign94840_e147493_d_n5, assign94840_e147493_d_n6, assign94840_e147493_d_n7, assign94840_e147493_d_n8, assign94840_e147493_d_n9, assign94840_e147493_d_n10, assign94840_e147493_d_n11, assign94840_e147493_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign94840_e147491: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign94840_e147491, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94840_e147493;
        locals.var_qiuld_dn0 = assign94840_e147493_d_n0;
        locals.var_qiuld_dn2 = assign94840_e147493_d_n2;
        locals.var_qiuld_dn4 = assign94840_e147493_d_n4;
        locals.var_qiuld_dn5 = assign94840_e147493_d_n5;
        locals.var_qiuld_dn6 = assign94840_e147493_d_n6;
        locals.var_qiuld_dn7 = assign94840_e147493_d_n7;
        locals.var_qiuld_dn8 = assign94840_e147493_d_n8;
        locals.var_qiuld_dn9 = assign94840_e147493_d_n9;
        locals.var_qiuld_dn10 = assign94840_e147493_d_n10;
        locals.var_qiuld_dn11 = assign94840_e147493_d_n11;
        locals.var_qiuld_dn14 = assign94840_e147493_d_n14;
        locals.var_qiuld_rv = 0.0;

        let assign94850_e147496: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2213 = assign94850_e147496;
        locals.var_guard2213_rv = 0.0;

        let (assign94860_e147505, assign94860_e147505_d_n0, assign94860_e147505_d_n2, assign94860_e147505_d_n4, assign94860_e147505_d_n5, assign94860_e147505_d_n6, assign94860_e147505_d_n7, assign94860_e147505_d_n8, assign94860_e147505_d_n9, assign94860_e147505_d_n10, assign94860_e147505_d_n11, assign94860_e147505_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) {
        let assign94860_e147503: f64 = (-locals.var_lover_func);
        (assign94860_e147503, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign94860_e147505;
        locals.var_lover_func_dn0 = assign94860_e147505_d_n0;
        locals.var_lover_func_dn2 = assign94860_e147505_d_n2;
        locals.var_lover_func_dn4 = assign94860_e147505_d_n4;
        locals.var_lover_func_dn5 = assign94860_e147505_d_n5;
        locals.var_lover_func_dn6 = assign94860_e147505_d_n6;
        locals.var_lover_func_dn7 = assign94860_e147505_d_n7;
        locals.var_lover_func_dn8 = assign94860_e147505_d_n8;
        locals.var_lover_func_dn9 = assign94860_e147505_d_n9;
        locals.var_lover_func_dn10 = assign94860_e147505_d_n10;
        locals.var_lover_func_dn11 = assign94860_e147505_d_n11;
        locals.var_lover_func_dn14 = assign94860_e147505_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign94870_e147508: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2214 = assign94870_e147508;
        locals.var_guard2214_rv = 0.0;

        let assign94880_e147511: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2215 = assign94880_e147511;
        locals.var_guard2215_rv = 0.0;

        let (assign94890_e147524, assign94890_e147524_d_n0, assign94890_e147524_d_n2, assign94890_e147524_d_n4, assign94890_e147524_d_n5, assign94890_e147524_d_n6, assign94890_e147524_d_n7, assign94890_e147524_d_n8, assign94890_e147524_d_n9, assign94890_e147524_d_n10, assign94890_e147524_d_n11, assign94890_e147524_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) && (locals.var_guard2215 != 0.0)) {
        let assign94890_e147522: f64 = (-locals.var_ps0ld);
        (assign94890_e147522, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx__blk2118, locals.var_vx__blk2118_dn0, locals.var_vx__blk2118_dn2, locals.var_vx__blk2118_dn4, locals.var_vx__blk2118_dn5, locals.var_vx__blk2118_dn6, locals.var_vx__blk2118_dn7, locals.var_vx__blk2118_dn8, locals.var_vx__blk2118_dn9, locals.var_vx__blk2118_dn10, locals.var_vx__blk2118_dn11, locals.var_vx__blk2118_dn14,)
    }
};
        locals.var_vx__blk2118 = assign94890_e147524;
        locals.var_vx__blk2118_dn0 = assign94890_e147524_d_n0;
        locals.var_vx__blk2118_dn2 = assign94890_e147524_d_n2;
        locals.var_vx__blk2118_dn4 = assign94890_e147524_d_n4;
        locals.var_vx__blk2118_dn5 = assign94890_e147524_d_n5;
        locals.var_vx__blk2118_dn6 = assign94890_e147524_d_n6;
        locals.var_vx__blk2118_dn7 = assign94890_e147524_d_n7;
        locals.var_vx__blk2118_dn8 = assign94890_e147524_d_n8;
        locals.var_vx__blk2118_dn9 = assign94890_e147524_d_n9;
        locals.var_vx__blk2118_dn10 = assign94890_e147524_d_n10;
        locals.var_vx__blk2118_dn11 = assign94890_e147524_d_n11;
        locals.var_vx__blk2118_dn14 = assign94890_e147524_d_n14;
        locals.var_vx__blk2118_rv = 0.0;

        let (assign94900_e147537, assign94900_e147537_d_n0, assign94900_e147537_d_n2, assign94900_e147537_d_n4, assign94900_e147537_d_n5, assign94900_e147537_d_n6, assign94900_e147537_d_n7, assign94900_e147537_d_n8, assign94900_e147537_d_n9, assign94900_e147537_d_n10, assign94900_e147537_d_n11, assign94900_e147537_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) && (locals.var_guard2215 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx__blk2118, locals.var_vx__blk2118_dn0, locals.var_vx__blk2118_dn2, locals.var_vx__blk2118_dn4, locals.var_vx__blk2118_dn5, locals.var_vx__blk2118_dn6, locals.var_vx__blk2118_dn7, locals.var_vx__blk2118_dn8, locals.var_vx__blk2118_dn9, locals.var_vx__blk2118_dn10, locals.var_vx__blk2118_dn11, locals.var_vx__blk2118_dn14,)
    }
};
        locals.var_vx__blk2118 = assign94900_e147537;
        locals.var_vx__blk2118_dn0 = assign94900_e147537_d_n0;
        locals.var_vx__blk2118_dn2 = assign94900_e147537_d_n2;
        locals.var_vx__blk2118_dn4 = assign94900_e147537_d_n4;
        locals.var_vx__blk2118_dn5 = assign94900_e147537_d_n5;
        locals.var_vx__blk2118_dn6 = assign94900_e147537_d_n6;
        locals.var_vx__blk2118_dn7 = assign94900_e147537_d_n7;
        locals.var_vx__blk2118_dn8 = assign94900_e147537_d_n8;
        locals.var_vx__blk2118_dn9 = assign94900_e147537_d_n9;
        locals.var_vx__blk2118_dn10 = assign94900_e147537_d_n10;
        locals.var_vx__blk2118_dn11 = assign94900_e147537_d_n11;
        locals.var_vx__blk2118_dn14 = assign94900_e147537_d_n14;
        locals.var_vx__blk2118_rv = 0.0;

        let (assign94910_e147560, assign94910_e147560_d_n0, assign94910_e147560_d_n2, assign94910_e147560_d_n4, assign94910_e147560_d_n5, assign94910_e147560_d_n6, assign94910_e147560_d_n7, assign94910_e147560_d_n8, assign94910_e147560_d_n9, assign94910_e147560_d_n10, assign94910_e147560_d_n11, assign94910_e147560_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94910_e147547: f64 = (locals.var_vx__blk2118 + p.p137);
        let assign94910_e147550: f64 = (locals.var_vx__blk2118 + p.p137);
        let assign94910_e147551: f64 = (assign94910_e147547 * assign94910_e147550);
        let assign94910_e147554: f64 = (4.0 * 0.1);
        let assign94910_e147556: f64 = (assign94910_e147554 * 0.1);
        let assign94910_e147557: f64 = (assign94910_e147551 + assign94910_e147556);
        let assign94910_e147558: f64 = (assign94910_e147557).sqrt();
        (assign94910_e147558, (((locals.var_vx__blk2118_dn0 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn0)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn2 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn2)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn4 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn4)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn5 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn5)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn6 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn6)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn7 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn7)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn8 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn8)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn9 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn9)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn10 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn10)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn11 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn11)) / (2.0 * assign94910_e147558)), (((locals.var_vx__blk2118_dn14 * assign94910_e147550) + (assign94910_e147547 * locals.var_vx__blk2118_dn14)) / (2.0 * assign94910_e147558)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94910_e147560;
        locals.var_tmf2_dn0 = assign94910_e147560_d_n0;
        locals.var_tmf2_dn2 = assign94910_e147560_d_n2;
        locals.var_tmf2_dn4 = assign94910_e147560_d_n4;
        locals.var_tmf2_dn5 = assign94910_e147560_d_n5;
        locals.var_tmf2_dn6 = assign94910_e147560_d_n6;
        locals.var_tmf2_dn7 = assign94910_e147560_d_n7;
        locals.var_tmf2_dn8 = assign94910_e147560_d_n8;
        locals.var_tmf2_dn9 = assign94910_e147560_d_n9;
        locals.var_tmf2_dn10 = assign94910_e147560_d_n10;
        locals.var_tmf2_dn11 = assign94910_e147560_d_n11;
        locals.var_tmf2_dn14 = assign94910_e147560_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94920_e147578, assign94920_e147578_d_n0, assign94920_e147578_d_n2, assign94920_e147578_d_n4, assign94920_e147578_d_n5, assign94920_e147578_d_n6, assign94920_e147578_d_n7, assign94920_e147578_d_n8, assign94920_e147578_d_n9, assign94920_e147578_d_n10, assign94920_e147578_d_n11, assign94920_e147578_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94920_e147572: f64 = (locals.var_vx__blk2118 + p.p137);
        let assign94920_e147574: f64 = (assign94920_e147572 / locals.var_tmf2);
        let assign94920_e147575: f64 = (1.0 + assign94920_e147574);
        let assign94920_e147576: f64 = (0.5 * assign94920_e147575);
        (assign94920_e147576, (0.5 * (((locals.var_vx__blk2118_dn0 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn2 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn4 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn5 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn6 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn7 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn8 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn9 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn10 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn11 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2118_dn14 * locals.var_tmf2) - (assign94920_e147572 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94920_e147578;
        locals.var_t9_dn0 = assign94920_e147578_d_n0;
        locals.var_t9_dn2 = assign94920_e147578_d_n2;
        locals.var_t9_dn4 = assign94920_e147578_d_n4;
        locals.var_t9_dn5 = assign94920_e147578_d_n5;
        locals.var_t9_dn6 = assign94920_e147578_d_n6;
        locals.var_t9_dn7 = assign94920_e147578_d_n7;
        locals.var_t9_dn8 = assign94920_e147578_d_n8;
        locals.var_t9_dn9 = assign94920_e147578_d_n9;
        locals.var_t9_dn10 = assign94920_e147578_d_n10;
        locals.var_t9_dn11 = assign94920_e147578_d_n11;
        locals.var_t9_dn14 = assign94920_e147578_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94930_e147594, assign94930_e147594_d_n0, assign94930_e147594_d_n2, assign94930_e147594_d_n4, assign94930_e147594_d_n5, assign94930_e147594_d_n6, assign94930_e147594_d_n7, assign94930_e147594_d_n8, assign94930_e147594_d_n9, assign94930_e147594_d_n10, assign94930_e147594_d_n11, assign94930_e147594_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94930_e147589: f64 = (locals.var_vx__blk2118 + p.p137);
        let assign94930_e147591: f64 = (assign94930_e147589 + locals.var_tmf2);
        let assign94930_e147592: f64 = (0.5 * assign94930_e147591);
        (assign94930_e147592, (0.5 * (locals.var_vx__blk2118_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2118_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2118_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2118_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2118_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2118_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2118_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2118_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2118_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2118_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx__blk2118_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94930_e147594;
        locals.var_t2_dn0 = assign94930_e147594_d_n0;
        locals.var_t2_dn2 = assign94930_e147594_d_n2;
        locals.var_t2_dn4 = assign94930_e147594_d_n4;
        locals.var_t2_dn5 = assign94930_e147594_d_n5;
        locals.var_t2_dn6 = assign94930_e147594_d_n6;
        locals.var_t2_dn7 = assign94930_e147594_d_n7;
        locals.var_t2_dn8 = assign94930_e147594_d_n8;
        locals.var_t2_dn9 = assign94930_e147594_d_n9;
        locals.var_t2_dn10 = assign94930_e147594_d_n10;
        locals.var_t2_dn11 = assign94930_e147594_d_n11;
        locals.var_t2_dn14 = assign94930_e147594_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94940_e147597: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2216 = assign94940_e147597;
        locals.var_guard2216_rv = 0.0;

        let (assign94950_e147609, assign94950_e147609_d_n0, assign94950_e147609_d_n2, assign94950_e147609_d_n4, assign94950_e147609_d_n5, assign94950_e147609_d_n6, assign94950_e147609_d_n7, assign94950_e147609_d_n8, assign94950_e147609_d_n9, assign94950_e147609_d_n10, assign94950_e147609_d_n11, assign94950_e147609_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94950_e147609;
        locals.var_t2_dn0 = assign94950_e147609_d_n0;
        locals.var_t2_dn2 = assign94950_e147609_d_n2;
        locals.var_t2_dn4 = assign94950_e147609_d_n4;
        locals.var_t2_dn5 = assign94950_e147609_d_n5;
        locals.var_t2_dn6 = assign94950_e147609_d_n6;
        locals.var_t2_dn7 = assign94950_e147609_d_n7;
        locals.var_t2_dn8 = assign94950_e147609_d_n8;
        locals.var_t2_dn9 = assign94950_e147609_d_n9;
        locals.var_t2_dn10 = assign94950_e147609_d_n10;
        locals.var_t2_dn11 = assign94950_e147609_d_n11;
        locals.var_t2_dn14 = assign94950_e147609_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94960_e147621, assign94960_e147621_d_n0, assign94960_e147621_d_n2, assign94960_e147621_d_n4, assign94960_e147621_d_n5, assign94960_e147621_d_n6, assign94960_e147621_d_n7, assign94960_e147621_d_n8, assign94960_e147621_d_n9, assign94960_e147621_d_n10, assign94960_e147621_d_n11, assign94960_e147621_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94960_e147621;
        locals.var_t9_dn0 = assign94960_e147621_d_n0;
        locals.var_t9_dn2 = assign94960_e147621_d_n2;
        locals.var_t9_dn4 = assign94960_e147621_d_n4;
        locals.var_t9_dn5 = assign94960_e147621_d_n5;
        locals.var_t9_dn6 = assign94960_e147621_d_n6;
        locals.var_t9_dn7 = assign94960_e147621_d_n7;
        locals.var_t9_dn8 = assign94960_e147621_d_n8;
        locals.var_t9_dn9 = assign94960_e147621_d_n9;
        locals.var_t9_dn10 = assign94960_e147621_d_n10;
        locals.var_t9_dn11 = assign94960_e147621_d_n11;
        locals.var_t9_dn14 = assign94960_e147621_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94970_e147636, assign94970_e147636_d_n0, assign94970_e147636_d_n2, assign94970_e147636_d_n4, assign94970_e147636_d_n5, assign94970_e147636_d_n6, assign94970_e147636_d_n7, assign94970_e147636_d_n8, assign94970_e147636_d_n9, assign94970_e147636_d_n10, assign94970_e147636_d_n11, assign94970_e147636_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94970_e147631: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94970_e147632: f64 = (assign94970_e147631).sqrt();
        let assign94970_e147634: f64 = (assign94970_e147632 * p.p432);
        (assign94970_e147634, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94970_e147632)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94970_e147632)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94970_e147636;
        locals.var_wjunc0_dn0 = assign94970_e147636_d_n0;
        locals.var_wjunc0_dn2 = assign94970_e147636_d_n2;
        locals.var_wjunc0_dn4 = assign94970_e147636_d_n4;
        locals.var_wjunc0_dn5 = assign94970_e147636_d_n5;
        locals.var_wjunc0_dn6 = assign94970_e147636_d_n6;
        locals.var_wjunc0_dn7 = assign94970_e147636_d_n7;
        locals.var_wjunc0_dn8 = assign94970_e147636_d_n8;
        locals.var_wjunc0_dn9 = assign94970_e147636_d_n9;
        locals.var_wjunc0_dn10 = assign94970_e147636_d_n10;
        locals.var_wjunc0_dn11 = assign94970_e147636_d_n11;
        locals.var_wjunc0_dn14 = assign94970_e147636_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign94980_e147652, assign94980_e147652_d_n0, assign94980_e147652_d_n2, assign94980_e147652_d_n4, assign94980_e147652_d_n5, assign94980_e147652_d_n6, assign94980_e147652_d_n7, assign94980_e147652_d_n8, assign94980_e147652_d_n9, assign94980_e147652_d_n10, assign94980_e147652_d_n11, assign94980_e147652_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94980_e147646: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign94980_e147649: f64 = (0.1 * locals.var_lover_func);
        let assign94980_e147650: f64 = (assign94980_e147646 - assign94980_e147649);
        (assign94980_e147650, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign94980_e147652;
        locals.var_tmf1_dn0 = assign94980_e147652_d_n0;
        locals.var_tmf1_dn2 = assign94980_e147652_d_n2;
        locals.var_tmf1_dn4 = assign94980_e147652_d_n4;
        locals.var_tmf1_dn5 = assign94980_e147652_d_n5;
        locals.var_tmf1_dn6 = assign94980_e147652_d_n6;
        locals.var_tmf1_dn7 = assign94980_e147652_d_n7;
        locals.var_tmf1_dn8 = assign94980_e147652_d_n8;
        locals.var_tmf1_dn9 = assign94980_e147652_d_n9;
        locals.var_tmf1_dn10 = assign94980_e147652_d_n10;
        locals.var_tmf1_dn11 = assign94980_e147652_d_n11;
        locals.var_tmf1_dn14 = assign94980_e147652_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign94990_e147668, assign94990_e147668_d_n0, assign94990_e147668_d_n2, assign94990_e147668_d_n4, assign94990_e147668_d_n5, assign94990_e147668_d_n6, assign94990_e147668_d_n7, assign94990_e147668_d_n8, assign94990_e147668_d_n9, assign94990_e147668_d_n10, assign94990_e147668_d_n11, assign94990_e147668_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign94990_e147662: f64 = (4.0 * locals.var_lover_func);
        let assign94990_e147665: f64 = (0.1 * locals.var_lover_func);
        let assign94990_e147666: f64 = (assign94990_e147662 * assign94990_e147665);
        (assign94990_e147666, (((4.0 * locals.var_lover_func_dn0) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign94990_e147665) + (assign94990_e147662 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94990_e147668;
        locals.var_tmf2_dn0 = assign94990_e147668_d_n0;
        locals.var_tmf2_dn2 = assign94990_e147668_d_n2;
        locals.var_tmf2_dn4 = assign94990_e147668_d_n4;
        locals.var_tmf2_dn5 = assign94990_e147668_d_n5;
        locals.var_tmf2_dn6 = assign94990_e147668_d_n6;
        locals.var_tmf2_dn7 = assign94990_e147668_d_n7;
        locals.var_tmf2_dn8 = assign94990_e147668_d_n8;
        locals.var_tmf2_dn9 = assign94990_e147668_d_n9;
        locals.var_tmf2_dn10 = assign94990_e147668_d_n10;
        locals.var_tmf2_dn11 = assign94990_e147668_d_n11;
        locals.var_tmf2_dn14 = assign94990_e147668_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_369(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95000_e147684, assign95000_e147684_d_n0, assign95000_e147684_d_n2, assign95000_e147684_d_n4, assign95000_e147684_d_n5, assign95000_e147684_d_n6, assign95000_e147684_d_n7, assign95000_e147684_d_n8, assign95000_e147684_d_n9, assign95000_e147684_d_n10, assign95000_e147684_d_n11, assign95000_e147684_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let (assign95000_e147682, assign95000_e147682_d_n0, assign95000_e147682_d_n2, assign95000_e147682_d_n4, assign95000_e147682_d_n5, assign95000_e147682_d_n6, assign95000_e147682_d_n7, assign95000_e147682_d_n8, assign95000_e147682_d_n9, assign95000_e147682_d_n10, assign95000_e147682_d_n11, assign95000_e147682_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign95000_e147681: f64 = (-locals.var_tmf2);
                (assign95000_e147681, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign95000_e147682, assign95000_e147682_d_n0, assign95000_e147682_d_n2, assign95000_e147682_d_n4, assign95000_e147682_d_n5, assign95000_e147682_d_n6, assign95000_e147682_d_n7, assign95000_e147682_d_n8, assign95000_e147682_d_n9, assign95000_e147682_d_n10, assign95000_e147682_d_n11, assign95000_e147682_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign95000_e147684;
        locals.var_tmf2_dn0 = assign95000_e147684_d_n0;
        locals.var_tmf2_dn2 = assign95000_e147684_d_n2;
        locals.var_tmf2_dn4 = assign95000_e147684_d_n4;
        locals.var_tmf2_dn5 = assign95000_e147684_d_n5;
        locals.var_tmf2_dn6 = assign95000_e147684_d_n6;
        locals.var_tmf2_dn7 = assign95000_e147684_d_n7;
        locals.var_tmf2_dn8 = assign95000_e147684_d_n8;
        locals.var_tmf2_dn9 = assign95000_e147684_d_n9;
        locals.var_tmf2_dn10 = assign95000_e147684_d_n10;
        locals.var_tmf2_dn11 = assign95000_e147684_d_n11;
        locals.var_tmf2_dn14 = assign95000_e147684_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign95010_e147699, assign95010_e147699_d_n0, assign95010_e147699_d_n2, assign95010_e147699_d_n4, assign95010_e147699_d_n5, assign95010_e147699_d_n6, assign95010_e147699_d_n7, assign95010_e147699_d_n8, assign95010_e147699_d_n9, assign95010_e147699_d_n10, assign95010_e147699_d_n11, assign95010_e147699_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign95010_e147694: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign95010_e147696: f64 = (assign95010_e147694 + locals.var_tmf2);
        let assign95010_e147697: f64 = (assign95010_e147696).sqrt();
        (assign95010_e147697, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign95010_e147697)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign95010_e147697)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign95010_e147699;
        locals.var_tmf2_dn0 = assign95010_e147699_d_n0;
        locals.var_tmf2_dn2 = assign95010_e147699_d_n2;
        locals.var_tmf2_dn4 = assign95010_e147699_d_n4;
        locals.var_tmf2_dn5 = assign95010_e147699_d_n5;
        locals.var_tmf2_dn6 = assign95010_e147699_d_n6;
        locals.var_tmf2_dn7 = assign95010_e147699_d_n7;
        locals.var_tmf2_dn8 = assign95010_e147699_d_n8;
        locals.var_tmf2_dn9 = assign95010_e147699_d_n9;
        locals.var_tmf2_dn10 = assign95010_e147699_d_n10;
        locals.var_tmf2_dn11 = assign95010_e147699_d_n11;
        locals.var_tmf2_dn14 = assign95010_e147699_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign95020_e147715, assign95020_e147715_d_n0, assign95020_e147715_d_n2, assign95020_e147715_d_n4, assign95020_e147715_d_n5, assign95020_e147715_d_n6, assign95020_e147715_d_n7, assign95020_e147715_d_n8, assign95020_e147715_d_n9, assign95020_e147715_d_n10, assign95020_e147715_d_n11, assign95020_e147715_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign95020_e147711: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign95020_e147712: f64 = (1.0 + assign95020_e147711);
        let assign95020_e147713: f64 = (0.5 * assign95020_e147712);
        (assign95020_e147713, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95020_e147715;
        locals.var_t0_dn0 = assign95020_e147715_d_n0;
        locals.var_t0_dn2 = assign95020_e147715_d_n2;
        locals.var_t0_dn4 = assign95020_e147715_d_n4;
        locals.var_t0_dn5 = assign95020_e147715_d_n5;
        locals.var_t0_dn6 = assign95020_e147715_d_n6;
        locals.var_t0_dn7 = assign95020_e147715_d_n7;
        locals.var_t0_dn8 = assign95020_e147715_d_n8;
        locals.var_t0_dn9 = assign95020_e147715_d_n9;
        locals.var_t0_dn10 = assign95020_e147715_d_n10;
        locals.var_t0_dn11 = assign95020_e147715_d_n11;
        locals.var_t0_dn14 = assign95020_e147715_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95030_e147731, assign95030_e147731_d_n0, assign95030_e147731_d_n2, assign95030_e147731_d_n4, assign95030_e147731_d_n5, assign95030_e147731_d_n6, assign95030_e147731_d_n7, assign95030_e147731_d_n8, assign95030_e147731_d_n9, assign95030_e147731_d_n10, assign95030_e147731_d_n11, assign95030_e147731_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign95030_e147727: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign95030_e147728: f64 = (0.5 * assign95030_e147727);
        let assign95030_e147729: f64 = (locals.var_lover_func - assign95030_e147728);
        (assign95030_e147729, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign95030_e147731;
        locals.var_wjuncld_dn0 = assign95030_e147731_d_n0;
        locals.var_wjuncld_dn2 = assign95030_e147731_d_n2;
        locals.var_wjuncld_dn4 = assign95030_e147731_d_n4;
        locals.var_wjuncld_dn5 = assign95030_e147731_d_n5;
        locals.var_wjuncld_dn6 = assign95030_e147731_d_n6;
        locals.var_wjuncld_dn7 = assign95030_e147731_d_n7;
        locals.var_wjuncld_dn8 = assign95030_e147731_d_n8;
        locals.var_wjuncld_dn9 = assign95030_e147731_d_n9;
        locals.var_wjuncld_dn10 = assign95030_e147731_d_n10;
        locals.var_wjuncld_dn11 = assign95030_e147731_d_n11;
        locals.var_wjuncld_dn14 = assign95030_e147731_d_n14;
        locals.var_wjuncld_rv = 0.0;

        let (assign95040_e147743, assign95040_e147743_d_n0, assign95040_e147743_d_n2, assign95040_e147743_d_n4, assign95040_e147743_d_n5, assign95040_e147743_d_n6, assign95040_e147743_d_n7, assign95040_e147743_d_n8, assign95040_e147743_d_n9, assign95040_e147743_d_n10, assign95040_e147743_d_n11, assign95040_e147743_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2213 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        let assign95040_e147741: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign95040_e147741, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign95040_e147743;
        locals.var_lover_func_dn0 = assign95040_e147743_d_n0;
        locals.var_lover_func_dn2 = assign95040_e147743_d_n2;
        locals.var_lover_func_dn4 = assign95040_e147743_d_n4;
        locals.var_lover_func_dn5 = assign95040_e147743_d_n5;
        locals.var_lover_func_dn6 = assign95040_e147743_d_n6;
        locals.var_lover_func_dn7 = assign95040_e147743_d_n7;
        locals.var_lover_func_dn8 = assign95040_e147743_d_n8;
        locals.var_lover_func_dn9 = assign95040_e147743_d_n9;
        locals.var_lover_func_dn10 = assign95040_e147743_d_n10;
        locals.var_lover_func_dn11 = assign95040_e147743_d_n11;
        locals.var_lover_func_dn14 = assign95040_e147743_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign95050_e147749, assign95050_e147749_d_n0, assign95050_e147749_d_n2, assign95050_e147749_d_n4, assign95050_e147749_d_n5, assign95050_e147749_d_n6, assign95050_e147749_d_n7, assign95050_e147749_d_n8, assign95050_e147749_d_n9, assign95050_e147749_d_n10, assign95050_e147749_d_n11, assign95050_e147749_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign95050_e147749;
        locals.var_rd_qbuld_dn0 = assign95050_e147749_d_n0;
        locals.var_rd_qbuld_dn2 = assign95050_e147749_d_n2;
        locals.var_rd_qbuld_dn4 = assign95050_e147749_d_n4;
        locals.var_rd_qbuld_dn5 = assign95050_e147749_d_n5;
        locals.var_rd_qbuld_dn6 = assign95050_e147749_d_n6;
        locals.var_rd_qbuld_dn7 = assign95050_e147749_d_n7;
        locals.var_rd_qbuld_dn8 = assign95050_e147749_d_n8;
        locals.var_rd_qbuld_dn9 = assign95050_e147749_d_n9;
        locals.var_rd_qbuld_dn10 = assign95050_e147749_d_n10;
        locals.var_rd_qbuld_dn11 = assign95050_e147749_d_n11;
        locals.var_rd_qbuld_dn14 = assign95050_e147749_d_n14;
        locals.var_rd_qbuld_rv = 0.0;

        let assign95060_e147760: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2217 = assign95060_e147760;
        locals.var_guard2217_rv = 0.0;

        let (assign95070_e147764,) = {
    if (locals.var_guard2217 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign95070_e147764;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign95080_e147768,) = {
    if (locals.var_guard2217 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95080_e147768;
        locals.var_cov_slp_rv = 0.0;

        let (assign95090_e147772,) = {
    if (locals.var_guard2217 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95090_e147772;
        locals.var_cov_mag_rv = 0.0;

        let (assign95100_e147778, assign95100_e147778_d_n0, assign95100_e147778_d_n2, assign95100_e147778_d_n4, assign95100_e147778_d_n5, assign95100_e147778_d_n6, assign95100_e147778_d_n7, assign95100_e147778_d_n8, assign95100_e147778_d_n9, assign95100_e147778_d_n10, assign95100_e147778_d_n11, assign95100_e147778_d_n14,) = {
    if (locals.var_guard2217 != 0.0) {
        let assign95100_e147776: f64 = (locals.var_cox0 * locals.var_weffcv_nf);
        (assign95100_e147776, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95100_e147778;
        locals.var_t1_dn0 = assign95100_e147778_d_n0;
        locals.var_t1_dn2 = assign95100_e147778_d_n2;
        locals.var_t1_dn4 = assign95100_e147778_d_n4;
        locals.var_t1_dn5 = assign95100_e147778_d_n5;
        locals.var_t1_dn6 = assign95100_e147778_d_n6;
        locals.var_t1_dn7 = assign95100_e147778_d_n7;
        locals.var_t1_dn8 = assign95100_e147778_d_n8;
        locals.var_t1_dn9 = assign95100_e147778_d_n9;
        locals.var_t1_dn10 = assign95100_e147778_d_n10;
        locals.var_t1_dn11 = assign95100_e147778_d_n11;
        locals.var_t1_dn14 = assign95100_e147778_d_n14;
        locals.var_t1_rv = 0.0;

        let assign95110_e147781: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2218 = assign95110_e147781;
        locals.var_guard2218_rv = 0.0;

        let (assign95120_e147793, assign95120_e147793_d_n0, assign95120_e147793_d_n2, assign95120_e147793_d_n4, assign95120_e147793_d_n5, assign95120_e147793_d_n6, assign95120_e147793_d_n7, assign95120_e147793_d_n8, assign95120_e147793_d_n9, assign95120_e147793_d_n10, assign95120_e147793_d_n11, assign95120_e147793_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95120_e147787: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95120_e147790: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95120_e147791: f64 = (assign95120_e147787 * assign95120_e147790);
        (assign95120_e147791, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn5) * assign95120_e147790), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95120_e147790) + (assign95120_e147787 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95120_e147790) + (assign95120_e147787 * locals.var_vgs_dn7)), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95120_e147790) + (assign95120_e147787 * locals.var_vgs_dn8)), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn11) * assign95120_e147790), ((locals.var_cov_slp * locals.var_t1_dn14) * assign95120_e147790),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95120_e147793;
        locals.var_t4_dn0 = assign95120_e147793_d_n0;
        locals.var_t4_dn2 = assign95120_e147793_d_n2;
        locals.var_t4_dn4 = assign95120_e147793_d_n4;
        locals.var_t4_dn5 = assign95120_e147793_d_n5;
        locals.var_t4_dn6 = assign95120_e147793_d_n6;
        locals.var_t4_dn7 = assign95120_e147793_d_n7;
        locals.var_t4_dn8 = assign95120_e147793_d_n8;
        locals.var_t4_dn9 = assign95120_e147793_d_n9;
        locals.var_t4_dn10 = assign95120_e147793_d_n10;
        locals.var_t4_dn11 = assign95120_e147793_d_n11;
        locals.var_t4_dn14 = assign95120_e147793_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95130_e147801, assign95130_e147801_d_n0, assign95130_e147801_d_n2, assign95130_e147801_d_n4, assign95130_e147801_d_n5, assign95130_e147801_d_n6, assign95130_e147801_d_n7, assign95130_e147801_d_n8, assign95130_e147801_d_n9, assign95130_e147801_d_n10, assign95130_e147801_d_n11, assign95130_e147801_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95130_e147799: f64 = (p.p66 * locals.var_t1);
        (assign95130_e147799, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn11), (p.p66 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95130_e147801;
        locals.var_t5_dn0 = assign95130_e147801_d_n0;
        locals.var_t5_dn2 = assign95130_e147801_d_n2;
        locals.var_t5_dn4 = assign95130_e147801_d_n4;
        locals.var_t5_dn5 = assign95130_e147801_d_n5;
        locals.var_t5_dn6 = assign95130_e147801_d_n6;
        locals.var_t5_dn7 = assign95130_e147801_d_n7;
        locals.var_t5_dn8 = assign95130_e147801_d_n8;
        locals.var_t5_dn9 = assign95130_e147801_d_n9;
        locals.var_t5_dn10 = assign95130_e147801_d_n10;
        locals.var_t5_dn11 = assign95130_e147801_d_n11;
        locals.var_t5_dn14 = assign95130_e147801_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95140_e147809, assign95140_e147809_d_n0, assign95140_e147809_d_n2, assign95140_e147809_d_n4, assign95140_e147809_d_n5, assign95140_e147809_d_n6, assign95140_e147809_d_n7, assign95140_e147809_d_n8, assign95140_e147809_d_n9, assign95140_e147809_d_n10, assign95140_e147809_d_n11, assign95140_e147809_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95140_e147807: f64 = (1.2 - locals.var_ps0);
        (assign95140_e147807, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95140_e147809;
        locals.var_t9_dn0 = assign95140_e147809_d_n0;
        locals.var_t9_dn2 = assign95140_e147809_d_n2;
        locals.var_t9_dn4 = assign95140_e147809_d_n4;
        locals.var_t9_dn5 = assign95140_e147809_d_n5;
        locals.var_t9_dn6 = assign95140_e147809_d_n6;
        locals.var_t9_dn7 = assign95140_e147809_d_n7;
        locals.var_t9_dn8 = assign95140_e147809_d_n8;
        locals.var_t9_dn9 = assign95140_e147809_d_n9;
        locals.var_t9_dn10 = assign95140_e147809_d_n10;
        locals.var_t9_dn11 = assign95140_e147809_d_n11;
        locals.var_t9_dn14 = assign95140_e147809_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95150_e147821, assign95150_e147821_d_n0, assign95150_e147821_d_n2, assign95150_e147821_d_n4, assign95150_e147821_d_n5, assign95150_e147821_d_n6, assign95150_e147821_d_n7, assign95150_e147821_d_n8, assign95150_e147821_d_n9, assign95150_e147821_d_n10, assign95150_e147821_d_n11, assign95150_e147821_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95150_e147815: f64 = (locals.var_vgs * locals.var_t5);
        let assign95150_e147818: f64 = (locals.var_t4 * locals.var_t9);
        let assign95150_e147819: f64 = (assign95150_e147815 - assign95150_e147818);
        (assign95150_e147819, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn14,)
    }
};
        locals.var_qgos = assign95150_e147821;
        locals.var_qgos_dn0 = assign95150_e147821_d_n0;
        locals.var_qgos_dn2 = assign95150_e147821_d_n2;
        locals.var_qgos_dn4 = assign95150_e147821_d_n4;
        locals.var_qgos_dn5 = assign95150_e147821_d_n5;
        locals.var_qgos_dn6 = assign95150_e147821_d_n6;
        locals.var_qgos_dn7 = assign95150_e147821_d_n7;
        locals.var_qgos_dn8 = assign95150_e147821_d_n8;
        locals.var_qgos_dn9 = assign95150_e147821_d_n9;
        locals.var_qgos_dn10 = assign95150_e147821_d_n10;
        locals.var_qgos_dn11 = assign95150_e147821_d_n11;
        locals.var_qgos_dn14 = assign95150_e147821_d_n14;
        locals.var_qgos_rv = 0.0;

        let (assign95160_e147836, assign95160_e147836_d_n0, assign95160_e147836_d_n2, assign95160_e147836_d_n4, assign95160_e147836_d_n5, assign95160_e147836_d_n6, assign95160_e147836_d_n7, assign95160_e147836_d_n8, assign95160_e147836_d_n9, assign95160_e147836_d_n10, assign95160_e147836_d_n11, assign95160_e147836_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95160_e147828: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95160_e147831: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95160_e147833: f64 = (assign95160_e147831 - locals.var_vds);
        let assign95160_e147834: f64 = (assign95160_e147828 * assign95160_e147833);
        (assign95160_e147834, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95160_e147833) + (assign95160_e147828 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95160_e147833) + (assign95160_e147828 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95160_e147833) + (assign95160_e147828 * (locals.var_vgs_dn8 - locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn11) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1_dn14) * assign95160_e147833) + (assign95160_e147828 * (-locals.var_vds_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95160_e147836;
        locals.var_t4_dn0 = assign95160_e147836_d_n0;
        locals.var_t4_dn2 = assign95160_e147836_d_n2;
        locals.var_t4_dn4 = assign95160_e147836_d_n4;
        locals.var_t4_dn5 = assign95160_e147836_d_n5;
        locals.var_t4_dn6 = assign95160_e147836_d_n6;
        locals.var_t4_dn7 = assign95160_e147836_d_n7;
        locals.var_t4_dn8 = assign95160_e147836_d_n8;
        locals.var_t4_dn9 = assign95160_e147836_d_n9;
        locals.var_t4_dn10 = assign95160_e147836_d_n10;
        locals.var_t4_dn11 = assign95160_e147836_d_n11;
        locals.var_t4_dn14 = assign95160_e147836_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95170_e147845, assign95170_e147845_d_n0, assign95170_e147845_d_n2, assign95170_e147845_d_n4, assign95170_e147845_d_n5, assign95170_e147845_d_n6, assign95170_e147845_d_n7, assign95170_e147845_d_n8, assign95170_e147845_d_n9, assign95170_e147845_d_n10, assign95170_e147845_d_n11, assign95170_e147845_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95170_e147843: f64 = (p.p66 * locals.var_t1);
        (assign95170_e147843, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn11), (p.p66 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95170_e147845;
        locals.var_t5_dn0 = assign95170_e147845_d_n0;
        locals.var_t5_dn2 = assign95170_e147845_d_n2;
        locals.var_t5_dn4 = assign95170_e147845_d_n4;
        locals.var_t5_dn5 = assign95170_e147845_d_n5;
        locals.var_t5_dn6 = assign95170_e147845_d_n6;
        locals.var_t5_dn7 = assign95170_e147845_d_n7;
        locals.var_t5_dn8 = assign95170_e147845_d_n8;
        locals.var_t5_dn9 = assign95170_e147845_d_n9;
        locals.var_t5_dn10 = assign95170_e147845_d_n10;
        locals.var_t5_dn11 = assign95170_e147845_d_n11;
        locals.var_t5_dn14 = assign95170_e147845_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95180_e147856, assign95180_e147856_d_n0, assign95180_e147856_d_n2, assign95180_e147856_d_n4, assign95180_e147856_d_n5, assign95180_e147856_d_n6, assign95180_e147856_d_n7, assign95180_e147856_d_n8, assign95180_e147856_d_n9, assign95180_e147856_d_n10, assign95180_e147856_d_n11, assign95180_e147856_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95180_e147852: f64 = (1.2 + locals.var_vds);
        let assign95180_e147854: f64 = (assign95180_e147852 - locals.var_psl);
        (assign95180_e147854, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn11 - locals.var_psl_dn11), (locals.var_vds_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95180_e147856;
        locals.var_t9_dn0 = assign95180_e147856_d_n0;
        locals.var_t9_dn2 = assign95180_e147856_d_n2;
        locals.var_t9_dn4 = assign95180_e147856_d_n4;
        locals.var_t9_dn5 = assign95180_e147856_d_n5;
        locals.var_t9_dn6 = assign95180_e147856_d_n6;
        locals.var_t9_dn7 = assign95180_e147856_d_n7;
        locals.var_t9_dn8 = assign95180_e147856_d_n8;
        locals.var_t9_dn9 = assign95180_e147856_d_n9;
        locals.var_t9_dn10 = assign95180_e147856_d_n10;
        locals.var_t9_dn11 = assign95180_e147856_d_n11;
        locals.var_t9_dn14 = assign95180_e147856_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95190_e147871, assign95190_e147871_d_n0, assign95190_e147871_d_n2, assign95190_e147871_d_n4, assign95190_e147871_d_n5, assign95190_e147871_d_n6, assign95190_e147871_d_n7, assign95190_e147871_d_n8, assign95190_e147871_d_n9, assign95190_e147871_d_n10, assign95190_e147871_d_n11, assign95190_e147871_d_n14,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95190_e147863: f64 = (locals.var_vgs - locals.var_vds);
        let assign95190_e147865: f64 = (assign95190_e147863 * locals.var_t5);
        let assign95190_e147868: f64 = (locals.var_t4 * locals.var_t9);
        let assign95190_e147869: f64 = (assign95190_e147865 - assign95190_e147868);
        (assign95190_e147869, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((-locals.var_vds_dn5) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((locals.var_vgs_dn8 - locals.var_vds_dn8) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn11) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn11)) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((((-locals.var_vds_dn14) * locals.var_t5) + (assign95190_e147863 * locals.var_t5_dn14)) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn14,)
    }
};
        locals.var_qgos = assign95190_e147871;
        locals.var_qgos_dn0 = assign95190_e147871_d_n0;
        locals.var_qgos_dn2 = assign95190_e147871_d_n2;
        locals.var_qgos_dn4 = assign95190_e147871_d_n4;
        locals.var_qgos_dn5 = assign95190_e147871_d_n5;
        locals.var_qgos_dn6 = assign95190_e147871_d_n6;
        locals.var_qgos_dn7 = assign95190_e147871_d_n7;
        locals.var_qgos_dn8 = assign95190_e147871_d_n8;
        locals.var_qgos_dn9 = assign95190_e147871_d_n9;
        locals.var_qgos_dn10 = assign95190_e147871_d_n10;
        locals.var_qgos_dn11 = assign95190_e147871_d_n11;
        locals.var_qgos_dn14 = assign95190_e147871_d_n14;
        locals.var_qgos_rv = 0.0;

        let assign95200_e147882: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2219 = assign95200_e147882;
        locals.var_guard2219_rv = 0.0;

        let (assign95210_e147886,) = {
    if (locals.var_guard2219 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign95210_e147886;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign95220_e147890,) = {
    if (locals.var_guard2219 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95220_e147890;
        locals.var_cov_slp_rv = 0.0;

        let (assign95230_e147894,) = {
    if (locals.var_guard2219 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95230_e147894;
        locals.var_cov_mag_rv = 0.0;

        let (assign95240_e147900, assign95240_e147900_d_n0, assign95240_e147900_d_n2, assign95240_e147900_d_n4, assign95240_e147900_d_n5, assign95240_e147900_d_n6, assign95240_e147900_d_n7, assign95240_e147900_d_n8, assign95240_e147900_d_n9, assign95240_e147900_d_n10, assign95240_e147900_d_n11, assign95240_e147900_d_n14,) = {
    if (locals.var_guard2219 != 0.0) {
        let assign95240_e147898: f64 = (locals.var_coxb0 * locals.var_weffcv_nf);
        (assign95240_e147898, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95240_e147900;
        locals.var_t1_dn0 = assign95240_e147900_d_n0;
        locals.var_t1_dn2 = assign95240_e147900_d_n2;
        locals.var_t1_dn4 = assign95240_e147900_d_n4;
        locals.var_t1_dn5 = assign95240_e147900_d_n5;
        locals.var_t1_dn6 = assign95240_e147900_d_n6;
        locals.var_t1_dn7 = assign95240_e147900_d_n7;
        locals.var_t1_dn8 = assign95240_e147900_d_n8;
        locals.var_t1_dn9 = assign95240_e147900_d_n9;
        locals.var_t1_dn10 = assign95240_e147900_d_n10;
        locals.var_t1_dn11 = assign95240_e147900_d_n11;
        locals.var_t1_dn14 = assign95240_e147900_d_n14;
        locals.var_t1_rv = 0.0;

        let assign95250_e147903: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2220 = assign95250_e147903;
        locals.var_guard2220_rv = 0.0;

        let (assign95260_e147917, assign95260_e147917_d_n0, assign95260_e147917_d_n2, assign95260_e147917_d_n4, assign95260_e147917_d_n5, assign95260_e147917_d_n6, assign95260_e147917_d_n7, assign95260_e147917_d_n8, assign95260_e147917_d_n9, assign95260_e147917_d_n10, assign95260_e147917_d_n11, assign95260_e147917_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95260_e147909: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95260_e147912: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95260_e147914: f64 = (assign95260_e147912 - locals.var_vds);
        let assign95260_e147915: f64 = (assign95260_e147909 * assign95260_e147914);
        (assign95260_e147915, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95260_e147914) + (assign95260_e147909 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95260_e147914) + (assign95260_e147909 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95260_e147914) + (assign95260_e147909 * (locals.var_vgs_dn8 - locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn11) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1_dn14) * assign95260_e147914) + (assign95260_e147909 * (-locals.var_vds_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95260_e147917;
        locals.var_t4_dn0 = assign95260_e147917_d_n0;
        locals.var_t4_dn2 = assign95260_e147917_d_n2;
        locals.var_t4_dn4 = assign95260_e147917_d_n4;
        locals.var_t4_dn5 = assign95260_e147917_d_n5;
        locals.var_t4_dn6 = assign95260_e147917_d_n6;
        locals.var_t4_dn7 = assign95260_e147917_d_n7;
        locals.var_t4_dn8 = assign95260_e147917_d_n8;
        locals.var_t4_dn9 = assign95260_e147917_d_n9;
        locals.var_t4_dn10 = assign95260_e147917_d_n10;
        locals.var_t4_dn11 = assign95260_e147917_d_n11;
        locals.var_t4_dn14 = assign95260_e147917_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95270_e147925, assign95270_e147925_d_n0, assign95270_e147925_d_n2, assign95270_e147925_d_n4, assign95270_e147925_d_n5, assign95270_e147925_d_n6, assign95270_e147925_d_n7, assign95270_e147925_d_n8, assign95270_e147925_d_n9, assign95270_e147925_d_n10, assign95270_e147925_d_n11, assign95270_e147925_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95270_e147923: f64 = (p.p63 * locals.var_t1);
        (assign95270_e147923, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95270_e147925;
        locals.var_t5_dn0 = assign95270_e147925_d_n0;
        locals.var_t5_dn2 = assign95270_e147925_d_n2;
        locals.var_t5_dn4 = assign95270_e147925_d_n4;
        locals.var_t5_dn5 = assign95270_e147925_d_n5;
        locals.var_t5_dn6 = assign95270_e147925_d_n6;
        locals.var_t5_dn7 = assign95270_e147925_d_n7;
        locals.var_t5_dn8 = assign95270_e147925_d_n8;
        locals.var_t5_dn9 = assign95270_e147925_d_n9;
        locals.var_t5_dn10 = assign95270_e147925_d_n10;
        locals.var_t5_dn11 = assign95270_e147925_d_n11;
        locals.var_t5_dn14 = assign95270_e147925_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_370(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95280_e147935, assign95280_e147935_d_n0, assign95280_e147935_d_n2, assign95280_e147935_d_n4, assign95280_e147935_d_n5, assign95280_e147935_d_n6, assign95280_e147935_d_n7, assign95280_e147935_d_n8, assign95280_e147935_d_n9, assign95280_e147935_d_n10, assign95280_e147935_d_n11, assign95280_e147935_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95280_e147931: f64 = (1.2 + locals.var_vds);
        let assign95280_e147933: f64 = (assign95280_e147931 - locals.var_psl);
        (assign95280_e147933, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn11 - locals.var_psl_dn11), (locals.var_vds_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95280_e147935;
        locals.var_t9_dn0 = assign95280_e147935_d_n0;
        locals.var_t9_dn2 = assign95280_e147935_d_n2;
        locals.var_t9_dn4 = assign95280_e147935_d_n4;
        locals.var_t9_dn5 = assign95280_e147935_d_n5;
        locals.var_t9_dn6 = assign95280_e147935_d_n6;
        locals.var_t9_dn7 = assign95280_e147935_d_n7;
        locals.var_t9_dn8 = assign95280_e147935_d_n8;
        locals.var_t9_dn9 = assign95280_e147935_d_n9;
        locals.var_t9_dn10 = assign95280_e147935_d_n10;
        locals.var_t9_dn11 = assign95280_e147935_d_n11;
        locals.var_t9_dn14 = assign95280_e147935_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95290_e147949, assign95290_e147949_d_n0, assign95290_e147949_d_n2, assign95290_e147949_d_n4, assign95290_e147949_d_n5, assign95290_e147949_d_n6, assign95290_e147949_d_n7, assign95290_e147949_d_n8, assign95290_e147949_d_n9, assign95290_e147949_d_n10, assign95290_e147949_d_n11, assign95290_e147949_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95290_e147941: f64 = (locals.var_vgs - locals.var_vds);
        let assign95290_e147943: f64 = (assign95290_e147941 * locals.var_t5);
        let assign95290_e147946: f64 = (locals.var_t4 * locals.var_t9);
        let assign95290_e147947: f64 = (assign95290_e147943 - assign95290_e147946);
        (assign95290_e147947, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((-locals.var_vds_dn5) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((locals.var_vgs_dn8 - locals.var_vds_dn8) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn11) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn11)) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((((-locals.var_vds_dn14) * locals.var_t5) + (assign95290_e147941 * locals.var_t5_dn14)) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95290_e147949;
        locals.var_qgod_dn0 = assign95290_e147949_d_n0;
        locals.var_qgod_dn2 = assign95290_e147949_d_n2;
        locals.var_qgod_dn4 = assign95290_e147949_d_n4;
        locals.var_qgod_dn5 = assign95290_e147949_d_n5;
        locals.var_qgod_dn6 = assign95290_e147949_d_n6;
        locals.var_qgod_dn7 = assign95290_e147949_d_n7;
        locals.var_qgod_dn8 = assign95290_e147949_d_n8;
        locals.var_qgod_dn9 = assign95290_e147949_d_n9;
        locals.var_qgod_dn10 = assign95290_e147949_d_n10;
        locals.var_qgod_dn11 = assign95290_e147949_d_n11;
        locals.var_qgod_dn14 = assign95290_e147949_d_n14;
        locals.var_qgod_rv = 0.0;

        let (assign95300_e147962, assign95300_e147962_d_n0, assign95300_e147962_d_n2, assign95300_e147962_d_n4, assign95300_e147962_d_n5, assign95300_e147962_d_n6, assign95300_e147962_d_n7, assign95300_e147962_d_n8, assign95300_e147962_d_n9, assign95300_e147962_d_n10, assign95300_e147962_d_n11, assign95300_e147962_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95300_e147956: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95300_e147959: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95300_e147960: f64 = (assign95300_e147956 * assign95300_e147959);
        (assign95300_e147960, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn5) * assign95300_e147959), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95300_e147959) + (assign95300_e147956 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95300_e147959) + (assign95300_e147956 * locals.var_vgs_dn7)), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95300_e147959) + (assign95300_e147956 * locals.var_vgs_dn8)), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn11) * assign95300_e147959), ((locals.var_cov_slp * locals.var_t1_dn14) * assign95300_e147959),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95300_e147962;
        locals.var_t4_dn0 = assign95300_e147962_d_n0;
        locals.var_t4_dn2 = assign95300_e147962_d_n2;
        locals.var_t4_dn4 = assign95300_e147962_d_n4;
        locals.var_t4_dn5 = assign95300_e147962_d_n5;
        locals.var_t4_dn6 = assign95300_e147962_d_n6;
        locals.var_t4_dn7 = assign95300_e147962_d_n7;
        locals.var_t4_dn8 = assign95300_e147962_d_n8;
        locals.var_t4_dn9 = assign95300_e147962_d_n9;
        locals.var_t4_dn10 = assign95300_e147962_d_n10;
        locals.var_t4_dn11 = assign95300_e147962_d_n11;
        locals.var_t4_dn14 = assign95300_e147962_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95310_e147971, assign95310_e147971_d_n0, assign95310_e147971_d_n2, assign95310_e147971_d_n4, assign95310_e147971_d_n5, assign95310_e147971_d_n6, assign95310_e147971_d_n7, assign95310_e147971_d_n8, assign95310_e147971_d_n9, assign95310_e147971_d_n10, assign95310_e147971_d_n11, assign95310_e147971_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95310_e147969: f64 = (p.p63 * locals.var_t1);
        (assign95310_e147969, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95310_e147971;
        locals.var_t5_dn0 = assign95310_e147971_d_n0;
        locals.var_t5_dn2 = assign95310_e147971_d_n2;
        locals.var_t5_dn4 = assign95310_e147971_d_n4;
        locals.var_t5_dn5 = assign95310_e147971_d_n5;
        locals.var_t5_dn6 = assign95310_e147971_d_n6;
        locals.var_t5_dn7 = assign95310_e147971_d_n7;
        locals.var_t5_dn8 = assign95310_e147971_d_n8;
        locals.var_t5_dn9 = assign95310_e147971_d_n9;
        locals.var_t5_dn10 = assign95310_e147971_d_n10;
        locals.var_t5_dn11 = assign95310_e147971_d_n11;
        locals.var_t5_dn14 = assign95310_e147971_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95320_e147980, assign95320_e147980_d_n0, assign95320_e147980_d_n2, assign95320_e147980_d_n4, assign95320_e147980_d_n5, assign95320_e147980_d_n6, assign95320_e147980_d_n7, assign95320_e147980_d_n8, assign95320_e147980_d_n9, assign95320_e147980_d_n10, assign95320_e147980_d_n11, assign95320_e147980_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95320_e147978: f64 = (1.2 - locals.var_ps0);
        (assign95320_e147978, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95320_e147980;
        locals.var_t9_dn0 = assign95320_e147980_d_n0;
        locals.var_t9_dn2 = assign95320_e147980_d_n2;
        locals.var_t9_dn4 = assign95320_e147980_d_n4;
        locals.var_t9_dn5 = assign95320_e147980_d_n5;
        locals.var_t9_dn6 = assign95320_e147980_d_n6;
        locals.var_t9_dn7 = assign95320_e147980_d_n7;
        locals.var_t9_dn8 = assign95320_e147980_d_n8;
        locals.var_t9_dn9 = assign95320_e147980_d_n9;
        locals.var_t9_dn10 = assign95320_e147980_d_n10;
        locals.var_t9_dn11 = assign95320_e147980_d_n11;
        locals.var_t9_dn14 = assign95320_e147980_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95330_e147993, assign95330_e147993_d_n0, assign95330_e147993_d_n2, assign95330_e147993_d_n4, assign95330_e147993_d_n5, assign95330_e147993_d_n6, assign95330_e147993_d_n7, assign95330_e147993_d_n8, assign95330_e147993_d_n9, assign95330_e147993_d_n10, assign95330_e147993_d_n11, assign95330_e147993_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95330_e147987: f64 = (locals.var_vgs * locals.var_t5);
        let assign95330_e147990: f64 = (locals.var_t4 * locals.var_t9);
        let assign95330_e147991: f64 = (assign95330_e147987 - assign95330_e147990);
        (assign95330_e147991, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95330_e147993;
        locals.var_qgod_dn0 = assign95330_e147993_d_n0;
        locals.var_qgod_dn2 = assign95330_e147993_d_n2;
        locals.var_qgod_dn4 = assign95330_e147993_d_n4;
        locals.var_qgod_dn5 = assign95330_e147993_d_n5;
        locals.var_qgod_dn6 = assign95330_e147993_d_n6;
        locals.var_qgod_dn7 = assign95330_e147993_d_n7;
        locals.var_qgod_dn8 = assign95330_e147993_d_n8;
        locals.var_qgod_dn9 = assign95330_e147993_d_n9;
        locals.var_qgod_dn10 = assign95330_e147993_d_n10;
        locals.var_qgod_dn11 = assign95330_e147993_d_n11;
        locals.var_qgod_dn14 = assign95330_e147993_d_n14;
        locals.var_qgod_rv = 0.0;

        let (assign95340_e148000,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95340_e147997: f64 = (-locals.var_weffcv_nf);
        let assign95340_e147998: f64 = (locals.var_uc_cgso * assign95340_e147997);
        (assign95340_e147998,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95340_e148000;
        locals.var_cgsoe_rv = 0.0;

        let assign95350_e148003: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2221 = assign95350_e148003;
        locals.var_guard2221_rv = 0.0;

        let (assign95360_e148015,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2221 != 0.0)) {
        let assign95360_e148009: f64 = (-locals.var_cox0);
        let assign95360_e148011: f64 = (assign95360_e148009 * p.p66);
        let assign95360_e148013: f64 = (assign95360_e148011 * locals.var_weffcv_nf);
        (assign95360_e148013,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95360_e148015;
        locals.var_cgsoe_rv = 0.0;

        let assign95370_e148017: f64 = (-locals.var_cgsoe);
        let assign95370_e148019: f64 = (assign95370_e148017 * locals.var_vgsei);
        locals.var_qgso = assign95370_e148019;
        locals.var_qgso_dn2 = (assign95370_e148017 * locals.var_vgsei_dn2);
        locals.var_qgso_dn7 = (assign95370_e148017 * locals.var_vgsei_dn7);
        locals.var_qgso_rv = 0.0;

        let (assign95380_e148026,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95380_e148023: f64 = (-locals.var_weffcv_nf);
        let assign95380_e148024: f64 = (locals.var_uc_cgdo * assign95380_e148023);
        (assign95380_e148024,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95380_e148026;
        locals.var_cgdoe_rv = 0.0;

        let assign95390_e148029: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2222 = assign95390_e148029;
        locals.var_guard2222_rv = 0.0;

        let (assign95400_e148041,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95400_e148035: f64 = (-locals.var_coxb0);
        let assign95400_e148037: f64 = (assign95400_e148035 * p.p63);
        let assign95400_e148039: f64 = (assign95400_e148037 * locals.var_weffcv_nf);
        (assign95400_e148039,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95400_e148041;
        locals.var_cgdoe_rv = 0.0;

        let assign95410_e148043: f64 = (-locals.var_cgdoe);
        let assign95410_e148046: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95410_e148047: f64 = (assign95410_e148043 * assign95410_e148046);
        locals.var_qgdo = assign95410_e148047;
        locals.var_qgdo_dn0 = (assign95410_e148043 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95410_e148043 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn7 = (assign95410_e148043 * locals.var_vgsei_dn7);
        locals.var_qgdo_rv = 0.0;

        let assign95420_e148050: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95420_e148050;
        locals.var_guard2223_rv = 0.0;

        let (assign95430_e148058, assign95430_e148058_d_n0, assign95430_e148058_d_n2, assign95430_e148058_d_n4, assign95430_e148058_d_n5, assign95430_e148058_d_n6, assign95430_e148058_d_n7, assign95430_e148058_d_n8, assign95430_e148058_d_n9, assign95430_e148058_d_n10, assign95430_e148058_d_n11, assign95430_e148058_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95430_e148055: f64 = (locals.var_vds - locals.var_pds);
        let assign95430_e148056: f64 = (p.p431 * assign95430_e148055);
        (assign95430_e148056, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (p.p431 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95430_e148058;
        locals.var_qodad_dn0 = assign95430_e148058_d_n0;
        locals.var_qodad_dn2 = assign95430_e148058_d_n2;
        locals.var_qodad_dn4 = assign95430_e148058_d_n4;
        locals.var_qodad_dn5 = assign95430_e148058_d_n5;
        locals.var_qodad_dn6 = assign95430_e148058_d_n6;
        locals.var_qodad_dn7 = assign95430_e148058_d_n7;
        locals.var_qodad_dn8 = assign95430_e148058_d_n8;
        locals.var_qodad_dn9 = assign95430_e148058_d_n9;
        locals.var_qodad_dn10 = assign95430_e148058_d_n10;
        locals.var_qodad_dn11 = assign95430_e148058_d_n11;
        locals.var_qodad_dn14 = assign95430_e148058_d_n14;
        locals.var_qodad_rv = 0.0;

        let (assign95440_e148064, assign95440_e148064_d_n0, assign95440_e148064_d_n2, assign95440_e148064_d_n4, assign95440_e148064_d_n5, assign95440_e148064_d_n6, assign95440_e148064_d_n7, assign95440_e148064_d_n8, assign95440_e148064_d_n9, assign95440_e148064_d_n10, assign95440_e148064_d_n11, assign95440_e148064_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95440_e148062: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95440_e148062, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn11, locals.var_qovd_add_dn14,)
    }
};
        locals.var_qovd_add = assign95440_e148064;
        locals.var_qovd_add_dn0 = assign95440_e148064_d_n0;
        locals.var_qovd_add_dn2 = assign95440_e148064_d_n2;
        locals.var_qovd_add_dn4 = assign95440_e148064_d_n4;
        locals.var_qovd_add_dn5 = assign95440_e148064_d_n5;
        locals.var_qovd_add_dn6 = assign95440_e148064_d_n6;
        locals.var_qovd_add_dn7 = assign95440_e148064_d_n7;
        locals.var_qovd_add_dn8 = assign95440_e148064_d_n8;
        locals.var_qovd_add_dn9 = assign95440_e148064_d_n9;
        locals.var_qovd_add_dn10 = assign95440_e148064_d_n10;
        locals.var_qovd_add_dn11 = assign95440_e148064_d_n11;
        locals.var_qovd_add_dn14 = assign95440_e148064_d_n14;
        locals.var_qovd_add_rv = 0.0;

        let (assign95450_e148070, assign95450_e148070_d_n0, assign95450_e148070_d_n2, assign95450_e148070_d_n4, assign95450_e148070_d_n5, assign95450_e148070_d_n6, assign95450_e148070_d_n7, assign95450_e148070_d_n8, assign95450_e148070_d_n9, assign95450_e148070_d_n10, assign95450_e148070_d_n11, assign95450_e148070_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95450_e148068: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95450_e148068, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn11, locals.var_qbdld_add_dn14,)
    }
};
        locals.var_qbdld_add = assign95450_e148070;
        locals.var_qbdld_add_dn0 = assign95450_e148070_d_n0;
        locals.var_qbdld_add_dn2 = assign95450_e148070_d_n2;
        locals.var_qbdld_add_dn4 = assign95450_e148070_d_n4;
        locals.var_qbdld_add_dn5 = assign95450_e148070_d_n5;
        locals.var_qbdld_add_dn6 = assign95450_e148070_d_n6;
        locals.var_qbdld_add_dn7 = assign95450_e148070_d_n7;
        locals.var_qbdld_add_dn8 = assign95450_e148070_d_n8;
        locals.var_qbdld_add_dn9 = assign95450_e148070_d_n9;
        locals.var_qbdld_add_dn10 = assign95450_e148070_d_n10;
        locals.var_qbdld_add_dn11 = assign95450_e148070_d_n11;
        locals.var_qbdld_add_dn14 = assign95450_e148070_d_n14;
        locals.var_qbdld_add_rv = 0.0;

        let (assign95460_e148080, assign95460_e148080_d_n0, assign95460_e148080_d_n2, assign95460_e148080_d_n4, assign95460_e148080_d_n5, assign95460_e148080_d_n6, assign95460_e148080_d_n7, assign95460_e148080_d_n8, assign95460_e148080_d_n9, assign95460_e148080_d_n10, assign95460_e148080_d_n11, assign95460_e148080_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95460_e148074: f64 = (-p.p431);
        let assign95460_e148077: f64 = (locals.var_vds - locals.var_pds);
        let assign95460_e148078: f64 = (assign95460_e148074 * assign95460_e148077);
        (assign95460_e148078, (assign95460_e148074 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95460_e148074 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95460_e148074 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95460_e148074 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95460_e148074 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95460_e148074 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95460_e148074 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95460_e148074 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95460_e148074 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95460_e148074 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (assign95460_e148074 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95460_e148080;
        locals.var_qodad_dn0 = assign95460_e148080_d_n0;
        locals.var_qodad_dn2 = assign95460_e148080_d_n2;
        locals.var_qodad_dn4 = assign95460_e148080_d_n4;
        locals.var_qodad_dn5 = assign95460_e148080_d_n5;
        locals.var_qodad_dn6 = assign95460_e148080_d_n6;
        locals.var_qodad_dn7 = assign95460_e148080_d_n7;
        locals.var_qodad_dn8 = assign95460_e148080_d_n8;
        locals.var_qodad_dn9 = assign95460_e148080_d_n9;
        locals.var_qodad_dn10 = assign95460_e148080_d_n10;
        locals.var_qodad_dn11 = assign95460_e148080_d_n11;
        locals.var_qodad_dn14 = assign95460_e148080_d_n14;
        locals.var_qodad_rv = 0.0;

        let (assign95470_e148087, assign95470_e148087_d_n0, assign95470_e148087_d_n2, assign95470_e148087_d_n4, assign95470_e148087_d_n5, assign95470_e148087_d_n6, assign95470_e148087_d_n7, assign95470_e148087_d_n8, assign95470_e148087_d_n9, assign95470_e148087_d_n10, assign95470_e148087_d_n11, assign95470_e148087_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95470_e148085: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95470_e148085, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn11, locals.var_qovs_add_dn14,)
    }
};
        locals.var_qovs_add = assign95470_e148087;
        locals.var_qovs_add_dn0 = assign95470_e148087_d_n0;
        locals.var_qovs_add_dn2 = assign95470_e148087_d_n2;
        locals.var_qovs_add_dn4 = assign95470_e148087_d_n4;
        locals.var_qovs_add_dn5 = assign95470_e148087_d_n5;
        locals.var_qovs_add_dn6 = assign95470_e148087_d_n6;
        locals.var_qovs_add_dn7 = assign95470_e148087_d_n7;
        locals.var_qovs_add_dn8 = assign95470_e148087_d_n8;
        locals.var_qovs_add_dn9 = assign95470_e148087_d_n9;
        locals.var_qovs_add_dn10 = assign95470_e148087_d_n10;
        locals.var_qovs_add_dn11 = assign95470_e148087_d_n11;
        locals.var_qovs_add_dn14 = assign95470_e148087_d_n14;
        locals.var_qovs_add_rv = 0.0;

        let (assign95480_e148094, assign95480_e148094_d_n0, assign95480_e148094_d_n2, assign95480_e148094_d_n4, assign95480_e148094_d_n5, assign95480_e148094_d_n6, assign95480_e148094_d_n7, assign95480_e148094_d_n8, assign95480_e148094_d_n9, assign95480_e148094_d_n10, assign95480_e148094_d_n11, assign95480_e148094_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95480_e148092: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95480_e148092, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn11, locals.var_qbsld_add_dn14,)
    }
};
        locals.var_qbsld_add = assign95480_e148094;
        locals.var_qbsld_add_dn0 = assign95480_e148094_d_n0;
        locals.var_qbsld_add_dn2 = assign95480_e148094_d_n2;
        locals.var_qbsld_add_dn4 = assign95480_e148094_d_n4;
        locals.var_qbsld_add_dn5 = assign95480_e148094_d_n5;
        locals.var_qbsld_add_dn6 = assign95480_e148094_d_n6;
        locals.var_qbsld_add_dn7 = assign95480_e148094_d_n7;
        locals.var_qbsld_add_dn8 = assign95480_e148094_d_n8;
        locals.var_qbsld_add_dn9 = assign95480_e148094_d_n9;
        locals.var_qbsld_add_dn10 = assign95480_e148094_d_n10;
        locals.var_qbsld_add_dn11 = assign95480_e148094_d_n11;
        locals.var_qbsld_add_dn14 = assign95480_e148094_d_n14;
        locals.var_qbsld_add_rv = 0.0;

        let assign95490_e148096: f64 = (-locals.var_uc_cgbo);
        let assign95490_e148098: f64 = (assign95490_e148096 * locals.var_lgate);
        locals.var_cgbo_loc = assign95490_e148098;
        locals.var_cgbo_loc_rv = 0.0;

        let assign95500_e148100: f64 = (-locals.var_cgbo_loc);
        let assign95500_e148103: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95500_e148104: f64 = (assign95500_e148100 * assign95500_e148103);
        locals.var_qgbo = assign95500_e148104;
        locals.var_qgbo_dn7 = (assign95500_e148100 * locals.var_vgsi_dn7);
        locals.var_qgbo_dn8 = (assign95500_e148100 * (locals.var_vgsi_dn8 - locals.var_vbsi_dn8));
        locals.var_qgbo_dn9 = (assign95500_e148100 * (-locals.var_vbsi_dn9));
        locals.var_qgbo_rv = 0.0;

        locals.var_aclm = locals.var_uc_clm1;
        locals.var_aclm_rv = 0.0;

        let assign95520_e148108: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95520_e148108;
        locals.var_guard2224_rv = 0.0;

        let (assign95530_e148122, assign95530_e148122_d_n0, assign95530_e148122_d_n2, assign95530_e148122_d_n4, assign95530_e148122_d_n5, assign95530_e148122_d_n6, assign95530_e148122_d_n7, assign95530_e148122_d_n8, assign95530_e148122_d_n9, assign95530_e148122_d_n10, assign95530_e148122_d_n11, assign95530_e148122_d_n14,) = {
    if (locals.var_guard2224 != 0.0) {
        let assign95530_e148113: f64 = (locals.var_vds + locals.var_ps0);
        let assign95530_e148114: f64 = (locals.var_aclm * assign95530_e148113);
        let assign95530_e148117: f64 = (1.0 - locals.var_aclm);
        let assign95530_e148119: f64 = (assign95530_e148117 * locals.var_psl);
        let assign95530_e148120: f64 = (assign95530_e148114 + assign95530_e148119);
        (assign95530_e148120, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95530_e148117 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95530_e148117 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95530_e148117 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95530_e148117 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95530_e148117 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95530_e148117 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95530_e148117 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95530_e148117 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95530_e148117 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign95530_e148117 * locals.var_psl_dn11)), ((locals.var_aclm * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + (assign95530_e148117 * locals.var_psl_dn14)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95530_e148122;
        locals.var_psdl_dn0 = assign95530_e148122_d_n0;
        locals.var_psdl_dn2 = assign95530_e148122_d_n2;
        locals.var_psdl_dn4 = assign95530_e148122_d_n4;
        locals.var_psdl_dn5 = assign95530_e148122_d_n5;
        locals.var_psdl_dn6 = assign95530_e148122_d_n6;
        locals.var_psdl_dn7 = assign95530_e148122_d_n7;
        locals.var_psdl_dn8 = assign95530_e148122_d_n8;
        locals.var_psdl_dn9 = assign95530_e148122_d_n9;
        locals.var_psdl_dn10 = assign95530_e148122_d_n10;
        locals.var_psdl_dn11 = assign95530_e148122_d_n11;
        locals.var_psdl_dn14 = assign95530_e148122_d_n14;
        locals.var_psdl_rv = 0.0;

        let assign95540_e148126: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95540_e148129: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148130: f64 = (assign95540_e148126 - assign95540_e148129);
        let assign95540_e148133: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148134: f64 = (assign95540_e148130 - assign95540_e148133);
        let assign95540_e148138: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148141: f64 = if ((locals.var_psdl > assign95540_e148134) && (assign95540_e148138 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95540_e148141;
        locals.var_guard2225_rv = 0.0;

        let (assign95550_e148159, assign95550_e148159_d_n0, assign95550_e148159_d_n2, assign95550_e148159_d_n4, assign95550_e148159_d_n5, assign95550_e148159_d_n6, assign95550_e148159_d_n7, assign95550_e148159_d_n8, assign95550_e148159_d_n9, assign95550_e148159_d_n10, assign95550_e148159_d_n11, assign95550_e148159_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95550_e148148: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95550_e148151: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148152: f64 = (assign95550_e148148 - assign95550_e148151);
        let assign95550_e148153: f64 = (locals.var_psdl - assign95550_e148152);
        let assign95550_e148156: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148157: f64 = (assign95550_e148153 + assign95550_e148156);
        (assign95550_e148157, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign95550_e148159;
        locals.var_tmf1_dn0 = assign95550_e148159_d_n0;
        locals.var_tmf1_dn2 = assign95550_e148159_d_n2;
        locals.var_tmf1_dn4 = assign95550_e148159_d_n4;
        locals.var_tmf1_dn5 = assign95550_e148159_d_n5;
        locals.var_tmf1_dn6 = assign95550_e148159_d_n6;
        locals.var_tmf1_dn7 = assign95550_e148159_d_n7;
        locals.var_tmf1_dn8 = assign95550_e148159_d_n8;
        locals.var_tmf1_dn9 = assign95550_e148159_d_n9;
        locals.var_tmf1_dn10 = assign95550_e148159_d_n10;
        locals.var_tmf1_dn11 = assign95550_e148159_d_n11;
        locals.var_tmf1_dn14 = assign95550_e148159_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign95560_e148167, assign95560_e148167_d_n0, assign95560_e148167_d_n2, assign95560_e148167_d_n4, assign95560_e148167_d_n5, assign95560_e148167_d_n6, assign95560_e148167_d_n7, assign95560_e148167_d_n8, assign95560_e148167_d_n9, assign95560_e148167_d_n10, assign95560_e148167_d_n11, assign95560_e148167_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95560_e148165: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95560_e148165, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign95560_e148167;
        locals.var_x2_dn0 = assign95560_e148167_d_n0;
        locals.var_x2_dn2 = assign95560_e148167_d_n2;
        locals.var_x2_dn4 = assign95560_e148167_d_n4;
        locals.var_x2_dn5 = assign95560_e148167_d_n5;
        locals.var_x2_dn6 = assign95560_e148167_d_n6;
        locals.var_x2_dn7 = assign95560_e148167_d_n7;
        locals.var_x2_dn8 = assign95560_e148167_d_n8;
        locals.var_x2_dn9 = assign95560_e148167_d_n9;
        locals.var_x2_dn10 = assign95560_e148167_d_n10;
        locals.var_x2_dn11 = assign95560_e148167_d_n11;
        locals.var_x2_dn14 = assign95560_e148167_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign95570_e148179, assign95570_e148179_d_n0, assign95570_e148179_d_n2, assign95570_e148179_d_n4, assign95570_e148179_d_n5, assign95570_e148179_d_n6, assign95570_e148179_d_n7, assign95570_e148179_d_n8, assign95570_e148179_d_n9, assign95570_e148179_d_n10, assign95570_e148179_d_n11, assign95570_e148179_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95570_e148173: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148176: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148177: f64 = (assign95570_e148173 * assign95570_e148176);
        (assign95570_e148177, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign95570_e148179;
        locals.var_xmax2_dn0 = assign95570_e148179_d_n0;
        locals.var_xmax2_dn2 = assign95570_e148179_d_n2;
        locals.var_xmax2_dn4 = assign95570_e148179_d_n4;
        locals.var_xmax2_dn5 = assign95570_e148179_d_n5;
        locals.var_xmax2_dn6 = assign95570_e148179_d_n6;
        locals.var_xmax2_dn7 = assign95570_e148179_d_n7;
        locals.var_xmax2_dn8 = assign95570_e148179_d_n8;
        locals.var_xmax2_dn9 = assign95570_e148179_d_n9;
        locals.var_xmax2_dn10 = assign95570_e148179_d_n10;
        locals.var_xmax2_dn11 = assign95570_e148179_d_n11;
        locals.var_xmax2_dn14 = assign95570_e148179_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign95580_e148185, assign95580_e148185_d_n0, assign95580_e148185_d_n2, assign95580_e148185_d_n4, assign95580_e148185_d_n5, assign95580_e148185_d_n6, assign95580_e148185_d_n7, assign95580_e148185_d_n8, assign95580_e148185_d_n9, assign95580_e148185_d_n10, assign95580_e148185_d_n11, assign95580_e148185_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95580_e148185;
        locals.var_xp_dn0 = assign95580_e148185_d_n0;
        locals.var_xp_dn2 = assign95580_e148185_d_n2;
        locals.var_xp_dn4 = assign95580_e148185_d_n4;
        locals.var_xp_dn5 = assign95580_e148185_d_n5;
        locals.var_xp_dn6 = assign95580_e148185_d_n6;
        locals.var_xp_dn7 = assign95580_e148185_d_n7;
        locals.var_xp_dn8 = assign95580_e148185_d_n8;
        locals.var_xp_dn9 = assign95580_e148185_d_n9;
        locals.var_xp_dn10 = assign95580_e148185_d_n10;
        locals.var_xp_dn11 = assign95580_e148185_d_n11;
        locals.var_xp_dn14 = assign95580_e148185_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_371(
        locals: &mut StampLocals,
    ) {
        let (assign95590_e148191, assign95590_e148191_d_n0, assign95590_e148191_d_n2, assign95590_e148191_d_n4, assign95590_e148191_d_n5, assign95590_e148191_d_n6, assign95590_e148191_d_n7, assign95590_e148191_d_n8, assign95590_e148191_d_n9, assign95590_e148191_d_n10, assign95590_e148191_d_n11, assign95590_e148191_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95590_e148191;
        locals.var_xmp_dn0 = assign95590_e148191_d_n0;
        locals.var_xmp_dn2 = assign95590_e148191_d_n2;
        locals.var_xmp_dn4 = assign95590_e148191_d_n4;
        locals.var_xmp_dn5 = assign95590_e148191_d_n5;
        locals.var_xmp_dn6 = assign95590_e148191_d_n6;
        locals.var_xmp_dn7 = assign95590_e148191_d_n7;
        locals.var_xmp_dn8 = assign95590_e148191_d_n8;
        locals.var_xmp_dn9 = assign95590_e148191_d_n9;
        locals.var_xmp_dn10 = assign95590_e148191_d_n10;
        locals.var_xmp_dn11 = assign95590_e148191_d_n11;
        locals.var_xmp_dn14 = assign95590_e148191_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95600_e148197,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95600_e148197;
        locals.var_m0_rv = 0.0;

        let (assign95610_e148203,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95610_e148203;
        locals.var_mm_rv = 0.0;

        let (assign95620_e148209, assign95620_e148209_d_n0, assign95620_e148209_d_n2, assign95620_e148209_d_n4, assign95620_e148209_d_n5, assign95620_e148209_d_n6, assign95620_e148209_d_n7, assign95620_e148209_d_n8, assign95620_e148209_d_n9, assign95620_e148209_d_n10, assign95620_e148209_d_n11, assign95620_e148209_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95620_e148209;
        locals.var_arg_dn0 = assign95620_e148209_d_n0;
        locals.var_arg_dn2 = assign95620_e148209_d_n2;
        locals.var_arg_dn4 = assign95620_e148209_d_n4;
        locals.var_arg_dn5 = assign95620_e148209_d_n5;
        locals.var_arg_dn6 = assign95620_e148209_d_n6;
        locals.var_arg_dn7 = assign95620_e148209_d_n7;
        locals.var_arg_dn8 = assign95620_e148209_d_n8;
        locals.var_arg_dn9 = assign95620_e148209_d_n9;
        locals.var_arg_dn10 = assign95620_e148209_d_n10;
        locals.var_arg_dn11 = assign95620_e148209_d_n11;
        locals.var_arg_dn14 = assign95620_e148209_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign95630_e148215, assign95630_e148215_d_n0, assign95630_e148215_d_n2, assign95630_e148215_d_n4, assign95630_e148215_d_n5, assign95630_e148215_d_n6, assign95630_e148215_d_n7, assign95630_e148215_d_n8, assign95630_e148215_d_n9, assign95630_e148215_d_n10, assign95630_e148215_d_n11, assign95630_e148215_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95630_e148215;
        locals.var_dnm_dn0 = assign95630_e148215_d_n0;
        locals.var_dnm_dn2 = assign95630_e148215_d_n2;
        locals.var_dnm_dn4 = assign95630_e148215_d_n4;
        locals.var_dnm_dn5 = assign95630_e148215_d_n5;
        locals.var_dnm_dn6 = assign95630_e148215_d_n6;
        locals.var_dnm_dn7 = assign95630_e148215_d_n7;
        locals.var_dnm_dn8 = assign95630_e148215_d_n8;
        locals.var_dnm_dn9 = assign95630_e148215_d_n9;
        locals.var_dnm_dn10 = assign95630_e148215_d_n10;
        locals.var_dnm_dn11 = assign95630_e148215_d_n11;
        locals.var_dnm_dn14 = assign95630_e148215_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95640_e148223, assign95640_e148223_d_n0, assign95640_e148223_d_n2, assign95640_e148223_d_n4, assign95640_e148223_d_n5, assign95640_e148223_d_n6, assign95640_e148223_d_n7, assign95640_e148223_d_n8, assign95640_e148223_d_n9, assign95640_e148223_d_n10, assign95640_e148223_d_n11, assign95640_e148223_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95640_e148221: f64 = (locals.var_xp * locals.var_x2);
        (assign95640_e148221, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95640_e148223;
        locals.var_xp_dn0 = assign95640_e148223_d_n0;
        locals.var_xp_dn2 = assign95640_e148223_d_n2;
        locals.var_xp_dn4 = assign95640_e148223_d_n4;
        locals.var_xp_dn5 = assign95640_e148223_d_n5;
        locals.var_xp_dn6 = assign95640_e148223_d_n6;
        locals.var_xp_dn7 = assign95640_e148223_d_n7;
        locals.var_xp_dn8 = assign95640_e148223_d_n8;
        locals.var_xp_dn9 = assign95640_e148223_d_n9;
        locals.var_xp_dn10 = assign95640_e148223_d_n10;
        locals.var_xp_dn11 = assign95640_e148223_d_n11;
        locals.var_xp_dn14 = assign95640_e148223_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign95650_e148231, assign95650_e148231_d_n0, assign95650_e148231_d_n2, assign95650_e148231_d_n4, assign95650_e148231_d_n5, assign95650_e148231_d_n6, assign95650_e148231_d_n7, assign95650_e148231_d_n8, assign95650_e148231_d_n9, assign95650_e148231_d_n10, assign95650_e148231_d_n11, assign95650_e148231_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95650_e148229: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95650_e148229, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95650_e148231;
        locals.var_xmp_dn0 = assign95650_e148231_d_n0;
        locals.var_xmp_dn2 = assign95650_e148231_d_n2;
        locals.var_xmp_dn4 = assign95650_e148231_d_n4;
        locals.var_xmp_dn5 = assign95650_e148231_d_n5;
        locals.var_xmp_dn6 = assign95650_e148231_d_n6;
        locals.var_xmp_dn7 = assign95650_e148231_d_n7;
        locals.var_xmp_dn8 = assign95650_e148231_d_n8;
        locals.var_xmp_dn9 = assign95650_e148231_d_n9;
        locals.var_xmp_dn10 = assign95650_e148231_d_n10;
        locals.var_xmp_dn11 = assign95650_e148231_d_n11;
        locals.var_xmp_dn14 = assign95650_e148231_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95660_e148239, assign95660_e148239_d_n0, assign95660_e148239_d_n2, assign95660_e148239_d_n4, assign95660_e148239_d_n5, assign95660_e148239_d_n6, assign95660_e148239_d_n7, assign95660_e148239_d_n8, assign95660_e148239_d_n9, assign95660_e148239_d_n10, assign95660_e148239_d_n11, assign95660_e148239_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95660_e148237: f64 = (locals.var_xp * locals.var_x2);
        (assign95660_e148237, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95660_e148239;
        locals.var_xp_dn0 = assign95660_e148239_d_n0;
        locals.var_xp_dn2 = assign95660_e148239_d_n2;
        locals.var_xp_dn4 = assign95660_e148239_d_n4;
        locals.var_xp_dn5 = assign95660_e148239_d_n5;
        locals.var_xp_dn6 = assign95660_e148239_d_n6;
        locals.var_xp_dn7 = assign95660_e148239_d_n7;
        locals.var_xp_dn8 = assign95660_e148239_d_n8;
        locals.var_xp_dn9 = assign95660_e148239_d_n9;
        locals.var_xp_dn10 = assign95660_e148239_d_n10;
        locals.var_xp_dn11 = assign95660_e148239_d_n11;
        locals.var_xp_dn14 = assign95660_e148239_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign95670_e148247, assign95670_e148247_d_n0, assign95670_e148247_d_n2, assign95670_e148247_d_n4, assign95670_e148247_d_n5, assign95670_e148247_d_n6, assign95670_e148247_d_n7, assign95670_e148247_d_n8, assign95670_e148247_d_n9, assign95670_e148247_d_n10, assign95670_e148247_d_n11, assign95670_e148247_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95670_e148245: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95670_e148245, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95670_e148247;
        locals.var_xmp_dn0 = assign95670_e148247_d_n0;
        locals.var_xmp_dn2 = assign95670_e148247_d_n2;
        locals.var_xmp_dn4 = assign95670_e148247_d_n4;
        locals.var_xmp_dn5 = assign95670_e148247_d_n5;
        locals.var_xmp_dn6 = assign95670_e148247_d_n6;
        locals.var_xmp_dn7 = assign95670_e148247_d_n7;
        locals.var_xmp_dn8 = assign95670_e148247_d_n8;
        locals.var_xmp_dn9 = assign95670_e148247_d_n9;
        locals.var_xmp_dn10 = assign95670_e148247_d_n10;
        locals.var_xmp_dn11 = assign95670_e148247_d_n11;
        locals.var_xmp_dn14 = assign95670_e148247_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95680_e148255, assign95680_e148255_d_n0, assign95680_e148255_d_n2, assign95680_e148255_d_n4, assign95680_e148255_d_n5, assign95680_e148255_d_n6, assign95680_e148255_d_n7, assign95680_e148255_d_n8, assign95680_e148255_d_n9, assign95680_e148255_d_n10, assign95680_e148255_d_n11, assign95680_e148255_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95680_e148253: f64 = (locals.var_xp + locals.var_xmp);
        (assign95680_e148253, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95680_e148255;
        locals.var_arg_dn0 = assign95680_e148255_d_n0;
        locals.var_arg_dn2 = assign95680_e148255_d_n2;
        locals.var_arg_dn4 = assign95680_e148255_d_n4;
        locals.var_arg_dn5 = assign95680_e148255_d_n5;
        locals.var_arg_dn6 = assign95680_e148255_d_n6;
        locals.var_arg_dn7 = assign95680_e148255_d_n7;
        locals.var_arg_dn8 = assign95680_e148255_d_n8;
        locals.var_arg_dn9 = assign95680_e148255_d_n9;
        locals.var_arg_dn10 = assign95680_e148255_d_n10;
        locals.var_arg_dn11 = assign95680_e148255_d_n11;
        locals.var_arg_dn14 = assign95680_e148255_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign95690_e148261, assign95690_e148261_d_n0, assign95690_e148261_d_n2, assign95690_e148261_d_n4, assign95690_e148261_d_n5, assign95690_e148261_d_n6, assign95690_e148261_d_n7, assign95690_e148261_d_n8, assign95690_e148261_d_n9, assign95690_e148261_d_n10, assign95690_e148261_d_n11, assign95690_e148261_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95690_e148261;
        locals.var_dnm_dn0 = assign95690_e148261_d_n0;
        locals.var_dnm_dn2 = assign95690_e148261_d_n2;
        locals.var_dnm_dn4 = assign95690_e148261_d_n4;
        locals.var_dnm_dn5 = assign95690_e148261_d_n5;
        locals.var_dnm_dn6 = assign95690_e148261_d_n6;
        locals.var_dnm_dn7 = assign95690_e148261_d_n7;
        locals.var_dnm_dn8 = assign95690_e148261_d_n8;
        locals.var_dnm_dn9 = assign95690_e148261_d_n9;
        locals.var_dnm_dn10 = assign95690_e148261_d_n10;
        locals.var_dnm_dn11 = assign95690_e148261_d_n11;
        locals.var_dnm_dn14 = assign95690_e148261_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign95700_e148276: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95700_e148276;
        locals.var_guard2226_rv = 0.0;

        let assign95710_e148279: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95710_e148279;
        locals.var_guard2227_rv = 0.0;

        let (assign95720_e148289,) = {
    if ((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95720_e148289;
        locals.var_mm_rv = 0.0;

        let assign95730_e148292: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95730_e148292;
        locals.var_guard2228_rv = 0.0;

        let (assign95740_e148305,) = {
    if (((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148305;
        locals.var_mm_rv = 0.0;

        let assign95750_e148308: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95750_e148308;
        locals.var_guard2229_rv = 0.0;

        let (assign95760_e148324,) = {
    if ((((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148324;
        locals.var_mm_rv = 0.0;

        let assign95770_e148327: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95770_e148327;
        locals.var_guard2230_rv = 0.0;

        let (assign95780_e148346,) = {
    if (((((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95780_e148346;
        locals.var_mm_rv = 0.0;

        let (assign95790_e148354,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95790_e148354;
        locals.var_m0_rv = 0.0;

        let mut assign95800_loop_guard: usize = 0;
        while {
            let assign95800_cond_e148363: f64 = if ((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95800_cond_e148363 != 0.0
        } {
            assign95800_loop_guard += 1;
            assert!(assign95800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95800_body0_e148372, assign95800_body0_e148372_d_n0, assign95800_body0_e148372_d_n2, assign95800_body0_e148372_d_n4, assign95800_body0_e148372_d_n5, assign95800_body0_e148372_d_n6, assign95800_body0_e148372_d_n7, assign95800_body0_e148372_d_n8, assign95800_body0_e148372_d_n9, assign95800_body0_e148372_d_n10, assign95800_body0_e148372_d_n11, assign95800_body0_e148372_d_n14,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        let assign95800_body0_e148370: f64 = (locals.var_dnm).sqrt();
        (assign95800_body0_e148370, (locals.var_dnm_dn0 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn2 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn4 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn5 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn6 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn7 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn8 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn9 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn10 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn11 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn14 / (2.0 * assign95800_body0_e148370)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign95800_body0_e148372;
            locals.var_dnm_dn0 = assign95800_body0_e148372_d_n0;
            locals.var_dnm_dn2 = assign95800_body0_e148372_d_n2;
            locals.var_dnm_dn4 = assign95800_body0_e148372_d_n4;
            locals.var_dnm_dn5 = assign95800_body0_e148372_d_n5;
            locals.var_dnm_dn6 = assign95800_body0_e148372_d_n6;
            locals.var_dnm_dn7 = assign95800_body0_e148372_d_n7;
            locals.var_dnm_dn8 = assign95800_body0_e148372_d_n8;
            locals.var_dnm_dn9 = assign95800_body0_e148372_d_n9;
            locals.var_dnm_dn10 = assign95800_body0_e148372_d_n10;
            locals.var_dnm_dn11 = assign95800_body0_e148372_d_n11;
            locals.var_dnm_dn14 = assign95800_body0_e148372_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign95800_body1_e148382,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        let assign95800_body1_e148380: f64 = (locals.var_m0 + 1.0);
        (assign95800_body1_e148380,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95800_body1_e148382;
            locals.var_m0_rv = 0.0;
        }

        let (assign95810_e148402, assign95810_e148402_d_n0, assign95810_e148402_d_n2, assign95810_e148402_d_n4, assign95810_e148402_d_n5, assign95810_e148402_d_n6, assign95810_e148402_d_n7, assign95810_e148402_d_n8, assign95810_e148402_d_n9, assign95810_e148402_d_n10, assign95810_e148402_d_n11, assign95810_e148402_d_n14,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 == 0.0)) {
        let (assign95810_e148400, assign95810_e148400_d_n0, assign95810_e148400_d_n2, assign95810_e148400_d_n4, assign95810_e148400_d_n5, assign95810_e148400_d_n6, assign95810_e148400_d_n7, assign95810_e148400_d_n8, assign95810_e148400_d_n9, assign95810_e148400_d_n10, assign95810_e148400_d_n11, assign95810_e148400_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95810_e148397: f64 = (2.0 * 2.0);
                let assign95810_e148398: f64 = (1.0 / assign95810_e148397);
                let assign95810_e148399: f64 = (locals.var_dnm).powf(assign95810_e148398);
                (assign95810_e148399, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn11)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn14)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign95810_e148400, assign95810_e148400_d_n0, assign95810_e148400_d_n2, assign95810_e148400_d_n4, assign95810_e148400_d_n5, assign95810_e148400_d_n6, assign95810_e148400_d_n7, assign95810_e148400_d_n8, assign95810_e148400_d_n9, assign95810_e148400_d_n10, assign95810_e148400_d_n11, assign95810_e148400_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95810_e148402;
        locals.var_dnm_dn0 = assign95810_e148402_d_n0;
        locals.var_dnm_dn2 = assign95810_e148402_d_n2;
        locals.var_dnm_dn4 = assign95810_e148402_d_n4;
        locals.var_dnm_dn5 = assign95810_e148402_d_n5;
        locals.var_dnm_dn6 = assign95810_e148402_d_n6;
        locals.var_dnm_dn7 = assign95810_e148402_d_n7;
        locals.var_dnm_dn8 = assign95810_e148402_d_n8;
        locals.var_dnm_dn9 = assign95810_e148402_d_n9;
        locals.var_dnm_dn10 = assign95810_e148402_d_n10;
        locals.var_dnm_dn11 = assign95810_e148402_d_n11;
        locals.var_dnm_dn14 = assign95810_e148402_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95820_e148410, assign95820_e148410_d_n0, assign95820_e148410_d_n2, assign95820_e148410_d_n4, assign95820_e148410_d_n5, assign95820_e148410_d_n6, assign95820_e148410_d_n7, assign95820_e148410_d_n8, assign95820_e148410_d_n9, assign95820_e148410_d_n10, assign95820_e148410_d_n11, assign95820_e148410_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95820_e148408: f64 = (1.0 / locals.var_dnm);
        (assign95820_e148408, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95820_e148410;
        locals.var_dnm_dn0 = assign95820_e148410_d_n0;
        locals.var_dnm_dn2 = assign95820_e148410_d_n2;
        locals.var_dnm_dn4 = assign95820_e148410_d_n4;
        locals.var_dnm_dn5 = assign95820_e148410_d_n5;
        locals.var_dnm_dn6 = assign95820_e148410_d_n6;
        locals.var_dnm_dn7 = assign95820_e148410_d_n7;
        locals.var_dnm_dn8 = assign95820_e148410_d_n8;
        locals.var_dnm_dn9 = assign95820_e148410_d_n9;
        locals.var_dnm_dn10 = assign95820_e148410_d_n10;
        locals.var_dnm_dn11 = assign95820_e148410_d_n11;
        locals.var_dnm_dn14 = assign95820_e148410_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95830_e148422, assign95830_e148422_d_n0, assign95830_e148422_d_n2, assign95830_e148422_d_n4, assign95830_e148422_d_n5, assign95830_e148422_d_n6, assign95830_e148422_d_n7, assign95830_e148422_d_n8, assign95830_e148422_d_n9, assign95830_e148422_d_n10, assign95830_e148422_d_n11, assign95830_e148422_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95830_e148417: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148418: f64 = (locals.var_tmf1 * assign95830_e148417);
        let assign95830_e148420: f64 = (assign95830_e148418 * locals.var_dnm);
        (assign95830_e148420, (((locals.var_tmf1_dn0 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign95830_e148422;
        locals.var_tmf0_dn0 = assign95830_e148422_d_n0;
        locals.var_tmf0_dn2 = assign95830_e148422_d_n2;
        locals.var_tmf0_dn4 = assign95830_e148422_d_n4;
        locals.var_tmf0_dn5 = assign95830_e148422_d_n5;
        locals.var_tmf0_dn6 = assign95830_e148422_d_n6;
        locals.var_tmf0_dn7 = assign95830_e148422_d_n7;
        locals.var_tmf0_dn8 = assign95830_e148422_d_n8;
        locals.var_tmf0_dn9 = assign95830_e148422_d_n9;
        locals.var_tmf0_dn10 = assign95830_e148422_d_n10;
        locals.var_tmf0_dn11 = assign95830_e148422_d_n11;
        locals.var_tmf0_dn14 = assign95830_e148422_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign95840_e148436, assign95840_e148436_d_n0, assign95840_e148436_d_n2, assign95840_e148436_d_n4, assign95840_e148436_d_n5, assign95840_e148436_d_n6, assign95840_e148436_d_n7, assign95840_e148436_d_n8, assign95840_e148436_d_n9, assign95840_e148436_d_n10, assign95840_e148436_d_n11, assign95840_e148436_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95840_e148428: f64 = (10.0 * 2.220446049250313e-16);
        let assign95840_e148430: f64 = (assign95840_e148428 * locals.var_xmp);
        let assign95840_e148432: f64 = (assign95840_e148430 * locals.var_dnm);
        let assign95840_e148434: f64 = (assign95840_e148432 / locals.var_arg);
        (assign95840_e148434, ((((((assign95840_e148428 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn11) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn11)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn14) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn14)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95840_e148436;
        locals.var_t0_dn0 = assign95840_e148436_d_n0;
        locals.var_t0_dn2 = assign95840_e148436_d_n2;
        locals.var_t0_dn4 = assign95840_e148436_d_n4;
        locals.var_t0_dn5 = assign95840_e148436_d_n5;
        locals.var_t0_dn6 = assign95840_e148436_d_n6;
        locals.var_t0_dn7 = assign95840_e148436_d_n7;
        locals.var_t0_dn8 = assign95840_e148436_d_n8;
        locals.var_t0_dn9 = assign95840_e148436_d_n9;
        locals.var_t0_dn10 = assign95840_e148436_d_n10;
        locals.var_t0_dn11 = assign95840_e148436_d_n11;
        locals.var_t0_dn14 = assign95840_e148436_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95850_e148454, assign95850_e148454_d_n0, assign95850_e148454_d_n2, assign95850_e148454_d_n4, assign95850_e148454_d_n5, assign95850_e148454_d_n6, assign95850_e148454_d_n7, assign95850_e148454_d_n8, assign95850_e148454_d_n9, assign95850_e148454_d_n10, assign95850_e148454_d_n11, assign95850_e148454_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95850_e148442: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95850_e148445: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148446: f64 = (assign95850_e148442 - assign95850_e148445);
        let assign95850_e148449: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148450: f64 = (assign95850_e148446 - assign95850_e148449);
        let assign95850_e148452: f64 = (assign95850_e148450 + locals.var_tmf0);
        (assign95850_e148452, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95850_e148454;
        locals.var_psdl_dn0 = assign95850_e148454_d_n0;
        locals.var_psdl_dn2 = assign95850_e148454_d_n2;
        locals.var_psdl_dn4 = assign95850_e148454_d_n4;
        locals.var_psdl_dn5 = assign95850_e148454_d_n5;
        locals.var_psdl_dn6 = assign95850_e148454_d_n6;
        locals.var_psdl_dn7 = assign95850_e148454_d_n7;
        locals.var_psdl_dn8 = assign95850_e148454_d_n8;
        locals.var_psdl_dn9 = assign95850_e148454_d_n9;
        locals.var_psdl_dn10 = assign95850_e148454_d_n10;
        locals.var_psdl_dn11 = assign95850_e148454_d_n11;
        locals.var_psdl_dn14 = assign95850_e148454_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign95860_e148460, assign95860_e148460_d_n0, assign95860_e148460_d_n2, assign95860_e148460_d_n4, assign95860_e148460_d_n5, assign95860_e148460_d_n6, assign95860_e148460_d_n7, assign95860_e148460_d_n8, assign95860_e148460_d_n9, assign95860_e148460_d_n10, assign95860_e148460_d_n11, assign95860_e148460_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95860_e148460;
        locals.var_t0_dn0 = assign95860_e148460_d_n0;
        locals.var_t0_dn2 = assign95860_e148460_d_n2;
        locals.var_t0_dn4 = assign95860_e148460_d_n4;
        locals.var_t0_dn5 = assign95860_e148460_d_n5;
        locals.var_t0_dn6 = assign95860_e148460_d_n6;
        locals.var_t0_dn7 = assign95860_e148460_d_n7;
        locals.var_t0_dn8 = assign95860_e148460_d_n8;
        locals.var_t0_dn9 = assign95860_e148460_d_n9;
        locals.var_t0_dn10 = assign95860_e148460_d_n10;
        locals.var_t0_dn11 = assign95860_e148460_d_n11;
        locals.var_t0_dn14 = assign95860_e148460_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95870_e148467, assign95870_e148467_d_n0, assign95870_e148467_d_n2, assign95870_e148467_d_n4, assign95870_e148467_d_n5, assign95870_e148467_d_n6, assign95870_e148467_d_n7, assign95870_e148467_d_n8, assign95870_e148467_d_n9, assign95870_e148467_d_n10, assign95870_e148467_d_n11, assign95870_e148467_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95870_e148467;
        locals.var_psdl_dn0 = assign95870_e148467_d_n0;
        locals.var_psdl_dn2 = assign95870_e148467_d_n2;
        locals.var_psdl_dn4 = assign95870_e148467_d_n4;
        locals.var_psdl_dn5 = assign95870_e148467_d_n5;
        locals.var_psdl_dn6 = assign95870_e148467_d_n6;
        locals.var_psdl_dn7 = assign95870_e148467_d_n7;
        locals.var_psdl_dn8 = assign95870_e148467_d_n8;
        locals.var_psdl_dn9 = assign95870_e148467_d_n9;
        locals.var_psdl_dn10 = assign95870_e148467_d_n10;
        locals.var_psdl_dn11 = assign95870_e148467_d_n11;
        locals.var_psdl_dn14 = assign95870_e148467_d_n14;
        locals.var_psdl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_372(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95880_e148474, assign95880_e148474_d_n0, assign95880_e148474_d_n2, assign95880_e148474_d_n4, assign95880_e148474_d_n5, assign95880_e148474_d_n6, assign95880_e148474_d_n7, assign95880_e148474_d_n8, assign95880_e148474_d_n9, assign95880_e148474_d_n10, assign95880_e148474_d_n11, assign95880_e148474_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95880_e148474;
        locals.var_t0_dn0 = assign95880_e148474_d_n0;
        locals.var_t0_dn2 = assign95880_e148474_d_n2;
        locals.var_t0_dn4 = assign95880_e148474_d_n4;
        locals.var_t0_dn5 = assign95880_e148474_d_n5;
        locals.var_t0_dn6 = assign95880_e148474_d_n6;
        locals.var_t0_dn7 = assign95880_e148474_d_n7;
        locals.var_t0_dn8 = assign95880_e148474_d_n8;
        locals.var_t0_dn9 = assign95880_e148474_d_n9;
        locals.var_t0_dn10 = assign95880_e148474_d_n10;
        locals.var_t0_dn11 = assign95880_e148474_d_n11;
        locals.var_t0_dn14 = assign95880_e148474_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95890_e148480, assign95890_e148480_d_n0, assign95890_e148480_d_n2, assign95890_e148480_d_n4, assign95890_e148480_d_n5, assign95890_e148480_d_n6, assign95890_e148480_d_n7, assign95890_e148480_d_n8, assign95890_e148480_d_n9, assign95890_e148480_d_n10, assign95890_e148480_d_n11, assign95890_e148480_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95890_e148480;
        locals.var_ec_dn0 = assign95890_e148480_d_n0;
        locals.var_ec_dn2 = assign95890_e148480_d_n2;
        locals.var_ec_dn4 = assign95890_e148480_d_n4;
        locals.var_ec_dn5 = assign95890_e148480_d_n5;
        locals.var_ec_dn6 = assign95890_e148480_d_n6;
        locals.var_ec_dn7 = assign95890_e148480_d_n7;
        locals.var_ec_dn8 = assign95890_e148480_d_n8;
        locals.var_ec_dn9 = assign95890_e148480_d_n9;
        locals.var_ec_dn10 = assign95890_e148480_d_n10;
        locals.var_ec_dn11 = assign95890_e148480_d_n11;
        locals.var_ec_dn14 = assign95890_e148480_d_n14;
        locals.var_ec_rv = 0.0;

        let assign95900_e148487: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95900_e148487;
        locals.var_guard2231_rv = 0.0;

        let (assign95910_e148496, assign95910_e148496_d_n0, assign95910_e148496_d_n2, assign95910_e148496_d_n4, assign95910_e148496_d_n5, assign95910_e148496_d_n6, assign95910_e148496_d_n7, assign95910_e148496_d_n8, assign95910_e148496_d_n9, assign95910_e148496_d_n10, assign95910_e148496_d_n11, assign95910_e148496_d_n14,) = {
    if (((locals.var_guard2224 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2231 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95910_e148496;
        locals.var_ec_dn0 = assign95910_e148496_d_n0;
        locals.var_ec_dn2 = assign95910_e148496_d_n2;
        locals.var_ec_dn4 = assign95910_e148496_d_n4;
        locals.var_ec_dn5 = assign95910_e148496_d_n5;
        locals.var_ec_dn6 = assign95910_e148496_d_n6;
        locals.var_ec_dn7 = assign95910_e148496_d_n7;
        locals.var_ec_dn8 = assign95910_e148496_d_n8;
        locals.var_ec_dn9 = assign95910_e148496_d_n9;
        locals.var_ec_dn10 = assign95910_e148496_d_n10;
        locals.var_ec_dn11 = assign95910_e148496_d_n11;
        locals.var_ec_dn14 = assign95910_e148496_d_n14;
        locals.var_ec_rv = 0.0;

        let (assign95920_e148512, assign95920_e148512_d_n0, assign95920_e148512_d_n2, assign95920_e148512_d_n4, assign95920_e148512_d_n5, assign95920_e148512_d_n6, assign95920_e148512_d_n7, assign95920_e148512_d_n8, assign95920_e148512_d_n9, assign95920_e148512_d_n10, assign95920_e148512_d_n11, assign95920_e148512_d_n14,) = {
    if (((locals.var_guard2224 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2231 == 0.0)) {
        let assign95920_e148506: f64 = (locals.var_idd / locals.var_qn0);
        let assign95920_e148508: f64 = (assign95920_e148506 * locals.var_beta_inv);
        let assign95920_e148510: f64 = (assign95920_e148508 / locals.var_leff);
        (assign95920_e148510, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn11 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn11)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn11)) / locals.var_leff), ((((((locals.var_idd_dn14 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn14)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn14)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95920_e148512;
        locals.var_ec_dn0 = assign95920_e148512_d_n0;
        locals.var_ec_dn2 = assign95920_e148512_d_n2;
        locals.var_ec_dn4 = assign95920_e148512_d_n4;
        locals.var_ec_dn5 = assign95920_e148512_d_n5;
        locals.var_ec_dn6 = assign95920_e148512_d_n6;
        locals.var_ec_dn7 = assign95920_e148512_d_n7;
        locals.var_ec_dn8 = assign95920_e148512_d_n8;
        locals.var_ec_dn9 = assign95920_e148512_d_n9;
        locals.var_ec_dn10 = assign95920_e148512_d_n10;
        locals.var_ec_dn11 = assign95920_e148512_d_n11;
        locals.var_ec_dn14 = assign95920_e148512_d_n14;
        locals.var_ec_rv = 0.0;

        let assign95930_e148515: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95930_e148515;
        locals.var_guard2232_rv = 0.0;

        let (assign95940_e148519, assign95940_e148519_d_n0, assign95940_e148519_d_n2, assign95940_e148519_d_n4, assign95940_e148519_d_n5, assign95940_e148519_d_n6, assign95940_e148519_d_n7, assign95940_e148519_d_n8, assign95940_e148519_d_n9, assign95940_e148519_d_n10, assign95940_e148519_d_n11, assign95940_e148519_d_n14,) = {
    if (locals.var_guard2232 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95940_e148519;
        locals.var_qy_dn0 = assign95940_e148519_d_n0;
        locals.var_qy_dn2 = assign95940_e148519_d_n2;
        locals.var_qy_dn4 = assign95940_e148519_d_n4;
        locals.var_qy_dn5 = assign95940_e148519_d_n5;
        locals.var_qy_dn6 = assign95940_e148519_d_n6;
        locals.var_qy_dn7 = assign95940_e148519_d_n7;
        locals.var_qy_dn8 = assign95940_e148519_d_n8;
        locals.var_qy_dn9 = assign95940_e148519_d_n9;
        locals.var_qy_dn10 = assign95940_e148519_d_n10;
        locals.var_qy_dn11 = assign95940_e148519_d_n11;
        locals.var_qy_dn14 = assign95940_e148519_d_n14;
        locals.var_qy_rv = 0.0;

        let (assign95950_e148530, assign95950_e148530_d_n0, assign95950_e148530_d_n2, assign95950_e148530_d_n4, assign95950_e148530_d_n5, assign95950_e148530_d_n6, assign95950_e148530_d_n7, assign95950_e148530_d_n8, assign95950_e148530_d_n9, assign95950_e148530_d_n10, assign95950_e148530_d_n11, assign95950_e148530_d_n14,) = {
    if (locals.var_guard2232 == 0.0) {
        let assign95950_e148524: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95950_e148526: f64 = (assign95950_e148524 * locals.var_wdpl);
        let assign95950_e148528: f64 = (assign95950_e148526 * 1.3);
        (assign95950_e148528, ((assign95950_e148524 * locals.var_wdpl_dn0) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn2) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn4) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn5) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn6) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn7) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn8) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn9) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn10) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn11) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn14) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign95950_e148530;
        locals.var_t2_dn0 = assign95950_e148530_d_n0;
        locals.var_t2_dn2 = assign95950_e148530_d_n2;
        locals.var_t2_dn4 = assign95950_e148530_d_n4;
        locals.var_t2_dn5 = assign95950_e148530_d_n5;
        locals.var_t2_dn6 = assign95950_e148530_d_n6;
        locals.var_t2_dn7 = assign95950_e148530_d_n7;
        locals.var_t2_dn8 = assign95950_e148530_d_n8;
        locals.var_t2_dn9 = assign95950_e148530_d_n9;
        locals.var_t2_dn10 = assign95950_e148530_d_n10;
        locals.var_t2_dn11 = assign95950_e148530_d_n11;
        locals.var_t2_dn14 = assign95950_e148530_d_n14;
        locals.var_t2_rv = 0.0;

        let assign95960_e148533: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign95960_e148533;
        locals.var_guard2233_rv = 0.0;

        let (assign95970_e148544, assign95970_e148544_d_n0, assign95970_e148544_d_n2, assign95970_e148544_d_n4, assign95970_e148544_d_n5, assign95970_e148544_d_n6, assign95970_e148544_d_n7, assign95970_e148544_d_n8, assign95970_e148544_d_n9, assign95970_e148544_d_n10, assign95970_e148544_d_n11, assign95970_e148544_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95970_e148540: f64 = (locals.var_ec * locals.var_leff);
        let assign95970_e148542: f64 = (assign95970_e148540 + locals.var_ps0);
        (assign95970_e148542, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn14 * locals.var_leff) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn14,)
    }
};
        locals.var_pslk = assign95970_e148544;
        locals.var_pslk_dn0 = assign95970_e148544_d_n0;
        locals.var_pslk_dn2 = assign95970_e148544_d_n2;
        locals.var_pslk_dn4 = assign95970_e148544_d_n4;
        locals.var_pslk_dn5 = assign95970_e148544_d_n5;
        locals.var_pslk_dn6 = assign95970_e148544_d_n6;
        locals.var_pslk_dn7 = assign95970_e148544_d_n7;
        locals.var_pslk_dn8 = assign95970_e148544_d_n8;
        locals.var_pslk_dn9 = assign95970_e148544_d_n9;
        locals.var_pslk_dn10 = assign95970_e148544_d_n10;
        locals.var_pslk_dn11 = assign95970_e148544_d_n11;
        locals.var_pslk_dn14 = assign95970_e148544_d_n14;
        locals.var_pslk_rv = 0.0;

        let (assign95980_e148561, assign95980_e148561_d_n0, assign95980_e148561_d_n2, assign95980_e148561_d_n4, assign95980_e148561_d_n5, assign95980_e148561_d_n6, assign95980_e148561_d_n7, assign95980_e148561_d_n8, assign95980_e148561_d_n9, assign95980_e148561_d_n10, assign95980_e148561_d_n11, assign95980_e148561_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95980_e148552: f64 = (locals.var_vdsz__blk441 + locals.var_ps0);
        let assign95980_e148553: f64 = (locals.var_aclm * assign95980_e148552);
        let assign95980_e148556: f64 = (1.0 - locals.var_aclm);
        let assign95980_e148558: f64 = (assign95980_e148556 * locals.var_pslk);
        let assign95980_e148559: f64 = (assign95980_e148553 + assign95980_e148558);
        (assign95980_e148559, ((locals.var_aclm * (locals.var_vdsz__blk441_dn0 + locals.var_ps0_dn0)) + (assign95980_e148556 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn2 + locals.var_ps0_dn2)) + (assign95980_e148556 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn4 + locals.var_ps0_dn4)) + (assign95980_e148556 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn5 + locals.var_ps0_dn5)) + (assign95980_e148556 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn6 + locals.var_ps0_dn6)) + (assign95980_e148556 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn7 + locals.var_ps0_dn7)) + (assign95980_e148556 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn8 + locals.var_ps0_dn8)) + (assign95980_e148556 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn9 + locals.var_ps0_dn9)) + (assign95980_e148556 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn10 + locals.var_ps0_dn10)) + (assign95980_e148556 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn11 + locals.var_ps0_dn11)) + (assign95980_e148556 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn14 + locals.var_ps0_dn14)) + (assign95980_e148556 * locals.var_pslk_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95980_e148561;
        locals.var_t1_dn0 = assign95980_e148561_d_n0;
        locals.var_t1_dn2 = assign95980_e148561_d_n2;
        locals.var_t1_dn4 = assign95980_e148561_d_n4;
        locals.var_t1_dn5 = assign95980_e148561_d_n5;
        locals.var_t1_dn6 = assign95980_e148561_d_n6;
        locals.var_t1_dn7 = assign95980_e148561_d_n7;
        locals.var_t1_dn8 = assign95980_e148561_d_n8;
        locals.var_t1_dn9 = assign95980_e148561_d_n9;
        locals.var_t1_dn10 = assign95980_e148561_d_n10;
        locals.var_t1_dn11 = assign95980_e148561_d_n11;
        locals.var_t1_dn14 = assign95980_e148561_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign95990_e148577, assign95990_e148577_d_n0, assign95990_e148577_d_n2, assign95990_e148577_d_n4, assign95990_e148577_d_n5, assign95990_e148577_d_n6, assign95990_e148577_d_n7, assign95990_e148577_d_n8, assign95990_e148577_d_n9, assign95990_e148577_d_n10, assign95990_e148577_d_n11, assign95990_e148577_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95990_e148568: f64 = (locals.var_ps0 + locals.var_vdsz__blk441);
        let assign95990_e148570: f64 = (assign95990_e148568 - locals.var_t1);
        let assign95990_e148572: f64 = (assign95990_e148570 / p.p133);
        let assign95990_e148573: f64 = (-assign95990_e148572);
        let assign95990_e148575: f64 = (assign95990_e148573 * locals.var_t2);
        (assign95990_e148575, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk441_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk441_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk441_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk441_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk441_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk441_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk441_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk441_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk441_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn11 + locals.var_vdsz__blk441_dn11) - locals.var_t1_dn11) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn11)), (((-(((locals.var_ps0_dn14 + locals.var_vdsz__blk441_dn14) - locals.var_t1_dn14) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95990_e148577;
        locals.var_qy_dn0 = assign95990_e148577_d_n0;
        locals.var_qy_dn2 = assign95990_e148577_d_n2;
        locals.var_qy_dn4 = assign95990_e148577_d_n4;
        locals.var_qy_dn5 = assign95990_e148577_d_n5;
        locals.var_qy_dn6 = assign95990_e148577_d_n6;
        locals.var_qy_dn7 = assign95990_e148577_d_n7;
        locals.var_qy_dn8 = assign95990_e148577_d_n8;
        locals.var_qy_dn9 = assign95990_e148577_d_n9;
        locals.var_qy_dn10 = assign95990_e148577_d_n10;
        locals.var_qy_dn11 = assign95990_e148577_d_n11;
        locals.var_qy_dn14 = assign95990_e148577_d_n14;
        locals.var_qy_rv = 0.0;

        let assign96000_e148580: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign96000_e148580;
        locals.var_guard2234_rv = 0.0;

        let (assign96010_e148591, assign96010_e148591_d_n0, assign96010_e148591_d_n2, assign96010_e148591_d_n4, assign96010_e148591_d_n5, assign96010_e148591_d_n6, assign96010_e148591_d_n7, assign96010_e148591_d_n8, assign96010_e148591_d_n9, assign96010_e148591_d_n10, assign96010_e148591_d_n11, assign96010_e148591_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2234 != 0.0)) {
        let assign96010_e148588: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign96010_e148589: f64 = (locals.var_qy + assign96010_e148588);
        (assign96010_e148589, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbs_dn6)), locals.var_qy_dn7, (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), (locals.var_qy_dn9 + (locals.var_cqyb0 * locals.var_vbs_dn9)), locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96010_e148591;
        locals.var_qy_dn0 = assign96010_e148591_d_n0;
        locals.var_qy_dn2 = assign96010_e148591_d_n2;
        locals.var_qy_dn4 = assign96010_e148591_d_n4;
        locals.var_qy_dn5 = assign96010_e148591_d_n5;
        locals.var_qy_dn6 = assign96010_e148591_d_n6;
        locals.var_qy_dn7 = assign96010_e148591_d_n7;
        locals.var_qy_dn8 = assign96010_e148591_d_n8;
        locals.var_qy_dn9 = assign96010_e148591_d_n9;
        locals.var_qy_dn10 = assign96010_e148591_d_n10;
        locals.var_qy_dn11 = assign96010_e148591_d_n11;
        locals.var_qy_dn14 = assign96010_e148591_d_n14;
        locals.var_qy_rv = 0.0;

        locals.var_cfd = locals.var_cfrng;
        locals.var_cfd_rv = 0.0;

        locals.var_cfs = locals.var_cfrng;
        locals.var_cfs_rv = 0.0;

        let assign96040_e148597: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96040_e148598: f64 = (locals.var_cfd * assign96040_e148597);
        locals.var_qfd = assign96040_e148598;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn7 = (locals.var_cfd * locals.var_vgsei_dn7);
        locals.var_qfd_rv = 0.0;

        let assign96050_e148601: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96050_e148601;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn7 = (locals.var_cfs * locals.var_vgsei_dn7);
        locals.var_qfs_rv = 0.0;

        let assign96060_e148608: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign96060_e148608;
        locals.var_guard2235_rv = 0.0;

        let (assign96070_e148614, assign96070_e148614_d_n0, assign96070_e148614_d_n2, assign96070_e148614_d_n4, assign96070_e148614_d_n5, assign96070_e148614_d_n6, assign96070_e148614_d_n7, assign96070_e148614_d_n8, assign96070_e148614_d_n9, assign96070_e148614_d_n10, assign96070_e148614_d_n11, assign96070_e148614_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96070_e148612: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96070_e148612, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign96070_e148614;
        locals.var_t0_dn0 = assign96070_e148614_d_n0;
        locals.var_t0_dn2 = assign96070_e148614_d_n2;
        locals.var_t0_dn4 = assign96070_e148614_d_n4;
        locals.var_t0_dn5 = assign96070_e148614_d_n5;
        locals.var_t0_dn6 = assign96070_e148614_d_n6;
        locals.var_t0_dn7 = assign96070_e148614_d_n7;
        locals.var_t0_dn8 = assign96070_e148614_d_n8;
        locals.var_t0_dn9 = assign96070_e148614_d_n9;
        locals.var_t0_dn10 = assign96070_e148614_d_n10;
        locals.var_t0_dn11 = assign96070_e148614_d_n11;
        locals.var_t0_dn14 = assign96070_e148614_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign96080_e148633, assign96080_e148633_d_n0, assign96080_e148633_d_n2, assign96080_e148633_d_n4, assign96080_e148633_d_n5, assign96080_e148633_d_n6, assign96080_e148633_d_n7, assign96080_e148633_d_n8, assign96080_e148633_d_n9, assign96080_e148633_d_n10, assign96080_e148633_d_n11, assign96080_e148633_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96080_e148619: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96080_e148622: f64 = (locals.var_eg * locals.var_beta);
        let assign96080_e148623: f64 = (assign96080_e148619 - assign96080_e148622);
        let assign96080_e148626: f64 = (p.p499 * locals.var_log_tratio);
        let assign96080_e148627: f64 = (assign96080_e148623 + assign96080_e148626);
        let assign96080_e148629: f64 = (assign96080_e148627 / locals.var_uc_njd);
        let assign96080_e148630: f64 = (assign96080_e148629).exp();
        let assign96080_e148631: f64 = (locals.var_uc_js0d * assign96080_e148630);
        (assign96080_e148631, (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96080_e148633;
        locals.var_js_dn0 = assign96080_e148633_d_n0;
        locals.var_js_dn2 = assign96080_e148633_d_n2;
        locals.var_js_dn4 = assign96080_e148633_d_n4;
        locals.var_js_dn5 = assign96080_e148633_d_n5;
        locals.var_js_dn6 = assign96080_e148633_d_n6;
        locals.var_js_dn7 = assign96080_e148633_d_n7;
        locals.var_js_dn8 = assign96080_e148633_d_n8;
        locals.var_js_dn9 = assign96080_e148633_d_n9;
        locals.var_js_dn10 = assign96080_e148633_d_n10;
        locals.var_js_dn11 = assign96080_e148633_d_n11;
        locals.var_js_dn14 = assign96080_e148633_d_n14;
        locals.var_js_rv = 0.0;

        let (assign96090_e148652, assign96090_e148652_d_n0, assign96090_e148652_d_n2, assign96090_e148652_d_n4, assign96090_e148652_d_n5, assign96090_e148652_d_n6, assign96090_e148652_d_n7, assign96090_e148652_d_n8, assign96090_e148652_d_n9, assign96090_e148652_d_n10, assign96090_e148652_d_n11, assign96090_e148652_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96090_e148638: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96090_e148641: f64 = (locals.var_eg * locals.var_beta);
        let assign96090_e148642: f64 = (assign96090_e148638 - assign96090_e148641);
        let assign96090_e148645: f64 = (p.p499 * locals.var_log_tratio);
        let assign96090_e148646: f64 = (assign96090_e148642 + assign96090_e148645);
        let assign96090_e148648: f64 = (assign96090_e148646 / p.p497);
        let assign96090_e148649: f64 = (assign96090_e148648).exp();
        let assign96090_e148650: f64 = (locals.var_uc_js0swd * assign96090_e148649);
        (assign96090_e148650, (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96090_e148652;
        locals.var_jssw_dn0 = assign96090_e148652_d_n0;
        locals.var_jssw_dn2 = assign96090_e148652_d_n2;
        locals.var_jssw_dn4 = assign96090_e148652_d_n4;
        locals.var_jssw_dn5 = assign96090_e148652_d_n5;
        locals.var_jssw_dn6 = assign96090_e148652_d_n6;
        locals.var_jssw_dn7 = assign96090_e148652_d_n7;
        locals.var_jssw_dn8 = assign96090_e148652_d_n8;
        locals.var_jssw_dn9 = assign96090_e148652_d_n9;
        locals.var_jssw_dn10 = assign96090_e148652_d_n10;
        locals.var_jssw_dn11 = assign96090_e148652_d_n11;
        locals.var_jssw_dn14 = assign96090_e148652_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign96100_e148671, assign96100_e148671_d_n0, assign96100_e148671_d_n2, assign96100_e148671_d_n4, assign96100_e148671_d_n5, assign96100_e148671_d_n6, assign96100_e148671_d_n7, assign96100_e148671_d_n8, assign96100_e148671_d_n9, assign96100_e148671_d_n10, assign96100_e148671_d_n11, assign96100_e148671_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96100_e148657: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148660: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148661: f64 = (assign96100_e148657 - assign96100_e148660);
        let assign96100_e148664: f64 = (p.p499 * locals.var_log_tratio);
        let assign96100_e148665: f64 = (assign96100_e148661 + assign96100_e148664);
        let assign96100_e148667: f64 = (assign96100_e148665 / p.p498);
        let assign96100_e148668: f64 = (assign96100_e148667).exp();
        let assign96100_e148669: f64 = (p.p495 * assign96100_e148668);
        (assign96100_e148669, (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96100_e148671;
        locals.var_jsswg_dn0 = assign96100_e148671_d_n0;
        locals.var_jsswg_dn2 = assign96100_e148671_d_n2;
        locals.var_jsswg_dn4 = assign96100_e148671_d_n4;
        locals.var_jsswg_dn5 = assign96100_e148671_d_n5;
        locals.var_jsswg_dn6 = assign96100_e148671_d_n6;
        locals.var_jsswg_dn7 = assign96100_e148671_d_n7;
        locals.var_jsswg_dn8 = assign96100_e148671_d_n8;
        locals.var_jsswg_dn9 = assign96100_e148671_d_n9;
        locals.var_jsswg_dn10 = assign96100_e148671_d_n10;
        locals.var_jsswg_dn11 = assign96100_e148671_d_n11;
        locals.var_jsswg_dn14 = assign96100_e148671_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign96110_e148690, assign96110_e148690_d_n0, assign96110_e148690_d_n2, assign96110_e148690_d_n4, assign96110_e148690_d_n5, assign96110_e148690_d_n6, assign96110_e148690_d_n7, assign96110_e148690_d_n8, assign96110_e148690_d_n9, assign96110_e148690_d_n10, assign96110_e148690_d_n11, assign96110_e148690_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96110_e148676: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148679: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148680: f64 = (assign96110_e148676 - assign96110_e148679);
        let assign96110_e148683: f64 = (p.p509 * locals.var_log_tratio);
        let assign96110_e148684: f64 = (assign96110_e148680 + assign96110_e148683);
        let assign96110_e148686: f64 = (assign96110_e148684 / locals.var_uc_njd);
        let assign96110_e148687: f64 = (assign96110_e148686).exp();
        let assign96110_e148688: f64 = (locals.var_uc_js0d * assign96110_e148687);
        (assign96110_e148688, (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96110_e148690;
        locals.var_js2_dn0 = assign96110_e148690_d_n0;
        locals.var_js2_dn2 = assign96110_e148690_d_n2;
        locals.var_js2_dn4 = assign96110_e148690_d_n4;
        locals.var_js2_dn5 = assign96110_e148690_d_n5;
        locals.var_js2_dn6 = assign96110_e148690_d_n6;
        locals.var_js2_dn7 = assign96110_e148690_d_n7;
        locals.var_js2_dn8 = assign96110_e148690_d_n8;
        locals.var_js2_dn9 = assign96110_e148690_d_n9;
        locals.var_js2_dn10 = assign96110_e148690_d_n10;
        locals.var_js2_dn11 = assign96110_e148690_d_n11;
        locals.var_js2_dn14 = assign96110_e148690_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign96120_e148709, assign96120_e148709_d_n0, assign96120_e148709_d_n2, assign96120_e148709_d_n4, assign96120_e148709_d_n5, assign96120_e148709_d_n6, assign96120_e148709_d_n7, assign96120_e148709_d_n8, assign96120_e148709_d_n9, assign96120_e148709_d_n10, assign96120_e148709_d_n11, assign96120_e148709_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96120_e148695: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96120_e148698: f64 = (locals.var_eg * locals.var_beta);
        let assign96120_e148699: f64 = (assign96120_e148695 - assign96120_e148698);
        let assign96120_e148702: f64 = (p.p509 * locals.var_log_tratio);
        let assign96120_e148703: f64 = (assign96120_e148699 + assign96120_e148702);
        let assign96120_e148705: f64 = (assign96120_e148703 / p.p497);
        let assign96120_e148706: f64 = (assign96120_e148705).exp();
        let assign96120_e148707: f64 = (locals.var_uc_js0swd * assign96120_e148706);
        (assign96120_e148707, (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96120_e148709;
        locals.var_jssw2_dn0 = assign96120_e148709_d_n0;
        locals.var_jssw2_dn2 = assign96120_e148709_d_n2;
        locals.var_jssw2_dn4 = assign96120_e148709_d_n4;
        locals.var_jssw2_dn5 = assign96120_e148709_d_n5;
        locals.var_jssw2_dn6 = assign96120_e148709_d_n6;
        locals.var_jssw2_dn7 = assign96120_e148709_d_n7;
        locals.var_jssw2_dn8 = assign96120_e148709_d_n8;
        locals.var_jssw2_dn9 = assign96120_e148709_d_n9;
        locals.var_jssw2_dn10 = assign96120_e148709_d_n10;
        locals.var_jssw2_dn11 = assign96120_e148709_d_n11;
        locals.var_jssw2_dn14 = assign96120_e148709_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign96130_e148728, assign96130_e148728_d_n0, assign96130_e148728_d_n2, assign96130_e148728_d_n4, assign96130_e148728_d_n5, assign96130_e148728_d_n6, assign96130_e148728_d_n7, assign96130_e148728_d_n8, assign96130_e148728_d_n9, assign96130_e148728_d_n10, assign96130_e148728_d_n11, assign96130_e148728_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96130_e148714: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96130_e148717: f64 = (locals.var_eg * locals.var_beta);
        let assign96130_e148718: f64 = (assign96130_e148714 - assign96130_e148717);
        let assign96130_e148721: f64 = (p.p509 * locals.var_log_tratio);
        let assign96130_e148722: f64 = (assign96130_e148718 + assign96130_e148721);
        let assign96130_e148724: f64 = (assign96130_e148722 / p.p498);
        let assign96130_e148725: f64 = (assign96130_e148724).exp();
        let assign96130_e148726: f64 = (p.p495 * assign96130_e148725);
        (assign96130_e148726, (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96130_e148728;
        locals.var_jsswg2_dn0 = assign96130_e148728_d_n0;
        locals.var_jsswg2_dn2 = assign96130_e148728_d_n2;
        locals.var_jsswg2_dn4 = assign96130_e148728_d_n4;
        locals.var_jsswg2_dn5 = assign96130_e148728_d_n5;
        locals.var_jsswg2_dn6 = assign96130_e148728_d_n6;
        locals.var_jsswg2_dn7 = assign96130_e148728_d_n7;
        locals.var_jsswg2_dn8 = assign96130_e148728_d_n8;
        locals.var_jsswg2_dn9 = assign96130_e148728_d_n9;
        locals.var_jsswg2_dn10 = assign96130_e148728_d_n10;
        locals.var_jsswg2_dn11 = assign96130_e148728_d_n11;
        locals.var_jsswg2_dn14 = assign96130_e148728_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign96140_e148731: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96140_e148731;
        locals.var_guard2236_rv = 0.0;

        let assign96150_e148734: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96150_e148734;
        locals.var_guard2237_rv = 0.0;

        let (assign96160_e148744, assign96160_e148744_d_n0, assign96160_e148744_d_n2, assign96160_e148744_d_n4, assign96160_e148744_d_n5, assign96160_e148744_d_n6, assign96160_e148744_d_n7, assign96160_e148744_d_n8, assign96160_e148744_d_n9, assign96160_e148744_d_n10, assign96160_e148744_d_n11, assign96160_e148744_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96160_e148742: f64 = (p.p13 * locals.var_js);
        (assign96160_e148742, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96160_e148744;
        locals.var_isbd_btm_dn0 = assign96160_e148744_d_n0;
        locals.var_isbd_btm_dn2 = assign96160_e148744_d_n2;
        locals.var_isbd_btm_dn4 = assign96160_e148744_d_n4;
        locals.var_isbd_btm_dn5 = assign96160_e148744_d_n5;
        locals.var_isbd_btm_dn6 = assign96160_e148744_d_n6;
        locals.var_isbd_btm_dn7 = assign96160_e148744_d_n7;
        locals.var_isbd_btm_dn8 = assign96160_e148744_d_n8;
        locals.var_isbd_btm_dn9 = assign96160_e148744_d_n9;
        locals.var_isbd_btm_dn10 = assign96160_e148744_d_n10;
        locals.var_isbd_btm_dn11 = assign96160_e148744_d_n11;
        locals.var_isbd_btm_dn14 = assign96160_e148744_d_n14;
        locals.var_isbd_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_373(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96170_e148754, assign96170_e148754_d_n0, assign96170_e148754_d_n2, assign96170_e148754_d_n4, assign96170_e148754_d_n5, assign96170_e148754_d_n6, assign96170_e148754_d_n7, assign96170_e148754_d_n8, assign96170_e148754_d_n9, assign96170_e148754_d_n10, assign96170_e148754_d_n11, assign96170_e148754_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96170_e148752: f64 = (p.p13 * locals.var_js2);
        (assign96170_e148752, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96170_e148754;
        locals.var_isbd2_btm_dn0 = assign96170_e148754_d_n0;
        locals.var_isbd2_btm_dn2 = assign96170_e148754_d_n2;
        locals.var_isbd2_btm_dn4 = assign96170_e148754_d_n4;
        locals.var_isbd2_btm_dn5 = assign96170_e148754_d_n5;
        locals.var_isbd2_btm_dn6 = assign96170_e148754_d_n6;
        locals.var_isbd2_btm_dn7 = assign96170_e148754_d_n7;
        locals.var_isbd2_btm_dn8 = assign96170_e148754_d_n8;
        locals.var_isbd2_btm_dn9 = assign96170_e148754_d_n9;
        locals.var_isbd2_btm_dn10 = assign96170_e148754_d_n10;
        locals.var_isbd2_btm_dn11 = assign96170_e148754_d_n11;
        locals.var_isbd2_btm_dn14 = assign96170_e148754_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96180_e148766, assign96180_e148766_d_n0, assign96180_e148766_d_n2, assign96180_e148766_d_n4, assign96180_e148766_d_n5, assign96180_e148766_d_n6, assign96180_e148766_d_n7, assign96180_e148766_d_n8, assign96180_e148766_d_n9, assign96180_e148766_d_n10, assign96180_e148766_d_n11, assign96180_e148766_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96180_e148762: f64 = (p.p15 - locals.var_weff_nf);
        let assign96180_e148764: f64 = (assign96180_e148762 * locals.var_jssw);
        (assign96180_e148764, (assign96180_e148762 * locals.var_jssw_dn0), (assign96180_e148762 * locals.var_jssw_dn2), (assign96180_e148762 * locals.var_jssw_dn4), (assign96180_e148762 * locals.var_jssw_dn5), (assign96180_e148762 * locals.var_jssw_dn6), (assign96180_e148762 * locals.var_jssw_dn7), (assign96180_e148762 * locals.var_jssw_dn8), (assign96180_e148762 * locals.var_jssw_dn9), (assign96180_e148762 * locals.var_jssw_dn10), (assign96180_e148762 * locals.var_jssw_dn11), (assign96180_e148762 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96180_e148766;
        locals.var_isbd_sws_dn0 = assign96180_e148766_d_n0;
        locals.var_isbd_sws_dn2 = assign96180_e148766_d_n2;
        locals.var_isbd_sws_dn4 = assign96180_e148766_d_n4;
        locals.var_isbd_sws_dn5 = assign96180_e148766_d_n5;
        locals.var_isbd_sws_dn6 = assign96180_e148766_d_n6;
        locals.var_isbd_sws_dn7 = assign96180_e148766_d_n7;
        locals.var_isbd_sws_dn8 = assign96180_e148766_d_n8;
        locals.var_isbd_sws_dn9 = assign96180_e148766_d_n9;
        locals.var_isbd_sws_dn10 = assign96180_e148766_d_n10;
        locals.var_isbd_sws_dn11 = assign96180_e148766_d_n11;
        locals.var_isbd_sws_dn14 = assign96180_e148766_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96190_e148778, assign96190_e148778_d_n0, assign96190_e148778_d_n2, assign96190_e148778_d_n4, assign96190_e148778_d_n5, assign96190_e148778_d_n6, assign96190_e148778_d_n7, assign96190_e148778_d_n8, assign96190_e148778_d_n9, assign96190_e148778_d_n10, assign96190_e148778_d_n11, assign96190_e148778_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96190_e148774: f64 = (p.p15 - locals.var_weff_nf);
        let assign96190_e148776: f64 = (assign96190_e148774 * locals.var_jssw2);
        (assign96190_e148776, (assign96190_e148774 * locals.var_jssw2_dn0), (assign96190_e148774 * locals.var_jssw2_dn2), (assign96190_e148774 * locals.var_jssw2_dn4), (assign96190_e148774 * locals.var_jssw2_dn5), (assign96190_e148774 * locals.var_jssw2_dn6), (assign96190_e148774 * locals.var_jssw2_dn7), (assign96190_e148774 * locals.var_jssw2_dn8), (assign96190_e148774 * locals.var_jssw2_dn9), (assign96190_e148774 * locals.var_jssw2_dn10), (assign96190_e148774 * locals.var_jssw2_dn11), (assign96190_e148774 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96190_e148778;
        locals.var_isbd2_sws_dn0 = assign96190_e148778_d_n0;
        locals.var_isbd2_sws_dn2 = assign96190_e148778_d_n2;
        locals.var_isbd2_sws_dn4 = assign96190_e148778_d_n4;
        locals.var_isbd2_sws_dn5 = assign96190_e148778_d_n5;
        locals.var_isbd2_sws_dn6 = assign96190_e148778_d_n6;
        locals.var_isbd2_sws_dn7 = assign96190_e148778_d_n7;
        locals.var_isbd2_sws_dn8 = assign96190_e148778_d_n8;
        locals.var_isbd2_sws_dn9 = assign96190_e148778_d_n9;
        locals.var_isbd2_sws_dn10 = assign96190_e148778_d_n10;
        locals.var_isbd2_sws_dn11 = assign96190_e148778_d_n11;
        locals.var_isbd2_sws_dn14 = assign96190_e148778_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96200_e148788, assign96200_e148788_d_n0, assign96200_e148788_d_n2, assign96200_e148788_d_n4, assign96200_e148788_d_n5, assign96200_e148788_d_n6, assign96200_e148788_d_n7, assign96200_e148788_d_n8, assign96200_e148788_d_n9, assign96200_e148788_d_n10, assign96200_e148788_d_n11, assign96200_e148788_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96200_e148786: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96200_e148786, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96200_e148788;
        locals.var_isbd_swg_dn0 = assign96200_e148788_d_n0;
        locals.var_isbd_swg_dn2 = assign96200_e148788_d_n2;
        locals.var_isbd_swg_dn4 = assign96200_e148788_d_n4;
        locals.var_isbd_swg_dn5 = assign96200_e148788_d_n5;
        locals.var_isbd_swg_dn6 = assign96200_e148788_d_n6;
        locals.var_isbd_swg_dn7 = assign96200_e148788_d_n7;
        locals.var_isbd_swg_dn8 = assign96200_e148788_d_n8;
        locals.var_isbd_swg_dn9 = assign96200_e148788_d_n9;
        locals.var_isbd_swg_dn10 = assign96200_e148788_d_n10;
        locals.var_isbd_swg_dn11 = assign96200_e148788_d_n11;
        locals.var_isbd_swg_dn14 = assign96200_e148788_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96210_e148798, assign96210_e148798_d_n0, assign96210_e148798_d_n2, assign96210_e148798_d_n4, assign96210_e148798_d_n5, assign96210_e148798_d_n6, assign96210_e148798_d_n7, assign96210_e148798_d_n8, assign96210_e148798_d_n9, assign96210_e148798_d_n10, assign96210_e148798_d_n11, assign96210_e148798_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96210_e148796: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96210_e148796, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96210_e148798;
        locals.var_isbd2_swg_dn0 = assign96210_e148798_d_n0;
        locals.var_isbd2_swg_dn2 = assign96210_e148798_d_n2;
        locals.var_isbd2_swg_dn4 = assign96210_e148798_d_n4;
        locals.var_isbd2_swg_dn5 = assign96210_e148798_d_n5;
        locals.var_isbd2_swg_dn6 = assign96210_e148798_d_n6;
        locals.var_isbd2_swg_dn7 = assign96210_e148798_d_n7;
        locals.var_isbd2_swg_dn8 = assign96210_e148798_d_n8;
        locals.var_isbd2_swg_dn9 = assign96210_e148798_d_n9;
        locals.var_isbd2_swg_dn10 = assign96210_e148798_d_n10;
        locals.var_isbd2_swg_dn11 = assign96210_e148798_d_n11;
        locals.var_isbd2_swg_dn14 = assign96210_e148798_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96220_e148809, assign96220_e148809_d_n0, assign96220_e148809_d_n2, assign96220_e148809_d_n4, assign96220_e148809_d_n5, assign96220_e148809_d_n6, assign96220_e148809_d_n7, assign96220_e148809_d_n8, assign96220_e148809_d_n9, assign96220_e148809_d_n10, assign96220_e148809_d_n11, assign96220_e148809_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96220_e148807: f64 = (p.p13 * locals.var_js);
        (assign96220_e148807, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96220_e148809;
        locals.var_isbd_btm_dn0 = assign96220_e148809_d_n0;
        locals.var_isbd_btm_dn2 = assign96220_e148809_d_n2;
        locals.var_isbd_btm_dn4 = assign96220_e148809_d_n4;
        locals.var_isbd_btm_dn5 = assign96220_e148809_d_n5;
        locals.var_isbd_btm_dn6 = assign96220_e148809_d_n6;
        locals.var_isbd_btm_dn7 = assign96220_e148809_d_n7;
        locals.var_isbd_btm_dn8 = assign96220_e148809_d_n8;
        locals.var_isbd_btm_dn9 = assign96220_e148809_d_n9;
        locals.var_isbd_btm_dn10 = assign96220_e148809_d_n10;
        locals.var_isbd_btm_dn11 = assign96220_e148809_d_n11;
        locals.var_isbd_btm_dn14 = assign96220_e148809_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96230_e148820, assign96230_e148820_d_n0, assign96230_e148820_d_n2, assign96230_e148820_d_n4, assign96230_e148820_d_n5, assign96230_e148820_d_n6, assign96230_e148820_d_n7, assign96230_e148820_d_n8, assign96230_e148820_d_n9, assign96230_e148820_d_n10, assign96230_e148820_d_n11, assign96230_e148820_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96230_e148818: f64 = (p.p13 * locals.var_js2);
        (assign96230_e148818, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96230_e148820;
        locals.var_isbd2_btm_dn0 = assign96230_e148820_d_n0;
        locals.var_isbd2_btm_dn2 = assign96230_e148820_d_n2;
        locals.var_isbd2_btm_dn4 = assign96230_e148820_d_n4;
        locals.var_isbd2_btm_dn5 = assign96230_e148820_d_n5;
        locals.var_isbd2_btm_dn6 = assign96230_e148820_d_n6;
        locals.var_isbd2_btm_dn7 = assign96230_e148820_d_n7;
        locals.var_isbd2_btm_dn8 = assign96230_e148820_d_n8;
        locals.var_isbd2_btm_dn9 = assign96230_e148820_d_n9;
        locals.var_isbd2_btm_dn10 = assign96230_e148820_d_n10;
        locals.var_isbd2_btm_dn11 = assign96230_e148820_d_n11;
        locals.var_isbd2_btm_dn14 = assign96230_e148820_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96240_e148829, assign96240_e148829_d_n0, assign96240_e148829_d_n2, assign96240_e148829_d_n4, assign96240_e148829_d_n5, assign96240_e148829_d_n6, assign96240_e148829_d_n7, assign96240_e148829_d_n8, assign96240_e148829_d_n9, assign96240_e148829_d_n10, assign96240_e148829_d_n11, assign96240_e148829_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96240_e148829;
        locals.var_isbd_sws_dn0 = assign96240_e148829_d_n0;
        locals.var_isbd_sws_dn2 = assign96240_e148829_d_n2;
        locals.var_isbd_sws_dn4 = assign96240_e148829_d_n4;
        locals.var_isbd_sws_dn5 = assign96240_e148829_d_n5;
        locals.var_isbd_sws_dn6 = assign96240_e148829_d_n6;
        locals.var_isbd_sws_dn7 = assign96240_e148829_d_n7;
        locals.var_isbd_sws_dn8 = assign96240_e148829_d_n8;
        locals.var_isbd_sws_dn9 = assign96240_e148829_d_n9;
        locals.var_isbd_sws_dn10 = assign96240_e148829_d_n10;
        locals.var_isbd_sws_dn11 = assign96240_e148829_d_n11;
        locals.var_isbd_sws_dn14 = assign96240_e148829_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96250_e148838, assign96250_e148838_d_n0, assign96250_e148838_d_n2, assign96250_e148838_d_n4, assign96250_e148838_d_n5, assign96250_e148838_d_n6, assign96250_e148838_d_n7, assign96250_e148838_d_n8, assign96250_e148838_d_n9, assign96250_e148838_d_n10, assign96250_e148838_d_n11, assign96250_e148838_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96250_e148838;
        locals.var_isbd2_sws_dn0 = assign96250_e148838_d_n0;
        locals.var_isbd2_sws_dn2 = assign96250_e148838_d_n2;
        locals.var_isbd2_sws_dn4 = assign96250_e148838_d_n4;
        locals.var_isbd2_sws_dn5 = assign96250_e148838_d_n5;
        locals.var_isbd2_sws_dn6 = assign96250_e148838_d_n6;
        locals.var_isbd2_sws_dn7 = assign96250_e148838_d_n7;
        locals.var_isbd2_sws_dn8 = assign96250_e148838_d_n8;
        locals.var_isbd2_sws_dn9 = assign96250_e148838_d_n9;
        locals.var_isbd2_sws_dn10 = assign96250_e148838_d_n10;
        locals.var_isbd2_sws_dn11 = assign96250_e148838_d_n11;
        locals.var_isbd2_sws_dn14 = assign96250_e148838_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96260_e148849, assign96260_e148849_d_n0, assign96260_e148849_d_n2, assign96260_e148849_d_n4, assign96260_e148849_d_n5, assign96260_e148849_d_n6, assign96260_e148849_d_n7, assign96260_e148849_d_n8, assign96260_e148849_d_n9, assign96260_e148849_d_n10, assign96260_e148849_d_n11, assign96260_e148849_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96260_e148847: f64 = (p.p15 * locals.var_jsswg);
        (assign96260_e148847, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96260_e148849;
        locals.var_isbd_swg_dn0 = assign96260_e148849_d_n0;
        locals.var_isbd_swg_dn2 = assign96260_e148849_d_n2;
        locals.var_isbd_swg_dn4 = assign96260_e148849_d_n4;
        locals.var_isbd_swg_dn5 = assign96260_e148849_d_n5;
        locals.var_isbd_swg_dn6 = assign96260_e148849_d_n6;
        locals.var_isbd_swg_dn7 = assign96260_e148849_d_n7;
        locals.var_isbd_swg_dn8 = assign96260_e148849_d_n8;
        locals.var_isbd_swg_dn9 = assign96260_e148849_d_n9;
        locals.var_isbd_swg_dn10 = assign96260_e148849_d_n10;
        locals.var_isbd_swg_dn11 = assign96260_e148849_d_n11;
        locals.var_isbd_swg_dn14 = assign96260_e148849_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96270_e148860, assign96270_e148860_d_n0, assign96270_e148860_d_n2, assign96270_e148860_d_n4, assign96270_e148860_d_n5, assign96270_e148860_d_n6, assign96270_e148860_d_n7, assign96270_e148860_d_n8, assign96270_e148860_d_n9, assign96270_e148860_d_n10, assign96270_e148860_d_n11, assign96270_e148860_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96270_e148858: f64 = (p.p15 * locals.var_jsswg2);
        (assign96270_e148858, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96270_e148860;
        locals.var_isbd2_swg_dn0 = assign96270_e148860_d_n0;
        locals.var_isbd2_swg_dn2 = assign96270_e148860_d_n2;
        locals.var_isbd2_swg_dn4 = assign96270_e148860_d_n4;
        locals.var_isbd2_swg_dn5 = assign96270_e148860_d_n5;
        locals.var_isbd2_swg_dn6 = assign96270_e148860_d_n6;
        locals.var_isbd2_swg_dn7 = assign96270_e148860_d_n7;
        locals.var_isbd2_swg_dn8 = assign96270_e148860_d_n8;
        locals.var_isbd2_swg_dn9 = assign96270_e148860_d_n9;
        locals.var_isbd2_swg_dn10 = assign96270_e148860_d_n10;
        locals.var_isbd2_swg_dn11 = assign96270_e148860_d_n11;
        locals.var_isbd2_swg_dn14 = assign96270_e148860_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96280_e148869, assign96280_e148869_d_n0, assign96280_e148869_d_n2, assign96280_e148869_d_n4, assign96280_e148869_d_n5, assign96280_e148869_d_n6, assign96280_e148869_d_n7, assign96280_e148869_d_n8, assign96280_e148869_d_n9, assign96280_e148869_d_n10, assign96280_e148869_d_n11, assign96280_e148869_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96280_e148867: f64 = (p.p13 * locals.var_js);
        (assign96280_e148867, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96280_e148869;
        locals.var_isbd_btm_dn0 = assign96280_e148869_d_n0;
        locals.var_isbd_btm_dn2 = assign96280_e148869_d_n2;
        locals.var_isbd_btm_dn4 = assign96280_e148869_d_n4;
        locals.var_isbd_btm_dn5 = assign96280_e148869_d_n5;
        locals.var_isbd_btm_dn6 = assign96280_e148869_d_n6;
        locals.var_isbd_btm_dn7 = assign96280_e148869_d_n7;
        locals.var_isbd_btm_dn8 = assign96280_e148869_d_n8;
        locals.var_isbd_btm_dn9 = assign96280_e148869_d_n9;
        locals.var_isbd_btm_dn10 = assign96280_e148869_d_n10;
        locals.var_isbd_btm_dn11 = assign96280_e148869_d_n11;
        locals.var_isbd_btm_dn14 = assign96280_e148869_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96290_e148878, assign96290_e148878_d_n0, assign96290_e148878_d_n2, assign96290_e148878_d_n4, assign96290_e148878_d_n5, assign96290_e148878_d_n6, assign96290_e148878_d_n7, assign96290_e148878_d_n8, assign96290_e148878_d_n9, assign96290_e148878_d_n10, assign96290_e148878_d_n11, assign96290_e148878_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96290_e148876: f64 = (p.p13 * locals.var_js2);
        (assign96290_e148876, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96290_e148878;
        locals.var_isbd2_btm_dn0 = assign96290_e148878_d_n0;
        locals.var_isbd2_btm_dn2 = assign96290_e148878_d_n2;
        locals.var_isbd2_btm_dn4 = assign96290_e148878_d_n4;
        locals.var_isbd2_btm_dn5 = assign96290_e148878_d_n5;
        locals.var_isbd2_btm_dn6 = assign96290_e148878_d_n6;
        locals.var_isbd2_btm_dn7 = assign96290_e148878_d_n7;
        locals.var_isbd2_btm_dn8 = assign96290_e148878_d_n8;
        locals.var_isbd2_btm_dn9 = assign96290_e148878_d_n9;
        locals.var_isbd2_btm_dn10 = assign96290_e148878_d_n10;
        locals.var_isbd2_btm_dn11 = assign96290_e148878_d_n11;
        locals.var_isbd2_btm_dn14 = assign96290_e148878_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96300_e148887, assign96300_e148887_d_n0, assign96300_e148887_d_n2, assign96300_e148887_d_n4, assign96300_e148887_d_n5, assign96300_e148887_d_n6, assign96300_e148887_d_n7, assign96300_e148887_d_n8, assign96300_e148887_d_n9, assign96300_e148887_d_n10, assign96300_e148887_d_n11, assign96300_e148887_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96300_e148885: f64 = (p.p15 * locals.var_jssw);
        (assign96300_e148885, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96300_e148887;
        locals.var_isbd_sws_dn0 = assign96300_e148887_d_n0;
        locals.var_isbd_sws_dn2 = assign96300_e148887_d_n2;
        locals.var_isbd_sws_dn4 = assign96300_e148887_d_n4;
        locals.var_isbd_sws_dn5 = assign96300_e148887_d_n5;
        locals.var_isbd_sws_dn6 = assign96300_e148887_d_n6;
        locals.var_isbd_sws_dn7 = assign96300_e148887_d_n7;
        locals.var_isbd_sws_dn8 = assign96300_e148887_d_n8;
        locals.var_isbd_sws_dn9 = assign96300_e148887_d_n9;
        locals.var_isbd_sws_dn10 = assign96300_e148887_d_n10;
        locals.var_isbd_sws_dn11 = assign96300_e148887_d_n11;
        locals.var_isbd_sws_dn14 = assign96300_e148887_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96310_e148896, assign96310_e148896_d_n0, assign96310_e148896_d_n2, assign96310_e148896_d_n4, assign96310_e148896_d_n5, assign96310_e148896_d_n6, assign96310_e148896_d_n7, assign96310_e148896_d_n8, assign96310_e148896_d_n9, assign96310_e148896_d_n10, assign96310_e148896_d_n11, assign96310_e148896_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96310_e148894: f64 = (p.p15 * locals.var_jssw2);
        (assign96310_e148894, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96310_e148896;
        locals.var_isbd2_sws_dn0 = assign96310_e148896_d_n0;
        locals.var_isbd2_sws_dn2 = assign96310_e148896_d_n2;
        locals.var_isbd2_sws_dn4 = assign96310_e148896_d_n4;
        locals.var_isbd2_sws_dn5 = assign96310_e148896_d_n5;
        locals.var_isbd2_sws_dn6 = assign96310_e148896_d_n6;
        locals.var_isbd2_sws_dn7 = assign96310_e148896_d_n7;
        locals.var_isbd2_sws_dn8 = assign96310_e148896_d_n8;
        locals.var_isbd2_sws_dn9 = assign96310_e148896_d_n9;
        locals.var_isbd2_sws_dn10 = assign96310_e148896_d_n10;
        locals.var_isbd2_sws_dn11 = assign96310_e148896_d_n11;
        locals.var_isbd2_sws_dn14 = assign96310_e148896_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96320_e148903, assign96320_e148903_d_n0, assign96320_e148903_d_n2, assign96320_e148903_d_n4, assign96320_e148903_d_n5, assign96320_e148903_d_n6, assign96320_e148903_d_n7, assign96320_e148903_d_n8, assign96320_e148903_d_n9, assign96320_e148903_d_n10, assign96320_e148903_d_n11, assign96320_e148903_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96320_e148903;
        locals.var_isbd_swg_dn0 = assign96320_e148903_d_n0;
        locals.var_isbd_swg_dn2 = assign96320_e148903_d_n2;
        locals.var_isbd_swg_dn4 = assign96320_e148903_d_n4;
        locals.var_isbd_swg_dn5 = assign96320_e148903_d_n5;
        locals.var_isbd_swg_dn6 = assign96320_e148903_d_n6;
        locals.var_isbd_swg_dn7 = assign96320_e148903_d_n7;
        locals.var_isbd_swg_dn8 = assign96320_e148903_d_n8;
        locals.var_isbd_swg_dn9 = assign96320_e148903_d_n9;
        locals.var_isbd_swg_dn10 = assign96320_e148903_d_n10;
        locals.var_isbd_swg_dn11 = assign96320_e148903_d_n11;
        locals.var_isbd_swg_dn14 = assign96320_e148903_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96330_e148910, assign96330_e148910_d_n0, assign96330_e148910_d_n2, assign96330_e148910_d_n4, assign96330_e148910_d_n5, assign96330_e148910_d_n6, assign96330_e148910_d_n7, assign96330_e148910_d_n8, assign96330_e148910_d_n9, assign96330_e148910_d_n10, assign96330_e148910_d_n11, assign96330_e148910_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96330_e148910;
        locals.var_isbd2_swg_dn0 = assign96330_e148910_d_n0;
        locals.var_isbd2_swg_dn2 = assign96330_e148910_d_n2;
        locals.var_isbd2_swg_dn4 = assign96330_e148910_d_n4;
        locals.var_isbd2_swg_dn5 = assign96330_e148910_d_n5;
        locals.var_isbd2_swg_dn6 = assign96330_e148910_d_n6;
        locals.var_isbd2_swg_dn7 = assign96330_e148910_d_n7;
        locals.var_isbd2_swg_dn8 = assign96330_e148910_d_n8;
        locals.var_isbd2_swg_dn9 = assign96330_e148910_d_n9;
        locals.var_isbd2_swg_dn10 = assign96330_e148910_d_n10;
        locals.var_isbd2_swg_dn11 = assign96330_e148910_d_n11;
        locals.var_isbd2_swg_dn14 = assign96330_e148910_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96340_e148918, assign96340_e148918_d_n0, assign96340_e148918_d_n2, assign96340_e148918_d_n4, assign96340_e148918_d_n5, assign96340_e148918_d_n6, assign96340_e148918_d_n7, assign96340_e148918_d_n8, assign96340_e148918_d_n9, assign96340_e148918_d_n10, assign96340_e148918_d_n11, assign96340_e148918_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96340_e148914: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96340_e148916: f64 = (assign96340_e148914 + locals.var_isbd_swg);
        (assign96340_e148916, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign96340_e148918;
        locals.var_isbd_dn0 = assign96340_e148918_d_n0;
        locals.var_isbd_dn2 = assign96340_e148918_d_n2;
        locals.var_isbd_dn4 = assign96340_e148918_d_n4;
        locals.var_isbd_dn5 = assign96340_e148918_d_n5;
        locals.var_isbd_dn6 = assign96340_e148918_d_n6;
        locals.var_isbd_dn7 = assign96340_e148918_d_n7;
        locals.var_isbd_dn8 = assign96340_e148918_d_n8;
        locals.var_isbd_dn9 = assign96340_e148918_d_n9;
        locals.var_isbd_dn10 = assign96340_e148918_d_n10;
        locals.var_isbd_dn11 = assign96340_e148918_d_n11;
        locals.var_isbd_dn14 = assign96340_e148918_d_n14;
        locals.var_isbd_rv = 0.0;

        let assign96350_e148921: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96350_e148921;
        locals.var_guard2238_rv = 0.0;

        let (assign96360_e148929, assign96360_e148929_d_n0, assign96360_e148929_d_n2, assign96360_e148929_d_n4, assign96360_e148929_d_n5, assign96360_e148929_d_n6, assign96360_e148929_d_n7, assign96360_e148929_d_n8, assign96360_e148929_d_n9, assign96360_e148929_d_n10, assign96360_e148929_d_n11, assign96360_e148929_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96360_e148927: f64 = (locals.var_isbd + 1e-25);
        (assign96360_e148927, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign96360_e148929;
        locals.var_t2_dn0 = assign96360_e148929_d_n0;
        locals.var_t2_dn2 = assign96360_e148929_d_n2;
        locals.var_t2_dn4 = assign96360_e148929_d_n4;
        locals.var_t2_dn5 = assign96360_e148929_d_n5;
        locals.var_t2_dn6 = assign96360_e148929_d_n6;
        locals.var_t2_dn7 = assign96360_e148929_d_n7;
        locals.var_t2_dn8 = assign96360_e148929_d_n8;
        locals.var_t2_dn9 = assign96360_e148929_d_n9;
        locals.var_t2_dn10 = assign96360_e148929_d_n10;
        locals.var_t2_dn11 = assign96360_e148929_d_n11;
        locals.var_t2_dn14 = assign96360_e148929_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign96370_e148946, assign96370_e148946_d_n0, assign96370_e148946_d_n2, assign96370_e148946_d_n4, assign96370_e148946_d_n5, assign96370_e148946_d_n6, assign96370_e148946_d_n7, assign96370_e148946_d_n8, assign96370_e148946_d_n9, assign96370_e148946_d_n10, assign96370_e148946_d_n11, assign96370_e148946_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96370_e148935: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96370_e148938: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96370_e148940: f64 = (assign96370_e148938 / locals.var_t2);
        let assign96370_e148942: f64 = (assign96370_e148940 + 1.0);
        let assign96370_e148943: f64 = (assign96370_e148942).ln();
        let assign96370_e148944: f64 = (assign96370_e148935 * assign96370_e148943);
        (assign96370_e148944, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign96370_e148946;
        locals.var_vbdt_dn0 = assign96370_e148946_d_n0;
        locals.var_vbdt_dn2 = assign96370_e148946_d_n2;
        locals.var_vbdt_dn4 = assign96370_e148946_d_n4;
        locals.var_vbdt_dn5 = assign96370_e148946_d_n5;
        locals.var_vbdt_dn6 = assign96370_e148946_d_n6;
        locals.var_vbdt_dn7 = assign96370_e148946_d_n7;
        locals.var_vbdt_dn8 = assign96370_e148946_d_n8;
        locals.var_vbdt_dn9 = assign96370_e148946_d_n9;
        locals.var_vbdt_dn10 = assign96370_e148946_d_n10;
        locals.var_vbdt_dn11 = assign96370_e148946_d_n11;
        locals.var_vbdt_dn14 = assign96370_e148946_d_n14;
        locals.var_vbdt_rv = 0.0;

        let (assign96380_e148957, assign96380_e148957_d_n0, assign96380_e148957_d_n2, assign96380_e148957_d_n4, assign96380_e148957_d_n5, assign96380_e148957_d_n6, assign96380_e148957_d_n7, assign96380_e148957_d_n8, assign96380_e148957_d_n9, assign96380_e148957_d_n10, assign96380_e148957_d_n11, assign96380_e148957_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96380_e148952: f64 = (locals.var_tratio - 1.0);
        let assign96380_e148954: f64 = (assign96380_e148952 * p.p512);
        let assign96380_e148955: f64 = (assign96380_e148954).exp();
        (assign96380_e148955, (assign96380_e148955 * (locals.var_tratio_dn0 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn2 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn4 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn5 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn6 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn7 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn8 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn9 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn10 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn11 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign96380_e148957;
        locals.var_exptempd_dn0 = assign96380_e148957_d_n0;
        locals.var_exptempd_dn2 = assign96380_e148957_d_n2;
        locals.var_exptempd_dn4 = assign96380_e148957_d_n4;
        locals.var_exptempd_dn5 = assign96380_e148957_d_n5;
        locals.var_exptempd_dn6 = assign96380_e148957_d_n6;
        locals.var_exptempd_dn7 = assign96380_e148957_d_n7;
        locals.var_exptempd_dn8 = assign96380_e148957_d_n8;
        locals.var_exptempd_dn9 = assign96380_e148957_d_n9;
        locals.var_exptempd_dn10 = assign96380_e148957_d_n10;
        locals.var_exptempd_dn11 = assign96380_e148957_d_n11;
        locals.var_exptempd_dn14 = assign96380_e148957_d_n14;
        locals.var_exptempd_rv = 0.0;

        let (assign96390_e148967, assign96390_e148967_d_n0, assign96390_e148967_d_n2, assign96390_e148967_d_n4, assign96390_e148967_d_n5, assign96390_e148967_d_n6, assign96390_e148967_d_n7, assign96390_e148967_d_n8, assign96390_e148967_d_n9, assign96390_e148967_d_n10, assign96390_e148967_d_n11, assign96390_e148967_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96390_e148964: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96390_e148965: f64 = (1.0 / assign96390_e148964);
        (assign96390_e148965, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign96390_e148967;
        locals.var_jd_nvtm_invd_dn0 = assign96390_e148967_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96390_e148967_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96390_e148967_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96390_e148967_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96390_e148967_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96390_e148967_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96390_e148967_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96390_e148967_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96390_e148967_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign96390_e148967_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign96390_e148967_d_n14;
        locals.var_jd_nvtm_invd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_374(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96400_e148976, assign96400_e148976_d_n0, assign96400_e148976_d_n2, assign96400_e148976_d_n4, assign96400_e148976_d_n5, assign96400_e148976_d_n6, assign96400_e148976_d_n7, assign96400_e148976_d_n8, assign96400_e148976_d_n9, assign96400_e148976_d_n10, assign96400_e148976_d_n11, assign96400_e148976_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96400_e148973: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96400_e148974: f64 = (assign96400_e148973).exp();
        (assign96400_e148974, (assign96400_e148974 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96400_e148974 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96400_e148974 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96400_e148974 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96400_e148974 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96400_e148974 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96400_e148974 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96400_e148974 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96400_e148974 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96400_e148974 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign96400_e148974 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign96400_e148976;
        locals.var_jd_expcd_dn0 = assign96400_e148976_d_n0;
        locals.var_jd_expcd_dn2 = assign96400_e148976_d_n2;
        locals.var_jd_expcd_dn4 = assign96400_e148976_d_n4;
        locals.var_jd_expcd_dn5 = assign96400_e148976_d_n5;
        locals.var_jd_expcd_dn6 = assign96400_e148976_d_n6;
        locals.var_jd_expcd_dn7 = assign96400_e148976_d_n7;
        locals.var_jd_expcd_dn8 = assign96400_e148976_d_n8;
        locals.var_jd_expcd_dn9 = assign96400_e148976_d_n9;
        locals.var_jd_expcd_dn10 = assign96400_e148976_d_n10;
        locals.var_jd_expcd_dn11 = assign96400_e148976_d_n11;
        locals.var_jd_expcd_dn14 = assign96400_e148976_d_n14;
        locals.var_jd_expcd_rv = 0.0;

        let (assign96410_e148995, assign96410_e148995_d_n0, assign96410_e148995_d_n2, assign96410_e148995_d_n4, assign96410_e148995_d_n5, assign96410_e148995_d_n6, assign96410_e148995_d_n7, assign96410_e148995_d_n8, assign96410_e148995_d_n9, assign96410_e148995_d_n10, assign96410_e148995_d_n11, assign96410_e148995_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96410_e148981: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96410_e148984: f64 = (locals.var_eg * locals.var_beta);
        let assign96410_e148985: f64 = (assign96410_e148981 - assign96410_e148984);
        let assign96410_e148988: f64 = (p.p522 * locals.var_log_tratio);
        let assign96410_e148989: f64 = (assign96410_e148985 + assign96410_e148988);
        let assign96410_e148991: f64 = (assign96410_e148989 / locals.var_uc_njs);
        let assign96410_e148992: f64 = (assign96410_e148991).exp();
        let assign96410_e148993: f64 = (locals.var_uc_js0s * assign96410_e148992);
        (assign96410_e148993, (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96410_e148995;
        locals.var_js_dn0 = assign96410_e148995_d_n0;
        locals.var_js_dn2 = assign96410_e148995_d_n2;
        locals.var_js_dn4 = assign96410_e148995_d_n4;
        locals.var_js_dn5 = assign96410_e148995_d_n5;
        locals.var_js_dn6 = assign96410_e148995_d_n6;
        locals.var_js_dn7 = assign96410_e148995_d_n7;
        locals.var_js_dn8 = assign96410_e148995_d_n8;
        locals.var_js_dn9 = assign96410_e148995_d_n9;
        locals.var_js_dn10 = assign96410_e148995_d_n10;
        locals.var_js_dn11 = assign96410_e148995_d_n11;
        locals.var_js_dn14 = assign96410_e148995_d_n14;
        locals.var_js_rv = 0.0;

        let (assign96420_e149014, assign96420_e149014_d_n0, assign96420_e149014_d_n2, assign96420_e149014_d_n4, assign96420_e149014_d_n5, assign96420_e149014_d_n6, assign96420_e149014_d_n7, assign96420_e149014_d_n8, assign96420_e149014_d_n9, assign96420_e149014_d_n10, assign96420_e149014_d_n11, assign96420_e149014_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96420_e149000: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96420_e149003: f64 = (locals.var_eg * locals.var_beta);
        let assign96420_e149004: f64 = (assign96420_e149000 - assign96420_e149003);
        let assign96420_e149007: f64 = (p.p522 * locals.var_log_tratio);
        let assign96420_e149008: f64 = (assign96420_e149004 + assign96420_e149007);
        let assign96420_e149010: f64 = (assign96420_e149008 / p.p520);
        let assign96420_e149011: f64 = (assign96420_e149010).exp();
        let assign96420_e149012: f64 = (locals.var_uc_js0sws * assign96420_e149011);
        (assign96420_e149012, (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96420_e149014;
        locals.var_jssw_dn0 = assign96420_e149014_d_n0;
        locals.var_jssw_dn2 = assign96420_e149014_d_n2;
        locals.var_jssw_dn4 = assign96420_e149014_d_n4;
        locals.var_jssw_dn5 = assign96420_e149014_d_n5;
        locals.var_jssw_dn6 = assign96420_e149014_d_n6;
        locals.var_jssw_dn7 = assign96420_e149014_d_n7;
        locals.var_jssw_dn8 = assign96420_e149014_d_n8;
        locals.var_jssw_dn9 = assign96420_e149014_d_n9;
        locals.var_jssw_dn10 = assign96420_e149014_d_n10;
        locals.var_jssw_dn11 = assign96420_e149014_d_n11;
        locals.var_jssw_dn14 = assign96420_e149014_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign96430_e149033, assign96430_e149033_d_n0, assign96430_e149033_d_n2, assign96430_e149033_d_n4, assign96430_e149033_d_n5, assign96430_e149033_d_n6, assign96430_e149033_d_n7, assign96430_e149033_d_n8, assign96430_e149033_d_n9, assign96430_e149033_d_n10, assign96430_e149033_d_n11, assign96430_e149033_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96430_e149019: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e149022: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e149023: f64 = (assign96430_e149019 - assign96430_e149022);
        let assign96430_e149026: f64 = (p.p522 * locals.var_log_tratio);
        let assign96430_e149027: f64 = (assign96430_e149023 + assign96430_e149026);
        let assign96430_e149029: f64 = (assign96430_e149027 / p.p521);
        let assign96430_e149030: f64 = (assign96430_e149029).exp();
        let assign96430_e149031: f64 = (p.p518 * assign96430_e149030);
        (assign96430_e149031, (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96430_e149033;
        locals.var_jsswg_dn0 = assign96430_e149033_d_n0;
        locals.var_jsswg_dn2 = assign96430_e149033_d_n2;
        locals.var_jsswg_dn4 = assign96430_e149033_d_n4;
        locals.var_jsswg_dn5 = assign96430_e149033_d_n5;
        locals.var_jsswg_dn6 = assign96430_e149033_d_n6;
        locals.var_jsswg_dn7 = assign96430_e149033_d_n7;
        locals.var_jsswg_dn8 = assign96430_e149033_d_n8;
        locals.var_jsswg_dn9 = assign96430_e149033_d_n9;
        locals.var_jsswg_dn10 = assign96430_e149033_d_n10;
        locals.var_jsswg_dn11 = assign96430_e149033_d_n11;
        locals.var_jsswg_dn14 = assign96430_e149033_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign96440_e149052, assign96440_e149052_d_n0, assign96440_e149052_d_n2, assign96440_e149052_d_n4, assign96440_e149052_d_n5, assign96440_e149052_d_n6, assign96440_e149052_d_n7, assign96440_e149052_d_n8, assign96440_e149052_d_n9, assign96440_e149052_d_n10, assign96440_e149052_d_n11, assign96440_e149052_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96440_e149038: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149041: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149042: f64 = (assign96440_e149038 - assign96440_e149041);
        let assign96440_e149045: f64 = (p.p532 * locals.var_log_tratio);
        let assign96440_e149046: f64 = (assign96440_e149042 + assign96440_e149045);
        let assign96440_e149048: f64 = (assign96440_e149046 / locals.var_uc_njs);
        let assign96440_e149049: f64 = (assign96440_e149048).exp();
        let assign96440_e149050: f64 = (locals.var_uc_js0s * assign96440_e149049);
        (assign96440_e149050, (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96440_e149052;
        locals.var_js2_dn0 = assign96440_e149052_d_n0;
        locals.var_js2_dn2 = assign96440_e149052_d_n2;
        locals.var_js2_dn4 = assign96440_e149052_d_n4;
        locals.var_js2_dn5 = assign96440_e149052_d_n5;
        locals.var_js2_dn6 = assign96440_e149052_d_n6;
        locals.var_js2_dn7 = assign96440_e149052_d_n7;
        locals.var_js2_dn8 = assign96440_e149052_d_n8;
        locals.var_js2_dn9 = assign96440_e149052_d_n9;
        locals.var_js2_dn10 = assign96440_e149052_d_n10;
        locals.var_js2_dn11 = assign96440_e149052_d_n11;
        locals.var_js2_dn14 = assign96440_e149052_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign96450_e149071, assign96450_e149071_d_n0, assign96450_e149071_d_n2, assign96450_e149071_d_n4, assign96450_e149071_d_n5, assign96450_e149071_d_n6, assign96450_e149071_d_n7, assign96450_e149071_d_n8, assign96450_e149071_d_n9, assign96450_e149071_d_n10, assign96450_e149071_d_n11, assign96450_e149071_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96450_e149057: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96450_e149060: f64 = (locals.var_eg * locals.var_beta);
        let assign96450_e149061: f64 = (assign96450_e149057 - assign96450_e149060);
        let assign96450_e149064: f64 = (p.p532 * locals.var_log_tratio);
        let assign96450_e149065: f64 = (assign96450_e149061 + assign96450_e149064);
        let assign96450_e149067: f64 = (assign96450_e149065 / p.p520);
        let assign96450_e149068: f64 = (assign96450_e149067).exp();
        let assign96450_e149069: f64 = (locals.var_uc_js0sws * assign96450_e149068);
        (assign96450_e149069, (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96450_e149071;
        locals.var_jssw2_dn0 = assign96450_e149071_d_n0;
        locals.var_jssw2_dn2 = assign96450_e149071_d_n2;
        locals.var_jssw2_dn4 = assign96450_e149071_d_n4;
        locals.var_jssw2_dn5 = assign96450_e149071_d_n5;
        locals.var_jssw2_dn6 = assign96450_e149071_d_n6;
        locals.var_jssw2_dn7 = assign96450_e149071_d_n7;
        locals.var_jssw2_dn8 = assign96450_e149071_d_n8;
        locals.var_jssw2_dn9 = assign96450_e149071_d_n9;
        locals.var_jssw2_dn10 = assign96450_e149071_d_n10;
        locals.var_jssw2_dn11 = assign96450_e149071_d_n11;
        locals.var_jssw2_dn14 = assign96450_e149071_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign96460_e149090, assign96460_e149090_d_n0, assign96460_e149090_d_n2, assign96460_e149090_d_n4, assign96460_e149090_d_n5, assign96460_e149090_d_n6, assign96460_e149090_d_n7, assign96460_e149090_d_n8, assign96460_e149090_d_n9, assign96460_e149090_d_n10, assign96460_e149090_d_n11, assign96460_e149090_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96460_e149076: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96460_e149079: f64 = (locals.var_eg * locals.var_beta);
        let assign96460_e149080: f64 = (assign96460_e149076 - assign96460_e149079);
        let assign96460_e149083: f64 = (p.p532 * locals.var_log_tratio);
        let assign96460_e149084: f64 = (assign96460_e149080 + assign96460_e149083);
        let assign96460_e149086: f64 = (assign96460_e149084 / p.p521);
        let assign96460_e149087: f64 = (assign96460_e149086).exp();
        let assign96460_e149088: f64 = (p.p518 * assign96460_e149087);
        (assign96460_e149088, (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96460_e149090;
        locals.var_jsswg2_dn0 = assign96460_e149090_d_n0;
        locals.var_jsswg2_dn2 = assign96460_e149090_d_n2;
        locals.var_jsswg2_dn4 = assign96460_e149090_d_n4;
        locals.var_jsswg2_dn5 = assign96460_e149090_d_n5;
        locals.var_jsswg2_dn6 = assign96460_e149090_d_n6;
        locals.var_jsswg2_dn7 = assign96460_e149090_d_n7;
        locals.var_jsswg2_dn8 = assign96460_e149090_d_n8;
        locals.var_jsswg2_dn9 = assign96460_e149090_d_n9;
        locals.var_jsswg2_dn10 = assign96460_e149090_d_n10;
        locals.var_jsswg2_dn11 = assign96460_e149090_d_n11;
        locals.var_jsswg2_dn14 = assign96460_e149090_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign96470_e149093: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96470_e149093;
        locals.var_guard2239_rv = 0.0;

        let assign96480_e149096: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96480_e149096;
        locals.var_guard2240_rv = 0.0;

        let (assign96490_e149106, assign96490_e149106_d_n0, assign96490_e149106_d_n2, assign96490_e149106_d_n4, assign96490_e149106_d_n5, assign96490_e149106_d_n6, assign96490_e149106_d_n7, assign96490_e149106_d_n8, assign96490_e149106_d_n9, assign96490_e149106_d_n10, assign96490_e149106_d_n11, assign96490_e149106_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96490_e149104: f64 = (p.p14 * locals.var_js);
        (assign96490_e149104, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96490_e149106;
        locals.var_isbs_btm_dn0 = assign96490_e149106_d_n0;
        locals.var_isbs_btm_dn2 = assign96490_e149106_d_n2;
        locals.var_isbs_btm_dn4 = assign96490_e149106_d_n4;
        locals.var_isbs_btm_dn5 = assign96490_e149106_d_n5;
        locals.var_isbs_btm_dn6 = assign96490_e149106_d_n6;
        locals.var_isbs_btm_dn7 = assign96490_e149106_d_n7;
        locals.var_isbs_btm_dn8 = assign96490_e149106_d_n8;
        locals.var_isbs_btm_dn9 = assign96490_e149106_d_n9;
        locals.var_isbs_btm_dn10 = assign96490_e149106_d_n10;
        locals.var_isbs_btm_dn11 = assign96490_e149106_d_n11;
        locals.var_isbs_btm_dn14 = assign96490_e149106_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96500_e149116, assign96500_e149116_d_n0, assign96500_e149116_d_n2, assign96500_e149116_d_n4, assign96500_e149116_d_n5, assign96500_e149116_d_n6, assign96500_e149116_d_n7, assign96500_e149116_d_n8, assign96500_e149116_d_n9, assign96500_e149116_d_n10, assign96500_e149116_d_n11, assign96500_e149116_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96500_e149114: f64 = (p.p14 * locals.var_js2);
        (assign96500_e149114, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96500_e149116;
        locals.var_isbs2_btm_dn0 = assign96500_e149116_d_n0;
        locals.var_isbs2_btm_dn2 = assign96500_e149116_d_n2;
        locals.var_isbs2_btm_dn4 = assign96500_e149116_d_n4;
        locals.var_isbs2_btm_dn5 = assign96500_e149116_d_n5;
        locals.var_isbs2_btm_dn6 = assign96500_e149116_d_n6;
        locals.var_isbs2_btm_dn7 = assign96500_e149116_d_n7;
        locals.var_isbs2_btm_dn8 = assign96500_e149116_d_n8;
        locals.var_isbs2_btm_dn9 = assign96500_e149116_d_n9;
        locals.var_isbs2_btm_dn10 = assign96500_e149116_d_n10;
        locals.var_isbs2_btm_dn11 = assign96500_e149116_d_n11;
        locals.var_isbs2_btm_dn14 = assign96500_e149116_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96510_e149128, assign96510_e149128_d_n0, assign96510_e149128_d_n2, assign96510_e149128_d_n4, assign96510_e149128_d_n5, assign96510_e149128_d_n6, assign96510_e149128_d_n7, assign96510_e149128_d_n8, assign96510_e149128_d_n9, assign96510_e149128_d_n10, assign96510_e149128_d_n11, assign96510_e149128_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96510_e149124: f64 = (p.p16 - locals.var_weff_nf);
        let assign96510_e149126: f64 = (assign96510_e149124 * locals.var_jssw);
        (assign96510_e149126, (assign96510_e149124 * locals.var_jssw_dn0), (assign96510_e149124 * locals.var_jssw_dn2), (assign96510_e149124 * locals.var_jssw_dn4), (assign96510_e149124 * locals.var_jssw_dn5), (assign96510_e149124 * locals.var_jssw_dn6), (assign96510_e149124 * locals.var_jssw_dn7), (assign96510_e149124 * locals.var_jssw_dn8), (assign96510_e149124 * locals.var_jssw_dn9), (assign96510_e149124 * locals.var_jssw_dn10), (assign96510_e149124 * locals.var_jssw_dn11), (assign96510_e149124 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96510_e149128;
        locals.var_isbs_sws_dn0 = assign96510_e149128_d_n0;
        locals.var_isbs_sws_dn2 = assign96510_e149128_d_n2;
        locals.var_isbs_sws_dn4 = assign96510_e149128_d_n4;
        locals.var_isbs_sws_dn5 = assign96510_e149128_d_n5;
        locals.var_isbs_sws_dn6 = assign96510_e149128_d_n6;
        locals.var_isbs_sws_dn7 = assign96510_e149128_d_n7;
        locals.var_isbs_sws_dn8 = assign96510_e149128_d_n8;
        locals.var_isbs_sws_dn9 = assign96510_e149128_d_n9;
        locals.var_isbs_sws_dn10 = assign96510_e149128_d_n10;
        locals.var_isbs_sws_dn11 = assign96510_e149128_d_n11;
        locals.var_isbs_sws_dn14 = assign96510_e149128_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96520_e149140, assign96520_e149140_d_n0, assign96520_e149140_d_n2, assign96520_e149140_d_n4, assign96520_e149140_d_n5, assign96520_e149140_d_n6, assign96520_e149140_d_n7, assign96520_e149140_d_n8, assign96520_e149140_d_n9, assign96520_e149140_d_n10, assign96520_e149140_d_n11, assign96520_e149140_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96520_e149136: f64 = (p.p16 - locals.var_weff_nf);
        let assign96520_e149138: f64 = (assign96520_e149136 * locals.var_jssw2);
        (assign96520_e149138, (assign96520_e149136 * locals.var_jssw2_dn0), (assign96520_e149136 * locals.var_jssw2_dn2), (assign96520_e149136 * locals.var_jssw2_dn4), (assign96520_e149136 * locals.var_jssw2_dn5), (assign96520_e149136 * locals.var_jssw2_dn6), (assign96520_e149136 * locals.var_jssw2_dn7), (assign96520_e149136 * locals.var_jssw2_dn8), (assign96520_e149136 * locals.var_jssw2_dn9), (assign96520_e149136 * locals.var_jssw2_dn10), (assign96520_e149136 * locals.var_jssw2_dn11), (assign96520_e149136 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96520_e149140;
        locals.var_isbs2_sws_dn0 = assign96520_e149140_d_n0;
        locals.var_isbs2_sws_dn2 = assign96520_e149140_d_n2;
        locals.var_isbs2_sws_dn4 = assign96520_e149140_d_n4;
        locals.var_isbs2_sws_dn5 = assign96520_e149140_d_n5;
        locals.var_isbs2_sws_dn6 = assign96520_e149140_d_n6;
        locals.var_isbs2_sws_dn7 = assign96520_e149140_d_n7;
        locals.var_isbs2_sws_dn8 = assign96520_e149140_d_n8;
        locals.var_isbs2_sws_dn9 = assign96520_e149140_d_n9;
        locals.var_isbs2_sws_dn10 = assign96520_e149140_d_n10;
        locals.var_isbs2_sws_dn11 = assign96520_e149140_d_n11;
        locals.var_isbs2_sws_dn14 = assign96520_e149140_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96530_e149150, assign96530_e149150_d_n0, assign96530_e149150_d_n2, assign96530_e149150_d_n4, assign96530_e149150_d_n5, assign96530_e149150_d_n6, assign96530_e149150_d_n7, assign96530_e149150_d_n8, assign96530_e149150_d_n9, assign96530_e149150_d_n10, assign96530_e149150_d_n11, assign96530_e149150_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96530_e149148: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96530_e149148, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96530_e149150;
        locals.var_isbs_swg_dn0 = assign96530_e149150_d_n0;
        locals.var_isbs_swg_dn2 = assign96530_e149150_d_n2;
        locals.var_isbs_swg_dn4 = assign96530_e149150_d_n4;
        locals.var_isbs_swg_dn5 = assign96530_e149150_d_n5;
        locals.var_isbs_swg_dn6 = assign96530_e149150_d_n6;
        locals.var_isbs_swg_dn7 = assign96530_e149150_d_n7;
        locals.var_isbs_swg_dn8 = assign96530_e149150_d_n8;
        locals.var_isbs_swg_dn9 = assign96530_e149150_d_n9;
        locals.var_isbs_swg_dn10 = assign96530_e149150_d_n10;
        locals.var_isbs_swg_dn11 = assign96530_e149150_d_n11;
        locals.var_isbs_swg_dn14 = assign96530_e149150_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96540_e149160, assign96540_e149160_d_n0, assign96540_e149160_d_n2, assign96540_e149160_d_n4, assign96540_e149160_d_n5, assign96540_e149160_d_n6, assign96540_e149160_d_n7, assign96540_e149160_d_n8, assign96540_e149160_d_n9, assign96540_e149160_d_n10, assign96540_e149160_d_n11, assign96540_e149160_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96540_e149158: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96540_e149158, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96540_e149160;
        locals.var_isbs2_swg_dn0 = assign96540_e149160_d_n0;
        locals.var_isbs2_swg_dn2 = assign96540_e149160_d_n2;
        locals.var_isbs2_swg_dn4 = assign96540_e149160_d_n4;
        locals.var_isbs2_swg_dn5 = assign96540_e149160_d_n5;
        locals.var_isbs2_swg_dn6 = assign96540_e149160_d_n6;
        locals.var_isbs2_swg_dn7 = assign96540_e149160_d_n7;
        locals.var_isbs2_swg_dn8 = assign96540_e149160_d_n8;
        locals.var_isbs2_swg_dn9 = assign96540_e149160_d_n9;
        locals.var_isbs2_swg_dn10 = assign96540_e149160_d_n10;
        locals.var_isbs2_swg_dn11 = assign96540_e149160_d_n11;
        locals.var_isbs2_swg_dn14 = assign96540_e149160_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96550_e149171, assign96550_e149171_d_n0, assign96550_e149171_d_n2, assign96550_e149171_d_n4, assign96550_e149171_d_n5, assign96550_e149171_d_n6, assign96550_e149171_d_n7, assign96550_e149171_d_n8, assign96550_e149171_d_n9, assign96550_e149171_d_n10, assign96550_e149171_d_n11, assign96550_e149171_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96550_e149169: f64 = (p.p14 * locals.var_js);
        (assign96550_e149169, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96550_e149171;
        locals.var_isbs_btm_dn0 = assign96550_e149171_d_n0;
        locals.var_isbs_btm_dn2 = assign96550_e149171_d_n2;
        locals.var_isbs_btm_dn4 = assign96550_e149171_d_n4;
        locals.var_isbs_btm_dn5 = assign96550_e149171_d_n5;
        locals.var_isbs_btm_dn6 = assign96550_e149171_d_n6;
        locals.var_isbs_btm_dn7 = assign96550_e149171_d_n7;
        locals.var_isbs_btm_dn8 = assign96550_e149171_d_n8;
        locals.var_isbs_btm_dn9 = assign96550_e149171_d_n9;
        locals.var_isbs_btm_dn10 = assign96550_e149171_d_n10;
        locals.var_isbs_btm_dn11 = assign96550_e149171_d_n11;
        locals.var_isbs_btm_dn14 = assign96550_e149171_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96560_e149182, assign96560_e149182_d_n0, assign96560_e149182_d_n2, assign96560_e149182_d_n4, assign96560_e149182_d_n5, assign96560_e149182_d_n6, assign96560_e149182_d_n7, assign96560_e149182_d_n8, assign96560_e149182_d_n9, assign96560_e149182_d_n10, assign96560_e149182_d_n11, assign96560_e149182_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96560_e149180: f64 = (p.p14 * locals.var_js2);
        (assign96560_e149180, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96560_e149182;
        locals.var_isbs2_btm_dn0 = assign96560_e149182_d_n0;
        locals.var_isbs2_btm_dn2 = assign96560_e149182_d_n2;
        locals.var_isbs2_btm_dn4 = assign96560_e149182_d_n4;
        locals.var_isbs2_btm_dn5 = assign96560_e149182_d_n5;
        locals.var_isbs2_btm_dn6 = assign96560_e149182_d_n6;
        locals.var_isbs2_btm_dn7 = assign96560_e149182_d_n7;
        locals.var_isbs2_btm_dn8 = assign96560_e149182_d_n8;
        locals.var_isbs2_btm_dn9 = assign96560_e149182_d_n9;
        locals.var_isbs2_btm_dn10 = assign96560_e149182_d_n10;
        locals.var_isbs2_btm_dn11 = assign96560_e149182_d_n11;
        locals.var_isbs2_btm_dn14 = assign96560_e149182_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96570_e149191, assign96570_e149191_d_n0, assign96570_e149191_d_n2, assign96570_e149191_d_n4, assign96570_e149191_d_n5, assign96570_e149191_d_n6, assign96570_e149191_d_n7, assign96570_e149191_d_n8, assign96570_e149191_d_n9, assign96570_e149191_d_n10, assign96570_e149191_d_n11, assign96570_e149191_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96570_e149191;
        locals.var_isbs_sws_dn0 = assign96570_e149191_d_n0;
        locals.var_isbs_sws_dn2 = assign96570_e149191_d_n2;
        locals.var_isbs_sws_dn4 = assign96570_e149191_d_n4;
        locals.var_isbs_sws_dn5 = assign96570_e149191_d_n5;
        locals.var_isbs_sws_dn6 = assign96570_e149191_d_n6;
        locals.var_isbs_sws_dn7 = assign96570_e149191_d_n7;
        locals.var_isbs_sws_dn8 = assign96570_e149191_d_n8;
        locals.var_isbs_sws_dn9 = assign96570_e149191_d_n9;
        locals.var_isbs_sws_dn10 = assign96570_e149191_d_n10;
        locals.var_isbs_sws_dn11 = assign96570_e149191_d_n11;
        locals.var_isbs_sws_dn14 = assign96570_e149191_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96580_e149200, assign96580_e149200_d_n0, assign96580_e149200_d_n2, assign96580_e149200_d_n4, assign96580_e149200_d_n5, assign96580_e149200_d_n6, assign96580_e149200_d_n7, assign96580_e149200_d_n8, assign96580_e149200_d_n9, assign96580_e149200_d_n10, assign96580_e149200_d_n11, assign96580_e149200_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96580_e149200;
        locals.var_isbs2_sws_dn0 = assign96580_e149200_d_n0;
        locals.var_isbs2_sws_dn2 = assign96580_e149200_d_n2;
        locals.var_isbs2_sws_dn4 = assign96580_e149200_d_n4;
        locals.var_isbs2_sws_dn5 = assign96580_e149200_d_n5;
        locals.var_isbs2_sws_dn6 = assign96580_e149200_d_n6;
        locals.var_isbs2_sws_dn7 = assign96580_e149200_d_n7;
        locals.var_isbs2_sws_dn8 = assign96580_e149200_d_n8;
        locals.var_isbs2_sws_dn9 = assign96580_e149200_d_n9;
        locals.var_isbs2_sws_dn10 = assign96580_e149200_d_n10;
        locals.var_isbs2_sws_dn11 = assign96580_e149200_d_n11;
        locals.var_isbs2_sws_dn14 = assign96580_e149200_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96590_e149211, assign96590_e149211_d_n0, assign96590_e149211_d_n2, assign96590_e149211_d_n4, assign96590_e149211_d_n5, assign96590_e149211_d_n6, assign96590_e149211_d_n7, assign96590_e149211_d_n8, assign96590_e149211_d_n9, assign96590_e149211_d_n10, assign96590_e149211_d_n11, assign96590_e149211_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96590_e149209: f64 = (p.p16 * locals.var_jsswg);
        (assign96590_e149209, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96590_e149211;
        locals.var_isbs_swg_dn0 = assign96590_e149211_d_n0;
        locals.var_isbs_swg_dn2 = assign96590_e149211_d_n2;
        locals.var_isbs_swg_dn4 = assign96590_e149211_d_n4;
        locals.var_isbs_swg_dn5 = assign96590_e149211_d_n5;
        locals.var_isbs_swg_dn6 = assign96590_e149211_d_n6;
        locals.var_isbs_swg_dn7 = assign96590_e149211_d_n7;
        locals.var_isbs_swg_dn8 = assign96590_e149211_d_n8;
        locals.var_isbs_swg_dn9 = assign96590_e149211_d_n9;
        locals.var_isbs_swg_dn10 = assign96590_e149211_d_n10;
        locals.var_isbs_swg_dn11 = assign96590_e149211_d_n11;
        locals.var_isbs_swg_dn14 = assign96590_e149211_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96600_e149222, assign96600_e149222_d_n0, assign96600_e149222_d_n2, assign96600_e149222_d_n4, assign96600_e149222_d_n5, assign96600_e149222_d_n6, assign96600_e149222_d_n7, assign96600_e149222_d_n8, assign96600_e149222_d_n9, assign96600_e149222_d_n10, assign96600_e149222_d_n11, assign96600_e149222_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96600_e149220: f64 = (p.p16 * locals.var_jsswg2);
        (assign96600_e149220, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96600_e149222;
        locals.var_isbs2_swg_dn0 = assign96600_e149222_d_n0;
        locals.var_isbs2_swg_dn2 = assign96600_e149222_d_n2;
        locals.var_isbs2_swg_dn4 = assign96600_e149222_d_n4;
        locals.var_isbs2_swg_dn5 = assign96600_e149222_d_n5;
        locals.var_isbs2_swg_dn6 = assign96600_e149222_d_n6;
        locals.var_isbs2_swg_dn7 = assign96600_e149222_d_n7;
        locals.var_isbs2_swg_dn8 = assign96600_e149222_d_n8;
        locals.var_isbs2_swg_dn9 = assign96600_e149222_d_n9;
        locals.var_isbs2_swg_dn10 = assign96600_e149222_d_n10;
        locals.var_isbs2_swg_dn11 = assign96600_e149222_d_n11;
        locals.var_isbs2_swg_dn14 = assign96600_e149222_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96610_e149231, assign96610_e149231_d_n0, assign96610_e149231_d_n2, assign96610_e149231_d_n4, assign96610_e149231_d_n5, assign96610_e149231_d_n6, assign96610_e149231_d_n7, assign96610_e149231_d_n8, assign96610_e149231_d_n9, assign96610_e149231_d_n10, assign96610_e149231_d_n11, assign96610_e149231_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96610_e149229: f64 = (p.p14 * locals.var_js);
        (assign96610_e149229, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96610_e149231;
        locals.var_isbs_btm_dn0 = assign96610_e149231_d_n0;
        locals.var_isbs_btm_dn2 = assign96610_e149231_d_n2;
        locals.var_isbs_btm_dn4 = assign96610_e149231_d_n4;
        locals.var_isbs_btm_dn5 = assign96610_e149231_d_n5;
        locals.var_isbs_btm_dn6 = assign96610_e149231_d_n6;
        locals.var_isbs_btm_dn7 = assign96610_e149231_d_n7;
        locals.var_isbs_btm_dn8 = assign96610_e149231_d_n8;
        locals.var_isbs_btm_dn9 = assign96610_e149231_d_n9;
        locals.var_isbs_btm_dn10 = assign96610_e149231_d_n10;
        locals.var_isbs_btm_dn11 = assign96610_e149231_d_n11;
        locals.var_isbs_btm_dn14 = assign96610_e149231_d_n14;
        locals.var_isbs_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_375(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96620_e149240, assign96620_e149240_d_n0, assign96620_e149240_d_n2, assign96620_e149240_d_n4, assign96620_e149240_d_n5, assign96620_e149240_d_n6, assign96620_e149240_d_n7, assign96620_e149240_d_n8, assign96620_e149240_d_n9, assign96620_e149240_d_n10, assign96620_e149240_d_n11, assign96620_e149240_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96620_e149238: f64 = (p.p14 * locals.var_js2);
        (assign96620_e149238, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96620_e149240;
        locals.var_isbs2_btm_dn0 = assign96620_e149240_d_n0;
        locals.var_isbs2_btm_dn2 = assign96620_e149240_d_n2;
        locals.var_isbs2_btm_dn4 = assign96620_e149240_d_n4;
        locals.var_isbs2_btm_dn5 = assign96620_e149240_d_n5;
        locals.var_isbs2_btm_dn6 = assign96620_e149240_d_n6;
        locals.var_isbs2_btm_dn7 = assign96620_e149240_d_n7;
        locals.var_isbs2_btm_dn8 = assign96620_e149240_d_n8;
        locals.var_isbs2_btm_dn9 = assign96620_e149240_d_n9;
        locals.var_isbs2_btm_dn10 = assign96620_e149240_d_n10;
        locals.var_isbs2_btm_dn11 = assign96620_e149240_d_n11;
        locals.var_isbs2_btm_dn14 = assign96620_e149240_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96630_e149249, assign96630_e149249_d_n0, assign96630_e149249_d_n2, assign96630_e149249_d_n4, assign96630_e149249_d_n5, assign96630_e149249_d_n6, assign96630_e149249_d_n7, assign96630_e149249_d_n8, assign96630_e149249_d_n9, assign96630_e149249_d_n10, assign96630_e149249_d_n11, assign96630_e149249_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96630_e149247: f64 = (p.p16 * locals.var_jssw);
        (assign96630_e149247, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96630_e149249;
        locals.var_isbs_sws_dn0 = assign96630_e149249_d_n0;
        locals.var_isbs_sws_dn2 = assign96630_e149249_d_n2;
        locals.var_isbs_sws_dn4 = assign96630_e149249_d_n4;
        locals.var_isbs_sws_dn5 = assign96630_e149249_d_n5;
        locals.var_isbs_sws_dn6 = assign96630_e149249_d_n6;
        locals.var_isbs_sws_dn7 = assign96630_e149249_d_n7;
        locals.var_isbs_sws_dn8 = assign96630_e149249_d_n8;
        locals.var_isbs_sws_dn9 = assign96630_e149249_d_n9;
        locals.var_isbs_sws_dn10 = assign96630_e149249_d_n10;
        locals.var_isbs_sws_dn11 = assign96630_e149249_d_n11;
        locals.var_isbs_sws_dn14 = assign96630_e149249_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96640_e149258, assign96640_e149258_d_n0, assign96640_e149258_d_n2, assign96640_e149258_d_n4, assign96640_e149258_d_n5, assign96640_e149258_d_n6, assign96640_e149258_d_n7, assign96640_e149258_d_n8, assign96640_e149258_d_n9, assign96640_e149258_d_n10, assign96640_e149258_d_n11, assign96640_e149258_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96640_e149256: f64 = (p.p16 * locals.var_jssw2);
        (assign96640_e149256, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96640_e149258;
        locals.var_isbs2_sws_dn0 = assign96640_e149258_d_n0;
        locals.var_isbs2_sws_dn2 = assign96640_e149258_d_n2;
        locals.var_isbs2_sws_dn4 = assign96640_e149258_d_n4;
        locals.var_isbs2_sws_dn5 = assign96640_e149258_d_n5;
        locals.var_isbs2_sws_dn6 = assign96640_e149258_d_n6;
        locals.var_isbs2_sws_dn7 = assign96640_e149258_d_n7;
        locals.var_isbs2_sws_dn8 = assign96640_e149258_d_n8;
        locals.var_isbs2_sws_dn9 = assign96640_e149258_d_n9;
        locals.var_isbs2_sws_dn10 = assign96640_e149258_d_n10;
        locals.var_isbs2_sws_dn11 = assign96640_e149258_d_n11;
        locals.var_isbs2_sws_dn14 = assign96640_e149258_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96650_e149265, assign96650_e149265_d_n0, assign96650_e149265_d_n2, assign96650_e149265_d_n4, assign96650_e149265_d_n5, assign96650_e149265_d_n6, assign96650_e149265_d_n7, assign96650_e149265_d_n8, assign96650_e149265_d_n9, assign96650_e149265_d_n10, assign96650_e149265_d_n11, assign96650_e149265_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96650_e149265;
        locals.var_isbs_swg_dn0 = assign96650_e149265_d_n0;
        locals.var_isbs_swg_dn2 = assign96650_e149265_d_n2;
        locals.var_isbs_swg_dn4 = assign96650_e149265_d_n4;
        locals.var_isbs_swg_dn5 = assign96650_e149265_d_n5;
        locals.var_isbs_swg_dn6 = assign96650_e149265_d_n6;
        locals.var_isbs_swg_dn7 = assign96650_e149265_d_n7;
        locals.var_isbs_swg_dn8 = assign96650_e149265_d_n8;
        locals.var_isbs_swg_dn9 = assign96650_e149265_d_n9;
        locals.var_isbs_swg_dn10 = assign96650_e149265_d_n10;
        locals.var_isbs_swg_dn11 = assign96650_e149265_d_n11;
        locals.var_isbs_swg_dn14 = assign96650_e149265_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96660_e149272, assign96660_e149272_d_n0, assign96660_e149272_d_n2, assign96660_e149272_d_n4, assign96660_e149272_d_n5, assign96660_e149272_d_n6, assign96660_e149272_d_n7, assign96660_e149272_d_n8, assign96660_e149272_d_n9, assign96660_e149272_d_n10, assign96660_e149272_d_n11, assign96660_e149272_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96660_e149272;
        locals.var_isbs2_swg_dn0 = assign96660_e149272_d_n0;
        locals.var_isbs2_swg_dn2 = assign96660_e149272_d_n2;
        locals.var_isbs2_swg_dn4 = assign96660_e149272_d_n4;
        locals.var_isbs2_swg_dn5 = assign96660_e149272_d_n5;
        locals.var_isbs2_swg_dn6 = assign96660_e149272_d_n6;
        locals.var_isbs2_swg_dn7 = assign96660_e149272_d_n7;
        locals.var_isbs2_swg_dn8 = assign96660_e149272_d_n8;
        locals.var_isbs2_swg_dn9 = assign96660_e149272_d_n9;
        locals.var_isbs2_swg_dn10 = assign96660_e149272_d_n10;
        locals.var_isbs2_swg_dn11 = assign96660_e149272_d_n11;
        locals.var_isbs2_swg_dn14 = assign96660_e149272_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96670_e149280, assign96670_e149280_d_n0, assign96670_e149280_d_n2, assign96670_e149280_d_n4, assign96670_e149280_d_n5, assign96670_e149280_d_n6, assign96670_e149280_d_n7, assign96670_e149280_d_n8, assign96670_e149280_d_n9, assign96670_e149280_d_n10, assign96670_e149280_d_n11, assign96670_e149280_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96670_e149276: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96670_e149278: f64 = (assign96670_e149276 + locals.var_isbs_swg);
        (assign96670_e149278, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign96670_e149280;
        locals.var_isbs_dn0 = assign96670_e149280_d_n0;
        locals.var_isbs_dn2 = assign96670_e149280_d_n2;
        locals.var_isbs_dn4 = assign96670_e149280_d_n4;
        locals.var_isbs_dn5 = assign96670_e149280_d_n5;
        locals.var_isbs_dn6 = assign96670_e149280_d_n6;
        locals.var_isbs_dn7 = assign96670_e149280_d_n7;
        locals.var_isbs_dn8 = assign96670_e149280_d_n8;
        locals.var_isbs_dn9 = assign96670_e149280_d_n9;
        locals.var_isbs_dn10 = assign96670_e149280_d_n10;
        locals.var_isbs_dn11 = assign96670_e149280_d_n11;
        locals.var_isbs_dn14 = assign96670_e149280_d_n14;
        locals.var_isbs_rv = 0.0;

        let assign96680_e149283: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96680_e149283;
        locals.var_guard2241_rv = 0.0;

        let (assign96690_e149291, assign96690_e149291_d_n0, assign96690_e149291_d_n2, assign96690_e149291_d_n4, assign96690_e149291_d_n5, assign96690_e149291_d_n6, assign96690_e149291_d_n7, assign96690_e149291_d_n8, assign96690_e149291_d_n9, assign96690_e149291_d_n10, assign96690_e149291_d_n11, assign96690_e149291_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96690_e149289: f64 = (locals.var_isbs + 1e-25);
        (assign96690_e149289, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign96690_e149291;
        locals.var_t3_dn0 = assign96690_e149291_d_n0;
        locals.var_t3_dn2 = assign96690_e149291_d_n2;
        locals.var_t3_dn4 = assign96690_e149291_d_n4;
        locals.var_t3_dn5 = assign96690_e149291_d_n5;
        locals.var_t3_dn6 = assign96690_e149291_d_n6;
        locals.var_t3_dn7 = assign96690_e149291_d_n7;
        locals.var_t3_dn8 = assign96690_e149291_d_n8;
        locals.var_t3_dn9 = assign96690_e149291_d_n9;
        locals.var_t3_dn10 = assign96690_e149291_d_n10;
        locals.var_t3_dn11 = assign96690_e149291_d_n11;
        locals.var_t3_dn14 = assign96690_e149291_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign96700_e149308, assign96700_e149308_d_n0, assign96700_e149308_d_n2, assign96700_e149308_d_n4, assign96700_e149308_d_n5, assign96700_e149308_d_n6, assign96700_e149308_d_n7, assign96700_e149308_d_n8, assign96700_e149308_d_n9, assign96700_e149308_d_n10, assign96700_e149308_d_n11, assign96700_e149308_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96700_e149297: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96700_e149300: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96700_e149302: f64 = (assign96700_e149300 / locals.var_t3);
        let assign96700_e149304: f64 = (assign96700_e149302 + 1.0);
        let assign96700_e149305: f64 = (assign96700_e149304).ln();
        let assign96700_e149306: f64 = (assign96700_e149297 * assign96700_e149305);
        (assign96700_e149306, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign96700_e149308;
        locals.var_vbst_dn0 = assign96700_e149308_d_n0;
        locals.var_vbst_dn2 = assign96700_e149308_d_n2;
        locals.var_vbst_dn4 = assign96700_e149308_d_n4;
        locals.var_vbst_dn5 = assign96700_e149308_d_n5;
        locals.var_vbst_dn6 = assign96700_e149308_d_n6;
        locals.var_vbst_dn7 = assign96700_e149308_d_n7;
        locals.var_vbst_dn8 = assign96700_e149308_d_n8;
        locals.var_vbst_dn9 = assign96700_e149308_d_n9;
        locals.var_vbst_dn10 = assign96700_e149308_d_n10;
        locals.var_vbst_dn11 = assign96700_e149308_d_n11;
        locals.var_vbst_dn14 = assign96700_e149308_d_n14;
        locals.var_vbst_rv = 0.0;

        let (assign96710_e149319, assign96710_e149319_d_n0, assign96710_e149319_d_n2, assign96710_e149319_d_n4, assign96710_e149319_d_n5, assign96710_e149319_d_n6, assign96710_e149319_d_n7, assign96710_e149319_d_n8, assign96710_e149319_d_n9, assign96710_e149319_d_n10, assign96710_e149319_d_n11, assign96710_e149319_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96710_e149314: f64 = (locals.var_tratio - 1.0);
        let assign96710_e149316: f64 = (assign96710_e149314 * p.p535);
        let assign96710_e149317: f64 = (assign96710_e149316).exp();
        (assign96710_e149317, (assign96710_e149317 * (locals.var_tratio_dn0 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn2 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn4 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn5 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn6 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn7 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn8 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn9 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn10 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn11 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign96710_e149319;
        locals.var_exptemps_dn0 = assign96710_e149319_d_n0;
        locals.var_exptemps_dn2 = assign96710_e149319_d_n2;
        locals.var_exptemps_dn4 = assign96710_e149319_d_n4;
        locals.var_exptemps_dn5 = assign96710_e149319_d_n5;
        locals.var_exptemps_dn6 = assign96710_e149319_d_n6;
        locals.var_exptemps_dn7 = assign96710_e149319_d_n7;
        locals.var_exptemps_dn8 = assign96710_e149319_d_n8;
        locals.var_exptemps_dn9 = assign96710_e149319_d_n9;
        locals.var_exptemps_dn10 = assign96710_e149319_d_n10;
        locals.var_exptemps_dn11 = assign96710_e149319_d_n11;
        locals.var_exptemps_dn14 = assign96710_e149319_d_n14;
        locals.var_exptemps_rv = 0.0;

        let (assign96720_e149329, assign96720_e149329_d_n0, assign96720_e149329_d_n2, assign96720_e149329_d_n4, assign96720_e149329_d_n5, assign96720_e149329_d_n6, assign96720_e149329_d_n7, assign96720_e149329_d_n8, assign96720_e149329_d_n9, assign96720_e149329_d_n10, assign96720_e149329_d_n11, assign96720_e149329_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96720_e149326: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96720_e149327: f64 = (1.0 / assign96720_e149326);
        (assign96720_e149327, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign96720_e149329;
        locals.var_jd_nvtm_invs_dn0 = assign96720_e149329_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96720_e149329_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96720_e149329_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96720_e149329_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96720_e149329_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96720_e149329_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96720_e149329_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96720_e149329_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96720_e149329_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign96720_e149329_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign96720_e149329_d_n14;
        locals.var_jd_nvtm_invs_rv = 0.0;

        let (assign96730_e149338, assign96730_e149338_d_n0, assign96730_e149338_d_n2, assign96730_e149338_d_n4, assign96730_e149338_d_n5, assign96730_e149338_d_n6, assign96730_e149338_d_n7, assign96730_e149338_d_n8, assign96730_e149338_d_n9, assign96730_e149338_d_n10, assign96730_e149338_d_n11, assign96730_e149338_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96730_e149335: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96730_e149336: f64 = (assign96730_e149335).exp();
        (assign96730_e149336, (assign96730_e149336 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96730_e149336 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96730_e149336 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96730_e149336 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96730_e149336 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96730_e149336 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96730_e149336 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96730_e149336 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96730_e149336 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96730_e149336 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign96730_e149336 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign96730_e149338;
        locals.var_jd_expcs_dn0 = assign96730_e149338_d_n0;
        locals.var_jd_expcs_dn2 = assign96730_e149338_d_n2;
        locals.var_jd_expcs_dn4 = assign96730_e149338_d_n4;
        locals.var_jd_expcs_dn5 = assign96730_e149338_d_n5;
        locals.var_jd_expcs_dn6 = assign96730_e149338_d_n6;
        locals.var_jd_expcs_dn7 = assign96730_e149338_d_n7;
        locals.var_jd_expcs_dn8 = assign96730_e149338_d_n8;
        locals.var_jd_expcs_dn9 = assign96730_e149338_d_n9;
        locals.var_jd_expcs_dn10 = assign96730_e149338_d_n10;
        locals.var_jd_expcs_dn11 = assign96730_e149338_d_n11;
        locals.var_jd_expcs_dn14 = assign96730_e149338_d_n14;
        locals.var_jd_expcs_rv = 0.0;

        let (assign96740_e149350, assign96740_e149350_d_n0, assign96740_e149350_d_n2, assign96740_e149350_d_n4, assign96740_e149350_d_n5, assign96740_e149350_d_n6, assign96740_e149350_d_n7, assign96740_e149350_d_n8, assign96740_e149350_d_n9, assign96740_e149350_d_n10, assign96740_e149350_d_n11, assign96740_e149350_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96740_e149342: f64 = (p.p500 * p.p13);
        let assign96740_e149346: f64 = (p.p481 * locals.var_tdiff);
        let assign96740_e149347: f64 = (1.0 + assign96740_e149346);
        let assign96740_e149348: f64 = (assign96740_e149342 * assign96740_e149347);
        (assign96740_e149348, (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn0)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn2)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn4)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn5)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn6)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn7)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn8)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn9)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn10)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn11)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96740_e149350;
        locals.var_czbd_dn0 = assign96740_e149350_d_n0;
        locals.var_czbd_dn2 = assign96740_e149350_d_n2;
        locals.var_czbd_dn4 = assign96740_e149350_d_n4;
        locals.var_czbd_dn5 = assign96740_e149350_d_n5;
        locals.var_czbd_dn6 = assign96740_e149350_d_n6;
        locals.var_czbd_dn7 = assign96740_e149350_d_n7;
        locals.var_czbd_dn8 = assign96740_e149350_d_n8;
        locals.var_czbd_dn9 = assign96740_e149350_d_n9;
        locals.var_czbd_dn10 = assign96740_e149350_d_n10;
        locals.var_czbd_dn11 = assign96740_e149350_d_n11;
        locals.var_czbd_dn14 = assign96740_e149350_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign96750_e149353: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96750_e149353;
        locals.var_guard2242_rv = 0.0;

        let (assign96760_e149369, assign96760_e149369_d_n0, assign96760_e149369_d_n2, assign96760_e149369_d_n4, assign96760_e149369_d_n5, assign96760_e149369_d_n6, assign96760_e149369_d_n7, assign96760_e149369_d_n8, assign96760_e149369_d_n9, assign96760_e149369_d_n10, assign96760_e149369_d_n11, assign96760_e149369_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 != 0.0)) {
        let assign96760_e149360: f64 = (p.p15 - locals.var_weff_nf);
        let assign96760_e149361: f64 = (p.p501 * assign96760_e149360);
        let assign96760_e149365: f64 = (p.p483 * locals.var_tdiff);
        let assign96760_e149366: f64 = (1.0 + assign96760_e149365);
        let assign96760_e149367: f64 = (assign96760_e149361 * assign96760_e149366);
        (assign96760_e149367, (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn0)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn2)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn4)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn5)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn6)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn7)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn8)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn9)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn10)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn11)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96760_e149369;
        locals.var_czbdsw_dn0 = assign96760_e149369_d_n0;
        locals.var_czbdsw_dn2 = assign96760_e149369_d_n2;
        locals.var_czbdsw_dn4 = assign96760_e149369_d_n4;
        locals.var_czbdsw_dn5 = assign96760_e149369_d_n5;
        locals.var_czbdsw_dn6 = assign96760_e149369_d_n6;
        locals.var_czbdsw_dn7 = assign96760_e149369_d_n7;
        locals.var_czbdsw_dn8 = assign96760_e149369_d_n8;
        locals.var_czbdsw_dn9 = assign96760_e149369_d_n9;
        locals.var_czbdsw_dn10 = assign96760_e149369_d_n10;
        locals.var_czbdsw_dn11 = assign96760_e149369_d_n11;
        locals.var_czbdsw_dn14 = assign96760_e149369_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign96770_e149383, assign96770_e149383_d_n0, assign96770_e149383_d_n2, assign96770_e149383_d_n4, assign96770_e149383_d_n5, assign96770_e149383_d_n6, assign96770_e149383_d_n7, assign96770_e149383_d_n8, assign96770_e149383_d_n9, assign96770_e149383_d_n10, assign96770_e149383_d_n11, assign96770_e149383_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 != 0.0)) {
        let assign96770_e149375: f64 = (p.p502 * locals.var_weff_nf);
        let assign96770_e149379: f64 = (p.p485 * locals.var_tdiff);
        let assign96770_e149380: f64 = (1.0 + assign96770_e149379);
        let assign96770_e149381: f64 = (assign96770_e149375 * assign96770_e149380);
        (assign96770_e149381, (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn0)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn2)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn4)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn5)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn6)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn7)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn8)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn9)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn10)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn11)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96770_e149383;
        locals.var_czbdswg_dn0 = assign96770_e149383_d_n0;
        locals.var_czbdswg_dn2 = assign96770_e149383_d_n2;
        locals.var_czbdswg_dn4 = assign96770_e149383_d_n4;
        locals.var_czbdswg_dn5 = assign96770_e149383_d_n5;
        locals.var_czbdswg_dn6 = assign96770_e149383_d_n6;
        locals.var_czbdswg_dn7 = assign96770_e149383_d_n7;
        locals.var_czbdswg_dn8 = assign96770_e149383_d_n8;
        locals.var_czbdswg_dn9 = assign96770_e149383_d_n9;
        locals.var_czbdswg_dn10 = assign96770_e149383_d_n10;
        locals.var_czbdswg_dn11 = assign96770_e149383_d_n11;
        locals.var_czbdswg_dn14 = assign96770_e149383_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign96780_e149390, assign96780_e149390_d_n0, assign96780_e149390_d_n2, assign96780_e149390_d_n4, assign96780_e149390_d_n5, assign96780_e149390_d_n6, assign96780_e149390_d_n7, assign96780_e149390_d_n8, assign96780_e149390_d_n9, assign96780_e149390_d_n10, assign96780_e149390_d_n11, assign96780_e149390_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96780_e149390;
        locals.var_czbdsw_dn0 = assign96780_e149390_d_n0;
        locals.var_czbdsw_dn2 = assign96780_e149390_d_n2;
        locals.var_czbdsw_dn4 = assign96780_e149390_d_n4;
        locals.var_czbdsw_dn5 = assign96780_e149390_d_n5;
        locals.var_czbdsw_dn6 = assign96780_e149390_d_n6;
        locals.var_czbdsw_dn7 = assign96780_e149390_d_n7;
        locals.var_czbdsw_dn8 = assign96780_e149390_d_n8;
        locals.var_czbdsw_dn9 = assign96780_e149390_d_n9;
        locals.var_czbdsw_dn10 = assign96780_e149390_d_n10;
        locals.var_czbdsw_dn11 = assign96780_e149390_d_n11;
        locals.var_czbdsw_dn14 = assign96780_e149390_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign96790_e149405, assign96790_e149405_d_n0, assign96790_e149405_d_n2, assign96790_e149405_d_n4, assign96790_e149405_d_n5, assign96790_e149405_d_n6, assign96790_e149405_d_n7, assign96790_e149405_d_n8, assign96790_e149405_d_n9, assign96790_e149405_d_n10, assign96790_e149405_d_n11, assign96790_e149405_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 == 0.0)) {
        let assign96790_e149397: f64 = (p.p502 * p.p15);
        let assign96790_e149401: f64 = (p.p485 * locals.var_tdiff);
        let assign96790_e149402: f64 = (1.0 + assign96790_e149401);
        let assign96790_e149403: f64 = (assign96790_e149397 * assign96790_e149402);
        (assign96790_e149403, (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn0)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn2)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn4)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn5)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn6)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn7)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn8)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn9)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn10)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn11)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96790_e149405;
        locals.var_czbdswg_dn0 = assign96790_e149405_d_n0;
        locals.var_czbdswg_dn2 = assign96790_e149405_d_n2;
        locals.var_czbdswg_dn4 = assign96790_e149405_d_n4;
        locals.var_czbdswg_dn5 = assign96790_e149405_d_n5;
        locals.var_czbdswg_dn6 = assign96790_e149405_d_n6;
        locals.var_czbdswg_dn7 = assign96790_e149405_d_n7;
        locals.var_czbdswg_dn8 = assign96790_e149405_d_n8;
        locals.var_czbdswg_dn9 = assign96790_e149405_d_n9;
        locals.var_czbdswg_dn10 = assign96790_e149405_d_n10;
        locals.var_czbdswg_dn11 = assign96790_e149405_d_n11;
        locals.var_czbdswg_dn14 = assign96790_e149405_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let assign96800_e149408: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96800_e149408;
        locals.var_guard2243_rv = 0.0;

        let (assign96810_e149414, assign96810_e149414_d_n0, assign96810_e149414_d_n2, assign96810_e149414_d_n4, assign96810_e149414_d_n5, assign96810_e149414_d_n6, assign96810_e149414_d_n7, assign96810_e149414_d_n8, assign96810_e149414_d_n9, assign96810_e149414_d_n10, assign96810_e149414_d_n11, assign96810_e149414_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96810_e149414;
        locals.var_czbd_dn0 = assign96810_e149414_d_n0;
        locals.var_czbd_dn2 = assign96810_e149414_d_n2;
        locals.var_czbd_dn4 = assign96810_e149414_d_n4;
        locals.var_czbd_dn5 = assign96810_e149414_d_n5;
        locals.var_czbd_dn6 = assign96810_e149414_d_n6;
        locals.var_czbd_dn7 = assign96810_e149414_d_n7;
        locals.var_czbd_dn8 = assign96810_e149414_d_n8;
        locals.var_czbd_dn9 = assign96810_e149414_d_n9;
        locals.var_czbd_dn10 = assign96810_e149414_d_n10;
        locals.var_czbd_dn11 = assign96810_e149414_d_n11;
        locals.var_czbd_dn14 = assign96810_e149414_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign96820_e149417: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96820_e149417;
        locals.var_guard2244_rv = 0.0;

        let (assign96830_e149423, assign96830_e149423_d_n0, assign96830_e149423_d_n2, assign96830_e149423_d_n4, assign96830_e149423_d_n5, assign96830_e149423_d_n6, assign96830_e149423_d_n7, assign96830_e149423_d_n8, assign96830_e149423_d_n9, assign96830_e149423_d_n10, assign96830_e149423_d_n11, assign96830_e149423_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2244 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96830_e149423;
        locals.var_czbdsw_dn0 = assign96830_e149423_d_n0;
        locals.var_czbdsw_dn2 = assign96830_e149423_d_n2;
        locals.var_czbdsw_dn4 = assign96830_e149423_d_n4;
        locals.var_czbdsw_dn5 = assign96830_e149423_d_n5;
        locals.var_czbdsw_dn6 = assign96830_e149423_d_n6;
        locals.var_czbdsw_dn7 = assign96830_e149423_d_n7;
        locals.var_czbdsw_dn8 = assign96830_e149423_d_n8;
        locals.var_czbdsw_dn9 = assign96830_e149423_d_n9;
        locals.var_czbdsw_dn10 = assign96830_e149423_d_n10;
        locals.var_czbdsw_dn11 = assign96830_e149423_d_n11;
        locals.var_czbdsw_dn14 = assign96830_e149423_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let assign96840_e149426: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96840_e149426;
        locals.var_guard2245_rv = 0.0;

        let (assign96850_e149432, assign96850_e149432_d_n0, assign96850_e149432_d_n2, assign96850_e149432_d_n4, assign96850_e149432_d_n5, assign96850_e149432_d_n6, assign96850_e149432_d_n7, assign96850_e149432_d_n8, assign96850_e149432_d_n9, assign96850_e149432_d_n10, assign96850_e149432_d_n11, assign96850_e149432_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96850_e149432;
        locals.var_czbdswg_dn0 = assign96850_e149432_d_n0;
        locals.var_czbdswg_dn2 = assign96850_e149432_d_n2;
        locals.var_czbdswg_dn4 = assign96850_e149432_d_n4;
        locals.var_czbdswg_dn5 = assign96850_e149432_d_n5;
        locals.var_czbdswg_dn6 = assign96850_e149432_d_n6;
        locals.var_czbdswg_dn7 = assign96850_e149432_d_n7;
        locals.var_czbdswg_dn8 = assign96850_e149432_d_n8;
        locals.var_czbdswg_dn9 = assign96850_e149432_d_n9;
        locals.var_czbdswg_dn10 = assign96850_e149432_d_n10;
        locals.var_czbdswg_dn11 = assign96850_e149432_d_n11;
        locals.var_czbdswg_dn14 = assign96850_e149432_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign96860_e149440, assign96860_e149440_d_n0, assign96860_e149440_d_n2, assign96860_e149440_d_n4, assign96860_e149440_d_n5, assign96860_e149440_d_n6, assign96860_e149440_d_n7, assign96860_e149440_d_n8, assign96860_e149440_d_n9, assign96860_e149440_d_n10, assign96860_e149440_d_n11, assign96860_e149440_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96860_e149437: f64 = (p.p487 * locals.var_tdiff);
        let assign96860_e149438: f64 = (p.p506 - assign96860_e149437);
        (assign96860_e149438, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96860_e149440;
        locals.var_pzbd_dn0 = assign96860_e149440_d_n0;
        locals.var_pzbd_dn2 = assign96860_e149440_d_n2;
        locals.var_pzbd_dn4 = assign96860_e149440_d_n4;
        locals.var_pzbd_dn5 = assign96860_e149440_d_n5;
        locals.var_pzbd_dn6 = assign96860_e149440_d_n6;
        locals.var_pzbd_dn7 = assign96860_e149440_d_n7;
        locals.var_pzbd_dn8 = assign96860_e149440_d_n8;
        locals.var_pzbd_dn9 = assign96860_e149440_d_n9;
        locals.var_pzbd_dn10 = assign96860_e149440_d_n10;
        locals.var_pzbd_dn11 = assign96860_e149440_d_n11;
        locals.var_pzbd_dn14 = assign96860_e149440_d_n14;
        locals.var_pzbd_rv = 0.0;

        let (assign96870_e149448, assign96870_e149448_d_n0, assign96870_e149448_d_n2, assign96870_e149448_d_n4, assign96870_e149448_d_n5, assign96870_e149448_d_n6, assign96870_e149448_d_n7, assign96870_e149448_d_n8, assign96870_e149448_d_n9, assign96870_e149448_d_n10, assign96870_e149448_d_n11, assign96870_e149448_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96870_e149445: f64 = (p.p489 * locals.var_tdiff);
        let assign96870_e149446: f64 = (p.p507 - assign96870_e149445);
        (assign96870_e149446, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96870_e149448;
        locals.var_pzbdsw_dn0 = assign96870_e149448_d_n0;
        locals.var_pzbdsw_dn2 = assign96870_e149448_d_n2;
        locals.var_pzbdsw_dn4 = assign96870_e149448_d_n4;
        locals.var_pzbdsw_dn5 = assign96870_e149448_d_n5;
        locals.var_pzbdsw_dn6 = assign96870_e149448_d_n6;
        locals.var_pzbdsw_dn7 = assign96870_e149448_d_n7;
        locals.var_pzbdsw_dn8 = assign96870_e149448_d_n8;
        locals.var_pzbdsw_dn9 = assign96870_e149448_d_n9;
        locals.var_pzbdsw_dn10 = assign96870_e149448_d_n10;
        locals.var_pzbdsw_dn11 = assign96870_e149448_d_n11;
        locals.var_pzbdsw_dn14 = assign96870_e149448_d_n14;
        locals.var_pzbdsw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_376(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign96880_e149456, assign96880_e149456_d_n0, assign96880_e149456_d_n2, assign96880_e149456_d_n4, assign96880_e149456_d_n5, assign96880_e149456_d_n6, assign96880_e149456_d_n7, assign96880_e149456_d_n8, assign96880_e149456_d_n9, assign96880_e149456_d_n10, assign96880_e149456_d_n11, assign96880_e149456_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96880_e149453: f64 = (p.p491 * locals.var_tdiff);
        let assign96880_e149454: f64 = (p.p508 - assign96880_e149453);
        (assign96880_e149454, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96880_e149456;
        locals.var_pzbdswg_dn0 = assign96880_e149456_d_n0;
        locals.var_pzbdswg_dn2 = assign96880_e149456_d_n2;
        locals.var_pzbdswg_dn4 = assign96880_e149456_d_n4;
        locals.var_pzbdswg_dn5 = assign96880_e149456_d_n5;
        locals.var_pzbdswg_dn6 = assign96880_e149456_d_n6;
        locals.var_pzbdswg_dn7 = assign96880_e149456_d_n7;
        locals.var_pzbdswg_dn8 = assign96880_e149456_d_n8;
        locals.var_pzbdswg_dn9 = assign96880_e149456_d_n9;
        locals.var_pzbdswg_dn10 = assign96880_e149456_d_n10;
        locals.var_pzbdswg_dn11 = assign96880_e149456_d_n11;
        locals.var_pzbdswg_dn14 = assign96880_e149456_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let assign96890_e149463: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96890_e149463;
        locals.var_guard2246_rv = 0.0;

        let (assign96900_e149469, assign96900_e149469_d_n0, assign96900_e149469_d_n2, assign96900_e149469_d_n4, assign96900_e149469_d_n5, assign96900_e149469_d_n6, assign96900_e149469_d_n7, assign96900_e149469_d_n8, assign96900_e149469_d_n9, assign96900_e149469_d_n10, assign96900_e149469_d_n11, assign96900_e149469_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96900_e149469;
        locals.var_pzbd_dn0 = assign96900_e149469_d_n0;
        locals.var_pzbd_dn2 = assign96900_e149469_d_n2;
        locals.var_pzbd_dn4 = assign96900_e149469_d_n4;
        locals.var_pzbd_dn5 = assign96900_e149469_d_n5;
        locals.var_pzbd_dn6 = assign96900_e149469_d_n6;
        locals.var_pzbd_dn7 = assign96900_e149469_d_n7;
        locals.var_pzbd_dn8 = assign96900_e149469_d_n8;
        locals.var_pzbd_dn9 = assign96900_e149469_d_n9;
        locals.var_pzbd_dn10 = assign96900_e149469_d_n10;
        locals.var_pzbd_dn11 = assign96900_e149469_d_n11;
        locals.var_pzbd_dn14 = assign96900_e149469_d_n14;
        locals.var_pzbd_rv = 0.0;

        let assign96910_e149476: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96910_e149476;
        locals.var_guard2247_rv = 0.0;

        let (assign96920_e149482, assign96920_e149482_d_n0, assign96920_e149482_d_n2, assign96920_e149482_d_n4, assign96920_e149482_d_n5, assign96920_e149482_d_n6, assign96920_e149482_d_n7, assign96920_e149482_d_n8, assign96920_e149482_d_n9, assign96920_e149482_d_n10, assign96920_e149482_d_n11, assign96920_e149482_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2247 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96920_e149482;
        locals.var_pzbdsw_dn0 = assign96920_e149482_d_n0;
        locals.var_pzbdsw_dn2 = assign96920_e149482_d_n2;
        locals.var_pzbdsw_dn4 = assign96920_e149482_d_n4;
        locals.var_pzbdsw_dn5 = assign96920_e149482_d_n5;
        locals.var_pzbdsw_dn6 = assign96920_e149482_d_n6;
        locals.var_pzbdsw_dn7 = assign96920_e149482_d_n7;
        locals.var_pzbdsw_dn8 = assign96920_e149482_d_n8;
        locals.var_pzbdsw_dn9 = assign96920_e149482_d_n9;
        locals.var_pzbdsw_dn10 = assign96920_e149482_d_n10;
        locals.var_pzbdsw_dn11 = assign96920_e149482_d_n11;
        locals.var_pzbdsw_dn14 = assign96920_e149482_d_n14;
        locals.var_pzbdsw_rv = 0.0;

        let assign96930_e149489: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96930_e149489;
        locals.var_guard2248_rv = 0.0;

        let (assign96940_e149495, assign96940_e149495_d_n0, assign96940_e149495_d_n2, assign96940_e149495_d_n4, assign96940_e149495_d_n5, assign96940_e149495_d_n6, assign96940_e149495_d_n7, assign96940_e149495_d_n8, assign96940_e149495_d_n9, assign96940_e149495_d_n10, assign96940_e149495_d_n11, assign96940_e149495_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96940_e149495;
        locals.var_pzbdswg_dn0 = assign96940_e149495_d_n0;
        locals.var_pzbdswg_dn2 = assign96940_e149495_d_n2;
        locals.var_pzbdswg_dn4 = assign96940_e149495_d_n4;
        locals.var_pzbdswg_dn5 = assign96940_e149495_d_n5;
        locals.var_pzbdswg_dn6 = assign96940_e149495_d_n6;
        locals.var_pzbdswg_dn7 = assign96940_e149495_d_n7;
        locals.var_pzbdswg_dn8 = assign96940_e149495_d_n8;
        locals.var_pzbdswg_dn9 = assign96940_e149495_d_n9;
        locals.var_pzbdswg_dn10 = assign96940_e149495_d_n10;
        locals.var_pzbdswg_dn11 = assign96940_e149495_d_n11;
        locals.var_pzbdswg_dn14 = assign96940_e149495_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let (assign96950_e149507, assign96950_e149507_d_n0, assign96950_e149507_d_n2, assign96950_e149507_d_n4, assign96950_e149507_d_n5, assign96950_e149507_d_n6, assign96950_e149507_d_n7, assign96950_e149507_d_n8, assign96950_e149507_d_n9, assign96950_e149507_d_n10, assign96950_e149507_d_n11, assign96950_e149507_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96950_e149499: f64 = (p.p523 * p.p14);
        let assign96950_e149503: f64 = (p.p482 * locals.var_tdiff);
        let assign96950_e149504: f64 = (1.0 + assign96950_e149503);
        let assign96950_e149505: f64 = (assign96950_e149499 * assign96950_e149504);
        (assign96950_e149505, (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn0)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn2)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn4)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn5)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn6)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn7)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn8)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn9)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn10)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn11)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign96950_e149507;
        locals.var_czbs_dn0 = assign96950_e149507_d_n0;
        locals.var_czbs_dn2 = assign96950_e149507_d_n2;
        locals.var_czbs_dn4 = assign96950_e149507_d_n4;
        locals.var_czbs_dn5 = assign96950_e149507_d_n5;
        locals.var_czbs_dn6 = assign96950_e149507_d_n6;
        locals.var_czbs_dn7 = assign96950_e149507_d_n7;
        locals.var_czbs_dn8 = assign96950_e149507_d_n8;
        locals.var_czbs_dn9 = assign96950_e149507_d_n9;
        locals.var_czbs_dn10 = assign96950_e149507_d_n10;
        locals.var_czbs_dn11 = assign96950_e149507_d_n11;
        locals.var_czbs_dn14 = assign96950_e149507_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign96960_e149510: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign96960_e149510;
        locals.var_guard2249_rv = 0.0;

        let (assign96970_e149526, assign96970_e149526_d_n0, assign96970_e149526_d_n2, assign96970_e149526_d_n4, assign96970_e149526_d_n5, assign96970_e149526_d_n6, assign96970_e149526_d_n7, assign96970_e149526_d_n8, assign96970_e149526_d_n9, assign96970_e149526_d_n10, assign96970_e149526_d_n11, assign96970_e149526_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 != 0.0)) {
        let assign96970_e149517: f64 = (p.p16 - locals.var_weff_nf);
        let assign96970_e149518: f64 = (p.p524 * assign96970_e149517);
        let assign96970_e149522: f64 = (p.p484 * locals.var_tdiff);
        let assign96970_e149523: f64 = (1.0 + assign96970_e149522);
        let assign96970_e149524: f64 = (assign96970_e149518 * assign96970_e149523);
        (assign96970_e149524, (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn0)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn2)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn4)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn5)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn6)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn7)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn8)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn9)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn10)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn11)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96970_e149526;
        locals.var_czbssw_dn0 = assign96970_e149526_d_n0;
        locals.var_czbssw_dn2 = assign96970_e149526_d_n2;
        locals.var_czbssw_dn4 = assign96970_e149526_d_n4;
        locals.var_czbssw_dn5 = assign96970_e149526_d_n5;
        locals.var_czbssw_dn6 = assign96970_e149526_d_n6;
        locals.var_czbssw_dn7 = assign96970_e149526_d_n7;
        locals.var_czbssw_dn8 = assign96970_e149526_d_n8;
        locals.var_czbssw_dn9 = assign96970_e149526_d_n9;
        locals.var_czbssw_dn10 = assign96970_e149526_d_n10;
        locals.var_czbssw_dn11 = assign96970_e149526_d_n11;
        locals.var_czbssw_dn14 = assign96970_e149526_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign96980_e149540, assign96980_e149540_d_n0, assign96980_e149540_d_n2, assign96980_e149540_d_n4, assign96980_e149540_d_n5, assign96980_e149540_d_n6, assign96980_e149540_d_n7, assign96980_e149540_d_n8, assign96980_e149540_d_n9, assign96980_e149540_d_n10, assign96980_e149540_d_n11, assign96980_e149540_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 != 0.0)) {
        let assign96980_e149532: f64 = (p.p525 * locals.var_weff_nf);
        let assign96980_e149536: f64 = (p.p486 * locals.var_tdiff);
        let assign96980_e149537: f64 = (1.0 + assign96980_e149536);
        let assign96980_e149538: f64 = (assign96980_e149532 * assign96980_e149537);
        (assign96980_e149538, (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn0)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn2)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn4)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn5)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn6)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn7)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn8)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn9)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn10)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn11)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign96980_e149540;
        locals.var_czbsswg_dn0 = assign96980_e149540_d_n0;
        locals.var_czbsswg_dn2 = assign96980_e149540_d_n2;
        locals.var_czbsswg_dn4 = assign96980_e149540_d_n4;
        locals.var_czbsswg_dn5 = assign96980_e149540_d_n5;
        locals.var_czbsswg_dn6 = assign96980_e149540_d_n6;
        locals.var_czbsswg_dn7 = assign96980_e149540_d_n7;
        locals.var_czbsswg_dn8 = assign96980_e149540_d_n8;
        locals.var_czbsswg_dn9 = assign96980_e149540_d_n9;
        locals.var_czbsswg_dn10 = assign96980_e149540_d_n10;
        locals.var_czbsswg_dn11 = assign96980_e149540_d_n11;
        locals.var_czbsswg_dn14 = assign96980_e149540_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign96990_e149547, assign96990_e149547_d_n0, assign96990_e149547_d_n2, assign96990_e149547_d_n4, assign96990_e149547_d_n5, assign96990_e149547_d_n6, assign96990_e149547_d_n7, assign96990_e149547_d_n8, assign96990_e149547_d_n9, assign96990_e149547_d_n10, assign96990_e149547_d_n11, assign96990_e149547_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96990_e149547;
        locals.var_czbssw_dn0 = assign96990_e149547_d_n0;
        locals.var_czbssw_dn2 = assign96990_e149547_d_n2;
        locals.var_czbssw_dn4 = assign96990_e149547_d_n4;
        locals.var_czbssw_dn5 = assign96990_e149547_d_n5;
        locals.var_czbssw_dn6 = assign96990_e149547_d_n6;
        locals.var_czbssw_dn7 = assign96990_e149547_d_n7;
        locals.var_czbssw_dn8 = assign96990_e149547_d_n8;
        locals.var_czbssw_dn9 = assign96990_e149547_d_n9;
        locals.var_czbssw_dn10 = assign96990_e149547_d_n10;
        locals.var_czbssw_dn11 = assign96990_e149547_d_n11;
        locals.var_czbssw_dn14 = assign96990_e149547_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign97000_e149562, assign97000_e149562_d_n0, assign97000_e149562_d_n2, assign97000_e149562_d_n4, assign97000_e149562_d_n5, assign97000_e149562_d_n6, assign97000_e149562_d_n7, assign97000_e149562_d_n8, assign97000_e149562_d_n9, assign97000_e149562_d_n10, assign97000_e149562_d_n11, assign97000_e149562_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 == 0.0)) {
        let assign97000_e149554: f64 = (p.p525 * p.p16);
        let assign97000_e149558: f64 = (p.p486 * locals.var_tdiff);
        let assign97000_e149559: f64 = (1.0 + assign97000_e149558);
        let assign97000_e149560: f64 = (assign97000_e149554 * assign97000_e149559);
        (assign97000_e149560, (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn0)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn2)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn4)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn5)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn6)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn7)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn8)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn9)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn10)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn11)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97000_e149562;
        locals.var_czbsswg_dn0 = assign97000_e149562_d_n0;
        locals.var_czbsswg_dn2 = assign97000_e149562_d_n2;
        locals.var_czbsswg_dn4 = assign97000_e149562_d_n4;
        locals.var_czbsswg_dn5 = assign97000_e149562_d_n5;
        locals.var_czbsswg_dn6 = assign97000_e149562_d_n6;
        locals.var_czbsswg_dn7 = assign97000_e149562_d_n7;
        locals.var_czbsswg_dn8 = assign97000_e149562_d_n8;
        locals.var_czbsswg_dn9 = assign97000_e149562_d_n9;
        locals.var_czbsswg_dn10 = assign97000_e149562_d_n10;
        locals.var_czbsswg_dn11 = assign97000_e149562_d_n11;
        locals.var_czbsswg_dn14 = assign97000_e149562_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let assign97010_e149565: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign97010_e149565;
        locals.var_guard2250_rv = 0.0;

        let (assign97020_e149571, assign97020_e149571_d_n0, assign97020_e149571_d_n2, assign97020_e149571_d_n4, assign97020_e149571_d_n5, assign97020_e149571_d_n6, assign97020_e149571_d_n7, assign97020_e149571_d_n8, assign97020_e149571_d_n9, assign97020_e149571_d_n10, assign97020_e149571_d_n11, assign97020_e149571_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign97020_e149571;
        locals.var_czbs_dn0 = assign97020_e149571_d_n0;
        locals.var_czbs_dn2 = assign97020_e149571_d_n2;
        locals.var_czbs_dn4 = assign97020_e149571_d_n4;
        locals.var_czbs_dn5 = assign97020_e149571_d_n5;
        locals.var_czbs_dn6 = assign97020_e149571_d_n6;
        locals.var_czbs_dn7 = assign97020_e149571_d_n7;
        locals.var_czbs_dn8 = assign97020_e149571_d_n8;
        locals.var_czbs_dn9 = assign97020_e149571_d_n9;
        locals.var_czbs_dn10 = assign97020_e149571_d_n10;
        locals.var_czbs_dn11 = assign97020_e149571_d_n11;
        locals.var_czbs_dn14 = assign97020_e149571_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign97030_e149574: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign97030_e149574;
        locals.var_guard2251_rv = 0.0;

        let (assign97040_e149580, assign97040_e149580_d_n0, assign97040_e149580_d_n2, assign97040_e149580_d_n4, assign97040_e149580_d_n5, assign97040_e149580_d_n6, assign97040_e149580_d_n7, assign97040_e149580_d_n8, assign97040_e149580_d_n9, assign97040_e149580_d_n10, assign97040_e149580_d_n11, assign97040_e149580_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2251 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97040_e149580;
        locals.var_czbssw_dn0 = assign97040_e149580_d_n0;
        locals.var_czbssw_dn2 = assign97040_e149580_d_n2;
        locals.var_czbssw_dn4 = assign97040_e149580_d_n4;
        locals.var_czbssw_dn5 = assign97040_e149580_d_n5;
        locals.var_czbssw_dn6 = assign97040_e149580_d_n6;
        locals.var_czbssw_dn7 = assign97040_e149580_d_n7;
        locals.var_czbssw_dn8 = assign97040_e149580_d_n8;
        locals.var_czbssw_dn9 = assign97040_e149580_d_n9;
        locals.var_czbssw_dn10 = assign97040_e149580_d_n10;
        locals.var_czbssw_dn11 = assign97040_e149580_d_n11;
        locals.var_czbssw_dn14 = assign97040_e149580_d_n14;
        locals.var_czbssw_rv = 0.0;

        let assign97050_e149583: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97050_e149583;
        locals.var_guard2252_rv = 0.0;

        let (assign97060_e149589, assign97060_e149589_d_n0, assign97060_e149589_d_n2, assign97060_e149589_d_n4, assign97060_e149589_d_n5, assign97060_e149589_d_n6, assign97060_e149589_d_n7, assign97060_e149589_d_n8, assign97060_e149589_d_n9, assign97060_e149589_d_n10, assign97060_e149589_d_n11, assign97060_e149589_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97060_e149589;
        locals.var_czbsswg_dn0 = assign97060_e149589_d_n0;
        locals.var_czbsswg_dn2 = assign97060_e149589_d_n2;
        locals.var_czbsswg_dn4 = assign97060_e149589_d_n4;
        locals.var_czbsswg_dn5 = assign97060_e149589_d_n5;
        locals.var_czbsswg_dn6 = assign97060_e149589_d_n6;
        locals.var_czbsswg_dn7 = assign97060_e149589_d_n7;
        locals.var_czbsswg_dn8 = assign97060_e149589_d_n8;
        locals.var_czbsswg_dn9 = assign97060_e149589_d_n9;
        locals.var_czbsswg_dn10 = assign97060_e149589_d_n10;
        locals.var_czbsswg_dn11 = assign97060_e149589_d_n11;
        locals.var_czbsswg_dn14 = assign97060_e149589_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign97070_e149597, assign97070_e149597_d_n0, assign97070_e149597_d_n2, assign97070_e149597_d_n4, assign97070_e149597_d_n5, assign97070_e149597_d_n6, assign97070_e149597_d_n7, assign97070_e149597_d_n8, assign97070_e149597_d_n9, assign97070_e149597_d_n10, assign97070_e149597_d_n11, assign97070_e149597_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97070_e149594: f64 = (p.p488 * locals.var_tdiff);
        let assign97070_e149595: f64 = (p.p529 - assign97070_e149594);
        (assign97070_e149595, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97070_e149597;
        locals.var_pzbs_dn0 = assign97070_e149597_d_n0;
        locals.var_pzbs_dn2 = assign97070_e149597_d_n2;
        locals.var_pzbs_dn4 = assign97070_e149597_d_n4;
        locals.var_pzbs_dn5 = assign97070_e149597_d_n5;
        locals.var_pzbs_dn6 = assign97070_e149597_d_n6;
        locals.var_pzbs_dn7 = assign97070_e149597_d_n7;
        locals.var_pzbs_dn8 = assign97070_e149597_d_n8;
        locals.var_pzbs_dn9 = assign97070_e149597_d_n9;
        locals.var_pzbs_dn10 = assign97070_e149597_d_n10;
        locals.var_pzbs_dn11 = assign97070_e149597_d_n11;
        locals.var_pzbs_dn14 = assign97070_e149597_d_n14;
        locals.var_pzbs_rv = 0.0;

        let (assign97080_e149605, assign97080_e149605_d_n0, assign97080_e149605_d_n2, assign97080_e149605_d_n4, assign97080_e149605_d_n5, assign97080_e149605_d_n6, assign97080_e149605_d_n7, assign97080_e149605_d_n8, assign97080_e149605_d_n9, assign97080_e149605_d_n10, assign97080_e149605_d_n11, assign97080_e149605_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97080_e149602: f64 = (p.p490 * locals.var_tdiff);
        let assign97080_e149603: f64 = (p.p530 - assign97080_e149602);
        (assign97080_e149603, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97080_e149605;
        locals.var_pzbssw_dn0 = assign97080_e149605_d_n0;
        locals.var_pzbssw_dn2 = assign97080_e149605_d_n2;
        locals.var_pzbssw_dn4 = assign97080_e149605_d_n4;
        locals.var_pzbssw_dn5 = assign97080_e149605_d_n5;
        locals.var_pzbssw_dn6 = assign97080_e149605_d_n6;
        locals.var_pzbssw_dn7 = assign97080_e149605_d_n7;
        locals.var_pzbssw_dn8 = assign97080_e149605_d_n8;
        locals.var_pzbssw_dn9 = assign97080_e149605_d_n9;
        locals.var_pzbssw_dn10 = assign97080_e149605_d_n10;
        locals.var_pzbssw_dn11 = assign97080_e149605_d_n11;
        locals.var_pzbssw_dn14 = assign97080_e149605_d_n14;
        locals.var_pzbssw_rv = 0.0;

        let (assign97090_e149613, assign97090_e149613_d_n0, assign97090_e149613_d_n2, assign97090_e149613_d_n4, assign97090_e149613_d_n5, assign97090_e149613_d_n6, assign97090_e149613_d_n7, assign97090_e149613_d_n8, assign97090_e149613_d_n9, assign97090_e149613_d_n10, assign97090_e149613_d_n11, assign97090_e149613_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97090_e149610: f64 = (p.p492 * locals.var_tdiff);
        let assign97090_e149611: f64 = (p.p531 - assign97090_e149610);
        (assign97090_e149611, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97090_e149613;
        locals.var_pzbsswg_dn0 = assign97090_e149613_d_n0;
        locals.var_pzbsswg_dn2 = assign97090_e149613_d_n2;
        locals.var_pzbsswg_dn4 = assign97090_e149613_d_n4;
        locals.var_pzbsswg_dn5 = assign97090_e149613_d_n5;
        locals.var_pzbsswg_dn6 = assign97090_e149613_d_n6;
        locals.var_pzbsswg_dn7 = assign97090_e149613_d_n7;
        locals.var_pzbsswg_dn8 = assign97090_e149613_d_n8;
        locals.var_pzbsswg_dn9 = assign97090_e149613_d_n9;
        locals.var_pzbsswg_dn10 = assign97090_e149613_d_n10;
        locals.var_pzbsswg_dn11 = assign97090_e149613_d_n11;
        locals.var_pzbsswg_dn14 = assign97090_e149613_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let assign97100_e149620: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97100_e149620;
        locals.var_guard2253_rv = 0.0;

        let (assign97110_e149626, assign97110_e149626_d_n0, assign97110_e149626_d_n2, assign97110_e149626_d_n4, assign97110_e149626_d_n5, assign97110_e149626_d_n6, assign97110_e149626_d_n7, assign97110_e149626_d_n8, assign97110_e149626_d_n9, assign97110_e149626_d_n10, assign97110_e149626_d_n11, assign97110_e149626_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97110_e149626;
        locals.var_pzbs_dn0 = assign97110_e149626_d_n0;
        locals.var_pzbs_dn2 = assign97110_e149626_d_n2;
        locals.var_pzbs_dn4 = assign97110_e149626_d_n4;
        locals.var_pzbs_dn5 = assign97110_e149626_d_n5;
        locals.var_pzbs_dn6 = assign97110_e149626_d_n6;
        locals.var_pzbs_dn7 = assign97110_e149626_d_n7;
        locals.var_pzbs_dn8 = assign97110_e149626_d_n8;
        locals.var_pzbs_dn9 = assign97110_e149626_d_n9;
        locals.var_pzbs_dn10 = assign97110_e149626_d_n10;
        locals.var_pzbs_dn11 = assign97110_e149626_d_n11;
        locals.var_pzbs_dn14 = assign97110_e149626_d_n14;
        locals.var_pzbs_rv = 0.0;

        let assign97120_e149633: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97120_e149633;
        locals.var_guard2254_rv = 0.0;

        let (assign97130_e149639, assign97130_e149639_d_n0, assign97130_e149639_d_n2, assign97130_e149639_d_n4, assign97130_e149639_d_n5, assign97130_e149639_d_n6, assign97130_e149639_d_n7, assign97130_e149639_d_n8, assign97130_e149639_d_n9, assign97130_e149639_d_n10, assign97130_e149639_d_n11, assign97130_e149639_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2254 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97130_e149639;
        locals.var_pzbssw_dn0 = assign97130_e149639_d_n0;
        locals.var_pzbssw_dn2 = assign97130_e149639_d_n2;
        locals.var_pzbssw_dn4 = assign97130_e149639_d_n4;
        locals.var_pzbssw_dn5 = assign97130_e149639_d_n5;
        locals.var_pzbssw_dn6 = assign97130_e149639_d_n6;
        locals.var_pzbssw_dn7 = assign97130_e149639_d_n7;
        locals.var_pzbssw_dn8 = assign97130_e149639_d_n8;
        locals.var_pzbssw_dn9 = assign97130_e149639_d_n9;
        locals.var_pzbssw_dn10 = assign97130_e149639_d_n10;
        locals.var_pzbssw_dn11 = assign97130_e149639_d_n11;
        locals.var_pzbssw_dn14 = assign97130_e149639_d_n14;
        locals.var_pzbssw_rv = 0.0;

        let assign97140_e149646: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97140_e149646;
        locals.var_guard2255_rv = 0.0;

        let (assign97150_e149652, assign97150_e149652_d_n0, assign97150_e149652_d_n2, assign97150_e149652_d_n4, assign97150_e149652_d_n5, assign97150_e149652_d_n6, assign97150_e149652_d_n7, assign97150_e149652_d_n8, assign97150_e149652_d_n9, assign97150_e149652_d_n10, assign97150_e149652_d_n11, assign97150_e149652_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2255 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97150_e149652;
        locals.var_pzbsswg_dn0 = assign97150_e149652_d_n0;
        locals.var_pzbsswg_dn2 = assign97150_e149652_d_n2;
        locals.var_pzbsswg_dn4 = assign97150_e149652_d_n4;
        locals.var_pzbsswg_dn5 = assign97150_e149652_d_n5;
        locals.var_pzbsswg_dn6 = assign97150_e149652_d_n6;
        locals.var_pzbsswg_dn7 = assign97150_e149652_d_n7;
        locals.var_pzbsswg_dn8 = assign97150_e149652_d_n8;
        locals.var_pzbsswg_dn9 = assign97150_e149652_d_n9;
        locals.var_pzbsswg_dn10 = assign97150_e149652_d_n10;
        locals.var_pzbsswg_dn11 = assign97150_e149652_d_n11;
        locals.var_pzbsswg_dn14 = assign97150_e149652_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let (assign97160_e149659, assign97160_e149659_d_n0, assign97160_e149659_d_n2, assign97160_e149659_d_n4, assign97160_e149659_d_n5, assign97160_e149659_d_n6, assign97160_e149659_d_n7, assign97160_e149659_d_n8, assign97160_e149659_d_n9, assign97160_e149659_d_n10, assign97160_e149659_d_n11, assign97160_e149659_d_n14,) = {
    if (locals.var_guard2235 == 0.0) {
        let assign97160_e149655: f64 = ctx_temp;
        let assign97160_e149657: f64 = (assign97160_e149655 + p.p11);
        (assign97160_e149657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign97160_e149659;
        locals.var_ttemp_dn0 = assign97160_e149659_d_n0;
        locals.var_ttemp_dn2 = assign97160_e149659_d_n2;
        locals.var_ttemp_dn4 = assign97160_e149659_d_n4;
        locals.var_ttemp_dn5 = assign97160_e149659_d_n5;
        locals.var_ttemp_dn6 = assign97160_e149659_d_n6;
        locals.var_ttemp_dn7 = assign97160_e149659_d_n7;
        locals.var_ttemp_dn8 = assign97160_e149659_d_n8;
        locals.var_ttemp_dn9 = assign97160_e149659_d_n9;
        locals.var_ttemp_dn10 = assign97160_e149659_d_n10;
        locals.var_ttemp_dn11 = assign97160_e149659_d_n11;
        locals.var_ttemp_dn14 = assign97160_e149659_d_n14;
        locals.var_ttemp_rv = 0.0;

        let assign97170_e149662: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97170_e149662;
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

        let assign97180_e149665: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97180_e149665;
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

        let assign97190_e149668: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97190_e149668;
        locals.var_guard2256_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97200_e149674, assign97200_e149674_d_n0, assign97200_e149674_d_n2, assign97200_e149674_d_n4, assign97200_e149674_d_n5, assign97200_e149674_d_n6, assign97200_e149674_d_n7, assign97200_e149674_d_n8, assign97200_e149674_d_n9, assign97200_e149674_d_n10, assign97200_e149674_d_n11, assign97200_e149674_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97200_e149672: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97200_e149672, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn11 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn11)), ((locals.var_isbd2_btm_dn14 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97200_e149674;
        locals.var_t0_dn0 = assign97200_e149674_d_n0;
        locals.var_t0_dn2 = assign97200_e149674_d_n2;
        locals.var_t0_dn4 = assign97200_e149674_d_n4;
        locals.var_t0_dn5 = assign97200_e149674_d_n5;
        locals.var_t0_dn6 = assign97200_e149674_d_n6;
        locals.var_t0_dn7 = assign97200_e149674_d_n7;
        locals.var_t0_dn8 = assign97200_e149674_d_n8;
        locals.var_t0_dn9 = assign97200_e149674_d_n9;
        locals.var_t0_dn10 = assign97200_e149674_d_n10;
        locals.var_t0_dn11 = assign97200_e149674_d_n11;
        locals.var_t0_dn14 = assign97200_e149674_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97210_e149681, assign97210_e149681_d_n0, assign97210_e149681_d_n2, assign97210_e149681_d_n4, assign97210_e149681_d_n5, assign97210_e149681_d_n6, assign97210_e149681_d_n7, assign97210_e149681_d_n8, assign97210_e149681_d_n9, assign97210_e149681_d_n10, assign97210_e149681_d_n11, assign97210_e149681_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97210_e149677: f64 = (-locals.var_vbd_jct);
        let assign97210_e149679: f64 = (assign97210_e149677 * locals.var_t10);
        (assign97210_e149679, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97210_e149677 * locals.var_t10_dn0)), (assign97210_e149677 * locals.var_t10_dn2), (assign97210_e149677 * locals.var_t10_dn4), (assign97210_e149677 * locals.var_t10_dn5), (assign97210_e149677 * locals.var_t10_dn6), (assign97210_e149677 * locals.var_t10_dn7), (assign97210_e149677 * locals.var_t10_dn8), (assign97210_e149677 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97210_e149677 * locals.var_t10_dn10)), (assign97210_e149677 * locals.var_t10_dn11), (assign97210_e149677 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97210_e149681;
        locals.var_tx_dn0 = assign97210_e149681_d_n0;
        locals.var_tx_dn2 = assign97210_e149681_d_n2;
        locals.var_tx_dn4 = assign97210_e149681_d_n4;
        locals.var_tx_dn5 = assign97210_e149681_d_n5;
        locals.var_tx_dn6 = assign97210_e149681_d_n6;
        locals.var_tx_dn7 = assign97210_e149681_d_n7;
        locals.var_tx_dn8 = assign97210_e149681_d_n8;
        locals.var_tx_dn9 = assign97210_e149681_d_n9;
        locals.var_tx_dn10 = assign97210_e149681_d_n10;
        locals.var_tx_dn11 = assign97210_e149681_d_n11;
        locals.var_tx_dn14 = assign97210_e149681_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97220_e149686, assign97220_e149686_d_n0, assign97220_e149686_d_n2, assign97220_e149686_d_n4, assign97220_e149686_d_n5, assign97220_e149686_d_n6, assign97220_e149686_d_n7, assign97220_e149686_d_n8, assign97220_e149686_d_n9, assign97220_e149686_d_n10, assign97220_e149686_d_n11, assign97220_e149686_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97220_e149684: f64 = (locals.var_tx).exp();
        (assign97220_e149684, (assign97220_e149684 * locals.var_tx_dn0), (assign97220_e149684 * locals.var_tx_dn2), (assign97220_e149684 * locals.var_tx_dn4), (assign97220_e149684 * locals.var_tx_dn5), (assign97220_e149684 * locals.var_tx_dn6), (assign97220_e149684 * locals.var_tx_dn7), (assign97220_e149684 * locals.var_tx_dn8), (assign97220_e149684 * locals.var_tx_dn9), (assign97220_e149684 * locals.var_tx_dn10), (assign97220_e149684 * locals.var_tx_dn11), (assign97220_e149684 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97220_e149686;
        locals.var_t2_dn0 = assign97220_e149686_d_n0;
        locals.var_t2_dn2 = assign97220_e149686_d_n2;
        locals.var_t2_dn4 = assign97220_e149686_d_n4;
        locals.var_t2_dn5 = assign97220_e149686_d_n5;
        locals.var_t2_dn6 = assign97220_e149686_d_n6;
        locals.var_t2_dn7 = assign97220_e149686_d_n7;
        locals.var_t2_dn8 = assign97220_e149686_d_n8;
        locals.var_t2_dn9 = assign97220_e149686_d_n9;
        locals.var_t2_dn10 = assign97220_e149686_d_n10;
        locals.var_t2_dn11 = assign97220_e149686_d_n11;
        locals.var_t2_dn14 = assign97220_e149686_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97230_e149690, assign97230_e149690_d_n0, assign97230_e149690_d_n2, assign97230_e149690_d_n4, assign97230_e149690_d_n5, assign97230_e149690_d_n6, assign97230_e149690_d_n7, assign97230_e149690_d_n8, assign97230_e149690_d_n9, assign97230_e149690_d_n10, assign97230_e149690_d_n11, assign97230_e149690_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97230_e149690;
        locals.var_t3_dn0 = assign97230_e149690_d_n0;
        locals.var_t3_dn2 = assign97230_e149690_d_n2;
        locals.var_t3_dn4 = assign97230_e149690_d_n4;
        locals.var_t3_dn5 = assign97230_e149690_d_n5;
        locals.var_t3_dn6 = assign97230_e149690_d_n6;
        locals.var_t3_dn7 = assign97230_e149690_d_n7;
        locals.var_t3_dn8 = assign97230_e149690_d_n8;
        locals.var_t3_dn9 = assign97230_e149690_d_n9;
        locals.var_t3_dn10 = assign97230_e149690_d_n10;
        locals.var_t3_dn11 = assign97230_e149690_d_n11;
        locals.var_t3_dn14 = assign97230_e149690_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97240_e149693: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97240_e149693;
        locals.var_guard2257_rv = 0.0;

        let (assign97250_e149701, assign97250_e149701_d_n0, assign97250_e149701_d_n2, assign97250_e149701_d_n4, assign97250_e149701_d_n5, assign97250_e149701_d_n6, assign97250_e149701_d_n7, assign97250_e149701_d_n8, assign97250_e149701_d_n9, assign97250_e149701_d_n10, assign97250_e149701_d_n11, assign97250_e149701_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) {
        let assign97250_e149699: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97250_e149699, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97250_e149701;
        locals.var_tx_dn0 = assign97250_e149701_d_n0;
        locals.var_tx_dn2 = assign97250_e149701_d_n2;
        locals.var_tx_dn4 = assign97250_e149701_d_n4;
        locals.var_tx_dn5 = assign97250_e149701_d_n5;
        locals.var_tx_dn6 = assign97250_e149701_d_n6;
        locals.var_tx_dn7 = assign97250_e149701_d_n7;
        locals.var_tx_dn8 = assign97250_e149701_d_n8;
        locals.var_tx_dn9 = assign97250_e149701_d_n9;
        locals.var_tx_dn10 = assign97250_e149701_d_n10;
        locals.var_tx_dn11 = assign97250_e149701_d_n11;
        locals.var_tx_dn14 = assign97250_e149701_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97260_e149704: f64 = (-3.0);
        let assign97260_e149706: f64 = (assign97260_e149704 * 34.0);
        let assign97260_e149707: f64 = if locals.var_tx < assign97260_e149706 { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97260_e149707;
        locals.var_guard2258_rv = 0.0;

        let (assign97270_e149715, assign97270_e149715_d_n0, assign97270_e149715_d_n2, assign97270_e149715_d_n4, assign97270_e149715_d_n5, assign97270_e149715_d_n6, assign97270_e149715_d_n7, assign97270_e149715_d_n8, assign97270_e149715_d_n9, assign97270_e149715_d_n10, assign97270_e149715_d_n11, assign97270_e149715_d_n14,) = {
    if (((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) && (locals.var_guard2258 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97270_e149715;
        locals.var_t1_dn0 = assign97270_e149715_d_n0;
        locals.var_t1_dn2 = assign97270_e149715_d_n2;
        locals.var_t1_dn4 = assign97270_e149715_d_n4;
        locals.var_t1_dn5 = assign97270_e149715_d_n5;
        locals.var_t1_dn6 = assign97270_e149715_d_n6;
        locals.var_t1_dn7 = assign97270_e149715_d_n7;
        locals.var_t1_dn8 = assign97270_e149715_d_n8;
        locals.var_t1_dn9 = assign97270_e149715_d_n9;
        locals.var_t1_dn10 = assign97270_e149715_d_n10;
        locals.var_t1_dn11 = assign97270_e149715_d_n11;
        locals.var_t1_dn14 = assign97270_e149715_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97280_e149725, assign97280_e149725_d_n0, assign97280_e149725_d_n2, assign97280_e149725_d_n4, assign97280_e149725_d_n5, assign97280_e149725_d_n6, assign97280_e149725_d_n7, assign97280_e149725_d_n8, assign97280_e149725_d_n9, assign97280_e149725_d_n10, assign97280_e149725_d_n11, assign97280_e149725_d_n14,) = {
    if (((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) && (locals.var_guard2258 == 0.0)) {
        let assign97280_e149723: f64 = (locals.var_tx).exp();
        (assign97280_e149723, (assign97280_e149723 * locals.var_tx_dn0), (assign97280_e149723 * locals.var_tx_dn2), (assign97280_e149723 * locals.var_tx_dn4), (assign97280_e149723 * locals.var_tx_dn5), (assign97280_e149723 * locals.var_tx_dn6), (assign97280_e149723 * locals.var_tx_dn7), (assign97280_e149723 * locals.var_tx_dn8), (assign97280_e149723 * locals.var_tx_dn9), (assign97280_e149723 * locals.var_tx_dn10), (assign97280_e149723 * locals.var_tx_dn11), (assign97280_e149723 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97280_e149725;
        locals.var_t1_dn0 = assign97280_e149725_d_n0;
        locals.var_t1_dn2 = assign97280_e149725_d_n2;
        locals.var_t1_dn4 = assign97280_e149725_d_n4;
        locals.var_t1_dn5 = assign97280_e149725_d_n5;
        locals.var_t1_dn6 = assign97280_e149725_d_n6;
        locals.var_t1_dn7 = assign97280_e149725_d_n7;
        locals.var_t1_dn8 = assign97280_e149725_d_n8;
        locals.var_t1_dn9 = assign97280_e149725_d_n9;
        locals.var_t1_dn10 = assign97280_e149725_d_n10;
        locals.var_t1_dn11 = assign97280_e149725_d_n11;
        locals.var_t1_dn14 = assign97280_e149725_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97300_e149754, assign97300_e149754_d_n0, assign97300_e149754_d_n2, assign97300_e149754_d_n4, assign97300_e149754_d_n5, assign97300_e149754_d_n6, assign97300_e149754_d_n7, assign97300_e149754_d_n8, assign97300_e149754_d_n9, assign97300_e149754_d_n10, assign97300_e149754_d_n11, assign97300_e149754_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97300_e149754;
        locals.var_t1_dn0 = assign97300_e149754_d_n0;
        locals.var_t1_dn2 = assign97300_e149754_d_n2;
        locals.var_t1_dn4 = assign97300_e149754_d_n4;
        locals.var_t1_dn5 = assign97300_e149754_d_n5;
        locals.var_t1_dn6 = assign97300_e149754_d_n6;
        locals.var_t1_dn7 = assign97300_e149754_d_n7;
        locals.var_t1_dn8 = assign97300_e149754_d_n8;
        locals.var_t1_dn9 = assign97300_e149754_d_n9;
        locals.var_t1_dn10 = assign97300_e149754_d_n10;
        locals.var_t1_dn11 = assign97300_e149754_d_n11;
        locals.var_t1_dn14 = assign97300_e149754_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97310_e149765, assign97310_e149765_d_n0, assign97310_e149765_d_n2, assign97310_e149765_d_n4, assign97310_e149765_d_n5, assign97310_e149765_d_n6, assign97310_e149765_d_n7, assign97310_e149765_d_n8, assign97310_e149765_d_n9, assign97310_e149765_d_n10, assign97310_e149765_d_n11, assign97310_e149765_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 == 0.0)) {
        let assign97310_e149761: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97310_e149763: f64 = (assign97310_e149761 * locals.var_t1);
        (assign97310_e149763, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn11)), ((((locals.var_isbd_btm_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97310_e149765;
        locals.var_t4_dn0 = assign97310_e149765_d_n0;
        locals.var_t4_dn2 = assign97310_e149765_d_n2;
        locals.var_t4_dn4 = assign97310_e149765_d_n4;
        locals.var_t4_dn5 = assign97310_e149765_d_n5;
        locals.var_t4_dn6 = assign97310_e149765_d_n6;
        locals.var_t4_dn7 = assign97310_e149765_d_n7;
        locals.var_t4_dn8 = assign97310_e149765_d_n8;
        locals.var_t4_dn9 = assign97310_e149765_d_n9;
        locals.var_t4_dn10 = assign97310_e149765_d_n10;
        locals.var_t4_dn11 = assign97310_e149765_d_n11;
        locals.var_t4_dn14 = assign97310_e149765_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97340_e149802: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97340_e149802;
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

        let assign97360_e149810: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97360_e149810;
        locals.var_guard2259_rv = 0.0;

        let (assign97370_e149816, assign97370_e149816_d_n0, assign97370_e149816_d_n2, assign97370_e149816_d_n4, assign97370_e149816_d_n5, assign97370_e149816_d_n6, assign97370_e149816_d_n7, assign97370_e149816_d_n8, assign97370_e149816_d_n9, assign97370_e149816_d_n10, assign97370_e149816_d_n11, assign97370_e149816_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97370_e149814: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97370_e149814, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn11 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn11)), ((locals.var_isbd2_sws_dn14 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97370_e149816;
        locals.var_t0_dn0 = assign97370_e149816_d_n0;
        locals.var_t0_dn2 = assign97370_e149816_d_n2;
        locals.var_t0_dn4 = assign97370_e149816_d_n4;
        locals.var_t0_dn5 = assign97370_e149816_d_n5;
        locals.var_t0_dn6 = assign97370_e149816_d_n6;
        locals.var_t0_dn7 = assign97370_e149816_d_n7;
        locals.var_t0_dn8 = assign97370_e149816_d_n8;
        locals.var_t0_dn9 = assign97370_e149816_d_n9;
        locals.var_t0_dn10 = assign97370_e149816_d_n10;
        locals.var_t0_dn11 = assign97370_e149816_d_n11;
        locals.var_t0_dn14 = assign97370_e149816_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97380_e149823, assign97380_e149823_d_n0, assign97380_e149823_d_n2, assign97380_e149823_d_n4, assign97380_e149823_d_n5, assign97380_e149823_d_n6, assign97380_e149823_d_n7, assign97380_e149823_d_n8, assign97380_e149823_d_n9, assign97380_e149823_d_n10, assign97380_e149823_d_n11, assign97380_e149823_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97380_e149819: f64 = (-locals.var_vbd_jct);
        let assign97380_e149821: f64 = (assign97380_e149819 * locals.var_t10);
        (assign97380_e149821, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97380_e149819 * locals.var_t10_dn0)), (assign97380_e149819 * locals.var_t10_dn2), (assign97380_e149819 * locals.var_t10_dn4), (assign97380_e149819 * locals.var_t10_dn5), (assign97380_e149819 * locals.var_t10_dn6), (assign97380_e149819 * locals.var_t10_dn7), (assign97380_e149819 * locals.var_t10_dn8), (assign97380_e149819 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97380_e149819 * locals.var_t10_dn10)), (assign97380_e149819 * locals.var_t10_dn11), (assign97380_e149819 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97380_e149823;
        locals.var_tx_dn0 = assign97380_e149823_d_n0;
        locals.var_tx_dn2 = assign97380_e149823_d_n2;
        locals.var_tx_dn4 = assign97380_e149823_d_n4;
        locals.var_tx_dn5 = assign97380_e149823_d_n5;
        locals.var_tx_dn6 = assign97380_e149823_d_n6;
        locals.var_tx_dn7 = assign97380_e149823_d_n7;
        locals.var_tx_dn8 = assign97380_e149823_d_n8;
        locals.var_tx_dn9 = assign97380_e149823_d_n9;
        locals.var_tx_dn10 = assign97380_e149823_d_n10;
        locals.var_tx_dn11 = assign97380_e149823_d_n11;
        locals.var_tx_dn14 = assign97380_e149823_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97390_e149828, assign97390_e149828_d_n0, assign97390_e149828_d_n2, assign97390_e149828_d_n4, assign97390_e149828_d_n5, assign97390_e149828_d_n6, assign97390_e149828_d_n7, assign97390_e149828_d_n8, assign97390_e149828_d_n9, assign97390_e149828_d_n10, assign97390_e149828_d_n11, assign97390_e149828_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97390_e149826: f64 = (locals.var_tx).exp();
        (assign97390_e149826, (assign97390_e149826 * locals.var_tx_dn0), (assign97390_e149826 * locals.var_tx_dn2), (assign97390_e149826 * locals.var_tx_dn4), (assign97390_e149826 * locals.var_tx_dn5), (assign97390_e149826 * locals.var_tx_dn6), (assign97390_e149826 * locals.var_tx_dn7), (assign97390_e149826 * locals.var_tx_dn8), (assign97390_e149826 * locals.var_tx_dn9), (assign97390_e149826 * locals.var_tx_dn10), (assign97390_e149826 * locals.var_tx_dn11), (assign97390_e149826 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97390_e149828;
        locals.var_t2_dn0 = assign97390_e149828_d_n0;
        locals.var_t2_dn2 = assign97390_e149828_d_n2;
        locals.var_t2_dn4 = assign97390_e149828_d_n4;
        locals.var_t2_dn5 = assign97390_e149828_d_n5;
        locals.var_t2_dn6 = assign97390_e149828_d_n6;
        locals.var_t2_dn7 = assign97390_e149828_d_n7;
        locals.var_t2_dn8 = assign97390_e149828_d_n8;
        locals.var_t2_dn9 = assign97390_e149828_d_n9;
        locals.var_t2_dn10 = assign97390_e149828_d_n10;
        locals.var_t2_dn11 = assign97390_e149828_d_n11;
        locals.var_t2_dn14 = assign97390_e149828_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97400_e149832, assign97400_e149832_d_n0, assign97400_e149832_d_n2, assign97400_e149832_d_n4, assign97400_e149832_d_n5, assign97400_e149832_d_n6, assign97400_e149832_d_n7, assign97400_e149832_d_n8, assign97400_e149832_d_n9, assign97400_e149832_d_n10, assign97400_e149832_d_n11, assign97400_e149832_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97400_e149832;
        locals.var_t3_dn0 = assign97400_e149832_d_n0;
        locals.var_t3_dn2 = assign97400_e149832_d_n2;
        locals.var_t3_dn4 = assign97400_e149832_d_n4;
        locals.var_t3_dn5 = assign97400_e149832_d_n5;
        locals.var_t3_dn6 = assign97400_e149832_d_n6;
        locals.var_t3_dn7 = assign97400_e149832_d_n7;
        locals.var_t3_dn8 = assign97400_e149832_d_n8;
        locals.var_t3_dn9 = assign97400_e149832_d_n9;
        locals.var_t3_dn10 = assign97400_e149832_d_n10;
        locals.var_t3_dn11 = assign97400_e149832_d_n11;
        locals.var_t3_dn14 = assign97400_e149832_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97410_e149835: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97410_e149835;
        locals.var_guard2260_rv = 0.0;

        let (assign97420_e149843, assign97420_e149843_d_n0, assign97420_e149843_d_n2, assign97420_e149843_d_n4, assign97420_e149843_d_n5, assign97420_e149843_d_n6, assign97420_e149843_d_n7, assign97420_e149843_d_n8, assign97420_e149843_d_n9, assign97420_e149843_d_n10, assign97420_e149843_d_n11, assign97420_e149843_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) {
        let assign97420_e149841: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97420_e149841, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97420_e149843;
        locals.var_tx_dn0 = assign97420_e149843_d_n0;
        locals.var_tx_dn2 = assign97420_e149843_d_n2;
        locals.var_tx_dn4 = assign97420_e149843_d_n4;
        locals.var_tx_dn5 = assign97420_e149843_d_n5;
        locals.var_tx_dn6 = assign97420_e149843_d_n6;
        locals.var_tx_dn7 = assign97420_e149843_d_n7;
        locals.var_tx_dn8 = assign97420_e149843_d_n8;
        locals.var_tx_dn9 = assign97420_e149843_d_n9;
        locals.var_tx_dn10 = assign97420_e149843_d_n10;
        locals.var_tx_dn11 = assign97420_e149843_d_n11;
        locals.var_tx_dn14 = assign97420_e149843_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97430_e149846: f64 = (-3.0);
        let assign97430_e149848: f64 = (assign97430_e149846 * 34.0);
        let assign97430_e149849: f64 = if locals.var_tx < assign97430_e149848 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97430_e149849;
        locals.var_guard2261_rv = 0.0;

        let (assign97440_e149857, assign97440_e149857_d_n0, assign97440_e149857_d_n2, assign97440_e149857_d_n4, assign97440_e149857_d_n5, assign97440_e149857_d_n6, assign97440_e149857_d_n7, assign97440_e149857_d_n8, assign97440_e149857_d_n9, assign97440_e149857_d_n10, assign97440_e149857_d_n11, assign97440_e149857_d_n14,) = {
    if (((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) && (locals.var_guard2261 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97440_e149857;
        locals.var_t1_dn0 = assign97440_e149857_d_n0;
        locals.var_t1_dn2 = assign97440_e149857_d_n2;
        locals.var_t1_dn4 = assign97440_e149857_d_n4;
        locals.var_t1_dn5 = assign97440_e149857_d_n5;
        locals.var_t1_dn6 = assign97440_e149857_d_n6;
        locals.var_t1_dn7 = assign97440_e149857_d_n7;
        locals.var_t1_dn8 = assign97440_e149857_d_n8;
        locals.var_t1_dn9 = assign97440_e149857_d_n9;
        locals.var_t1_dn10 = assign97440_e149857_d_n10;
        locals.var_t1_dn11 = assign97440_e149857_d_n11;
        locals.var_t1_dn14 = assign97440_e149857_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97450_e149867, assign97450_e149867_d_n0, assign97450_e149867_d_n2, assign97450_e149867_d_n4, assign97450_e149867_d_n5, assign97450_e149867_d_n6, assign97450_e149867_d_n7, assign97450_e149867_d_n8, assign97450_e149867_d_n9, assign97450_e149867_d_n10, assign97450_e149867_d_n11, assign97450_e149867_d_n14,) = {
    if (((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) && (locals.var_guard2261 == 0.0)) {
        let assign97450_e149865: f64 = (locals.var_tx).exp();
        (assign97450_e149865, (assign97450_e149865 * locals.var_tx_dn0), (assign97450_e149865 * locals.var_tx_dn2), (assign97450_e149865 * locals.var_tx_dn4), (assign97450_e149865 * locals.var_tx_dn5), (assign97450_e149865 * locals.var_tx_dn6), (assign97450_e149865 * locals.var_tx_dn7), (assign97450_e149865 * locals.var_tx_dn8), (assign97450_e149865 * locals.var_tx_dn9), (assign97450_e149865 * locals.var_tx_dn10), (assign97450_e149865 * locals.var_tx_dn11), (assign97450_e149865 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97450_e149867;
        locals.var_t1_dn0 = assign97450_e149867_d_n0;
        locals.var_t1_dn2 = assign97450_e149867_d_n2;
        locals.var_t1_dn4 = assign97450_e149867_d_n4;
        locals.var_t1_dn5 = assign97450_e149867_d_n5;
        locals.var_t1_dn6 = assign97450_e149867_d_n6;
        locals.var_t1_dn7 = assign97450_e149867_d_n7;
        locals.var_t1_dn8 = assign97450_e149867_d_n8;
        locals.var_t1_dn9 = assign97450_e149867_d_n9;
        locals.var_t1_dn10 = assign97450_e149867_d_n10;
        locals.var_t1_dn11 = assign97450_e149867_d_n11;
        locals.var_t1_dn14 = assign97450_e149867_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97470_e149896, assign97470_e149896_d_n0, assign97470_e149896_d_n2, assign97470_e149896_d_n4, assign97470_e149896_d_n5, assign97470_e149896_d_n6, assign97470_e149896_d_n7, assign97470_e149896_d_n8, assign97470_e149896_d_n9, assign97470_e149896_d_n10, assign97470_e149896_d_n11, assign97470_e149896_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97470_e149896;
        locals.var_t1_dn0 = assign97470_e149896_d_n0;
        locals.var_t1_dn2 = assign97470_e149896_d_n2;
        locals.var_t1_dn4 = assign97470_e149896_d_n4;
        locals.var_t1_dn5 = assign97470_e149896_d_n5;
        locals.var_t1_dn6 = assign97470_e149896_d_n6;
        locals.var_t1_dn7 = assign97470_e149896_d_n7;
        locals.var_t1_dn8 = assign97470_e149896_d_n8;
        locals.var_t1_dn9 = assign97470_e149896_d_n9;
        locals.var_t1_dn10 = assign97470_e149896_d_n10;
        locals.var_t1_dn11 = assign97470_e149896_d_n11;
        locals.var_t1_dn14 = assign97470_e149896_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97480_e149907, assign97480_e149907_d_n0, assign97480_e149907_d_n2, assign97480_e149907_d_n4, assign97480_e149907_d_n5, assign97480_e149907_d_n6, assign97480_e149907_d_n7, assign97480_e149907_d_n8, assign97480_e149907_d_n9, assign97480_e149907_d_n10, assign97480_e149907_d_n11, assign97480_e149907_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 == 0.0)) {
        let assign97480_e149903: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97480_e149905: f64 = (assign97480_e149903 * locals.var_t1);
        (assign97480_e149905, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn11)), ((((locals.var_isbd_sws_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97480_e149907;
        locals.var_t4_dn0 = assign97480_e149907_d_n0;
        locals.var_t4_dn2 = assign97480_e149907_d_n2;
        locals.var_t4_dn4 = assign97480_e149907_d_n4;
        locals.var_t4_dn5 = assign97480_e149907_d_n5;
        locals.var_t4_dn6 = assign97480_e149907_d_n6;
        locals.var_t4_dn7 = assign97480_e149907_d_n7;
        locals.var_t4_dn8 = assign97480_e149907_d_n8;
        locals.var_t4_dn9 = assign97480_e149907_d_n9;
        locals.var_t4_dn10 = assign97480_e149907_d_n10;
        locals.var_t4_dn11 = assign97480_e149907_d_n11;
        locals.var_t4_dn14 = assign97480_e149907_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97510_e149944: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97510_e149944;
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

        let assign97530_e149952: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97530_e149952;
        locals.var_guard2262_rv = 0.0;

        let assign97540_e149955: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97540_e149955;
        locals.var_guard2263_rv = 0.0;

        let (assign97550_e149963, assign97550_e149963_d_n0, assign97550_e149963_d_n2, assign97550_e149963_d_n4, assign97550_e149963_d_n5, assign97550_e149963_d_n6, assign97550_e149963_d_n7, assign97550_e149963_d_n8, assign97550_e149963_d_n9, assign97550_e149963_d_n10, assign97550_e149963_d_n11, assign97550_e149963_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97550_e149961: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97550_e149961, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn11 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn11)), ((locals.var_isbd2_swg_dn14 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97550_e149963;
        locals.var_t0_dn0 = assign97550_e149963_d_n0;
        locals.var_t0_dn2 = assign97550_e149963_d_n2;
        locals.var_t0_dn4 = assign97550_e149963_d_n4;
        locals.var_t0_dn5 = assign97550_e149963_d_n5;
        locals.var_t0_dn6 = assign97550_e149963_d_n6;
        locals.var_t0_dn7 = assign97550_e149963_d_n7;
        locals.var_t0_dn8 = assign97550_e149963_d_n8;
        locals.var_t0_dn9 = assign97550_e149963_d_n9;
        locals.var_t0_dn10 = assign97550_e149963_d_n10;
        locals.var_t0_dn11 = assign97550_e149963_d_n11;
        locals.var_t0_dn14 = assign97550_e149963_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97560_e149972, assign97560_e149972_d_n0, assign97560_e149972_d_n2, assign97560_e149972_d_n4, assign97560_e149972_d_n5, assign97560_e149972_d_n6, assign97560_e149972_d_n7, assign97560_e149972_d_n8, assign97560_e149972_d_n9, assign97560_e149972_d_n10, assign97560_e149972_d_n11, assign97560_e149972_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97560_e149968: f64 = (-locals.var_vbdi_jct);
        let assign97560_e149970: f64 = (assign97560_e149968 * locals.var_t10);
        (assign97560_e149970, (assign97560_e149968 * locals.var_t10_dn0), (assign97560_e149968 * locals.var_t10_dn2), (assign97560_e149968 * locals.var_t10_dn4), (assign97560_e149968 * locals.var_t10_dn5), (((-locals.var_vbdi_jct_dn6) * locals.var_t10) + (assign97560_e149968 * locals.var_t10_dn6)), (assign97560_e149968 * locals.var_t10_dn7), (assign97560_e149968 * locals.var_t10_dn8), (((-locals.var_vbdi_jct_dn9) * locals.var_t10) + (assign97560_e149968 * locals.var_t10_dn9)), (assign97560_e149968 * locals.var_t10_dn10), (assign97560_e149968 * locals.var_t10_dn11), (assign97560_e149968 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97560_e149972;
        locals.var_tx_dn0 = assign97560_e149972_d_n0;
        locals.var_tx_dn2 = assign97560_e149972_d_n2;
        locals.var_tx_dn4 = assign97560_e149972_d_n4;
        locals.var_tx_dn5 = assign97560_e149972_d_n5;
        locals.var_tx_dn6 = assign97560_e149972_d_n6;
        locals.var_tx_dn7 = assign97560_e149972_d_n7;
        locals.var_tx_dn8 = assign97560_e149972_d_n8;
        locals.var_tx_dn9 = assign97560_e149972_d_n9;
        locals.var_tx_dn10 = assign97560_e149972_d_n10;
        locals.var_tx_dn11 = assign97560_e149972_d_n11;
        locals.var_tx_dn14 = assign97560_e149972_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_378(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97570_e149979, assign97570_e149979_d_n0, assign97570_e149979_d_n2, assign97570_e149979_d_n4, assign97570_e149979_d_n5, assign97570_e149979_d_n6, assign97570_e149979_d_n7, assign97570_e149979_d_n8, assign97570_e149979_d_n9, assign97570_e149979_d_n10, assign97570_e149979_d_n11, assign97570_e149979_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97570_e149977: f64 = (locals.var_tx).exp();
        (assign97570_e149977, (assign97570_e149977 * locals.var_tx_dn0), (assign97570_e149977 * locals.var_tx_dn2), (assign97570_e149977 * locals.var_tx_dn4), (assign97570_e149977 * locals.var_tx_dn5), (assign97570_e149977 * locals.var_tx_dn6), (assign97570_e149977 * locals.var_tx_dn7), (assign97570_e149977 * locals.var_tx_dn8), (assign97570_e149977 * locals.var_tx_dn9), (assign97570_e149977 * locals.var_tx_dn10), (assign97570_e149977 * locals.var_tx_dn11), (assign97570_e149977 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97570_e149979;
        locals.var_t2_dn0 = assign97570_e149979_d_n0;
        locals.var_t2_dn2 = assign97570_e149979_d_n2;
        locals.var_t2_dn4 = assign97570_e149979_d_n4;
        locals.var_t2_dn5 = assign97570_e149979_d_n5;
        locals.var_t2_dn6 = assign97570_e149979_d_n6;
        locals.var_t2_dn7 = assign97570_e149979_d_n7;
        locals.var_t2_dn8 = assign97570_e149979_d_n8;
        locals.var_t2_dn9 = assign97570_e149979_d_n9;
        locals.var_t2_dn10 = assign97570_e149979_d_n10;
        locals.var_t2_dn11 = assign97570_e149979_d_n11;
        locals.var_t2_dn14 = assign97570_e149979_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97580_e149985, assign97580_e149985_d_n0, assign97580_e149985_d_n2, assign97580_e149985_d_n4, assign97580_e149985_d_n5, assign97580_e149985_d_n6, assign97580_e149985_d_n7, assign97580_e149985_d_n8, assign97580_e149985_d_n9, assign97580_e149985_d_n10, assign97580_e149985_d_n11, assign97580_e149985_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97580_e149985;
        locals.var_t3_dn0 = assign97580_e149985_d_n0;
        locals.var_t3_dn2 = assign97580_e149985_d_n2;
        locals.var_t3_dn4 = assign97580_e149985_d_n4;
        locals.var_t3_dn5 = assign97580_e149985_d_n5;
        locals.var_t3_dn6 = assign97580_e149985_d_n6;
        locals.var_t3_dn7 = assign97580_e149985_d_n7;
        locals.var_t3_dn8 = assign97580_e149985_d_n8;
        locals.var_t3_dn9 = assign97580_e149985_d_n9;
        locals.var_t3_dn10 = assign97580_e149985_d_n10;
        locals.var_t3_dn11 = assign97580_e149985_d_n11;
        locals.var_t3_dn14 = assign97580_e149985_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97590_e149988: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97590_e149988;
        locals.var_guard2264_rv = 0.0;

        let (assign97600_e149998, assign97600_e149998_d_n0, assign97600_e149998_d_n2, assign97600_e149998_d_n4, assign97600_e149998_d_n5, assign97600_e149998_d_n6, assign97600_e149998_d_n7, assign97600_e149998_d_n8, assign97600_e149998_d_n9, assign97600_e149998_d_n10, assign97600_e149998_d_n11, assign97600_e149998_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) {
        let assign97600_e149996: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97600_e149996, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5), ((locals.var_vbdi_jct_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbdi_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97600_e149998;
        locals.var_tx_dn0 = assign97600_e149998_d_n0;
        locals.var_tx_dn2 = assign97600_e149998_d_n2;
        locals.var_tx_dn4 = assign97600_e149998_d_n4;
        locals.var_tx_dn5 = assign97600_e149998_d_n5;
        locals.var_tx_dn6 = assign97600_e149998_d_n6;
        locals.var_tx_dn7 = assign97600_e149998_d_n7;
        locals.var_tx_dn8 = assign97600_e149998_d_n8;
        locals.var_tx_dn9 = assign97600_e149998_d_n9;
        locals.var_tx_dn10 = assign97600_e149998_d_n10;
        locals.var_tx_dn11 = assign97600_e149998_d_n11;
        locals.var_tx_dn14 = assign97600_e149998_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97610_e150001: f64 = (-3.0);
        let assign97610_e150003: f64 = (assign97610_e150001 * 34.0);
        let assign97610_e150004: f64 = if locals.var_tx < assign97610_e150003 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97610_e150004;
        locals.var_guard2265_rv = 0.0;

        let (assign97620_e150014, assign97620_e150014_d_n0, assign97620_e150014_d_n2, assign97620_e150014_d_n4, assign97620_e150014_d_n5, assign97620_e150014_d_n6, assign97620_e150014_d_n7, assign97620_e150014_d_n8, assign97620_e150014_d_n9, assign97620_e150014_d_n10, assign97620_e150014_d_n11, assign97620_e150014_d_n14,) = {
    if ((((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) && (locals.var_guard2265 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97620_e150014;
        locals.var_t1_dn0 = assign97620_e150014_d_n0;
        locals.var_t1_dn2 = assign97620_e150014_d_n2;
        locals.var_t1_dn4 = assign97620_e150014_d_n4;
        locals.var_t1_dn5 = assign97620_e150014_d_n5;
        locals.var_t1_dn6 = assign97620_e150014_d_n6;
        locals.var_t1_dn7 = assign97620_e150014_d_n7;
        locals.var_t1_dn8 = assign97620_e150014_d_n8;
        locals.var_t1_dn9 = assign97620_e150014_d_n9;
        locals.var_t1_dn10 = assign97620_e150014_d_n10;
        locals.var_t1_dn11 = assign97620_e150014_d_n11;
        locals.var_t1_dn14 = assign97620_e150014_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97630_e150026, assign97630_e150026_d_n0, assign97630_e150026_d_n2, assign97630_e150026_d_n4, assign97630_e150026_d_n5, assign97630_e150026_d_n6, assign97630_e150026_d_n7, assign97630_e150026_d_n8, assign97630_e150026_d_n9, assign97630_e150026_d_n10, assign97630_e150026_d_n11, assign97630_e150026_d_n14,) = {
    if ((((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) && (locals.var_guard2265 == 0.0)) {
        let assign97630_e150024: f64 = (locals.var_tx).exp();
        (assign97630_e150024, (assign97630_e150024 * locals.var_tx_dn0), (assign97630_e150024 * locals.var_tx_dn2), (assign97630_e150024 * locals.var_tx_dn4), (assign97630_e150024 * locals.var_tx_dn5), (assign97630_e150024 * locals.var_tx_dn6), (assign97630_e150024 * locals.var_tx_dn7), (assign97630_e150024 * locals.var_tx_dn8), (assign97630_e150024 * locals.var_tx_dn9), (assign97630_e150024 * locals.var_tx_dn10), (assign97630_e150024 * locals.var_tx_dn11), (assign97630_e150024 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97630_e150026;
        locals.var_t1_dn0 = assign97630_e150026_d_n0;
        locals.var_t1_dn2 = assign97630_e150026_d_n2;
        locals.var_t1_dn4 = assign97630_e150026_d_n4;
        locals.var_t1_dn5 = assign97630_e150026_d_n5;
        locals.var_t1_dn6 = assign97630_e150026_d_n6;
        locals.var_t1_dn7 = assign97630_e150026_d_n7;
        locals.var_t1_dn8 = assign97630_e150026_d_n8;
        locals.var_t1_dn9 = assign97630_e150026_d_n9;
        locals.var_t1_dn10 = assign97630_e150026_d_n10;
        locals.var_t1_dn11 = assign97630_e150026_d_n11;
        locals.var_t1_dn14 = assign97630_e150026_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97650_e150059, assign97650_e150059_d_n0, assign97650_e150059_d_n2, assign97650_e150059_d_n4, assign97650_e150059_d_n5, assign97650_e150059_d_n6, assign97650_e150059_d_n7, assign97650_e150059_d_n8, assign97650_e150059_d_n9, assign97650_e150059_d_n10, assign97650_e150059_d_n11, assign97650_e150059_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97650_e150059;
        locals.var_t1_dn0 = assign97650_e150059_d_n0;
        locals.var_t1_dn2 = assign97650_e150059_d_n2;
        locals.var_t1_dn4 = assign97650_e150059_d_n4;
        locals.var_t1_dn5 = assign97650_e150059_d_n5;
        locals.var_t1_dn6 = assign97650_e150059_d_n6;
        locals.var_t1_dn7 = assign97650_e150059_d_n7;
        locals.var_t1_dn8 = assign97650_e150059_d_n8;
        locals.var_t1_dn9 = assign97650_e150059_d_n9;
        locals.var_t1_dn10 = assign97650_e150059_d_n10;
        locals.var_t1_dn11 = assign97650_e150059_d_n11;
        locals.var_t1_dn14 = assign97650_e150059_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97660_e150072, assign97660_e150072_d_n0, assign97660_e150072_d_n2, assign97660_e150072_d_n4, assign97660_e150072_d_n5, assign97660_e150072_d_n6, assign97660_e150072_d_n7, assign97660_e150072_d_n8, assign97660_e150072_d_n9, assign97660_e150072_d_n10, assign97660_e150072_d_n11, assign97660_e150072_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 == 0.0)) {
        let assign97660_e150068: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97660_e150070: f64 = (assign97660_e150068 * locals.var_t1);
        (assign97660_e150070, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn11)), ((((locals.var_isbd_swg_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97660_e150072;
        locals.var_t4_dn0 = assign97660_e150072_d_n0;
        locals.var_t4_dn2 = assign97660_e150072_d_n2;
        locals.var_t4_dn4 = assign97660_e150072_d_n4;
        locals.var_t4_dn5 = assign97660_e150072_d_n5;
        locals.var_t4_dn6 = assign97660_e150072_d_n6;
        locals.var_t4_dn7 = assign97660_e150072_d_n7;
        locals.var_t4_dn8 = assign97660_e150072_d_n8;
        locals.var_t4_dn9 = assign97660_e150072_d_n9;
        locals.var_t4_dn10 = assign97660_e150072_d_n10;
        locals.var_t4_dn11 = assign97660_e150072_d_n11;
        locals.var_t4_dn14 = assign97660_e150072_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign97690_e150116, assign97690_e150116_d_n0, assign97690_e150116_d_n2, assign97690_e150116_d_n4, assign97690_e150116_d_n5, assign97690_e150116_d_n6, assign97690_e150116_d_n7, assign97690_e150116_d_n8, assign97690_e150116_d_n9, assign97690_e150116_d_n10, assign97690_e150116_d_n11, assign97690_e150116_d_n14,) = {
    if (locals.var_guard2262 != 0.0) {
        let assign97690_e150114: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97690_e150114, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn11), (p.p514 * locals.var_isbd2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign97690_e150116;
        locals.var_t12_dn0 = assign97690_e150116_d_n0;
        locals.var_t12_dn2 = assign97690_e150116_d_n2;
        locals.var_t12_dn4 = assign97690_e150116_d_n4;
        locals.var_t12_dn5 = assign97690_e150116_d_n5;
        locals.var_t12_dn6 = assign97690_e150116_d_n6;
        locals.var_t12_dn7 = assign97690_e150116_d_n7;
        locals.var_t12_dn8 = assign97690_e150116_d_n8;
        locals.var_t12_dn9 = assign97690_e150116_d_n9;
        locals.var_t12_dn10 = assign97690_e150116_d_n10;
        locals.var_t12_dn11 = assign97690_e150116_d_n11;
        locals.var_t12_dn14 = assign97690_e150116_d_n14;
        locals.var_t12_rv = 0.0;

        let assign97720_e150132: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97720_e150132;
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

        let assign97730_e150135: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97730_e150135;
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

        let assign97740_e150138: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97740_e150138;
        locals.var_guard2266_rv = 0.0;

        let (assign97750_e150144, assign97750_e150144_d_n0, assign97750_e150144_d_n2, assign97750_e150144_d_n4, assign97750_e150144_d_n5, assign97750_e150144_d_n6, assign97750_e150144_d_n7, assign97750_e150144_d_n8, assign97750_e150144_d_n9, assign97750_e150144_d_n10, assign97750_e150144_d_n11, assign97750_e150144_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97750_e150142: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97750_e150142, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn11 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn11)), ((locals.var_isbs2_btm_dn14 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97750_e150144;
        locals.var_t0_dn0 = assign97750_e150144_d_n0;
        locals.var_t0_dn2 = assign97750_e150144_d_n2;
        locals.var_t0_dn4 = assign97750_e150144_d_n4;
        locals.var_t0_dn5 = assign97750_e150144_d_n5;
        locals.var_t0_dn6 = assign97750_e150144_d_n6;
        locals.var_t0_dn7 = assign97750_e150144_d_n7;
        locals.var_t0_dn8 = assign97750_e150144_d_n8;
        locals.var_t0_dn9 = assign97750_e150144_d_n9;
        locals.var_t0_dn10 = assign97750_e150144_d_n10;
        locals.var_t0_dn11 = assign97750_e150144_d_n11;
        locals.var_t0_dn14 = assign97750_e150144_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97760_e150151, assign97760_e150151_d_n0, assign97760_e150151_d_n2, assign97760_e150151_d_n4, assign97760_e150151_d_n5, assign97760_e150151_d_n6, assign97760_e150151_d_n7, assign97760_e150151_d_n8, assign97760_e150151_d_n9, assign97760_e150151_d_n10, assign97760_e150151_d_n11, assign97760_e150151_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97760_e150147: f64 = (-locals.var_vbs_jct);
        let assign97760_e150149: f64 = (assign97760_e150147 * locals.var_t10);
        (assign97760_e150149, (assign97760_e150147 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97760_e150147 * locals.var_t10_dn2)), (assign97760_e150147 * locals.var_t10_dn4), (assign97760_e150147 * locals.var_t10_dn5), (assign97760_e150147 * locals.var_t10_dn6), (assign97760_e150147 * locals.var_t10_dn7), (assign97760_e150147 * locals.var_t10_dn8), (assign97760_e150147 * locals.var_t10_dn9), (assign97760_e150147 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97760_e150147 * locals.var_t10_dn11)), (assign97760_e150147 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97760_e150151;
        locals.var_tx_dn0 = assign97760_e150151_d_n0;
        locals.var_tx_dn2 = assign97760_e150151_d_n2;
        locals.var_tx_dn4 = assign97760_e150151_d_n4;
        locals.var_tx_dn5 = assign97760_e150151_d_n5;
        locals.var_tx_dn6 = assign97760_e150151_d_n6;
        locals.var_tx_dn7 = assign97760_e150151_d_n7;
        locals.var_tx_dn8 = assign97760_e150151_d_n8;
        locals.var_tx_dn9 = assign97760_e150151_d_n9;
        locals.var_tx_dn10 = assign97760_e150151_d_n10;
        locals.var_tx_dn11 = assign97760_e150151_d_n11;
        locals.var_tx_dn14 = assign97760_e150151_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97770_e150156, assign97770_e150156_d_n0, assign97770_e150156_d_n2, assign97770_e150156_d_n4, assign97770_e150156_d_n5, assign97770_e150156_d_n6, assign97770_e150156_d_n7, assign97770_e150156_d_n8, assign97770_e150156_d_n9, assign97770_e150156_d_n10, assign97770_e150156_d_n11, assign97770_e150156_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97770_e150154: f64 = (locals.var_tx).exp();
        (assign97770_e150154, (assign97770_e150154 * locals.var_tx_dn0), (assign97770_e150154 * locals.var_tx_dn2), (assign97770_e150154 * locals.var_tx_dn4), (assign97770_e150154 * locals.var_tx_dn5), (assign97770_e150154 * locals.var_tx_dn6), (assign97770_e150154 * locals.var_tx_dn7), (assign97770_e150154 * locals.var_tx_dn8), (assign97770_e150154 * locals.var_tx_dn9), (assign97770_e150154 * locals.var_tx_dn10), (assign97770_e150154 * locals.var_tx_dn11), (assign97770_e150154 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97770_e150156;
        locals.var_t2_dn0 = assign97770_e150156_d_n0;
        locals.var_t2_dn2 = assign97770_e150156_d_n2;
        locals.var_t2_dn4 = assign97770_e150156_d_n4;
        locals.var_t2_dn5 = assign97770_e150156_d_n5;
        locals.var_t2_dn6 = assign97770_e150156_d_n6;
        locals.var_t2_dn7 = assign97770_e150156_d_n7;
        locals.var_t2_dn8 = assign97770_e150156_d_n8;
        locals.var_t2_dn9 = assign97770_e150156_d_n9;
        locals.var_t2_dn10 = assign97770_e150156_d_n10;
        locals.var_t2_dn11 = assign97770_e150156_d_n11;
        locals.var_t2_dn14 = assign97770_e150156_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97780_e150160, assign97780_e150160_d_n0, assign97780_e150160_d_n2, assign97780_e150160_d_n4, assign97780_e150160_d_n5, assign97780_e150160_d_n6, assign97780_e150160_d_n7, assign97780_e150160_d_n8, assign97780_e150160_d_n9, assign97780_e150160_d_n10, assign97780_e150160_d_n11, assign97780_e150160_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97780_e150160;
        locals.var_t3_dn0 = assign97780_e150160_d_n0;
        locals.var_t3_dn2 = assign97780_e150160_d_n2;
        locals.var_t3_dn4 = assign97780_e150160_d_n4;
        locals.var_t3_dn5 = assign97780_e150160_d_n5;
        locals.var_t3_dn6 = assign97780_e150160_d_n6;
        locals.var_t3_dn7 = assign97780_e150160_d_n7;
        locals.var_t3_dn8 = assign97780_e150160_d_n8;
        locals.var_t3_dn9 = assign97780_e150160_d_n9;
        locals.var_t3_dn10 = assign97780_e150160_d_n10;
        locals.var_t3_dn11 = assign97780_e150160_d_n11;
        locals.var_t3_dn14 = assign97780_e150160_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97790_e150163: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97790_e150163;
        locals.var_guard2267_rv = 0.0;

        let (assign97800_e150171, assign97800_e150171_d_n0, assign97800_e150171_d_n2, assign97800_e150171_d_n4, assign97800_e150171_d_n5, assign97800_e150171_d_n6, assign97800_e150171_d_n7, assign97800_e150171_d_n8, assign97800_e150171_d_n9, assign97800_e150171_d_n10, assign97800_e150171_d_n11, assign97800_e150171_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) {
        let assign97800_e150169: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97800_e150169, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97800_e150171;
        locals.var_tx_dn0 = assign97800_e150171_d_n0;
        locals.var_tx_dn2 = assign97800_e150171_d_n2;
        locals.var_tx_dn4 = assign97800_e150171_d_n4;
        locals.var_tx_dn5 = assign97800_e150171_d_n5;
        locals.var_tx_dn6 = assign97800_e150171_d_n6;
        locals.var_tx_dn7 = assign97800_e150171_d_n7;
        locals.var_tx_dn8 = assign97800_e150171_d_n8;
        locals.var_tx_dn9 = assign97800_e150171_d_n9;
        locals.var_tx_dn10 = assign97800_e150171_d_n10;
        locals.var_tx_dn11 = assign97800_e150171_d_n11;
        locals.var_tx_dn14 = assign97800_e150171_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97810_e150174: f64 = (-3.0);
        let assign97810_e150176: f64 = (assign97810_e150174 * 34.0);
        let assign97810_e150177: f64 = if locals.var_tx < assign97810_e150176 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97810_e150177;
        locals.var_guard2268_rv = 0.0;

        let (assign97820_e150185, assign97820_e150185_d_n0, assign97820_e150185_d_n2, assign97820_e150185_d_n4, assign97820_e150185_d_n5, assign97820_e150185_d_n6, assign97820_e150185_d_n7, assign97820_e150185_d_n8, assign97820_e150185_d_n9, assign97820_e150185_d_n10, assign97820_e150185_d_n11, assign97820_e150185_d_n14,) = {
    if (((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) && (locals.var_guard2268 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97820_e150185;
        locals.var_t1_dn0 = assign97820_e150185_d_n0;
        locals.var_t1_dn2 = assign97820_e150185_d_n2;
        locals.var_t1_dn4 = assign97820_e150185_d_n4;
        locals.var_t1_dn5 = assign97820_e150185_d_n5;
        locals.var_t1_dn6 = assign97820_e150185_d_n6;
        locals.var_t1_dn7 = assign97820_e150185_d_n7;
        locals.var_t1_dn8 = assign97820_e150185_d_n8;
        locals.var_t1_dn9 = assign97820_e150185_d_n9;
        locals.var_t1_dn10 = assign97820_e150185_d_n10;
        locals.var_t1_dn11 = assign97820_e150185_d_n11;
        locals.var_t1_dn14 = assign97820_e150185_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97830_e150195, assign97830_e150195_d_n0, assign97830_e150195_d_n2, assign97830_e150195_d_n4, assign97830_e150195_d_n5, assign97830_e150195_d_n6, assign97830_e150195_d_n7, assign97830_e150195_d_n8, assign97830_e150195_d_n9, assign97830_e150195_d_n10, assign97830_e150195_d_n11, assign97830_e150195_d_n14,) = {
    if (((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) && (locals.var_guard2268 == 0.0)) {
        let assign97830_e150193: f64 = (locals.var_tx).exp();
        (assign97830_e150193, (assign97830_e150193 * locals.var_tx_dn0), (assign97830_e150193 * locals.var_tx_dn2), (assign97830_e150193 * locals.var_tx_dn4), (assign97830_e150193 * locals.var_tx_dn5), (assign97830_e150193 * locals.var_tx_dn6), (assign97830_e150193 * locals.var_tx_dn7), (assign97830_e150193 * locals.var_tx_dn8), (assign97830_e150193 * locals.var_tx_dn9), (assign97830_e150193 * locals.var_tx_dn10), (assign97830_e150193 * locals.var_tx_dn11), (assign97830_e150193 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97830_e150195;
        locals.var_t1_dn0 = assign97830_e150195_d_n0;
        locals.var_t1_dn2 = assign97830_e150195_d_n2;
        locals.var_t1_dn4 = assign97830_e150195_d_n4;
        locals.var_t1_dn5 = assign97830_e150195_d_n5;
        locals.var_t1_dn6 = assign97830_e150195_d_n6;
        locals.var_t1_dn7 = assign97830_e150195_d_n7;
        locals.var_t1_dn8 = assign97830_e150195_d_n8;
        locals.var_t1_dn9 = assign97830_e150195_d_n9;
        locals.var_t1_dn10 = assign97830_e150195_d_n10;
        locals.var_t1_dn11 = assign97830_e150195_d_n11;
        locals.var_t1_dn14 = assign97830_e150195_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97850_e150224, assign97850_e150224_d_n0, assign97850_e150224_d_n2, assign97850_e150224_d_n4, assign97850_e150224_d_n5, assign97850_e150224_d_n6, assign97850_e150224_d_n7, assign97850_e150224_d_n8, assign97850_e150224_d_n9, assign97850_e150224_d_n10, assign97850_e150224_d_n11, assign97850_e150224_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97850_e150224;
        locals.var_t1_dn0 = assign97850_e150224_d_n0;
        locals.var_t1_dn2 = assign97850_e150224_d_n2;
        locals.var_t1_dn4 = assign97850_e150224_d_n4;
        locals.var_t1_dn5 = assign97850_e150224_d_n5;
        locals.var_t1_dn6 = assign97850_e150224_d_n6;
        locals.var_t1_dn7 = assign97850_e150224_d_n7;
        locals.var_t1_dn8 = assign97850_e150224_d_n8;
        locals.var_t1_dn9 = assign97850_e150224_d_n9;
        locals.var_t1_dn10 = assign97850_e150224_d_n10;
        locals.var_t1_dn11 = assign97850_e150224_d_n11;
        locals.var_t1_dn14 = assign97850_e150224_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97860_e150235, assign97860_e150235_d_n0, assign97860_e150235_d_n2, assign97860_e150235_d_n4, assign97860_e150235_d_n5, assign97860_e150235_d_n6, assign97860_e150235_d_n7, assign97860_e150235_d_n8, assign97860_e150235_d_n9, assign97860_e150235_d_n10, assign97860_e150235_d_n11, assign97860_e150235_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 == 0.0)) {
        let assign97860_e150231: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97860_e150233: f64 = (assign97860_e150231 * locals.var_t1);
        (assign97860_e150233, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn11)), ((((locals.var_isbs_btm_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97860_e150235;
        locals.var_t4_dn0 = assign97860_e150235_d_n0;
        locals.var_t4_dn2 = assign97860_e150235_d_n2;
        locals.var_t4_dn4 = assign97860_e150235_d_n4;
        locals.var_t4_dn5 = assign97860_e150235_d_n5;
        locals.var_t4_dn6 = assign97860_e150235_d_n6;
        locals.var_t4_dn7 = assign97860_e150235_d_n7;
        locals.var_t4_dn8 = assign97860_e150235_d_n8;
        locals.var_t4_dn9 = assign97860_e150235_d_n9;
        locals.var_t4_dn10 = assign97860_e150235_d_n10;
        locals.var_t4_dn11 = assign97860_e150235_d_n11;
        locals.var_t4_dn14 = assign97860_e150235_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97890_e150272: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97890_e150272;
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

        let assign97910_e150280: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97910_e150280;
        locals.var_guard2269_rv = 0.0;

        let (assign97920_e150286, assign97920_e150286_d_n0, assign97920_e150286_d_n2, assign97920_e150286_d_n4, assign97920_e150286_d_n5, assign97920_e150286_d_n6, assign97920_e150286_d_n7, assign97920_e150286_d_n8, assign97920_e150286_d_n9, assign97920_e150286_d_n10, assign97920_e150286_d_n11, assign97920_e150286_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97920_e150284: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97920_e150284, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn11 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn11)), ((locals.var_isbs2_sws_dn14 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97920_e150286;
        locals.var_t0_dn0 = assign97920_e150286_d_n0;
        locals.var_t0_dn2 = assign97920_e150286_d_n2;
        locals.var_t0_dn4 = assign97920_e150286_d_n4;
        locals.var_t0_dn5 = assign97920_e150286_d_n5;
        locals.var_t0_dn6 = assign97920_e150286_d_n6;
        locals.var_t0_dn7 = assign97920_e150286_d_n7;
        locals.var_t0_dn8 = assign97920_e150286_d_n8;
        locals.var_t0_dn9 = assign97920_e150286_d_n9;
        locals.var_t0_dn10 = assign97920_e150286_d_n10;
        locals.var_t0_dn11 = assign97920_e150286_d_n11;
        locals.var_t0_dn14 = assign97920_e150286_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97930_e150293, assign97930_e150293_d_n0, assign97930_e150293_d_n2, assign97930_e150293_d_n4, assign97930_e150293_d_n5, assign97930_e150293_d_n6, assign97930_e150293_d_n7, assign97930_e150293_d_n8, assign97930_e150293_d_n9, assign97930_e150293_d_n10, assign97930_e150293_d_n11, assign97930_e150293_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97930_e150289: f64 = (-locals.var_vbs_jct);
        let assign97930_e150291: f64 = (assign97930_e150289 * locals.var_t10);
        (assign97930_e150291, (assign97930_e150289 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97930_e150289 * locals.var_t10_dn2)), (assign97930_e150289 * locals.var_t10_dn4), (assign97930_e150289 * locals.var_t10_dn5), (assign97930_e150289 * locals.var_t10_dn6), (assign97930_e150289 * locals.var_t10_dn7), (assign97930_e150289 * locals.var_t10_dn8), (assign97930_e150289 * locals.var_t10_dn9), (assign97930_e150289 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97930_e150289 * locals.var_t10_dn11)), (assign97930_e150289 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97930_e150293;
        locals.var_tx_dn0 = assign97930_e150293_d_n0;
        locals.var_tx_dn2 = assign97930_e150293_d_n2;
        locals.var_tx_dn4 = assign97930_e150293_d_n4;
        locals.var_tx_dn5 = assign97930_e150293_d_n5;
        locals.var_tx_dn6 = assign97930_e150293_d_n6;
        locals.var_tx_dn7 = assign97930_e150293_d_n7;
        locals.var_tx_dn8 = assign97930_e150293_d_n8;
        locals.var_tx_dn9 = assign97930_e150293_d_n9;
        locals.var_tx_dn10 = assign97930_e150293_d_n10;
        locals.var_tx_dn11 = assign97930_e150293_d_n11;
        locals.var_tx_dn14 = assign97930_e150293_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97940_e150298, assign97940_e150298_d_n0, assign97940_e150298_d_n2, assign97940_e150298_d_n4, assign97940_e150298_d_n5, assign97940_e150298_d_n6, assign97940_e150298_d_n7, assign97940_e150298_d_n8, assign97940_e150298_d_n9, assign97940_e150298_d_n10, assign97940_e150298_d_n11, assign97940_e150298_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97940_e150296: f64 = (locals.var_tx).exp();
        (assign97940_e150296, (assign97940_e150296 * locals.var_tx_dn0), (assign97940_e150296 * locals.var_tx_dn2), (assign97940_e150296 * locals.var_tx_dn4), (assign97940_e150296 * locals.var_tx_dn5), (assign97940_e150296 * locals.var_tx_dn6), (assign97940_e150296 * locals.var_tx_dn7), (assign97940_e150296 * locals.var_tx_dn8), (assign97940_e150296 * locals.var_tx_dn9), (assign97940_e150296 * locals.var_tx_dn10), (assign97940_e150296 * locals.var_tx_dn11), (assign97940_e150296 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97940_e150298;
        locals.var_t2_dn0 = assign97940_e150298_d_n0;
        locals.var_t2_dn2 = assign97940_e150298_d_n2;
        locals.var_t2_dn4 = assign97940_e150298_d_n4;
        locals.var_t2_dn5 = assign97940_e150298_d_n5;
        locals.var_t2_dn6 = assign97940_e150298_d_n6;
        locals.var_t2_dn7 = assign97940_e150298_d_n7;
        locals.var_t2_dn8 = assign97940_e150298_d_n8;
        locals.var_t2_dn9 = assign97940_e150298_d_n9;
        locals.var_t2_dn10 = assign97940_e150298_d_n10;
        locals.var_t2_dn11 = assign97940_e150298_d_n11;
        locals.var_t2_dn14 = assign97940_e150298_d_n14;
        locals.var_t2_rv = 0.0;

    }
}
