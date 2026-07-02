#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13430_e7779, assign13430_e7779_d_n0, assign13430_e7779_d_n2, assign13430_e7779_d_n4, assign13430_e7779_d_n5, assign13430_e7779_d_n6, assign13430_e7779_d_n7, assign13430_e7779_d_n8, assign13430_e7779_d_n9, assign13430_e7779_d_n10, assign13430_e7779_d_n11, assign13430_e7779_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13430_e7764: f64 = (0.4 * locals.var_tratio);
        let assign13430_e7765: f64 = (1.8 + assign13430_e7764);
        let assign13430_e7768: f64 = (0.1 * locals.var_tratio);
        let assign13430_e7770: f64 = (assign13430_e7768 * locals.var_tratio);
        let assign13430_e7771: f64 = (assign13430_e7765 + assign13430_e7770);
        let assign13430_e7775: f64 = (1.0 - locals.var_tratio);
        let assign13430_e7776: f64 = (p.p379 * assign13430_e7775);
        let assign13430_e7777: f64 = (assign13430_e7771 - assign13430_e7776);
        (assign13430_e7777, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13430_e7779;
        locals.var_t0_dn0 = assign13430_e7779_d_n0;
        locals.var_t0_dn2 = assign13430_e7779_d_n2;
        locals.var_t0_dn4 = assign13430_e7779_d_n4;
        locals.var_t0_dn5 = assign13430_e7779_d_n5;
        locals.var_t0_dn6 = assign13430_e7779_d_n6;
        locals.var_t0_dn7 = assign13430_e7779_d_n7;
        locals.var_t0_dn8 = assign13430_e7779_d_n8;
        locals.var_t0_dn9 = assign13430_e7779_d_n9;
        locals.var_t0_dn10 = assign13430_e7779_d_n10;
        locals.var_t0_dn11 = assign13430_e7779_d_n11;
        locals.var_t0_dn14 = assign13430_e7779_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13440_e7790, assign13440_e7790_d_n0, assign13440_e7790_d_n2, assign13440_e7790_d_n4, assign13440_e7790_d_n5, assign13440_e7790_d_n6, assign13440_e7790_d_n7, assign13440_e7790_d_n8, assign13440_e7790_d_n9, assign13440_e7790_d_n10, assign13440_e7790_d_n11, assign13440_e7790_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13440_e7788: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13440_e7788, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13440_e7790;
        locals.var_uc_depvmax_dn0 = assign13440_e7790_d_n0;
        locals.var_uc_depvmax_dn2 = assign13440_e7790_d_n2;
        locals.var_uc_depvmax_dn4 = assign13440_e7790_d_n4;
        locals.var_uc_depvmax_dn5 = assign13440_e7790_d_n5;
        locals.var_uc_depvmax_dn6 = assign13440_e7790_d_n6;
        locals.var_uc_depvmax_dn7 = assign13440_e7790_d_n7;
        locals.var_uc_depvmax_dn8 = assign13440_e7790_d_n8;
        locals.var_uc_depvmax_dn9 = assign13440_e7790_d_n9;
        locals.var_uc_depvmax_dn10 = assign13440_e7790_d_n10;
        locals.var_uc_depvmax_dn11 = assign13440_e7790_d_n11;
        locals.var_uc_depvmax_dn14 = assign13440_e7790_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let assign13460_e7798: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign13460_e7798;
        locals.var_guard297_rv = 0.0;

        let (assign13470_e7809, assign13470_e7809_d_n0, assign13470_e7809_d_n2, assign13470_e7809_d_n4, assign13470_e7809_d_n5, assign13470_e7809_d_n6, assign13470_e7809_d_n7, assign13470_e7809_d_n8, assign13470_e7809_d_n9, assign13470_e7809_d_n10, assign13470_e7809_d_n11, assign13470_e7809_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13470_e7809;
        locals.var_uc_depvmax_dn0 = assign13470_e7809_d_n0;
        locals.var_uc_depvmax_dn2 = assign13470_e7809_d_n2;
        locals.var_uc_depvmax_dn4 = assign13470_e7809_d_n4;
        locals.var_uc_depvmax_dn5 = assign13470_e7809_d_n5;
        locals.var_uc_depvmax_dn6 = assign13470_e7809_d_n6;
        locals.var_uc_depvmax_dn7 = assign13470_e7809_d_n7;
        locals.var_uc_depvmax_dn8 = assign13470_e7809_d_n8;
        locals.var_uc_depvmax_dn9 = assign13470_e7809_d_n9;
        locals.var_uc_depvmax_dn10 = assign13470_e7809_d_n10;
        locals.var_uc_depvmax_dn11 = assign13470_e7809_d_n11;
        locals.var_uc_depvmax_dn14 = assign13470_e7809_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign13480_e7822, assign13480_e7822_d_n0, assign13480_e7822_d_n2, assign13480_e7822_d_n4, assign13480_e7822_d_n5, assign13480_e7822_d_n6, assign13480_e7822_d_n7, assign13480_e7822_d_n8, assign13480_e7822_d_n9, assign13480_e7822_d_n10, assign13480_e7822_d_n11, assign13480_e7822_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13480_e7819: f64 = (locals.var_tratio).powf(p.p381);
        let assign13480_e7820: f64 = (locals.var_uc_depmue0 / assign13480_e7819);
        (assign13480_e7820, (((locals.var_uc_depmue0_dn0 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn2 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn4 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn5 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn6 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn7 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn8 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn9 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn10 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn11 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn14 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign13480_e7822;
        locals.var_uc_depmue0_dn0 = assign13480_e7822_d_n0;
        locals.var_uc_depmue0_dn2 = assign13480_e7822_d_n2;
        locals.var_uc_depmue0_dn4 = assign13480_e7822_d_n4;
        locals.var_uc_depmue0_dn5 = assign13480_e7822_d_n5;
        locals.var_uc_depmue0_dn6 = assign13480_e7822_d_n6;
        locals.var_uc_depmue0_dn7 = assign13480_e7822_d_n7;
        locals.var_uc_depmue0_dn8 = assign13480_e7822_d_n8;
        locals.var_uc_depmue0_dn9 = assign13480_e7822_d_n9;
        locals.var_uc_depmue0_dn10 = assign13480_e7822_d_n10;
        locals.var_uc_depmue0_dn11 = assign13480_e7822_d_n11;
        locals.var_uc_depmue0_dn14 = assign13480_e7822_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign13490_e7837, assign13490_e7837_d_n0, assign13490_e7837_d_n2, assign13490_e7837_d_n4, assign13490_e7837_d_n5, assign13490_e7837_d_n6, assign13490_e7837_d_n7, assign13490_e7837_d_n8, assign13490_e7837_d_n9, assign13490_e7837_d_n10, assign13490_e7837_d_n11, assign13490_e7837_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13490_e7833: f64 = (locals.var_tratio - 1.0);
        let assign13490_e7834: f64 = (p.p365 * assign13490_e7833);
        let assign13490_e7835: f64 = (p.p364 + assign13490_e7834);
        (assign13490_e7835, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn11), (p.p365 * locals.var_tratio_dn14),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn11, locals.var_uc_depwlp_dn14,)
    }
};
        locals.var_uc_depwlp = assign13490_e7837;
        locals.var_uc_depwlp_dn0 = assign13490_e7837_d_n0;
        locals.var_uc_depwlp_dn2 = assign13490_e7837_d_n2;
        locals.var_uc_depwlp_dn4 = assign13490_e7837_d_n4;
        locals.var_uc_depwlp_dn5 = assign13490_e7837_d_n5;
        locals.var_uc_depwlp_dn6 = assign13490_e7837_d_n6;
        locals.var_uc_depwlp_dn7 = assign13490_e7837_d_n7;
        locals.var_uc_depwlp_dn8 = assign13490_e7837_d_n8;
        locals.var_uc_depwlp_dn9 = assign13490_e7837_d_n9;
        locals.var_uc_depwlp_dn10 = assign13490_e7837_d_n10;
        locals.var_uc_depwlp_dn11 = assign13490_e7837_d_n11;
        locals.var_uc_depwlp_dn14 = assign13490_e7837_d_n14;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign13500_e7847, assign13500_e7847_d_n0, assign13500_e7847_d_n2, assign13500_e7847_d_n4, assign13500_e7847_d_n5, assign13500_e7847_d_n6, assign13500_e7847_d_n7, assign13500_e7847_d_n8, assign13500_e7847_d_n9, assign13500_e7847_d_n10, assign13500_e7847_d_n11, assign13500_e7847_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13500_e7847;
        locals.var_pb2n_dn0 = assign13500_e7847_d_n0;
        locals.var_pb2n_dn2 = assign13500_e7847_d_n2;
        locals.var_pb2n_dn4 = assign13500_e7847_d_n4;
        locals.var_pb2n_dn5 = assign13500_e7847_d_n5;
        locals.var_pb2n_dn6 = assign13500_e7847_d_n6;
        locals.var_pb2n_dn7 = assign13500_e7847_d_n7;
        locals.var_pb2n_dn8 = assign13500_e7847_d_n8;
        locals.var_pb2n_dn9 = assign13500_e7847_d_n9;
        locals.var_pb2n_dn10 = assign13500_e7847_d_n10;
        locals.var_pb2n_dn11 = assign13500_e7847_d_n11;
        locals.var_pb2n_dn14 = assign13500_e7847_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign13510_e7866, assign13510_e7866_d_n0, assign13510_e7866_d_n2, assign13510_e7866_d_n4, assign13510_e7866_d_n5, assign13510_e7866_d_n6, assign13510_e7866_d_n7, assign13510_e7866_d_n8, assign13510_e7866_d_n9, assign13510_e7866_d_n10, assign13510_e7866_d_n11, assign13510_e7866_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        let assign13510_e7858: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign13510_e7860: f64 = (assign13510_e7858 * locals.var_nsub);
        let assign13510_e7862: f64 = (assign13510_e7860 / locals.var_nin);
        let assign13510_e7863: f64 = (assign13510_e7862).ln();
        let assign13510_e7864: f64 = (locals.var_beta_inv * assign13510_e7863);
        (assign13510_e7864, ((locals.var_beta_inv_dn0 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn0)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn2 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn2)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn4 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn4)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn5 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn5)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn6 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn6)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn7 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn7)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn8 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn8)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn9 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn9)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn10 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn10)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn11 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn11)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn14 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn14)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13510_e7866;
        locals.var_vbipn_dn0 = assign13510_e7866_d_n0;
        locals.var_vbipn_dn2 = assign13510_e7866_d_n2;
        locals.var_vbipn_dn4 = assign13510_e7866_d_n4;
        locals.var_vbipn_dn5 = assign13510_e7866_d_n5;
        locals.var_vbipn_dn6 = assign13510_e7866_d_n6;
        locals.var_vbipn_dn7 = assign13510_e7866_d_n7;
        locals.var_vbipn_dn8 = assign13510_e7866_d_n8;
        locals.var_vbipn_dn9 = assign13510_e7866_d_n9;
        locals.var_vbipn_dn10 = assign13510_e7866_d_n10;
        locals.var_vbipn_dn11 = assign13510_e7866_d_n11;
        locals.var_vbipn_dn14 = assign13510_e7866_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign13520_e7876, assign13520_e7876_d_n0, assign13520_e7876_d_n2, assign13520_e7876_d_n4, assign13520_e7876_d_n5, assign13520_e7876_d_n6, assign13520_e7876_d_n7, assign13520_e7876_d_n8, assign13520_e7876_d_n9, assign13520_e7876_d_n10, assign13520_e7876_d_n11, assign13520_e7876_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13520_e7876;
        locals.var_depmphn0_dn0 = assign13520_e7876_d_n0;
        locals.var_depmphn0_dn2 = assign13520_e7876_d_n2;
        locals.var_depmphn0_dn4 = assign13520_e7876_d_n4;
        locals.var_depmphn0_dn5 = assign13520_e7876_d_n5;
        locals.var_depmphn0_dn6 = assign13520_e7876_d_n6;
        locals.var_depmphn0_dn7 = assign13520_e7876_d_n7;
        locals.var_depmphn0_dn8 = assign13520_e7876_d_n8;
        locals.var_depmphn0_dn9 = assign13520_e7876_d_n9;
        locals.var_depmphn0_dn10 = assign13520_e7876_d_n10;
        locals.var_depmphn0_dn11 = assign13520_e7876_d_n11;
        locals.var_depmphn0_dn14 = assign13520_e7876_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign13530_e7882, assign13530_e7882_d_n0, assign13530_e7882_d_n2, assign13530_e7882_d_n4, assign13530_e7882_d_n5, assign13530_e7882_d_n6, assign13530_e7882_d_n7, assign13530_e7882_d_n8, assign13530_e7882_d_n9, assign13530_e7882_d_n10, assign13530_e7882_d_n11, assign13530_e7882_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13530_e7880: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign13530_e7880, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn11 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn11)), ((locals.var_ptovr0_dn14 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn11, locals.var_ptovr_dn14,)
    }
};
        locals.var_ptovr = assign13530_e7882;
        locals.var_ptovr_dn0 = assign13530_e7882_d_n0;
        locals.var_ptovr_dn2 = assign13530_e7882_d_n2;
        locals.var_ptovr_dn4 = assign13530_e7882_d_n4;
        locals.var_ptovr_dn5 = assign13530_e7882_d_n5;
        locals.var_ptovr_dn6 = assign13530_e7882_d_n6;
        locals.var_ptovr_dn7 = assign13530_e7882_d_n7;
        locals.var_ptovr_dn8 = assign13530_e7882_d_n8;
        locals.var_ptovr_dn9 = assign13530_e7882_d_n9;
        locals.var_ptovr_dn10 = assign13530_e7882_d_n10;
        locals.var_ptovr_dn11 = assign13530_e7882_d_n11;
        locals.var_ptovr_dn14 = assign13530_e7882_d_n14;
        locals.var_ptovr_rv = 0.0;

        let (assign13540_e7888, assign13540_e7888_d_n0, assign13540_e7888_d_n2, assign13540_e7888_d_n4, assign13540_e7888_d_n5, assign13540_e7888_d_n6, assign13540_e7888_d_n7, assign13540_e7888_d_n8, assign13540_e7888_d_n9, assign13540_e7888_d_n10, assign13540_e7888_d_n11, assign13540_e7888_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13540_e7886: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13540_e7886, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13540_e7888;
        locals.var_t1_dn0 = assign13540_e7888_d_n0;
        locals.var_t1_dn2 = assign13540_e7888_d_n2;
        locals.var_t1_dn4 = assign13540_e7888_d_n4;
        locals.var_t1_dn5 = assign13540_e7888_d_n5;
        locals.var_t1_dn6 = assign13540_e7888_d_n6;
        locals.var_t1_dn7 = assign13540_e7888_d_n7;
        locals.var_t1_dn8 = assign13540_e7888_d_n8;
        locals.var_t1_dn9 = assign13540_e7888_d_n9;
        locals.var_t1_dn10 = assign13540_e7888_d_n10;
        locals.var_t1_dn11 = assign13540_e7888_d_n11;
        locals.var_t1_dn14 = assign13540_e7888_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13550_e7908, assign13550_e7908_d_n0, assign13550_e7908_d_n2, assign13550_e7908_d_n4, assign13550_e7908_d_n5, assign13550_e7908_d_n6, assign13550_e7908_d_n7, assign13550_e7908_d_n8, assign13550_e7908_d_n9, assign13550_e7908_d_n10, assign13550_e7908_d_n11, assign13550_e7908_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13550_e7893: f64 = (0.4 * locals.var_t1);
        let assign13550_e7894: f64 = (1.8 + assign13550_e7893);
        let assign13550_e7897: f64 = (0.1 * locals.var_t1);
        let assign13550_e7899: f64 = (assign13550_e7897 * locals.var_t1);
        let assign13550_e7900: f64 = (assign13550_e7894 + assign13550_e7899);
        let assign13550_e7904: f64 = (1.0 - locals.var_t1);
        let assign13550_e7905: f64 = (locals.var_uc_vtmp * assign13550_e7904);
        let assign13550_e7906: f64 = (assign13550_e7900 - assign13550_e7905);
        (assign13550_e7906, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn11))) - (locals.var_uc_vtmp * (-locals.var_t1_dn11))), (((0.4 * locals.var_t1_dn14) + (((0.1 * locals.var_t1_dn14) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn14))) - (locals.var_uc_vtmp * (-locals.var_t1_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13550_e7908;
        locals.var_t0_dn0 = assign13550_e7908_d_n0;
        locals.var_t0_dn2 = assign13550_e7908_d_n2;
        locals.var_t0_dn4 = assign13550_e7908_d_n4;
        locals.var_t0_dn5 = assign13550_e7908_d_n5;
        locals.var_t0_dn6 = assign13550_e7908_d_n6;
        locals.var_t0_dn7 = assign13550_e7908_d_n7;
        locals.var_t0_dn8 = assign13550_e7908_d_n8;
        locals.var_t0_dn9 = assign13550_e7908_d_n9;
        locals.var_t0_dn10 = assign13550_e7908_d_n10;
        locals.var_t0_dn11 = assign13550_e7908_d_n11;
        locals.var_t0_dn14 = assign13550_e7908_d_n14;
        locals.var_t0_rv = 0.0;

        let assign13560_e7911: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign13560_e7911;
        locals.var_guard298_rv = 0.0;

        let (assign13570_e7931, assign13570_e7931_d_n0, assign13570_e7931_d_n2, assign13570_e7931_d_n4, assign13570_e7931_d_n5, assign13570_e7931_d_n6, assign13570_e7931_d_n7, assign13570_e7931_d_n8, assign13570_e7931_d_n9, assign13570_e7931_d_n10, assign13570_e7931_d_n11, assign13570_e7931_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13570_e7917: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13570_e7919: f64 = (assign13570_e7917 / locals.var_t0);
        let assign13570_e7923: f64 = (p.p90 * locals.var_tdiff0);
        let assign13570_e7924: f64 = (1.0 + assign13570_e7923);
        let assign13570_e7927: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign13570_e7928: f64 = (assign13570_e7924 + assign13570_e7927);
        let assign13570_e7929: f64 = (assign13570_e7919 * assign13570_e7928);
        (assign13570_e7929, (((-((assign13570_e7917 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign13570_e7917 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign13570_e7917 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign13570_e7917 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign13570_e7917 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign13570_e7917 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign13570_e7917 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign13570_e7917 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign13570_e7917 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign13570_e7917 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn11) + (p.p91 * locals.var_tdiff0_2_dn11)))), (((-((assign13570_e7917 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn14) + (p.p91 * locals.var_tdiff0_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13570_e7931;
        locals.var_vmaxeff_dn0 = assign13570_e7931_d_n0;
        locals.var_vmaxeff_dn2 = assign13570_e7931_d_n2;
        locals.var_vmaxeff_dn4 = assign13570_e7931_d_n4;
        locals.var_vmaxeff_dn5 = assign13570_e7931_d_n5;
        locals.var_vmaxeff_dn6 = assign13570_e7931_d_n6;
        locals.var_vmaxeff_dn7 = assign13570_e7931_d_n7;
        locals.var_vmaxeff_dn8 = assign13570_e7931_d_n8;
        locals.var_vmaxeff_dn9 = assign13570_e7931_d_n9;
        locals.var_vmaxeff_dn10 = assign13570_e7931_d_n10;
        locals.var_vmaxeff_dn11 = assign13570_e7931_d_n11;
        locals.var_vmaxeff_dn14 = assign13570_e7931_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let (assign13580_e7952, assign13580_e7952_d_n0, assign13580_e7952_d_n2, assign13580_e7952_d_n4, assign13580_e7952_d_n5, assign13580_e7952_d_n6, assign13580_e7952_d_n7, assign13580_e7952_d_n8, assign13580_e7952_d_n9, assign13580_e7952_d_n10, assign13580_e7952_d_n11, assign13580_e7952_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13580_e7938: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13580_e7940: f64 = (assign13580_e7938 / locals.var_t0);
        let assign13580_e7944: f64 = (p.p90 * locals.var_tdiff);
        let assign13580_e7945: f64 = (1.0 + assign13580_e7944);
        let assign13580_e7948: f64 = (p.p91 * locals.var_tdiff_2);
        let assign13580_e7949: f64 = (assign13580_e7945 + assign13580_e7948);
        let assign13580_e7950: f64 = (assign13580_e7940 * assign13580_e7949);
        (assign13580_e7950, (((-((assign13580_e7938 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign13580_e7938 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign13580_e7938 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign13580_e7938 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign13580_e7938 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign13580_e7938 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign13580_e7938 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign13580_e7938 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign13580_e7938 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign13580_e7938 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn11) + (p.p91 * locals.var_tdiff_2_dn11)))), (((-((assign13580_e7938 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn14) + (p.p91 * locals.var_tdiff_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13580_e7952;
        locals.var_vmaxeff_dn0 = assign13580_e7952_d_n0;
        locals.var_vmaxeff_dn2 = assign13580_e7952_d_n2;
        locals.var_vmaxeff_dn4 = assign13580_e7952_d_n4;
        locals.var_vmaxeff_dn5 = assign13580_e7952_d_n5;
        locals.var_vmaxeff_dn6 = assign13580_e7952_d_n6;
        locals.var_vmaxeff_dn7 = assign13580_e7952_d_n7;
        locals.var_vmaxeff_dn8 = assign13580_e7952_d_n8;
        locals.var_vmaxeff_dn9 = assign13580_e7952_d_n9;
        locals.var_vmaxeff_dn10 = assign13580_e7952_d_n10;
        locals.var_vmaxeff_dn11 = assign13580_e7952_d_n11;
        locals.var_vmaxeff_dn14 = assign13580_e7952_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let assign13600_e7960: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign13600_e7960;
        locals.var_guard300_rv = 0.0;

        let (assign13610_e7976, assign13610_e7976_d_n0, assign13610_e7976_d_n2, assign13610_e7976_d_n4, assign13610_e7976_d_n5, assign13610_e7976_d_n6, assign13610_e7976_d_n7, assign13610_e7976_d_n8, assign13610_e7976_d_n9, assign13610_e7976_d_n10, assign13610_e7976_d_n11, assign13610_e7976_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13610_e7968: f64 = (p.p324 * locals.var_tdiff0);
        let assign13610_e7969: f64 = (1.0 + assign13610_e7968);
        let assign13610_e7972: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign13610_e7973: f64 = (assign13610_e7969 + assign13610_e7972);
        let assign13610_e7974: f64 = (locals.var_ninvd0 * assign13610_e7973);
        (assign13610_e7974, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn11) + (p.p325 * locals.var_tdiff0_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn14) + (p.p325 * locals.var_tdiff0_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13610_e7976;
        locals.var_ninvde_dn0 = assign13610_e7976_d_n0;
        locals.var_ninvde_dn2 = assign13610_e7976_d_n2;
        locals.var_ninvde_dn4 = assign13610_e7976_d_n4;
        locals.var_ninvde_dn5 = assign13610_e7976_d_n5;
        locals.var_ninvde_dn6 = assign13610_e7976_d_n6;
        locals.var_ninvde_dn7 = assign13610_e7976_d_n7;
        locals.var_ninvde_dn8 = assign13610_e7976_d_n8;
        locals.var_ninvde_dn9 = assign13610_e7976_d_n9;
        locals.var_ninvde_dn10 = assign13610_e7976_d_n10;
        locals.var_ninvde_dn11 = assign13610_e7976_d_n11;
        locals.var_ninvde_dn14 = assign13610_e7976_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign13620_e7990, assign13620_e7990_d_n0, assign13620_e7990_d_n2, assign13620_e7990_d_n4, assign13620_e7990_d_n5, assign13620_e7990_d_n6, assign13620_e7990_d_n7, assign13620_e7990_d_n8, assign13620_e7990_d_n9, assign13620_e7990_d_n10, assign13620_e7990_d_n11, assign13620_e7990_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13620_e7983: f64 = (p.p390 * locals.var_tdiff0);
        let assign13620_e7984: f64 = (1.0 + assign13620_e7983);
        let assign13620_e7987: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign13620_e7988: f64 = (assign13620_e7984 + assign13620_e7987);
        (assign13620_e7988, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn11) + (p.p391 * locals.var_tdiff0_2_dn11)), ((p.p390 * locals.var_tdiff0_dn14) + (p.p391 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13620_e7990;
        locals.var_t1_dn0 = assign13620_e7990_d_n0;
        locals.var_t1_dn2 = assign13620_e7990_d_n2;
        locals.var_t1_dn4 = assign13620_e7990_d_n4;
        locals.var_t1_dn5 = assign13620_e7990_d_n5;
        locals.var_t1_dn6 = assign13620_e7990_d_n6;
        locals.var_t1_dn7 = assign13620_e7990_d_n7;
        locals.var_t1_dn8 = assign13620_e7990_d_n8;
        locals.var_t1_dn9 = assign13620_e7990_d_n9;
        locals.var_t1_dn10 = assign13620_e7990_d_n10;
        locals.var_t1_dn11 = assign13620_e7990_d_n11;
        locals.var_t1_dn14 = assign13620_e7990_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13630_e7998, assign13630_e7998_d_n0, assign13630_e7998_d_n2, assign13630_e7998_d_n4, assign13630_e7998_d_n5, assign13630_e7998_d_n6, assign13630_e7998_d_n7, assign13630_e7998_d_n8, assign13630_e7998_d_n9, assign13630_e7998_d_n10, assign13630_e7998_d_n11, assign13630_e7998_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13630_e7996: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13630_e7996, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13630_e7998;
        locals.var_ninvdecres_dn0 = assign13630_e7998_d_n0;
        locals.var_ninvdecres_dn2 = assign13630_e7998_d_n2;
        locals.var_ninvdecres_dn4 = assign13630_e7998_d_n4;
        locals.var_ninvdecres_dn5 = assign13630_e7998_d_n5;
        locals.var_ninvdecres_dn6 = assign13630_e7998_d_n6;
        locals.var_ninvdecres_dn7 = assign13630_e7998_d_n7;
        locals.var_ninvdecres_dn8 = assign13630_e7998_d_n8;
        locals.var_ninvdecres_dn9 = assign13630_e7998_d_n9;
        locals.var_ninvdecres_dn10 = assign13630_e7998_d_n10;
        locals.var_ninvdecres_dn11 = assign13630_e7998_d_n11;
        locals.var_ninvdecres_dn14 = assign13630_e7998_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let (assign13640_e8006, assign13640_e8006_d_n0, assign13640_e8006_d_n2, assign13640_e8006_d_n4, assign13640_e8006_d_n5, assign13640_e8006_d_n6, assign13640_e8006_d_n7, assign13640_e8006_d_n8, assign13640_e8006_d_n9, assign13640_e8006_d_n10, assign13640_e8006_d_n11, assign13640_e8006_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13640_e8004: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13640_e8004, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13640_e8006;
        locals.var_ninvdehres_dn0 = assign13640_e8006_d_n0;
        locals.var_ninvdehres_dn2 = assign13640_e8006_d_n2;
        locals.var_ninvdehres_dn4 = assign13640_e8006_d_n4;
        locals.var_ninvdehres_dn5 = assign13640_e8006_d_n5;
        locals.var_ninvdehres_dn6 = assign13640_e8006_d_n6;
        locals.var_ninvdehres_dn7 = assign13640_e8006_d_n7;
        locals.var_ninvdehres_dn8 = assign13640_e8006_d_n8;
        locals.var_ninvdehres_dn9 = assign13640_e8006_d_n9;
        locals.var_ninvdehres_dn10 = assign13640_e8006_d_n10;
        locals.var_ninvdehres_dn11 = assign13640_e8006_d_n11;
        locals.var_ninvdehres_dn14 = assign13640_e8006_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13650_e8023, assign13650_e8023_d_n0, assign13650_e8023_d_n2, assign13650_e8023_d_n4, assign13650_e8023_d_n5, assign13650_e8023_d_n6, assign13650_e8023_d_n7, assign13650_e8023_d_n8, assign13650_e8023_d_n9, assign13650_e8023_d_n10, assign13650_e8023_d_n11, assign13650_e8023_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13650_e8015: f64 = (p.p324 * locals.var_tdiff);
        let assign13650_e8016: f64 = (1.0 + assign13650_e8015);
        let assign13650_e8019: f64 = (p.p325 * locals.var_tdiff_2);
        let assign13650_e8020: f64 = (assign13650_e8016 + assign13650_e8019);
        let assign13650_e8021: f64 = (locals.var_ninvd0 * assign13650_e8020);
        (assign13650_e8021, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn11) + (p.p325 * locals.var_tdiff_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn14) + (p.p325 * locals.var_tdiff_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13650_e8023;
        locals.var_ninvde_dn0 = assign13650_e8023_d_n0;
        locals.var_ninvde_dn2 = assign13650_e8023_d_n2;
        locals.var_ninvde_dn4 = assign13650_e8023_d_n4;
        locals.var_ninvde_dn5 = assign13650_e8023_d_n5;
        locals.var_ninvde_dn6 = assign13650_e8023_d_n6;
        locals.var_ninvde_dn7 = assign13650_e8023_d_n7;
        locals.var_ninvde_dn8 = assign13650_e8023_d_n8;
        locals.var_ninvde_dn9 = assign13650_e8023_d_n9;
        locals.var_ninvde_dn10 = assign13650_e8023_d_n10;
        locals.var_ninvde_dn11 = assign13650_e8023_d_n11;
        locals.var_ninvde_dn14 = assign13650_e8023_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign13660_e8038, assign13660_e8038_d_n0, assign13660_e8038_d_n2, assign13660_e8038_d_n4, assign13660_e8038_d_n5, assign13660_e8038_d_n6, assign13660_e8038_d_n7, assign13660_e8038_d_n8, assign13660_e8038_d_n9, assign13660_e8038_d_n10, assign13660_e8038_d_n11, assign13660_e8038_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13660_e8031: f64 = (p.p390 * locals.var_tdiff);
        let assign13660_e8032: f64 = (1.0 + assign13660_e8031);
        let assign13660_e8035: f64 = (p.p391 * locals.var_tdiff_2);
        let assign13660_e8036: f64 = (assign13660_e8032 + assign13660_e8035);
        (assign13660_e8036, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn11) + (p.p391 * locals.var_tdiff_2_dn11)), ((p.p390 * locals.var_tdiff_dn14) + (p.p391 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13660_e8038;
        locals.var_t1_dn0 = assign13660_e8038_d_n0;
        locals.var_t1_dn2 = assign13660_e8038_d_n2;
        locals.var_t1_dn4 = assign13660_e8038_d_n4;
        locals.var_t1_dn5 = assign13660_e8038_d_n5;
        locals.var_t1_dn6 = assign13660_e8038_d_n6;
        locals.var_t1_dn7 = assign13660_e8038_d_n7;
        locals.var_t1_dn8 = assign13660_e8038_d_n8;
        locals.var_t1_dn9 = assign13660_e8038_d_n9;
        locals.var_t1_dn10 = assign13660_e8038_d_n10;
        locals.var_t1_dn11 = assign13660_e8038_d_n11;
        locals.var_t1_dn14 = assign13660_e8038_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13670_e8047, assign13670_e8047_d_n0, assign13670_e8047_d_n2, assign13670_e8047_d_n4, assign13670_e8047_d_n5, assign13670_e8047_d_n6, assign13670_e8047_d_n7, assign13670_e8047_d_n8, assign13670_e8047_d_n9, assign13670_e8047_d_n10, assign13670_e8047_d_n11, assign13670_e8047_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13670_e8045: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13670_e8045, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13670_e8047;
        locals.var_ninvdecres_dn0 = assign13670_e8047_d_n0;
        locals.var_ninvdecres_dn2 = assign13670_e8047_d_n2;
        locals.var_ninvdecres_dn4 = assign13670_e8047_d_n4;
        locals.var_ninvdecres_dn5 = assign13670_e8047_d_n5;
        locals.var_ninvdecres_dn6 = assign13670_e8047_d_n6;
        locals.var_ninvdecres_dn7 = assign13670_e8047_d_n7;
        locals.var_ninvdecres_dn8 = assign13670_e8047_d_n8;
        locals.var_ninvdecres_dn9 = assign13670_e8047_d_n9;
        locals.var_ninvdecres_dn10 = assign13670_e8047_d_n10;
        locals.var_ninvdecres_dn11 = assign13670_e8047_d_n11;
        locals.var_ninvdecres_dn14 = assign13670_e8047_d_n14;
        locals.var_ninvdecres_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13680_e8056, assign13680_e8056_d_n0, assign13680_e8056_d_n2, assign13680_e8056_d_n4, assign13680_e8056_d_n5, assign13680_e8056_d_n6, assign13680_e8056_d_n7, assign13680_e8056_d_n8, assign13680_e8056_d_n9, assign13680_e8056_d_n10, assign13680_e8056_d_n11, assign13680_e8056_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13680_e8054: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13680_e8054, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13680_e8056;
        locals.var_ninvdehres_dn0 = assign13680_e8056_d_n0;
        locals.var_ninvdehres_dn2 = assign13680_e8056_d_n2;
        locals.var_ninvdehres_dn4 = assign13680_e8056_d_n4;
        locals.var_ninvdehres_dn5 = assign13680_e8056_d_n5;
        locals.var_ninvdehres_dn6 = assign13680_e8056_d_n6;
        locals.var_ninvdehres_dn7 = assign13680_e8056_d_n7;
        locals.var_ninvdehres_dn8 = assign13680_e8056_d_n8;
        locals.var_ninvdehres_dn9 = assign13680_e8056_d_n9;
        locals.var_ninvdehres_dn10 = assign13680_e8056_d_n10;
        locals.var_ninvdehres_dn11 = assign13680_e8056_d_n11;
        locals.var_ninvdehres_dn14 = assign13680_e8056_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let assign13700_e8064: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign13700_e8064;
        locals.var_guard302_rv = 0.0;

        let (assign13710_e8070, assign13710_e8070_d_n0, assign13710_e8070_d_n2, assign13710_e8070_d_n4, assign13710_e8070_d_n5, assign13710_e8070_d_n6, assign13710_e8070_d_n7, assign13710_e8070_d_n8, assign13710_e8070_d_n9, assign13710_e8070_d_n10, assign13710_e8070_d_n11, assign13710_e8070_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13710_e8070;
        locals.var_ninvde_dn0 = assign13710_e8070_d_n0;
        locals.var_ninvde_dn2 = assign13710_e8070_d_n2;
        locals.var_ninvde_dn4 = assign13710_e8070_d_n4;
        locals.var_ninvde_dn5 = assign13710_e8070_d_n5;
        locals.var_ninvde_dn6 = assign13710_e8070_d_n6;
        locals.var_ninvde_dn7 = assign13710_e8070_d_n7;
        locals.var_ninvde_dn8 = assign13710_e8070_d_n8;
        locals.var_ninvde_dn9 = assign13710_e8070_d_n9;
        locals.var_ninvde_dn10 = assign13710_e8070_d_n10;
        locals.var_ninvde_dn11 = assign13710_e8070_d_n11;
        locals.var_ninvde_dn14 = assign13710_e8070_d_n14;
        locals.var_ninvde_rv = 0.0;

        let assign13730_e8078: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign13730_e8078;
        locals.var_guard304_rv = 0.0;

        let (assign13740_e8084, assign13740_e8084_d_n0, assign13740_e8084_d_n2, assign13740_e8084_d_n4, assign13740_e8084_d_n5, assign13740_e8084_d_n6, assign13740_e8084_d_n7, assign13740_e8084_d_n8, assign13740_e8084_d_n9, assign13740_e8084_d_n10, assign13740_e8084_d_n11, assign13740_e8084_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13740_e8084;
        locals.var_ninvdecres_dn0 = assign13740_e8084_d_n0;
        locals.var_ninvdecres_dn2 = assign13740_e8084_d_n2;
        locals.var_ninvdecres_dn4 = assign13740_e8084_d_n4;
        locals.var_ninvdecres_dn5 = assign13740_e8084_d_n5;
        locals.var_ninvdecres_dn6 = assign13740_e8084_d_n6;
        locals.var_ninvdecres_dn7 = assign13740_e8084_d_n7;
        locals.var_ninvdecres_dn8 = assign13740_e8084_d_n8;
        locals.var_ninvdecres_dn9 = assign13740_e8084_d_n9;
        locals.var_ninvdecres_dn10 = assign13740_e8084_d_n10;
        locals.var_ninvdecres_dn11 = assign13740_e8084_d_n11;
        locals.var_ninvdecres_dn14 = assign13740_e8084_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let assign13760_e8092: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign13760_e8092;
        locals.var_guard306_rv = 0.0;

        let (assign13770_e8098, assign13770_e8098_d_n0, assign13770_e8098_d_n2, assign13770_e8098_d_n4, assign13770_e8098_d_n5, assign13770_e8098_d_n6, assign13770_e8098_d_n7, assign13770_e8098_d_n8, assign13770_e8098_d_n9, assign13770_e8098_d_n10, assign13770_e8098_d_n11, assign13770_e8098_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard306 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13770_e8098;
        locals.var_ninvdehres_dn0 = assign13770_e8098_d_n0;
        locals.var_ninvdehres_dn2 = assign13770_e8098_d_n2;
        locals.var_ninvdehres_dn4 = assign13770_e8098_d_n4;
        locals.var_ninvdehres_dn5 = assign13770_e8098_d_n5;
        locals.var_ninvdehres_dn6 = assign13770_e8098_d_n6;
        locals.var_ninvdehres_dn7 = assign13770_e8098_d_n7;
        locals.var_ninvdehres_dn8 = assign13770_e8098_d_n8;
        locals.var_ninvdehres_dn9 = assign13770_e8098_d_n9;
        locals.var_ninvdehres_dn10 = assign13770_e8098_d_n10;
        locals.var_ninvdehres_dn11 = assign13770_e8098_d_n11;
        locals.var_ninvdehres_dn14 = assign13770_e8098_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13780_e8114, assign13780_e8114_d_n0, assign13780_e8114_d_n2, assign13780_e8114_d_n4, assign13780_e8114_d_n5, assign13780_e8114_d_n6, assign13780_e8114_d_n7, assign13780_e8114_d_n8, assign13780_e8114_d_n9, assign13780_e8114_d_n10, assign13780_e8114_d_n11, assign13780_e8114_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) {
        let assign13780_e8105: f64 = (p.p328 * locals.var_tdiff0);
        let assign13780_e8106: f64 = (locals.var_uc_rth0 + assign13780_e8105);
        let assign13780_e8109: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign13780_e8110: f64 = (assign13780_e8106 + assign13780_e8109);
        let assign13780_e8112: f64 = (assign13780_e8110 * locals.var_rthtemp0);
        (assign13780_e8112, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn11) + (p.p329 * locals.var_tdiff0_2_dn11)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn14) + (p.p329 * locals.var_tdiff0_2_dn14)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13780_e8114;
        locals.var_rth_dn0 = assign13780_e8114_d_n0;
        locals.var_rth_dn2 = assign13780_e8114_d_n2;
        locals.var_rth_dn4 = assign13780_e8114_d_n4;
        locals.var_rth_dn5 = assign13780_e8114_d_n5;
        locals.var_rth_dn6 = assign13780_e8114_d_n6;
        locals.var_rth_dn7 = assign13780_e8114_d_n7;
        locals.var_rth_dn8 = assign13780_e8114_d_n8;
        locals.var_rth_dn9 = assign13780_e8114_d_n9;
        locals.var_rth_dn10 = assign13780_e8114_d_n10;
        locals.var_rth_dn11 = assign13780_e8114_d_n11;
        locals.var_rth_dn14 = assign13780_e8114_d_n14;
        locals.var_rth_rv = 0.0;

        let assign13800_e8122: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign13800_e8122;
        locals.var_guard308_rv = 0.0;

        let (assign13810_e8130, assign13810_e8130_d_n0, assign13810_e8130_d_n2, assign13810_e8130_d_n4, assign13810_e8130_d_n5, assign13810_e8130_d_n6, assign13810_e8130_d_n7, assign13810_e8130_d_n8, assign13810_e8130_d_n9, assign13810_e8130_d_n10, assign13810_e8130_d_n11, assign13810_e8130_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13810_e8130;
        locals.var_rth_dn0 = assign13810_e8130_d_n0;
        locals.var_rth_dn2 = assign13810_e8130_d_n2;
        locals.var_rth_dn4 = assign13810_e8130_d_n4;
        locals.var_rth_dn5 = assign13810_e8130_d_n5;
        locals.var_rth_dn6 = assign13810_e8130_d_n6;
        locals.var_rth_dn7 = assign13810_e8130_d_n7;
        locals.var_rth_dn8 = assign13810_e8130_d_n8;
        locals.var_rth_dn9 = assign13810_e8130_d_n9;
        locals.var_rth_dn10 = assign13810_e8130_d_n10;
        locals.var_rth_dn11 = assign13810_e8130_d_n11;
        locals.var_rth_dn14 = assign13810_e8130_d_n14;
        locals.var_rth_rv = 0.0;

        let (assign13820_e8142, assign13820_e8142_d_n0, assign13820_e8142_d_n2, assign13820_e8142_d_n4, assign13820_e8142_d_n5, assign13820_e8142_d_n6, assign13820_e8142_d_n7, assign13820_e8142_d_n8, assign13820_e8142_d_n9, assign13820_e8142_d_n10, assign13820_e8142_d_n11, assign13820_e8142_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13820_e8135: f64 = (p.p330 * locals.var_tdiff0);
        let assign13820_e8136: f64 = (locals.var_uc_powrat + assign13820_e8135);
        let assign13820_e8139: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign13820_e8140: f64 = (assign13820_e8136 + assign13820_e8139);
        (assign13820_e8140, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn11) + (p.p331 * locals.var_tdiff0_2_dn11)), ((p.p330 * locals.var_tdiff0_dn14) + (p.p331 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13820_e8142;
        locals.var_t2_dn0 = assign13820_e8142_d_n0;
        locals.var_t2_dn2 = assign13820_e8142_d_n2;
        locals.var_t2_dn4 = assign13820_e8142_d_n4;
        locals.var_t2_dn5 = assign13820_e8142_d_n5;
        locals.var_t2_dn6 = assign13820_e8142_d_n6;
        locals.var_t2_dn7 = assign13820_e8142_d_n7;
        locals.var_t2_dn8 = assign13820_e8142_d_n8;
        locals.var_t2_dn9 = assign13820_e8142_d_n9;
        locals.var_t2_dn10 = assign13820_e8142_d_n10;
        locals.var_t2_dn11 = assign13820_e8142_d_n11;
        locals.var_t2_dn14 = assign13820_e8142_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13830_e8150, assign13830_e8150_d_n0, assign13830_e8150_d_n2, assign13830_e8150_d_n4, assign13830_e8150_d_n5, assign13830_e8150_d_n6, assign13830_e8150_d_n7, assign13830_e8150_d_n8, assign13830_e8150_d_n9, assign13830_e8150_d_n10, assign13830_e8150_d_n11, assign13830_e8150_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13830_e8146: f64 = locals.var_t2;
        let assign13830_e8148: f64 = (assign13830_e8146 - 0.05);
        (assign13830_e8148, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13830_e8150;
        locals.var_tmf1_dn0 = assign13830_e8150_d_n0;
        locals.var_tmf1_dn2 = assign13830_e8150_d_n2;
        locals.var_tmf1_dn4 = assign13830_e8150_d_n4;
        locals.var_tmf1_dn5 = assign13830_e8150_d_n5;
        locals.var_tmf1_dn6 = assign13830_e8150_d_n6;
        locals.var_tmf1_dn7 = assign13830_e8150_d_n7;
        locals.var_tmf1_dn8 = assign13830_e8150_d_n8;
        locals.var_tmf1_dn9 = assign13830_e8150_d_n9;
        locals.var_tmf1_dn10 = assign13830_e8150_d_n10;
        locals.var_tmf1_dn11 = assign13830_e8150_d_n11;
        locals.var_tmf1_dn14 = assign13830_e8150_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign13840_e8158, assign13840_e8158_d_n0, assign13840_e8158_d_n2, assign13840_e8158_d_n4, assign13840_e8158_d_n5, assign13840_e8158_d_n6, assign13840_e8158_d_n7, assign13840_e8158_d_n8, assign13840_e8158_d_n9, assign13840_e8158_d_n10, assign13840_e8158_d_n11, assign13840_e8158_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13840_e8158;
        locals.var_tmf2_dn0 = assign13840_e8158_d_n0;
        locals.var_tmf2_dn2 = assign13840_e8158_d_n2;
        locals.var_tmf2_dn4 = assign13840_e8158_d_n4;
        locals.var_tmf2_dn5 = assign13840_e8158_d_n5;
        locals.var_tmf2_dn6 = assign13840_e8158_d_n6;
        locals.var_tmf2_dn7 = assign13840_e8158_d_n7;
        locals.var_tmf2_dn8 = assign13840_e8158_d_n8;
        locals.var_tmf2_dn9 = assign13840_e8158_d_n9;
        locals.var_tmf2_dn10 = assign13840_e8158_d_n10;
        locals.var_tmf2_dn11 = assign13840_e8158_d_n11;
        locals.var_tmf2_dn14 = assign13840_e8158_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13850_e8168, assign13850_e8168_d_n0, assign13850_e8168_d_n2, assign13850_e8168_d_n4, assign13850_e8168_d_n5, assign13850_e8168_d_n6, assign13850_e8168_d_n7, assign13850_e8168_d_n8, assign13850_e8168_d_n9, assign13850_e8168_d_n10, assign13850_e8168_d_n11, assign13850_e8168_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13850_e8165: f64 = (-locals.var_tmf2);
                (assign13850_e8165, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13850_e8168;
        locals.var_tmf2_dn0 = assign13850_e8168_d_n0;
        locals.var_tmf2_dn2 = assign13850_e8168_d_n2;
        locals.var_tmf2_dn4 = assign13850_e8168_d_n4;
        locals.var_tmf2_dn5 = assign13850_e8168_d_n5;
        locals.var_tmf2_dn6 = assign13850_e8168_d_n6;
        locals.var_tmf2_dn7 = assign13850_e8168_d_n7;
        locals.var_tmf2_dn8 = assign13850_e8168_d_n8;
        locals.var_tmf2_dn9 = assign13850_e8168_d_n9;
        locals.var_tmf2_dn10 = assign13850_e8168_d_n10;
        locals.var_tmf2_dn11 = assign13850_e8168_d_n11;
        locals.var_tmf2_dn14 = assign13850_e8168_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13860_e8177, assign13860_e8177_d_n0, assign13860_e8177_d_n2, assign13860_e8177_d_n4, assign13860_e8177_d_n5, assign13860_e8177_d_n6, assign13860_e8177_d_n7, assign13860_e8177_d_n8, assign13860_e8177_d_n9, assign13860_e8177_d_n10, assign13860_e8177_d_n11, assign13860_e8177_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13860_e8172: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13860_e8174: f64 = (assign13860_e8172 + locals.var_tmf2);
        let assign13860_e8175: f64 = (assign13860_e8174).sqrt();
        (assign13860_e8175, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13860_e8175)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13860_e8177;
        locals.var_tmf2_dn0 = assign13860_e8177_d_n0;
        locals.var_tmf2_dn2 = assign13860_e8177_d_n2;
        locals.var_tmf2_dn4 = assign13860_e8177_d_n4;
        locals.var_tmf2_dn5 = assign13860_e8177_d_n5;
        locals.var_tmf2_dn6 = assign13860_e8177_d_n6;
        locals.var_tmf2_dn7 = assign13860_e8177_d_n7;
        locals.var_tmf2_dn8 = assign13860_e8177_d_n8;
        locals.var_tmf2_dn9 = assign13860_e8177_d_n9;
        locals.var_tmf2_dn10 = assign13860_e8177_d_n10;
        locals.var_tmf2_dn11 = assign13860_e8177_d_n11;
        locals.var_tmf2_dn14 = assign13860_e8177_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13870_e8187, assign13870_e8187_d_n0, assign13870_e8187_d_n2, assign13870_e8187_d_n4, assign13870_e8187_d_n5, assign13870_e8187_d_n6, assign13870_e8187_d_n7, assign13870_e8187_d_n8, assign13870_e8187_d_n9, assign13870_e8187_d_n10, assign13870_e8187_d_n11, assign13870_e8187_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13870_e8183: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13870_e8184: f64 = (1.0 + assign13870_e8183);
        let assign13870_e8185: f64 = (0.5 * assign13870_e8184);
        (assign13870_e8185, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13870_e8187;
        locals.var_t0_dn0 = assign13870_e8187_d_n0;
        locals.var_t0_dn2 = assign13870_e8187_d_n2;
        locals.var_t0_dn4 = assign13870_e8187_d_n4;
        locals.var_t0_dn5 = assign13870_e8187_d_n5;
        locals.var_t0_dn6 = assign13870_e8187_d_n6;
        locals.var_t0_dn7 = assign13870_e8187_d_n7;
        locals.var_t0_dn8 = assign13870_e8187_d_n8;
        locals.var_t0_dn9 = assign13870_e8187_d_n9;
        locals.var_t0_dn10 = assign13870_e8187_d_n10;
        locals.var_t0_dn11 = assign13870_e8187_d_n11;
        locals.var_t0_dn14 = assign13870_e8187_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13880_e8197, assign13880_e8197_d_n0, assign13880_e8197_d_n2, assign13880_e8197_d_n4, assign13880_e8197_d_n5, assign13880_e8197_d_n6, assign13880_e8197_d_n7, assign13880_e8197_d_n8, assign13880_e8197_d_n9, assign13880_e8197_d_n10, assign13880_e8197_d_n11, assign13880_e8197_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13880_e8193: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13880_e8194: f64 = (0.5 * assign13880_e8193);
        let assign13880_e8195: f64 = assign13880_e8194;
        (assign13880_e8195, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13880_e8197;
        locals.var_t2_dn0 = assign13880_e8197_d_n0;
        locals.var_t2_dn2 = assign13880_e8197_d_n2;
        locals.var_t2_dn4 = assign13880_e8197_d_n4;
        locals.var_t2_dn5 = assign13880_e8197_d_n5;
        locals.var_t2_dn6 = assign13880_e8197_d_n6;
        locals.var_t2_dn7 = assign13880_e8197_d_n7;
        locals.var_t2_dn8 = assign13880_e8197_d_n8;
        locals.var_t2_dn9 = assign13880_e8197_d_n9;
        locals.var_t2_dn10 = assign13880_e8197_d_n10;
        locals.var_t2_dn11 = assign13880_e8197_d_n11;
        locals.var_t2_dn14 = assign13880_e8197_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13890_e8205, assign13890_e8205_d_n0, assign13890_e8205_d_n2, assign13890_e8205_d_n4, assign13890_e8205_d_n5, assign13890_e8205_d_n6, assign13890_e8205_d_n7, assign13890_e8205_d_n8, assign13890_e8205_d_n9, assign13890_e8205_d_n10, assign13890_e8205_d_n11, assign13890_e8205_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13890_e8201: f64 = (1.0 - locals.var_t2);
        let assign13890_e8203: f64 = (assign13890_e8201 - 0.05);
        (assign13890_e8203, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13890_e8205;
        locals.var_tmf1_dn0 = assign13890_e8205_d_n0;
        locals.var_tmf1_dn2 = assign13890_e8205_d_n2;
        locals.var_tmf1_dn4 = assign13890_e8205_d_n4;
        locals.var_tmf1_dn5 = assign13890_e8205_d_n5;
        locals.var_tmf1_dn6 = assign13890_e8205_d_n6;
        locals.var_tmf1_dn7 = assign13890_e8205_d_n7;
        locals.var_tmf1_dn8 = assign13890_e8205_d_n8;
        locals.var_tmf1_dn9 = assign13890_e8205_d_n9;
        locals.var_tmf1_dn10 = assign13890_e8205_d_n10;
        locals.var_tmf1_dn11 = assign13890_e8205_d_n11;
        locals.var_tmf1_dn14 = assign13890_e8205_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign13900_e8213, assign13900_e8213_d_n0, assign13900_e8213_d_n2, assign13900_e8213_d_n4, assign13900_e8213_d_n5, assign13900_e8213_d_n6, assign13900_e8213_d_n7, assign13900_e8213_d_n8, assign13900_e8213_d_n9, assign13900_e8213_d_n10, assign13900_e8213_d_n11, assign13900_e8213_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13900_e8209: f64 = 4.0;
        let assign13900_e8211: f64 = (assign13900_e8209 * 0.05);
        (assign13900_e8211, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13900_e8213;
        locals.var_tmf2_dn0 = assign13900_e8213_d_n0;
        locals.var_tmf2_dn2 = assign13900_e8213_d_n2;
        locals.var_tmf2_dn4 = assign13900_e8213_d_n4;
        locals.var_tmf2_dn5 = assign13900_e8213_d_n5;
        locals.var_tmf2_dn6 = assign13900_e8213_d_n6;
        locals.var_tmf2_dn7 = assign13900_e8213_d_n7;
        locals.var_tmf2_dn8 = assign13900_e8213_d_n8;
        locals.var_tmf2_dn9 = assign13900_e8213_d_n9;
        locals.var_tmf2_dn10 = assign13900_e8213_d_n10;
        locals.var_tmf2_dn11 = assign13900_e8213_d_n11;
        locals.var_tmf2_dn14 = assign13900_e8213_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13910_e8223, assign13910_e8223_d_n0, assign13910_e8223_d_n2, assign13910_e8223_d_n4, assign13910_e8223_d_n5, assign13910_e8223_d_n6, assign13910_e8223_d_n7, assign13910_e8223_d_n8, assign13910_e8223_d_n9, assign13910_e8223_d_n10, assign13910_e8223_d_n11, assign13910_e8223_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13910_e8220: f64 = (-locals.var_tmf2);
                (assign13910_e8220, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13910_e8223;
        locals.var_tmf2_dn0 = assign13910_e8223_d_n0;
        locals.var_tmf2_dn2 = assign13910_e8223_d_n2;
        locals.var_tmf2_dn4 = assign13910_e8223_d_n4;
        locals.var_tmf2_dn5 = assign13910_e8223_d_n5;
        locals.var_tmf2_dn6 = assign13910_e8223_d_n6;
        locals.var_tmf2_dn7 = assign13910_e8223_d_n7;
        locals.var_tmf2_dn8 = assign13910_e8223_d_n8;
        locals.var_tmf2_dn9 = assign13910_e8223_d_n9;
        locals.var_tmf2_dn10 = assign13910_e8223_d_n10;
        locals.var_tmf2_dn11 = assign13910_e8223_d_n11;
        locals.var_tmf2_dn14 = assign13910_e8223_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13920_e8232, assign13920_e8232_d_n0, assign13920_e8232_d_n2, assign13920_e8232_d_n4, assign13920_e8232_d_n5, assign13920_e8232_d_n6, assign13920_e8232_d_n7, assign13920_e8232_d_n8, assign13920_e8232_d_n9, assign13920_e8232_d_n10, assign13920_e8232_d_n11, assign13920_e8232_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13920_e8227: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13920_e8229: f64 = (assign13920_e8227 + locals.var_tmf2);
        let assign13920_e8230: f64 = (assign13920_e8229).sqrt();
        (assign13920_e8230, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13920_e8230)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13920_e8232;
        locals.var_tmf2_dn0 = assign13920_e8232_d_n0;
        locals.var_tmf2_dn2 = assign13920_e8232_d_n2;
        locals.var_tmf2_dn4 = assign13920_e8232_d_n4;
        locals.var_tmf2_dn5 = assign13920_e8232_d_n5;
        locals.var_tmf2_dn6 = assign13920_e8232_d_n6;
        locals.var_tmf2_dn7 = assign13920_e8232_d_n7;
        locals.var_tmf2_dn8 = assign13920_e8232_d_n8;
        locals.var_tmf2_dn9 = assign13920_e8232_d_n9;
        locals.var_tmf2_dn10 = assign13920_e8232_d_n10;
        locals.var_tmf2_dn11 = assign13920_e8232_d_n11;
        locals.var_tmf2_dn14 = assign13920_e8232_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13930_e8242, assign13930_e8242_d_n0, assign13930_e8242_d_n2, assign13930_e8242_d_n4, assign13930_e8242_d_n5, assign13930_e8242_d_n6, assign13930_e8242_d_n7, assign13930_e8242_d_n8, assign13930_e8242_d_n9, assign13930_e8242_d_n10, assign13930_e8242_d_n11, assign13930_e8242_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13930_e8238: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13930_e8239: f64 = (1.0 + assign13930_e8238);
        let assign13930_e8240: f64 = (0.5 * assign13930_e8239);
        (assign13930_e8240, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13930_e8242;
        locals.var_t0_dn0 = assign13930_e8242_d_n0;
        locals.var_t0_dn2 = assign13930_e8242_d_n2;
        locals.var_t0_dn4 = assign13930_e8242_d_n4;
        locals.var_t0_dn5 = assign13930_e8242_d_n5;
        locals.var_t0_dn6 = assign13930_e8242_d_n6;
        locals.var_t0_dn7 = assign13930_e8242_d_n7;
        locals.var_t0_dn8 = assign13930_e8242_d_n8;
        locals.var_t0_dn9 = assign13930_e8242_d_n9;
        locals.var_t0_dn10 = assign13930_e8242_d_n10;
        locals.var_t0_dn11 = assign13930_e8242_d_n11;
        locals.var_t0_dn14 = assign13930_e8242_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13940_e8252, assign13940_e8252_d_n0, assign13940_e8252_d_n2, assign13940_e8252_d_n4, assign13940_e8252_d_n5, assign13940_e8252_d_n6, assign13940_e8252_d_n7, assign13940_e8252_d_n8, assign13940_e8252_d_n9, assign13940_e8252_d_n10, assign13940_e8252_d_n11, assign13940_e8252_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13940_e8248: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13940_e8249: f64 = (0.5 * assign13940_e8248);
        let assign13940_e8250: f64 = (1.0 - assign13940_e8249);
        (assign13940_e8250, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign13940_e8252;
        locals.var_powratio_dn0 = assign13940_e8252_d_n0;
        locals.var_powratio_dn2 = assign13940_e8252_d_n2;
        locals.var_powratio_dn4 = assign13940_e8252_d_n4;
        locals.var_powratio_dn5 = assign13940_e8252_d_n5;
        locals.var_powratio_dn6 = assign13940_e8252_d_n6;
        locals.var_powratio_dn7 = assign13940_e8252_d_n7;
        locals.var_powratio_dn8 = assign13940_e8252_d_n8;
        locals.var_powratio_dn9 = assign13940_e8252_d_n9;
        locals.var_powratio_dn10 = assign13940_e8252_d_n10;
        locals.var_powratio_dn11 = assign13940_e8252_d_n11;
        locals.var_powratio_dn14 = assign13940_e8252_d_n14;
        locals.var_powratio_rv = 0.0;

        let (assign13950_e8263, assign13950_e8263_d_n0, assign13950_e8263_d_n2, assign13950_e8263_d_n4, assign13950_e8263_d_n5, assign13950_e8263_d_n6, assign13950_e8263_d_n7, assign13950_e8263_d_n8, assign13950_e8263_d_n9, assign13950_e8263_d_n10, assign13950_e8263_d_n11, assign13950_e8263_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13950_e8256: f64 = (2.0 * locals.var_beta_inv);
        let assign13950_e8259: f64 = (locals.var_nsub / locals.var_nin);
        let assign13950_e8260: f64 = (assign13950_e8259).ln();
        let assign13950_e8261: f64 = (assign13950_e8256 * assign13950_e8260);
        (assign13950_e8261, (((2.0 * locals.var_beta_inv_dn0) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn2) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn4) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn5) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn6) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn7) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn8) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn9) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn10) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn11) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn14) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign13950_e8263;
        locals.var_pb2_dn0 = assign13950_e8263_d_n0;
        locals.var_pb2_dn2 = assign13950_e8263_d_n2;
        locals.var_pb2_dn4 = assign13950_e8263_d_n4;
        locals.var_pb2_dn5 = assign13950_e8263_d_n5;
        locals.var_pb2_dn6 = assign13950_e8263_d_n6;
        locals.var_pb2_dn7 = assign13950_e8263_d_n7;
        locals.var_pb2_dn8 = assign13950_e8263_d_n8;
        locals.var_pb2_dn9 = assign13950_e8263_d_n9;
        locals.var_pb2_dn10 = assign13950_e8263_d_n10;
        locals.var_pb2_dn11 = assign13950_e8263_d_n11;
        locals.var_pb2_dn14 = assign13950_e8263_d_n14;
        locals.var_pb2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13960_e8271, assign13960_e8271_d_n0, assign13960_e8271_d_n2, assign13960_e8271_d_n4, assign13960_e8271_d_n5, assign13960_e8271_d_n6, assign13960_e8271_d_n7, assign13960_e8271_d_n8, assign13960_e8271_d_n9, assign13960_e8271_d_n10, assign13960_e8271_d_n11, assign13960_e8271_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13960_e8267: f64 = (2.0 * 1.034943e-10);
        let assign13960_e8269: f64 = (assign13960_e8267 / 1.6021918e-19);
        (assign13960_e8269, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13960_e8271;
        locals.var_t1_dn0 = assign13960_e8271_d_n0;
        locals.var_t1_dn2 = assign13960_e8271_d_n2;
        locals.var_t1_dn4 = assign13960_e8271_d_n4;
        locals.var_t1_dn5 = assign13960_e8271_d_n5;
        locals.var_t1_dn6 = assign13960_e8271_d_n6;
        locals.var_t1_dn7 = assign13960_e8271_d_n7;
        locals.var_t1_dn8 = assign13960_e8271_d_n8;
        locals.var_t1_dn9 = assign13960_e8271_d_n9;
        locals.var_t1_dn10 = assign13960_e8271_d_n10;
        locals.var_t1_dn11 = assign13960_e8271_d_n11;
        locals.var_t1_dn14 = assign13960_e8271_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13970_e8278, assign13970_e8278_d_n0, assign13970_e8278_d_n2, assign13970_e8278_d_n4, assign13970_e8278_d_n5, assign13970_e8278_d_n6, assign13970_e8278_d_n7, assign13970_e8278_d_n8, assign13970_e8278_d_n9, assign13970_e8278_d_n10, assign13970_e8278_d_n11, assign13970_e8278_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13970_e8275: f64 = (locals.var_t1 / locals.var_nsub);
        let assign13970_e8276: f64 = (assign13970_e8275).sqrt();
        (assign13970_e8276, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn14 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    }
};
        locals.var_wdpl = assign13970_e8278;
        locals.var_wdpl_dn0 = assign13970_e8278_d_n0;
        locals.var_wdpl_dn2 = assign13970_e8278_d_n2;
        locals.var_wdpl_dn4 = assign13970_e8278_d_n4;
        locals.var_wdpl_dn5 = assign13970_e8278_d_n5;
        locals.var_wdpl_dn6 = assign13970_e8278_d_n6;
        locals.var_wdpl_dn7 = assign13970_e8278_d_n7;
        locals.var_wdpl_dn8 = assign13970_e8278_d_n8;
        locals.var_wdpl_dn9 = assign13970_e8278_d_n9;
        locals.var_wdpl_dn10 = assign13970_e8278_d_n10;
        locals.var_wdpl_dn11 = assign13970_e8278_d_n11;
        locals.var_wdpl_dn14 = assign13970_e8278_d_n14;
        locals.var_wdpl_rv = 0.0;

        let (assign13980_e8285, assign13980_e8285_d_n0, assign13980_e8285_d_n2, assign13980_e8285_d_n4, assign13980_e8285_d_n5, assign13980_e8285_d_n6, assign13980_e8285_d_n7, assign13980_e8285_d_n8, assign13980_e8285_d_n9, assign13980_e8285_d_n10, assign13980_e8285_d_n11, assign13980_e8285_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13980_e8282: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign13980_e8283: f64 = (assign13980_e8282).sqrt();
        (assign13980_e8283, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn11 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn11)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn14 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn14)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    }
};
        locals.var_wdplp = assign13980_e8285;
        locals.var_wdplp_dn0 = assign13980_e8285_d_n0;
        locals.var_wdplp_dn2 = assign13980_e8285_d_n2;
        locals.var_wdplp_dn4 = assign13980_e8285_d_n4;
        locals.var_wdplp_dn5 = assign13980_e8285_d_n5;
        locals.var_wdplp_dn6 = assign13980_e8285_d_n6;
        locals.var_wdplp_dn7 = assign13980_e8285_d_n7;
        locals.var_wdplp_dn8 = assign13980_e8285_d_n8;
        locals.var_wdplp_dn9 = assign13980_e8285_d_n9;
        locals.var_wdplp_dn10 = assign13980_e8285_d_n10;
        locals.var_wdplp_dn11 = assign13980_e8285_d_n11;
        locals.var_wdplp_dn14 = assign13980_e8285_d_n14;
        locals.var_wdplp_rv = 0.0;

        let assign13990_e8288: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign13990_e8288;
        locals.var_guard309_rv = 0.0;

        let (assign14000_e8303, assign14000_e8303_d_n0, assign14000_e8303_d_n2, assign14000_e8303_d_n4, assign14000_e8303_d_n5, assign14000_e8303_d_n6, assign14000_e8303_d_n7, assign14000_e8303_d_n8, assign14000_e8303_d_n9, assign14000_e8303_d_n10, assign14000_e8303_d_n11, assign14000_e8303_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14000_e8294: f64 = (2.0 * 1.034943e-10);
        let assign14000_e8296: f64 = (assign14000_e8294 * 1.6021918e-19);
        let assign14000_e8298: f64 = (assign14000_e8296 * locals.var_nsub);
        let assign14000_e8300: f64 = (assign14000_e8298 * locals.var_beta_inv);
        let assign14000_e8301: f64 = (assign14000_e8300).sqrt();
        (assign14000_e8301, ((((assign14000_e8296 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn0)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn2)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn4)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn5)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn6)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn7)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn8)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn9)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn10)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn11) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn11)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn14) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn14)) / (2.0 * assign14000_e8301)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign14000_e8303;
        locals.var_cnst0_dn0 = assign14000_e8303_d_n0;
        locals.var_cnst0_dn2 = assign14000_e8303_d_n2;
        locals.var_cnst0_dn4 = assign14000_e8303_d_n4;
        locals.var_cnst0_dn5 = assign14000_e8303_d_n5;
        locals.var_cnst0_dn6 = assign14000_e8303_d_n6;
        locals.var_cnst0_dn7 = assign14000_e8303_d_n7;
        locals.var_cnst0_dn8 = assign14000_e8303_d_n8;
        locals.var_cnst0_dn9 = assign14000_e8303_d_n9;
        locals.var_cnst0_dn10 = assign14000_e8303_d_n10;
        locals.var_cnst0_dn11 = assign14000_e8303_d_n11;
        locals.var_cnst0_dn14 = assign14000_e8303_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign14010_e8311, assign14010_e8311_d_n0, assign14010_e8311_d_n2, assign14010_e8311_d_n4, assign14010_e8311_d_n5, assign14010_e8311_d_n6, assign14010_e8311_d_n7, assign14010_e8311_d_n8, assign14010_e8311_d_n9, assign14010_e8311_d_n10, assign14010_e8311_d_n11, assign14010_e8311_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14010_e8309: f64 = (locals.var_nin / locals.var_nsub);
        (assign14010_e8309, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn14 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14010_e8311;
        locals.var_t1_dn0 = assign14010_e8311_d_n0;
        locals.var_t1_dn2 = assign14010_e8311_d_n2;
        locals.var_t1_dn4 = assign14010_e8311_d_n4;
        locals.var_t1_dn5 = assign14010_e8311_d_n5;
        locals.var_t1_dn6 = assign14010_e8311_d_n6;
        locals.var_t1_dn7 = assign14010_e8311_d_n7;
        locals.var_t1_dn8 = assign14010_e8311_d_n8;
        locals.var_t1_dn9 = assign14010_e8311_d_n9;
        locals.var_t1_dn10 = assign14010_e8311_d_n10;
        locals.var_t1_dn11 = assign14010_e8311_d_n11;
        locals.var_t1_dn14 = assign14010_e8311_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14020_e8319, assign14020_e8319_d_n0, assign14020_e8319_d_n2, assign14020_e8319_d_n4, assign14020_e8319_d_n5, assign14020_e8319_d_n6, assign14020_e8319_d_n7, assign14020_e8319_d_n8, assign14020_e8319_d_n9, assign14020_e8319_d_n10, assign14020_e8319_d_n11, assign14020_e8319_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14020_e8317: f64 = (locals.var_t1 * locals.var_t1);
        (assign14020_e8317, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign14020_e8319;
        locals.var_cnst1_dn0 = assign14020_e8319_d_n0;
        locals.var_cnst1_dn2 = assign14020_e8319_d_n2;
        locals.var_cnst1_dn4 = assign14020_e8319_d_n4;
        locals.var_cnst1_dn5 = assign14020_e8319_d_n5;
        locals.var_cnst1_dn6 = assign14020_e8319_d_n6;
        locals.var_cnst1_dn7 = assign14020_e8319_d_n7;
        locals.var_cnst1_dn8 = assign14020_e8319_d_n8;
        locals.var_cnst1_dn9 = assign14020_e8319_d_n9;
        locals.var_cnst1_dn10 = assign14020_e8319_d_n10;
        locals.var_cnst1_dn11 = assign14020_e8319_d_n11;
        locals.var_cnst1_dn14 = assign14020_e8319_d_n14;
        locals.var_cnst1_rv = 0.0;

        let assign14030_e8322: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign14030_e8322;
        locals.var_guard310_rv = 0.0;

        let assign14040_e8325: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign14040_e8325;
        locals.var_guard311_rv = 0.0;

        let (assign14050_e8338, assign14050_e8338_d_n0, assign14050_e8338_d_n2, assign14050_e8338_d_n4, assign14050_e8338_d_n5, assign14050_e8338_d_n6, assign14050_e8338_d_n7, assign14050_e8338_d_n8, assign14050_e8338_d_n9, assign14050_e8338_d_n10, assign14050_e8338_d_n11, assign14050_e8338_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign14050_e8334: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign14050_e8335: f64 = (assign14050_e8334).sqrt();
        let assign14050_e8336: f64 = (locals.var_cnst0 * assign14050_e8335);
        (assign14050_e8336, ((locals.var_cnst0_dn0 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn2 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn4 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn5 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn6 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn7 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn8 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn9 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn10 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn11 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn14 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign14050_e8338;
        locals.var_cnst0over_dn0 = assign14050_e8338_d_n0;
        locals.var_cnst0over_dn2 = assign14050_e8338_d_n2;
        locals.var_cnst0over_dn4 = assign14050_e8338_d_n4;
        locals.var_cnst0over_dn5 = assign14050_e8338_d_n5;
        locals.var_cnst0over_dn6 = assign14050_e8338_d_n6;
        locals.var_cnst0over_dn7 = assign14050_e8338_d_n7;
        locals.var_cnst0over_dn8 = assign14050_e8338_d_n8;
        locals.var_cnst0over_dn9 = assign14050_e8338_d_n9;
        locals.var_cnst0over_dn10 = assign14050_e8338_d_n10;
        locals.var_cnst0over_dn11 = assign14050_e8338_d_n11;
        locals.var_cnst0over_dn14 = assign14050_e8338_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign14060_e8341: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign14060_e8341;
        locals.var_guard312_rv = 0.0;

        let (assign14070_e8354, assign14070_e8354_d_n0, assign14070_e8354_d_n2, assign14070_e8354_d_n4, assign14070_e8354_d_n5, assign14070_e8354_d_n6, assign14070_e8354_d_n7, assign14070_e8354_d_n8, assign14070_e8354_d_n9, assign14070_e8354_d_n10, assign14070_e8354_d_n11, assign14070_e8354_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 != 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign14070_e8350: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign14070_e8351: f64 = (assign14070_e8350).sqrt();
        let assign14070_e8352: f64 = (locals.var_cnst0 * assign14070_e8351);
        (assign14070_e8352, ((locals.var_cnst0_dn0 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn2 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn4 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn5 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn6 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn7 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn8 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn9 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn10 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn11 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn14 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign14070_e8354;
        locals.var_cnst0overs_dn0 = assign14070_e8354_d_n0;
        locals.var_cnst0overs_dn2 = assign14070_e8354_d_n2;
        locals.var_cnst0overs_dn4 = assign14070_e8354_d_n4;
        locals.var_cnst0overs_dn5 = assign14070_e8354_d_n5;
        locals.var_cnst0overs_dn6 = assign14070_e8354_d_n6;
        locals.var_cnst0overs_dn7 = assign14070_e8354_d_n7;
        locals.var_cnst0overs_dn8 = assign14070_e8354_d_n8;
        locals.var_cnst0overs_dn9 = assign14070_e8354_d_n9;
        locals.var_cnst0overs_dn10 = assign14070_e8354_d_n10;
        locals.var_cnst0overs_dn11 = assign14070_e8354_d_n11;
        locals.var_cnst0overs_dn14 = assign14070_e8354_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign14080_e8357: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign14080_e8357;
        locals.var_guard313_rv = 0.0;

        let (assign14090_e8371, assign14090_e8371_d_n0, assign14090_e8371_d_n2, assign14090_e8371_d_n4, assign14090_e8371_d_n5, assign14090_e8371_d_n6, assign14090_e8371_d_n7, assign14090_e8371_d_n8, assign14090_e8371_d_n9, assign14090_e8371_d_n10, assign14090_e8371_d_n11, assign14090_e8371_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign14090_e8367: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign14090_e8368: f64 = (assign14090_e8367).sqrt();
        let assign14090_e8369: f64 = (locals.var_cnst0 * assign14090_e8368);
        (assign14090_e8369, ((locals.var_cnst0_dn0 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn2 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn4 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn5 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn6 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn7 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn8 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn9 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn10 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn11 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn14 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign14090_e8371;
        locals.var_cnst0over_dn0 = assign14090_e8371_d_n0;
        locals.var_cnst0over_dn2 = assign14090_e8371_d_n2;
        locals.var_cnst0over_dn4 = assign14090_e8371_d_n4;
        locals.var_cnst0over_dn5 = assign14090_e8371_d_n5;
        locals.var_cnst0over_dn6 = assign14090_e8371_d_n6;
        locals.var_cnst0over_dn7 = assign14090_e8371_d_n7;
        locals.var_cnst0over_dn8 = assign14090_e8371_d_n8;
        locals.var_cnst0over_dn9 = assign14090_e8371_d_n9;
        locals.var_cnst0over_dn10 = assign14090_e8371_d_n10;
        locals.var_cnst0over_dn11 = assign14090_e8371_d_n11;
        locals.var_cnst0over_dn14 = assign14090_e8371_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign14100_e8374: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign14100_e8374;
        locals.var_guard314_rv = 0.0;

        let (assign14110_e8388, assign14110_e8388_d_n0, assign14110_e8388_d_n2, assign14110_e8388_d_n4, assign14110_e8388_d_n5, assign14110_e8388_d_n6, assign14110_e8388_d_n7, assign14110_e8388_d_n8, assign14110_e8388_d_n9, assign14110_e8388_d_n10, assign14110_e8388_d_n11, assign14110_e8388_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 == 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign14110_e8384: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign14110_e8385: f64 = (assign14110_e8384).sqrt();
        let assign14110_e8386: f64 = (locals.var_cnst0 * assign14110_e8385);
        (assign14110_e8386, ((locals.var_cnst0_dn0 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn2 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn4 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn5 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn6 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn7 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn8 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn9 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn10 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn11 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn14 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign14110_e8388;
        locals.var_cnst0overs_dn0 = assign14110_e8388_d_n0;
        locals.var_cnst0overs_dn2 = assign14110_e8388_d_n2;
        locals.var_cnst0overs_dn4 = assign14110_e8388_d_n4;
        locals.var_cnst0overs_dn5 = assign14110_e8388_d_n5;
        locals.var_cnst0overs_dn6 = assign14110_e8388_d_n6;
        locals.var_cnst0overs_dn7 = assign14110_e8388_d_n7;
        locals.var_cnst0overs_dn8 = assign14110_e8388_d_n8;
        locals.var_cnst0overs_dn9 = assign14110_e8388_d_n9;
        locals.var_cnst0overs_dn10 = assign14110_e8388_d_n10;
        locals.var_cnst0overs_dn11 = assign14110_e8388_d_n11;
        locals.var_cnst0overs_dn14 = assign14110_e8388_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign14120_e8391: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign14120_e8391;
        locals.var_guard315_rv = 0.0;

        let assign14130_e8394: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign14130_e8394;
        locals.var_guard316_rv = 0.0;

        let (assign14140_e8418, assign14140_e8418_d_n0, assign14140_e8418_d_n2, assign14140_e8418_d_n4, assign14140_e8418_d_n5, assign14140_e8418_d_n6, assign14140_e8418_d_n7, assign14140_e8418_d_n8, assign14140_e8418_d_n9, assign14140_e8418_d_n10, assign14140_e8418_d_n11, assign14140_e8418_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) {
        let assign14140_e8403: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14140_e8405: f64 = (assign14140_e8403 * 1000000.0);
        let assign14140_e8407: f64 = (assign14140_e8405 + locals.var_uc_rdict1);
        let assign14140_e8408: f64 = (locals.var_rdtemp0 * assign14140_e8407);
        let assign14140_e8411: f64 = (p.p68 * p.p100);
        let assign14140_e8413: f64 = (assign14140_e8411 * 1000000.0);
        let assign14140_e8415: f64 = (assign14140_e8413 + p.p101);
        let assign14140_e8416: f64 = (assign14140_e8408 * assign14140_e8415);
        (assign14140_e8416, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14140_e8418;
        locals.var_t2_dn0 = assign14140_e8418_d_n0;
        locals.var_t2_dn2 = assign14140_e8418_d_n2;
        locals.var_t2_dn4 = assign14140_e8418_d_n4;
        locals.var_t2_dn5 = assign14140_e8418_d_n5;
        locals.var_t2_dn6 = assign14140_e8418_d_n6;
        locals.var_t2_dn7 = assign14140_e8418_d_n7;
        locals.var_t2_dn8 = assign14140_e8418_d_n8;
        locals.var_t2_dn9 = assign14140_e8418_d_n9;
        locals.var_t2_dn10 = assign14140_e8418_d_n10;
        locals.var_t2_dn11 = assign14140_e8418_d_n11;
        locals.var_t2_dn14 = assign14140_e8418_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14150_e8421: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign14150_e8421;
        locals.var_guard317_rv = 0.0;

        let (assign14160_e8441, assign14160_e8441_d_n0, assign14160_e8441_d_n2, assign14160_e8441_d_n4, assign14160_e8441_d_n5, assign14160_e8441_d_n6, assign14160_e8441_d_n7, assign14160_e8441_d_n8, assign14160_e8441_d_n9, assign14160_e8441_d_n10, assign14160_e8441_d_n11, assign14160_e8441_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14160_e8432: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14160_e8433: f64 = (locals.var_uc_rd + assign14160_e8432);
        let assign14160_e8436: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14160_e8437: f64 = (assign14160_e8433 + assign14160_e8436);
        let assign14160_e8439: f64 = (assign14160_e8437 * locals.var_t2);
        (assign14160_e8439, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14160_e8441;
        locals.var_rde_dn0 = assign14160_e8441_d_n0;
        locals.var_rde_dn2 = assign14160_e8441_d_n2;
        locals.var_rde_dn4 = assign14160_e8441_d_n4;
        locals.var_rde_dn5 = assign14160_e8441_d_n5;
        locals.var_rde_dn6 = assign14160_e8441_d_n6;
        locals.var_rde_dn7 = assign14160_e8441_d_n7;
        locals.var_rde_dn8 = assign14160_e8441_d_n8;
        locals.var_rde_dn9 = assign14160_e8441_d_n9;
        locals.var_rde_dn10 = assign14160_e8441_d_n10;
        locals.var_rde_dn11 = assign14160_e8441_d_n11;
        locals.var_rde_dn14 = assign14160_e8441_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign14170_e8459, assign14170_e8459_d_n0, assign14170_e8459_d_n2, assign14170_e8459_d_n4, assign14170_e8459_d_n5, assign14170_e8459_d_n6, assign14170_e8459_d_n7, assign14170_e8459_d_n8, assign14170_e8459_d_n9, assign14170_e8459_d_n10, assign14170_e8459_d_n11, assign14170_e8459_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14170_e8452: f64 = (0.005 * locals.var_uc_rd);
        let assign14170_e8453: f64 = (locals.var_rde - assign14170_e8452);
        let assign14170_e8456: f64 = (0.01 * locals.var_uc_rd);
        let assign14170_e8457: f64 = (assign14170_e8453 - assign14170_e8456);
        (assign14170_e8457, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14170_e8459;
        locals.var_tmf1_dn0 = assign14170_e8459_d_n0;
        locals.var_tmf1_dn2 = assign14170_e8459_d_n2;
        locals.var_tmf1_dn4 = assign14170_e8459_d_n4;
        locals.var_tmf1_dn5 = assign14170_e8459_d_n5;
        locals.var_tmf1_dn6 = assign14170_e8459_d_n6;
        locals.var_tmf1_dn7 = assign14170_e8459_d_n7;
        locals.var_tmf1_dn8 = assign14170_e8459_d_n8;
        locals.var_tmf1_dn9 = assign14170_e8459_d_n9;
        locals.var_tmf1_dn10 = assign14170_e8459_d_n10;
        locals.var_tmf1_dn11 = assign14170_e8459_d_n11;
        locals.var_tmf1_dn14 = assign14170_e8459_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14180_e8477, assign14180_e8477_d_n0, assign14180_e8477_d_n2, assign14180_e8477_d_n4, assign14180_e8477_d_n5, assign14180_e8477_d_n6, assign14180_e8477_d_n7, assign14180_e8477_d_n8, assign14180_e8477_d_n9, assign14180_e8477_d_n10, assign14180_e8477_d_n11, assign14180_e8477_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14180_e8470: f64 = (0.005 * locals.var_uc_rd);
        let assign14180_e8471: f64 = (4.0 * assign14180_e8470);
        let assign14180_e8474: f64 = (0.01 * locals.var_uc_rd);
        let assign14180_e8475: f64 = (assign14180_e8471 * assign14180_e8474);
        (assign14180_e8475, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14180_e8477;
        locals.var_tmf2_dn0 = assign14180_e8477_d_n0;
        locals.var_tmf2_dn2 = assign14180_e8477_d_n2;
        locals.var_tmf2_dn4 = assign14180_e8477_d_n4;
        locals.var_tmf2_dn5 = assign14180_e8477_d_n5;
        locals.var_tmf2_dn6 = assign14180_e8477_d_n6;
        locals.var_tmf2_dn7 = assign14180_e8477_d_n7;
        locals.var_tmf2_dn8 = assign14180_e8477_d_n8;
        locals.var_tmf2_dn9 = assign14180_e8477_d_n9;
        locals.var_tmf2_dn10 = assign14180_e8477_d_n10;
        locals.var_tmf2_dn11 = assign14180_e8477_d_n11;
        locals.var_tmf2_dn14 = assign14180_e8477_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14190_e8493, assign14190_e8493_d_n0, assign14190_e8493_d_n2, assign14190_e8493_d_n4, assign14190_e8493_d_n5, assign14190_e8493_d_n6, assign14190_e8493_d_n7, assign14190_e8493_d_n8, assign14190_e8493_d_n9, assign14190_e8493_d_n10, assign14190_e8493_d_n11, assign14190_e8493_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign14190_e8491, assign14190_e8491_d_n0, assign14190_e8491_d_n2, assign14190_e8491_d_n4, assign14190_e8491_d_n5, assign14190_e8491_d_n6, assign14190_e8491_d_n7, assign14190_e8491_d_n8, assign14190_e8491_d_n9, assign14190_e8491_d_n10, assign14190_e8491_d_n11, assign14190_e8491_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14190_e8490: f64 = (-locals.var_tmf2);
                (assign14190_e8490, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14190_e8491, assign14190_e8491_d_n0, assign14190_e8491_d_n2, assign14190_e8491_d_n4, assign14190_e8491_d_n5, assign14190_e8491_d_n6, assign14190_e8491_d_n7, assign14190_e8491_d_n8, assign14190_e8491_d_n9, assign14190_e8491_d_n10, assign14190_e8491_d_n11, assign14190_e8491_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14190_e8493;
        locals.var_tmf2_dn0 = assign14190_e8493_d_n0;
        locals.var_tmf2_dn2 = assign14190_e8493_d_n2;
        locals.var_tmf2_dn4 = assign14190_e8493_d_n4;
        locals.var_tmf2_dn5 = assign14190_e8493_d_n5;
        locals.var_tmf2_dn6 = assign14190_e8493_d_n6;
        locals.var_tmf2_dn7 = assign14190_e8493_d_n7;
        locals.var_tmf2_dn8 = assign14190_e8493_d_n8;
        locals.var_tmf2_dn9 = assign14190_e8493_d_n9;
        locals.var_tmf2_dn10 = assign14190_e8493_d_n10;
        locals.var_tmf2_dn11 = assign14190_e8493_d_n11;
        locals.var_tmf2_dn14 = assign14190_e8493_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14200_e8508, assign14200_e8508_d_n0, assign14200_e8508_d_n2, assign14200_e8508_d_n4, assign14200_e8508_d_n5, assign14200_e8508_d_n6, assign14200_e8508_d_n7, assign14200_e8508_d_n8, assign14200_e8508_d_n9, assign14200_e8508_d_n10, assign14200_e8508_d_n11, assign14200_e8508_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14200_e8503: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14200_e8505: f64 = (assign14200_e8503 + locals.var_tmf2);
        let assign14200_e8506: f64 = (assign14200_e8505).sqrt();
        (assign14200_e8506, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14200_e8506)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14200_e8508;
        locals.var_tmf2_dn0 = assign14200_e8508_d_n0;
        locals.var_tmf2_dn2 = assign14200_e8508_d_n2;
        locals.var_tmf2_dn4 = assign14200_e8508_d_n4;
        locals.var_tmf2_dn5 = assign14200_e8508_d_n5;
        locals.var_tmf2_dn6 = assign14200_e8508_d_n6;
        locals.var_tmf2_dn7 = assign14200_e8508_d_n7;
        locals.var_tmf2_dn8 = assign14200_e8508_d_n8;
        locals.var_tmf2_dn9 = assign14200_e8508_d_n9;
        locals.var_tmf2_dn10 = assign14200_e8508_d_n10;
        locals.var_tmf2_dn11 = assign14200_e8508_d_n11;
        locals.var_tmf2_dn14 = assign14200_e8508_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14210_e8524, assign14210_e8524_d_n0, assign14210_e8524_d_n2, assign14210_e8524_d_n4, assign14210_e8524_d_n5, assign14210_e8524_d_n6, assign14210_e8524_d_n7, assign14210_e8524_d_n8, assign14210_e8524_d_n9, assign14210_e8524_d_n10, assign14210_e8524_d_n11, assign14210_e8524_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14210_e8520: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14210_e8521: f64 = (1.0 + assign14210_e8520);
        let assign14210_e8522: f64 = (0.5 * assign14210_e8521);
        (assign14210_e8522, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14210_e8524;
        locals.var_t0_dn0 = assign14210_e8524_d_n0;
        locals.var_t0_dn2 = assign14210_e8524_d_n2;
        locals.var_t0_dn4 = assign14210_e8524_d_n4;
        locals.var_t0_dn5 = assign14210_e8524_d_n5;
        locals.var_t0_dn6 = assign14210_e8524_d_n6;
        locals.var_t0_dn7 = assign14210_e8524_d_n7;
        locals.var_t0_dn8 = assign14210_e8524_d_n8;
        locals.var_t0_dn9 = assign14210_e8524_d_n9;
        locals.var_t0_dn10 = assign14210_e8524_d_n10;
        locals.var_t0_dn11 = assign14210_e8524_d_n11;
        locals.var_t0_dn14 = assign14210_e8524_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14220_e8542, assign14220_e8542_d_n0, assign14220_e8542_d_n2, assign14220_e8542_d_n4, assign14220_e8542_d_n5, assign14220_e8542_d_n6, assign14220_e8542_d_n7, assign14220_e8542_d_n8, assign14220_e8542_d_n9, assign14220_e8542_d_n10, assign14220_e8542_d_n11, assign14220_e8542_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14220_e8534: f64 = (0.005 * locals.var_uc_rd);
        let assign14220_e8538: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14220_e8539: f64 = (0.5 * assign14220_e8538);
        let assign14220_e8540: f64 = (assign14220_e8534 + assign14220_e8539);
        (assign14220_e8540, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14220_e8542;
        locals.var_rde_dn0 = assign14220_e8542_d_n0;
        locals.var_rde_dn2 = assign14220_e8542_d_n2;
        locals.var_rde_dn4 = assign14220_e8542_d_n4;
        locals.var_rde_dn5 = assign14220_e8542_d_n5;
        locals.var_rde_dn6 = assign14220_e8542_d_n6;
        locals.var_rde_dn7 = assign14220_e8542_d_n7;
        locals.var_rde_dn8 = assign14220_e8542_d_n8;
        locals.var_rde_dn9 = assign14220_e8542_d_n9;
        locals.var_rde_dn10 = assign14220_e8542_d_n10;
        locals.var_rde_dn11 = assign14220_e8542_d_n11;
        locals.var_rde_dn14 = assign14220_e8542_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign14230_e8563, assign14230_e8563_d_n0, assign14230_e8563_d_n2, assign14230_e8563_d_n4, assign14230_e8563_d_n5, assign14230_e8563_d_n6, assign14230_e8563_d_n7, assign14230_e8563_d_n8, assign14230_e8563_d_n9, assign14230_e8563_d_n10, assign14230_e8563_d_n11, assign14230_e8563_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14230_e8554: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14230_e8555: f64 = (locals.var_uc_rd + assign14230_e8554);
        let assign14230_e8558: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14230_e8559: f64 = (assign14230_e8555 + assign14230_e8558);
        let assign14230_e8561: f64 = (assign14230_e8559 * locals.var_t2);
        (assign14230_e8561, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14230_e8563;
        locals.var_rde_dn0 = assign14230_e8563_d_n0;
        locals.var_rde_dn2 = assign14230_e8563_d_n2;
        locals.var_rde_dn4 = assign14230_e8563_d_n4;
        locals.var_rde_dn5 = assign14230_e8563_d_n5;
        locals.var_rde_dn6 = assign14230_e8563_d_n6;
        locals.var_rde_dn7 = assign14230_e8563_d_n7;
        locals.var_rde_dn8 = assign14230_e8563_d_n8;
        locals.var_rde_dn9 = assign14230_e8563_d_n9;
        locals.var_rde_dn10 = assign14230_e8563_d_n10;
        locals.var_rde_dn11 = assign14230_e8563_d_n11;
        locals.var_rde_dn14 = assign14230_e8563_d_n14;
        locals.var_rde_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14240_e8582, assign14240_e8582_d_n0, assign14240_e8582_d_n2, assign14240_e8582_d_n4, assign14240_e8582_d_n5, assign14240_e8582_d_n6, assign14240_e8582_d_n7, assign14240_e8582_d_n8, assign14240_e8582_d_n9, assign14240_e8582_d_n10, assign14240_e8582_d_n11, assign14240_e8582_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14240_e8575: f64 = (0.005 * locals.var_uc_rd);
        let assign14240_e8576: f64 = (locals.var_rde - assign14240_e8575);
        let assign14240_e8579: f64 = (0.01 * locals.var_uc_rd);
        let assign14240_e8580: f64 = (assign14240_e8576 - assign14240_e8579);
        (assign14240_e8580, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14240_e8582;
        locals.var_tmf1_dn0 = assign14240_e8582_d_n0;
        locals.var_tmf1_dn2 = assign14240_e8582_d_n2;
        locals.var_tmf1_dn4 = assign14240_e8582_d_n4;
        locals.var_tmf1_dn5 = assign14240_e8582_d_n5;
        locals.var_tmf1_dn6 = assign14240_e8582_d_n6;
        locals.var_tmf1_dn7 = assign14240_e8582_d_n7;
        locals.var_tmf1_dn8 = assign14240_e8582_d_n8;
        locals.var_tmf1_dn9 = assign14240_e8582_d_n9;
        locals.var_tmf1_dn10 = assign14240_e8582_d_n10;
        locals.var_tmf1_dn11 = assign14240_e8582_d_n11;
        locals.var_tmf1_dn14 = assign14240_e8582_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14250_e8601, assign14250_e8601_d_n0, assign14250_e8601_d_n2, assign14250_e8601_d_n4, assign14250_e8601_d_n5, assign14250_e8601_d_n6, assign14250_e8601_d_n7, assign14250_e8601_d_n8, assign14250_e8601_d_n9, assign14250_e8601_d_n10, assign14250_e8601_d_n11, assign14250_e8601_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14250_e8594: f64 = (0.005 * locals.var_uc_rd);
        let assign14250_e8595: f64 = (4.0 * assign14250_e8594);
        let assign14250_e8598: f64 = (0.01 * locals.var_uc_rd);
        let assign14250_e8599: f64 = (assign14250_e8595 * assign14250_e8598);
        (assign14250_e8599, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14250_e8601;
        locals.var_tmf2_dn0 = assign14250_e8601_d_n0;
        locals.var_tmf2_dn2 = assign14250_e8601_d_n2;
        locals.var_tmf2_dn4 = assign14250_e8601_d_n4;
        locals.var_tmf2_dn5 = assign14250_e8601_d_n5;
        locals.var_tmf2_dn6 = assign14250_e8601_d_n6;
        locals.var_tmf2_dn7 = assign14250_e8601_d_n7;
        locals.var_tmf2_dn8 = assign14250_e8601_d_n8;
        locals.var_tmf2_dn9 = assign14250_e8601_d_n9;
        locals.var_tmf2_dn10 = assign14250_e8601_d_n10;
        locals.var_tmf2_dn11 = assign14250_e8601_d_n11;
        locals.var_tmf2_dn14 = assign14250_e8601_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14260_e8618, assign14260_e8618_d_n0, assign14260_e8618_d_n2, assign14260_e8618_d_n4, assign14260_e8618_d_n5, assign14260_e8618_d_n6, assign14260_e8618_d_n7, assign14260_e8618_d_n8, assign14260_e8618_d_n9, assign14260_e8618_d_n10, assign14260_e8618_d_n11, assign14260_e8618_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign14260_e8616, assign14260_e8616_d_n0, assign14260_e8616_d_n2, assign14260_e8616_d_n4, assign14260_e8616_d_n5, assign14260_e8616_d_n6, assign14260_e8616_d_n7, assign14260_e8616_d_n8, assign14260_e8616_d_n9, assign14260_e8616_d_n10, assign14260_e8616_d_n11, assign14260_e8616_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14260_e8615: f64 = (-locals.var_tmf2);
                (assign14260_e8615, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14260_e8616, assign14260_e8616_d_n0, assign14260_e8616_d_n2, assign14260_e8616_d_n4, assign14260_e8616_d_n5, assign14260_e8616_d_n6, assign14260_e8616_d_n7, assign14260_e8616_d_n8, assign14260_e8616_d_n9, assign14260_e8616_d_n10, assign14260_e8616_d_n11, assign14260_e8616_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14260_e8618;
        locals.var_tmf2_dn0 = assign14260_e8618_d_n0;
        locals.var_tmf2_dn2 = assign14260_e8618_d_n2;
        locals.var_tmf2_dn4 = assign14260_e8618_d_n4;
        locals.var_tmf2_dn5 = assign14260_e8618_d_n5;
        locals.var_tmf2_dn6 = assign14260_e8618_d_n6;
        locals.var_tmf2_dn7 = assign14260_e8618_d_n7;
        locals.var_tmf2_dn8 = assign14260_e8618_d_n8;
        locals.var_tmf2_dn9 = assign14260_e8618_d_n9;
        locals.var_tmf2_dn10 = assign14260_e8618_d_n10;
        locals.var_tmf2_dn11 = assign14260_e8618_d_n11;
        locals.var_tmf2_dn14 = assign14260_e8618_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14270_e8634, assign14270_e8634_d_n0, assign14270_e8634_d_n2, assign14270_e8634_d_n4, assign14270_e8634_d_n5, assign14270_e8634_d_n6, assign14270_e8634_d_n7, assign14270_e8634_d_n8, assign14270_e8634_d_n9, assign14270_e8634_d_n10, assign14270_e8634_d_n11, assign14270_e8634_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14270_e8629: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14270_e8631: f64 = (assign14270_e8629 + locals.var_tmf2);
        let assign14270_e8632: f64 = (assign14270_e8631).sqrt();
        (assign14270_e8632, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14270_e8632)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14270_e8634;
        locals.var_tmf2_dn0 = assign14270_e8634_d_n0;
        locals.var_tmf2_dn2 = assign14270_e8634_d_n2;
        locals.var_tmf2_dn4 = assign14270_e8634_d_n4;
        locals.var_tmf2_dn5 = assign14270_e8634_d_n5;
        locals.var_tmf2_dn6 = assign14270_e8634_d_n6;
        locals.var_tmf2_dn7 = assign14270_e8634_d_n7;
        locals.var_tmf2_dn8 = assign14270_e8634_d_n8;
        locals.var_tmf2_dn9 = assign14270_e8634_d_n9;
        locals.var_tmf2_dn10 = assign14270_e8634_d_n10;
        locals.var_tmf2_dn11 = assign14270_e8634_d_n11;
        locals.var_tmf2_dn14 = assign14270_e8634_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14280_e8651, assign14280_e8651_d_n0, assign14280_e8651_d_n2, assign14280_e8651_d_n4, assign14280_e8651_d_n5, assign14280_e8651_d_n6, assign14280_e8651_d_n7, assign14280_e8651_d_n8, assign14280_e8651_d_n9, assign14280_e8651_d_n10, assign14280_e8651_d_n11, assign14280_e8651_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14280_e8647: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14280_e8648: f64 = (1.0 + assign14280_e8647);
        let assign14280_e8649: f64 = (0.5 * assign14280_e8648);
        (assign14280_e8649, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14280_e8651;
        locals.var_t0_dn0 = assign14280_e8651_d_n0;
        locals.var_t0_dn2 = assign14280_e8651_d_n2;
        locals.var_t0_dn4 = assign14280_e8651_d_n4;
        locals.var_t0_dn5 = assign14280_e8651_d_n5;
        locals.var_t0_dn6 = assign14280_e8651_d_n6;
        locals.var_t0_dn7 = assign14280_e8651_d_n7;
        locals.var_t0_dn8 = assign14280_e8651_d_n8;
        locals.var_t0_dn9 = assign14280_e8651_d_n9;
        locals.var_t0_dn10 = assign14280_e8651_d_n10;
        locals.var_t0_dn11 = assign14280_e8651_d_n11;
        locals.var_t0_dn14 = assign14280_e8651_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14290_e8670, assign14290_e8670_d_n0, assign14290_e8670_d_n2, assign14290_e8670_d_n4, assign14290_e8670_d_n5, assign14290_e8670_d_n6, assign14290_e8670_d_n7, assign14290_e8670_d_n8, assign14290_e8670_d_n9, assign14290_e8670_d_n10, assign14290_e8670_d_n11, assign14290_e8670_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14290_e8662: f64 = (0.005 * locals.var_uc_rd);
        let assign14290_e8666: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14290_e8667: f64 = (0.5 * assign14290_e8666);
        let assign14290_e8668: f64 = (assign14290_e8662 + assign14290_e8667);
        (assign14290_e8668, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14290_e8670;
        locals.var_rde_dn0 = assign14290_e8670_d_n0;
        locals.var_rde_dn2 = assign14290_e8670_d_n2;
        locals.var_rde_dn4 = assign14290_e8670_d_n4;
        locals.var_rde_dn5 = assign14290_e8670_d_n5;
        locals.var_rde_dn6 = assign14290_e8670_d_n6;
        locals.var_rde_dn7 = assign14290_e8670_d_n7;
        locals.var_rde_dn8 = assign14290_e8670_d_n8;
        locals.var_rde_dn9 = assign14290_e8670_d_n9;
        locals.var_rde_dn10 = assign14290_e8670_d_n10;
        locals.var_rde_dn11 = assign14290_e8670_d_n11;
        locals.var_rde_dn14 = assign14290_e8670_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign14300_e8679, assign14300_e8679_d_n0, assign14300_e8679_d_n2, assign14300_e8679_d_n4, assign14300_e8679_d_n5, assign14300_e8679_d_n6, assign14300_e8679_d_n7, assign14300_e8679_d_n8, assign14300_e8679_d_n9, assign14300_e8679_d_n10, assign14300_e8679_d_n11, assign14300_e8679_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14300_e8679;
        locals.var_rde_dn0 = assign14300_e8679_d_n0;
        locals.var_rde_dn2 = assign14300_e8679_d_n2;
        locals.var_rde_dn4 = assign14300_e8679_d_n4;
        locals.var_rde_dn5 = assign14300_e8679_d_n5;
        locals.var_rde_dn6 = assign14300_e8679_d_n6;
        locals.var_rde_dn7 = assign14300_e8679_d_n7;
        locals.var_rde_dn8 = assign14300_e8679_d_n8;
        locals.var_rde_dn9 = assign14300_e8679_d_n9;
        locals.var_rde_dn10 = assign14300_e8679_d_n10;
        locals.var_rde_dn11 = assign14300_e8679_d_n11;
        locals.var_rde_dn14 = assign14300_e8679_d_n14;
        locals.var_rde_rv = 0.0;

        let assign14310_e8682: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign14310_e8682;
        locals.var_guard318_rv = 0.0;

        let (assign14320_e8706, assign14320_e8706_d_n0, assign14320_e8706_d_n2, assign14320_e8706_d_n4, assign14320_e8706_d_n5, assign14320_e8706_d_n6, assign14320_e8706_d_n7, assign14320_e8706_d_n8, assign14320_e8706_d_n9, assign14320_e8706_d_n10, assign14320_e8706_d_n11, assign14320_e8706_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14320_e8691: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14320_e8693: f64 = (assign14320_e8691 * 1000000.0);
        let assign14320_e8695: f64 = (assign14320_e8693 + locals.var_uc_rdict1);
        let assign14320_e8696: f64 = (locals.var_rdtemp0 * assign14320_e8695);
        let assign14320_e8699: f64 = (p.p70 * p.p100);
        let assign14320_e8701: f64 = (assign14320_e8699 * 1000000.0);
        let assign14320_e8703: f64 = (assign14320_e8701 + p.p101);
        let assign14320_e8704: f64 = (assign14320_e8696 * assign14320_e8703);
        (assign14320_e8704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14320_e8706;
        locals.var_t2_dn0 = assign14320_e8706_d_n0;
        locals.var_t2_dn2 = assign14320_e8706_d_n2;
        locals.var_t2_dn4 = assign14320_e8706_d_n4;
        locals.var_t2_dn5 = assign14320_e8706_d_n5;
        locals.var_t2_dn6 = assign14320_e8706_d_n6;
        locals.var_t2_dn7 = assign14320_e8706_d_n7;
        locals.var_t2_dn8 = assign14320_e8706_d_n8;
        locals.var_t2_dn9 = assign14320_e8706_d_n9;
        locals.var_t2_dn10 = assign14320_e8706_d_n10;
        locals.var_t2_dn11 = assign14320_e8706_d_n11;
        locals.var_t2_dn14 = assign14320_e8706_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14330_e8709: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign14330_e8709;
        locals.var_guard319_rv = 0.0;

        let (assign14340_e8729, assign14340_e8729_d_n0, assign14340_e8729_d_n2, assign14340_e8729_d_n4, assign14340_e8729_d_n5, assign14340_e8729_d_n6, assign14340_e8729_d_n7, assign14340_e8729_d_n8, assign14340_e8729_d_n9, assign14340_e8729_d_n10, assign14340_e8729_d_n11, assign14340_e8729_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14340_e8720: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14340_e8721: f64 = (locals.var_uc_rs + assign14340_e8720);
        let assign14340_e8724: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14340_e8725: f64 = (assign14340_e8721 + assign14340_e8724);
        let assign14340_e8727: f64 = (assign14340_e8725 * locals.var_t2);
        (assign14340_e8727, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14340_e8729;
        locals.var_rse_dn0 = assign14340_e8729_d_n0;
        locals.var_rse_dn2 = assign14340_e8729_d_n2;
        locals.var_rse_dn4 = assign14340_e8729_d_n4;
        locals.var_rse_dn5 = assign14340_e8729_d_n5;
        locals.var_rse_dn6 = assign14340_e8729_d_n6;
        locals.var_rse_dn7 = assign14340_e8729_d_n7;
        locals.var_rse_dn8 = assign14340_e8729_d_n8;
        locals.var_rse_dn9 = assign14340_e8729_d_n9;
        locals.var_rse_dn10 = assign14340_e8729_d_n10;
        locals.var_rse_dn11 = assign14340_e8729_d_n11;
        locals.var_rse_dn14 = assign14340_e8729_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign14350_e8747, assign14350_e8747_d_n0, assign14350_e8747_d_n2, assign14350_e8747_d_n4, assign14350_e8747_d_n5, assign14350_e8747_d_n6, assign14350_e8747_d_n7, assign14350_e8747_d_n8, assign14350_e8747_d_n9, assign14350_e8747_d_n10, assign14350_e8747_d_n11, assign14350_e8747_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14350_e8740: f64 = (0.005 * locals.var_uc_rs);
        let assign14350_e8741: f64 = (locals.var_rse - assign14350_e8740);
        let assign14350_e8744: f64 = (0.01 * locals.var_uc_rs);
        let assign14350_e8745: f64 = (assign14350_e8741 - assign14350_e8744);
        (assign14350_e8745, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14350_e8747;
        locals.var_tmf1_dn0 = assign14350_e8747_d_n0;
        locals.var_tmf1_dn2 = assign14350_e8747_d_n2;
        locals.var_tmf1_dn4 = assign14350_e8747_d_n4;
        locals.var_tmf1_dn5 = assign14350_e8747_d_n5;
        locals.var_tmf1_dn6 = assign14350_e8747_d_n6;
        locals.var_tmf1_dn7 = assign14350_e8747_d_n7;
        locals.var_tmf1_dn8 = assign14350_e8747_d_n8;
        locals.var_tmf1_dn9 = assign14350_e8747_d_n9;
        locals.var_tmf1_dn10 = assign14350_e8747_d_n10;
        locals.var_tmf1_dn11 = assign14350_e8747_d_n11;
        locals.var_tmf1_dn14 = assign14350_e8747_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14360_e8765, assign14360_e8765_d_n0, assign14360_e8765_d_n2, assign14360_e8765_d_n4, assign14360_e8765_d_n5, assign14360_e8765_d_n6, assign14360_e8765_d_n7, assign14360_e8765_d_n8, assign14360_e8765_d_n9, assign14360_e8765_d_n10, assign14360_e8765_d_n11, assign14360_e8765_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14360_e8758: f64 = (0.005 * locals.var_uc_rs);
        let assign14360_e8759: f64 = (4.0 * assign14360_e8758);
        let assign14360_e8762: f64 = (0.01 * locals.var_uc_rs);
        let assign14360_e8763: f64 = (assign14360_e8759 * assign14360_e8762);
        (assign14360_e8763, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14360_e8765;
        locals.var_tmf2_dn0 = assign14360_e8765_d_n0;
        locals.var_tmf2_dn2 = assign14360_e8765_d_n2;
        locals.var_tmf2_dn4 = assign14360_e8765_d_n4;
        locals.var_tmf2_dn5 = assign14360_e8765_d_n5;
        locals.var_tmf2_dn6 = assign14360_e8765_d_n6;
        locals.var_tmf2_dn7 = assign14360_e8765_d_n7;
        locals.var_tmf2_dn8 = assign14360_e8765_d_n8;
        locals.var_tmf2_dn9 = assign14360_e8765_d_n9;
        locals.var_tmf2_dn10 = assign14360_e8765_d_n10;
        locals.var_tmf2_dn11 = assign14360_e8765_d_n11;
        locals.var_tmf2_dn14 = assign14360_e8765_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14370_e8781, assign14370_e8781_d_n0, assign14370_e8781_d_n2, assign14370_e8781_d_n4, assign14370_e8781_d_n5, assign14370_e8781_d_n6, assign14370_e8781_d_n7, assign14370_e8781_d_n8, assign14370_e8781_d_n9, assign14370_e8781_d_n10, assign14370_e8781_d_n11, assign14370_e8781_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let (assign14370_e8779, assign14370_e8779_d_n0, assign14370_e8779_d_n2, assign14370_e8779_d_n4, assign14370_e8779_d_n5, assign14370_e8779_d_n6, assign14370_e8779_d_n7, assign14370_e8779_d_n8, assign14370_e8779_d_n9, assign14370_e8779_d_n10, assign14370_e8779_d_n11, assign14370_e8779_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14370_e8778: f64 = (-locals.var_tmf2);
                (assign14370_e8778, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14370_e8779, assign14370_e8779_d_n0, assign14370_e8779_d_n2, assign14370_e8779_d_n4, assign14370_e8779_d_n5, assign14370_e8779_d_n6, assign14370_e8779_d_n7, assign14370_e8779_d_n8, assign14370_e8779_d_n9, assign14370_e8779_d_n10, assign14370_e8779_d_n11, assign14370_e8779_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14370_e8781;
        locals.var_tmf2_dn0 = assign14370_e8781_d_n0;
        locals.var_tmf2_dn2 = assign14370_e8781_d_n2;
        locals.var_tmf2_dn4 = assign14370_e8781_d_n4;
        locals.var_tmf2_dn5 = assign14370_e8781_d_n5;
        locals.var_tmf2_dn6 = assign14370_e8781_d_n6;
        locals.var_tmf2_dn7 = assign14370_e8781_d_n7;
        locals.var_tmf2_dn8 = assign14370_e8781_d_n8;
        locals.var_tmf2_dn9 = assign14370_e8781_d_n9;
        locals.var_tmf2_dn10 = assign14370_e8781_d_n10;
        locals.var_tmf2_dn11 = assign14370_e8781_d_n11;
        locals.var_tmf2_dn14 = assign14370_e8781_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14380_e8796, assign14380_e8796_d_n0, assign14380_e8796_d_n2, assign14380_e8796_d_n4, assign14380_e8796_d_n5, assign14380_e8796_d_n6, assign14380_e8796_d_n7, assign14380_e8796_d_n8, assign14380_e8796_d_n9, assign14380_e8796_d_n10, assign14380_e8796_d_n11, assign14380_e8796_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14380_e8791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14380_e8793: f64 = (assign14380_e8791 + locals.var_tmf2);
        let assign14380_e8794: f64 = (assign14380_e8793).sqrt();
        (assign14380_e8794, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14380_e8794)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14380_e8796;
        locals.var_tmf2_dn0 = assign14380_e8796_d_n0;
        locals.var_tmf2_dn2 = assign14380_e8796_d_n2;
        locals.var_tmf2_dn4 = assign14380_e8796_d_n4;
        locals.var_tmf2_dn5 = assign14380_e8796_d_n5;
        locals.var_tmf2_dn6 = assign14380_e8796_d_n6;
        locals.var_tmf2_dn7 = assign14380_e8796_d_n7;
        locals.var_tmf2_dn8 = assign14380_e8796_d_n8;
        locals.var_tmf2_dn9 = assign14380_e8796_d_n9;
        locals.var_tmf2_dn10 = assign14380_e8796_d_n10;
        locals.var_tmf2_dn11 = assign14380_e8796_d_n11;
        locals.var_tmf2_dn14 = assign14380_e8796_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14390_e8812, assign14390_e8812_d_n0, assign14390_e8812_d_n2, assign14390_e8812_d_n4, assign14390_e8812_d_n5, assign14390_e8812_d_n6, assign14390_e8812_d_n7, assign14390_e8812_d_n8, assign14390_e8812_d_n9, assign14390_e8812_d_n10, assign14390_e8812_d_n11, assign14390_e8812_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14390_e8808: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14390_e8809: f64 = (1.0 + assign14390_e8808);
        let assign14390_e8810: f64 = (0.5 * assign14390_e8809);
        (assign14390_e8810, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14390_e8812;
        locals.var_t0_dn0 = assign14390_e8812_d_n0;
        locals.var_t0_dn2 = assign14390_e8812_d_n2;
        locals.var_t0_dn4 = assign14390_e8812_d_n4;
        locals.var_t0_dn5 = assign14390_e8812_d_n5;
        locals.var_t0_dn6 = assign14390_e8812_d_n6;
        locals.var_t0_dn7 = assign14390_e8812_d_n7;
        locals.var_t0_dn8 = assign14390_e8812_d_n8;
        locals.var_t0_dn9 = assign14390_e8812_d_n9;
        locals.var_t0_dn10 = assign14390_e8812_d_n10;
        locals.var_t0_dn11 = assign14390_e8812_d_n11;
        locals.var_t0_dn14 = assign14390_e8812_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14400_e8830, assign14400_e8830_d_n0, assign14400_e8830_d_n2, assign14400_e8830_d_n4, assign14400_e8830_d_n5, assign14400_e8830_d_n6, assign14400_e8830_d_n7, assign14400_e8830_d_n8, assign14400_e8830_d_n9, assign14400_e8830_d_n10, assign14400_e8830_d_n11, assign14400_e8830_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14400_e8822: f64 = (0.005 * locals.var_uc_rs);
        let assign14400_e8826: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14400_e8827: f64 = (0.5 * assign14400_e8826);
        let assign14400_e8828: f64 = (assign14400_e8822 + assign14400_e8827);
        (assign14400_e8828, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14400_e8830;
        locals.var_rse_dn0 = assign14400_e8830_d_n0;
        locals.var_rse_dn2 = assign14400_e8830_d_n2;
        locals.var_rse_dn4 = assign14400_e8830_d_n4;
        locals.var_rse_dn5 = assign14400_e8830_d_n5;
        locals.var_rse_dn6 = assign14400_e8830_d_n6;
        locals.var_rse_dn7 = assign14400_e8830_d_n7;
        locals.var_rse_dn8 = assign14400_e8830_d_n8;
        locals.var_rse_dn9 = assign14400_e8830_d_n9;
        locals.var_rse_dn10 = assign14400_e8830_d_n10;
        locals.var_rse_dn11 = assign14400_e8830_d_n11;
        locals.var_rse_dn14 = assign14400_e8830_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign14410_e8851, assign14410_e8851_d_n0, assign14410_e8851_d_n2, assign14410_e8851_d_n4, assign14410_e8851_d_n5, assign14410_e8851_d_n6, assign14410_e8851_d_n7, assign14410_e8851_d_n8, assign14410_e8851_d_n9, assign14410_e8851_d_n10, assign14410_e8851_d_n11, assign14410_e8851_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14410_e8842: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14410_e8843: f64 = (locals.var_uc_rs + assign14410_e8842);
        let assign14410_e8846: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14410_e8847: f64 = (assign14410_e8843 + assign14410_e8846);
        let assign14410_e8849: f64 = (assign14410_e8847 * locals.var_t2);
        (assign14410_e8849, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14410_e8851;
        locals.var_rse_dn0 = assign14410_e8851_d_n0;
        locals.var_rse_dn2 = assign14410_e8851_d_n2;
        locals.var_rse_dn4 = assign14410_e8851_d_n4;
        locals.var_rse_dn5 = assign14410_e8851_d_n5;
        locals.var_rse_dn6 = assign14410_e8851_d_n6;
        locals.var_rse_dn7 = assign14410_e8851_d_n7;
        locals.var_rse_dn8 = assign14410_e8851_d_n8;
        locals.var_rse_dn9 = assign14410_e8851_d_n9;
        locals.var_rse_dn10 = assign14410_e8851_d_n10;
        locals.var_rse_dn11 = assign14410_e8851_d_n11;
        locals.var_rse_dn14 = assign14410_e8851_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign14420_e8870, assign14420_e8870_d_n0, assign14420_e8870_d_n2, assign14420_e8870_d_n4, assign14420_e8870_d_n5, assign14420_e8870_d_n6, assign14420_e8870_d_n7, assign14420_e8870_d_n8, assign14420_e8870_d_n9, assign14420_e8870_d_n10, assign14420_e8870_d_n11, assign14420_e8870_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14420_e8863: f64 = (0.005 * locals.var_uc_rs);
        let assign14420_e8864: f64 = (locals.var_rse - assign14420_e8863);
        let assign14420_e8867: f64 = (0.01 * locals.var_uc_rs);
        let assign14420_e8868: f64 = (assign14420_e8864 - assign14420_e8867);
        (assign14420_e8868, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14420_e8870;
        locals.var_tmf1_dn0 = assign14420_e8870_d_n0;
        locals.var_tmf1_dn2 = assign14420_e8870_d_n2;
        locals.var_tmf1_dn4 = assign14420_e8870_d_n4;
        locals.var_tmf1_dn5 = assign14420_e8870_d_n5;
        locals.var_tmf1_dn6 = assign14420_e8870_d_n6;
        locals.var_tmf1_dn7 = assign14420_e8870_d_n7;
        locals.var_tmf1_dn8 = assign14420_e8870_d_n8;
        locals.var_tmf1_dn9 = assign14420_e8870_d_n9;
        locals.var_tmf1_dn10 = assign14420_e8870_d_n10;
        locals.var_tmf1_dn11 = assign14420_e8870_d_n11;
        locals.var_tmf1_dn14 = assign14420_e8870_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14430_e8889, assign14430_e8889_d_n0, assign14430_e8889_d_n2, assign14430_e8889_d_n4, assign14430_e8889_d_n5, assign14430_e8889_d_n6, assign14430_e8889_d_n7, assign14430_e8889_d_n8, assign14430_e8889_d_n9, assign14430_e8889_d_n10, assign14430_e8889_d_n11, assign14430_e8889_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14430_e8882: f64 = (0.005 * locals.var_uc_rs);
        let assign14430_e8883: f64 = (4.0 * assign14430_e8882);
        let assign14430_e8886: f64 = (0.01 * locals.var_uc_rs);
        let assign14430_e8887: f64 = (assign14430_e8883 * assign14430_e8886);
        (assign14430_e8887, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14430_e8889;
        locals.var_tmf2_dn0 = assign14430_e8889_d_n0;
        locals.var_tmf2_dn2 = assign14430_e8889_d_n2;
        locals.var_tmf2_dn4 = assign14430_e8889_d_n4;
        locals.var_tmf2_dn5 = assign14430_e8889_d_n5;
        locals.var_tmf2_dn6 = assign14430_e8889_d_n6;
        locals.var_tmf2_dn7 = assign14430_e8889_d_n7;
        locals.var_tmf2_dn8 = assign14430_e8889_d_n8;
        locals.var_tmf2_dn9 = assign14430_e8889_d_n9;
        locals.var_tmf2_dn10 = assign14430_e8889_d_n10;
        locals.var_tmf2_dn11 = assign14430_e8889_d_n11;
        locals.var_tmf2_dn14 = assign14430_e8889_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14440_e8906, assign14440_e8906_d_n0, assign14440_e8906_d_n2, assign14440_e8906_d_n4, assign14440_e8906_d_n5, assign14440_e8906_d_n6, assign14440_e8906_d_n7, assign14440_e8906_d_n8, assign14440_e8906_d_n9, assign14440_e8906_d_n10, assign14440_e8906_d_n11, assign14440_e8906_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let (assign14440_e8904, assign14440_e8904_d_n0, assign14440_e8904_d_n2, assign14440_e8904_d_n4, assign14440_e8904_d_n5, assign14440_e8904_d_n6, assign14440_e8904_d_n7, assign14440_e8904_d_n8, assign14440_e8904_d_n9, assign14440_e8904_d_n10, assign14440_e8904_d_n11, assign14440_e8904_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14440_e8903: f64 = (-locals.var_tmf2);
                (assign14440_e8903, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14440_e8904, assign14440_e8904_d_n0, assign14440_e8904_d_n2, assign14440_e8904_d_n4, assign14440_e8904_d_n5, assign14440_e8904_d_n6, assign14440_e8904_d_n7, assign14440_e8904_d_n8, assign14440_e8904_d_n9, assign14440_e8904_d_n10, assign14440_e8904_d_n11, assign14440_e8904_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14440_e8906;
        locals.var_tmf2_dn0 = assign14440_e8906_d_n0;
        locals.var_tmf2_dn2 = assign14440_e8906_d_n2;
        locals.var_tmf2_dn4 = assign14440_e8906_d_n4;
        locals.var_tmf2_dn5 = assign14440_e8906_d_n5;
        locals.var_tmf2_dn6 = assign14440_e8906_d_n6;
        locals.var_tmf2_dn7 = assign14440_e8906_d_n7;
        locals.var_tmf2_dn8 = assign14440_e8906_d_n8;
        locals.var_tmf2_dn9 = assign14440_e8906_d_n9;
        locals.var_tmf2_dn10 = assign14440_e8906_d_n10;
        locals.var_tmf2_dn11 = assign14440_e8906_d_n11;
        locals.var_tmf2_dn14 = assign14440_e8906_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14450_e8922, assign14450_e8922_d_n0, assign14450_e8922_d_n2, assign14450_e8922_d_n4, assign14450_e8922_d_n5, assign14450_e8922_d_n6, assign14450_e8922_d_n7, assign14450_e8922_d_n8, assign14450_e8922_d_n9, assign14450_e8922_d_n10, assign14450_e8922_d_n11, assign14450_e8922_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14450_e8917: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14450_e8919: f64 = (assign14450_e8917 + locals.var_tmf2);
        let assign14450_e8920: f64 = (assign14450_e8919).sqrt();
        (assign14450_e8920, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14450_e8920)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14450_e8922;
        locals.var_tmf2_dn0 = assign14450_e8922_d_n0;
        locals.var_tmf2_dn2 = assign14450_e8922_d_n2;
        locals.var_tmf2_dn4 = assign14450_e8922_d_n4;
        locals.var_tmf2_dn5 = assign14450_e8922_d_n5;
        locals.var_tmf2_dn6 = assign14450_e8922_d_n6;
        locals.var_tmf2_dn7 = assign14450_e8922_d_n7;
        locals.var_tmf2_dn8 = assign14450_e8922_d_n8;
        locals.var_tmf2_dn9 = assign14450_e8922_d_n9;
        locals.var_tmf2_dn10 = assign14450_e8922_d_n10;
        locals.var_tmf2_dn11 = assign14450_e8922_d_n11;
        locals.var_tmf2_dn14 = assign14450_e8922_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14460_e8939, assign14460_e8939_d_n0, assign14460_e8939_d_n2, assign14460_e8939_d_n4, assign14460_e8939_d_n5, assign14460_e8939_d_n6, assign14460_e8939_d_n7, assign14460_e8939_d_n8, assign14460_e8939_d_n9, assign14460_e8939_d_n10, assign14460_e8939_d_n11, assign14460_e8939_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14460_e8935: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14460_e8936: f64 = (1.0 + assign14460_e8935);
        let assign14460_e8937: f64 = (0.5 * assign14460_e8936);
        (assign14460_e8937, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14460_e8939;
        locals.var_t0_dn0 = assign14460_e8939_d_n0;
        locals.var_t0_dn2 = assign14460_e8939_d_n2;
        locals.var_t0_dn4 = assign14460_e8939_d_n4;
        locals.var_t0_dn5 = assign14460_e8939_d_n5;
        locals.var_t0_dn6 = assign14460_e8939_d_n6;
        locals.var_t0_dn7 = assign14460_e8939_d_n7;
        locals.var_t0_dn8 = assign14460_e8939_d_n8;
        locals.var_t0_dn9 = assign14460_e8939_d_n9;
        locals.var_t0_dn10 = assign14460_e8939_d_n10;
        locals.var_t0_dn11 = assign14460_e8939_d_n11;
        locals.var_t0_dn14 = assign14460_e8939_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14470_e8958, assign14470_e8958_d_n0, assign14470_e8958_d_n2, assign14470_e8958_d_n4, assign14470_e8958_d_n5, assign14470_e8958_d_n6, assign14470_e8958_d_n7, assign14470_e8958_d_n8, assign14470_e8958_d_n9, assign14470_e8958_d_n10, assign14470_e8958_d_n11, assign14470_e8958_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14470_e8950: f64 = (0.005 * locals.var_uc_rs);
        let assign14470_e8954: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14470_e8955: f64 = (0.5 * assign14470_e8954);
        let assign14470_e8956: f64 = (assign14470_e8950 + assign14470_e8955);
        (assign14470_e8956, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14470_e8958;
        locals.var_rse_dn0 = assign14470_e8958_d_n0;
        locals.var_rse_dn2 = assign14470_e8958_d_n2;
        locals.var_rse_dn4 = assign14470_e8958_d_n4;
        locals.var_rse_dn5 = assign14470_e8958_d_n5;
        locals.var_rse_dn6 = assign14470_e8958_d_n6;
        locals.var_rse_dn7 = assign14470_e8958_d_n7;
        locals.var_rse_dn8 = assign14470_e8958_d_n8;
        locals.var_rse_dn9 = assign14470_e8958_d_n9;
        locals.var_rse_dn10 = assign14470_e8958_d_n10;
        locals.var_rse_dn11 = assign14470_e8958_d_n11;
        locals.var_rse_dn14 = assign14470_e8958_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign14480_e8967, assign14480_e8967_d_n0, assign14480_e8967_d_n2, assign14480_e8967_d_n4, assign14480_e8967_d_n5, assign14480_e8967_d_n6, assign14480_e8967_d_n7, assign14480_e8967_d_n8, assign14480_e8967_d_n9, assign14480_e8967_d_n10, assign14480_e8967_d_n11, assign14480_e8967_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14480_e8967;
        locals.var_rse_dn0 = assign14480_e8967_d_n0;
        locals.var_rse_dn2 = assign14480_e8967_d_n2;
        locals.var_rse_dn4 = assign14480_e8967_d_n4;
        locals.var_rse_dn5 = assign14480_e8967_d_n5;
        locals.var_rse_dn6 = assign14480_e8967_d_n6;
        locals.var_rse_dn7 = assign14480_e8967_d_n7;
        locals.var_rse_dn8 = assign14480_e8967_d_n8;
        locals.var_rse_dn9 = assign14480_e8967_d_n9;
        locals.var_rse_dn10 = assign14480_e8967_d_n10;
        locals.var_rse_dn11 = assign14480_e8967_d_n11;
        locals.var_rse_dn14 = assign14480_e8967_d_n14;
        locals.var_rse_rv = 0.0;

        let assign14490_e8970: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign14490_e8970;
        locals.var_guard320_rv = 0.0;

        let (assign14500_e8994, assign14500_e8994_d_n0, assign14500_e8994_d_n2, assign14500_e8994_d_n4, assign14500_e8994_d_n5, assign14500_e8994_d_n6, assign14500_e8994_d_n7, assign14500_e8994_d_n8, assign14500_e8994_d_n9, assign14500_e8994_d_n10, assign14500_e8994_d_n11, assign14500_e8994_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14500_e8979: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14500_e8981: f64 = (assign14500_e8979 * 1000000.0);
        let assign14500_e8983: f64 = (assign14500_e8981 + locals.var_uc_rdict1);
        let assign14500_e8984: f64 = (locals.var_rdvdtemp0 * assign14500_e8983);
        let assign14500_e8987: f64 = (p.p68 * p.p100);
        let assign14500_e8989: f64 = (assign14500_e8987 * 1000000.0);
        let assign14500_e8991: f64 = (assign14500_e8989 + p.p101);
        let assign14500_e8992: f64 = (assign14500_e8984 * assign14500_e8991);
        (assign14500_e8992, ((locals.var_rdvdtemp0_dn0 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn2 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn4 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn5 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn6 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn7 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn8 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn9 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn10 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn11 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn14 * assign14500_e8983) * assign14500_e8991),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14500_e8994;
        locals.var_t4_dn0 = assign14500_e8994_d_n0;
        locals.var_t4_dn2 = assign14500_e8994_d_n2;
        locals.var_t4_dn4 = assign14500_e8994_d_n4;
        locals.var_t4_dn5 = assign14500_e8994_d_n5;
        locals.var_t4_dn6 = assign14500_e8994_d_n6;
        locals.var_t4_dn7 = assign14500_e8994_d_n7;
        locals.var_t4_dn8 = assign14500_e8994_d_n8;
        locals.var_t4_dn9 = assign14500_e8994_d_n9;
        locals.var_t4_dn10 = assign14500_e8994_d_n10;
        locals.var_t4_dn11 = assign14500_e8994_d_n11;
        locals.var_t4_dn14 = assign14500_e8994_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign14510_e9008, assign14510_e9008_d_n0, assign14510_e9008_d_n2, assign14510_e9008_d_n4, assign14510_e9008_d_n5, assign14510_e9008_d_n6, assign14510_e9008_d_n7, assign14510_e9008_d_n8, assign14510_e9008_d_n9, assign14510_e9008_d_n10, assign14510_e9008_d_n11, assign14510_e9008_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14510_e9002: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14510_e9004: f64 = (assign14510_e9002 * p.p63);
        let assign14510_e9006: f64 = (assign14510_e9004 * 1000000.0);
        (assign14510_e9006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14510_e9008;
        locals.var_t1_dn0 = assign14510_e9008_d_n0;
        locals.var_t1_dn2 = assign14510_e9008_d_n2;
        locals.var_t1_dn4 = assign14510_e9008_d_n4;
        locals.var_t1_dn5 = assign14510_e9008_d_n5;
        locals.var_t1_dn6 = assign14510_e9008_d_n6;
        locals.var_t1_dn7 = assign14510_e9008_d_n7;
        locals.var_t1_dn8 = assign14510_e9008_d_n8;
        locals.var_t1_dn9 = assign14510_e9008_d_n9;
        locals.var_t1_dn10 = assign14510_e9008_d_n10;
        locals.var_t1_dn11 = assign14510_e9008_d_n11;
        locals.var_t1_dn14 = assign14510_e9008_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14520_e9029, assign14520_e9029_d_n0, assign14520_e9029_d_n2, assign14520_e9029_d_n4, assign14520_e9029_d_n5, assign14520_e9029_d_n6, assign14520_e9029_d_n7, assign14520_e9029_d_n8, assign14520_e9029_d_n9, assign14520_e9029_d_n10, assign14520_e9029_d_n11, assign14520_e9029_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14520_e9016: f64 = (p.p99 * p.p99);
        let assign14520_e9020: f64 = (0.0001 * 0.01);
        let assign14520_e9021: f64 = (4.0 * assign14520_e9020);
        let assign14520_e9024: f64 = (0.0001 * 0.01);
        let assign14520_e9025: f64 = (assign14520_e9021 * assign14520_e9024);
        let assign14520_e9026: f64 = (assign14520_e9016 + assign14520_e9025);
        let assign14520_e9027: f64 = (assign14520_e9026).sqrt();
        (assign14520_e9027, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14520_e9029;
        locals.var_tmf2_dn0 = assign14520_e9029_d_n0;
        locals.var_tmf2_dn2 = assign14520_e9029_d_n2;
        locals.var_tmf2_dn4 = assign14520_e9029_d_n4;
        locals.var_tmf2_dn5 = assign14520_e9029_d_n5;
        locals.var_tmf2_dn6 = assign14520_e9029_d_n6;
        locals.var_tmf2_dn7 = assign14520_e9029_d_n7;
        locals.var_tmf2_dn8 = assign14520_e9029_d_n8;
        locals.var_tmf2_dn9 = assign14520_e9029_d_n9;
        locals.var_tmf2_dn10 = assign14520_e9029_d_n10;
        locals.var_tmf2_dn11 = assign14520_e9029_d_n11;
        locals.var_tmf2_dn14 = assign14520_e9029_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14530_e9043, assign14530_e9043_d_n0, assign14530_e9043_d_n2, assign14530_e9043_d_n4, assign14530_e9043_d_n5, assign14530_e9043_d_n6, assign14530_e9043_d_n7, assign14530_e9043_d_n8, assign14530_e9043_d_n9, assign14530_e9043_d_n10, assign14530_e9043_d_n11, assign14530_e9043_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14530_e9039: f64 = (p.p99 / locals.var_tmf2);
        let assign14530_e9040: f64 = (1.0 + assign14530_e9039);
        let assign14530_e9041: f64 = (0.5 * assign14530_e9040);
        (assign14530_e9041, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14530_e9043;
        locals.var_t0_dn0 = assign14530_e9043_d_n0;
        locals.var_t0_dn2 = assign14530_e9043_d_n2;
        locals.var_t0_dn4 = assign14530_e9043_d_n4;
        locals.var_t0_dn5 = assign14530_e9043_d_n5;
        locals.var_t0_dn6 = assign14530_e9043_d_n6;
        locals.var_t0_dn7 = assign14530_e9043_d_n7;
        locals.var_t0_dn8 = assign14530_e9043_d_n8;
        locals.var_t0_dn9 = assign14530_e9043_d_n9;
        locals.var_t0_dn10 = assign14530_e9043_d_n10;
        locals.var_t0_dn11 = assign14530_e9043_d_n11;
        locals.var_t0_dn14 = assign14530_e9043_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14540_e9055, assign14540_e9055_d_n0, assign14540_e9055_d_n2, assign14540_e9055_d_n4, assign14540_e9055_d_n5, assign14540_e9055_d_n6, assign14540_e9055_d_n7, assign14540_e9055_d_n8, assign14540_e9055_d_n9, assign14540_e9055_d_n10, assign14540_e9055_d_n11, assign14540_e9055_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14540_e9052: f64 = (p.p99 + locals.var_tmf2);
        let assign14540_e9053: f64 = (0.5 * assign14540_e9052);
        (assign14540_e9053, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14540_e9055;
        locals.var_t2_dn0 = assign14540_e9055_d_n0;
        locals.var_t2_dn2 = assign14540_e9055_d_n2;
        locals.var_t2_dn4 = assign14540_e9055_d_n4;
        locals.var_t2_dn5 = assign14540_e9055_d_n5;
        locals.var_t2_dn6 = assign14540_e9055_d_n6;
        locals.var_t2_dn7 = assign14540_e9055_d_n7;
        locals.var_t2_dn8 = assign14540_e9055_d_n8;
        locals.var_t2_dn9 = assign14540_e9055_d_n9;
        locals.var_t2_dn10 = assign14540_e9055_d_n10;
        locals.var_t2_dn11 = assign14540_e9055_d_n11;
        locals.var_t2_dn14 = assign14540_e9055_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14550_e9058: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign14550_e9058;
        locals.var_guard321_rv = 0.0;

        let (assign14560_e9068, assign14560_e9068_d_n0, assign14560_e9068_d_n2, assign14560_e9068_d_n4, assign14560_e9068_d_n5, assign14560_e9068_d_n6, assign14560_e9068_d_n7, assign14560_e9068_d_n8, assign14560_e9068_d_n9, assign14560_e9068_d_n10, assign14560_e9068_d_n11, assign14560_e9068_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14560_e9068;
        locals.var_t2_dn0 = assign14560_e9068_d_n0;
        locals.var_t2_dn2 = assign14560_e9068_d_n2;
        locals.var_t2_dn4 = assign14560_e9068_d_n4;
        locals.var_t2_dn5 = assign14560_e9068_d_n5;
        locals.var_t2_dn6 = assign14560_e9068_d_n6;
        locals.var_t2_dn7 = assign14560_e9068_d_n7;
        locals.var_t2_dn8 = assign14560_e9068_d_n8;
        locals.var_t2_dn9 = assign14560_e9068_d_n9;
        locals.var_t2_dn10 = assign14560_e9068_d_n10;
        locals.var_t2_dn11 = assign14560_e9068_d_n11;
        locals.var_t2_dn14 = assign14560_e9068_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign14570_e9078, assign14570_e9078_d_n0, assign14570_e9078_d_n2, assign14570_e9078_d_n4, assign14570_e9078_d_n5, assign14570_e9078_d_n6, assign14570_e9078_d_n7, assign14570_e9078_d_n8, assign14570_e9078_d_n9, assign14570_e9078_d_n10, assign14570_e9078_d_n11, assign14570_e9078_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14570_e9078;
        locals.var_t0_dn0 = assign14570_e9078_d_n0;
        locals.var_t0_dn2 = assign14570_e9078_d_n2;
        locals.var_t0_dn4 = assign14570_e9078_d_n4;
        locals.var_t0_dn5 = assign14570_e9078_d_n5;
        locals.var_t0_dn6 = assign14570_e9078_d_n6;
        locals.var_t0_dn7 = assign14570_e9078_d_n7;
        locals.var_t0_dn8 = assign14570_e9078_d_n8;
        locals.var_t0_dn9 = assign14570_e9078_d_n9;
        locals.var_t0_dn10 = assign14570_e9078_d_n10;
        locals.var_t0_dn11 = assign14570_e9078_d_n11;
        locals.var_t0_dn14 = assign14570_e9078_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14580_e9089, assign14580_e9089_d_n0, assign14580_e9089_d_n2, assign14580_e9089_d_n4, assign14580_e9089_d_n5, assign14580_e9089_d_n6, assign14580_e9089_d_n7, assign14580_e9089_d_n8, assign14580_e9089_d_n9, assign14580_e9089_d_n10, assign14580_e9089_d_n11, assign14580_e9089_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14580_e9085: f64 = (-p.p98);
        let assign14580_e9087: f64 = (assign14580_e9085 / locals.var_t2);
        (assign14580_e9087, (-((assign14580_e9085 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign14580_e9089;
        locals.var_t8_dn0 = assign14580_e9089_d_n0;
        locals.var_t8_dn2 = assign14580_e9089_d_n2;
        locals.var_t8_dn4 = assign14580_e9089_d_n4;
        locals.var_t8_dn5 = assign14580_e9089_d_n5;
        locals.var_t8_dn6 = assign14580_e9089_d_n6;
        locals.var_t8_dn7 = assign14580_e9089_d_n7;
        locals.var_t8_dn8 = assign14580_e9089_d_n8;
        locals.var_t8_dn9 = assign14580_e9089_d_n9;
        locals.var_t8_dn10 = assign14580_e9089_d_n10;
        locals.var_t8_dn11 = assign14580_e9089_d_n11;
        locals.var_t8_dn14 = assign14580_e9089_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign14590_e9105, assign14590_e9105_d_n0, assign14590_e9105_d_n2, assign14590_e9105_d_n4, assign14590_e9105_d_n5, assign14590_e9105_d_n6, assign14590_e9105_d_n7, assign14590_e9105_d_n8, assign14590_e9105_d_n9, assign14590_e9105_d_n10, assign14590_e9105_d_n11, assign14590_e9105_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14590_e9097: f64 = (locals.var_t8 * p.p63);
        let assign14590_e9099: f64 = (assign14590_e9097 * 1000000.0);
        let assign14590_e9101: f64 = (assign14590_e9099 + 1.0);
        let assign14590_e9103: f64 = (assign14590_e9101 + p.p98);
        (assign14590_e9103, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn11 * p.p63) * 1000000.0), ((locals.var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14590_e9105;
        locals.var_t3_dn0 = assign14590_e9105_d_n0;
        locals.var_t3_dn2 = assign14590_e9105_d_n2;
        locals.var_t3_dn4 = assign14590_e9105_d_n4;
        locals.var_t3_dn5 = assign14590_e9105_d_n5;
        locals.var_t3_dn6 = assign14590_e9105_d_n6;
        locals.var_t3_dn7 = assign14590_e9105_d_n7;
        locals.var_t3_dn8 = assign14590_e9105_d_n8;
        locals.var_t3_dn9 = assign14590_e9105_d_n9;
        locals.var_t3_dn10 = assign14590_e9105_d_n10;
        locals.var_t3_dn11 = assign14590_e9105_d_n11;
        locals.var_t3_dn14 = assign14590_e9105_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14600_e9119, assign14600_e9119_d_n0, assign14600_e9119_d_n2, assign14600_e9119_d_n4, assign14600_e9119_d_n5, assign14600_e9119_d_n6, assign14600_e9119_d_n7, assign14600_e9119_d_n8, assign14600_e9119_d_n9, assign14600_e9119_d_n10, assign14600_e9119_d_n11, assign14600_e9119_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14600_e9113: f64 = (locals.var_t3 * locals.var_t4);
        let assign14600_e9115: f64 = (assign14600_e9113 - locals.var_t4);
        let assign14600_e9117: f64 = (assign14600_e9115 - 0.01);
        (assign14600_e9117, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14600_e9119;
        locals.var_tmf1_dn0 = assign14600_e9119_d_n0;
        locals.var_tmf1_dn2 = assign14600_e9119_d_n2;
        locals.var_tmf1_dn4 = assign14600_e9119_d_n4;
        locals.var_tmf1_dn5 = assign14600_e9119_d_n5;
        locals.var_tmf1_dn6 = assign14600_e9119_d_n6;
        locals.var_tmf1_dn7 = assign14600_e9119_d_n7;
        locals.var_tmf1_dn8 = assign14600_e9119_d_n8;
        locals.var_tmf1_dn9 = assign14600_e9119_d_n9;
        locals.var_tmf1_dn10 = assign14600_e9119_d_n10;
        locals.var_tmf1_dn11 = assign14600_e9119_d_n11;
        locals.var_tmf1_dn14 = assign14600_e9119_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14610_e9131, assign14610_e9131_d_n0, assign14610_e9131_d_n2, assign14610_e9131_d_n4, assign14610_e9131_d_n5, assign14610_e9131_d_n6, assign14610_e9131_d_n7, assign14610_e9131_d_n8, assign14610_e9131_d_n9, assign14610_e9131_d_n10, assign14610_e9131_d_n11, assign14610_e9131_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14610_e9127: f64 = (4.0 * locals.var_t4);
        let assign14610_e9129: f64 = (assign14610_e9127 * 0.01);
        (assign14610_e9129, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14610_e9131;
        locals.var_tmf2_dn0 = assign14610_e9131_d_n0;
        locals.var_tmf2_dn2 = assign14610_e9131_d_n2;
        locals.var_tmf2_dn4 = assign14610_e9131_d_n4;
        locals.var_tmf2_dn5 = assign14610_e9131_d_n5;
        locals.var_tmf2_dn6 = assign14610_e9131_d_n6;
        locals.var_tmf2_dn7 = assign14610_e9131_d_n7;
        locals.var_tmf2_dn8 = assign14610_e9131_d_n8;
        locals.var_tmf2_dn9 = assign14610_e9131_d_n9;
        locals.var_tmf2_dn10 = assign14610_e9131_d_n10;
        locals.var_tmf2_dn11 = assign14610_e9131_d_n11;
        locals.var_tmf2_dn14 = assign14610_e9131_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14620_e9145, assign14620_e9145_d_n0, assign14620_e9145_d_n2, assign14620_e9145_d_n4, assign14620_e9145_d_n5, assign14620_e9145_d_n6, assign14620_e9145_d_n7, assign14620_e9145_d_n8, assign14620_e9145_d_n9, assign14620_e9145_d_n10, assign14620_e9145_d_n11, assign14620_e9145_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14620_e9143, assign14620_e9143_d_n0, assign14620_e9143_d_n2, assign14620_e9143_d_n4, assign14620_e9143_d_n5, assign14620_e9143_d_n6, assign14620_e9143_d_n7, assign14620_e9143_d_n8, assign14620_e9143_d_n9, assign14620_e9143_d_n10, assign14620_e9143_d_n11, assign14620_e9143_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14620_e9142: f64 = (-locals.var_tmf2);
                (assign14620_e9142, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14620_e9143, assign14620_e9143_d_n0, assign14620_e9143_d_n2, assign14620_e9143_d_n4, assign14620_e9143_d_n5, assign14620_e9143_d_n6, assign14620_e9143_d_n7, assign14620_e9143_d_n8, assign14620_e9143_d_n9, assign14620_e9143_d_n10, assign14620_e9143_d_n11, assign14620_e9143_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14620_e9145;
        locals.var_tmf2_dn0 = assign14620_e9145_d_n0;
        locals.var_tmf2_dn2 = assign14620_e9145_d_n2;
        locals.var_tmf2_dn4 = assign14620_e9145_d_n4;
        locals.var_tmf2_dn5 = assign14620_e9145_d_n5;
        locals.var_tmf2_dn6 = assign14620_e9145_d_n6;
        locals.var_tmf2_dn7 = assign14620_e9145_d_n7;
        locals.var_tmf2_dn8 = assign14620_e9145_d_n8;
        locals.var_tmf2_dn9 = assign14620_e9145_d_n9;
        locals.var_tmf2_dn10 = assign14620_e9145_d_n10;
        locals.var_tmf2_dn11 = assign14620_e9145_d_n11;
        locals.var_tmf2_dn14 = assign14620_e9145_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14630_e9158, assign14630_e9158_d_n0, assign14630_e9158_d_n2, assign14630_e9158_d_n4, assign14630_e9158_d_n5, assign14630_e9158_d_n6, assign14630_e9158_d_n7, assign14630_e9158_d_n8, assign14630_e9158_d_n9, assign14630_e9158_d_n10, assign14630_e9158_d_n11, assign14630_e9158_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14630_e9153: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14630_e9155: f64 = (assign14630_e9153 + locals.var_tmf2);
        let assign14630_e9156: f64 = (assign14630_e9155).sqrt();
        (assign14630_e9156, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14630_e9156)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14630_e9158;
        locals.var_tmf2_dn0 = assign14630_e9158_d_n0;
        locals.var_tmf2_dn2 = assign14630_e9158_d_n2;
        locals.var_tmf2_dn4 = assign14630_e9158_d_n4;
        locals.var_tmf2_dn5 = assign14630_e9158_d_n5;
        locals.var_tmf2_dn6 = assign14630_e9158_d_n6;
        locals.var_tmf2_dn7 = assign14630_e9158_d_n7;
        locals.var_tmf2_dn8 = assign14630_e9158_d_n8;
        locals.var_tmf2_dn9 = assign14630_e9158_d_n9;
        locals.var_tmf2_dn10 = assign14630_e9158_d_n10;
        locals.var_tmf2_dn11 = assign14630_e9158_d_n11;
        locals.var_tmf2_dn14 = assign14630_e9158_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14640_e9172, assign14640_e9172_d_n0, assign14640_e9172_d_n2, assign14640_e9172_d_n4, assign14640_e9172_d_n5, assign14640_e9172_d_n6, assign14640_e9172_d_n7, assign14640_e9172_d_n8, assign14640_e9172_d_n9, assign14640_e9172_d_n10, assign14640_e9172_d_n11, assign14640_e9172_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14640_e9168: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14640_e9169: f64 = (1.0 + assign14640_e9168);
        let assign14640_e9170: f64 = (0.5 * assign14640_e9169);
        (assign14640_e9170, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14640_e9172;
        locals.var_t6_dn0 = assign14640_e9172_d_n0;
        locals.var_t6_dn2 = assign14640_e9172_d_n2;
        locals.var_t6_dn4 = assign14640_e9172_d_n4;
        locals.var_t6_dn5 = assign14640_e9172_d_n5;
        locals.var_t6_dn6 = assign14640_e9172_d_n6;
        locals.var_t6_dn7 = assign14640_e9172_d_n7;
        locals.var_t6_dn8 = assign14640_e9172_d_n8;
        locals.var_t6_dn9 = assign14640_e9172_d_n9;
        locals.var_t6_dn10 = assign14640_e9172_d_n10;
        locals.var_t6_dn11 = assign14640_e9172_d_n11;
        locals.var_t6_dn14 = assign14640_e9172_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign14650_e9186, assign14650_e9186_d_n0, assign14650_e9186_d_n2, assign14650_e9186_d_n4, assign14650_e9186_d_n5, assign14650_e9186_d_n6, assign14650_e9186_d_n7, assign14650_e9186_d_n8, assign14650_e9186_d_n9, assign14650_e9186_d_n10, assign14650_e9186_d_n11, assign14650_e9186_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14650_e9182: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14650_e9183: f64 = (0.5 * assign14650_e9182);
        let assign14650_e9184: f64 = (locals.var_t4 + assign14650_e9183);
        (assign14650_e9184, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign14650_e9186;
        locals.var_t5_dn0 = assign14650_e9186_d_n0;
        locals.var_t5_dn2 = assign14650_e9186_d_n2;
        locals.var_t5_dn4 = assign14650_e9186_d_n4;
        locals.var_t5_dn5 = assign14650_e9186_d_n5;
        locals.var_t5_dn6 = assign14650_e9186_d_n6;
        locals.var_t5_dn7 = assign14650_e9186_d_n7;
        locals.var_t5_dn8 = assign14650_e9186_d_n8;
        locals.var_t5_dn9 = assign14650_e9186_d_n9;
        locals.var_t5_dn10 = assign14650_e9186_d_n10;
        locals.var_t5_dn11 = assign14650_e9186_d_n11;
        locals.var_t5_dn14 = assign14650_e9186_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign14660_e9202, assign14660_e9202_d_n0, assign14660_e9202_d_n2, assign14660_e9202_d_n4, assign14660_e9202_d_n5, assign14660_e9202_d_n6, assign14660_e9202_d_n7, assign14660_e9202_d_n8, assign14660_e9202_d_n9, assign14660_e9202_d_n10, assign14660_e9202_d_n11, assign14660_e9202_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14660_e9195: f64 = (p.p98 + 1.0);
        let assign14660_e9196: f64 = (locals.var_t4 * assign14660_e9195);
        let assign14660_e9198: f64 = (assign14660_e9196 - locals.var_t5);
        let assign14660_e9200: f64 = (assign14660_e9198 - 5e-5);
        (assign14660_e9200, ((locals.var_t4_dn0 * assign14660_e9195) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign14660_e9195) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign14660_e9195) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign14660_e9195) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign14660_e9195) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign14660_e9195) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign14660_e9195) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign14660_e9195) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign14660_e9195) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign14660_e9195) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign14660_e9195) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14660_e9202;
        locals.var_tmf1_dn0 = assign14660_e9202_d_n0;
        locals.var_tmf1_dn2 = assign14660_e9202_d_n2;
        locals.var_tmf1_dn4 = assign14660_e9202_d_n4;
        locals.var_tmf1_dn5 = assign14660_e9202_d_n5;
        locals.var_tmf1_dn6 = assign14660_e9202_d_n6;
        locals.var_tmf1_dn7 = assign14660_e9202_d_n7;
        locals.var_tmf1_dn8 = assign14660_e9202_d_n8;
        locals.var_tmf1_dn9 = assign14660_e9202_d_n9;
        locals.var_tmf1_dn10 = assign14660_e9202_d_n10;
        locals.var_tmf1_dn11 = assign14660_e9202_d_n11;
        locals.var_tmf1_dn14 = assign14660_e9202_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14670_e9218, assign14670_e9218_d_n0, assign14670_e9218_d_n2, assign14670_e9218_d_n4, assign14670_e9218_d_n5, assign14670_e9218_d_n6, assign14670_e9218_d_n7, assign14670_e9218_d_n8, assign14670_e9218_d_n9, assign14670_e9218_d_n10, assign14670_e9218_d_n11, assign14670_e9218_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14670_e9212: f64 = (p.p98 + 1.0);
        let assign14670_e9213: f64 = (locals.var_t4 * assign14670_e9212);
        let assign14670_e9214: f64 = (4.0 * assign14670_e9213);
        let assign14670_e9216: f64 = (assign14670_e9214 * 5e-5);
        (assign14670_e9216, ((4.0 * (locals.var_t4_dn0 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign14670_e9212)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14670_e9218;
        locals.var_tmf2_dn0 = assign14670_e9218_d_n0;
        locals.var_tmf2_dn2 = assign14670_e9218_d_n2;
        locals.var_tmf2_dn4 = assign14670_e9218_d_n4;
        locals.var_tmf2_dn5 = assign14670_e9218_d_n5;
        locals.var_tmf2_dn6 = assign14670_e9218_d_n6;
        locals.var_tmf2_dn7 = assign14670_e9218_d_n7;
        locals.var_tmf2_dn8 = assign14670_e9218_d_n8;
        locals.var_tmf2_dn9 = assign14670_e9218_d_n9;
        locals.var_tmf2_dn10 = assign14670_e9218_d_n10;
        locals.var_tmf2_dn11 = assign14670_e9218_d_n11;
        locals.var_tmf2_dn14 = assign14670_e9218_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14680_e9232, assign14680_e9232_d_n0, assign14680_e9232_d_n2, assign14680_e9232_d_n4, assign14680_e9232_d_n5, assign14680_e9232_d_n6, assign14680_e9232_d_n7, assign14680_e9232_d_n8, assign14680_e9232_d_n9, assign14680_e9232_d_n10, assign14680_e9232_d_n11, assign14680_e9232_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14680_e9230, assign14680_e9230_d_n0, assign14680_e9230_d_n2, assign14680_e9230_d_n4, assign14680_e9230_d_n5, assign14680_e9230_d_n6, assign14680_e9230_d_n7, assign14680_e9230_d_n8, assign14680_e9230_d_n9, assign14680_e9230_d_n10, assign14680_e9230_d_n11, assign14680_e9230_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14680_e9229: f64 = (-locals.var_tmf2);
                (assign14680_e9229, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14680_e9230, assign14680_e9230_d_n0, assign14680_e9230_d_n2, assign14680_e9230_d_n4, assign14680_e9230_d_n5, assign14680_e9230_d_n6, assign14680_e9230_d_n7, assign14680_e9230_d_n8, assign14680_e9230_d_n9, assign14680_e9230_d_n10, assign14680_e9230_d_n11, assign14680_e9230_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14680_e9232;
        locals.var_tmf2_dn0 = assign14680_e9232_d_n0;
        locals.var_tmf2_dn2 = assign14680_e9232_d_n2;
        locals.var_tmf2_dn4 = assign14680_e9232_d_n4;
        locals.var_tmf2_dn5 = assign14680_e9232_d_n5;
        locals.var_tmf2_dn6 = assign14680_e9232_d_n6;
        locals.var_tmf2_dn7 = assign14680_e9232_d_n7;
        locals.var_tmf2_dn8 = assign14680_e9232_d_n8;
        locals.var_tmf2_dn9 = assign14680_e9232_d_n9;
        locals.var_tmf2_dn10 = assign14680_e9232_d_n10;
        locals.var_tmf2_dn11 = assign14680_e9232_d_n11;
        locals.var_tmf2_dn14 = assign14680_e9232_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14690_e9245, assign14690_e9245_d_n0, assign14690_e9245_d_n2, assign14690_e9245_d_n4, assign14690_e9245_d_n5, assign14690_e9245_d_n6, assign14690_e9245_d_n7, assign14690_e9245_d_n8, assign14690_e9245_d_n9, assign14690_e9245_d_n10, assign14690_e9245_d_n11, assign14690_e9245_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14690_e9240: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14690_e9242: f64 = (assign14690_e9240 + locals.var_tmf2);
        let assign14690_e9243: f64 = (assign14690_e9242).sqrt();
        (assign14690_e9243, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14690_e9243)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14690_e9245;
        locals.var_tmf2_dn0 = assign14690_e9245_d_n0;
        locals.var_tmf2_dn2 = assign14690_e9245_d_n2;
        locals.var_tmf2_dn4 = assign14690_e9245_d_n4;
        locals.var_tmf2_dn5 = assign14690_e9245_d_n5;
        locals.var_tmf2_dn6 = assign14690_e9245_d_n6;
        locals.var_tmf2_dn7 = assign14690_e9245_d_n7;
        locals.var_tmf2_dn8 = assign14690_e9245_d_n8;
        locals.var_tmf2_dn9 = assign14690_e9245_d_n9;
        locals.var_tmf2_dn10 = assign14690_e9245_d_n10;
        locals.var_tmf2_dn11 = assign14690_e9245_d_n11;
        locals.var_tmf2_dn14 = assign14690_e9245_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14700_e9259, assign14700_e9259_d_n0, assign14700_e9259_d_n2, assign14700_e9259_d_n4, assign14700_e9259_d_n5, assign14700_e9259_d_n6, assign14700_e9259_d_n7, assign14700_e9259_d_n8, assign14700_e9259_d_n9, assign14700_e9259_d_n10, assign14700_e9259_d_n11, assign14700_e9259_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14700_e9255: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14700_e9256: f64 = (1.0 + assign14700_e9255);
        let assign14700_e9257: f64 = (0.5 * assign14700_e9256);
        (assign14700_e9257, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14700_e9259;
        locals.var_t6_dn0 = assign14700_e9259_d_n0;
        locals.var_t6_dn2 = assign14700_e9259_d_n2;
        locals.var_t6_dn4 = assign14700_e9259_d_n4;
        locals.var_t6_dn5 = assign14700_e9259_d_n5;
        locals.var_t6_dn6 = assign14700_e9259_d_n6;
        locals.var_t6_dn7 = assign14700_e9259_d_n7;
        locals.var_t6_dn8 = assign14700_e9259_d_n8;
        locals.var_t6_dn9 = assign14700_e9259_d_n9;
        locals.var_t6_dn10 = assign14700_e9259_d_n10;
        locals.var_t6_dn11 = assign14700_e9259_d_n11;
        locals.var_t6_dn14 = assign14700_e9259_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign14710_e9277, assign14710_e9277_d_n0, assign14710_e9277_d_n2, assign14710_e9277_d_n4, assign14710_e9277_d_n5, assign14710_e9277_d_n6, assign14710_e9277_d_n7, assign14710_e9277_d_n8, assign14710_e9277_d_n9, assign14710_e9277_d_n10, assign14710_e9277_d_n11, assign14710_e9277_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14710_e9268: f64 = (p.p98 + 1.0);
        let assign14710_e9269: f64 = (locals.var_t4 * assign14710_e9268);
        let assign14710_e9273: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14710_e9274: f64 = (0.5 * assign14710_e9273);
        let assign14710_e9275: f64 = (assign14710_e9269 - assign14710_e9274);
        (assign14710_e9275, ((locals.var_t4_dn0 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign14710_e9277;
        locals.var_t7_dn0 = assign14710_e9277_d_n0;
        locals.var_t7_dn2 = assign14710_e9277_d_n2;
        locals.var_t7_dn4 = assign14710_e9277_d_n4;
        locals.var_t7_dn5 = assign14710_e9277_d_n5;
        locals.var_t7_dn6 = assign14710_e9277_d_n6;
        locals.var_t7_dn7 = assign14710_e9277_d_n7;
        locals.var_t7_dn8 = assign14710_e9277_d_n8;
        locals.var_t7_dn9 = assign14710_e9277_d_n9;
        locals.var_t7_dn10 = assign14710_e9277_d_n10;
        locals.var_t7_dn11 = assign14710_e9277_d_n11;
        locals.var_t7_dn14 = assign14710_e9277_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign14720_e9293, assign14720_e9293_d_n0, assign14720_e9293_d_n2, assign14720_e9293_d_n4, assign14720_e9293_d_n5, assign14720_e9293_d_n6, assign14720_e9293_d_n7, assign14720_e9293_d_n8, assign14720_e9293_d_n9, assign14720_e9293_d_n10, assign14720_e9293_d_n11, assign14720_e9293_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14720_e9286: f64 = (locals.var_t1 * locals.var_t4);
        let assign14720_e9287: f64 = (locals.var_t7 + assign14720_e9286);
        let assign14720_e9289: f64 = assign14720_e9287;
        let assign14720_e9291: f64 = (assign14720_e9289 - 5e-5);
        (assign14720_e9291, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14720_e9293;
        locals.var_tmf1_dn0 = assign14720_e9293_d_n0;
        locals.var_tmf1_dn2 = assign14720_e9293_d_n2;
        locals.var_tmf1_dn4 = assign14720_e9293_d_n4;
        locals.var_tmf1_dn5 = assign14720_e9293_d_n5;
        locals.var_tmf1_dn6 = assign14720_e9293_d_n6;
        locals.var_tmf1_dn7 = assign14720_e9293_d_n7;
        locals.var_tmf1_dn8 = assign14720_e9293_d_n8;
        locals.var_tmf1_dn9 = assign14720_e9293_d_n9;
        locals.var_tmf1_dn10 = assign14720_e9293_d_n10;
        locals.var_tmf1_dn11 = assign14720_e9293_d_n11;
        locals.var_tmf1_dn14 = assign14720_e9293_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14730_e9305, assign14730_e9305_d_n0, assign14730_e9305_d_n2, assign14730_e9305_d_n4, assign14730_e9305_d_n5, assign14730_e9305_d_n6, assign14730_e9305_d_n7, assign14730_e9305_d_n8, assign14730_e9305_d_n9, assign14730_e9305_d_n10, assign14730_e9305_d_n11, assign14730_e9305_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14730_e9305;
        locals.var_tmf2_dn0 = assign14730_e9305_d_n0;
        locals.var_tmf2_dn2 = assign14730_e9305_d_n2;
        locals.var_tmf2_dn4 = assign14730_e9305_d_n4;
        locals.var_tmf2_dn5 = assign14730_e9305_d_n5;
        locals.var_tmf2_dn6 = assign14730_e9305_d_n6;
        locals.var_tmf2_dn7 = assign14730_e9305_d_n7;
        locals.var_tmf2_dn8 = assign14730_e9305_d_n8;
        locals.var_tmf2_dn9 = assign14730_e9305_d_n9;
        locals.var_tmf2_dn10 = assign14730_e9305_d_n10;
        locals.var_tmf2_dn11 = assign14730_e9305_d_n11;
        locals.var_tmf2_dn14 = assign14730_e9305_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14740_e9319, assign14740_e9319_d_n0, assign14740_e9319_d_n2, assign14740_e9319_d_n4, assign14740_e9319_d_n5, assign14740_e9319_d_n6, assign14740_e9319_d_n7, assign14740_e9319_d_n8, assign14740_e9319_d_n9, assign14740_e9319_d_n10, assign14740_e9319_d_n11, assign14740_e9319_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14740_e9317, assign14740_e9317_d_n0, assign14740_e9317_d_n2, assign14740_e9317_d_n4, assign14740_e9317_d_n5, assign14740_e9317_d_n6, assign14740_e9317_d_n7, assign14740_e9317_d_n8, assign14740_e9317_d_n9, assign14740_e9317_d_n10, assign14740_e9317_d_n11, assign14740_e9317_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14740_e9316: f64 = (-locals.var_tmf2);
                (assign14740_e9316, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14740_e9317, assign14740_e9317_d_n0, assign14740_e9317_d_n2, assign14740_e9317_d_n4, assign14740_e9317_d_n5, assign14740_e9317_d_n6, assign14740_e9317_d_n7, assign14740_e9317_d_n8, assign14740_e9317_d_n9, assign14740_e9317_d_n10, assign14740_e9317_d_n11, assign14740_e9317_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14740_e9319;
        locals.var_tmf2_dn0 = assign14740_e9319_d_n0;
        locals.var_tmf2_dn2 = assign14740_e9319_d_n2;
        locals.var_tmf2_dn4 = assign14740_e9319_d_n4;
        locals.var_tmf2_dn5 = assign14740_e9319_d_n5;
        locals.var_tmf2_dn6 = assign14740_e9319_d_n6;
        locals.var_tmf2_dn7 = assign14740_e9319_d_n7;
        locals.var_tmf2_dn8 = assign14740_e9319_d_n8;
        locals.var_tmf2_dn9 = assign14740_e9319_d_n9;
        locals.var_tmf2_dn10 = assign14740_e9319_d_n10;
        locals.var_tmf2_dn11 = assign14740_e9319_d_n11;
        locals.var_tmf2_dn14 = assign14740_e9319_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14750_e9332, assign14750_e9332_d_n0, assign14750_e9332_d_n2, assign14750_e9332_d_n4, assign14750_e9332_d_n5, assign14750_e9332_d_n6, assign14750_e9332_d_n7, assign14750_e9332_d_n8, assign14750_e9332_d_n9, assign14750_e9332_d_n10, assign14750_e9332_d_n11, assign14750_e9332_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14750_e9327: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14750_e9329: f64 = (assign14750_e9327 + locals.var_tmf2);
        let assign14750_e9330: f64 = (assign14750_e9329).sqrt();
        (assign14750_e9330, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14750_e9330)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14750_e9332;
        locals.var_tmf2_dn0 = assign14750_e9332_d_n0;
        locals.var_tmf2_dn2 = assign14750_e9332_d_n2;
        locals.var_tmf2_dn4 = assign14750_e9332_d_n4;
        locals.var_tmf2_dn5 = assign14750_e9332_d_n5;
        locals.var_tmf2_dn6 = assign14750_e9332_d_n6;
        locals.var_tmf2_dn7 = assign14750_e9332_d_n7;
        locals.var_tmf2_dn8 = assign14750_e9332_d_n8;
        locals.var_tmf2_dn9 = assign14750_e9332_d_n9;
        locals.var_tmf2_dn10 = assign14750_e9332_d_n10;
        locals.var_tmf2_dn11 = assign14750_e9332_d_n11;
        locals.var_tmf2_dn14 = assign14750_e9332_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14760_e9346, assign14760_e9346_d_n0, assign14760_e9346_d_n2, assign14760_e9346_d_n4, assign14760_e9346_d_n5, assign14760_e9346_d_n6, assign14760_e9346_d_n7, assign14760_e9346_d_n8, assign14760_e9346_d_n9, assign14760_e9346_d_n10, assign14760_e9346_d_n11, assign14760_e9346_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14760_e9342: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14760_e9343: f64 = (1.0 + assign14760_e9342);
        let assign14760_e9344: f64 = (0.5 * assign14760_e9343);
        (assign14760_e9344, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14760_e9346;
        locals.var_t6_dn0 = assign14760_e9346_d_n0;
        locals.var_t6_dn2 = assign14760_e9346_d_n2;
        locals.var_t6_dn4 = assign14760_e9346_d_n4;
        locals.var_t6_dn5 = assign14760_e9346_d_n5;
        locals.var_t6_dn6 = assign14760_e9346_d_n6;
        locals.var_t6_dn7 = assign14760_e9346_d_n7;
        locals.var_t6_dn8 = assign14760_e9346_d_n8;
        locals.var_t6_dn9 = assign14760_e9346_d_n9;
        locals.var_t6_dn10 = assign14760_e9346_d_n10;
        locals.var_t6_dn11 = assign14760_e9346_d_n11;
        locals.var_t6_dn14 = assign14760_e9346_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign14770_e9360, assign14770_e9360_d_n0, assign14770_e9360_d_n2, assign14770_e9360_d_n4, assign14770_e9360_d_n5, assign14770_e9360_d_n6, assign14770_e9360_d_n7, assign14770_e9360_d_n8, assign14770_e9360_d_n9, assign14770_e9360_d_n10, assign14770_e9360_d_n11, assign14770_e9360_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14770_e9356: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14770_e9357: f64 = (0.5 * assign14770_e9356);
        let assign14770_e9358: f64 = assign14770_e9357;
        (assign14770_e9358, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14770_e9360;
        locals.var_t2_dn0 = assign14770_e9360_d_n0;
        locals.var_t2_dn2 = assign14770_e9360_d_n2;
        locals.var_t2_dn4 = assign14770_e9360_d_n4;
        locals.var_t2_dn5 = assign14770_e9360_d_n5;
        locals.var_t2_dn6 = assign14770_e9360_d_n6;
        locals.var_t2_dn7 = assign14770_e9360_d_n7;
        locals.var_t2_dn8 = assign14770_e9360_d_n8;
        locals.var_t2_dn9 = assign14770_e9360_d_n9;
        locals.var_t2_dn10 = assign14770_e9360_d_n10;
        locals.var_t2_dn11 = assign14770_e9360_d_n11;
        locals.var_t2_dn14 = assign14770_e9360_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14780_e9367: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard322 = assign14780_e9367;
        locals.var_guard322_rv = 0.0;

        let (assign14790_e9387, assign14790_e9387_d_n0, assign14790_e9387_d_n2, assign14790_e9387_d_n4, assign14790_e9387_d_n5, assign14790_e9387_d_n6, assign14790_e9387_d_n7, assign14790_e9387_d_n8, assign14790_e9387_d_n9, assign14790_e9387_d_n10, assign14790_e9387_d_n11, assign14790_e9387_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14790_e9378: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign14790_e9379: f64 = (locals.var_uc_rdvd + assign14790_e9378);
        let assign14790_e9382: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign14790_e9383: f64 = (assign14790_e9379 + assign14790_e9382);
        let assign14790_e9385: f64 = (assign14790_e9383 * locals.var_t2);
        (assign14790_e9385, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14790_e9387;
        locals.var_rdvde_dn0 = assign14790_e9387_d_n0;
        locals.var_rdvde_dn2 = assign14790_e9387_d_n2;
        locals.var_rdvde_dn4 = assign14790_e9387_d_n4;
        locals.var_rdvde_dn5 = assign14790_e9387_d_n5;
        locals.var_rdvde_dn6 = assign14790_e9387_d_n6;
        locals.var_rdvde_dn7 = assign14790_e9387_d_n7;
        locals.var_rdvde_dn8 = assign14790_e9387_d_n8;
        locals.var_rdvde_dn9 = assign14790_e9387_d_n9;
        locals.var_rdvde_dn10 = assign14790_e9387_d_n10;
        locals.var_rdvde_dn11 = assign14790_e9387_d_n11;
        locals.var_rdvde_dn14 = assign14790_e9387_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign14800_e9405, assign14800_e9405_d_n0, assign14800_e9405_d_n2, assign14800_e9405_d_n4, assign14800_e9405_d_n5, assign14800_e9405_d_n6, assign14800_e9405_d_n7, assign14800_e9405_d_n8, assign14800_e9405_d_n9, assign14800_e9405_d_n10, assign14800_e9405_d_n11, assign14800_e9405_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14800_e9398: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14800_e9399: f64 = (locals.var_rdvde - assign14800_e9398);
        let assign14800_e9402: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14800_e9403: f64 = (assign14800_e9399 - assign14800_e9402);
        (assign14800_e9403, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14800_e9405;
        locals.var_tmf1_dn0 = assign14800_e9405_d_n0;
        locals.var_tmf1_dn2 = assign14800_e9405_d_n2;
        locals.var_tmf1_dn4 = assign14800_e9405_d_n4;
        locals.var_tmf1_dn5 = assign14800_e9405_d_n5;
        locals.var_tmf1_dn6 = assign14800_e9405_d_n6;
        locals.var_tmf1_dn7 = assign14800_e9405_d_n7;
        locals.var_tmf1_dn8 = assign14800_e9405_d_n8;
        locals.var_tmf1_dn9 = assign14800_e9405_d_n9;
        locals.var_tmf1_dn10 = assign14800_e9405_d_n10;
        locals.var_tmf1_dn11 = assign14800_e9405_d_n11;
        locals.var_tmf1_dn14 = assign14800_e9405_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14810_e9423, assign14810_e9423_d_n0, assign14810_e9423_d_n2, assign14810_e9423_d_n4, assign14810_e9423_d_n5, assign14810_e9423_d_n6, assign14810_e9423_d_n7, assign14810_e9423_d_n8, assign14810_e9423_d_n9, assign14810_e9423_d_n10, assign14810_e9423_d_n11, assign14810_e9423_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14810_e9416: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14810_e9417: f64 = (4.0 * assign14810_e9416);
        let assign14810_e9420: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14810_e9421: f64 = (assign14810_e9417 * assign14810_e9420);
        (assign14810_e9421, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14810_e9423;
        locals.var_tmf2_dn0 = assign14810_e9423_d_n0;
        locals.var_tmf2_dn2 = assign14810_e9423_d_n2;
        locals.var_tmf2_dn4 = assign14810_e9423_d_n4;
        locals.var_tmf2_dn5 = assign14810_e9423_d_n5;
        locals.var_tmf2_dn6 = assign14810_e9423_d_n6;
        locals.var_tmf2_dn7 = assign14810_e9423_d_n7;
        locals.var_tmf2_dn8 = assign14810_e9423_d_n8;
        locals.var_tmf2_dn9 = assign14810_e9423_d_n9;
        locals.var_tmf2_dn10 = assign14810_e9423_d_n10;
        locals.var_tmf2_dn11 = assign14810_e9423_d_n11;
        locals.var_tmf2_dn14 = assign14810_e9423_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14820_e9439, assign14820_e9439_d_n0, assign14820_e9439_d_n2, assign14820_e9439_d_n4, assign14820_e9439_d_n5, assign14820_e9439_d_n6, assign14820_e9439_d_n7, assign14820_e9439_d_n8, assign14820_e9439_d_n9, assign14820_e9439_d_n10, assign14820_e9439_d_n11, assign14820_e9439_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let (assign14820_e9437, assign14820_e9437_d_n0, assign14820_e9437_d_n2, assign14820_e9437_d_n4, assign14820_e9437_d_n5, assign14820_e9437_d_n6, assign14820_e9437_d_n7, assign14820_e9437_d_n8, assign14820_e9437_d_n9, assign14820_e9437_d_n10, assign14820_e9437_d_n11, assign14820_e9437_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14820_e9436: f64 = (-locals.var_tmf2);
                (assign14820_e9436, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14820_e9437, assign14820_e9437_d_n0, assign14820_e9437_d_n2, assign14820_e9437_d_n4, assign14820_e9437_d_n5, assign14820_e9437_d_n6, assign14820_e9437_d_n7, assign14820_e9437_d_n8, assign14820_e9437_d_n9, assign14820_e9437_d_n10, assign14820_e9437_d_n11, assign14820_e9437_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14820_e9439;
        locals.var_tmf2_dn0 = assign14820_e9439_d_n0;
        locals.var_tmf2_dn2 = assign14820_e9439_d_n2;
        locals.var_tmf2_dn4 = assign14820_e9439_d_n4;
        locals.var_tmf2_dn5 = assign14820_e9439_d_n5;
        locals.var_tmf2_dn6 = assign14820_e9439_d_n6;
        locals.var_tmf2_dn7 = assign14820_e9439_d_n7;
        locals.var_tmf2_dn8 = assign14820_e9439_d_n8;
        locals.var_tmf2_dn9 = assign14820_e9439_d_n9;
        locals.var_tmf2_dn10 = assign14820_e9439_d_n10;
        locals.var_tmf2_dn11 = assign14820_e9439_d_n11;
        locals.var_tmf2_dn14 = assign14820_e9439_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14830_e9454, assign14830_e9454_d_n0, assign14830_e9454_d_n2, assign14830_e9454_d_n4, assign14830_e9454_d_n5, assign14830_e9454_d_n6, assign14830_e9454_d_n7, assign14830_e9454_d_n8, assign14830_e9454_d_n9, assign14830_e9454_d_n10, assign14830_e9454_d_n11, assign14830_e9454_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14830_e9449: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14830_e9451: f64 = (assign14830_e9449 + locals.var_tmf2);
        let assign14830_e9452: f64 = (assign14830_e9451).sqrt();
        (assign14830_e9452, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14830_e9452)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14830_e9454;
        locals.var_tmf2_dn0 = assign14830_e9454_d_n0;
        locals.var_tmf2_dn2 = assign14830_e9454_d_n2;
        locals.var_tmf2_dn4 = assign14830_e9454_d_n4;
        locals.var_tmf2_dn5 = assign14830_e9454_d_n5;
        locals.var_tmf2_dn6 = assign14830_e9454_d_n6;
        locals.var_tmf2_dn7 = assign14830_e9454_d_n7;
        locals.var_tmf2_dn8 = assign14830_e9454_d_n8;
        locals.var_tmf2_dn9 = assign14830_e9454_d_n9;
        locals.var_tmf2_dn10 = assign14830_e9454_d_n10;
        locals.var_tmf2_dn11 = assign14830_e9454_d_n11;
        locals.var_tmf2_dn14 = assign14830_e9454_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14840_e9470, assign14840_e9470_d_n0, assign14840_e9470_d_n2, assign14840_e9470_d_n4, assign14840_e9470_d_n5, assign14840_e9470_d_n6, assign14840_e9470_d_n7, assign14840_e9470_d_n8, assign14840_e9470_d_n9, assign14840_e9470_d_n10, assign14840_e9470_d_n11, assign14840_e9470_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14840_e9466: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14840_e9467: f64 = (1.0 + assign14840_e9466);
        let assign14840_e9468: f64 = (0.5 * assign14840_e9467);
        (assign14840_e9468, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14840_e9470;
        locals.var_t0_dn0 = assign14840_e9470_d_n0;
        locals.var_t0_dn2 = assign14840_e9470_d_n2;
        locals.var_t0_dn4 = assign14840_e9470_d_n4;
        locals.var_t0_dn5 = assign14840_e9470_d_n5;
        locals.var_t0_dn6 = assign14840_e9470_d_n6;
        locals.var_t0_dn7 = assign14840_e9470_d_n7;
        locals.var_t0_dn8 = assign14840_e9470_d_n8;
        locals.var_t0_dn9 = assign14840_e9470_d_n9;
        locals.var_t0_dn10 = assign14840_e9470_d_n10;
        locals.var_t0_dn11 = assign14840_e9470_d_n11;
        locals.var_t0_dn14 = assign14840_e9470_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14850_e9488, assign14850_e9488_d_n0, assign14850_e9488_d_n2, assign14850_e9488_d_n4, assign14850_e9488_d_n5, assign14850_e9488_d_n6, assign14850_e9488_d_n7, assign14850_e9488_d_n8, assign14850_e9488_d_n9, assign14850_e9488_d_n10, assign14850_e9488_d_n11, assign14850_e9488_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14850_e9480: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14850_e9484: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14850_e9485: f64 = (0.5 * assign14850_e9484);
        let assign14850_e9486: f64 = (assign14850_e9480 + assign14850_e9485);
        (assign14850_e9486, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14850_e9488;
        locals.var_rdvde_dn0 = assign14850_e9488_d_n0;
        locals.var_rdvde_dn2 = assign14850_e9488_d_n2;
        locals.var_rdvde_dn4 = assign14850_e9488_d_n4;
        locals.var_rdvde_dn5 = assign14850_e9488_d_n5;
        locals.var_rdvde_dn6 = assign14850_e9488_d_n6;
        locals.var_rdvde_dn7 = assign14850_e9488_d_n7;
        locals.var_rdvde_dn8 = assign14850_e9488_d_n8;
        locals.var_rdvde_dn9 = assign14850_e9488_d_n9;
        locals.var_rdvde_dn10 = assign14850_e9488_d_n10;
        locals.var_rdvde_dn11 = assign14850_e9488_d_n11;
        locals.var_rdvde_dn14 = assign14850_e9488_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign14860_e9509, assign14860_e9509_d_n0, assign14860_e9509_d_n2, assign14860_e9509_d_n4, assign14860_e9509_d_n5, assign14860_e9509_d_n6, assign14860_e9509_d_n7, assign14860_e9509_d_n8, assign14860_e9509_d_n9, assign14860_e9509_d_n10, assign14860_e9509_d_n11, assign14860_e9509_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14860_e9500: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign14860_e9501: f64 = (locals.var_uc_rdvd + assign14860_e9500);
        let assign14860_e9504: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign14860_e9505: f64 = (assign14860_e9501 + assign14860_e9504);
        let assign14860_e9507: f64 = (assign14860_e9505 * locals.var_t2);
        (assign14860_e9507, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14860_e9509;
        locals.var_rdvde_dn0 = assign14860_e9509_d_n0;
        locals.var_rdvde_dn2 = assign14860_e9509_d_n2;
        locals.var_rdvde_dn4 = assign14860_e9509_d_n4;
        locals.var_rdvde_dn5 = assign14860_e9509_d_n5;
        locals.var_rdvde_dn6 = assign14860_e9509_d_n6;
        locals.var_rdvde_dn7 = assign14860_e9509_d_n7;
        locals.var_rdvde_dn8 = assign14860_e9509_d_n8;
        locals.var_rdvde_dn9 = assign14860_e9509_d_n9;
        locals.var_rdvde_dn10 = assign14860_e9509_d_n10;
        locals.var_rdvde_dn11 = assign14860_e9509_d_n11;
        locals.var_rdvde_dn14 = assign14860_e9509_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign14870_e9528, assign14870_e9528_d_n0, assign14870_e9528_d_n2, assign14870_e9528_d_n4, assign14870_e9528_d_n5, assign14870_e9528_d_n6, assign14870_e9528_d_n7, assign14870_e9528_d_n8, assign14870_e9528_d_n9, assign14870_e9528_d_n10, assign14870_e9528_d_n11, assign14870_e9528_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14870_e9521: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14870_e9522: f64 = (locals.var_rdvde - assign14870_e9521);
        let assign14870_e9525: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14870_e9526: f64 = (assign14870_e9522 - assign14870_e9525);
        (assign14870_e9526, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14870_e9528;
        locals.var_tmf1_dn0 = assign14870_e9528_d_n0;
        locals.var_tmf1_dn2 = assign14870_e9528_d_n2;
        locals.var_tmf1_dn4 = assign14870_e9528_d_n4;
        locals.var_tmf1_dn5 = assign14870_e9528_d_n5;
        locals.var_tmf1_dn6 = assign14870_e9528_d_n6;
        locals.var_tmf1_dn7 = assign14870_e9528_d_n7;
        locals.var_tmf1_dn8 = assign14870_e9528_d_n8;
        locals.var_tmf1_dn9 = assign14870_e9528_d_n9;
        locals.var_tmf1_dn10 = assign14870_e9528_d_n10;
        locals.var_tmf1_dn11 = assign14870_e9528_d_n11;
        locals.var_tmf1_dn14 = assign14870_e9528_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14880_e9547, assign14880_e9547_d_n0, assign14880_e9547_d_n2, assign14880_e9547_d_n4, assign14880_e9547_d_n5, assign14880_e9547_d_n6, assign14880_e9547_d_n7, assign14880_e9547_d_n8, assign14880_e9547_d_n9, assign14880_e9547_d_n10, assign14880_e9547_d_n11, assign14880_e9547_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14880_e9540: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14880_e9541: f64 = (4.0 * assign14880_e9540);
        let assign14880_e9544: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14880_e9545: f64 = (assign14880_e9541 * assign14880_e9544);
        (assign14880_e9545, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14880_e9547;
        locals.var_tmf2_dn0 = assign14880_e9547_d_n0;
        locals.var_tmf2_dn2 = assign14880_e9547_d_n2;
        locals.var_tmf2_dn4 = assign14880_e9547_d_n4;
        locals.var_tmf2_dn5 = assign14880_e9547_d_n5;
        locals.var_tmf2_dn6 = assign14880_e9547_d_n6;
        locals.var_tmf2_dn7 = assign14880_e9547_d_n7;
        locals.var_tmf2_dn8 = assign14880_e9547_d_n8;
        locals.var_tmf2_dn9 = assign14880_e9547_d_n9;
        locals.var_tmf2_dn10 = assign14880_e9547_d_n10;
        locals.var_tmf2_dn11 = assign14880_e9547_d_n11;
        locals.var_tmf2_dn14 = assign14880_e9547_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14890_e9564, assign14890_e9564_d_n0, assign14890_e9564_d_n2, assign14890_e9564_d_n4, assign14890_e9564_d_n5, assign14890_e9564_d_n6, assign14890_e9564_d_n7, assign14890_e9564_d_n8, assign14890_e9564_d_n9, assign14890_e9564_d_n10, assign14890_e9564_d_n11, assign14890_e9564_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let (assign14890_e9562, assign14890_e9562_d_n0, assign14890_e9562_d_n2, assign14890_e9562_d_n4, assign14890_e9562_d_n5, assign14890_e9562_d_n6, assign14890_e9562_d_n7, assign14890_e9562_d_n8, assign14890_e9562_d_n9, assign14890_e9562_d_n10, assign14890_e9562_d_n11, assign14890_e9562_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14890_e9561: f64 = (-locals.var_tmf2);
                (assign14890_e9561, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14890_e9562, assign14890_e9562_d_n0, assign14890_e9562_d_n2, assign14890_e9562_d_n4, assign14890_e9562_d_n5, assign14890_e9562_d_n6, assign14890_e9562_d_n7, assign14890_e9562_d_n8, assign14890_e9562_d_n9, assign14890_e9562_d_n10, assign14890_e9562_d_n11, assign14890_e9562_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14890_e9564;
        locals.var_tmf2_dn0 = assign14890_e9564_d_n0;
        locals.var_tmf2_dn2 = assign14890_e9564_d_n2;
        locals.var_tmf2_dn4 = assign14890_e9564_d_n4;
        locals.var_tmf2_dn5 = assign14890_e9564_d_n5;
        locals.var_tmf2_dn6 = assign14890_e9564_d_n6;
        locals.var_tmf2_dn7 = assign14890_e9564_d_n7;
        locals.var_tmf2_dn8 = assign14890_e9564_d_n8;
        locals.var_tmf2_dn9 = assign14890_e9564_d_n9;
        locals.var_tmf2_dn10 = assign14890_e9564_d_n10;
        locals.var_tmf2_dn11 = assign14890_e9564_d_n11;
        locals.var_tmf2_dn14 = assign14890_e9564_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14900_e9580, assign14900_e9580_d_n0, assign14900_e9580_d_n2, assign14900_e9580_d_n4, assign14900_e9580_d_n5, assign14900_e9580_d_n6, assign14900_e9580_d_n7, assign14900_e9580_d_n8, assign14900_e9580_d_n9, assign14900_e9580_d_n10, assign14900_e9580_d_n11, assign14900_e9580_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14900_e9575: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14900_e9577: f64 = (assign14900_e9575 + locals.var_tmf2);
        let assign14900_e9578: f64 = (assign14900_e9577).sqrt();
        (assign14900_e9578, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14900_e9578)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14900_e9580;
        locals.var_tmf2_dn0 = assign14900_e9580_d_n0;
        locals.var_tmf2_dn2 = assign14900_e9580_d_n2;
        locals.var_tmf2_dn4 = assign14900_e9580_d_n4;
        locals.var_tmf2_dn5 = assign14900_e9580_d_n5;
        locals.var_tmf2_dn6 = assign14900_e9580_d_n6;
        locals.var_tmf2_dn7 = assign14900_e9580_d_n7;
        locals.var_tmf2_dn8 = assign14900_e9580_d_n8;
        locals.var_tmf2_dn9 = assign14900_e9580_d_n9;
        locals.var_tmf2_dn10 = assign14900_e9580_d_n10;
        locals.var_tmf2_dn11 = assign14900_e9580_d_n11;
        locals.var_tmf2_dn14 = assign14900_e9580_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14910_e9597, assign14910_e9597_d_n0, assign14910_e9597_d_n2, assign14910_e9597_d_n4, assign14910_e9597_d_n5, assign14910_e9597_d_n6, assign14910_e9597_d_n7, assign14910_e9597_d_n8, assign14910_e9597_d_n9, assign14910_e9597_d_n10, assign14910_e9597_d_n11, assign14910_e9597_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14910_e9593: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14910_e9594: f64 = (1.0 + assign14910_e9593);
        let assign14910_e9595: f64 = (0.5 * assign14910_e9594);
        (assign14910_e9595, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14910_e9597;
        locals.var_t0_dn0 = assign14910_e9597_d_n0;
        locals.var_t0_dn2 = assign14910_e9597_d_n2;
        locals.var_t0_dn4 = assign14910_e9597_d_n4;
        locals.var_t0_dn5 = assign14910_e9597_d_n5;
        locals.var_t0_dn6 = assign14910_e9597_d_n6;
        locals.var_t0_dn7 = assign14910_e9597_d_n7;
        locals.var_t0_dn8 = assign14910_e9597_d_n8;
        locals.var_t0_dn9 = assign14910_e9597_d_n9;
        locals.var_t0_dn10 = assign14910_e9597_d_n10;
        locals.var_t0_dn11 = assign14910_e9597_d_n11;
        locals.var_t0_dn14 = assign14910_e9597_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign14920_e9616, assign14920_e9616_d_n0, assign14920_e9616_d_n2, assign14920_e9616_d_n4, assign14920_e9616_d_n5, assign14920_e9616_d_n6, assign14920_e9616_d_n7, assign14920_e9616_d_n8, assign14920_e9616_d_n9, assign14920_e9616_d_n10, assign14920_e9616_d_n11, assign14920_e9616_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14920_e9608: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14920_e9612: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14920_e9613: f64 = (0.5 * assign14920_e9612);
        let assign14920_e9614: f64 = (assign14920_e9608 + assign14920_e9613);
        (assign14920_e9614, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14920_e9616;
        locals.var_rdvde_dn0 = assign14920_e9616_d_n0;
        locals.var_rdvde_dn2 = assign14920_e9616_d_n2;
        locals.var_rdvde_dn4 = assign14920_e9616_d_n4;
        locals.var_rdvde_dn5 = assign14920_e9616_d_n5;
        locals.var_rdvde_dn6 = assign14920_e9616_d_n6;
        locals.var_rdvde_dn7 = assign14920_e9616_d_n7;
        locals.var_rdvde_dn8 = assign14920_e9616_d_n8;
        locals.var_rdvde_dn9 = assign14920_e9616_d_n9;
        locals.var_rdvde_dn10 = assign14920_e9616_d_n10;
        locals.var_rdvde_dn11 = assign14920_e9616_d_n11;
        locals.var_rdvde_dn14 = assign14920_e9616_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign14930_e9640, assign14930_e9640_d_n0, assign14930_e9640_d_n2, assign14930_e9640_d_n4, assign14930_e9640_d_n5, assign14930_e9640_d_n6, assign14930_e9640_d_n7, assign14930_e9640_d_n8, assign14930_e9640_d_n9, assign14930_e9640_d_n10, assign14930_e9640_d_n11, assign14930_e9640_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14930_e9625: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14930_e9627: f64 = (assign14930_e9625 * 1000000.0);
        let assign14930_e9629: f64 = (assign14930_e9627 + locals.var_uc_rdict1);
        let assign14930_e9630: f64 = (locals.var_rdvdtemp0 * assign14930_e9629);
        let assign14930_e9633: f64 = (p.p70 * p.p100);
        let assign14930_e9635: f64 = (assign14930_e9633 * 1000000.0);
        let assign14930_e9637: f64 = (assign14930_e9635 + p.p101);
        let assign14930_e9638: f64 = (assign14930_e9630 * assign14930_e9637);
        (assign14930_e9638, ((locals.var_rdvdtemp0_dn0 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn2 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn4 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn5 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn6 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn7 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn8 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn9 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn10 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn11 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn14 * assign14930_e9629) * assign14930_e9637),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14930_e9640;
        locals.var_t4_dn0 = assign14930_e9640_d_n0;
        locals.var_t4_dn2 = assign14930_e9640_d_n2;
        locals.var_t4_dn4 = assign14930_e9640_d_n4;
        locals.var_t4_dn5 = assign14930_e9640_d_n5;
        locals.var_t4_dn6 = assign14930_e9640_d_n6;
        locals.var_t4_dn7 = assign14930_e9640_d_n7;
        locals.var_t4_dn8 = assign14930_e9640_d_n8;
        locals.var_t4_dn9 = assign14930_e9640_d_n9;
        locals.var_t4_dn10 = assign14930_e9640_d_n10;
        locals.var_t4_dn11 = assign14930_e9640_d_n11;
        locals.var_t4_dn14 = assign14930_e9640_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign14940_e9654, assign14940_e9654_d_n0, assign14940_e9654_d_n2, assign14940_e9654_d_n4, assign14940_e9654_d_n5, assign14940_e9654_d_n6, assign14940_e9654_d_n7, assign14940_e9654_d_n8, assign14940_e9654_d_n9, assign14940_e9654_d_n10, assign14940_e9654_d_n11, assign14940_e9654_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14940_e9648: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14940_e9650: f64 = (assign14940_e9648 * p.p66);
        let assign14940_e9652: f64 = (assign14940_e9650 * 1000000.0);
        (assign14940_e9652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14940_e9654;
        locals.var_t1_dn0 = assign14940_e9654_d_n0;
        locals.var_t1_dn2 = assign14940_e9654_d_n2;
        locals.var_t1_dn4 = assign14940_e9654_d_n4;
        locals.var_t1_dn5 = assign14940_e9654_d_n5;
        locals.var_t1_dn6 = assign14940_e9654_d_n6;
        locals.var_t1_dn7 = assign14940_e9654_d_n7;
        locals.var_t1_dn8 = assign14940_e9654_d_n8;
        locals.var_t1_dn9 = assign14940_e9654_d_n9;
        locals.var_t1_dn10 = assign14940_e9654_d_n10;
        locals.var_t1_dn11 = assign14940_e9654_d_n11;
        locals.var_t1_dn14 = assign14940_e9654_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14950_e9670, assign14950_e9670_d_n0, assign14950_e9670_d_n2, assign14950_e9670_d_n4, assign14950_e9670_d_n5, assign14950_e9670_d_n6, assign14950_e9670_d_n7, assign14950_e9670_d_n8, assign14950_e9670_d_n9, assign14950_e9670_d_n10, assign14950_e9670_d_n11, assign14950_e9670_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14950_e9662: f64 = (locals.var_t8 * p.p66);
        let assign14950_e9664: f64 = (assign14950_e9662 * 1000000.0);
        let assign14950_e9666: f64 = (assign14950_e9664 + 1.0);
        let assign14950_e9668: f64 = (assign14950_e9666 + p.p98);
        (assign14950_e9668, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn11 * p.p66) * 1000000.0), ((locals.var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14950_e9670;
        locals.var_t3_dn0 = assign14950_e9670_d_n0;
        locals.var_t3_dn2 = assign14950_e9670_d_n2;
        locals.var_t3_dn4 = assign14950_e9670_d_n4;
        locals.var_t3_dn5 = assign14950_e9670_d_n5;
        locals.var_t3_dn6 = assign14950_e9670_d_n6;
        locals.var_t3_dn7 = assign14950_e9670_d_n7;
        locals.var_t3_dn8 = assign14950_e9670_d_n8;
        locals.var_t3_dn9 = assign14950_e9670_d_n9;
        locals.var_t3_dn10 = assign14950_e9670_d_n10;
        locals.var_t3_dn11 = assign14950_e9670_d_n11;
        locals.var_t3_dn14 = assign14950_e9670_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14960_e9684, assign14960_e9684_d_n0, assign14960_e9684_d_n2, assign14960_e9684_d_n4, assign14960_e9684_d_n5, assign14960_e9684_d_n6, assign14960_e9684_d_n7, assign14960_e9684_d_n8, assign14960_e9684_d_n9, assign14960_e9684_d_n10, assign14960_e9684_d_n11, assign14960_e9684_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14960_e9678: f64 = (locals.var_t3 * locals.var_t4);
        let assign14960_e9680: f64 = (assign14960_e9678 - locals.var_t4);
        let assign14960_e9682: f64 = (assign14960_e9680 - 0.01);
        (assign14960_e9682, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14960_e9684;
        locals.var_tmf1_dn0 = assign14960_e9684_d_n0;
        locals.var_tmf1_dn2 = assign14960_e9684_d_n2;
        locals.var_tmf1_dn4 = assign14960_e9684_d_n4;
        locals.var_tmf1_dn5 = assign14960_e9684_d_n5;
        locals.var_tmf1_dn6 = assign14960_e9684_d_n6;
        locals.var_tmf1_dn7 = assign14960_e9684_d_n7;
        locals.var_tmf1_dn8 = assign14960_e9684_d_n8;
        locals.var_tmf1_dn9 = assign14960_e9684_d_n9;
        locals.var_tmf1_dn10 = assign14960_e9684_d_n10;
        locals.var_tmf1_dn11 = assign14960_e9684_d_n11;
        locals.var_tmf1_dn14 = assign14960_e9684_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign14970_e9696, assign14970_e9696_d_n0, assign14970_e9696_d_n2, assign14970_e9696_d_n4, assign14970_e9696_d_n5, assign14970_e9696_d_n6, assign14970_e9696_d_n7, assign14970_e9696_d_n8, assign14970_e9696_d_n9, assign14970_e9696_d_n10, assign14970_e9696_d_n11, assign14970_e9696_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14970_e9692: f64 = (4.0 * locals.var_t4);
        let assign14970_e9694: f64 = (assign14970_e9692 * 0.01);
        (assign14970_e9694, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14970_e9696;
        locals.var_tmf2_dn0 = assign14970_e9696_d_n0;
        locals.var_tmf2_dn2 = assign14970_e9696_d_n2;
        locals.var_tmf2_dn4 = assign14970_e9696_d_n4;
        locals.var_tmf2_dn5 = assign14970_e9696_d_n5;
        locals.var_tmf2_dn6 = assign14970_e9696_d_n6;
        locals.var_tmf2_dn7 = assign14970_e9696_d_n7;
        locals.var_tmf2_dn8 = assign14970_e9696_d_n8;
        locals.var_tmf2_dn9 = assign14970_e9696_d_n9;
        locals.var_tmf2_dn10 = assign14970_e9696_d_n10;
        locals.var_tmf2_dn11 = assign14970_e9696_d_n11;
        locals.var_tmf2_dn14 = assign14970_e9696_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14980_e9710, assign14980_e9710_d_n0, assign14980_e9710_d_n2, assign14980_e9710_d_n4, assign14980_e9710_d_n5, assign14980_e9710_d_n6, assign14980_e9710_d_n7, assign14980_e9710_d_n8, assign14980_e9710_d_n9, assign14980_e9710_d_n10, assign14980_e9710_d_n11, assign14980_e9710_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14980_e9708, assign14980_e9708_d_n0, assign14980_e9708_d_n2, assign14980_e9708_d_n4, assign14980_e9708_d_n5, assign14980_e9708_d_n6, assign14980_e9708_d_n7, assign14980_e9708_d_n8, assign14980_e9708_d_n9, assign14980_e9708_d_n10, assign14980_e9708_d_n11, assign14980_e9708_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14980_e9707: f64 = (-locals.var_tmf2);
                (assign14980_e9707, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14980_e9708, assign14980_e9708_d_n0, assign14980_e9708_d_n2, assign14980_e9708_d_n4, assign14980_e9708_d_n5, assign14980_e9708_d_n6, assign14980_e9708_d_n7, assign14980_e9708_d_n8, assign14980_e9708_d_n9, assign14980_e9708_d_n10, assign14980_e9708_d_n11, assign14980_e9708_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14980_e9710;
        locals.var_tmf2_dn0 = assign14980_e9710_d_n0;
        locals.var_tmf2_dn2 = assign14980_e9710_d_n2;
        locals.var_tmf2_dn4 = assign14980_e9710_d_n4;
        locals.var_tmf2_dn5 = assign14980_e9710_d_n5;
        locals.var_tmf2_dn6 = assign14980_e9710_d_n6;
        locals.var_tmf2_dn7 = assign14980_e9710_d_n7;
        locals.var_tmf2_dn8 = assign14980_e9710_d_n8;
        locals.var_tmf2_dn9 = assign14980_e9710_d_n9;
        locals.var_tmf2_dn10 = assign14980_e9710_d_n10;
        locals.var_tmf2_dn11 = assign14980_e9710_d_n11;
        locals.var_tmf2_dn14 = assign14980_e9710_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign14990_e9723, assign14990_e9723_d_n0, assign14990_e9723_d_n2, assign14990_e9723_d_n4, assign14990_e9723_d_n5, assign14990_e9723_d_n6, assign14990_e9723_d_n7, assign14990_e9723_d_n8, assign14990_e9723_d_n9, assign14990_e9723_d_n10, assign14990_e9723_d_n11, assign14990_e9723_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14990_e9718: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14990_e9720: f64 = (assign14990_e9718 + locals.var_tmf2);
        let assign14990_e9721: f64 = (assign14990_e9720).sqrt();
        (assign14990_e9721, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14990_e9721)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14990_e9723;
        locals.var_tmf2_dn0 = assign14990_e9723_d_n0;
        locals.var_tmf2_dn2 = assign14990_e9723_d_n2;
        locals.var_tmf2_dn4 = assign14990_e9723_d_n4;
        locals.var_tmf2_dn5 = assign14990_e9723_d_n5;
        locals.var_tmf2_dn6 = assign14990_e9723_d_n6;
        locals.var_tmf2_dn7 = assign14990_e9723_d_n7;
        locals.var_tmf2_dn8 = assign14990_e9723_d_n8;
        locals.var_tmf2_dn9 = assign14990_e9723_d_n9;
        locals.var_tmf2_dn10 = assign14990_e9723_d_n10;
        locals.var_tmf2_dn11 = assign14990_e9723_d_n11;
        locals.var_tmf2_dn14 = assign14990_e9723_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15000_e9737, assign15000_e9737_d_n0, assign15000_e9737_d_n2, assign15000_e9737_d_n4, assign15000_e9737_d_n5, assign15000_e9737_d_n6, assign15000_e9737_d_n7, assign15000_e9737_d_n8, assign15000_e9737_d_n9, assign15000_e9737_d_n10, assign15000_e9737_d_n11, assign15000_e9737_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15000_e9733: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15000_e9734: f64 = (1.0 + assign15000_e9733);
        let assign15000_e9735: f64 = (0.5 * assign15000_e9734);
        (assign15000_e9735, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15000_e9737;
        locals.var_t6_dn0 = assign15000_e9737_d_n0;
        locals.var_t6_dn2 = assign15000_e9737_d_n2;
        locals.var_t6_dn4 = assign15000_e9737_d_n4;
        locals.var_t6_dn5 = assign15000_e9737_d_n5;
        locals.var_t6_dn6 = assign15000_e9737_d_n6;
        locals.var_t6_dn7 = assign15000_e9737_d_n7;
        locals.var_t6_dn8 = assign15000_e9737_d_n8;
        locals.var_t6_dn9 = assign15000_e9737_d_n9;
        locals.var_t6_dn10 = assign15000_e9737_d_n10;
        locals.var_t6_dn11 = assign15000_e9737_d_n11;
        locals.var_t6_dn14 = assign15000_e9737_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign15010_e9751, assign15010_e9751_d_n0, assign15010_e9751_d_n2, assign15010_e9751_d_n4, assign15010_e9751_d_n5, assign15010_e9751_d_n6, assign15010_e9751_d_n7, assign15010_e9751_d_n8, assign15010_e9751_d_n9, assign15010_e9751_d_n10, assign15010_e9751_d_n11, assign15010_e9751_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15010_e9747: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15010_e9748: f64 = (0.5 * assign15010_e9747);
        let assign15010_e9749: f64 = (locals.var_t4 + assign15010_e9748);
        (assign15010_e9749, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign15010_e9751;
        locals.var_t5_dn0 = assign15010_e9751_d_n0;
        locals.var_t5_dn2 = assign15010_e9751_d_n2;
        locals.var_t5_dn4 = assign15010_e9751_d_n4;
        locals.var_t5_dn5 = assign15010_e9751_d_n5;
        locals.var_t5_dn6 = assign15010_e9751_d_n6;
        locals.var_t5_dn7 = assign15010_e9751_d_n7;
        locals.var_t5_dn8 = assign15010_e9751_d_n8;
        locals.var_t5_dn9 = assign15010_e9751_d_n9;
        locals.var_t5_dn10 = assign15010_e9751_d_n10;
        locals.var_t5_dn11 = assign15010_e9751_d_n11;
        locals.var_t5_dn14 = assign15010_e9751_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign15020_e9767, assign15020_e9767_d_n0, assign15020_e9767_d_n2, assign15020_e9767_d_n4, assign15020_e9767_d_n5, assign15020_e9767_d_n6, assign15020_e9767_d_n7, assign15020_e9767_d_n8, assign15020_e9767_d_n9, assign15020_e9767_d_n10, assign15020_e9767_d_n11, assign15020_e9767_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15020_e9760: f64 = (p.p98 + 1.0);
        let assign15020_e9761: f64 = (locals.var_t4 * assign15020_e9760);
        let assign15020_e9763: f64 = (assign15020_e9761 - locals.var_t5);
        let assign15020_e9765: f64 = (assign15020_e9763 - 5e-5);
        (assign15020_e9765, ((locals.var_t4_dn0 * assign15020_e9760) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign15020_e9760) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign15020_e9760) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign15020_e9760) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign15020_e9760) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign15020_e9760) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign15020_e9760) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign15020_e9760) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign15020_e9760) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign15020_e9760) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign15020_e9760) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15020_e9767;
        locals.var_tmf1_dn0 = assign15020_e9767_d_n0;
        locals.var_tmf1_dn2 = assign15020_e9767_d_n2;
        locals.var_tmf1_dn4 = assign15020_e9767_d_n4;
        locals.var_tmf1_dn5 = assign15020_e9767_d_n5;
        locals.var_tmf1_dn6 = assign15020_e9767_d_n6;
        locals.var_tmf1_dn7 = assign15020_e9767_d_n7;
        locals.var_tmf1_dn8 = assign15020_e9767_d_n8;
        locals.var_tmf1_dn9 = assign15020_e9767_d_n9;
        locals.var_tmf1_dn10 = assign15020_e9767_d_n10;
        locals.var_tmf1_dn11 = assign15020_e9767_d_n11;
        locals.var_tmf1_dn14 = assign15020_e9767_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign15030_e9783, assign15030_e9783_d_n0, assign15030_e9783_d_n2, assign15030_e9783_d_n4, assign15030_e9783_d_n5, assign15030_e9783_d_n6, assign15030_e9783_d_n7, assign15030_e9783_d_n8, assign15030_e9783_d_n9, assign15030_e9783_d_n10, assign15030_e9783_d_n11, assign15030_e9783_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15030_e9777: f64 = (p.p98 + 1.0);
        let assign15030_e9778: f64 = (locals.var_t4 * assign15030_e9777);
        let assign15030_e9779: f64 = (4.0 * assign15030_e9778);
        let assign15030_e9781: f64 = (assign15030_e9779 * 5e-5);
        (assign15030_e9781, ((4.0 * (locals.var_t4_dn0 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign15030_e9777)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15030_e9783;
        locals.var_tmf2_dn0 = assign15030_e9783_d_n0;
        locals.var_tmf2_dn2 = assign15030_e9783_d_n2;
        locals.var_tmf2_dn4 = assign15030_e9783_d_n4;
        locals.var_tmf2_dn5 = assign15030_e9783_d_n5;
        locals.var_tmf2_dn6 = assign15030_e9783_d_n6;
        locals.var_tmf2_dn7 = assign15030_e9783_d_n7;
        locals.var_tmf2_dn8 = assign15030_e9783_d_n8;
        locals.var_tmf2_dn9 = assign15030_e9783_d_n9;
        locals.var_tmf2_dn10 = assign15030_e9783_d_n10;
        locals.var_tmf2_dn11 = assign15030_e9783_d_n11;
        locals.var_tmf2_dn14 = assign15030_e9783_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15040_e9797, assign15040_e9797_d_n0, assign15040_e9797_d_n2, assign15040_e9797_d_n4, assign15040_e9797_d_n5, assign15040_e9797_d_n6, assign15040_e9797_d_n7, assign15040_e9797_d_n8, assign15040_e9797_d_n9, assign15040_e9797_d_n10, assign15040_e9797_d_n11, assign15040_e9797_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign15040_e9795, assign15040_e9795_d_n0, assign15040_e9795_d_n2, assign15040_e9795_d_n4, assign15040_e9795_d_n5, assign15040_e9795_d_n6, assign15040_e9795_d_n7, assign15040_e9795_d_n8, assign15040_e9795_d_n9, assign15040_e9795_d_n10, assign15040_e9795_d_n11, assign15040_e9795_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15040_e9794: f64 = (-locals.var_tmf2);
                (assign15040_e9794, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15040_e9795, assign15040_e9795_d_n0, assign15040_e9795_d_n2, assign15040_e9795_d_n4, assign15040_e9795_d_n5, assign15040_e9795_d_n6, assign15040_e9795_d_n7, assign15040_e9795_d_n8, assign15040_e9795_d_n9, assign15040_e9795_d_n10, assign15040_e9795_d_n11, assign15040_e9795_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15040_e9797;
        locals.var_tmf2_dn0 = assign15040_e9797_d_n0;
        locals.var_tmf2_dn2 = assign15040_e9797_d_n2;
        locals.var_tmf2_dn4 = assign15040_e9797_d_n4;
        locals.var_tmf2_dn5 = assign15040_e9797_d_n5;
        locals.var_tmf2_dn6 = assign15040_e9797_d_n6;
        locals.var_tmf2_dn7 = assign15040_e9797_d_n7;
        locals.var_tmf2_dn8 = assign15040_e9797_d_n8;
        locals.var_tmf2_dn9 = assign15040_e9797_d_n9;
        locals.var_tmf2_dn10 = assign15040_e9797_d_n10;
        locals.var_tmf2_dn11 = assign15040_e9797_d_n11;
        locals.var_tmf2_dn14 = assign15040_e9797_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15050_e9810, assign15050_e9810_d_n0, assign15050_e9810_d_n2, assign15050_e9810_d_n4, assign15050_e9810_d_n5, assign15050_e9810_d_n6, assign15050_e9810_d_n7, assign15050_e9810_d_n8, assign15050_e9810_d_n9, assign15050_e9810_d_n10, assign15050_e9810_d_n11, assign15050_e9810_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15050_e9805: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15050_e9807: f64 = (assign15050_e9805 + locals.var_tmf2);
        let assign15050_e9808: f64 = (assign15050_e9807).sqrt();
        (assign15050_e9808, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15050_e9808)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15050_e9810;
        locals.var_tmf2_dn0 = assign15050_e9810_d_n0;
        locals.var_tmf2_dn2 = assign15050_e9810_d_n2;
        locals.var_tmf2_dn4 = assign15050_e9810_d_n4;
        locals.var_tmf2_dn5 = assign15050_e9810_d_n5;
        locals.var_tmf2_dn6 = assign15050_e9810_d_n6;
        locals.var_tmf2_dn7 = assign15050_e9810_d_n7;
        locals.var_tmf2_dn8 = assign15050_e9810_d_n8;
        locals.var_tmf2_dn9 = assign15050_e9810_d_n9;
        locals.var_tmf2_dn10 = assign15050_e9810_d_n10;
        locals.var_tmf2_dn11 = assign15050_e9810_d_n11;
        locals.var_tmf2_dn14 = assign15050_e9810_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15060_e9824, assign15060_e9824_d_n0, assign15060_e9824_d_n2, assign15060_e9824_d_n4, assign15060_e9824_d_n5, assign15060_e9824_d_n6, assign15060_e9824_d_n7, assign15060_e9824_d_n8, assign15060_e9824_d_n9, assign15060_e9824_d_n10, assign15060_e9824_d_n11, assign15060_e9824_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15060_e9820: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15060_e9821: f64 = (1.0 + assign15060_e9820);
        let assign15060_e9822: f64 = (0.5 * assign15060_e9821);
        (assign15060_e9822, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15060_e9824;
        locals.var_t6_dn0 = assign15060_e9824_d_n0;
        locals.var_t6_dn2 = assign15060_e9824_d_n2;
        locals.var_t6_dn4 = assign15060_e9824_d_n4;
        locals.var_t6_dn5 = assign15060_e9824_d_n5;
        locals.var_t6_dn6 = assign15060_e9824_d_n6;
        locals.var_t6_dn7 = assign15060_e9824_d_n7;
        locals.var_t6_dn8 = assign15060_e9824_d_n8;
        locals.var_t6_dn9 = assign15060_e9824_d_n9;
        locals.var_t6_dn10 = assign15060_e9824_d_n10;
        locals.var_t6_dn11 = assign15060_e9824_d_n11;
        locals.var_t6_dn14 = assign15060_e9824_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign15070_e9842, assign15070_e9842_d_n0, assign15070_e9842_d_n2, assign15070_e9842_d_n4, assign15070_e9842_d_n5, assign15070_e9842_d_n6, assign15070_e9842_d_n7, assign15070_e9842_d_n8, assign15070_e9842_d_n9, assign15070_e9842_d_n10, assign15070_e9842_d_n11, assign15070_e9842_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15070_e9833: f64 = (p.p98 + 1.0);
        let assign15070_e9834: f64 = (locals.var_t4 * assign15070_e9833);
        let assign15070_e9838: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15070_e9839: f64 = (0.5 * assign15070_e9838);
        let assign15070_e9840: f64 = (assign15070_e9834 - assign15070_e9839);
        (assign15070_e9840, ((locals.var_t4_dn0 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign15070_e9842;
        locals.var_t7_dn0 = assign15070_e9842_d_n0;
        locals.var_t7_dn2 = assign15070_e9842_d_n2;
        locals.var_t7_dn4 = assign15070_e9842_d_n4;
        locals.var_t7_dn5 = assign15070_e9842_d_n5;
        locals.var_t7_dn6 = assign15070_e9842_d_n6;
        locals.var_t7_dn7 = assign15070_e9842_d_n7;
        locals.var_t7_dn8 = assign15070_e9842_d_n8;
        locals.var_t7_dn9 = assign15070_e9842_d_n9;
        locals.var_t7_dn10 = assign15070_e9842_d_n10;
        locals.var_t7_dn11 = assign15070_e9842_d_n11;
        locals.var_t7_dn14 = assign15070_e9842_d_n14;
        locals.var_t7_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15080_e9858, assign15080_e9858_d_n0, assign15080_e9858_d_n2, assign15080_e9858_d_n4, assign15080_e9858_d_n5, assign15080_e9858_d_n6, assign15080_e9858_d_n7, assign15080_e9858_d_n8, assign15080_e9858_d_n9, assign15080_e9858_d_n10, assign15080_e9858_d_n11, assign15080_e9858_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15080_e9851: f64 = (locals.var_t1 * locals.var_t4);
        let assign15080_e9852: f64 = (locals.var_t7 + assign15080_e9851);
        let assign15080_e9854: f64 = assign15080_e9852;
        let assign15080_e9856: f64 = (assign15080_e9854 - 5e-5);
        (assign15080_e9856, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15080_e9858;
        locals.var_tmf1_dn0 = assign15080_e9858_d_n0;
        locals.var_tmf1_dn2 = assign15080_e9858_d_n2;
        locals.var_tmf1_dn4 = assign15080_e9858_d_n4;
        locals.var_tmf1_dn5 = assign15080_e9858_d_n5;
        locals.var_tmf1_dn6 = assign15080_e9858_d_n6;
        locals.var_tmf1_dn7 = assign15080_e9858_d_n7;
        locals.var_tmf1_dn8 = assign15080_e9858_d_n8;
        locals.var_tmf1_dn9 = assign15080_e9858_d_n9;
        locals.var_tmf1_dn10 = assign15080_e9858_d_n10;
        locals.var_tmf1_dn11 = assign15080_e9858_d_n11;
        locals.var_tmf1_dn14 = assign15080_e9858_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign15090_e9870, assign15090_e9870_d_n0, assign15090_e9870_d_n2, assign15090_e9870_d_n4, assign15090_e9870_d_n5, assign15090_e9870_d_n6, assign15090_e9870_d_n7, assign15090_e9870_d_n8, assign15090_e9870_d_n9, assign15090_e9870_d_n10, assign15090_e9870_d_n11, assign15090_e9870_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15090_e9870;
        locals.var_tmf2_dn0 = assign15090_e9870_d_n0;
        locals.var_tmf2_dn2 = assign15090_e9870_d_n2;
        locals.var_tmf2_dn4 = assign15090_e9870_d_n4;
        locals.var_tmf2_dn5 = assign15090_e9870_d_n5;
        locals.var_tmf2_dn6 = assign15090_e9870_d_n6;
        locals.var_tmf2_dn7 = assign15090_e9870_d_n7;
        locals.var_tmf2_dn8 = assign15090_e9870_d_n8;
        locals.var_tmf2_dn9 = assign15090_e9870_d_n9;
        locals.var_tmf2_dn10 = assign15090_e9870_d_n10;
        locals.var_tmf2_dn11 = assign15090_e9870_d_n11;
        locals.var_tmf2_dn14 = assign15090_e9870_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15100_e9884, assign15100_e9884_d_n0, assign15100_e9884_d_n2, assign15100_e9884_d_n4, assign15100_e9884_d_n5, assign15100_e9884_d_n6, assign15100_e9884_d_n7, assign15100_e9884_d_n8, assign15100_e9884_d_n9, assign15100_e9884_d_n10, assign15100_e9884_d_n11, assign15100_e9884_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign15100_e9882, assign15100_e9882_d_n0, assign15100_e9882_d_n2, assign15100_e9882_d_n4, assign15100_e9882_d_n5, assign15100_e9882_d_n6, assign15100_e9882_d_n7, assign15100_e9882_d_n8, assign15100_e9882_d_n9, assign15100_e9882_d_n10, assign15100_e9882_d_n11, assign15100_e9882_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15100_e9881: f64 = (-locals.var_tmf2);
                (assign15100_e9881, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15100_e9882, assign15100_e9882_d_n0, assign15100_e9882_d_n2, assign15100_e9882_d_n4, assign15100_e9882_d_n5, assign15100_e9882_d_n6, assign15100_e9882_d_n7, assign15100_e9882_d_n8, assign15100_e9882_d_n9, assign15100_e9882_d_n10, assign15100_e9882_d_n11, assign15100_e9882_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15100_e9884;
        locals.var_tmf2_dn0 = assign15100_e9884_d_n0;
        locals.var_tmf2_dn2 = assign15100_e9884_d_n2;
        locals.var_tmf2_dn4 = assign15100_e9884_d_n4;
        locals.var_tmf2_dn5 = assign15100_e9884_d_n5;
        locals.var_tmf2_dn6 = assign15100_e9884_d_n6;
        locals.var_tmf2_dn7 = assign15100_e9884_d_n7;
        locals.var_tmf2_dn8 = assign15100_e9884_d_n8;
        locals.var_tmf2_dn9 = assign15100_e9884_d_n9;
        locals.var_tmf2_dn10 = assign15100_e9884_d_n10;
        locals.var_tmf2_dn11 = assign15100_e9884_d_n11;
        locals.var_tmf2_dn14 = assign15100_e9884_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15110_e9897, assign15110_e9897_d_n0, assign15110_e9897_d_n2, assign15110_e9897_d_n4, assign15110_e9897_d_n5, assign15110_e9897_d_n6, assign15110_e9897_d_n7, assign15110_e9897_d_n8, assign15110_e9897_d_n9, assign15110_e9897_d_n10, assign15110_e9897_d_n11, assign15110_e9897_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15110_e9892: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15110_e9894: f64 = (assign15110_e9892 + locals.var_tmf2);
        let assign15110_e9895: f64 = (assign15110_e9894).sqrt();
        (assign15110_e9895, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15110_e9895)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15110_e9897;
        locals.var_tmf2_dn0 = assign15110_e9897_d_n0;
        locals.var_tmf2_dn2 = assign15110_e9897_d_n2;
        locals.var_tmf2_dn4 = assign15110_e9897_d_n4;
        locals.var_tmf2_dn5 = assign15110_e9897_d_n5;
        locals.var_tmf2_dn6 = assign15110_e9897_d_n6;
        locals.var_tmf2_dn7 = assign15110_e9897_d_n7;
        locals.var_tmf2_dn8 = assign15110_e9897_d_n8;
        locals.var_tmf2_dn9 = assign15110_e9897_d_n9;
        locals.var_tmf2_dn10 = assign15110_e9897_d_n10;
        locals.var_tmf2_dn11 = assign15110_e9897_d_n11;
        locals.var_tmf2_dn14 = assign15110_e9897_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15120_e9911, assign15120_e9911_d_n0, assign15120_e9911_d_n2, assign15120_e9911_d_n4, assign15120_e9911_d_n5, assign15120_e9911_d_n6, assign15120_e9911_d_n7, assign15120_e9911_d_n8, assign15120_e9911_d_n9, assign15120_e9911_d_n10, assign15120_e9911_d_n11, assign15120_e9911_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15120_e9907: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15120_e9908: f64 = (1.0 + assign15120_e9907);
        let assign15120_e9909: f64 = (0.5 * assign15120_e9908);
        (assign15120_e9909, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15120_e9911;
        locals.var_t6_dn0 = assign15120_e9911_d_n0;
        locals.var_t6_dn2 = assign15120_e9911_d_n2;
        locals.var_t6_dn4 = assign15120_e9911_d_n4;
        locals.var_t6_dn5 = assign15120_e9911_d_n5;
        locals.var_t6_dn6 = assign15120_e9911_d_n6;
        locals.var_t6_dn7 = assign15120_e9911_d_n7;
        locals.var_t6_dn8 = assign15120_e9911_d_n8;
        locals.var_t6_dn9 = assign15120_e9911_d_n9;
        locals.var_t6_dn10 = assign15120_e9911_d_n10;
        locals.var_t6_dn11 = assign15120_e9911_d_n11;
        locals.var_t6_dn14 = assign15120_e9911_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign15130_e9925, assign15130_e9925_d_n0, assign15130_e9925_d_n2, assign15130_e9925_d_n4, assign15130_e9925_d_n5, assign15130_e9925_d_n6, assign15130_e9925_d_n7, assign15130_e9925_d_n8, assign15130_e9925_d_n9, assign15130_e9925_d_n10, assign15130_e9925_d_n11, assign15130_e9925_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15130_e9921: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15130_e9922: f64 = (0.5 * assign15130_e9921);
        let assign15130_e9923: f64 = assign15130_e9922;
        (assign15130_e9923, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15130_e9925;
        locals.var_t2_dn0 = assign15130_e9925_d_n0;
        locals.var_t2_dn2 = assign15130_e9925_d_n2;
        locals.var_t2_dn4 = assign15130_e9925_d_n4;
        locals.var_t2_dn5 = assign15130_e9925_d_n5;
        locals.var_t2_dn6 = assign15130_e9925_d_n6;
        locals.var_t2_dn7 = assign15130_e9925_d_n7;
        locals.var_t2_dn8 = assign15130_e9925_d_n8;
        locals.var_t2_dn9 = assign15130_e9925_d_n9;
        locals.var_t2_dn10 = assign15130_e9925_d_n10;
        locals.var_t2_dn11 = assign15130_e9925_d_n11;
        locals.var_t2_dn14 = assign15130_e9925_d_n14;
        locals.var_t2_rv = 0.0;

        let assign15140_e9932: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard323 = assign15140_e9932;
        locals.var_guard323_rv = 0.0;

        let (assign15150_e9952, assign15150_e9952_d_n0, assign15150_e9952_d_n2, assign15150_e9952_d_n4, assign15150_e9952_d_n5, assign15150_e9952_d_n6, assign15150_e9952_d_n7, assign15150_e9952_d_n8, assign15150_e9952_d_n9, assign15150_e9952_d_n10, assign15150_e9952_d_n11, assign15150_e9952_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15150_e9943: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign15150_e9944: f64 = (locals.var_uc_rdvd + assign15150_e9943);
        let assign15150_e9947: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign15150_e9948: f64 = (assign15150_e9944 + assign15150_e9947);
        let assign15150_e9950: f64 = (assign15150_e9948 * locals.var_t2);
        (assign15150_e9950, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15150_e9952;
        locals.var_rsvde_dn0 = assign15150_e9952_d_n0;
        locals.var_rsvde_dn2 = assign15150_e9952_d_n2;
        locals.var_rsvde_dn4 = assign15150_e9952_d_n4;
        locals.var_rsvde_dn5 = assign15150_e9952_d_n5;
        locals.var_rsvde_dn6 = assign15150_e9952_d_n6;
        locals.var_rsvde_dn7 = assign15150_e9952_d_n7;
        locals.var_rsvde_dn8 = assign15150_e9952_d_n8;
        locals.var_rsvde_dn9 = assign15150_e9952_d_n9;
        locals.var_rsvde_dn10 = assign15150_e9952_d_n10;
        locals.var_rsvde_dn11 = assign15150_e9952_d_n11;
        locals.var_rsvde_dn14 = assign15150_e9952_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign15160_e9970, assign15160_e9970_d_n0, assign15160_e9970_d_n2, assign15160_e9970_d_n4, assign15160_e9970_d_n5, assign15160_e9970_d_n6, assign15160_e9970_d_n7, assign15160_e9970_d_n8, assign15160_e9970_d_n9, assign15160_e9970_d_n10, assign15160_e9970_d_n11, assign15160_e9970_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15160_e9963: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15160_e9964: f64 = (locals.var_rsvde - assign15160_e9963);
        let assign15160_e9967: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15160_e9968: f64 = (assign15160_e9964 - assign15160_e9967);
        (assign15160_e9968, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15160_e9970;
        locals.var_tmf1_dn0 = assign15160_e9970_d_n0;
        locals.var_tmf1_dn2 = assign15160_e9970_d_n2;
        locals.var_tmf1_dn4 = assign15160_e9970_d_n4;
        locals.var_tmf1_dn5 = assign15160_e9970_d_n5;
        locals.var_tmf1_dn6 = assign15160_e9970_d_n6;
        locals.var_tmf1_dn7 = assign15160_e9970_d_n7;
        locals.var_tmf1_dn8 = assign15160_e9970_d_n8;
        locals.var_tmf1_dn9 = assign15160_e9970_d_n9;
        locals.var_tmf1_dn10 = assign15160_e9970_d_n10;
        locals.var_tmf1_dn11 = assign15160_e9970_d_n11;
        locals.var_tmf1_dn14 = assign15160_e9970_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign15170_e9988, assign15170_e9988_d_n0, assign15170_e9988_d_n2, assign15170_e9988_d_n4, assign15170_e9988_d_n5, assign15170_e9988_d_n6, assign15170_e9988_d_n7, assign15170_e9988_d_n8, assign15170_e9988_d_n9, assign15170_e9988_d_n10, assign15170_e9988_d_n11, assign15170_e9988_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15170_e9981: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15170_e9982: f64 = (4.0 * assign15170_e9981);
        let assign15170_e9985: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15170_e9986: f64 = (assign15170_e9982 * assign15170_e9985);
        (assign15170_e9986, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15170_e9988;
        locals.var_tmf2_dn0 = assign15170_e9988_d_n0;
        locals.var_tmf2_dn2 = assign15170_e9988_d_n2;
        locals.var_tmf2_dn4 = assign15170_e9988_d_n4;
        locals.var_tmf2_dn5 = assign15170_e9988_d_n5;
        locals.var_tmf2_dn6 = assign15170_e9988_d_n6;
        locals.var_tmf2_dn7 = assign15170_e9988_d_n7;
        locals.var_tmf2_dn8 = assign15170_e9988_d_n8;
        locals.var_tmf2_dn9 = assign15170_e9988_d_n9;
        locals.var_tmf2_dn10 = assign15170_e9988_d_n10;
        locals.var_tmf2_dn11 = assign15170_e9988_d_n11;
        locals.var_tmf2_dn14 = assign15170_e9988_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15180_e10004, assign15180_e10004_d_n0, assign15180_e10004_d_n2, assign15180_e10004_d_n4, assign15180_e10004_d_n5, assign15180_e10004_d_n6, assign15180_e10004_d_n7, assign15180_e10004_d_n8, assign15180_e10004_d_n9, assign15180_e10004_d_n10, assign15180_e10004_d_n11, assign15180_e10004_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign15180_e10002, assign15180_e10002_d_n0, assign15180_e10002_d_n2, assign15180_e10002_d_n4, assign15180_e10002_d_n5, assign15180_e10002_d_n6, assign15180_e10002_d_n7, assign15180_e10002_d_n8, assign15180_e10002_d_n9, assign15180_e10002_d_n10, assign15180_e10002_d_n11, assign15180_e10002_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15180_e10001: f64 = (-locals.var_tmf2);
                (assign15180_e10001, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15180_e10002, assign15180_e10002_d_n0, assign15180_e10002_d_n2, assign15180_e10002_d_n4, assign15180_e10002_d_n5, assign15180_e10002_d_n6, assign15180_e10002_d_n7, assign15180_e10002_d_n8, assign15180_e10002_d_n9, assign15180_e10002_d_n10, assign15180_e10002_d_n11, assign15180_e10002_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15180_e10004;
        locals.var_tmf2_dn0 = assign15180_e10004_d_n0;
        locals.var_tmf2_dn2 = assign15180_e10004_d_n2;
        locals.var_tmf2_dn4 = assign15180_e10004_d_n4;
        locals.var_tmf2_dn5 = assign15180_e10004_d_n5;
        locals.var_tmf2_dn6 = assign15180_e10004_d_n6;
        locals.var_tmf2_dn7 = assign15180_e10004_d_n7;
        locals.var_tmf2_dn8 = assign15180_e10004_d_n8;
        locals.var_tmf2_dn9 = assign15180_e10004_d_n9;
        locals.var_tmf2_dn10 = assign15180_e10004_d_n10;
        locals.var_tmf2_dn11 = assign15180_e10004_d_n11;
        locals.var_tmf2_dn14 = assign15180_e10004_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15190_e10019, assign15190_e10019_d_n0, assign15190_e10019_d_n2, assign15190_e10019_d_n4, assign15190_e10019_d_n5, assign15190_e10019_d_n6, assign15190_e10019_d_n7, assign15190_e10019_d_n8, assign15190_e10019_d_n9, assign15190_e10019_d_n10, assign15190_e10019_d_n11, assign15190_e10019_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15190_e10014: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15190_e10016: f64 = (assign15190_e10014 + locals.var_tmf2);
        let assign15190_e10017: f64 = (assign15190_e10016).sqrt();
        (assign15190_e10017, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15190_e10017)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15190_e10019;
        locals.var_tmf2_dn0 = assign15190_e10019_d_n0;
        locals.var_tmf2_dn2 = assign15190_e10019_d_n2;
        locals.var_tmf2_dn4 = assign15190_e10019_d_n4;
        locals.var_tmf2_dn5 = assign15190_e10019_d_n5;
        locals.var_tmf2_dn6 = assign15190_e10019_d_n6;
        locals.var_tmf2_dn7 = assign15190_e10019_d_n7;
        locals.var_tmf2_dn8 = assign15190_e10019_d_n8;
        locals.var_tmf2_dn9 = assign15190_e10019_d_n9;
        locals.var_tmf2_dn10 = assign15190_e10019_d_n10;
        locals.var_tmf2_dn11 = assign15190_e10019_d_n11;
        locals.var_tmf2_dn14 = assign15190_e10019_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15200_e10035, assign15200_e10035_d_n0, assign15200_e10035_d_n2, assign15200_e10035_d_n4, assign15200_e10035_d_n5, assign15200_e10035_d_n6, assign15200_e10035_d_n7, assign15200_e10035_d_n8, assign15200_e10035_d_n9, assign15200_e10035_d_n10, assign15200_e10035_d_n11, assign15200_e10035_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15200_e10031: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15200_e10032: f64 = (1.0 + assign15200_e10031);
        let assign15200_e10033: f64 = (0.5 * assign15200_e10032);
        (assign15200_e10033, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15200_e10035;
        locals.var_t0_dn0 = assign15200_e10035_d_n0;
        locals.var_t0_dn2 = assign15200_e10035_d_n2;
        locals.var_t0_dn4 = assign15200_e10035_d_n4;
        locals.var_t0_dn5 = assign15200_e10035_d_n5;
        locals.var_t0_dn6 = assign15200_e10035_d_n6;
        locals.var_t0_dn7 = assign15200_e10035_d_n7;
        locals.var_t0_dn8 = assign15200_e10035_d_n8;
        locals.var_t0_dn9 = assign15200_e10035_d_n9;
        locals.var_t0_dn10 = assign15200_e10035_d_n10;
        locals.var_t0_dn11 = assign15200_e10035_d_n11;
        locals.var_t0_dn14 = assign15200_e10035_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15210_e10053, assign15210_e10053_d_n0, assign15210_e10053_d_n2, assign15210_e10053_d_n4, assign15210_e10053_d_n5, assign15210_e10053_d_n6, assign15210_e10053_d_n7, assign15210_e10053_d_n8, assign15210_e10053_d_n9, assign15210_e10053_d_n10, assign15210_e10053_d_n11, assign15210_e10053_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15210_e10045: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15210_e10049: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15210_e10050: f64 = (0.5 * assign15210_e10049);
        let assign15210_e10051: f64 = (assign15210_e10045 + assign15210_e10050);
        (assign15210_e10051, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15210_e10053;
        locals.var_rsvde_dn0 = assign15210_e10053_d_n0;
        locals.var_rsvde_dn2 = assign15210_e10053_d_n2;
        locals.var_rsvde_dn4 = assign15210_e10053_d_n4;
        locals.var_rsvde_dn5 = assign15210_e10053_d_n5;
        locals.var_rsvde_dn6 = assign15210_e10053_d_n6;
        locals.var_rsvde_dn7 = assign15210_e10053_d_n7;
        locals.var_rsvde_dn8 = assign15210_e10053_d_n8;
        locals.var_rsvde_dn9 = assign15210_e10053_d_n9;
        locals.var_rsvde_dn10 = assign15210_e10053_d_n10;
        locals.var_rsvde_dn11 = assign15210_e10053_d_n11;
        locals.var_rsvde_dn14 = assign15210_e10053_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign15220_e10074, assign15220_e10074_d_n0, assign15220_e10074_d_n2, assign15220_e10074_d_n4, assign15220_e10074_d_n5, assign15220_e10074_d_n6, assign15220_e10074_d_n7, assign15220_e10074_d_n8, assign15220_e10074_d_n9, assign15220_e10074_d_n10, assign15220_e10074_d_n11, assign15220_e10074_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15220_e10065: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign15220_e10066: f64 = (locals.var_uc_rdvd + assign15220_e10065);
        let assign15220_e10069: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign15220_e10070: f64 = (assign15220_e10066 + assign15220_e10069);
        let assign15220_e10072: f64 = (assign15220_e10070 * locals.var_t2);
        (assign15220_e10072, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15220_e10074;
        locals.var_rsvde_dn0 = assign15220_e10074_d_n0;
        locals.var_rsvde_dn2 = assign15220_e10074_d_n2;
        locals.var_rsvde_dn4 = assign15220_e10074_d_n4;
        locals.var_rsvde_dn5 = assign15220_e10074_d_n5;
        locals.var_rsvde_dn6 = assign15220_e10074_d_n6;
        locals.var_rsvde_dn7 = assign15220_e10074_d_n7;
        locals.var_rsvde_dn8 = assign15220_e10074_d_n8;
        locals.var_rsvde_dn9 = assign15220_e10074_d_n9;
        locals.var_rsvde_dn10 = assign15220_e10074_d_n10;
        locals.var_rsvde_dn11 = assign15220_e10074_d_n11;
        locals.var_rsvde_dn14 = assign15220_e10074_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign15230_e10093, assign15230_e10093_d_n0, assign15230_e10093_d_n2, assign15230_e10093_d_n4, assign15230_e10093_d_n5, assign15230_e10093_d_n6, assign15230_e10093_d_n7, assign15230_e10093_d_n8, assign15230_e10093_d_n9, assign15230_e10093_d_n10, assign15230_e10093_d_n11, assign15230_e10093_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15230_e10086: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15230_e10087: f64 = (locals.var_rsvde - assign15230_e10086);
        let assign15230_e10090: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15230_e10091: f64 = (assign15230_e10087 - assign15230_e10090);
        (assign15230_e10091, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15230_e10093;
        locals.var_tmf1_dn0 = assign15230_e10093_d_n0;
        locals.var_tmf1_dn2 = assign15230_e10093_d_n2;
        locals.var_tmf1_dn4 = assign15230_e10093_d_n4;
        locals.var_tmf1_dn5 = assign15230_e10093_d_n5;
        locals.var_tmf1_dn6 = assign15230_e10093_d_n6;
        locals.var_tmf1_dn7 = assign15230_e10093_d_n7;
        locals.var_tmf1_dn8 = assign15230_e10093_d_n8;
        locals.var_tmf1_dn9 = assign15230_e10093_d_n9;
        locals.var_tmf1_dn10 = assign15230_e10093_d_n10;
        locals.var_tmf1_dn11 = assign15230_e10093_d_n11;
        locals.var_tmf1_dn14 = assign15230_e10093_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign15240_e10112, assign15240_e10112_d_n0, assign15240_e10112_d_n2, assign15240_e10112_d_n4, assign15240_e10112_d_n5, assign15240_e10112_d_n6, assign15240_e10112_d_n7, assign15240_e10112_d_n8, assign15240_e10112_d_n9, assign15240_e10112_d_n10, assign15240_e10112_d_n11, assign15240_e10112_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15240_e10105: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15240_e10106: f64 = (4.0 * assign15240_e10105);
        let assign15240_e10109: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15240_e10110: f64 = (assign15240_e10106 * assign15240_e10109);
        (assign15240_e10110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15240_e10112;
        locals.var_tmf2_dn0 = assign15240_e10112_d_n0;
        locals.var_tmf2_dn2 = assign15240_e10112_d_n2;
        locals.var_tmf2_dn4 = assign15240_e10112_d_n4;
        locals.var_tmf2_dn5 = assign15240_e10112_d_n5;
        locals.var_tmf2_dn6 = assign15240_e10112_d_n6;
        locals.var_tmf2_dn7 = assign15240_e10112_d_n7;
        locals.var_tmf2_dn8 = assign15240_e10112_d_n8;
        locals.var_tmf2_dn9 = assign15240_e10112_d_n9;
        locals.var_tmf2_dn10 = assign15240_e10112_d_n10;
        locals.var_tmf2_dn11 = assign15240_e10112_d_n11;
        locals.var_tmf2_dn14 = assign15240_e10112_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15250_e10129, assign15250_e10129_d_n0, assign15250_e10129_d_n2, assign15250_e10129_d_n4, assign15250_e10129_d_n5, assign15250_e10129_d_n6, assign15250_e10129_d_n7, assign15250_e10129_d_n8, assign15250_e10129_d_n9, assign15250_e10129_d_n10, assign15250_e10129_d_n11, assign15250_e10129_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let (assign15250_e10127, assign15250_e10127_d_n0, assign15250_e10127_d_n2, assign15250_e10127_d_n4, assign15250_e10127_d_n5, assign15250_e10127_d_n6, assign15250_e10127_d_n7, assign15250_e10127_d_n8, assign15250_e10127_d_n9, assign15250_e10127_d_n10, assign15250_e10127_d_n11, assign15250_e10127_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15250_e10126: f64 = (-locals.var_tmf2);
                (assign15250_e10126, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15250_e10127, assign15250_e10127_d_n0, assign15250_e10127_d_n2, assign15250_e10127_d_n4, assign15250_e10127_d_n5, assign15250_e10127_d_n6, assign15250_e10127_d_n7, assign15250_e10127_d_n8, assign15250_e10127_d_n9, assign15250_e10127_d_n10, assign15250_e10127_d_n11, assign15250_e10127_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15250_e10129;
        locals.var_tmf2_dn0 = assign15250_e10129_d_n0;
        locals.var_tmf2_dn2 = assign15250_e10129_d_n2;
        locals.var_tmf2_dn4 = assign15250_e10129_d_n4;
        locals.var_tmf2_dn5 = assign15250_e10129_d_n5;
        locals.var_tmf2_dn6 = assign15250_e10129_d_n6;
        locals.var_tmf2_dn7 = assign15250_e10129_d_n7;
        locals.var_tmf2_dn8 = assign15250_e10129_d_n8;
        locals.var_tmf2_dn9 = assign15250_e10129_d_n9;
        locals.var_tmf2_dn10 = assign15250_e10129_d_n10;
        locals.var_tmf2_dn11 = assign15250_e10129_d_n11;
        locals.var_tmf2_dn14 = assign15250_e10129_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15260_e10145, assign15260_e10145_d_n0, assign15260_e10145_d_n2, assign15260_e10145_d_n4, assign15260_e10145_d_n5, assign15260_e10145_d_n6, assign15260_e10145_d_n7, assign15260_e10145_d_n8, assign15260_e10145_d_n9, assign15260_e10145_d_n10, assign15260_e10145_d_n11, assign15260_e10145_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15260_e10140: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15260_e10142: f64 = (assign15260_e10140 + locals.var_tmf2);
        let assign15260_e10143: f64 = (assign15260_e10142).sqrt();
        (assign15260_e10143, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15260_e10143)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15260_e10145;
        locals.var_tmf2_dn0 = assign15260_e10145_d_n0;
        locals.var_tmf2_dn2 = assign15260_e10145_d_n2;
        locals.var_tmf2_dn4 = assign15260_e10145_d_n4;
        locals.var_tmf2_dn5 = assign15260_e10145_d_n5;
        locals.var_tmf2_dn6 = assign15260_e10145_d_n6;
        locals.var_tmf2_dn7 = assign15260_e10145_d_n7;
        locals.var_tmf2_dn8 = assign15260_e10145_d_n8;
        locals.var_tmf2_dn9 = assign15260_e10145_d_n9;
        locals.var_tmf2_dn10 = assign15260_e10145_d_n10;
        locals.var_tmf2_dn11 = assign15260_e10145_d_n11;
        locals.var_tmf2_dn14 = assign15260_e10145_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign15270_e10162, assign15270_e10162_d_n0, assign15270_e10162_d_n2, assign15270_e10162_d_n4, assign15270_e10162_d_n5, assign15270_e10162_d_n6, assign15270_e10162_d_n7, assign15270_e10162_d_n8, assign15270_e10162_d_n9, assign15270_e10162_d_n10, assign15270_e10162_d_n11, assign15270_e10162_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15270_e10158: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15270_e10159: f64 = (1.0 + assign15270_e10158);
        let assign15270_e10160: f64 = (0.5 * assign15270_e10159);
        (assign15270_e10160, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15270_e10162;
        locals.var_t0_dn0 = assign15270_e10162_d_n0;
        locals.var_t0_dn2 = assign15270_e10162_d_n2;
        locals.var_t0_dn4 = assign15270_e10162_d_n4;
        locals.var_t0_dn5 = assign15270_e10162_d_n5;
        locals.var_t0_dn6 = assign15270_e10162_d_n6;
        locals.var_t0_dn7 = assign15270_e10162_d_n7;
        locals.var_t0_dn8 = assign15270_e10162_d_n8;
        locals.var_t0_dn9 = assign15270_e10162_d_n9;
        locals.var_t0_dn10 = assign15270_e10162_d_n10;
        locals.var_t0_dn11 = assign15270_e10162_d_n11;
        locals.var_t0_dn14 = assign15270_e10162_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15280_e10181, assign15280_e10181_d_n0, assign15280_e10181_d_n2, assign15280_e10181_d_n4, assign15280_e10181_d_n5, assign15280_e10181_d_n6, assign15280_e10181_d_n7, assign15280_e10181_d_n8, assign15280_e10181_d_n9, assign15280_e10181_d_n10, assign15280_e10181_d_n11, assign15280_e10181_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15280_e10173: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15280_e10177: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15280_e10178: f64 = (0.5 * assign15280_e10177);
        let assign15280_e10179: f64 = (assign15280_e10173 + assign15280_e10178);
        (assign15280_e10179, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15280_e10181;
        locals.var_rsvde_dn0 = assign15280_e10181_d_n0;
        locals.var_rsvde_dn2 = assign15280_e10181_d_n2;
        locals.var_rsvde_dn4 = assign15280_e10181_d_n4;
        locals.var_rsvde_dn5 = assign15280_e10181_d_n5;
        locals.var_rsvde_dn6 = assign15280_e10181_d_n6;
        locals.var_rsvde_dn7 = assign15280_e10181_d_n7;
        locals.var_rsvde_dn8 = assign15280_e10181_d_n8;
        locals.var_rsvde_dn9 = assign15280_e10181_d_n9;
        locals.var_rsvde_dn10 = assign15280_e10181_d_n10;
        locals.var_rsvde_dn11 = assign15280_e10181_d_n11;
        locals.var_rsvde_dn14 = assign15280_e10181_d_n14;
        locals.var_rsvde_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15290_e10190, assign15290_e10190_d_n0, assign15290_e10190_d_n2, assign15290_e10190_d_n4, assign15290_e10190_d_n5, assign15290_e10190_d_n6, assign15290_e10190_d_n7, assign15290_e10190_d_n8, assign15290_e10190_d_n9, assign15290_e10190_d_n10, assign15290_e10190_d_n11, assign15290_e10190_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign15290_e10190;
        locals.var_rdvde_dn0 = assign15290_e10190_d_n0;
        locals.var_rdvde_dn2 = assign15290_e10190_d_n2;
        locals.var_rdvde_dn4 = assign15290_e10190_d_n4;
        locals.var_rdvde_dn5 = assign15290_e10190_d_n5;
        locals.var_rdvde_dn6 = assign15290_e10190_d_n6;
        locals.var_rdvde_dn7 = assign15290_e10190_d_n7;
        locals.var_rdvde_dn8 = assign15290_e10190_d_n8;
        locals.var_rdvde_dn9 = assign15290_e10190_d_n9;
        locals.var_rdvde_dn10 = assign15290_e10190_d_n10;
        locals.var_rdvde_dn11 = assign15290_e10190_d_n11;
        locals.var_rdvde_dn14 = assign15290_e10190_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign15300_e10199, assign15300_e10199_d_n0, assign15300_e10199_d_n2, assign15300_e10199_d_n4, assign15300_e10199_d_n5, assign15300_e10199_d_n6, assign15300_e10199_d_n7, assign15300_e10199_d_n8, assign15300_e10199_d_n9, assign15300_e10199_d_n10, assign15300_e10199_d_n11, assign15300_e10199_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15300_e10199;
        locals.var_rsvde_dn0 = assign15300_e10199_d_n0;
        locals.var_rsvde_dn2 = assign15300_e10199_d_n2;
        locals.var_rsvde_dn4 = assign15300_e10199_d_n4;
        locals.var_rsvde_dn5 = assign15300_e10199_d_n5;
        locals.var_rsvde_dn6 = assign15300_e10199_d_n6;
        locals.var_rsvde_dn7 = assign15300_e10199_d_n7;
        locals.var_rsvde_dn8 = assign15300_e10199_d_n8;
        locals.var_rsvde_dn9 = assign15300_e10199_d_n9;
        locals.var_rsvde_dn10 = assign15300_e10199_d_n10;
        locals.var_rsvde_dn11 = assign15300_e10199_d_n11;
        locals.var_rsvde_dn14 = assign15300_e10199_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign15310_e10206, assign15310_e10206_d_n0, assign15310_e10206_d_n2, assign15310_e10206_d_n4, assign15310_e10206_d_n5, assign15310_e10206_d_n6, assign15310_e10206_d_n7, assign15310_e10206_d_n8, assign15310_e10206_d_n9, assign15310_e10206_d_n10, assign15310_e10206_d_n11, assign15310_e10206_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15310_e10203: f64 = (locals.var_beta_inv).sqrt();
        let assign15310_e10204: f64 = (locals.var_costi00 * assign15310_e10203);
        (assign15310_e10204, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn11 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn14 / (2.0 * assign15310_e10203))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn14,)
    }
};
        locals.var_costi0 = assign15310_e10206;
        locals.var_costi0_dn0 = assign15310_e10206_d_n0;
        locals.var_costi0_dn2 = assign15310_e10206_d_n2;
        locals.var_costi0_dn4 = assign15310_e10206_d_n4;
        locals.var_costi0_dn5 = assign15310_e10206_d_n5;
        locals.var_costi0_dn6 = assign15310_e10206_d_n6;
        locals.var_costi0_dn7 = assign15310_e10206_d_n7;
        locals.var_costi0_dn8 = assign15310_e10206_d_n8;
        locals.var_costi0_dn9 = assign15310_e10206_d_n9;
        locals.var_costi0_dn10 = assign15310_e10206_d_n10;
        locals.var_costi0_dn11 = assign15310_e10206_d_n11;
        locals.var_costi0_dn14 = assign15310_e10206_d_n14;
        locals.var_costi0_rv = 0.0;

        let (assign15320_e10212, assign15320_e10212_d_n0, assign15320_e10212_d_n2, assign15320_e10212_d_n4, assign15320_e10212_d_n5, assign15320_e10212_d_n6, assign15320_e10212_d_n7, assign15320_e10212_d_n8, assign15320_e10212_d_n9, assign15320_e10212_d_n10, assign15320_e10212_d_n11, assign15320_e10212_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15320_e10210: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign15320_e10210, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)), ((locals.var_costi0_dn14 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn14)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn11, locals.var_costi0_p2_dn14,)
    }
};
        locals.var_costi0_p2 = assign15320_e10212;
        locals.var_costi0_p2_dn0 = assign15320_e10212_d_n0;
        locals.var_costi0_p2_dn2 = assign15320_e10212_d_n2;
        locals.var_costi0_p2_dn4 = assign15320_e10212_d_n4;
        locals.var_costi0_p2_dn5 = assign15320_e10212_d_n5;
        locals.var_costi0_p2_dn6 = assign15320_e10212_d_n6;
        locals.var_costi0_p2_dn7 = assign15320_e10212_d_n7;
        locals.var_costi0_p2_dn8 = assign15320_e10212_d_n8;
        locals.var_costi0_p2_dn9 = assign15320_e10212_d_n9;
        locals.var_costi0_p2_dn10 = assign15320_e10212_d_n10;
        locals.var_costi0_p2_dn11 = assign15320_e10212_d_n11;
        locals.var_costi0_p2_dn14 = assign15320_e10212_d_n14;
        locals.var_costi0_p2_rv = 0.0;

        let (assign15330_e10220, assign15330_e10220_d_n0, assign15330_e10220_d_n2, assign15330_e10220_d_n4, assign15330_e10220_d_n5, assign15330_e10220_d_n6, assign15330_e10220_d_n7, assign15330_e10220_d_n8, assign15330_e10220_d_n9, assign15330_e10220_d_n10, assign15330_e10220_d_n11, assign15330_e10220_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15330_e10216: f64 = (locals.var_nin * locals.var_nin);
        let assign15330_e10218: f64 = (assign15330_e10216 * locals.var_nsti_p2);
        (assign15330_e10218, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2), (((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn14,)
    }
};
        locals.var_costi1 = assign15330_e10220;
        locals.var_costi1_dn0 = assign15330_e10220_d_n0;
        locals.var_costi1_dn2 = assign15330_e10220_d_n2;
        locals.var_costi1_dn4 = assign15330_e10220_d_n4;
        locals.var_costi1_dn5 = assign15330_e10220_d_n5;
        locals.var_costi1_dn6 = assign15330_e10220_d_n6;
        locals.var_costi1_dn7 = assign15330_e10220_d_n7;
        locals.var_costi1_dn8 = assign15330_e10220_d_n8;
        locals.var_costi1_dn9 = assign15330_e10220_d_n9;
        locals.var_costi1_dn10 = assign15330_e10220_d_n10;
        locals.var_costi1_dn11 = assign15330_e10220_d_n11;
        locals.var_costi1_dn14 = assign15330_e10220_d_n14;
        locals.var_costi1_rv = 0.0;

        let (assign15340_e10228, assign15340_e10228_d_n0, assign15340_e10228_d_n2, assign15340_e10228_d_n4, assign15340_e10228_d_n5, assign15340_e10228_d_n6, assign15340_e10228_d_n7, assign15340_e10228_d_n8, assign15340_e10228_d_n9, assign15340_e10228_d_n10, assign15340_e10228_d_n11, assign15340_e10228_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15340_e10225: f64 = (p.p448 * locals.var_tdiff);
        let assign15340_e10226: f64 = (p.p447 + assign15340_e10225);
        (assign15340_e10226, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn11), (p.p448 * locals.var_tdiff_dn14),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    }
};
        locals.var_hbdceff = assign15340_e10228;
        locals.var_hbdceff_dn0 = assign15340_e10228_d_n0;
        locals.var_hbdceff_dn2 = assign15340_e10228_d_n2;
        locals.var_hbdceff_dn4 = assign15340_e10228_d_n4;
        locals.var_hbdceff_dn5 = assign15340_e10228_d_n5;
        locals.var_hbdceff_dn6 = assign15340_e10228_d_n6;
        locals.var_hbdceff_dn7 = assign15340_e10228_d_n7;
        locals.var_hbdceff_dn8 = assign15340_e10228_d_n8;
        locals.var_hbdceff_dn9 = assign15340_e10228_d_n9;
        locals.var_hbdceff_dn10 = assign15340_e10228_d_n10;
        locals.var_hbdceff_dn11 = assign15340_e10228_d_n11;
        locals.var_hbdceff_dn14 = assign15340_e10228_d_n14;
        locals.var_hbdceff_rv = 0.0;

        let (assign15350_e10232,) = {
    if (locals.var_guard291 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15350_e10232;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15380_e10245: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign15380_e10245;
        locals.var_guard326_rv = 0.0;

        let (assign15390_e10251,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15390_e10251;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15400_e10254: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard327 = assign15400_e10254;
        locals.var_guard327_rv = 0.0;

        let (assign15410_e10260,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard327 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15410_e10260;
        locals.var_uc_subtmp_rv = 0.0;

        let assign15420_e10263: f64 = if locals.var_uc_cordrift > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign15420_e10263;
        locals.var_guard328_rv = 0.0;

        let (assign15430_e10276, assign15430_e10276_d_n0, assign15430_e10276_d_n2, assign15430_e10276_d_n4, assign15430_e10276_d_n5, assign15430_e10276_d_n6, assign15430_e10276_d_n7, assign15430_e10276_d_n8, assign15430_e10276_d_n9, assign15430_e10276_d_n10, assign15430_e10276_d_n11, assign15430_e10276_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let (assign15430_e10274, assign15430_e10274_d_n0, assign15430_e10274_d_n2, assign15430_e10274_d_n4, assign15430_e10274_d_n5, assign15430_e10274_d_n6, assign15430_e10274_d_n7, assign15430_e10274_d_n8, assign15430_e10274_d_n9, assign15430_e10274_d_n10, assign15430_e10274_d_n11, assign15430_e10274_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15430_e10273: f64 = (locals.var_tratio).powf(p.p416);
                (assign15430_e10273, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn11)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn14)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign15430_e10274, assign15430_e10274_d_n0, assign15430_e10274_d_n2, assign15430_e10274_d_n4, assign15430_e10274_d_n5, assign15430_e10274_d_n6, assign15430_e10274_d_n7, assign15430_e10274_d_n8, assign15430_e10274_d_n9, assign15430_e10274_d_n10, assign15430_e10274_d_n11, assign15430_e10274_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15430_e10276;
        locals.var_t1_dn0 = assign15430_e10276_d_n0;
        locals.var_t1_dn2 = assign15430_e10276_d_n2;
        locals.var_t1_dn4 = assign15430_e10276_d_n4;
        locals.var_t1_dn5 = assign15430_e10276_d_n5;
        locals.var_t1_dn6 = assign15430_e10276_d_n6;
        locals.var_t1_dn7 = assign15430_e10276_d_n7;
        locals.var_t1_dn8 = assign15430_e10276_d_n8;
        locals.var_t1_dn9 = assign15430_e10276_d_n9;
        locals.var_t1_dn10 = assign15430_e10276_d_n10;
        locals.var_t1_dn11 = assign15430_e10276_d_n11;
        locals.var_t1_dn14 = assign15430_e10276_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15440_e10284, assign15440_e10284_d_n0, assign15440_e10284_d_n2, assign15440_e10284_d_n4, assign15440_e10284_d_n5, assign15440_e10284_d_n6, assign15440_e10284_d_n7, assign15440_e10284_d_n8, assign15440_e10284_d_n9, assign15440_e10284_d_n10, assign15440_e10284_d_n11, assign15440_e10284_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15440_e10282: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign15440_e10282, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn11, locals.var_rrdrmues_dn14,)
    }
};
        locals.var_rrdrmues = assign15440_e10284;
        locals.var_rrdrmues_dn0 = assign15440_e10284_d_n0;
        locals.var_rrdrmues_dn2 = assign15440_e10284_d_n2;
        locals.var_rrdrmues_dn4 = assign15440_e10284_d_n4;
        locals.var_rrdrmues_dn5 = assign15440_e10284_d_n5;
        locals.var_rrdrmues_dn6 = assign15440_e10284_d_n6;
        locals.var_rrdrmues_dn7 = assign15440_e10284_d_n7;
        locals.var_rrdrmues_dn8 = assign15440_e10284_d_n8;
        locals.var_rrdrmues_dn9 = assign15440_e10284_d_n9;
        locals.var_rrdrmues_dn10 = assign15440_e10284_d_n10;
        locals.var_rrdrmues_dn11 = assign15440_e10284_d_n11;
        locals.var_rrdrmues_dn14 = assign15440_e10284_d_n14;
        locals.var_rrdrmues_rv = 0.0;

        let (assign15450_e10306, assign15450_e10306_d_n0, assign15450_e10306_d_n2, assign15450_e10306_d_n4, assign15450_e10306_d_n5, assign15450_e10306_d_n6, assign15450_e10306_d_n7, assign15450_e10306_d_n8, assign15450_e10306_d_n9, assign15450_e10306_d_n10, assign15450_e10306_d_n11, assign15450_e10306_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15450_e10291: f64 = (0.4 * locals.var_tratio);
        let assign15450_e10292: f64 = (1.8 + assign15450_e10291);
        let assign15450_e10295: f64 = (0.1 * locals.var_tratio);
        let assign15450_e10297: f64 = (assign15450_e10295 * locals.var_tratio);
        let assign15450_e10298: f64 = (assign15450_e10292 + assign15450_e10297);
        let assign15450_e10302: f64 = (1.0 - locals.var_tratio);
        let assign15450_e10303: f64 = (p.p418 * assign15450_e10302);
        let assign15450_e10304: f64 = (assign15450_e10298 - assign15450_e10303);
        (assign15450_e10304, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn11))) - (p.p418 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn14))) - (p.p418 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15450_e10306;
        locals.var_t0_dn0 = assign15450_e10306_d_n0;
        locals.var_t0_dn2 = assign15450_e10306_d_n2;
        locals.var_t0_dn4 = assign15450_e10306_d_n4;
        locals.var_t0_dn5 = assign15450_e10306_d_n5;
        locals.var_t0_dn6 = assign15450_e10306_d_n6;
        locals.var_t0_dn7 = assign15450_e10306_d_n7;
        locals.var_t0_dn8 = assign15450_e10306_d_n8;
        locals.var_t0_dn9 = assign15450_e10306_d_n9;
        locals.var_t0_dn10 = assign15450_e10306_d_n10;
        locals.var_t0_dn11 = assign15450_e10306_d_n11;
        locals.var_t0_dn14 = assign15450_e10306_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15460_e10314, assign15460_e10314_d_n0, assign15460_e10314_d_n2, assign15460_e10314_d_n4, assign15460_e10314_d_n5, assign15460_e10314_d_n6, assign15460_e10314_d_n7, assign15460_e10314_d_n8, assign15460_e10314_d_n9, assign15460_e10314_d_n10, assign15460_e10314_d_n11, assign15460_e10314_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15460_e10312: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign15460_e10312, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn11, locals.var_rrdrvmaxs_dn14,)
    }
};
        locals.var_rrdrvmaxs = assign15460_e10314;
        locals.var_rrdrvmaxs_dn0 = assign15460_e10314_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign15460_e10314_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign15460_e10314_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign15460_e10314_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign15460_e10314_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign15460_e10314_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign15460_e10314_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign15460_e10314_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign15460_e10314_d_n10;
        locals.var_rrdrvmaxs_dn11 = assign15460_e10314_d_n11;
        locals.var_rrdrvmaxs_dn14 = assign15460_e10314_d_n14;
        locals.var_rrdrvmaxs_rv = 0.0;

        let (assign15470_e10326, assign15470_e10326_d_n0, assign15470_e10326_d_n2, assign15470_e10326_d_n4, assign15470_e10326_d_n5, assign15470_e10326_d_n6, assign15470_e10326_d_n7, assign15470_e10326_d_n8, assign15470_e10326_d_n9, assign15470_e10326_d_n10, assign15470_e10326_d_n11, assign15470_e10326_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15470_e10322: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15470_e10323: f64 = (p.p439 * assign15470_e10322);
        let assign15470_e10324: f64 = (locals.var_uc_rdrbb_s + assign15470_e10323);
        (assign15470_e10324, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn11 + (p.p439 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_s_dn14 + (p.p439 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn11, locals.var_uc_rdrbb_s_dn14,)
    }
};
        locals.var_uc_rdrbb_s = assign15470_e10326;
        locals.var_uc_rdrbb_s_dn0 = assign15470_e10326_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign15470_e10326_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign15470_e10326_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign15470_e10326_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign15470_e10326_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign15470_e10326_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign15470_e10326_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign15470_e10326_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign15470_e10326_d_n10;
        locals.var_uc_rdrbb_s_dn11 = assign15470_e10326_d_n11;
        locals.var_uc_rdrbb_s_dn14 = assign15470_e10326_d_n14;
        locals.var_uc_rdrbb_s_rv = 0.0;

        let (assign15480_e10339, assign15480_e10339_d_n0, assign15480_e10339_d_n2, assign15480_e10339_d_n4, assign15480_e10339_d_n5, assign15480_e10339_d_n6, assign15480_e10339_d_n7, assign15480_e10339_d_n8, assign15480_e10339_d_n9, assign15480_e10339_d_n10, assign15480_e10339_d_n11, assign15480_e10339_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let (assign15480_e10337, assign15480_e10337_d_n0, assign15480_e10337_d_n2, assign15480_e10337_d_n4, assign15480_e10337_d_n5, assign15480_e10337_d_n6, assign15480_e10337_d_n7, assign15480_e10337_d_n8, assign15480_e10337_d_n9, assign15480_e10337_d_n10, assign15480_e10337_d_n11, assign15480_e10337_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15480_e10336: f64 = (locals.var_tratio).powf(p.p415);
                (assign15480_e10336, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn11)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn14)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign15480_e10337, assign15480_e10337_d_n0, assign15480_e10337_d_n2, assign15480_e10337_d_n4, assign15480_e10337_d_n5, assign15480_e10337_d_n6, assign15480_e10337_d_n7, assign15480_e10337_d_n8, assign15480_e10337_d_n9, assign15480_e10337_d_n10, assign15480_e10337_d_n11, assign15480_e10337_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15480_e10339;
        locals.var_t1_dn0 = assign15480_e10339_d_n0;
        locals.var_t1_dn2 = assign15480_e10339_d_n2;
        locals.var_t1_dn4 = assign15480_e10339_d_n4;
        locals.var_t1_dn5 = assign15480_e10339_d_n5;
        locals.var_t1_dn6 = assign15480_e10339_d_n6;
        locals.var_t1_dn7 = assign15480_e10339_d_n7;
        locals.var_t1_dn8 = assign15480_e10339_d_n8;
        locals.var_t1_dn9 = assign15480_e10339_d_n9;
        locals.var_t1_dn10 = assign15480_e10339_d_n10;
        locals.var_t1_dn11 = assign15480_e10339_d_n11;
        locals.var_t1_dn14 = assign15480_e10339_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15490_e10347, assign15490_e10347_d_n0, assign15490_e10347_d_n2, assign15490_e10347_d_n4, assign15490_e10347_d_n5, assign15490_e10347_d_n6, assign15490_e10347_d_n7, assign15490_e10347_d_n8, assign15490_e10347_d_n9, assign15490_e10347_d_n10, assign15490_e10347_d_n11, assign15490_e10347_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15490_e10345: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign15490_e10345, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn11, locals.var_rrdrmue_dn14,)
    }
};
        locals.var_rrdrmue = assign15490_e10347;
        locals.var_rrdrmue_dn0 = assign15490_e10347_d_n0;
        locals.var_rrdrmue_dn2 = assign15490_e10347_d_n2;
        locals.var_rrdrmue_dn4 = assign15490_e10347_d_n4;
        locals.var_rrdrmue_dn5 = assign15490_e10347_d_n5;
        locals.var_rrdrmue_dn6 = assign15490_e10347_d_n6;
        locals.var_rrdrmue_dn7 = assign15490_e10347_d_n7;
        locals.var_rrdrmue_dn8 = assign15490_e10347_d_n8;
        locals.var_rrdrmue_dn9 = assign15490_e10347_d_n9;
        locals.var_rrdrmue_dn10 = assign15490_e10347_d_n10;
        locals.var_rrdrmue_dn11 = assign15490_e10347_d_n11;
        locals.var_rrdrmue_dn14 = assign15490_e10347_d_n14;
        locals.var_rrdrmue_rv = 0.0;

        let (assign15500_e10369, assign15500_e10369_d_n0, assign15500_e10369_d_n2, assign15500_e10369_d_n4, assign15500_e10369_d_n5, assign15500_e10369_d_n6, assign15500_e10369_d_n7, assign15500_e10369_d_n8, assign15500_e10369_d_n9, assign15500_e10369_d_n10, assign15500_e10369_d_n11, assign15500_e10369_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15500_e10354: f64 = (0.4 * locals.var_tratio);
        let assign15500_e10355: f64 = (1.8 + assign15500_e10354);
        let assign15500_e10358: f64 = (0.1 * locals.var_tratio);
        let assign15500_e10360: f64 = (assign15500_e10358 * locals.var_tratio);
        let assign15500_e10361: f64 = (assign15500_e10355 + assign15500_e10360);
        let assign15500_e10365: f64 = (1.0 - locals.var_tratio);
        let assign15500_e10366: f64 = (p.p417 * assign15500_e10365);
        let assign15500_e10367: f64 = (assign15500_e10361 - assign15500_e10366);
        (assign15500_e10367, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn11))) - (p.p417 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn14))) - (p.p417 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15500_e10369;
        locals.var_t0_dn0 = assign15500_e10369_d_n0;
        locals.var_t0_dn2 = assign15500_e10369_d_n2;
        locals.var_t0_dn4 = assign15500_e10369_d_n4;
        locals.var_t0_dn5 = assign15500_e10369_d_n5;
        locals.var_t0_dn6 = assign15500_e10369_d_n6;
        locals.var_t0_dn7 = assign15500_e10369_d_n7;
        locals.var_t0_dn8 = assign15500_e10369_d_n8;
        locals.var_t0_dn9 = assign15500_e10369_d_n9;
        locals.var_t0_dn10 = assign15500_e10369_d_n10;
        locals.var_t0_dn11 = assign15500_e10369_d_n11;
        locals.var_t0_dn14 = assign15500_e10369_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15510_e10377, assign15510_e10377_d_n0, assign15510_e10377_d_n2, assign15510_e10377_d_n4, assign15510_e10377_d_n5, assign15510_e10377_d_n6, assign15510_e10377_d_n7, assign15510_e10377_d_n8, assign15510_e10377_d_n9, assign15510_e10377_d_n10, assign15510_e10377_d_n11, assign15510_e10377_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15510_e10375: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign15510_e10375, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn11, locals.var_rrdrvmax_dn14,)
    }
};
        locals.var_rrdrvmax = assign15510_e10377;
        locals.var_rrdrvmax_dn0 = assign15510_e10377_d_n0;
        locals.var_rrdrvmax_dn2 = assign15510_e10377_d_n2;
        locals.var_rrdrvmax_dn4 = assign15510_e10377_d_n4;
        locals.var_rrdrvmax_dn5 = assign15510_e10377_d_n5;
        locals.var_rrdrvmax_dn6 = assign15510_e10377_d_n6;
        locals.var_rrdrvmax_dn7 = assign15510_e10377_d_n7;
        locals.var_rrdrvmax_dn8 = assign15510_e10377_d_n8;
        locals.var_rrdrvmax_dn9 = assign15510_e10377_d_n9;
        locals.var_rrdrvmax_dn10 = assign15510_e10377_d_n10;
        locals.var_rrdrvmax_dn11 = assign15510_e10377_d_n11;
        locals.var_rrdrvmax_dn14 = assign15510_e10377_d_n14;
        locals.var_rrdrvmax_rv = 0.0;

        let (assign15520_e10389, assign15520_e10389_d_n0, assign15520_e10389_d_n2, assign15520_e10389_d_n4, assign15520_e10389_d_n5, assign15520_e10389_d_n6, assign15520_e10389_d_n7, assign15520_e10389_d_n8, assign15520_e10389_d_n9, assign15520_e10389_d_n10, assign15520_e10389_d_n11, assign15520_e10389_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15520_e10385: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15520_e10386: f64 = (p.p438 * assign15520_e10385);
        let assign15520_e10387: f64 = (locals.var_uc_rdrbb + assign15520_e10386);
        (assign15520_e10387, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn11 + (p.p438 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_dn14 + (p.p438 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign15520_e10389;
        locals.var_uc_rdrbb_dn0 = assign15520_e10389_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15520_e10389_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15520_e10389_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15520_e10389_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15520_e10389_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15520_e10389_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15520_e10389_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15520_e10389_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15520_e10389_d_n10;
        locals.var_uc_rdrbb_dn11 = assign15520_e10389_d_n11;
        locals.var_uc_rdrbb_dn14 = assign15520_e10389_d_n14;
        locals.var_uc_rdrbb_rv = 0.0;

        let assign15540_e10397: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign15540_e10397;
        locals.var_guard330_rv = 0.0;

        let (assign15550_e10405, assign15550_e10405_d_n0, assign15550_e10405_d_n2, assign15550_e10405_d_n4, assign15550_e10405_d_n5, assign15550_e10405_d_n6, assign15550_e10405_d_n7, assign15550_e10405_d_n8, assign15550_e10405_d_n9, assign15550_e10405_d_n10, assign15550_e10405_d_n11, assign15550_e10405_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign15550_e10405;
        locals.var_uc_rdrbb_dn0 = assign15550_e10405_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15550_e10405_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15550_e10405_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15550_e10405_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15550_e10405_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15550_e10405_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15550_e10405_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15550_e10405_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15550_e10405_d_n10;
        locals.var_uc_rdrbb_dn11 = assign15550_e10405_d_n11;
        locals.var_uc_rdrbb_dn14 = assign15550_e10405_d_n14;
        locals.var_uc_rdrbb_rv = 0.0;

        let (assign15560_e10411, assign15560_e10411_d_n0, assign15560_e10411_d_n2, assign15560_e10411_d_n4, assign15560_e10411_d_n5, assign15560_e10411_d_n6, assign15560_e10411_d_n7, assign15560_e10411_d_n8, assign15560_e10411_d_n9, assign15560_e10411_d_n10, assign15560_e10411_d_n11, assign15560_e10411_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15560_e10409: f64 = (locals.var_tratio * locals.var_tratio);
        (assign15560_e10409, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15560_e10411;
        locals.var_t0_dn0 = assign15560_e10411_d_n0;
        locals.var_t0_dn2 = assign15560_e10411_d_n2;
        locals.var_t0_dn4 = assign15560_e10411_d_n4;
        locals.var_t0_dn5 = assign15560_e10411_d_n5;
        locals.var_t0_dn6 = assign15560_e10411_d_n6;
        locals.var_t0_dn7 = assign15560_e10411_d_n7;
        locals.var_t0_dn8 = assign15560_e10411_d_n8;
        locals.var_t0_dn9 = assign15560_e10411_d_n9;
        locals.var_t0_dn10 = assign15560_e10411_d_n10;
        locals.var_t0_dn11 = assign15560_e10411_d_n11;
        locals.var_t0_dn14 = assign15560_e10411_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15570_e10430, assign15570_e10430_d_n0, assign15570_e10430_d_n2, assign15570_e10430_d_n4, assign15570_e10430_d_n5, assign15570_e10430_d_n6, assign15570_e10430_d_n7, assign15570_e10430_d_n8, assign15570_e10430_d_n9, assign15570_e10430_d_n10, assign15570_e10430_d_n11, assign15570_e10430_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15570_e10416: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15570_e10419: f64 = (locals.var_eg * locals.var_beta);
        let assign15570_e10420: f64 = (assign15570_e10416 - assign15570_e10419);
        let assign15570_e10423: f64 = (p.p499 * locals.var_log_tratio);
        let assign15570_e10424: f64 = (assign15570_e10420 + assign15570_e10423);
        let assign15570_e10426: f64 = (assign15570_e10424 / locals.var_uc_njd);
        let assign15570_e10427: f64 = (assign15570_e10426).exp();
        let assign15570_e10428: f64 = (locals.var_uc_js0d * assign15570_e10427);
        (assign15570_e10428, (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign15570_e10430;
        locals.var_js_dn0 = assign15570_e10430_d_n0;
        locals.var_js_dn2 = assign15570_e10430_d_n2;
        locals.var_js_dn4 = assign15570_e10430_d_n4;
        locals.var_js_dn5 = assign15570_e10430_d_n5;
        locals.var_js_dn6 = assign15570_e10430_d_n6;
        locals.var_js_dn7 = assign15570_e10430_d_n7;
        locals.var_js_dn8 = assign15570_e10430_d_n8;
        locals.var_js_dn9 = assign15570_e10430_d_n9;
        locals.var_js_dn10 = assign15570_e10430_d_n10;
        locals.var_js_dn11 = assign15570_e10430_d_n11;
        locals.var_js_dn14 = assign15570_e10430_d_n14;
        locals.var_js_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15580_e10449, assign15580_e10449_d_n0, assign15580_e10449_d_n2, assign15580_e10449_d_n4, assign15580_e10449_d_n5, assign15580_e10449_d_n6, assign15580_e10449_d_n7, assign15580_e10449_d_n8, assign15580_e10449_d_n9, assign15580_e10449_d_n10, assign15580_e10449_d_n11, assign15580_e10449_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15580_e10435: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15580_e10438: f64 = (locals.var_eg * locals.var_beta);
        let assign15580_e10439: f64 = (assign15580_e10435 - assign15580_e10438);
        let assign15580_e10442: f64 = (p.p499 * locals.var_log_tratio);
        let assign15580_e10443: f64 = (assign15580_e10439 + assign15580_e10442);
        let assign15580_e10445: f64 = (assign15580_e10443 / p.p497);
        let assign15580_e10446: f64 = (assign15580_e10445).exp();
        let assign15580_e10447: f64 = (locals.var_uc_js0swd * assign15580_e10446);
        (assign15580_e10447, (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign15580_e10449;
        locals.var_jssw_dn0 = assign15580_e10449_d_n0;
        locals.var_jssw_dn2 = assign15580_e10449_d_n2;
        locals.var_jssw_dn4 = assign15580_e10449_d_n4;
        locals.var_jssw_dn5 = assign15580_e10449_d_n5;
        locals.var_jssw_dn6 = assign15580_e10449_d_n6;
        locals.var_jssw_dn7 = assign15580_e10449_d_n7;
        locals.var_jssw_dn8 = assign15580_e10449_d_n8;
        locals.var_jssw_dn9 = assign15580_e10449_d_n9;
        locals.var_jssw_dn10 = assign15580_e10449_d_n10;
        locals.var_jssw_dn11 = assign15580_e10449_d_n11;
        locals.var_jssw_dn14 = assign15580_e10449_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign15590_e10468, assign15590_e10468_d_n0, assign15590_e10468_d_n2, assign15590_e10468_d_n4, assign15590_e10468_d_n5, assign15590_e10468_d_n6, assign15590_e10468_d_n7, assign15590_e10468_d_n8, assign15590_e10468_d_n9, assign15590_e10468_d_n10, assign15590_e10468_d_n11, assign15590_e10468_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15590_e10454: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15590_e10457: f64 = (locals.var_eg * locals.var_beta);
        let assign15590_e10458: f64 = (assign15590_e10454 - assign15590_e10457);
        let assign15590_e10461: f64 = (p.p499 * locals.var_log_tratio);
        let assign15590_e10462: f64 = (assign15590_e10458 + assign15590_e10461);
        let assign15590_e10464: f64 = (assign15590_e10462 / p.p498);
        let assign15590_e10465: f64 = (assign15590_e10464).exp();
        let assign15590_e10466: f64 = (p.p495 * assign15590_e10465);
        (assign15590_e10466, (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign15590_e10468;
        locals.var_jsswg_dn0 = assign15590_e10468_d_n0;
        locals.var_jsswg_dn2 = assign15590_e10468_d_n2;
        locals.var_jsswg_dn4 = assign15590_e10468_d_n4;
        locals.var_jsswg_dn5 = assign15590_e10468_d_n5;
        locals.var_jsswg_dn6 = assign15590_e10468_d_n6;
        locals.var_jsswg_dn7 = assign15590_e10468_d_n7;
        locals.var_jsswg_dn8 = assign15590_e10468_d_n8;
        locals.var_jsswg_dn9 = assign15590_e10468_d_n9;
        locals.var_jsswg_dn10 = assign15590_e10468_d_n10;
        locals.var_jsswg_dn11 = assign15590_e10468_d_n11;
        locals.var_jsswg_dn14 = assign15590_e10468_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign15600_e10487, assign15600_e10487_d_n0, assign15600_e10487_d_n2, assign15600_e10487_d_n4, assign15600_e10487_d_n5, assign15600_e10487_d_n6, assign15600_e10487_d_n7, assign15600_e10487_d_n8, assign15600_e10487_d_n9, assign15600_e10487_d_n10, assign15600_e10487_d_n11, assign15600_e10487_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15600_e10473: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15600_e10476: f64 = (locals.var_eg * locals.var_beta);
        let assign15600_e10477: f64 = (assign15600_e10473 - assign15600_e10476);
        let assign15600_e10480: f64 = (p.p509 * locals.var_log_tratio);
        let assign15600_e10481: f64 = (assign15600_e10477 + assign15600_e10480);
        let assign15600_e10483: f64 = (assign15600_e10481 / locals.var_uc_njd);
        let assign15600_e10484: f64 = (assign15600_e10483).exp();
        let assign15600_e10485: f64 = (locals.var_uc_js0d * assign15600_e10484);
        (assign15600_e10485, (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign15600_e10487;
        locals.var_js2_dn0 = assign15600_e10487_d_n0;
        locals.var_js2_dn2 = assign15600_e10487_d_n2;
        locals.var_js2_dn4 = assign15600_e10487_d_n4;
        locals.var_js2_dn5 = assign15600_e10487_d_n5;
        locals.var_js2_dn6 = assign15600_e10487_d_n6;
        locals.var_js2_dn7 = assign15600_e10487_d_n7;
        locals.var_js2_dn8 = assign15600_e10487_d_n8;
        locals.var_js2_dn9 = assign15600_e10487_d_n9;
        locals.var_js2_dn10 = assign15600_e10487_d_n10;
        locals.var_js2_dn11 = assign15600_e10487_d_n11;
        locals.var_js2_dn14 = assign15600_e10487_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign15610_e10506, assign15610_e10506_d_n0, assign15610_e10506_d_n2, assign15610_e10506_d_n4, assign15610_e10506_d_n5, assign15610_e10506_d_n6, assign15610_e10506_d_n7, assign15610_e10506_d_n8, assign15610_e10506_d_n9, assign15610_e10506_d_n10, assign15610_e10506_d_n11, assign15610_e10506_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15610_e10492: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15610_e10495: f64 = (locals.var_eg * locals.var_beta);
        let assign15610_e10496: f64 = (assign15610_e10492 - assign15610_e10495);
        let assign15610_e10499: f64 = (p.p509 * locals.var_log_tratio);
        let assign15610_e10500: f64 = (assign15610_e10496 + assign15610_e10499);
        let assign15610_e10502: f64 = (assign15610_e10500 / p.p497);
        let assign15610_e10503: f64 = (assign15610_e10502).exp();
        let assign15610_e10504: f64 = (locals.var_uc_js0swd * assign15610_e10503);
        (assign15610_e10504, (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign15610_e10503 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign15610_e10506;
        locals.var_jssw2_dn0 = assign15610_e10506_d_n0;
        locals.var_jssw2_dn2 = assign15610_e10506_d_n2;
        locals.var_jssw2_dn4 = assign15610_e10506_d_n4;
        locals.var_jssw2_dn5 = assign15610_e10506_d_n5;
        locals.var_jssw2_dn6 = assign15610_e10506_d_n6;
        locals.var_jssw2_dn7 = assign15610_e10506_d_n7;
        locals.var_jssw2_dn8 = assign15610_e10506_d_n8;
        locals.var_jssw2_dn9 = assign15610_e10506_d_n9;
        locals.var_jssw2_dn10 = assign15610_e10506_d_n10;
        locals.var_jssw2_dn11 = assign15610_e10506_d_n11;
        locals.var_jssw2_dn14 = assign15610_e10506_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign15620_e10525, assign15620_e10525_d_n0, assign15620_e10525_d_n2, assign15620_e10525_d_n4, assign15620_e10525_d_n5, assign15620_e10525_d_n6, assign15620_e10525_d_n7, assign15620_e10525_d_n8, assign15620_e10525_d_n9, assign15620_e10525_d_n10, assign15620_e10525_d_n11, assign15620_e10525_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15620_e10511: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15620_e10514: f64 = (locals.var_eg * locals.var_beta);
        let assign15620_e10515: f64 = (assign15620_e10511 - assign15620_e10514);
        let assign15620_e10518: f64 = (p.p509 * locals.var_log_tratio);
        let assign15620_e10519: f64 = (assign15620_e10515 + assign15620_e10518);
        let assign15620_e10521: f64 = (assign15620_e10519 / p.p498);
        let assign15620_e10522: f64 = (assign15620_e10521).exp();
        let assign15620_e10523: f64 = (p.p495 * assign15620_e10522);
        (assign15620_e10523, (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign15620_e10522 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign15620_e10525;
        locals.var_jsswg2_dn0 = assign15620_e10525_d_n0;
        locals.var_jsswg2_dn2 = assign15620_e10525_d_n2;
        locals.var_jsswg2_dn4 = assign15620_e10525_d_n4;
        locals.var_jsswg2_dn5 = assign15620_e10525_d_n5;
        locals.var_jsswg2_dn6 = assign15620_e10525_d_n6;
        locals.var_jsswg2_dn7 = assign15620_e10525_d_n7;
        locals.var_jsswg2_dn8 = assign15620_e10525_d_n8;
        locals.var_jsswg2_dn9 = assign15620_e10525_d_n9;
        locals.var_jsswg2_dn10 = assign15620_e10525_d_n10;
        locals.var_jsswg2_dn11 = assign15620_e10525_d_n11;
        locals.var_jsswg2_dn14 = assign15620_e10525_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign15630_e10528: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign15630_e10528;
        locals.var_guard331_rv = 0.0;

        let assign15640_e10531: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard332 = assign15640_e10531;
        locals.var_guard332_rv = 0.0;

        let (assign15650_e10541, assign15650_e10541_d_n0, assign15650_e10541_d_n2, assign15650_e10541_d_n4, assign15650_e10541_d_n5, assign15650_e10541_d_n6, assign15650_e10541_d_n7, assign15650_e10541_d_n8, assign15650_e10541_d_n9, assign15650_e10541_d_n10, assign15650_e10541_d_n11, assign15650_e10541_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15650_e10539: f64 = (p.p13 * locals.var_js);
        (assign15650_e10539, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15650_e10541;
        locals.var_isbd_btm_dn0 = assign15650_e10541_d_n0;
        locals.var_isbd_btm_dn2 = assign15650_e10541_d_n2;
        locals.var_isbd_btm_dn4 = assign15650_e10541_d_n4;
        locals.var_isbd_btm_dn5 = assign15650_e10541_d_n5;
        locals.var_isbd_btm_dn6 = assign15650_e10541_d_n6;
        locals.var_isbd_btm_dn7 = assign15650_e10541_d_n7;
        locals.var_isbd_btm_dn8 = assign15650_e10541_d_n8;
        locals.var_isbd_btm_dn9 = assign15650_e10541_d_n9;
        locals.var_isbd_btm_dn10 = assign15650_e10541_d_n10;
        locals.var_isbd_btm_dn11 = assign15650_e10541_d_n11;
        locals.var_isbd_btm_dn14 = assign15650_e10541_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15660_e10551, assign15660_e10551_d_n0, assign15660_e10551_d_n2, assign15660_e10551_d_n4, assign15660_e10551_d_n5, assign15660_e10551_d_n6, assign15660_e10551_d_n7, assign15660_e10551_d_n8, assign15660_e10551_d_n9, assign15660_e10551_d_n10, assign15660_e10551_d_n11, assign15660_e10551_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15660_e10549: f64 = (p.p13 * locals.var_js2);
        (assign15660_e10549, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15660_e10551;
        locals.var_isbd2_btm_dn0 = assign15660_e10551_d_n0;
        locals.var_isbd2_btm_dn2 = assign15660_e10551_d_n2;
        locals.var_isbd2_btm_dn4 = assign15660_e10551_d_n4;
        locals.var_isbd2_btm_dn5 = assign15660_e10551_d_n5;
        locals.var_isbd2_btm_dn6 = assign15660_e10551_d_n6;
        locals.var_isbd2_btm_dn7 = assign15660_e10551_d_n7;
        locals.var_isbd2_btm_dn8 = assign15660_e10551_d_n8;
        locals.var_isbd2_btm_dn9 = assign15660_e10551_d_n9;
        locals.var_isbd2_btm_dn10 = assign15660_e10551_d_n10;
        locals.var_isbd2_btm_dn11 = assign15660_e10551_d_n11;
        locals.var_isbd2_btm_dn14 = assign15660_e10551_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15670_e10563, assign15670_e10563_d_n0, assign15670_e10563_d_n2, assign15670_e10563_d_n4, assign15670_e10563_d_n5, assign15670_e10563_d_n6, assign15670_e10563_d_n7, assign15670_e10563_d_n8, assign15670_e10563_d_n9, assign15670_e10563_d_n10, assign15670_e10563_d_n11, assign15670_e10563_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15670_e10559: f64 = (p.p15 - locals.var_weff_nf);
        let assign15670_e10561: f64 = (assign15670_e10559 * locals.var_jssw);
        (assign15670_e10561, (assign15670_e10559 * locals.var_jssw_dn0), (assign15670_e10559 * locals.var_jssw_dn2), (assign15670_e10559 * locals.var_jssw_dn4), (assign15670_e10559 * locals.var_jssw_dn5), (assign15670_e10559 * locals.var_jssw_dn6), (assign15670_e10559 * locals.var_jssw_dn7), (assign15670_e10559 * locals.var_jssw_dn8), (assign15670_e10559 * locals.var_jssw_dn9), (assign15670_e10559 * locals.var_jssw_dn10), (assign15670_e10559 * locals.var_jssw_dn11), (assign15670_e10559 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15670_e10563;
        locals.var_isbd_sws_dn0 = assign15670_e10563_d_n0;
        locals.var_isbd_sws_dn2 = assign15670_e10563_d_n2;
        locals.var_isbd_sws_dn4 = assign15670_e10563_d_n4;
        locals.var_isbd_sws_dn5 = assign15670_e10563_d_n5;
        locals.var_isbd_sws_dn6 = assign15670_e10563_d_n6;
        locals.var_isbd_sws_dn7 = assign15670_e10563_d_n7;
        locals.var_isbd_sws_dn8 = assign15670_e10563_d_n8;
        locals.var_isbd_sws_dn9 = assign15670_e10563_d_n9;
        locals.var_isbd_sws_dn10 = assign15670_e10563_d_n10;
        locals.var_isbd_sws_dn11 = assign15670_e10563_d_n11;
        locals.var_isbd_sws_dn14 = assign15670_e10563_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15680_e10575, assign15680_e10575_d_n0, assign15680_e10575_d_n2, assign15680_e10575_d_n4, assign15680_e10575_d_n5, assign15680_e10575_d_n6, assign15680_e10575_d_n7, assign15680_e10575_d_n8, assign15680_e10575_d_n9, assign15680_e10575_d_n10, assign15680_e10575_d_n11, assign15680_e10575_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15680_e10571: f64 = (p.p15 - locals.var_weff_nf);
        let assign15680_e10573: f64 = (assign15680_e10571 * locals.var_jssw2);
        (assign15680_e10573, (assign15680_e10571 * locals.var_jssw2_dn0), (assign15680_e10571 * locals.var_jssw2_dn2), (assign15680_e10571 * locals.var_jssw2_dn4), (assign15680_e10571 * locals.var_jssw2_dn5), (assign15680_e10571 * locals.var_jssw2_dn6), (assign15680_e10571 * locals.var_jssw2_dn7), (assign15680_e10571 * locals.var_jssw2_dn8), (assign15680_e10571 * locals.var_jssw2_dn9), (assign15680_e10571 * locals.var_jssw2_dn10), (assign15680_e10571 * locals.var_jssw2_dn11), (assign15680_e10571 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15680_e10575;
        locals.var_isbd2_sws_dn0 = assign15680_e10575_d_n0;
        locals.var_isbd2_sws_dn2 = assign15680_e10575_d_n2;
        locals.var_isbd2_sws_dn4 = assign15680_e10575_d_n4;
        locals.var_isbd2_sws_dn5 = assign15680_e10575_d_n5;
        locals.var_isbd2_sws_dn6 = assign15680_e10575_d_n6;
        locals.var_isbd2_sws_dn7 = assign15680_e10575_d_n7;
        locals.var_isbd2_sws_dn8 = assign15680_e10575_d_n8;
        locals.var_isbd2_sws_dn9 = assign15680_e10575_d_n9;
        locals.var_isbd2_sws_dn10 = assign15680_e10575_d_n10;
        locals.var_isbd2_sws_dn11 = assign15680_e10575_d_n11;
        locals.var_isbd2_sws_dn14 = assign15680_e10575_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign15690_e10585, assign15690_e10585_d_n0, assign15690_e10585_d_n2, assign15690_e10585_d_n4, assign15690_e10585_d_n5, assign15690_e10585_d_n6, assign15690_e10585_d_n7, assign15690_e10585_d_n8, assign15690_e10585_d_n9, assign15690_e10585_d_n10, assign15690_e10585_d_n11, assign15690_e10585_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15690_e10583: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign15690_e10583, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15690_e10585;
        locals.var_isbd_swg_dn0 = assign15690_e10585_d_n0;
        locals.var_isbd_swg_dn2 = assign15690_e10585_d_n2;
        locals.var_isbd_swg_dn4 = assign15690_e10585_d_n4;
        locals.var_isbd_swg_dn5 = assign15690_e10585_d_n5;
        locals.var_isbd_swg_dn6 = assign15690_e10585_d_n6;
        locals.var_isbd_swg_dn7 = assign15690_e10585_d_n7;
        locals.var_isbd_swg_dn8 = assign15690_e10585_d_n8;
        locals.var_isbd_swg_dn9 = assign15690_e10585_d_n9;
        locals.var_isbd_swg_dn10 = assign15690_e10585_d_n10;
        locals.var_isbd_swg_dn11 = assign15690_e10585_d_n11;
        locals.var_isbd_swg_dn14 = assign15690_e10585_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15700_e10595, assign15700_e10595_d_n0, assign15700_e10595_d_n2, assign15700_e10595_d_n4, assign15700_e10595_d_n5, assign15700_e10595_d_n6, assign15700_e10595_d_n7, assign15700_e10595_d_n8, assign15700_e10595_d_n9, assign15700_e10595_d_n10, assign15700_e10595_d_n11, assign15700_e10595_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 != 0.0)) {
        let assign15700_e10593: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign15700_e10593, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15700_e10595;
        locals.var_isbd2_swg_dn0 = assign15700_e10595_d_n0;
        locals.var_isbd2_swg_dn2 = assign15700_e10595_d_n2;
        locals.var_isbd2_swg_dn4 = assign15700_e10595_d_n4;
        locals.var_isbd2_swg_dn5 = assign15700_e10595_d_n5;
        locals.var_isbd2_swg_dn6 = assign15700_e10595_d_n6;
        locals.var_isbd2_swg_dn7 = assign15700_e10595_d_n7;
        locals.var_isbd2_swg_dn8 = assign15700_e10595_d_n8;
        locals.var_isbd2_swg_dn9 = assign15700_e10595_d_n9;
        locals.var_isbd2_swg_dn10 = assign15700_e10595_d_n10;
        locals.var_isbd2_swg_dn11 = assign15700_e10595_d_n11;
        locals.var_isbd2_swg_dn14 = assign15700_e10595_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15710_e10606, assign15710_e10606_d_n0, assign15710_e10606_d_n2, assign15710_e10606_d_n4, assign15710_e10606_d_n5, assign15710_e10606_d_n6, assign15710_e10606_d_n7, assign15710_e10606_d_n8, assign15710_e10606_d_n9, assign15710_e10606_d_n10, assign15710_e10606_d_n11, assign15710_e10606_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign15710_e10604: f64 = (p.p13 * locals.var_js);
        (assign15710_e10604, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15710_e10606;
        locals.var_isbd_btm_dn0 = assign15710_e10606_d_n0;
        locals.var_isbd_btm_dn2 = assign15710_e10606_d_n2;
        locals.var_isbd_btm_dn4 = assign15710_e10606_d_n4;
        locals.var_isbd_btm_dn5 = assign15710_e10606_d_n5;
        locals.var_isbd_btm_dn6 = assign15710_e10606_d_n6;
        locals.var_isbd_btm_dn7 = assign15710_e10606_d_n7;
        locals.var_isbd_btm_dn8 = assign15710_e10606_d_n8;
        locals.var_isbd_btm_dn9 = assign15710_e10606_d_n9;
        locals.var_isbd_btm_dn10 = assign15710_e10606_d_n10;
        locals.var_isbd_btm_dn11 = assign15710_e10606_d_n11;
        locals.var_isbd_btm_dn14 = assign15710_e10606_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15720_e10617, assign15720_e10617_d_n0, assign15720_e10617_d_n2, assign15720_e10617_d_n4, assign15720_e10617_d_n5, assign15720_e10617_d_n6, assign15720_e10617_d_n7, assign15720_e10617_d_n8, assign15720_e10617_d_n9, assign15720_e10617_d_n10, assign15720_e10617_d_n11, assign15720_e10617_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign15720_e10615: f64 = (p.p13 * locals.var_js2);
        (assign15720_e10615, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15720_e10617;
        locals.var_isbd2_btm_dn0 = assign15720_e10617_d_n0;
        locals.var_isbd2_btm_dn2 = assign15720_e10617_d_n2;
        locals.var_isbd2_btm_dn4 = assign15720_e10617_d_n4;
        locals.var_isbd2_btm_dn5 = assign15720_e10617_d_n5;
        locals.var_isbd2_btm_dn6 = assign15720_e10617_d_n6;
        locals.var_isbd2_btm_dn7 = assign15720_e10617_d_n7;
        locals.var_isbd2_btm_dn8 = assign15720_e10617_d_n8;
        locals.var_isbd2_btm_dn9 = assign15720_e10617_d_n9;
        locals.var_isbd2_btm_dn10 = assign15720_e10617_d_n10;
        locals.var_isbd2_btm_dn11 = assign15720_e10617_d_n11;
        locals.var_isbd2_btm_dn14 = assign15720_e10617_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15730_e10626, assign15730_e10626_d_n0, assign15730_e10626_d_n2, assign15730_e10626_d_n4, assign15730_e10626_d_n5, assign15730_e10626_d_n6, assign15730_e10626_d_n7, assign15730_e10626_d_n8, assign15730_e10626_d_n9, assign15730_e10626_d_n10, assign15730_e10626_d_n11, assign15730_e10626_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15730_e10626;
        locals.var_isbd_sws_dn0 = assign15730_e10626_d_n0;
        locals.var_isbd_sws_dn2 = assign15730_e10626_d_n2;
        locals.var_isbd_sws_dn4 = assign15730_e10626_d_n4;
        locals.var_isbd_sws_dn5 = assign15730_e10626_d_n5;
        locals.var_isbd_sws_dn6 = assign15730_e10626_d_n6;
        locals.var_isbd_sws_dn7 = assign15730_e10626_d_n7;
        locals.var_isbd_sws_dn8 = assign15730_e10626_d_n8;
        locals.var_isbd_sws_dn9 = assign15730_e10626_d_n9;
        locals.var_isbd_sws_dn10 = assign15730_e10626_d_n10;
        locals.var_isbd_sws_dn11 = assign15730_e10626_d_n11;
        locals.var_isbd_sws_dn14 = assign15730_e10626_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15740_e10635, assign15740_e10635_d_n0, assign15740_e10635_d_n2, assign15740_e10635_d_n4, assign15740_e10635_d_n5, assign15740_e10635_d_n6, assign15740_e10635_d_n7, assign15740_e10635_d_n8, assign15740_e10635_d_n9, assign15740_e10635_d_n10, assign15740_e10635_d_n11, assign15740_e10635_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15740_e10635;
        locals.var_isbd2_sws_dn0 = assign15740_e10635_d_n0;
        locals.var_isbd2_sws_dn2 = assign15740_e10635_d_n2;
        locals.var_isbd2_sws_dn4 = assign15740_e10635_d_n4;
        locals.var_isbd2_sws_dn5 = assign15740_e10635_d_n5;
        locals.var_isbd2_sws_dn6 = assign15740_e10635_d_n6;
        locals.var_isbd2_sws_dn7 = assign15740_e10635_d_n7;
        locals.var_isbd2_sws_dn8 = assign15740_e10635_d_n8;
        locals.var_isbd2_sws_dn9 = assign15740_e10635_d_n9;
        locals.var_isbd2_sws_dn10 = assign15740_e10635_d_n10;
        locals.var_isbd2_sws_dn11 = assign15740_e10635_d_n11;
        locals.var_isbd2_sws_dn14 = assign15740_e10635_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign15750_e10646, assign15750_e10646_d_n0, assign15750_e10646_d_n2, assign15750_e10646_d_n4, assign15750_e10646_d_n5, assign15750_e10646_d_n6, assign15750_e10646_d_n7, assign15750_e10646_d_n8, assign15750_e10646_d_n9, assign15750_e10646_d_n10, assign15750_e10646_d_n11, assign15750_e10646_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign15750_e10644: f64 = (p.p15 * locals.var_jsswg);
        (assign15750_e10644, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15750_e10646;
        locals.var_isbd_swg_dn0 = assign15750_e10646_d_n0;
        locals.var_isbd_swg_dn2 = assign15750_e10646_d_n2;
        locals.var_isbd_swg_dn4 = assign15750_e10646_d_n4;
        locals.var_isbd_swg_dn5 = assign15750_e10646_d_n5;
        locals.var_isbd_swg_dn6 = assign15750_e10646_d_n6;
        locals.var_isbd_swg_dn7 = assign15750_e10646_d_n7;
        locals.var_isbd_swg_dn8 = assign15750_e10646_d_n8;
        locals.var_isbd_swg_dn9 = assign15750_e10646_d_n9;
        locals.var_isbd_swg_dn10 = assign15750_e10646_d_n10;
        locals.var_isbd_swg_dn11 = assign15750_e10646_d_n11;
        locals.var_isbd_swg_dn14 = assign15750_e10646_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15760_e10657, assign15760_e10657_d_n0, assign15760_e10657_d_n2, assign15760_e10657_d_n4, assign15760_e10657_d_n5, assign15760_e10657_d_n6, assign15760_e10657_d_n7, assign15760_e10657_d_n8, assign15760_e10657_d_n9, assign15760_e10657_d_n10, assign15760_e10657_d_n11, assign15760_e10657_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard331 != 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign15760_e10655: f64 = (p.p15 * locals.var_jsswg2);
        (assign15760_e10655, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15760_e10657;
        locals.var_isbd2_swg_dn0 = assign15760_e10657_d_n0;
        locals.var_isbd2_swg_dn2 = assign15760_e10657_d_n2;
        locals.var_isbd2_swg_dn4 = assign15760_e10657_d_n4;
        locals.var_isbd2_swg_dn5 = assign15760_e10657_d_n5;
        locals.var_isbd2_swg_dn6 = assign15760_e10657_d_n6;
        locals.var_isbd2_swg_dn7 = assign15760_e10657_d_n7;
        locals.var_isbd2_swg_dn8 = assign15760_e10657_d_n8;
        locals.var_isbd2_swg_dn9 = assign15760_e10657_d_n9;
        locals.var_isbd2_swg_dn10 = assign15760_e10657_d_n10;
        locals.var_isbd2_swg_dn11 = assign15760_e10657_d_n11;
        locals.var_isbd2_swg_dn14 = assign15760_e10657_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15770_e10666, assign15770_e10666_d_n0, assign15770_e10666_d_n2, assign15770_e10666_d_n4, assign15770_e10666_d_n5, assign15770_e10666_d_n6, assign15770_e10666_d_n7, assign15770_e10666_d_n8, assign15770_e10666_d_n9, assign15770_e10666_d_n10, assign15770_e10666_d_n11, assign15770_e10666_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        let assign15770_e10664: f64 = (p.p13 * locals.var_js);
        (assign15770_e10664, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15770_e10666;
        locals.var_isbd_btm_dn0 = assign15770_e10666_d_n0;
        locals.var_isbd_btm_dn2 = assign15770_e10666_d_n2;
        locals.var_isbd_btm_dn4 = assign15770_e10666_d_n4;
        locals.var_isbd_btm_dn5 = assign15770_e10666_d_n5;
        locals.var_isbd_btm_dn6 = assign15770_e10666_d_n6;
        locals.var_isbd_btm_dn7 = assign15770_e10666_d_n7;
        locals.var_isbd_btm_dn8 = assign15770_e10666_d_n8;
        locals.var_isbd_btm_dn9 = assign15770_e10666_d_n9;
        locals.var_isbd_btm_dn10 = assign15770_e10666_d_n10;
        locals.var_isbd_btm_dn11 = assign15770_e10666_d_n11;
        locals.var_isbd_btm_dn14 = assign15770_e10666_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign15780_e10675, assign15780_e10675_d_n0, assign15780_e10675_d_n2, assign15780_e10675_d_n4, assign15780_e10675_d_n5, assign15780_e10675_d_n6, assign15780_e10675_d_n7, assign15780_e10675_d_n8, assign15780_e10675_d_n9, assign15780_e10675_d_n10, assign15780_e10675_d_n11, assign15780_e10675_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        let assign15780_e10673: f64 = (p.p13 * locals.var_js2);
        (assign15780_e10673, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15780_e10675;
        locals.var_isbd2_btm_dn0 = assign15780_e10675_d_n0;
        locals.var_isbd2_btm_dn2 = assign15780_e10675_d_n2;
        locals.var_isbd2_btm_dn4 = assign15780_e10675_d_n4;
        locals.var_isbd2_btm_dn5 = assign15780_e10675_d_n5;
        locals.var_isbd2_btm_dn6 = assign15780_e10675_d_n6;
        locals.var_isbd2_btm_dn7 = assign15780_e10675_d_n7;
        locals.var_isbd2_btm_dn8 = assign15780_e10675_d_n8;
        locals.var_isbd2_btm_dn9 = assign15780_e10675_d_n9;
        locals.var_isbd2_btm_dn10 = assign15780_e10675_d_n10;
        locals.var_isbd2_btm_dn11 = assign15780_e10675_d_n11;
        locals.var_isbd2_btm_dn14 = assign15780_e10675_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign15790_e10684, assign15790_e10684_d_n0, assign15790_e10684_d_n2, assign15790_e10684_d_n4, assign15790_e10684_d_n5, assign15790_e10684_d_n6, assign15790_e10684_d_n7, assign15790_e10684_d_n8, assign15790_e10684_d_n9, assign15790_e10684_d_n10, assign15790_e10684_d_n11, assign15790_e10684_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        let assign15790_e10682: f64 = (p.p15 * locals.var_jssw);
        (assign15790_e10682, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15790_e10684;
        locals.var_isbd_sws_dn0 = assign15790_e10684_d_n0;
        locals.var_isbd_sws_dn2 = assign15790_e10684_d_n2;
        locals.var_isbd_sws_dn4 = assign15790_e10684_d_n4;
        locals.var_isbd_sws_dn5 = assign15790_e10684_d_n5;
        locals.var_isbd_sws_dn6 = assign15790_e10684_d_n6;
        locals.var_isbd_sws_dn7 = assign15790_e10684_d_n7;
        locals.var_isbd_sws_dn8 = assign15790_e10684_d_n8;
        locals.var_isbd_sws_dn9 = assign15790_e10684_d_n9;
        locals.var_isbd_sws_dn10 = assign15790_e10684_d_n10;
        locals.var_isbd_sws_dn11 = assign15790_e10684_d_n11;
        locals.var_isbd_sws_dn14 = assign15790_e10684_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign15800_e10693, assign15800_e10693_d_n0, assign15800_e10693_d_n2, assign15800_e10693_d_n4, assign15800_e10693_d_n5, assign15800_e10693_d_n6, assign15800_e10693_d_n7, assign15800_e10693_d_n8, assign15800_e10693_d_n9, assign15800_e10693_d_n10, assign15800_e10693_d_n11, assign15800_e10693_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        let assign15800_e10691: f64 = (p.p15 * locals.var_jssw2);
        (assign15800_e10691, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15800_e10693;
        locals.var_isbd2_sws_dn0 = assign15800_e10693_d_n0;
        locals.var_isbd2_sws_dn2 = assign15800_e10693_d_n2;
        locals.var_isbd2_sws_dn4 = assign15800_e10693_d_n4;
        locals.var_isbd2_sws_dn5 = assign15800_e10693_d_n5;
        locals.var_isbd2_sws_dn6 = assign15800_e10693_d_n6;
        locals.var_isbd2_sws_dn7 = assign15800_e10693_d_n7;
        locals.var_isbd2_sws_dn8 = assign15800_e10693_d_n8;
        locals.var_isbd2_sws_dn9 = assign15800_e10693_d_n9;
        locals.var_isbd2_sws_dn10 = assign15800_e10693_d_n10;
        locals.var_isbd2_sws_dn11 = assign15800_e10693_d_n11;
        locals.var_isbd2_sws_dn14 = assign15800_e10693_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15810_e10700, assign15810_e10700_d_n0, assign15810_e10700_d_n2, assign15810_e10700_d_n4, assign15810_e10700_d_n5, assign15810_e10700_d_n6, assign15810_e10700_d_n7, assign15810_e10700_d_n8, assign15810_e10700_d_n9, assign15810_e10700_d_n10, assign15810_e10700_d_n11, assign15810_e10700_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15810_e10700;
        locals.var_isbd_swg_dn0 = assign15810_e10700_d_n0;
        locals.var_isbd_swg_dn2 = assign15810_e10700_d_n2;
        locals.var_isbd_swg_dn4 = assign15810_e10700_d_n4;
        locals.var_isbd_swg_dn5 = assign15810_e10700_d_n5;
        locals.var_isbd_swg_dn6 = assign15810_e10700_d_n6;
        locals.var_isbd_swg_dn7 = assign15810_e10700_d_n7;
        locals.var_isbd_swg_dn8 = assign15810_e10700_d_n8;
        locals.var_isbd_swg_dn9 = assign15810_e10700_d_n9;
        locals.var_isbd_swg_dn10 = assign15810_e10700_d_n10;
        locals.var_isbd_swg_dn11 = assign15810_e10700_d_n11;
        locals.var_isbd_swg_dn14 = assign15810_e10700_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign15820_e10707, assign15820_e10707_d_n0, assign15820_e10707_d_n2, assign15820_e10707_d_n4, assign15820_e10707_d_n5, assign15820_e10707_d_n6, assign15820_e10707_d_n7, assign15820_e10707_d_n8, assign15820_e10707_d_n9, assign15820_e10707_d_n10, assign15820_e10707_d_n11, assign15820_e10707_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard331 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15820_e10707;
        locals.var_isbd2_swg_dn0 = assign15820_e10707_d_n0;
        locals.var_isbd2_swg_dn2 = assign15820_e10707_d_n2;
        locals.var_isbd2_swg_dn4 = assign15820_e10707_d_n4;
        locals.var_isbd2_swg_dn5 = assign15820_e10707_d_n5;
        locals.var_isbd2_swg_dn6 = assign15820_e10707_d_n6;
        locals.var_isbd2_swg_dn7 = assign15820_e10707_d_n7;
        locals.var_isbd2_swg_dn8 = assign15820_e10707_d_n8;
        locals.var_isbd2_swg_dn9 = assign15820_e10707_d_n9;
        locals.var_isbd2_swg_dn10 = assign15820_e10707_d_n10;
        locals.var_isbd2_swg_dn11 = assign15820_e10707_d_n11;
        locals.var_isbd2_swg_dn14 = assign15820_e10707_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign15830_e10715, assign15830_e10715_d_n0, assign15830_e10715_d_n2, assign15830_e10715_d_n4, assign15830_e10715_d_n5, assign15830_e10715_d_n6, assign15830_e10715_d_n7, assign15830_e10715_d_n8, assign15830_e10715_d_n9, assign15830_e10715_d_n10, assign15830_e10715_d_n11, assign15830_e10715_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15830_e10711: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign15830_e10713: f64 = (assign15830_e10711 + locals.var_isbd_swg);
        (assign15830_e10713, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign15830_e10715;
        locals.var_isbd_dn0 = assign15830_e10715_d_n0;
        locals.var_isbd_dn2 = assign15830_e10715_d_n2;
        locals.var_isbd_dn4 = assign15830_e10715_d_n4;
        locals.var_isbd_dn5 = assign15830_e10715_d_n5;
        locals.var_isbd_dn6 = assign15830_e10715_d_n6;
        locals.var_isbd_dn7 = assign15830_e10715_d_n7;
        locals.var_isbd_dn8 = assign15830_e10715_d_n8;
        locals.var_isbd_dn9 = assign15830_e10715_d_n9;
        locals.var_isbd_dn10 = assign15830_e10715_d_n10;
        locals.var_isbd_dn11 = assign15830_e10715_d_n11;
        locals.var_isbd_dn14 = assign15830_e10715_d_n14;
        locals.var_isbd_rv = 0.0;

        let assign15840_e10718: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign15840_e10718;
        locals.var_guard333_rv = 0.0;

        let (assign15850_e10726, assign15850_e10726_d_n0, assign15850_e10726_d_n2, assign15850_e10726_d_n4, assign15850_e10726_d_n5, assign15850_e10726_d_n6, assign15850_e10726_d_n7, assign15850_e10726_d_n8, assign15850_e10726_d_n9, assign15850_e10726_d_n10, assign15850_e10726_d_n11, assign15850_e10726_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard333 != 0.0)) {
        let assign15850_e10724: f64 = (locals.var_isbd + 1e-25);
        (assign15850_e10724, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15850_e10726;
        locals.var_t2_dn0 = assign15850_e10726_d_n0;
        locals.var_t2_dn2 = assign15850_e10726_d_n2;
        locals.var_t2_dn4 = assign15850_e10726_d_n4;
        locals.var_t2_dn5 = assign15850_e10726_d_n5;
        locals.var_t2_dn6 = assign15850_e10726_d_n6;
        locals.var_t2_dn7 = assign15850_e10726_d_n7;
        locals.var_t2_dn8 = assign15850_e10726_d_n8;
        locals.var_t2_dn9 = assign15850_e10726_d_n9;
        locals.var_t2_dn10 = assign15850_e10726_d_n10;
        locals.var_t2_dn11 = assign15850_e10726_d_n11;
        locals.var_t2_dn14 = assign15850_e10726_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15860_e10743, assign15860_e10743_d_n0, assign15860_e10743_d_n2, assign15860_e10743_d_n4, assign15860_e10743_d_n5, assign15860_e10743_d_n6, assign15860_e10743_d_n7, assign15860_e10743_d_n8, assign15860_e10743_d_n9, assign15860_e10743_d_n10, assign15860_e10743_d_n11, assign15860_e10743_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard333 != 0.0)) {
        let assign15860_e10732: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15860_e10735: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign15860_e10737: f64 = (assign15860_e10735 / locals.var_t2);
        let assign15860_e10739: f64 = (assign15860_e10737 + 1.0);
        let assign15860_e10740: f64 = (assign15860_e10739).ln();
        let assign15860_e10741: f64 = (assign15860_e10732 * assign15860_e10740);
        (assign15860_e10741, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign15860_e10740) + (assign15860_e10732 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign15860_e10735 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign15860_e10739))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign15860_e10743;
        locals.var_vbdt_dn0 = assign15860_e10743_d_n0;
        locals.var_vbdt_dn2 = assign15860_e10743_d_n2;
        locals.var_vbdt_dn4 = assign15860_e10743_d_n4;
        locals.var_vbdt_dn5 = assign15860_e10743_d_n5;
        locals.var_vbdt_dn6 = assign15860_e10743_d_n6;
        locals.var_vbdt_dn7 = assign15860_e10743_d_n7;
        locals.var_vbdt_dn8 = assign15860_e10743_d_n8;
        locals.var_vbdt_dn9 = assign15860_e10743_d_n9;
        locals.var_vbdt_dn10 = assign15860_e10743_d_n10;
        locals.var_vbdt_dn11 = assign15860_e10743_d_n11;
        locals.var_vbdt_dn14 = assign15860_e10743_d_n14;
        locals.var_vbdt_rv = 0.0;

        let (assign15870_e10754, assign15870_e10754_d_n0, assign15870_e10754_d_n2, assign15870_e10754_d_n4, assign15870_e10754_d_n5, assign15870_e10754_d_n6, assign15870_e10754_d_n7, assign15870_e10754_d_n8, assign15870_e10754_d_n9, assign15870_e10754_d_n10, assign15870_e10754_d_n11, assign15870_e10754_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard333 != 0.0)) {
        let assign15870_e10749: f64 = (locals.var_tratio - 1.0);
        let assign15870_e10751: f64 = (assign15870_e10749 * p.p512);
        let assign15870_e10752: f64 = (assign15870_e10751).exp();
        (assign15870_e10752, (assign15870_e10752 * (locals.var_tratio_dn0 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn2 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn4 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn5 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn6 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn7 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn8 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn9 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn10 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn11 * p.p512)), (assign15870_e10752 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign15870_e10754;
        locals.var_exptempd_dn0 = assign15870_e10754_d_n0;
        locals.var_exptempd_dn2 = assign15870_e10754_d_n2;
        locals.var_exptempd_dn4 = assign15870_e10754_d_n4;
        locals.var_exptempd_dn5 = assign15870_e10754_d_n5;
        locals.var_exptempd_dn6 = assign15870_e10754_d_n6;
        locals.var_exptempd_dn7 = assign15870_e10754_d_n7;
        locals.var_exptempd_dn8 = assign15870_e10754_d_n8;
        locals.var_exptempd_dn9 = assign15870_e10754_d_n9;
        locals.var_exptempd_dn10 = assign15870_e10754_d_n10;
        locals.var_exptempd_dn11 = assign15870_e10754_d_n11;
        locals.var_exptempd_dn14 = assign15870_e10754_d_n14;
        locals.var_exptempd_rv = 0.0;

        let (assign15880_e10764, assign15880_e10764_d_n0, assign15880_e10764_d_n2, assign15880_e10764_d_n4, assign15880_e10764_d_n5, assign15880_e10764_d_n6, assign15880_e10764_d_n7, assign15880_e10764_d_n8, assign15880_e10764_d_n9, assign15880_e10764_d_n10, assign15880_e10764_d_n11, assign15880_e10764_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard333 != 0.0)) {
        let assign15880_e10761: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15880_e10762: f64 = (1.0 / assign15880_e10761);
        (assign15880_e10762, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign15880_e10761 * assign15880_e10761))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign15880_e10764;
        locals.var_jd_nvtm_invd_dn0 = assign15880_e10764_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign15880_e10764_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign15880_e10764_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign15880_e10764_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign15880_e10764_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign15880_e10764_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign15880_e10764_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign15880_e10764_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign15880_e10764_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign15880_e10764_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign15880_e10764_d_n14;
        locals.var_jd_nvtm_invd_rv = 0.0;

        let (assign15890_e10773, assign15890_e10773_d_n0, assign15890_e10773_d_n2, assign15890_e10773_d_n4, assign15890_e10773_d_n5, assign15890_e10773_d_n6, assign15890_e10773_d_n7, assign15890_e10773_d_n8, assign15890_e10773_d_n9, assign15890_e10773_d_n10, assign15890_e10773_d_n11, assign15890_e10773_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard333 != 0.0)) {
        let assign15890_e10770: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign15890_e10771: f64 = (assign15890_e10770).exp();
        (assign15890_e10771, (assign15890_e10771 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign15890_e10771 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign15890_e10771 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign15890_e10771 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign15890_e10771 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign15890_e10771 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign15890_e10771 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign15890_e10771 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign15890_e10771 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign15890_e10771 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign15890_e10771 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign15890_e10773;
        locals.var_jd_expcd_dn0 = assign15890_e10773_d_n0;
        locals.var_jd_expcd_dn2 = assign15890_e10773_d_n2;
        locals.var_jd_expcd_dn4 = assign15890_e10773_d_n4;
        locals.var_jd_expcd_dn5 = assign15890_e10773_d_n5;
        locals.var_jd_expcd_dn6 = assign15890_e10773_d_n6;
        locals.var_jd_expcd_dn7 = assign15890_e10773_d_n7;
        locals.var_jd_expcd_dn8 = assign15890_e10773_d_n8;
        locals.var_jd_expcd_dn9 = assign15890_e10773_d_n9;
        locals.var_jd_expcd_dn10 = assign15890_e10773_d_n10;
        locals.var_jd_expcd_dn11 = assign15890_e10773_d_n11;
        locals.var_jd_expcd_dn14 = assign15890_e10773_d_n14;
        locals.var_jd_expcd_rv = 0.0;

        let (assign15900_e10792, assign15900_e10792_d_n0, assign15900_e10792_d_n2, assign15900_e10792_d_n4, assign15900_e10792_d_n5, assign15900_e10792_d_n6, assign15900_e10792_d_n7, assign15900_e10792_d_n8, assign15900_e10792_d_n9, assign15900_e10792_d_n10, assign15900_e10792_d_n11, assign15900_e10792_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15900_e10778: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15900_e10781: f64 = (locals.var_eg * locals.var_beta);
        let assign15900_e10782: f64 = (assign15900_e10778 - assign15900_e10781);
        let assign15900_e10785: f64 = (p.p522 * locals.var_log_tratio);
        let assign15900_e10786: f64 = (assign15900_e10782 + assign15900_e10785);
        let assign15900_e10788: f64 = (assign15900_e10786 / locals.var_uc_njs);
        let assign15900_e10789: f64 = (assign15900_e10788).exp();
        let assign15900_e10790: f64 = (locals.var_uc_js0s * assign15900_e10789);
        (assign15900_e10790, (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15900_e10789 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign15900_e10792;
        locals.var_js_dn0 = assign15900_e10792_d_n0;
        locals.var_js_dn2 = assign15900_e10792_d_n2;
        locals.var_js_dn4 = assign15900_e10792_d_n4;
        locals.var_js_dn5 = assign15900_e10792_d_n5;
        locals.var_js_dn6 = assign15900_e10792_d_n6;
        locals.var_js_dn7 = assign15900_e10792_d_n7;
        locals.var_js_dn8 = assign15900_e10792_d_n8;
        locals.var_js_dn9 = assign15900_e10792_d_n9;
        locals.var_js_dn10 = assign15900_e10792_d_n10;
        locals.var_js_dn11 = assign15900_e10792_d_n11;
        locals.var_js_dn14 = assign15900_e10792_d_n14;
        locals.var_js_rv = 0.0;

        let (assign15910_e10811, assign15910_e10811_d_n0, assign15910_e10811_d_n2, assign15910_e10811_d_n4, assign15910_e10811_d_n5, assign15910_e10811_d_n6, assign15910_e10811_d_n7, assign15910_e10811_d_n8, assign15910_e10811_d_n9, assign15910_e10811_d_n10, assign15910_e10811_d_n11, assign15910_e10811_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15910_e10797: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15910_e10800: f64 = (locals.var_eg * locals.var_beta);
        let assign15910_e10801: f64 = (assign15910_e10797 - assign15910_e10800);
        let assign15910_e10804: f64 = (p.p522 * locals.var_log_tratio);
        let assign15910_e10805: f64 = (assign15910_e10801 + assign15910_e10804);
        let assign15910_e10807: f64 = (assign15910_e10805 / p.p520);
        let assign15910_e10808: f64 = (assign15910_e10807).exp();
        let assign15910_e10809: f64 = (locals.var_uc_js0sws * assign15910_e10808);
        (assign15910_e10809, (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign15910_e10808 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign15910_e10811;
        locals.var_jssw_dn0 = assign15910_e10811_d_n0;
        locals.var_jssw_dn2 = assign15910_e10811_d_n2;
        locals.var_jssw_dn4 = assign15910_e10811_d_n4;
        locals.var_jssw_dn5 = assign15910_e10811_d_n5;
        locals.var_jssw_dn6 = assign15910_e10811_d_n6;
        locals.var_jssw_dn7 = assign15910_e10811_d_n7;
        locals.var_jssw_dn8 = assign15910_e10811_d_n8;
        locals.var_jssw_dn9 = assign15910_e10811_d_n9;
        locals.var_jssw_dn10 = assign15910_e10811_d_n10;
        locals.var_jssw_dn11 = assign15910_e10811_d_n11;
        locals.var_jssw_dn14 = assign15910_e10811_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign15920_e10830, assign15920_e10830_d_n0, assign15920_e10830_d_n2, assign15920_e10830_d_n4, assign15920_e10830_d_n5, assign15920_e10830_d_n6, assign15920_e10830_d_n7, assign15920_e10830_d_n8, assign15920_e10830_d_n9, assign15920_e10830_d_n10, assign15920_e10830_d_n11, assign15920_e10830_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15920_e10816: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15920_e10819: f64 = (locals.var_eg * locals.var_beta);
        let assign15920_e10820: f64 = (assign15920_e10816 - assign15920_e10819);
        let assign15920_e10823: f64 = (p.p522 * locals.var_log_tratio);
        let assign15920_e10824: f64 = (assign15920_e10820 + assign15920_e10823);
        let assign15920_e10826: f64 = (assign15920_e10824 / p.p521);
        let assign15920_e10827: f64 = (assign15920_e10826).exp();
        let assign15920_e10828: f64 = (p.p518 * assign15920_e10827);
        (assign15920_e10828, (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign15920_e10827 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign15920_e10830;
        locals.var_jsswg_dn0 = assign15920_e10830_d_n0;
        locals.var_jsswg_dn2 = assign15920_e10830_d_n2;
        locals.var_jsswg_dn4 = assign15920_e10830_d_n4;
        locals.var_jsswg_dn5 = assign15920_e10830_d_n5;
        locals.var_jsswg_dn6 = assign15920_e10830_d_n6;
        locals.var_jsswg_dn7 = assign15920_e10830_d_n7;
        locals.var_jsswg_dn8 = assign15920_e10830_d_n8;
        locals.var_jsswg_dn9 = assign15920_e10830_d_n9;
        locals.var_jsswg_dn10 = assign15920_e10830_d_n10;
        locals.var_jsswg_dn11 = assign15920_e10830_d_n11;
        locals.var_jsswg_dn14 = assign15920_e10830_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign15930_e10849, assign15930_e10849_d_n0, assign15930_e10849_d_n2, assign15930_e10849_d_n4, assign15930_e10849_d_n5, assign15930_e10849_d_n6, assign15930_e10849_d_n7, assign15930_e10849_d_n8, assign15930_e10849_d_n9, assign15930_e10849_d_n10, assign15930_e10849_d_n11, assign15930_e10849_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15930_e10835: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15930_e10838: f64 = (locals.var_eg * locals.var_beta);
        let assign15930_e10839: f64 = (assign15930_e10835 - assign15930_e10838);
        let assign15930_e10842: f64 = (p.p532 * locals.var_log_tratio);
        let assign15930_e10843: f64 = (assign15930_e10839 + assign15930_e10842);
        let assign15930_e10845: f64 = (assign15930_e10843 / locals.var_uc_njs);
        let assign15930_e10846: f64 = (assign15930_e10845).exp();
        let assign15930_e10847: f64 = (locals.var_uc_js0s * assign15930_e10846);
        (assign15930_e10847, (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15930_e10846 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign15930_e10849;
        locals.var_js2_dn0 = assign15930_e10849_d_n0;
        locals.var_js2_dn2 = assign15930_e10849_d_n2;
        locals.var_js2_dn4 = assign15930_e10849_d_n4;
        locals.var_js2_dn5 = assign15930_e10849_d_n5;
        locals.var_js2_dn6 = assign15930_e10849_d_n6;
        locals.var_js2_dn7 = assign15930_e10849_d_n7;
        locals.var_js2_dn8 = assign15930_e10849_d_n8;
        locals.var_js2_dn9 = assign15930_e10849_d_n9;
        locals.var_js2_dn10 = assign15930_e10849_d_n10;
        locals.var_js2_dn11 = assign15930_e10849_d_n11;
        locals.var_js2_dn14 = assign15930_e10849_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign15940_e10868, assign15940_e10868_d_n0, assign15940_e10868_d_n2, assign15940_e10868_d_n4, assign15940_e10868_d_n5, assign15940_e10868_d_n6, assign15940_e10868_d_n7, assign15940_e10868_d_n8, assign15940_e10868_d_n9, assign15940_e10868_d_n10, assign15940_e10868_d_n11, assign15940_e10868_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15940_e10854: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15940_e10857: f64 = (locals.var_eg * locals.var_beta);
        let assign15940_e10858: f64 = (assign15940_e10854 - assign15940_e10857);
        let assign15940_e10861: f64 = (p.p532 * locals.var_log_tratio);
        let assign15940_e10862: f64 = (assign15940_e10858 + assign15940_e10861);
        let assign15940_e10864: f64 = (assign15940_e10862 / p.p520);
        let assign15940_e10865: f64 = (assign15940_e10864).exp();
        let assign15940_e10866: f64 = (locals.var_uc_js0sws * assign15940_e10865);
        (assign15940_e10866, (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign15940_e10865 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign15940_e10868;
        locals.var_jssw2_dn0 = assign15940_e10868_d_n0;
        locals.var_jssw2_dn2 = assign15940_e10868_d_n2;
        locals.var_jssw2_dn4 = assign15940_e10868_d_n4;
        locals.var_jssw2_dn5 = assign15940_e10868_d_n5;
        locals.var_jssw2_dn6 = assign15940_e10868_d_n6;
        locals.var_jssw2_dn7 = assign15940_e10868_d_n7;
        locals.var_jssw2_dn8 = assign15940_e10868_d_n8;
        locals.var_jssw2_dn9 = assign15940_e10868_d_n9;
        locals.var_jssw2_dn10 = assign15940_e10868_d_n10;
        locals.var_jssw2_dn11 = assign15940_e10868_d_n11;
        locals.var_jssw2_dn14 = assign15940_e10868_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign15950_e10887, assign15950_e10887_d_n0, assign15950_e10887_d_n2, assign15950_e10887_d_n4, assign15950_e10887_d_n5, assign15950_e10887_d_n6, assign15950_e10887_d_n7, assign15950_e10887_d_n8, assign15950_e10887_d_n9, assign15950_e10887_d_n10, assign15950_e10887_d_n11, assign15950_e10887_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15950_e10873: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15950_e10876: f64 = (locals.var_eg * locals.var_beta);
        let assign15950_e10877: f64 = (assign15950_e10873 - assign15950_e10876);
        let assign15950_e10880: f64 = (p.p532 * locals.var_log_tratio);
        let assign15950_e10881: f64 = (assign15950_e10877 + assign15950_e10880);
        let assign15950_e10883: f64 = (assign15950_e10881 / p.p521);
        let assign15950_e10884: f64 = (assign15950_e10883).exp();
        let assign15950_e10885: f64 = (p.p518 * assign15950_e10884);
        (assign15950_e10885, (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign15950_e10884 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign15950_e10887;
        locals.var_jsswg2_dn0 = assign15950_e10887_d_n0;
        locals.var_jsswg2_dn2 = assign15950_e10887_d_n2;
        locals.var_jsswg2_dn4 = assign15950_e10887_d_n4;
        locals.var_jsswg2_dn5 = assign15950_e10887_d_n5;
        locals.var_jsswg2_dn6 = assign15950_e10887_d_n6;
        locals.var_jsswg2_dn7 = assign15950_e10887_d_n7;
        locals.var_jsswg2_dn8 = assign15950_e10887_d_n8;
        locals.var_jsswg2_dn9 = assign15950_e10887_d_n9;
        locals.var_jsswg2_dn10 = assign15950_e10887_d_n10;
        locals.var_jsswg2_dn11 = assign15950_e10887_d_n11;
        locals.var_jsswg2_dn14 = assign15950_e10887_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign15960_e10890: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign15960_e10890;
        locals.var_guard334_rv = 0.0;

        let assign15970_e10893: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard335 = assign15970_e10893;
        locals.var_guard335_rv = 0.0;

        let (assign15980_e10903, assign15980_e10903_d_n0, assign15980_e10903_d_n2, assign15980_e10903_d_n4, assign15980_e10903_d_n5, assign15980_e10903_d_n6, assign15980_e10903_d_n7, assign15980_e10903_d_n8, assign15980_e10903_d_n9, assign15980_e10903_d_n10, assign15980_e10903_d_n11, assign15980_e10903_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign15980_e10901: f64 = (p.p14 * locals.var_js);
        (assign15980_e10901, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign15980_e10903;
        locals.var_isbs_btm_dn0 = assign15980_e10903_d_n0;
        locals.var_isbs_btm_dn2 = assign15980_e10903_d_n2;
        locals.var_isbs_btm_dn4 = assign15980_e10903_d_n4;
        locals.var_isbs_btm_dn5 = assign15980_e10903_d_n5;
        locals.var_isbs_btm_dn6 = assign15980_e10903_d_n6;
        locals.var_isbs_btm_dn7 = assign15980_e10903_d_n7;
        locals.var_isbs_btm_dn8 = assign15980_e10903_d_n8;
        locals.var_isbs_btm_dn9 = assign15980_e10903_d_n9;
        locals.var_isbs_btm_dn10 = assign15980_e10903_d_n10;
        locals.var_isbs_btm_dn11 = assign15980_e10903_d_n11;
        locals.var_isbs_btm_dn14 = assign15980_e10903_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign15990_e10913, assign15990_e10913_d_n0, assign15990_e10913_d_n2, assign15990_e10913_d_n4, assign15990_e10913_d_n5, assign15990_e10913_d_n6, assign15990_e10913_d_n7, assign15990_e10913_d_n8, assign15990_e10913_d_n9, assign15990_e10913_d_n10, assign15990_e10913_d_n11, assign15990_e10913_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign15990_e10911: f64 = (p.p14 * locals.var_js2);
        (assign15990_e10911, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign15990_e10913;
        locals.var_isbs2_btm_dn0 = assign15990_e10913_d_n0;
        locals.var_isbs2_btm_dn2 = assign15990_e10913_d_n2;
        locals.var_isbs2_btm_dn4 = assign15990_e10913_d_n4;
        locals.var_isbs2_btm_dn5 = assign15990_e10913_d_n5;
        locals.var_isbs2_btm_dn6 = assign15990_e10913_d_n6;
        locals.var_isbs2_btm_dn7 = assign15990_e10913_d_n7;
        locals.var_isbs2_btm_dn8 = assign15990_e10913_d_n8;
        locals.var_isbs2_btm_dn9 = assign15990_e10913_d_n9;
        locals.var_isbs2_btm_dn10 = assign15990_e10913_d_n10;
        locals.var_isbs2_btm_dn11 = assign15990_e10913_d_n11;
        locals.var_isbs2_btm_dn14 = assign15990_e10913_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign16000_e10925, assign16000_e10925_d_n0, assign16000_e10925_d_n2, assign16000_e10925_d_n4, assign16000_e10925_d_n5, assign16000_e10925_d_n6, assign16000_e10925_d_n7, assign16000_e10925_d_n8, assign16000_e10925_d_n9, assign16000_e10925_d_n10, assign16000_e10925_d_n11, assign16000_e10925_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign16000_e10921: f64 = (p.p16 - locals.var_weff_nf);
        let assign16000_e10923: f64 = (assign16000_e10921 * locals.var_jssw);
        (assign16000_e10923, (assign16000_e10921 * locals.var_jssw_dn0), (assign16000_e10921 * locals.var_jssw_dn2), (assign16000_e10921 * locals.var_jssw_dn4), (assign16000_e10921 * locals.var_jssw_dn5), (assign16000_e10921 * locals.var_jssw_dn6), (assign16000_e10921 * locals.var_jssw_dn7), (assign16000_e10921 * locals.var_jssw_dn8), (assign16000_e10921 * locals.var_jssw_dn9), (assign16000_e10921 * locals.var_jssw_dn10), (assign16000_e10921 * locals.var_jssw_dn11), (assign16000_e10921 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16000_e10925;
        locals.var_isbs_sws_dn0 = assign16000_e10925_d_n0;
        locals.var_isbs_sws_dn2 = assign16000_e10925_d_n2;
        locals.var_isbs_sws_dn4 = assign16000_e10925_d_n4;
        locals.var_isbs_sws_dn5 = assign16000_e10925_d_n5;
        locals.var_isbs_sws_dn6 = assign16000_e10925_d_n6;
        locals.var_isbs_sws_dn7 = assign16000_e10925_d_n7;
        locals.var_isbs_sws_dn8 = assign16000_e10925_d_n8;
        locals.var_isbs_sws_dn9 = assign16000_e10925_d_n9;
        locals.var_isbs_sws_dn10 = assign16000_e10925_d_n10;
        locals.var_isbs_sws_dn11 = assign16000_e10925_d_n11;
        locals.var_isbs_sws_dn14 = assign16000_e10925_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign16010_e10937, assign16010_e10937_d_n0, assign16010_e10937_d_n2, assign16010_e10937_d_n4, assign16010_e10937_d_n5, assign16010_e10937_d_n6, assign16010_e10937_d_n7, assign16010_e10937_d_n8, assign16010_e10937_d_n9, assign16010_e10937_d_n10, assign16010_e10937_d_n11, assign16010_e10937_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign16010_e10933: f64 = (p.p16 - locals.var_weff_nf);
        let assign16010_e10935: f64 = (assign16010_e10933 * locals.var_jssw2);
        (assign16010_e10935, (assign16010_e10933 * locals.var_jssw2_dn0), (assign16010_e10933 * locals.var_jssw2_dn2), (assign16010_e10933 * locals.var_jssw2_dn4), (assign16010_e10933 * locals.var_jssw2_dn5), (assign16010_e10933 * locals.var_jssw2_dn6), (assign16010_e10933 * locals.var_jssw2_dn7), (assign16010_e10933 * locals.var_jssw2_dn8), (assign16010_e10933 * locals.var_jssw2_dn9), (assign16010_e10933 * locals.var_jssw2_dn10), (assign16010_e10933 * locals.var_jssw2_dn11), (assign16010_e10933 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16010_e10937;
        locals.var_isbs2_sws_dn0 = assign16010_e10937_d_n0;
        locals.var_isbs2_sws_dn2 = assign16010_e10937_d_n2;
        locals.var_isbs2_sws_dn4 = assign16010_e10937_d_n4;
        locals.var_isbs2_sws_dn5 = assign16010_e10937_d_n5;
        locals.var_isbs2_sws_dn6 = assign16010_e10937_d_n6;
        locals.var_isbs2_sws_dn7 = assign16010_e10937_d_n7;
        locals.var_isbs2_sws_dn8 = assign16010_e10937_d_n8;
        locals.var_isbs2_sws_dn9 = assign16010_e10937_d_n9;
        locals.var_isbs2_sws_dn10 = assign16010_e10937_d_n10;
        locals.var_isbs2_sws_dn11 = assign16010_e10937_d_n11;
        locals.var_isbs2_sws_dn14 = assign16010_e10937_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16020_e10947, assign16020_e10947_d_n0, assign16020_e10947_d_n2, assign16020_e10947_d_n4, assign16020_e10947_d_n5, assign16020_e10947_d_n6, assign16020_e10947_d_n7, assign16020_e10947_d_n8, assign16020_e10947_d_n9, assign16020_e10947_d_n10, assign16020_e10947_d_n11, assign16020_e10947_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign16020_e10945: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign16020_e10945, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16020_e10947;
        locals.var_isbs_swg_dn0 = assign16020_e10947_d_n0;
        locals.var_isbs_swg_dn2 = assign16020_e10947_d_n2;
        locals.var_isbs_swg_dn4 = assign16020_e10947_d_n4;
        locals.var_isbs_swg_dn5 = assign16020_e10947_d_n5;
        locals.var_isbs_swg_dn6 = assign16020_e10947_d_n6;
        locals.var_isbs_swg_dn7 = assign16020_e10947_d_n7;
        locals.var_isbs_swg_dn8 = assign16020_e10947_d_n8;
        locals.var_isbs_swg_dn9 = assign16020_e10947_d_n9;
        locals.var_isbs_swg_dn10 = assign16020_e10947_d_n10;
        locals.var_isbs_swg_dn11 = assign16020_e10947_d_n11;
        locals.var_isbs_swg_dn14 = assign16020_e10947_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16030_e10957, assign16030_e10957_d_n0, assign16030_e10957_d_n2, assign16030_e10957_d_n4, assign16030_e10957_d_n5, assign16030_e10957_d_n6, assign16030_e10957_d_n7, assign16030_e10957_d_n8, assign16030_e10957_d_n9, assign16030_e10957_d_n10, assign16030_e10957_d_n11, assign16030_e10957_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) {
        let assign16030_e10955: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign16030_e10955, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16030_e10957;
        locals.var_isbs2_swg_dn0 = assign16030_e10957_d_n0;
        locals.var_isbs2_swg_dn2 = assign16030_e10957_d_n2;
        locals.var_isbs2_swg_dn4 = assign16030_e10957_d_n4;
        locals.var_isbs2_swg_dn5 = assign16030_e10957_d_n5;
        locals.var_isbs2_swg_dn6 = assign16030_e10957_d_n6;
        locals.var_isbs2_swg_dn7 = assign16030_e10957_d_n7;
        locals.var_isbs2_swg_dn8 = assign16030_e10957_d_n8;
        locals.var_isbs2_swg_dn9 = assign16030_e10957_d_n9;
        locals.var_isbs2_swg_dn10 = assign16030_e10957_d_n10;
        locals.var_isbs2_swg_dn11 = assign16030_e10957_d_n11;
        locals.var_isbs2_swg_dn14 = assign16030_e10957_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16040_e10968, assign16040_e10968_d_n0, assign16040_e10968_d_n2, assign16040_e10968_d_n4, assign16040_e10968_d_n5, assign16040_e10968_d_n6, assign16040_e10968_d_n7, assign16040_e10968_d_n8, assign16040_e10968_d_n9, assign16040_e10968_d_n10, assign16040_e10968_d_n11, assign16040_e10968_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        let assign16040_e10966: f64 = (p.p14 * locals.var_js);
        (assign16040_e10966, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign16040_e10968;
        locals.var_isbs_btm_dn0 = assign16040_e10968_d_n0;
        locals.var_isbs_btm_dn2 = assign16040_e10968_d_n2;
        locals.var_isbs_btm_dn4 = assign16040_e10968_d_n4;
        locals.var_isbs_btm_dn5 = assign16040_e10968_d_n5;
        locals.var_isbs_btm_dn6 = assign16040_e10968_d_n6;
        locals.var_isbs_btm_dn7 = assign16040_e10968_d_n7;
        locals.var_isbs_btm_dn8 = assign16040_e10968_d_n8;
        locals.var_isbs_btm_dn9 = assign16040_e10968_d_n9;
        locals.var_isbs_btm_dn10 = assign16040_e10968_d_n10;
        locals.var_isbs_btm_dn11 = assign16040_e10968_d_n11;
        locals.var_isbs_btm_dn14 = assign16040_e10968_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign16050_e10979, assign16050_e10979_d_n0, assign16050_e10979_d_n2, assign16050_e10979_d_n4, assign16050_e10979_d_n5, assign16050_e10979_d_n6, assign16050_e10979_d_n7, assign16050_e10979_d_n8, assign16050_e10979_d_n9, assign16050_e10979_d_n10, assign16050_e10979_d_n11, assign16050_e10979_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        let assign16050_e10977: f64 = (p.p14 * locals.var_js2);
        (assign16050_e10977, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign16050_e10979;
        locals.var_isbs2_btm_dn0 = assign16050_e10979_d_n0;
        locals.var_isbs2_btm_dn2 = assign16050_e10979_d_n2;
        locals.var_isbs2_btm_dn4 = assign16050_e10979_d_n4;
        locals.var_isbs2_btm_dn5 = assign16050_e10979_d_n5;
        locals.var_isbs2_btm_dn6 = assign16050_e10979_d_n6;
        locals.var_isbs2_btm_dn7 = assign16050_e10979_d_n7;
        locals.var_isbs2_btm_dn8 = assign16050_e10979_d_n8;
        locals.var_isbs2_btm_dn9 = assign16050_e10979_d_n9;
        locals.var_isbs2_btm_dn10 = assign16050_e10979_d_n10;
        locals.var_isbs2_btm_dn11 = assign16050_e10979_d_n11;
        locals.var_isbs2_btm_dn14 = assign16050_e10979_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign16060_e10988, assign16060_e10988_d_n0, assign16060_e10988_d_n2, assign16060_e10988_d_n4, assign16060_e10988_d_n5, assign16060_e10988_d_n6, assign16060_e10988_d_n7, assign16060_e10988_d_n8, assign16060_e10988_d_n9, assign16060_e10988_d_n10, assign16060_e10988_d_n11, assign16060_e10988_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16060_e10988;
        locals.var_isbs_sws_dn0 = assign16060_e10988_d_n0;
        locals.var_isbs_sws_dn2 = assign16060_e10988_d_n2;
        locals.var_isbs_sws_dn4 = assign16060_e10988_d_n4;
        locals.var_isbs_sws_dn5 = assign16060_e10988_d_n5;
        locals.var_isbs_sws_dn6 = assign16060_e10988_d_n6;
        locals.var_isbs_sws_dn7 = assign16060_e10988_d_n7;
        locals.var_isbs_sws_dn8 = assign16060_e10988_d_n8;
        locals.var_isbs_sws_dn9 = assign16060_e10988_d_n9;
        locals.var_isbs_sws_dn10 = assign16060_e10988_d_n10;
        locals.var_isbs_sws_dn11 = assign16060_e10988_d_n11;
        locals.var_isbs_sws_dn14 = assign16060_e10988_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign16070_e10997, assign16070_e10997_d_n0, assign16070_e10997_d_n2, assign16070_e10997_d_n4, assign16070_e10997_d_n5, assign16070_e10997_d_n6, assign16070_e10997_d_n7, assign16070_e10997_d_n8, assign16070_e10997_d_n9, assign16070_e10997_d_n10, assign16070_e10997_d_n11, assign16070_e10997_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16070_e10997;
        locals.var_isbs2_sws_dn0 = assign16070_e10997_d_n0;
        locals.var_isbs2_sws_dn2 = assign16070_e10997_d_n2;
        locals.var_isbs2_sws_dn4 = assign16070_e10997_d_n4;
        locals.var_isbs2_sws_dn5 = assign16070_e10997_d_n5;
        locals.var_isbs2_sws_dn6 = assign16070_e10997_d_n6;
        locals.var_isbs2_sws_dn7 = assign16070_e10997_d_n7;
        locals.var_isbs2_sws_dn8 = assign16070_e10997_d_n8;
        locals.var_isbs2_sws_dn9 = assign16070_e10997_d_n9;
        locals.var_isbs2_sws_dn10 = assign16070_e10997_d_n10;
        locals.var_isbs2_sws_dn11 = assign16070_e10997_d_n11;
        locals.var_isbs2_sws_dn14 = assign16070_e10997_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16080_e11008, assign16080_e11008_d_n0, assign16080_e11008_d_n2, assign16080_e11008_d_n4, assign16080_e11008_d_n5, assign16080_e11008_d_n6, assign16080_e11008_d_n7, assign16080_e11008_d_n8, assign16080_e11008_d_n9, assign16080_e11008_d_n10, assign16080_e11008_d_n11, assign16080_e11008_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        let assign16080_e11006: f64 = (p.p16 * locals.var_jsswg);
        (assign16080_e11006, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16080_e11008;
        locals.var_isbs_swg_dn0 = assign16080_e11008_d_n0;
        locals.var_isbs_swg_dn2 = assign16080_e11008_d_n2;
        locals.var_isbs_swg_dn4 = assign16080_e11008_d_n4;
        locals.var_isbs_swg_dn5 = assign16080_e11008_d_n5;
        locals.var_isbs_swg_dn6 = assign16080_e11008_d_n6;
        locals.var_isbs_swg_dn7 = assign16080_e11008_d_n7;
        locals.var_isbs_swg_dn8 = assign16080_e11008_d_n8;
        locals.var_isbs_swg_dn9 = assign16080_e11008_d_n9;
        locals.var_isbs_swg_dn10 = assign16080_e11008_d_n10;
        locals.var_isbs_swg_dn11 = assign16080_e11008_d_n11;
        locals.var_isbs_swg_dn14 = assign16080_e11008_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16090_e11019, assign16090_e11019_d_n0, assign16090_e11019_d_n2, assign16090_e11019_d_n4, assign16090_e11019_d_n5, assign16090_e11019_d_n6, assign16090_e11019_d_n7, assign16090_e11019_d_n8, assign16090_e11019_d_n9, assign16090_e11019_d_n10, assign16090_e11019_d_n11, assign16090_e11019_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) {
        let assign16090_e11017: f64 = (p.p16 * locals.var_jsswg2);
        (assign16090_e11017, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16090_e11019;
        locals.var_isbs2_swg_dn0 = assign16090_e11019_d_n0;
        locals.var_isbs2_swg_dn2 = assign16090_e11019_d_n2;
        locals.var_isbs2_swg_dn4 = assign16090_e11019_d_n4;
        locals.var_isbs2_swg_dn5 = assign16090_e11019_d_n5;
        locals.var_isbs2_swg_dn6 = assign16090_e11019_d_n6;
        locals.var_isbs2_swg_dn7 = assign16090_e11019_d_n7;
        locals.var_isbs2_swg_dn8 = assign16090_e11019_d_n8;
        locals.var_isbs2_swg_dn9 = assign16090_e11019_d_n9;
        locals.var_isbs2_swg_dn10 = assign16090_e11019_d_n10;
        locals.var_isbs2_swg_dn11 = assign16090_e11019_d_n11;
        locals.var_isbs2_swg_dn14 = assign16090_e11019_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign16100_e11028, assign16100_e11028_d_n0, assign16100_e11028_d_n2, assign16100_e11028_d_n4, assign16100_e11028_d_n5, assign16100_e11028_d_n6, assign16100_e11028_d_n7, assign16100_e11028_d_n8, assign16100_e11028_d_n9, assign16100_e11028_d_n10, assign16100_e11028_d_n11, assign16100_e11028_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        let assign16100_e11026: f64 = (p.p14 * locals.var_js);
        (assign16100_e11026, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign16100_e11028;
        locals.var_isbs_btm_dn0 = assign16100_e11028_d_n0;
        locals.var_isbs_btm_dn2 = assign16100_e11028_d_n2;
        locals.var_isbs_btm_dn4 = assign16100_e11028_d_n4;
        locals.var_isbs_btm_dn5 = assign16100_e11028_d_n5;
        locals.var_isbs_btm_dn6 = assign16100_e11028_d_n6;
        locals.var_isbs_btm_dn7 = assign16100_e11028_d_n7;
        locals.var_isbs_btm_dn8 = assign16100_e11028_d_n8;
        locals.var_isbs_btm_dn9 = assign16100_e11028_d_n9;
        locals.var_isbs_btm_dn10 = assign16100_e11028_d_n10;
        locals.var_isbs_btm_dn11 = assign16100_e11028_d_n11;
        locals.var_isbs_btm_dn14 = assign16100_e11028_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign16110_e11037, assign16110_e11037_d_n0, assign16110_e11037_d_n2, assign16110_e11037_d_n4, assign16110_e11037_d_n5, assign16110_e11037_d_n6, assign16110_e11037_d_n7, assign16110_e11037_d_n8, assign16110_e11037_d_n9, assign16110_e11037_d_n10, assign16110_e11037_d_n11, assign16110_e11037_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        let assign16110_e11035: f64 = (p.p14 * locals.var_js2);
        (assign16110_e11035, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign16110_e11037;
        locals.var_isbs2_btm_dn0 = assign16110_e11037_d_n0;
        locals.var_isbs2_btm_dn2 = assign16110_e11037_d_n2;
        locals.var_isbs2_btm_dn4 = assign16110_e11037_d_n4;
        locals.var_isbs2_btm_dn5 = assign16110_e11037_d_n5;
        locals.var_isbs2_btm_dn6 = assign16110_e11037_d_n6;
        locals.var_isbs2_btm_dn7 = assign16110_e11037_d_n7;
        locals.var_isbs2_btm_dn8 = assign16110_e11037_d_n8;
        locals.var_isbs2_btm_dn9 = assign16110_e11037_d_n9;
        locals.var_isbs2_btm_dn10 = assign16110_e11037_d_n10;
        locals.var_isbs2_btm_dn11 = assign16110_e11037_d_n11;
        locals.var_isbs2_btm_dn14 = assign16110_e11037_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign16120_e11046, assign16120_e11046_d_n0, assign16120_e11046_d_n2, assign16120_e11046_d_n4, assign16120_e11046_d_n5, assign16120_e11046_d_n6, assign16120_e11046_d_n7, assign16120_e11046_d_n8, assign16120_e11046_d_n9, assign16120_e11046_d_n10, assign16120_e11046_d_n11, assign16120_e11046_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        let assign16120_e11044: f64 = (p.p16 * locals.var_jssw);
        (assign16120_e11044, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16120_e11046;
        locals.var_isbs_sws_dn0 = assign16120_e11046_d_n0;
        locals.var_isbs_sws_dn2 = assign16120_e11046_d_n2;
        locals.var_isbs_sws_dn4 = assign16120_e11046_d_n4;
        locals.var_isbs_sws_dn5 = assign16120_e11046_d_n5;
        locals.var_isbs_sws_dn6 = assign16120_e11046_d_n6;
        locals.var_isbs_sws_dn7 = assign16120_e11046_d_n7;
        locals.var_isbs_sws_dn8 = assign16120_e11046_d_n8;
        locals.var_isbs_sws_dn9 = assign16120_e11046_d_n9;
        locals.var_isbs_sws_dn10 = assign16120_e11046_d_n10;
        locals.var_isbs_sws_dn11 = assign16120_e11046_d_n11;
        locals.var_isbs_sws_dn14 = assign16120_e11046_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign16130_e11055, assign16130_e11055_d_n0, assign16130_e11055_d_n2, assign16130_e11055_d_n4, assign16130_e11055_d_n5, assign16130_e11055_d_n6, assign16130_e11055_d_n7, assign16130_e11055_d_n8, assign16130_e11055_d_n9, assign16130_e11055_d_n10, assign16130_e11055_d_n11, assign16130_e11055_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        let assign16130_e11053: f64 = (p.p16 * locals.var_jssw2);
        (assign16130_e11053, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16130_e11055;
        locals.var_isbs2_sws_dn0 = assign16130_e11055_d_n0;
        locals.var_isbs2_sws_dn2 = assign16130_e11055_d_n2;
        locals.var_isbs2_sws_dn4 = assign16130_e11055_d_n4;
        locals.var_isbs2_sws_dn5 = assign16130_e11055_d_n5;
        locals.var_isbs2_sws_dn6 = assign16130_e11055_d_n6;
        locals.var_isbs2_sws_dn7 = assign16130_e11055_d_n7;
        locals.var_isbs2_sws_dn8 = assign16130_e11055_d_n8;
        locals.var_isbs2_sws_dn9 = assign16130_e11055_d_n9;
        locals.var_isbs2_sws_dn10 = assign16130_e11055_d_n10;
        locals.var_isbs2_sws_dn11 = assign16130_e11055_d_n11;
        locals.var_isbs2_sws_dn14 = assign16130_e11055_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign16140_e11062, assign16140_e11062_d_n0, assign16140_e11062_d_n2, assign16140_e11062_d_n4, assign16140_e11062_d_n5, assign16140_e11062_d_n6, assign16140_e11062_d_n7, assign16140_e11062_d_n8, assign16140_e11062_d_n9, assign16140_e11062_d_n10, assign16140_e11062_d_n11, assign16140_e11062_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16140_e11062;
        locals.var_isbs_swg_dn0 = assign16140_e11062_d_n0;
        locals.var_isbs_swg_dn2 = assign16140_e11062_d_n2;
        locals.var_isbs_swg_dn4 = assign16140_e11062_d_n4;
        locals.var_isbs_swg_dn5 = assign16140_e11062_d_n5;
        locals.var_isbs_swg_dn6 = assign16140_e11062_d_n6;
        locals.var_isbs_swg_dn7 = assign16140_e11062_d_n7;
        locals.var_isbs_swg_dn8 = assign16140_e11062_d_n8;
        locals.var_isbs_swg_dn9 = assign16140_e11062_d_n9;
        locals.var_isbs_swg_dn10 = assign16140_e11062_d_n10;
        locals.var_isbs_swg_dn11 = assign16140_e11062_d_n11;
        locals.var_isbs_swg_dn14 = assign16140_e11062_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign16150_e11069, assign16150_e11069_d_n0, assign16150_e11069_d_n2, assign16150_e11069_d_n4, assign16150_e11069_d_n5, assign16150_e11069_d_n6, assign16150_e11069_d_n7, assign16150_e11069_d_n8, assign16150_e11069_d_n9, assign16150_e11069_d_n10, assign16150_e11069_d_n11, assign16150_e11069_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard334 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16150_e11069;
        locals.var_isbs2_swg_dn0 = assign16150_e11069_d_n0;
        locals.var_isbs2_swg_dn2 = assign16150_e11069_d_n2;
        locals.var_isbs2_swg_dn4 = assign16150_e11069_d_n4;
        locals.var_isbs2_swg_dn5 = assign16150_e11069_d_n5;
        locals.var_isbs2_swg_dn6 = assign16150_e11069_d_n6;
        locals.var_isbs2_swg_dn7 = assign16150_e11069_d_n7;
        locals.var_isbs2_swg_dn8 = assign16150_e11069_d_n8;
        locals.var_isbs2_swg_dn9 = assign16150_e11069_d_n9;
        locals.var_isbs2_swg_dn10 = assign16150_e11069_d_n10;
        locals.var_isbs2_swg_dn11 = assign16150_e11069_d_n11;
        locals.var_isbs2_swg_dn14 = assign16150_e11069_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign16160_e11077, assign16160_e11077_d_n0, assign16160_e11077_d_n2, assign16160_e11077_d_n4, assign16160_e11077_d_n5, assign16160_e11077_d_n6, assign16160_e11077_d_n7, assign16160_e11077_d_n8, assign16160_e11077_d_n9, assign16160_e11077_d_n10, assign16160_e11077_d_n11, assign16160_e11077_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16160_e11073: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign16160_e11075: f64 = (assign16160_e11073 + locals.var_isbs_swg);
        (assign16160_e11075, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign16160_e11077;
        locals.var_isbs_dn0 = assign16160_e11077_d_n0;
        locals.var_isbs_dn2 = assign16160_e11077_d_n2;
        locals.var_isbs_dn4 = assign16160_e11077_d_n4;
        locals.var_isbs_dn5 = assign16160_e11077_d_n5;
        locals.var_isbs_dn6 = assign16160_e11077_d_n6;
        locals.var_isbs_dn7 = assign16160_e11077_d_n7;
        locals.var_isbs_dn8 = assign16160_e11077_d_n8;
        locals.var_isbs_dn9 = assign16160_e11077_d_n9;
        locals.var_isbs_dn10 = assign16160_e11077_d_n10;
        locals.var_isbs_dn11 = assign16160_e11077_d_n11;
        locals.var_isbs_dn14 = assign16160_e11077_d_n14;
        locals.var_isbs_rv = 0.0;

        let assign16170_e11080: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign16170_e11080;
        locals.var_guard336_rv = 0.0;

        let (assign16180_e11088, assign16180_e11088_d_n0, assign16180_e11088_d_n2, assign16180_e11088_d_n4, assign16180_e11088_d_n5, assign16180_e11088_d_n6, assign16180_e11088_d_n7, assign16180_e11088_d_n8, assign16180_e11088_d_n9, assign16180_e11088_d_n10, assign16180_e11088_d_n11, assign16180_e11088_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard336 != 0.0)) {
        let assign16180_e11086: f64 = (locals.var_isbs + 1e-25);
        (assign16180_e11086, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign16180_e11088;
        locals.var_t3_dn0 = assign16180_e11088_d_n0;
        locals.var_t3_dn2 = assign16180_e11088_d_n2;
        locals.var_t3_dn4 = assign16180_e11088_d_n4;
        locals.var_t3_dn5 = assign16180_e11088_d_n5;
        locals.var_t3_dn6 = assign16180_e11088_d_n6;
        locals.var_t3_dn7 = assign16180_e11088_d_n7;
        locals.var_t3_dn8 = assign16180_e11088_d_n8;
        locals.var_t3_dn9 = assign16180_e11088_d_n9;
        locals.var_t3_dn10 = assign16180_e11088_d_n10;
        locals.var_t3_dn11 = assign16180_e11088_d_n11;
        locals.var_t3_dn14 = assign16180_e11088_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign16190_e11105, assign16190_e11105_d_n0, assign16190_e11105_d_n2, assign16190_e11105_d_n4, assign16190_e11105_d_n5, assign16190_e11105_d_n6, assign16190_e11105_d_n7, assign16190_e11105_d_n8, assign16190_e11105_d_n9, assign16190_e11105_d_n10, assign16190_e11105_d_n11, assign16190_e11105_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard336 != 0.0)) {
        let assign16190_e11094: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16190_e11097: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign16190_e11099: f64 = (assign16190_e11097 / locals.var_t3);
        let assign16190_e11101: f64 = (assign16190_e11099 + 1.0);
        let assign16190_e11102: f64 = (assign16190_e11101).ln();
        let assign16190_e11103: f64 = (assign16190_e11094 * assign16190_e11102);
        (assign16190_e11103, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign16190_e11102) + (assign16190_e11094 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign16190_e11097 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign16190_e11101))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign16190_e11105;
        locals.var_vbst_dn0 = assign16190_e11105_d_n0;
        locals.var_vbst_dn2 = assign16190_e11105_d_n2;
        locals.var_vbst_dn4 = assign16190_e11105_d_n4;
        locals.var_vbst_dn5 = assign16190_e11105_d_n5;
        locals.var_vbst_dn6 = assign16190_e11105_d_n6;
        locals.var_vbst_dn7 = assign16190_e11105_d_n7;
        locals.var_vbst_dn8 = assign16190_e11105_d_n8;
        locals.var_vbst_dn9 = assign16190_e11105_d_n9;
        locals.var_vbst_dn10 = assign16190_e11105_d_n10;
        locals.var_vbst_dn11 = assign16190_e11105_d_n11;
        locals.var_vbst_dn14 = assign16190_e11105_d_n14;
        locals.var_vbst_rv = 0.0;

        let (assign16200_e11116, assign16200_e11116_d_n0, assign16200_e11116_d_n2, assign16200_e11116_d_n4, assign16200_e11116_d_n5, assign16200_e11116_d_n6, assign16200_e11116_d_n7, assign16200_e11116_d_n8, assign16200_e11116_d_n9, assign16200_e11116_d_n10, assign16200_e11116_d_n11, assign16200_e11116_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard336 != 0.0)) {
        let assign16200_e11111: f64 = (locals.var_tratio - 1.0);
        let assign16200_e11113: f64 = (assign16200_e11111 * p.p535);
        let assign16200_e11114: f64 = (assign16200_e11113).exp();
        (assign16200_e11114, (assign16200_e11114 * (locals.var_tratio_dn0 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn2 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn4 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn5 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn6 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn7 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn8 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn9 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn10 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn11 * p.p535)), (assign16200_e11114 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign16200_e11116;
        locals.var_exptemps_dn0 = assign16200_e11116_d_n0;
        locals.var_exptemps_dn2 = assign16200_e11116_d_n2;
        locals.var_exptemps_dn4 = assign16200_e11116_d_n4;
        locals.var_exptemps_dn5 = assign16200_e11116_d_n5;
        locals.var_exptemps_dn6 = assign16200_e11116_d_n6;
        locals.var_exptemps_dn7 = assign16200_e11116_d_n7;
        locals.var_exptemps_dn8 = assign16200_e11116_d_n8;
        locals.var_exptemps_dn9 = assign16200_e11116_d_n9;
        locals.var_exptemps_dn10 = assign16200_e11116_d_n10;
        locals.var_exptemps_dn11 = assign16200_e11116_d_n11;
        locals.var_exptemps_dn14 = assign16200_e11116_d_n14;
        locals.var_exptemps_rv = 0.0;

        let (assign16210_e11126, assign16210_e11126_d_n0, assign16210_e11126_d_n2, assign16210_e11126_d_n4, assign16210_e11126_d_n5, assign16210_e11126_d_n6, assign16210_e11126_d_n7, assign16210_e11126_d_n8, assign16210_e11126_d_n9, assign16210_e11126_d_n10, assign16210_e11126_d_n11, assign16210_e11126_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard336 != 0.0)) {
        let assign16210_e11123: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16210_e11124: f64 = (1.0 / assign16210_e11123);
        (assign16210_e11124, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign16210_e11123 * assign16210_e11123))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign16210_e11126;
        locals.var_jd_nvtm_invs_dn0 = assign16210_e11126_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign16210_e11126_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign16210_e11126_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign16210_e11126_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign16210_e11126_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign16210_e11126_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign16210_e11126_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign16210_e11126_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign16210_e11126_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign16210_e11126_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign16210_e11126_d_n14;
        locals.var_jd_nvtm_invs_rv = 0.0;

        let (assign16220_e11135, assign16220_e11135_d_n0, assign16220_e11135_d_n2, assign16220_e11135_d_n4, assign16220_e11135_d_n5, assign16220_e11135_d_n6, assign16220_e11135_d_n7, assign16220_e11135_d_n8, assign16220_e11135_d_n9, assign16220_e11135_d_n10, assign16220_e11135_d_n11, assign16220_e11135_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard336 != 0.0)) {
        let assign16220_e11132: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign16220_e11133: f64 = (assign16220_e11132).exp();
        (assign16220_e11133, (assign16220_e11133 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign16220_e11133 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign16220_e11133 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign16220_e11133 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign16220_e11133 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign16220_e11133 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign16220_e11133 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign16220_e11133 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign16220_e11133 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign16220_e11133 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign16220_e11133 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign16220_e11135;
        locals.var_jd_expcs_dn0 = assign16220_e11135_d_n0;
        locals.var_jd_expcs_dn2 = assign16220_e11135_d_n2;
        locals.var_jd_expcs_dn4 = assign16220_e11135_d_n4;
        locals.var_jd_expcs_dn5 = assign16220_e11135_d_n5;
        locals.var_jd_expcs_dn6 = assign16220_e11135_d_n6;
        locals.var_jd_expcs_dn7 = assign16220_e11135_d_n7;
        locals.var_jd_expcs_dn8 = assign16220_e11135_d_n8;
        locals.var_jd_expcs_dn9 = assign16220_e11135_d_n9;
        locals.var_jd_expcs_dn10 = assign16220_e11135_d_n10;
        locals.var_jd_expcs_dn11 = assign16220_e11135_d_n11;
        locals.var_jd_expcs_dn14 = assign16220_e11135_d_n14;
        locals.var_jd_expcs_rv = 0.0;

        let (assign16230_e11147, assign16230_e11147_d_n0, assign16230_e11147_d_n2, assign16230_e11147_d_n4, assign16230_e11147_d_n5, assign16230_e11147_d_n6, assign16230_e11147_d_n7, assign16230_e11147_d_n8, assign16230_e11147_d_n9, assign16230_e11147_d_n10, assign16230_e11147_d_n11, assign16230_e11147_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16230_e11139: f64 = (p.p500 * p.p13);
        let assign16230_e11143: f64 = (p.p481 * locals.var_tdiff);
        let assign16230_e11144: f64 = (1.0 + assign16230_e11143);
        let assign16230_e11145: f64 = (assign16230_e11139 * assign16230_e11144);
        (assign16230_e11145, (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn0)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn2)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn4)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn5)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn6)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn7)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn8)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn9)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn10)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn11)), (assign16230_e11139 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign16230_e11147;
        locals.var_czbd_dn0 = assign16230_e11147_d_n0;
        locals.var_czbd_dn2 = assign16230_e11147_d_n2;
        locals.var_czbd_dn4 = assign16230_e11147_d_n4;
        locals.var_czbd_dn5 = assign16230_e11147_d_n5;
        locals.var_czbd_dn6 = assign16230_e11147_d_n6;
        locals.var_czbd_dn7 = assign16230_e11147_d_n7;
        locals.var_czbd_dn8 = assign16230_e11147_d_n8;
        locals.var_czbd_dn9 = assign16230_e11147_d_n9;
        locals.var_czbd_dn10 = assign16230_e11147_d_n10;
        locals.var_czbd_dn11 = assign16230_e11147_d_n11;
        locals.var_czbd_dn14 = assign16230_e11147_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign16240_e11150: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard337 = assign16240_e11150;
        locals.var_guard337_rv = 0.0;

        let (assign16250_e11166, assign16250_e11166_d_n0, assign16250_e11166_d_n2, assign16250_e11166_d_n4, assign16250_e11166_d_n5, assign16250_e11166_d_n6, assign16250_e11166_d_n7, assign16250_e11166_d_n8, assign16250_e11166_d_n9, assign16250_e11166_d_n10, assign16250_e11166_d_n11, assign16250_e11166_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard337 != 0.0)) {
        let assign16250_e11157: f64 = (p.p15 - locals.var_weff_nf);
        let assign16250_e11158: f64 = (p.p501 * assign16250_e11157);
        let assign16250_e11162: f64 = (p.p483 * locals.var_tdiff);
        let assign16250_e11163: f64 = (1.0 + assign16250_e11162);
        let assign16250_e11164: f64 = (assign16250_e11158 * assign16250_e11163);
        (assign16250_e11164, (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn0)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn2)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn4)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn5)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn6)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn7)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn8)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn9)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn10)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn11)), (assign16250_e11158 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16250_e11166;
        locals.var_czbdsw_dn0 = assign16250_e11166_d_n0;
        locals.var_czbdsw_dn2 = assign16250_e11166_d_n2;
        locals.var_czbdsw_dn4 = assign16250_e11166_d_n4;
        locals.var_czbdsw_dn5 = assign16250_e11166_d_n5;
        locals.var_czbdsw_dn6 = assign16250_e11166_d_n6;
        locals.var_czbdsw_dn7 = assign16250_e11166_d_n7;
        locals.var_czbdsw_dn8 = assign16250_e11166_d_n8;
        locals.var_czbdsw_dn9 = assign16250_e11166_d_n9;
        locals.var_czbdsw_dn10 = assign16250_e11166_d_n10;
        locals.var_czbdsw_dn11 = assign16250_e11166_d_n11;
        locals.var_czbdsw_dn14 = assign16250_e11166_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign16260_e11180, assign16260_e11180_d_n0, assign16260_e11180_d_n2, assign16260_e11180_d_n4, assign16260_e11180_d_n5, assign16260_e11180_d_n6, assign16260_e11180_d_n7, assign16260_e11180_d_n8, assign16260_e11180_d_n9, assign16260_e11180_d_n10, assign16260_e11180_d_n11, assign16260_e11180_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard337 != 0.0)) {
        let assign16260_e11172: f64 = (p.p502 * locals.var_weff_nf);
        let assign16260_e11176: f64 = (p.p485 * locals.var_tdiff);
        let assign16260_e11177: f64 = (1.0 + assign16260_e11176);
        let assign16260_e11178: f64 = (assign16260_e11172 * assign16260_e11177);
        (assign16260_e11178, (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn0)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn2)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn4)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn5)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn6)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn7)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn8)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn9)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn10)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn11)), (assign16260_e11172 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16260_e11180;
        locals.var_czbdswg_dn0 = assign16260_e11180_d_n0;
        locals.var_czbdswg_dn2 = assign16260_e11180_d_n2;
        locals.var_czbdswg_dn4 = assign16260_e11180_d_n4;
        locals.var_czbdswg_dn5 = assign16260_e11180_d_n5;
        locals.var_czbdswg_dn6 = assign16260_e11180_d_n6;
        locals.var_czbdswg_dn7 = assign16260_e11180_d_n7;
        locals.var_czbdswg_dn8 = assign16260_e11180_d_n8;
        locals.var_czbdswg_dn9 = assign16260_e11180_d_n9;
        locals.var_czbdswg_dn10 = assign16260_e11180_d_n10;
        locals.var_czbdswg_dn11 = assign16260_e11180_d_n11;
        locals.var_czbdswg_dn14 = assign16260_e11180_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign16270_e11187, assign16270_e11187_d_n0, assign16270_e11187_d_n2, assign16270_e11187_d_n4, assign16270_e11187_d_n5, assign16270_e11187_d_n6, assign16270_e11187_d_n7, assign16270_e11187_d_n8, assign16270_e11187_d_n9, assign16270_e11187_d_n10, assign16270_e11187_d_n11, assign16270_e11187_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard337 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16270_e11187;
        locals.var_czbdsw_dn0 = assign16270_e11187_d_n0;
        locals.var_czbdsw_dn2 = assign16270_e11187_d_n2;
        locals.var_czbdsw_dn4 = assign16270_e11187_d_n4;
        locals.var_czbdsw_dn5 = assign16270_e11187_d_n5;
        locals.var_czbdsw_dn6 = assign16270_e11187_d_n6;
        locals.var_czbdsw_dn7 = assign16270_e11187_d_n7;
        locals.var_czbdsw_dn8 = assign16270_e11187_d_n8;
        locals.var_czbdsw_dn9 = assign16270_e11187_d_n9;
        locals.var_czbdsw_dn10 = assign16270_e11187_d_n10;
        locals.var_czbdsw_dn11 = assign16270_e11187_d_n11;
        locals.var_czbdsw_dn14 = assign16270_e11187_d_n14;
        locals.var_czbdsw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16280_e11202, assign16280_e11202_d_n0, assign16280_e11202_d_n2, assign16280_e11202_d_n4, assign16280_e11202_d_n5, assign16280_e11202_d_n6, assign16280_e11202_d_n7, assign16280_e11202_d_n8, assign16280_e11202_d_n9, assign16280_e11202_d_n10, assign16280_e11202_d_n11, assign16280_e11202_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard337 == 0.0)) {
        let assign16280_e11194: f64 = (p.p502 * p.p15);
        let assign16280_e11198: f64 = (p.p485 * locals.var_tdiff);
        let assign16280_e11199: f64 = (1.0 + assign16280_e11198);
        let assign16280_e11200: f64 = (assign16280_e11194 * assign16280_e11199);
        (assign16280_e11200, (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn0)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn2)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn4)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn5)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn6)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn7)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn8)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn9)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn10)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn11)), (assign16280_e11194 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16280_e11202;
        locals.var_czbdswg_dn0 = assign16280_e11202_d_n0;
        locals.var_czbdswg_dn2 = assign16280_e11202_d_n2;
        locals.var_czbdswg_dn4 = assign16280_e11202_d_n4;
        locals.var_czbdswg_dn5 = assign16280_e11202_d_n5;
        locals.var_czbdswg_dn6 = assign16280_e11202_d_n6;
        locals.var_czbdswg_dn7 = assign16280_e11202_d_n7;
        locals.var_czbdswg_dn8 = assign16280_e11202_d_n8;
        locals.var_czbdswg_dn9 = assign16280_e11202_d_n9;
        locals.var_czbdswg_dn10 = assign16280_e11202_d_n10;
        locals.var_czbdswg_dn11 = assign16280_e11202_d_n11;
        locals.var_czbdswg_dn14 = assign16280_e11202_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let assign16290_e11205: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign16290_e11205;
        locals.var_guard338_rv = 0.0;

        let (assign16300_e11211, assign16300_e11211_d_n0, assign16300_e11211_d_n2, assign16300_e11211_d_n4, assign16300_e11211_d_n5, assign16300_e11211_d_n6, assign16300_e11211_d_n7, assign16300_e11211_d_n8, assign16300_e11211_d_n9, assign16300_e11211_d_n10, assign16300_e11211_d_n11, assign16300_e11211_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign16300_e11211;
        locals.var_czbd_dn0 = assign16300_e11211_d_n0;
        locals.var_czbd_dn2 = assign16300_e11211_d_n2;
        locals.var_czbd_dn4 = assign16300_e11211_d_n4;
        locals.var_czbd_dn5 = assign16300_e11211_d_n5;
        locals.var_czbd_dn6 = assign16300_e11211_d_n6;
        locals.var_czbd_dn7 = assign16300_e11211_d_n7;
        locals.var_czbd_dn8 = assign16300_e11211_d_n8;
        locals.var_czbd_dn9 = assign16300_e11211_d_n9;
        locals.var_czbd_dn10 = assign16300_e11211_d_n10;
        locals.var_czbd_dn11 = assign16300_e11211_d_n11;
        locals.var_czbd_dn14 = assign16300_e11211_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign16310_e11214: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard339 = assign16310_e11214;
        locals.var_guard339_rv = 0.0;

        let (assign16320_e11220, assign16320_e11220_d_n0, assign16320_e11220_d_n2, assign16320_e11220_d_n4, assign16320_e11220_d_n5, assign16320_e11220_d_n6, assign16320_e11220_d_n7, assign16320_e11220_d_n8, assign16320_e11220_d_n9, assign16320_e11220_d_n10, assign16320_e11220_d_n11, assign16320_e11220_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard339 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16320_e11220;
        locals.var_czbdsw_dn0 = assign16320_e11220_d_n0;
        locals.var_czbdsw_dn2 = assign16320_e11220_d_n2;
        locals.var_czbdsw_dn4 = assign16320_e11220_d_n4;
        locals.var_czbdsw_dn5 = assign16320_e11220_d_n5;
        locals.var_czbdsw_dn6 = assign16320_e11220_d_n6;
        locals.var_czbdsw_dn7 = assign16320_e11220_d_n7;
        locals.var_czbdsw_dn8 = assign16320_e11220_d_n8;
        locals.var_czbdsw_dn9 = assign16320_e11220_d_n9;
        locals.var_czbdsw_dn10 = assign16320_e11220_d_n10;
        locals.var_czbdsw_dn11 = assign16320_e11220_d_n11;
        locals.var_czbdsw_dn14 = assign16320_e11220_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let assign16330_e11223: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard340 = assign16330_e11223;
        locals.var_guard340_rv = 0.0;

        let (assign16340_e11229, assign16340_e11229_d_n0, assign16340_e11229_d_n2, assign16340_e11229_d_n4, assign16340_e11229_d_n5, assign16340_e11229_d_n6, assign16340_e11229_d_n7, assign16340_e11229_d_n8, assign16340_e11229_d_n9, assign16340_e11229_d_n10, assign16340_e11229_d_n11, assign16340_e11229_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard340 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16340_e11229;
        locals.var_czbdswg_dn0 = assign16340_e11229_d_n0;
        locals.var_czbdswg_dn2 = assign16340_e11229_d_n2;
        locals.var_czbdswg_dn4 = assign16340_e11229_d_n4;
        locals.var_czbdswg_dn5 = assign16340_e11229_d_n5;
        locals.var_czbdswg_dn6 = assign16340_e11229_d_n6;
        locals.var_czbdswg_dn7 = assign16340_e11229_d_n7;
        locals.var_czbdswg_dn8 = assign16340_e11229_d_n8;
        locals.var_czbdswg_dn9 = assign16340_e11229_d_n9;
        locals.var_czbdswg_dn10 = assign16340_e11229_d_n10;
        locals.var_czbdswg_dn11 = assign16340_e11229_d_n11;
        locals.var_czbdswg_dn14 = assign16340_e11229_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign16350_e11237, assign16350_e11237_d_n0, assign16350_e11237_d_n2, assign16350_e11237_d_n4, assign16350_e11237_d_n5, assign16350_e11237_d_n6, assign16350_e11237_d_n7, assign16350_e11237_d_n8, assign16350_e11237_d_n9, assign16350_e11237_d_n10, assign16350_e11237_d_n11, assign16350_e11237_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16350_e11234: f64 = (p.p487 * locals.var_tdiff);
        let assign16350_e11235: f64 = (p.p506 - assign16350_e11234);
        (assign16350_e11235, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign16350_e11237;
        locals.var_pzbd_dn0 = assign16350_e11237_d_n0;
        locals.var_pzbd_dn2 = assign16350_e11237_d_n2;
        locals.var_pzbd_dn4 = assign16350_e11237_d_n4;
        locals.var_pzbd_dn5 = assign16350_e11237_d_n5;
        locals.var_pzbd_dn6 = assign16350_e11237_d_n6;
        locals.var_pzbd_dn7 = assign16350_e11237_d_n7;
        locals.var_pzbd_dn8 = assign16350_e11237_d_n8;
        locals.var_pzbd_dn9 = assign16350_e11237_d_n9;
        locals.var_pzbd_dn10 = assign16350_e11237_d_n10;
        locals.var_pzbd_dn11 = assign16350_e11237_d_n11;
        locals.var_pzbd_dn14 = assign16350_e11237_d_n14;
        locals.var_pzbd_rv = 0.0;

        let (assign16360_e11245, assign16360_e11245_d_n0, assign16360_e11245_d_n2, assign16360_e11245_d_n4, assign16360_e11245_d_n5, assign16360_e11245_d_n6, assign16360_e11245_d_n7, assign16360_e11245_d_n8, assign16360_e11245_d_n9, assign16360_e11245_d_n10, assign16360_e11245_d_n11, assign16360_e11245_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16360_e11242: f64 = (p.p489 * locals.var_tdiff);
        let assign16360_e11243: f64 = (p.p507 - assign16360_e11242);
        (assign16360_e11243, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign16360_e11245;
        locals.var_pzbdsw_dn0 = assign16360_e11245_d_n0;
        locals.var_pzbdsw_dn2 = assign16360_e11245_d_n2;
        locals.var_pzbdsw_dn4 = assign16360_e11245_d_n4;
        locals.var_pzbdsw_dn5 = assign16360_e11245_d_n5;
        locals.var_pzbdsw_dn6 = assign16360_e11245_d_n6;
        locals.var_pzbdsw_dn7 = assign16360_e11245_d_n7;
        locals.var_pzbdsw_dn8 = assign16360_e11245_d_n8;
        locals.var_pzbdsw_dn9 = assign16360_e11245_d_n9;
        locals.var_pzbdsw_dn10 = assign16360_e11245_d_n10;
        locals.var_pzbdsw_dn11 = assign16360_e11245_d_n11;
        locals.var_pzbdsw_dn14 = assign16360_e11245_d_n14;
        locals.var_pzbdsw_rv = 0.0;

        let (assign16370_e11253, assign16370_e11253_d_n0, assign16370_e11253_d_n2, assign16370_e11253_d_n4, assign16370_e11253_d_n5, assign16370_e11253_d_n6, assign16370_e11253_d_n7, assign16370_e11253_d_n8, assign16370_e11253_d_n9, assign16370_e11253_d_n10, assign16370_e11253_d_n11, assign16370_e11253_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16370_e11250: f64 = (p.p491 * locals.var_tdiff);
        let assign16370_e11251: f64 = (p.p508 - assign16370_e11250);
        (assign16370_e11251, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign16370_e11253;
        locals.var_pzbdswg_dn0 = assign16370_e11253_d_n0;
        locals.var_pzbdswg_dn2 = assign16370_e11253_d_n2;
        locals.var_pzbdswg_dn4 = assign16370_e11253_d_n4;
        locals.var_pzbdswg_dn5 = assign16370_e11253_d_n5;
        locals.var_pzbdswg_dn6 = assign16370_e11253_d_n6;
        locals.var_pzbdswg_dn7 = assign16370_e11253_d_n7;
        locals.var_pzbdswg_dn8 = assign16370_e11253_d_n8;
        locals.var_pzbdswg_dn9 = assign16370_e11253_d_n9;
        locals.var_pzbdswg_dn10 = assign16370_e11253_d_n10;
        locals.var_pzbdswg_dn11 = assign16370_e11253_d_n11;
        locals.var_pzbdswg_dn14 = assign16370_e11253_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let assign16380_e11260: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign16380_e11260;
        locals.var_guard341_rv = 0.0;

        let (assign16390_e11266, assign16390_e11266_d_n0, assign16390_e11266_d_n2, assign16390_e11266_d_n4, assign16390_e11266_d_n5, assign16390_e11266_d_n6, assign16390_e11266_d_n7, assign16390_e11266_d_n8, assign16390_e11266_d_n9, assign16390_e11266_d_n10, assign16390_e11266_d_n11, assign16390_e11266_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard341 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign16390_e11266;
        locals.var_pzbd_dn0 = assign16390_e11266_d_n0;
        locals.var_pzbd_dn2 = assign16390_e11266_d_n2;
        locals.var_pzbd_dn4 = assign16390_e11266_d_n4;
        locals.var_pzbd_dn5 = assign16390_e11266_d_n5;
        locals.var_pzbd_dn6 = assign16390_e11266_d_n6;
        locals.var_pzbd_dn7 = assign16390_e11266_d_n7;
        locals.var_pzbd_dn8 = assign16390_e11266_d_n8;
        locals.var_pzbd_dn9 = assign16390_e11266_d_n9;
        locals.var_pzbd_dn10 = assign16390_e11266_d_n10;
        locals.var_pzbd_dn11 = assign16390_e11266_d_n11;
        locals.var_pzbd_dn14 = assign16390_e11266_d_n14;
        locals.var_pzbd_rv = 0.0;

        let assign16400_e11273: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard342 = assign16400_e11273;
        locals.var_guard342_rv = 0.0;

        let (assign16410_e11279, assign16410_e11279_d_n0, assign16410_e11279_d_n2, assign16410_e11279_d_n4, assign16410_e11279_d_n5, assign16410_e11279_d_n6, assign16410_e11279_d_n7, assign16410_e11279_d_n8, assign16410_e11279_d_n9, assign16410_e11279_d_n10, assign16410_e11279_d_n11, assign16410_e11279_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard342 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign16410_e11279;
        locals.var_pzbdsw_dn0 = assign16410_e11279_d_n0;
        locals.var_pzbdsw_dn2 = assign16410_e11279_d_n2;
        locals.var_pzbdsw_dn4 = assign16410_e11279_d_n4;
        locals.var_pzbdsw_dn5 = assign16410_e11279_d_n5;
        locals.var_pzbdsw_dn6 = assign16410_e11279_d_n6;
        locals.var_pzbdsw_dn7 = assign16410_e11279_d_n7;
        locals.var_pzbdsw_dn8 = assign16410_e11279_d_n8;
        locals.var_pzbdsw_dn9 = assign16410_e11279_d_n9;
        locals.var_pzbdsw_dn10 = assign16410_e11279_d_n10;
        locals.var_pzbdsw_dn11 = assign16410_e11279_d_n11;
        locals.var_pzbdsw_dn14 = assign16410_e11279_d_n14;
        locals.var_pzbdsw_rv = 0.0;

        let assign16420_e11286: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard343 = assign16420_e11286;
        locals.var_guard343_rv = 0.0;

        let (assign16430_e11292, assign16430_e11292_d_n0, assign16430_e11292_d_n2, assign16430_e11292_d_n4, assign16430_e11292_d_n5, assign16430_e11292_d_n6, assign16430_e11292_d_n7, assign16430_e11292_d_n8, assign16430_e11292_d_n9, assign16430_e11292_d_n10, assign16430_e11292_d_n11, assign16430_e11292_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard343 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign16430_e11292;
        locals.var_pzbdswg_dn0 = assign16430_e11292_d_n0;
        locals.var_pzbdswg_dn2 = assign16430_e11292_d_n2;
        locals.var_pzbdswg_dn4 = assign16430_e11292_d_n4;
        locals.var_pzbdswg_dn5 = assign16430_e11292_d_n5;
        locals.var_pzbdswg_dn6 = assign16430_e11292_d_n6;
        locals.var_pzbdswg_dn7 = assign16430_e11292_d_n7;
        locals.var_pzbdswg_dn8 = assign16430_e11292_d_n8;
        locals.var_pzbdswg_dn9 = assign16430_e11292_d_n9;
        locals.var_pzbdswg_dn10 = assign16430_e11292_d_n10;
        locals.var_pzbdswg_dn11 = assign16430_e11292_d_n11;
        locals.var_pzbdswg_dn14 = assign16430_e11292_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let (assign16440_e11304, assign16440_e11304_d_n0, assign16440_e11304_d_n2, assign16440_e11304_d_n4, assign16440_e11304_d_n5, assign16440_e11304_d_n6, assign16440_e11304_d_n7, assign16440_e11304_d_n8, assign16440_e11304_d_n9, assign16440_e11304_d_n10, assign16440_e11304_d_n11, assign16440_e11304_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16440_e11296: f64 = (p.p523 * p.p14);
        let assign16440_e11300: f64 = (p.p482 * locals.var_tdiff);
        let assign16440_e11301: f64 = (1.0 + assign16440_e11300);
        let assign16440_e11302: f64 = (assign16440_e11296 * assign16440_e11301);
        (assign16440_e11302, (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn0)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn2)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn4)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn5)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn6)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn7)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn8)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn9)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn10)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn11)), (assign16440_e11296 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign16440_e11304;
        locals.var_czbs_dn0 = assign16440_e11304_d_n0;
        locals.var_czbs_dn2 = assign16440_e11304_d_n2;
        locals.var_czbs_dn4 = assign16440_e11304_d_n4;
        locals.var_czbs_dn5 = assign16440_e11304_d_n5;
        locals.var_czbs_dn6 = assign16440_e11304_d_n6;
        locals.var_czbs_dn7 = assign16440_e11304_d_n7;
        locals.var_czbs_dn8 = assign16440_e11304_d_n8;
        locals.var_czbs_dn9 = assign16440_e11304_d_n9;
        locals.var_czbs_dn10 = assign16440_e11304_d_n10;
        locals.var_czbs_dn11 = assign16440_e11304_d_n11;
        locals.var_czbs_dn14 = assign16440_e11304_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign16450_e11307: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard344 = assign16450_e11307;
        locals.var_guard344_rv = 0.0;

        let (assign16460_e11323, assign16460_e11323_d_n0, assign16460_e11323_d_n2, assign16460_e11323_d_n4, assign16460_e11323_d_n5, assign16460_e11323_d_n6, assign16460_e11323_d_n7, assign16460_e11323_d_n8, assign16460_e11323_d_n9, assign16460_e11323_d_n10, assign16460_e11323_d_n11, assign16460_e11323_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard344 != 0.0)) {
        let assign16460_e11314: f64 = (p.p16 - locals.var_weff_nf);
        let assign16460_e11315: f64 = (p.p524 * assign16460_e11314);
        let assign16460_e11319: f64 = (p.p484 * locals.var_tdiff);
        let assign16460_e11320: f64 = (1.0 + assign16460_e11319);
        let assign16460_e11321: f64 = (assign16460_e11315 * assign16460_e11320);
        (assign16460_e11321, (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn0)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn2)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn4)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn5)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn6)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn7)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn8)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn9)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn10)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn11)), (assign16460_e11315 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16460_e11323;
        locals.var_czbssw_dn0 = assign16460_e11323_d_n0;
        locals.var_czbssw_dn2 = assign16460_e11323_d_n2;
        locals.var_czbssw_dn4 = assign16460_e11323_d_n4;
        locals.var_czbssw_dn5 = assign16460_e11323_d_n5;
        locals.var_czbssw_dn6 = assign16460_e11323_d_n6;
        locals.var_czbssw_dn7 = assign16460_e11323_d_n7;
        locals.var_czbssw_dn8 = assign16460_e11323_d_n8;
        locals.var_czbssw_dn9 = assign16460_e11323_d_n9;
        locals.var_czbssw_dn10 = assign16460_e11323_d_n10;
        locals.var_czbssw_dn11 = assign16460_e11323_d_n11;
        locals.var_czbssw_dn14 = assign16460_e11323_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign16470_e11337, assign16470_e11337_d_n0, assign16470_e11337_d_n2, assign16470_e11337_d_n4, assign16470_e11337_d_n5, assign16470_e11337_d_n6, assign16470_e11337_d_n7, assign16470_e11337_d_n8, assign16470_e11337_d_n9, assign16470_e11337_d_n10, assign16470_e11337_d_n11, assign16470_e11337_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard344 != 0.0)) {
        let assign16470_e11329: f64 = (p.p525 * locals.var_weff_nf);
        let assign16470_e11333: f64 = (p.p486 * locals.var_tdiff);
        let assign16470_e11334: f64 = (1.0 + assign16470_e11333);
        let assign16470_e11335: f64 = (assign16470_e11329 * assign16470_e11334);
        (assign16470_e11335, (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn0)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn2)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn4)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn5)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn6)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn7)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn8)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn9)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn10)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn11)), (assign16470_e11329 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16470_e11337;
        locals.var_czbsswg_dn0 = assign16470_e11337_d_n0;
        locals.var_czbsswg_dn2 = assign16470_e11337_d_n2;
        locals.var_czbsswg_dn4 = assign16470_e11337_d_n4;
        locals.var_czbsswg_dn5 = assign16470_e11337_d_n5;
        locals.var_czbsswg_dn6 = assign16470_e11337_d_n6;
        locals.var_czbsswg_dn7 = assign16470_e11337_d_n7;
        locals.var_czbsswg_dn8 = assign16470_e11337_d_n8;
        locals.var_czbsswg_dn9 = assign16470_e11337_d_n9;
        locals.var_czbsswg_dn10 = assign16470_e11337_d_n10;
        locals.var_czbsswg_dn11 = assign16470_e11337_d_n11;
        locals.var_czbsswg_dn14 = assign16470_e11337_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign16480_e11344, assign16480_e11344_d_n0, assign16480_e11344_d_n2, assign16480_e11344_d_n4, assign16480_e11344_d_n5, assign16480_e11344_d_n6, assign16480_e11344_d_n7, assign16480_e11344_d_n8, assign16480_e11344_d_n9, assign16480_e11344_d_n10, assign16480_e11344_d_n11, assign16480_e11344_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard344 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16480_e11344;
        locals.var_czbssw_dn0 = assign16480_e11344_d_n0;
        locals.var_czbssw_dn2 = assign16480_e11344_d_n2;
        locals.var_czbssw_dn4 = assign16480_e11344_d_n4;
        locals.var_czbssw_dn5 = assign16480_e11344_d_n5;
        locals.var_czbssw_dn6 = assign16480_e11344_d_n6;
        locals.var_czbssw_dn7 = assign16480_e11344_d_n7;
        locals.var_czbssw_dn8 = assign16480_e11344_d_n8;
        locals.var_czbssw_dn9 = assign16480_e11344_d_n9;
        locals.var_czbssw_dn10 = assign16480_e11344_d_n10;
        locals.var_czbssw_dn11 = assign16480_e11344_d_n11;
        locals.var_czbssw_dn14 = assign16480_e11344_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign16490_e11359, assign16490_e11359_d_n0, assign16490_e11359_d_n2, assign16490_e11359_d_n4, assign16490_e11359_d_n5, assign16490_e11359_d_n6, assign16490_e11359_d_n7, assign16490_e11359_d_n8, assign16490_e11359_d_n9, assign16490_e11359_d_n10, assign16490_e11359_d_n11, assign16490_e11359_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard344 == 0.0)) {
        let assign16490_e11351: f64 = (p.p525 * p.p16);
        let assign16490_e11355: f64 = (p.p486 * locals.var_tdiff);
        let assign16490_e11356: f64 = (1.0 + assign16490_e11355);
        let assign16490_e11357: f64 = (assign16490_e11351 * assign16490_e11356);
        (assign16490_e11357, (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn0)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn2)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn4)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn5)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn6)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn7)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn8)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn9)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn10)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn11)), (assign16490_e11351 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16490_e11359;
        locals.var_czbsswg_dn0 = assign16490_e11359_d_n0;
        locals.var_czbsswg_dn2 = assign16490_e11359_d_n2;
        locals.var_czbsswg_dn4 = assign16490_e11359_d_n4;
        locals.var_czbsswg_dn5 = assign16490_e11359_d_n5;
        locals.var_czbsswg_dn6 = assign16490_e11359_d_n6;
        locals.var_czbsswg_dn7 = assign16490_e11359_d_n7;
        locals.var_czbsswg_dn8 = assign16490_e11359_d_n8;
        locals.var_czbsswg_dn9 = assign16490_e11359_d_n9;
        locals.var_czbsswg_dn10 = assign16490_e11359_d_n10;
        locals.var_czbsswg_dn11 = assign16490_e11359_d_n11;
        locals.var_czbsswg_dn14 = assign16490_e11359_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let assign16500_e11362: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign16500_e11362;
        locals.var_guard345_rv = 0.0;

        let (assign16510_e11368, assign16510_e11368_d_n0, assign16510_e11368_d_n2, assign16510_e11368_d_n4, assign16510_e11368_d_n5, assign16510_e11368_d_n6, assign16510_e11368_d_n7, assign16510_e11368_d_n8, assign16510_e11368_d_n9, assign16510_e11368_d_n10, assign16510_e11368_d_n11, assign16510_e11368_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign16510_e11368;
        locals.var_czbs_dn0 = assign16510_e11368_d_n0;
        locals.var_czbs_dn2 = assign16510_e11368_d_n2;
        locals.var_czbs_dn4 = assign16510_e11368_d_n4;
        locals.var_czbs_dn5 = assign16510_e11368_d_n5;
        locals.var_czbs_dn6 = assign16510_e11368_d_n6;
        locals.var_czbs_dn7 = assign16510_e11368_d_n7;
        locals.var_czbs_dn8 = assign16510_e11368_d_n8;
        locals.var_czbs_dn9 = assign16510_e11368_d_n9;
        locals.var_czbs_dn10 = assign16510_e11368_d_n10;
        locals.var_czbs_dn11 = assign16510_e11368_d_n11;
        locals.var_czbs_dn14 = assign16510_e11368_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign16520_e11371: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign16520_e11371;
        locals.var_guard346_rv = 0.0;

        let (assign16530_e11377, assign16530_e11377_d_n0, assign16530_e11377_d_n2, assign16530_e11377_d_n4, assign16530_e11377_d_n5, assign16530_e11377_d_n6, assign16530_e11377_d_n7, assign16530_e11377_d_n8, assign16530_e11377_d_n9, assign16530_e11377_d_n10, assign16530_e11377_d_n11, assign16530_e11377_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16530_e11377;
        locals.var_czbssw_dn0 = assign16530_e11377_d_n0;
        locals.var_czbssw_dn2 = assign16530_e11377_d_n2;
        locals.var_czbssw_dn4 = assign16530_e11377_d_n4;
        locals.var_czbssw_dn5 = assign16530_e11377_d_n5;
        locals.var_czbssw_dn6 = assign16530_e11377_d_n6;
        locals.var_czbssw_dn7 = assign16530_e11377_d_n7;
        locals.var_czbssw_dn8 = assign16530_e11377_d_n8;
        locals.var_czbssw_dn9 = assign16530_e11377_d_n9;
        locals.var_czbssw_dn10 = assign16530_e11377_d_n10;
        locals.var_czbssw_dn11 = assign16530_e11377_d_n11;
        locals.var_czbssw_dn14 = assign16530_e11377_d_n14;
        locals.var_czbssw_rv = 0.0;

        let assign16540_e11380: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign16540_e11380;
        locals.var_guard347_rv = 0.0;

        let (assign16550_e11386, assign16550_e11386_d_n0, assign16550_e11386_d_n2, assign16550_e11386_d_n4, assign16550_e11386_d_n5, assign16550_e11386_d_n6, assign16550_e11386_d_n7, assign16550_e11386_d_n8, assign16550_e11386_d_n9, assign16550_e11386_d_n10, assign16550_e11386_d_n11, assign16550_e11386_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16550_e11386;
        locals.var_czbsswg_dn0 = assign16550_e11386_d_n0;
        locals.var_czbsswg_dn2 = assign16550_e11386_d_n2;
        locals.var_czbsswg_dn4 = assign16550_e11386_d_n4;
        locals.var_czbsswg_dn5 = assign16550_e11386_d_n5;
        locals.var_czbsswg_dn6 = assign16550_e11386_d_n6;
        locals.var_czbsswg_dn7 = assign16550_e11386_d_n7;
        locals.var_czbsswg_dn8 = assign16550_e11386_d_n8;
        locals.var_czbsswg_dn9 = assign16550_e11386_d_n9;
        locals.var_czbsswg_dn10 = assign16550_e11386_d_n10;
        locals.var_czbsswg_dn11 = assign16550_e11386_d_n11;
        locals.var_czbsswg_dn14 = assign16550_e11386_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign16560_e11394, assign16560_e11394_d_n0, assign16560_e11394_d_n2, assign16560_e11394_d_n4, assign16560_e11394_d_n5, assign16560_e11394_d_n6, assign16560_e11394_d_n7, assign16560_e11394_d_n8, assign16560_e11394_d_n9, assign16560_e11394_d_n10, assign16560_e11394_d_n11, assign16560_e11394_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16560_e11391: f64 = (p.p488 * locals.var_tdiff);
        let assign16560_e11392: f64 = (p.p529 - assign16560_e11391);
        (assign16560_e11392, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign16560_e11394;
        locals.var_pzbs_dn0 = assign16560_e11394_d_n0;
        locals.var_pzbs_dn2 = assign16560_e11394_d_n2;
        locals.var_pzbs_dn4 = assign16560_e11394_d_n4;
        locals.var_pzbs_dn5 = assign16560_e11394_d_n5;
        locals.var_pzbs_dn6 = assign16560_e11394_d_n6;
        locals.var_pzbs_dn7 = assign16560_e11394_d_n7;
        locals.var_pzbs_dn8 = assign16560_e11394_d_n8;
        locals.var_pzbs_dn9 = assign16560_e11394_d_n9;
        locals.var_pzbs_dn10 = assign16560_e11394_d_n10;
        locals.var_pzbs_dn11 = assign16560_e11394_d_n11;
        locals.var_pzbs_dn14 = assign16560_e11394_d_n14;
        locals.var_pzbs_rv = 0.0;

        let (assign16570_e11402, assign16570_e11402_d_n0, assign16570_e11402_d_n2, assign16570_e11402_d_n4, assign16570_e11402_d_n5, assign16570_e11402_d_n6, assign16570_e11402_d_n7, assign16570_e11402_d_n8, assign16570_e11402_d_n9, assign16570_e11402_d_n10, assign16570_e11402_d_n11, assign16570_e11402_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16570_e11399: f64 = (p.p490 * locals.var_tdiff);
        let assign16570_e11400: f64 = (p.p530 - assign16570_e11399);
        (assign16570_e11400, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign16570_e11402;
        locals.var_pzbssw_dn0 = assign16570_e11402_d_n0;
        locals.var_pzbssw_dn2 = assign16570_e11402_d_n2;
        locals.var_pzbssw_dn4 = assign16570_e11402_d_n4;
        locals.var_pzbssw_dn5 = assign16570_e11402_d_n5;
        locals.var_pzbssw_dn6 = assign16570_e11402_d_n6;
        locals.var_pzbssw_dn7 = assign16570_e11402_d_n7;
        locals.var_pzbssw_dn8 = assign16570_e11402_d_n8;
        locals.var_pzbssw_dn9 = assign16570_e11402_d_n9;
        locals.var_pzbssw_dn10 = assign16570_e11402_d_n10;
        locals.var_pzbssw_dn11 = assign16570_e11402_d_n11;
        locals.var_pzbssw_dn14 = assign16570_e11402_d_n14;
        locals.var_pzbssw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (assign16580_e11410, assign16580_e11410_d_n0, assign16580_e11410_d_n2, assign16580_e11410_d_n4, assign16580_e11410_d_n5, assign16580_e11410_d_n6, assign16580_e11410_d_n7, assign16580_e11410_d_n8, assign16580_e11410_d_n9, assign16580_e11410_d_n10, assign16580_e11410_d_n11, assign16580_e11410_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign16580_e11407: f64 = (p.p492 * locals.var_tdiff);
        let assign16580_e11408: f64 = (p.p531 - assign16580_e11407);
        (assign16580_e11408, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign16580_e11410;
        locals.var_pzbsswg_dn0 = assign16580_e11410_d_n0;
        locals.var_pzbsswg_dn2 = assign16580_e11410_d_n2;
        locals.var_pzbsswg_dn4 = assign16580_e11410_d_n4;
        locals.var_pzbsswg_dn5 = assign16580_e11410_d_n5;
        locals.var_pzbsswg_dn6 = assign16580_e11410_d_n6;
        locals.var_pzbsswg_dn7 = assign16580_e11410_d_n7;
        locals.var_pzbsswg_dn8 = assign16580_e11410_d_n8;
        locals.var_pzbsswg_dn9 = assign16580_e11410_d_n9;
        locals.var_pzbsswg_dn10 = assign16580_e11410_d_n10;
        locals.var_pzbsswg_dn11 = assign16580_e11410_d_n11;
        locals.var_pzbsswg_dn14 = assign16580_e11410_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let assign16590_e11417: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign16590_e11417;
        locals.var_guard348_rv = 0.0;

        let (assign16600_e11423, assign16600_e11423_d_n0, assign16600_e11423_d_n2, assign16600_e11423_d_n4, assign16600_e11423_d_n5, assign16600_e11423_d_n6, assign16600_e11423_d_n7, assign16600_e11423_d_n8, assign16600_e11423_d_n9, assign16600_e11423_d_n10, assign16600_e11423_d_n11, assign16600_e11423_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard348 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign16600_e11423;
        locals.var_pzbs_dn0 = assign16600_e11423_d_n0;
        locals.var_pzbs_dn2 = assign16600_e11423_d_n2;
        locals.var_pzbs_dn4 = assign16600_e11423_d_n4;
        locals.var_pzbs_dn5 = assign16600_e11423_d_n5;
        locals.var_pzbs_dn6 = assign16600_e11423_d_n6;
        locals.var_pzbs_dn7 = assign16600_e11423_d_n7;
        locals.var_pzbs_dn8 = assign16600_e11423_d_n8;
        locals.var_pzbs_dn9 = assign16600_e11423_d_n9;
        locals.var_pzbs_dn10 = assign16600_e11423_d_n10;
        locals.var_pzbs_dn11 = assign16600_e11423_d_n11;
        locals.var_pzbs_dn14 = assign16600_e11423_d_n14;
        locals.var_pzbs_rv = 0.0;

        let assign16610_e11430: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard349 = assign16610_e11430;
        locals.var_guard349_rv = 0.0;

        let (assign16620_e11436, assign16620_e11436_d_n0, assign16620_e11436_d_n2, assign16620_e11436_d_n4, assign16620_e11436_d_n5, assign16620_e11436_d_n6, assign16620_e11436_d_n7, assign16620_e11436_d_n8, assign16620_e11436_d_n9, assign16620_e11436_d_n10, assign16620_e11436_d_n11, assign16620_e11436_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard349 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign16620_e11436;
        locals.var_pzbssw_dn0 = assign16620_e11436_d_n0;
        locals.var_pzbssw_dn2 = assign16620_e11436_d_n2;
        locals.var_pzbssw_dn4 = assign16620_e11436_d_n4;
        locals.var_pzbssw_dn5 = assign16620_e11436_d_n5;
        locals.var_pzbssw_dn6 = assign16620_e11436_d_n6;
        locals.var_pzbssw_dn7 = assign16620_e11436_d_n7;
        locals.var_pzbssw_dn8 = assign16620_e11436_d_n8;
        locals.var_pzbssw_dn9 = assign16620_e11436_d_n9;
        locals.var_pzbssw_dn10 = assign16620_e11436_d_n10;
        locals.var_pzbssw_dn11 = assign16620_e11436_d_n11;
        locals.var_pzbssw_dn14 = assign16620_e11436_d_n14;
        locals.var_pzbssw_rv = 0.0;

        let assign16630_e11443: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard350 = assign16630_e11443;
        locals.var_guard350_rv = 0.0;

        let (assign16640_e11449, assign16640_e11449_d_n0, assign16640_e11449_d_n2, assign16640_e11449_d_n4, assign16640_e11449_d_n5, assign16640_e11449_d_n6, assign16640_e11449_d_n7, assign16640_e11449_d_n8, assign16640_e11449_d_n9, assign16640_e11449_d_n10, assign16640_e11449_d_n11, assign16640_e11449_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard350 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign16640_e11449;
        locals.var_pzbsswg_dn0 = assign16640_e11449_d_n0;
        locals.var_pzbsswg_dn2 = assign16640_e11449_d_n2;
        locals.var_pzbsswg_dn4 = assign16640_e11449_d_n4;
        locals.var_pzbsswg_dn5 = assign16640_e11449_d_n5;
        locals.var_pzbsswg_dn6 = assign16640_e11449_d_n6;
        locals.var_pzbsswg_dn7 = assign16640_e11449_d_n7;
        locals.var_pzbsswg_dn8 = assign16640_e11449_d_n8;
        locals.var_pzbsswg_dn9 = assign16640_e11449_d_n9;
        locals.var_pzbsswg_dn10 = assign16640_e11449_d_n10;
        locals.var_pzbsswg_dn11 = assign16640_e11449_d_n11;
        locals.var_pzbsswg_dn14 = assign16640_e11449_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let assign16650_e11452: f64 = (p.p87 * (nv6 - nv8));
        locals.var_vdsi = assign16650_e11452;
        locals.var_vdsi_dn6 = p.p87;
        locals.var_vdsi_dn8 = (-p.p87);
        locals.var_vdsi_rv = 0.0;

        let assign16660_e11455: f64 = (p.p87 * (nv7 - nv8));
        locals.var_vgsi = assign16660_e11455;
        locals.var_vgsi_dn7 = p.p87;
        locals.var_vgsi_dn8 = (-p.p87);
        locals.var_vgsi_rv = 0.0;

        let assign16670_e11458: f64 = (p.p87 * (nv9 - nv8));
        locals.var_vbsi = assign16670_e11458;
        locals.var_vbsi_dn8 = (-p.p87);
        locals.var_vbsi_dn9 = p.p87;
        locals.var_vbsi_rv = 0.0;

        let assign16680_e11461: f64 = (p.p87 * (nv0 - nv2));
        locals.var_vdsei = assign16680_e11461;
        locals.var_vdsei_dn0 = p.p87;
        locals.var_vdsei_dn2 = (-p.p87);
        locals.var_vdsei_rv = 0.0;

        let assign16690_e11464: f64 = (p.p87 * (nv7 - nv2));
        locals.var_vgsei = assign16690_e11464;
        locals.var_vgsei_dn2 = (-p.p87);
        locals.var_vgsei_dn7 = p.p87;
        locals.var_vgsei_rv = 0.0;

        let assign16700_e11467: f64 = (p.p87 * (nv9 - nv2));
        locals.var_vbsei = assign16700_e11467;
        locals.var_vbsei_dn2 = (-p.p87);
        locals.var_vbsei_dn9 = p.p87;
        locals.var_vbsei_rv = 0.0;

        let assign16710_e11470: f64 = (p.p87 * (nv0 - nv6));
        locals.var_vddp = assign16710_e11470;
        locals.var_vddp_dn0 = p.p87;
        locals.var_vddp_dn6 = (-p.p87);
        locals.var_vddp_rv = 0.0;

        let assign16720_e11473: f64 = (p.p87 * (nv8 - nv2));
        locals.var_vsps = assign16720_e11473;
        locals.var_vsps_dn2 = (-p.p87);
        locals.var_vsps_dn8 = p.p87;
        locals.var_vsps_rv = 0.0;

        let assign16730_e11476: f64 = (p.p87 * (nv11 - nv2));
        locals.var_vsbs = assign16730_e11476;
        locals.var_vsbs_dn2 = (-p.p87);
        locals.var_vsbs_dn11 = p.p87;
        locals.var_vsbs_rv = 0.0;

        let assign16740_e11479: f64 = (p.p87 * (nv10 - nv0));
        locals.var_vdbd = assign16740_e11479;
        locals.var_vdbd_dn0 = (-p.p87);
        locals.var_vdbd_dn10 = p.p87;
        locals.var_vdbd_rv = 0.0;

        let assign16750_e11482: f64 = (p.p87 * (nv9 - nv8));
        locals.var_vbpsp = assign16750_e11482;
        locals.var_vbpsp_dn8 = (-p.p87);
        locals.var_vbpsp_dn9 = p.p87;
        locals.var_vbpsp_rv = 0.0;

        let assign16760_e11485: f64 = (p.p87 * (nv9 - nv6));
        locals.var_vbpdp = assign16760_e11485;
        locals.var_vbpdp_dn6 = (-p.p87);
        locals.var_vbpdp_dn9 = p.p87;
        locals.var_vbpdp_rv = 0.0;

        locals.var_vbs_jct = locals.var_vsbs;
        locals.var_vbs_jct_dn2 = locals.var_vsbs_dn2;
        locals.var_vbs_jct_dn11 = locals.var_vsbs_dn11;
        locals.var_vbs_jct_rv = 0.0;

        locals.var_vbd_jct = locals.var_vdbd;
        locals.var_vbd_jct_dn0 = locals.var_vdbd_dn0;
        locals.var_vbd_jct_dn10 = locals.var_vdbd_dn10;
        locals.var_vbd_jct_rv = 0.0;

        locals.var_vbsi_jct = locals.var_vbpsp;
        locals.var_vbsi_jct_dn8 = locals.var_vbpsp_dn8;
        locals.var_vbsi_jct_dn9 = locals.var_vbpsp_dn9;
        locals.var_vbsi_jct_rv = 0.0;

        locals.var_vbdi_jct = locals.var_vbpdp;
        locals.var_vbdi_jct_dn6 = locals.var_vbpdp_dn6;
        locals.var_vbdi_jct_dn9 = locals.var_vbpdp_dn9;
        locals.var_vbdi_jct_rv = 0.0;

        let assign16810_e11492: f64 = (p.p87 * (nv4 - nv2));
        locals.var_vsubs = assign16810_e11492;
        locals.var_vsubs_dn2 = (-p.p87);
        locals.var_vsubs_dn4 = p.p87;
        locals.var_vsubs_rv = 0.0;

        let (assign16820_e11496, assign16820_e11496_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn12,)
    }
};
        locals.var_qi_nqs = assign16820_e11496;
        locals.var_qi_nqs_dn12 = assign16820_e11496_d_n12;
        locals.var_qi_nqs_rv = 0.0;

        let (assign16830_e11500, assign16830_e11500_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign16830_e11500;
        locals.var_qb_nqs_dn13 = assign16830_e11500_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign16840_e11505, assign16840_e11505_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn12,)
    }
};
        locals.var_qi_nqs = assign16840_e11505;
        locals.var_qi_nqs_dn12 = assign16840_e11505_d_n12;
        locals.var_qi_nqs_rv = 0.0;

        let (assign16850_e11510, assign16850_e11510_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign16850_e11510;
        locals.var_qb_nqs_dn13 = assign16850_e11510_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let assign16860_e11513: f64 = (locals.var_vgsi - locals.var_vdsi);
        locals.var_vgd = assign16860_e11513;
        locals.var_vgd_dn6 = (-locals.var_vdsi_dn6);
        locals.var_vgd_dn7 = locals.var_vgsi_dn7;
        locals.var_vgd_dn8 = (locals.var_vgsi_dn8 - locals.var_vdsi_dn8);
        locals.var_vgd_rv = 0.0;

        let assign16870_e11516: f64 = (locals.var_vbsi - locals.var_vdsi);
        locals.var_vbd = assign16870_e11516;
        locals.var_vbd_dn6 = (-locals.var_vdsi_dn6);
        locals.var_vbd_dn8 = (locals.var_vbsi_dn8 - locals.var_vdsi_dn8);
        locals.var_vbd_dn9 = locals.var_vbsi_dn9;
        locals.var_vbd_rv = 0.0;

        let assign16880_e11519: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign16880_e11519;
        locals.var_guard351_rv = 0.0;

        let (assign16890_e11523,) = {
    if (locals.var_guard351 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16890_e11523;
        locals.var_mode_rv = 0.0;

        let (assign16900_e11527, assign16900_e11527_d_n0, assign16900_e11527_d_n2, assign16900_e11527_d_n4, assign16900_e11527_d_n5, assign16900_e11527_d_n6, assign16900_e11527_d_n7, assign16900_e11527_d_n8, assign16900_e11527_d_n9, assign16900_e11527_d_n10, assign16900_e11527_d_n11, assign16900_e11527_d_n14,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign16900_e11527;
        locals.var_vds_dn0 = assign16900_e11527_d_n0;
        locals.var_vds_dn2 = assign16900_e11527_d_n2;
        locals.var_vds_dn4 = assign16900_e11527_d_n4;
        locals.var_vds_dn5 = assign16900_e11527_d_n5;
        locals.var_vds_dn6 = assign16900_e11527_d_n6;
        locals.var_vds_dn7 = assign16900_e11527_d_n7;
        locals.var_vds_dn8 = assign16900_e11527_d_n8;
        locals.var_vds_dn9 = assign16900_e11527_d_n9;
        locals.var_vds_dn10 = assign16900_e11527_d_n10;
        locals.var_vds_dn11 = assign16900_e11527_d_n11;
        locals.var_vds_dn14 = assign16900_e11527_d_n14;
        locals.var_vds_rv = 0.0;

        let (assign16910_e11531, assign16910_e11531_d_n6, assign16910_e11531_d_n7, assign16910_e11531_d_n8,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn7, locals.var_vgsi_dn8,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn8,)
    }
};
        locals.var_vgs = assign16910_e11531;
        locals.var_vgs_dn6 = assign16910_e11531_d_n6;
        locals.var_vgs_dn7 = assign16910_e11531_d_n7;
        locals.var_vgs_dn8 = assign16910_e11531_d_n8;
        locals.var_vgs_rv = 0.0;

        let (assign16920_e11535, assign16920_e11535_d_n6, assign16920_e11535_d_n8, assign16920_e11535_d_n9,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vbsi, 0.0, locals.var_vbsi_dn8, locals.var_vbsi_dn9,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn6, locals.var_vbs_dn8, locals.var_vbs_dn9,)
    }
};
        locals.var_vbs = assign16920_e11535;
        locals.var_vbs_dn6 = assign16920_e11535_d_n6;
        locals.var_vbs_dn8 = assign16920_e11535_d_n8;
        locals.var_vbs_dn9 = assign16920_e11535_d_n9;
        locals.var_vbs_rv = 0.0;

        let (assign16930_e11539, assign16930_e11539_d_n0, assign16930_e11539_d_n2,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16930_e11539;
        locals.var_vdse_dn0 = assign16930_e11539_d_n0;
        locals.var_vdse_dn2 = assign16930_e11539_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign16940_e11543, assign16940_e11543_d_n0, assign16940_e11543_d_n2, assign16940_e11543_d_n7,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vgsei, 0.0, locals.var_vgsei_dn2, locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn7,)
    }
};
        locals.var_vgse = assign16940_e11543;
        locals.var_vgse_dn0 = assign16940_e11543_d_n0;
        locals.var_vgse_dn2 = assign16940_e11543_d_n2;
        locals.var_vgse_dn7 = assign16940_e11543_d_n7;
        locals.var_vgse_rv = 0.0;

        let (assign16950_e11547, assign16950_e11547_d_n0, assign16950_e11547_d_n2, assign16950_e11547_d_n9,) = {
    if (locals.var_guard351 != 0.0) {
        (locals.var_vbsei, 0.0, locals.var_vbsei_dn2, locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn9,)
    }
};
        locals.var_vbse = assign16950_e11547;
        locals.var_vbse_dn0 = assign16950_e11547_d_n0;
        locals.var_vbse_dn2 = assign16950_e11547_d_n2;
        locals.var_vbse_dn9 = assign16950_e11547_d_n9;
        locals.var_vbse_rv = 0.0;

        let (assign16960_e11553,) = {
    if (locals.var_guard351 == 0.0) {
        let assign16960_e11551: f64 = (-1.0);
        (assign16960_e11551,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16960_e11553;
        locals.var_mode_rv = 0.0;

        let (assign16970_e11559, assign16970_e11559_d_n0, assign16970_e11559_d_n2, assign16970_e11559_d_n4, assign16970_e11559_d_n5, assign16970_e11559_d_n6, assign16970_e11559_d_n7, assign16970_e11559_d_n8, assign16970_e11559_d_n9, assign16970_e11559_d_n10, assign16970_e11559_d_n11, assign16970_e11559_d_n14,) = {
    if (locals.var_guard351 == 0.0) {
        let assign16970_e11557: f64 = (-locals.var_vdsi);
        (assign16970_e11557, 0.0, 0.0, 0.0, 0.0, (-locals.var_vdsi_dn6), 0.0, (-locals.var_vdsi_dn8), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign16970_e11559;
        locals.var_vds_dn0 = assign16970_e11559_d_n0;
        locals.var_vds_dn2 = assign16970_e11559_d_n2;
        locals.var_vds_dn4 = assign16970_e11559_d_n4;
        locals.var_vds_dn5 = assign16970_e11559_d_n5;
        locals.var_vds_dn6 = assign16970_e11559_d_n6;
        locals.var_vds_dn7 = assign16970_e11559_d_n7;
        locals.var_vds_dn8 = assign16970_e11559_d_n8;
        locals.var_vds_dn9 = assign16970_e11559_d_n9;
        locals.var_vds_dn10 = assign16970_e11559_d_n10;
        locals.var_vds_dn11 = assign16970_e11559_d_n11;
        locals.var_vds_dn14 = assign16970_e11559_d_n14;
        locals.var_vds_rv = 0.0;

        let (assign16980_e11564, assign16980_e11564_d_n6, assign16980_e11564_d_n7, assign16980_e11564_d_n8,) = {
    if (locals.var_guard351 == 0.0) {
        (locals.var_vgd, locals.var_vgd_dn6, locals.var_vgd_dn7, locals.var_vgd_dn8,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn8,)
    }
};
        locals.var_vgs = assign16980_e11564;
        locals.var_vgs_dn6 = assign16980_e11564_d_n6;
        locals.var_vgs_dn7 = assign16980_e11564_d_n7;
        locals.var_vgs_dn8 = assign16980_e11564_d_n8;
        locals.var_vgs_rv = 0.0;

        let (assign16990_e11569, assign16990_e11569_d_n6, assign16990_e11569_d_n8, assign16990_e11569_d_n9,) = {
    if (locals.var_guard351 == 0.0) {
        (locals.var_vbd, locals.var_vbd_dn6, locals.var_vbd_dn8, locals.var_vbd_dn9,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn6, locals.var_vbs_dn8, locals.var_vbs_dn9,)
    }
};
        locals.var_vbs = assign16990_e11569;
        locals.var_vbs_dn6 = assign16990_e11569_d_n6;
        locals.var_vbs_dn8 = assign16990_e11569_d_n8;
        locals.var_vbs_dn9 = assign16990_e11569_d_n9;
        locals.var_vbs_rv = 0.0;

        let (assign17000_e11575, assign17000_e11575_d_n0, assign17000_e11575_d_n2,) = {
    if (locals.var_guard351 == 0.0) {
        let assign17000_e11573: f64 = (-locals.var_vdsei);
        (assign17000_e11573, (-locals.var_vdsei_dn0), (-locals.var_vdsei_dn2),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign17000_e11575;
        locals.var_vdse_dn0 = assign17000_e11575_d_n0;
        locals.var_vdse_dn2 = assign17000_e11575_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign17010_e11582, assign17010_e11582_d_n0, assign17010_e11582_d_n2, assign17010_e11582_d_n7,) = {
    if (locals.var_guard351 == 0.0) {
        let assign17010_e11580: f64 = (locals.var_vgsei - locals.var_vdsei);
        (assign17010_e11580, (-locals.var_vdsei_dn0), (locals.var_vgsei_dn2 - locals.var_vdsei_dn2), locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn7,)
    }
};
        locals.var_vgse = assign17010_e11582;
        locals.var_vgse_dn0 = assign17010_e11582_d_n0;
        locals.var_vgse_dn2 = assign17010_e11582_d_n2;
        locals.var_vgse_dn7 = assign17010_e11582_d_n7;
        locals.var_vgse_rv = 0.0;

        let (assign17020_e11589, assign17020_e11589_d_n0, assign17020_e11589_d_n2, assign17020_e11589_d_n9,) = {
    if (locals.var_guard351 == 0.0) {
        let assign17020_e11587: f64 = (locals.var_vbsei - locals.var_vdsei);
        (assign17020_e11587, (-locals.var_vdsei_dn0), (locals.var_vbsei_dn2 - locals.var_vdsei_dn2), locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn9,)
    }
};
        locals.var_vbse = assign17020_e11589;
        locals.var_vbse_dn0 = assign17020_e11589_d_n0;
        locals.var_vbse_dn2 = assign17020_e11589_d_n2;
        locals.var_vbse_dn9 = assign17020_e11589_d_n9;
        locals.var_vbse_rv = 0.0;

        let assign17050_e11602: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard354 = assign17050_e11602;
        locals.var_guard354_rv = 0.0;

        let (assign17060_e11606, assign17060_e11606_d_n0, assign17060_e11606_d_n2, assign17060_e11606_d_n4, assign17060_e11606_d_n5, assign17060_e11606_d_n6, assign17060_e11606_d_n7, assign17060_e11606_d_n8, assign17060_e11606_d_n9, assign17060_e11606_d_n10, assign17060_e11606_d_n11, assign17060_e11606_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        ((nv5 - 0.0), 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn11, locals.var_deltemp_dn14,)
    }
};
        locals.var_deltemp = assign17060_e11606;
        locals.var_deltemp_dn0 = assign17060_e11606_d_n0;
        locals.var_deltemp_dn2 = assign17060_e11606_d_n2;
        locals.var_deltemp_dn4 = assign17060_e11606_d_n4;
        locals.var_deltemp_dn5 = assign17060_e11606_d_n5;
        locals.var_deltemp_dn6 = assign17060_e11606_d_n6;
        locals.var_deltemp_dn7 = assign17060_e11606_d_n7;
        locals.var_deltemp_dn8 = assign17060_e11606_d_n8;
        locals.var_deltemp_dn9 = assign17060_e11606_d_n9;
        locals.var_deltemp_dn10 = assign17060_e11606_d_n10;
        locals.var_deltemp_dn11 = assign17060_e11606_d_n11;
        locals.var_deltemp_dn14 = assign17060_e11606_d_n14;
        locals.var_deltemp_rv = 0.0;

        let assign17070_e11609: f64 = if p.p53 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard355 = assign17070_e11609;
        locals.var_guard355_rv = 0.0;

        let (assign17080_e11621, assign17080_e11621_d_n0, assign17080_e11621_d_n2, assign17080_e11621_d_n4, assign17080_e11621_d_n5, assign17080_e11621_d_n6, assign17080_e11621_d_n7, assign17080_e11621_d_n8, assign17080_e11621_d_n9, assign17080_e11621_d_n10, assign17080_e11621_d_n11, assign17080_e11621_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17080_e11615: f64 = (p.p433 - locals.var_deltemp);
        let assign17080_e11618: f64 = (p.p337 * 10.0);
        let assign17080_e11619: f64 = (assign17080_e11615 - assign17080_e11618);
        (assign17080_e11619, (-locals.var_deltemp_dn0), (-locals.var_deltemp_dn2), (-locals.var_deltemp_dn4), (-locals.var_deltemp_dn5), (-locals.var_deltemp_dn6), (-locals.var_deltemp_dn7), (-locals.var_deltemp_dn8), (-locals.var_deltemp_dn9), (-locals.var_deltemp_dn10), (-locals.var_deltemp_dn11), (-locals.var_deltemp_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign17080_e11621;
        locals.var_tmf1_dn0 = assign17080_e11621_d_n0;
        locals.var_tmf1_dn2 = assign17080_e11621_d_n2;
        locals.var_tmf1_dn4 = assign17080_e11621_d_n4;
        locals.var_tmf1_dn5 = assign17080_e11621_d_n5;
        locals.var_tmf1_dn6 = assign17080_e11621_d_n6;
        locals.var_tmf1_dn7 = assign17080_e11621_d_n7;
        locals.var_tmf1_dn8 = assign17080_e11621_d_n8;
        locals.var_tmf1_dn9 = assign17080_e11621_d_n9;
        locals.var_tmf1_dn10 = assign17080_e11621_d_n10;
        locals.var_tmf1_dn11 = assign17080_e11621_d_n11;
        locals.var_tmf1_dn14 = assign17080_e11621_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign17090_e11633, assign17090_e11633_d_n0, assign17090_e11633_d_n2, assign17090_e11633_d_n4, assign17090_e11633_d_n5, assign17090_e11633_d_n6, assign17090_e11633_d_n7, assign17090_e11633_d_n8, assign17090_e11633_d_n9, assign17090_e11633_d_n10, assign17090_e11633_d_n11, assign17090_e11633_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17090_e11627: f64 = (4.0 * p.p433);
        let assign17090_e11630: f64 = (p.p337 * 10.0);
        let assign17090_e11631: f64 = (assign17090_e11627 * assign17090_e11630);
        (assign17090_e11631, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17090_e11633;
        locals.var_tmf2_dn0 = assign17090_e11633_d_n0;
        locals.var_tmf2_dn2 = assign17090_e11633_d_n2;
        locals.var_tmf2_dn4 = assign17090_e11633_d_n4;
        locals.var_tmf2_dn5 = assign17090_e11633_d_n5;
        locals.var_tmf2_dn6 = assign17090_e11633_d_n6;
        locals.var_tmf2_dn7 = assign17090_e11633_d_n7;
        locals.var_tmf2_dn8 = assign17090_e11633_d_n8;
        locals.var_tmf2_dn9 = assign17090_e11633_d_n9;
        locals.var_tmf2_dn10 = assign17090_e11633_d_n10;
        locals.var_tmf2_dn11 = assign17090_e11633_d_n11;
        locals.var_tmf2_dn14 = assign17090_e11633_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign17100_e11645, assign17100_e11645_d_n0, assign17100_e11645_d_n2, assign17100_e11645_d_n4, assign17100_e11645_d_n5, assign17100_e11645_d_n6, assign17100_e11645_d_n7, assign17100_e11645_d_n8, assign17100_e11645_d_n9, assign17100_e11645_d_n10, assign17100_e11645_d_n11, assign17100_e11645_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let (assign17100_e11643, assign17100_e11643_d_n0, assign17100_e11643_d_n2, assign17100_e11643_d_n4, assign17100_e11643_d_n5, assign17100_e11643_d_n6, assign17100_e11643_d_n7, assign17100_e11643_d_n8, assign17100_e11643_d_n9, assign17100_e11643_d_n10, assign17100_e11643_d_n11, assign17100_e11643_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign17100_e11642: f64 = (-locals.var_tmf2);
                (assign17100_e11642, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign17100_e11643, assign17100_e11643_d_n0, assign17100_e11643_d_n2, assign17100_e11643_d_n4, assign17100_e11643_d_n5, assign17100_e11643_d_n6, assign17100_e11643_d_n7, assign17100_e11643_d_n8, assign17100_e11643_d_n9, assign17100_e11643_d_n10, assign17100_e11643_d_n11, assign17100_e11643_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17100_e11645;
        locals.var_tmf2_dn0 = assign17100_e11645_d_n0;
        locals.var_tmf2_dn2 = assign17100_e11645_d_n2;
        locals.var_tmf2_dn4 = assign17100_e11645_d_n4;
        locals.var_tmf2_dn5 = assign17100_e11645_d_n5;
        locals.var_tmf2_dn6 = assign17100_e11645_d_n6;
        locals.var_tmf2_dn7 = assign17100_e11645_d_n7;
        locals.var_tmf2_dn8 = assign17100_e11645_d_n8;
        locals.var_tmf2_dn9 = assign17100_e11645_d_n9;
        locals.var_tmf2_dn10 = assign17100_e11645_d_n10;
        locals.var_tmf2_dn11 = assign17100_e11645_d_n11;
        locals.var_tmf2_dn14 = assign17100_e11645_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign17110_e11656, assign17110_e11656_d_n0, assign17110_e11656_d_n2, assign17110_e11656_d_n4, assign17110_e11656_d_n5, assign17110_e11656_d_n6, assign17110_e11656_d_n7, assign17110_e11656_d_n8, assign17110_e11656_d_n9, assign17110_e11656_d_n10, assign17110_e11656_d_n11, assign17110_e11656_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17110_e11651: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17110_e11653: f64 = (assign17110_e11651 + locals.var_tmf2);
        let assign17110_e11654: f64 = (assign17110_e11653).sqrt();
        (assign17110_e11654, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign17110_e11654)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign17110_e11654)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17110_e11656;
        locals.var_tmf2_dn0 = assign17110_e11656_d_n0;
        locals.var_tmf2_dn2 = assign17110_e11656_d_n2;
        locals.var_tmf2_dn4 = assign17110_e11656_d_n4;
        locals.var_tmf2_dn5 = assign17110_e11656_d_n5;
        locals.var_tmf2_dn6 = assign17110_e11656_d_n6;
        locals.var_tmf2_dn7 = assign17110_e11656_d_n7;
        locals.var_tmf2_dn8 = assign17110_e11656_d_n8;
        locals.var_tmf2_dn9 = assign17110_e11656_d_n9;
        locals.var_tmf2_dn10 = assign17110_e11656_d_n10;
        locals.var_tmf2_dn11 = assign17110_e11656_d_n11;
        locals.var_tmf2_dn14 = assign17110_e11656_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign17120_e11668, assign17120_e11668_d_n0, assign17120_e11668_d_n2, assign17120_e11668_d_n4, assign17120_e11668_d_n5, assign17120_e11668_d_n6, assign17120_e11668_d_n7, assign17120_e11668_d_n8, assign17120_e11668_d_n9, assign17120_e11668_d_n10, assign17120_e11668_d_n11, assign17120_e11668_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17120_e11664: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17120_e11665: f64 = (1.0 + assign17120_e11664);
        let assign17120_e11666: f64 = (0.5 * assign17120_e11665);
        (assign17120_e11666, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17120_e11668;
        locals.var_t0_dn0 = assign17120_e11668_d_n0;
        locals.var_t0_dn2 = assign17120_e11668_d_n2;
        locals.var_t0_dn4 = assign17120_e11668_d_n4;
        locals.var_t0_dn5 = assign17120_e11668_d_n5;
        locals.var_t0_dn6 = assign17120_e11668_d_n6;
        locals.var_t0_dn7 = assign17120_e11668_d_n7;
        locals.var_t0_dn8 = assign17120_e11668_d_n8;
        locals.var_t0_dn9 = assign17120_e11668_d_n9;
        locals.var_t0_dn10 = assign17120_e11668_d_n10;
        locals.var_t0_dn11 = assign17120_e11668_d_n11;
        locals.var_t0_dn14 = assign17120_e11668_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign17130_e11680, assign17130_e11680_d_n0, assign17130_e11680_d_n2, assign17130_e11680_d_n4, assign17130_e11680_d_n5, assign17130_e11680_d_n6, assign17130_e11680_d_n7, assign17130_e11680_d_n8, assign17130_e11680_d_n9, assign17130_e11680_d_n10, assign17130_e11680_d_n11, assign17130_e11680_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17130_e11676: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17130_e11677: f64 = (0.5 * assign17130_e11676);
        let assign17130_e11678: f64 = (p.p433 - assign17130_e11677);
        (assign17130_e11678, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn11, locals.var_deltemp_dn14,)
    }
};
        locals.var_deltemp = assign17130_e11680;
        locals.var_deltemp_dn0 = assign17130_e11680_d_n0;
        locals.var_deltemp_dn2 = assign17130_e11680_d_n2;
        locals.var_deltemp_dn4 = assign17130_e11680_d_n4;
        locals.var_deltemp_dn5 = assign17130_e11680_d_n5;
        locals.var_deltemp_dn6 = assign17130_e11680_d_n6;
        locals.var_deltemp_dn7 = assign17130_e11680_d_n7;
        locals.var_deltemp_dn8 = assign17130_e11680_d_n8;
        locals.var_deltemp_dn9 = assign17130_e11680_d_n9;
        locals.var_deltemp_dn10 = assign17130_e11680_d_n10;
        locals.var_deltemp_dn11 = assign17130_e11680_d_n11;
        locals.var_deltemp_dn14 = assign17130_e11680_d_n14;
        locals.var_deltemp_rv = 0.0;

        let (assign17150_e11689, assign17150_e11689_d_n0, assign17150_e11689_d_n2, assign17150_e11689_d_n4, assign17150_e11689_d_n5, assign17150_e11689_d_n6, assign17150_e11689_d_n7, assign17150_e11689_d_n8, assign17150_e11689_d_n9, assign17150_e11689_d_n10, assign17150_e11689_d_n11, assign17150_e11689_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17150_e11685: f64 = ctx_temp;
        let assign17150_e11687: f64 = (assign17150_e11685 + p.p11);
        (assign17150_e11687, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign17150_e11689;
        locals.var_ttemp_dn0 = assign17150_e11689_d_n0;
        locals.var_ttemp_dn2 = assign17150_e11689_d_n2;
        locals.var_ttemp_dn4 = assign17150_e11689_d_n4;
        locals.var_ttemp_dn5 = assign17150_e11689_d_n5;
        locals.var_ttemp_dn6 = assign17150_e11689_d_n6;
        locals.var_ttemp_dn7 = assign17150_e11689_d_n7;
        locals.var_ttemp_dn8 = assign17150_e11689_d_n8;
        locals.var_ttemp_dn9 = assign17150_e11689_d_n9;
        locals.var_ttemp_dn10 = assign17150_e11689_d_n10;
        locals.var_ttemp_dn11 = assign17150_e11689_d_n11;
        locals.var_ttemp_dn14 = assign17150_e11689_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign17160_e11693, assign17160_e11693_d_n0, assign17160_e11693_d_n2, assign17160_e11693_d_n4, assign17160_e11693_d_n5, assign17160_e11693_d_n6, assign17160_e11693_d_n7, assign17160_e11693_d_n8, assign17160_e11693_d_n9, assign17160_e11693_d_n10, assign17160_e11693_d_n11, assign17160_e11693_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    }
};
        locals.var_ttemp0 = assign17160_e11693;
        locals.var_ttemp0_dn0 = assign17160_e11693_d_n0;
        locals.var_ttemp0_dn2 = assign17160_e11693_d_n2;
        locals.var_ttemp0_dn4 = assign17160_e11693_d_n4;
        locals.var_ttemp0_dn5 = assign17160_e11693_d_n5;
        locals.var_ttemp0_dn6 = assign17160_e11693_d_n6;
        locals.var_ttemp0_dn7 = assign17160_e11693_d_n7;
        locals.var_ttemp0_dn8 = assign17160_e11693_d_n8;
        locals.var_ttemp0_dn9 = assign17160_e11693_d_n9;
        locals.var_ttemp0_dn10 = assign17160_e11693_d_n10;
        locals.var_ttemp0_dn11 = assign17160_e11693_d_n11;
        locals.var_ttemp0_dn14 = assign17160_e11693_d_n14;
        locals.var_ttemp0_rv = 0.0;

        let (assign17170_e11699, assign17170_e11699_d_n0, assign17170_e11699_d_n2, assign17170_e11699_d_n4, assign17170_e11699_d_n5, assign17170_e11699_d_n6, assign17170_e11699_d_n7, assign17170_e11699_d_n8, assign17170_e11699_d_n9, assign17170_e11699_d_n10, assign17170_e11699_d_n11, assign17170_e11699_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17170_e11697: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign17170_e11697, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn11 + locals.var_deltemp_dn11), (locals.var_ttemp_dn14 + locals.var_deltemp_dn14),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign17170_e11699;
        locals.var_ttemp_dn0 = assign17170_e11699_d_n0;
        locals.var_ttemp_dn2 = assign17170_e11699_d_n2;
        locals.var_ttemp_dn4 = assign17170_e11699_d_n4;
        locals.var_ttemp_dn5 = assign17170_e11699_d_n5;
        locals.var_ttemp_dn6 = assign17170_e11699_d_n6;
        locals.var_ttemp_dn7 = assign17170_e11699_d_n7;
        locals.var_ttemp_dn8 = assign17170_e11699_d_n8;
        locals.var_ttemp_dn9 = assign17170_e11699_d_n9;
        locals.var_ttemp_dn10 = assign17170_e11699_d_n10;
        locals.var_ttemp_dn11 = assign17170_e11699_d_n11;
        locals.var_ttemp_dn14 = assign17170_e11699_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign17180_e11705, assign17180_e11705_d_n0, assign17180_e11705_d_n2, assign17180_e11705_d_n4, assign17180_e11705_d_n5, assign17180_e11705_d_n6, assign17180_e11705_d_n7, assign17180_e11705_d_n8, assign17180_e11705_d_n9, assign17180_e11705_d_n10, assign17180_e11705_d_n11, assign17180_e11705_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17180_e11703: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign17180_e11703, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn11, locals.var_tdiff0_dn14,)
    }
};
        locals.var_tdiff0 = assign17180_e11705;
        locals.var_tdiff0_dn0 = assign17180_e11705_d_n0;
        locals.var_tdiff0_dn2 = assign17180_e11705_d_n2;
        locals.var_tdiff0_dn4 = assign17180_e11705_d_n4;
        locals.var_tdiff0_dn5 = assign17180_e11705_d_n5;
        locals.var_tdiff0_dn6 = assign17180_e11705_d_n6;
        locals.var_tdiff0_dn7 = assign17180_e11705_d_n7;
        locals.var_tdiff0_dn8 = assign17180_e11705_d_n8;
        locals.var_tdiff0_dn9 = assign17180_e11705_d_n9;
        locals.var_tdiff0_dn10 = assign17180_e11705_d_n10;
        locals.var_tdiff0_dn11 = assign17180_e11705_d_n11;
        locals.var_tdiff0_dn14 = assign17180_e11705_d_n14;
        locals.var_tdiff0_rv = 0.0;

        let (assign17190_e11715, assign17190_e11715_d_n0, assign17190_e11715_d_n2, assign17190_e11715_d_n4, assign17190_e11715_d_n5, assign17190_e11715_d_n6, assign17190_e11715_d_n7, assign17190_e11715_d_n8, assign17190_e11715_d_n9, assign17190_e11715_d_n10, assign17190_e11715_d_n11, assign17190_e11715_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17190_e11709: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign17190_e11712: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17190_e11713: f64 = (assign17190_e11709 - assign17190_e11712);
        (assign17190_e11713, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn11 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn11)), ((locals.var_ttemp0_dn14 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn14)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn11, locals.var_tdiff0_2_dn14,)
    }
};
        locals.var_tdiff0_2 = assign17190_e11715;
        locals.var_tdiff0_2_dn0 = assign17190_e11715_d_n0;
        locals.var_tdiff0_2_dn2 = assign17190_e11715_d_n2;
        locals.var_tdiff0_2_dn4 = assign17190_e11715_d_n4;
        locals.var_tdiff0_2_dn5 = assign17190_e11715_d_n5;
        locals.var_tdiff0_2_dn6 = assign17190_e11715_d_n6;
        locals.var_tdiff0_2_dn7 = assign17190_e11715_d_n7;
        locals.var_tdiff0_2_dn8 = assign17190_e11715_d_n8;
        locals.var_tdiff0_2_dn9 = assign17190_e11715_d_n9;
        locals.var_tdiff0_2_dn10 = assign17190_e11715_d_n10;
        locals.var_tdiff0_2_dn11 = assign17190_e11715_d_n11;
        locals.var_tdiff0_2_dn14 = assign17190_e11715_d_n14;
        locals.var_tdiff0_2_rv = 0.0;

        let (assign17200_e11721, assign17200_e11721_d_n0, assign17200_e11721_d_n2, assign17200_e11721_d_n4, assign17200_e11721_d_n5, assign17200_e11721_d_n6, assign17200_e11721_d_n7, assign17200_e11721_d_n8, assign17200_e11721_d_n9, assign17200_e11721_d_n10, assign17200_e11721_d_n11, assign17200_e11721_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17200_e11719: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign17200_e11719, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn11, locals.var_tdiff_dn14,)
    }
};
        locals.var_tdiff = assign17200_e11721;
        locals.var_tdiff_dn0 = assign17200_e11721_d_n0;
        locals.var_tdiff_dn2 = assign17200_e11721_d_n2;
        locals.var_tdiff_dn4 = assign17200_e11721_d_n4;
        locals.var_tdiff_dn5 = assign17200_e11721_d_n5;
        locals.var_tdiff_dn6 = assign17200_e11721_d_n6;
        locals.var_tdiff_dn7 = assign17200_e11721_d_n7;
        locals.var_tdiff_dn8 = assign17200_e11721_d_n8;
        locals.var_tdiff_dn9 = assign17200_e11721_d_n9;
        locals.var_tdiff_dn10 = assign17200_e11721_d_n10;
        locals.var_tdiff_dn11 = assign17200_e11721_d_n11;
        locals.var_tdiff_dn14 = assign17200_e11721_d_n14;
        locals.var_tdiff_rv = 0.0;

        let (assign17210_e11731, assign17210_e11731_d_n0, assign17210_e11731_d_n2, assign17210_e11731_d_n4, assign17210_e11731_d_n5, assign17210_e11731_d_n6, assign17210_e11731_d_n7, assign17210_e11731_d_n8, assign17210_e11731_d_n9, assign17210_e11731_d_n10, assign17210_e11731_d_n11, assign17210_e11731_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17210_e11725: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign17210_e11728: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17210_e11729: f64 = (assign17210_e11725 - assign17210_e11728);
        (assign17210_e11729, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn11 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn11)), ((locals.var_ttemp_dn14 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn11, locals.var_tdiff_2_dn14,)
    }
};
        locals.var_tdiff_2 = assign17210_e11731;
        locals.var_tdiff_2_dn0 = assign17210_e11731_d_n0;
        locals.var_tdiff_2_dn2 = assign17210_e11731_d_n2;
        locals.var_tdiff_2_dn4 = assign17210_e11731_d_n4;
        locals.var_tdiff_2_dn5 = assign17210_e11731_d_n5;
        locals.var_tdiff_2_dn6 = assign17210_e11731_d_n6;
        locals.var_tdiff_2_dn7 = assign17210_e11731_d_n7;
        locals.var_tdiff_2_dn8 = assign17210_e11731_d_n8;
        locals.var_tdiff_2_dn9 = assign17210_e11731_d_n9;
        locals.var_tdiff_2_dn10 = assign17210_e11731_d_n10;
        locals.var_tdiff_2_dn11 = assign17210_e11731_d_n11;
        locals.var_tdiff_2_dn14 = assign17210_e11731_d_n14;
        locals.var_tdiff_2_rv = 0.0;

        let (assign17220_e11737, assign17220_e11737_d_n0, assign17220_e11737_d_n2, assign17220_e11737_d_n4, assign17220_e11737_d_n5, assign17220_e11737_d_n6, assign17220_e11737_d_n7, assign17220_e11737_d_n8, assign17220_e11737_d_n9, assign17220_e11737_d_n10, assign17220_e11737_d_n11, assign17220_e11737_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17220_e11735: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17220_e11735, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn11, locals.var_tratio_dn14,)
    }
};
        locals.var_tratio = assign17220_e11737;
        locals.var_tratio_dn0 = assign17220_e11737_d_n0;
        locals.var_tratio_dn2 = assign17220_e11737_d_n2;
        locals.var_tratio_dn4 = assign17220_e11737_d_n4;
        locals.var_tratio_dn5 = assign17220_e11737_d_n5;
        locals.var_tratio_dn6 = assign17220_e11737_d_n6;
        locals.var_tratio_dn7 = assign17220_e11737_d_n7;
        locals.var_tratio_dn8 = assign17220_e11737_d_n8;
        locals.var_tratio_dn9 = assign17220_e11737_d_n9;
        locals.var_tratio_dn10 = assign17220_e11737_d_n10;
        locals.var_tratio_dn11 = assign17220_e11737_d_n11;
        locals.var_tratio_dn14 = assign17220_e11737_d_n14;
        locals.var_tratio_rv = 0.0;

        let (assign17230_e11742, assign17230_e11742_d_n0, assign17230_e11742_d_n2, assign17230_e11742_d_n4, assign17230_e11742_d_n5, assign17230_e11742_d_n6, assign17230_e11742_d_n7, assign17230_e11742_d_n8, assign17230_e11742_d_n9, assign17230_e11742_d_n10, assign17230_e11742_d_n11, assign17230_e11742_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17230_e11740: f64 = (locals.var_tratio).ln();
        (assign17230_e11740, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn11 / locals.var_tratio), (locals.var_tratio_dn14 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn11, locals.var_log_tratio_dn14,)
    }
};
        locals.var_log_tratio = assign17230_e11742;
        locals.var_log_tratio_dn0 = assign17230_e11742_d_n0;
        locals.var_log_tratio_dn2 = assign17230_e11742_d_n2;
        locals.var_log_tratio_dn4 = assign17230_e11742_d_n4;
        locals.var_log_tratio_dn5 = assign17230_e11742_d_n5;
        locals.var_log_tratio_dn6 = assign17230_e11742_d_n6;
        locals.var_log_tratio_dn7 = assign17230_e11742_d_n7;
        locals.var_log_tratio_dn8 = assign17230_e11742_d_n8;
        locals.var_log_tratio_dn9 = assign17230_e11742_d_n9;
        locals.var_log_tratio_dn10 = assign17230_e11742_d_n10;
        locals.var_log_tratio_dn11 = assign17230_e11742_d_n11;
        locals.var_log_tratio_dn14 = assign17230_e11742_d_n14;
        locals.var_log_tratio_rv = 0.0;

        let (assign17240_e11754, assign17240_e11754_d_n0, assign17240_e11754_d_n2, assign17240_e11754_d_n4, assign17240_e11754_d_n5, assign17240_e11754_d_n6, assign17240_e11754_d_n7, assign17240_e11754_d_n8, assign17240_e11754_d_n9, assign17240_e11754_d_n10, assign17240_e11754_d_n11, assign17240_e11754_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17240_e11747: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign17240_e11748: f64 = (locals.var_egtnom - assign17240_e11747);
        let assign17240_e11751: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign17240_e11752: f64 = (assign17240_e11748 - assign17240_e11751);
        (assign17240_e11752, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn11)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn11)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn14)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn11, locals.var_eg_dn14,)
    }
};
        locals.var_eg = assign17240_e11754;
        locals.var_eg_dn0 = assign17240_e11754_d_n0;
        locals.var_eg_dn2 = assign17240_e11754_d_n2;
        locals.var_eg_dn4 = assign17240_e11754_d_n4;
        locals.var_eg_dn5 = assign17240_e11754_d_n5;
        locals.var_eg_dn6 = assign17240_e11754_d_n6;
        locals.var_eg_dn7 = assign17240_e11754_d_n7;
        locals.var_eg_dn8 = assign17240_e11754_d_n8;
        locals.var_eg_dn9 = assign17240_e11754_d_n9;
        locals.var_eg_dn10 = assign17240_e11754_d_n10;
        locals.var_eg_dn11 = assign17240_e11754_d_n11;
        locals.var_eg_dn14 = assign17240_e11754_d_n14;
        locals.var_eg_rv = 0.0;

        let (assign17250_e11759, assign17250_e11759_d_n0, assign17250_e11759_d_n2, assign17250_e11759_d_n4, assign17250_e11759_d_n5, assign17250_e11759_d_n6, assign17250_e11759_d_n7, assign17250_e11759_d_n8, assign17250_e11759_d_n9, assign17250_e11759_d_n10, assign17250_e11759_d_n11, assign17250_e11759_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17250_e11757: f64 = (locals.var_eg).sqrt();
        (assign17250_e11757, (locals.var_eg_dn0 / (2.0 * assign17250_e11757)), (locals.var_eg_dn2 / (2.0 * assign17250_e11757)), (locals.var_eg_dn4 / (2.0 * assign17250_e11757)), (locals.var_eg_dn5 / (2.0 * assign17250_e11757)), (locals.var_eg_dn6 / (2.0 * assign17250_e11757)), (locals.var_eg_dn7 / (2.0 * assign17250_e11757)), (locals.var_eg_dn8 / (2.0 * assign17250_e11757)), (locals.var_eg_dn9 / (2.0 * assign17250_e11757)), (locals.var_eg_dn10 / (2.0 * assign17250_e11757)), (locals.var_eg_dn11 / (2.0 * assign17250_e11757)), (locals.var_eg_dn14 / (2.0 * assign17250_e11757)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    }
};
        locals.var_sqrt_eg = assign17250_e11759;
        locals.var_sqrt_eg_dn0 = assign17250_e11759_d_n0;
        locals.var_sqrt_eg_dn2 = assign17250_e11759_d_n2;
        locals.var_sqrt_eg_dn4 = assign17250_e11759_d_n4;
        locals.var_sqrt_eg_dn5 = assign17250_e11759_d_n5;
        locals.var_sqrt_eg_dn6 = assign17250_e11759_d_n6;
        locals.var_sqrt_eg_dn7 = assign17250_e11759_d_n7;
        locals.var_sqrt_eg_dn8 = assign17250_e11759_d_n8;
        locals.var_sqrt_eg_dn9 = assign17250_e11759_d_n9;
        locals.var_sqrt_eg_dn10 = assign17250_e11759_d_n10;
        locals.var_sqrt_eg_dn11 = assign17250_e11759_d_n11;
        locals.var_sqrt_eg_dn14 = assign17250_e11759_d_n14;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign17260_e11765, assign17260_e11765_d_n0, assign17260_e11765_d_n2, assign17260_e11765_d_n4, assign17260_e11765_d_n5, assign17260_e11765_d_n6, assign17260_e11765_d_n7, assign17260_e11765_d_n8, assign17260_e11765_d_n9, assign17260_e11765_d_n10, assign17260_e11765_d_n11, assign17260_e11765_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17260_e11763: f64 = (1.0 / locals.var_ttemp);
        (assign17260_e11763, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn11 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn14 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17260_e11765;
        locals.var_t1_dn0 = assign17260_e11765_d_n0;
        locals.var_t1_dn2 = assign17260_e11765_d_n2;
        locals.var_t1_dn4 = assign17260_e11765_d_n4;
        locals.var_t1_dn5 = assign17260_e11765_d_n5;
        locals.var_t1_dn6 = assign17260_e11765_d_n6;
        locals.var_t1_dn7 = assign17260_e11765_d_n7;
        locals.var_t1_dn8 = assign17260_e11765_d_n8;
        locals.var_t1_dn9 = assign17260_e11765_d_n9;
        locals.var_t1_dn10 = assign17260_e11765_d_n10;
        locals.var_t1_dn11 = assign17260_e11765_d_n11;
        locals.var_t1_dn14 = assign17260_e11765_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17270_e11771, assign17270_e11771_d_n0, assign17270_e11771_d_n2, assign17270_e11771_d_n4, assign17270_e11771_d_n5, assign17270_e11771_d_n6, assign17270_e11771_d_n7, assign17270_e11771_d_n8, assign17270_e11771_d_n9, assign17270_e11771_d_n10, assign17270_e11771_d_n11, assign17270_e11771_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17270_e11769: f64 = (1.0 / locals.var_ktnom);
        (assign17270_e11769, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17270_e11771;
        locals.var_t2_dn0 = assign17270_e11771_d_n0;
        locals.var_t2_dn2 = assign17270_e11771_d_n2;
        locals.var_t2_dn4 = assign17270_e11771_d_n4;
        locals.var_t2_dn5 = assign17270_e11771_d_n5;
        locals.var_t2_dn6 = assign17270_e11771_d_n6;
        locals.var_t2_dn7 = assign17270_e11771_d_n7;
        locals.var_t2_dn8 = assign17270_e11771_d_n8;
        locals.var_t2_dn9 = assign17270_e11771_d_n9;
        locals.var_t2_dn10 = assign17270_e11771_d_n10;
        locals.var_t2_dn11 = assign17270_e11771_d_n11;
        locals.var_t2_dn14 = assign17270_e11771_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign17280_e11793, assign17280_e11793_d_n0, assign17280_e11793_d_n2, assign17280_e11793_d_n4, assign17280_e11793_d_n5, assign17280_e11793_d_n6, assign17280_e11793_d_n7, assign17280_e11793_d_n8, assign17280_e11793_d_n9, assign17280_e11793_d_n10, assign17280_e11793_d_n11, assign17280_e11793_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17280_e11775: f64 = (locals.var_egtnom + p.p259);
        let assign17280_e11779: f64 = (locals.var_t1 - locals.var_t2);
        let assign17280_e11780: f64 = (p.p260 * assign17280_e11779);
        let assign17280_e11781: f64 = (assign17280_e11775 + assign17280_e11780);
        let assign17280_e11785: f64 = (locals.var_t1 * locals.var_t1);
        let assign17280_e11788: f64 = (locals.var_t2 * locals.var_t2);
        let assign17280_e11789: f64 = (assign17280_e11785 - assign17280_e11788);
        let assign17280_e11790: f64 = (p.p261 * assign17280_e11789);
        let assign17280_e11791: f64 = (assign17280_e11781 + assign17280_e11790);
        (assign17280_e11791, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn11 - locals.var_t2_dn11)) + (p.p261 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) - ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))))), ((p.p260 * (locals.var_t1_dn14 - locals.var_t2_dn14)) + (p.p261 * (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) - ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign17280_e11793;
        locals.var_t3_dn0 = assign17280_e11793_d_n0;
        locals.var_t3_dn2 = assign17280_e11793_d_n2;
        locals.var_t3_dn4 = assign17280_e11793_d_n4;
        locals.var_t3_dn5 = assign17280_e11793_d_n5;
        locals.var_t3_dn6 = assign17280_e11793_d_n6;
        locals.var_t3_dn7 = assign17280_e11793_d_n7;
        locals.var_t3_dn8 = assign17280_e11793_d_n8;
        locals.var_t3_dn9 = assign17280_e11793_d_n9;
        locals.var_t3_dn10 = assign17280_e11793_d_n10;
        locals.var_t3_dn11 = assign17280_e11793_d_n11;
        locals.var_t3_dn14 = assign17280_e11793_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign17290_e11798, assign17290_e11798_d_n0, assign17290_e11798_d_n2, assign17290_e11798_d_n4, assign17290_e11798_d_n5, assign17290_e11798_d_n6, assign17290_e11798_d_n7, assign17290_e11798_d_n8, assign17290_e11798_d_n9, assign17290_e11798_d_n10, assign17290_e11798_d_n11, assign17290_e11798_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17290_e11796: f64 = (locals.var_t3).sqrt();
        (assign17290_e11796, (locals.var_t3_dn0 / (2.0 * assign17290_e11796)), (locals.var_t3_dn2 / (2.0 * assign17290_e11796)), (locals.var_t3_dn4 / (2.0 * assign17290_e11796)), (locals.var_t3_dn5 / (2.0 * assign17290_e11796)), (locals.var_t3_dn6 / (2.0 * assign17290_e11796)), (locals.var_t3_dn7 / (2.0 * assign17290_e11796)), (locals.var_t3_dn8 / (2.0 * assign17290_e11796)), (locals.var_t3_dn9 / (2.0 * assign17290_e11796)), (locals.var_t3_dn10 / (2.0 * assign17290_e11796)), (locals.var_t3_dn11 / (2.0 * assign17290_e11796)), (locals.var_t3_dn14 / (2.0 * assign17290_e11796)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn11, locals.var_egp12_dn14,)
    }
};
        locals.var_egp12 = assign17290_e11798;
        locals.var_egp12_dn0 = assign17290_e11798_d_n0;
        locals.var_egp12_dn2 = assign17290_e11798_d_n2;
        locals.var_egp12_dn4 = assign17290_e11798_d_n4;
        locals.var_egp12_dn5 = assign17290_e11798_d_n5;
        locals.var_egp12_dn6 = assign17290_e11798_d_n6;
        locals.var_egp12_dn7 = assign17290_e11798_d_n7;
        locals.var_egp12_dn8 = assign17290_e11798_d_n8;
        locals.var_egp12_dn9 = assign17290_e11798_d_n9;
        locals.var_egp12_dn10 = assign17290_e11798_d_n10;
        locals.var_egp12_dn11 = assign17290_e11798_d_n11;
        locals.var_egp12_dn14 = assign17290_e11798_d_n14;
        locals.var_egp12_rv = 0.0;

        let (assign17300_e11804, assign17300_e11804_d_n0, assign17300_e11804_d_n2, assign17300_e11804_d_n4, assign17300_e11804_d_n5, assign17300_e11804_d_n6, assign17300_e11804_d_n7, assign17300_e11804_d_n8, assign17300_e11804_d_n9, assign17300_e11804_d_n10, assign17300_e11804_d_n11, assign17300_e11804_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17300_e11802: f64 = (locals.var_t3 * locals.var_egp12);
        (assign17300_e11802, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn11 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn11)), ((locals.var_t3_dn14 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn14)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn11, locals.var_egp32_dn14,)
    }
};
        locals.var_egp32 = assign17300_e11804;
        locals.var_egp32_dn0 = assign17300_e11804_d_n0;
        locals.var_egp32_dn2 = assign17300_e11804_d_n2;
        locals.var_egp32_dn4 = assign17300_e11804_d_n4;
        locals.var_egp32_dn5 = assign17300_e11804_d_n5;
        locals.var_egp32_dn6 = assign17300_e11804_d_n6;
        locals.var_egp32_dn7 = assign17300_e11804_d_n7;
        locals.var_egp32_dn8 = assign17300_e11804_d_n8;
        locals.var_egp32_dn9 = assign17300_e11804_d_n9;
        locals.var_egp32_dn10 = assign17300_e11804_d_n10;
        locals.var_egp32_dn11 = assign17300_e11804_d_n11;
        locals.var_egp32_dn14 = assign17300_e11804_d_n14;
        locals.var_egp32_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17310_e11812, assign17310_e11812_d_n0, assign17310_e11812_d_n2, assign17310_e11812_d_n4, assign17310_e11812_d_n5, assign17310_e11812_d_n6, assign17310_e11812_d_n7, assign17310_e11812_d_n8, assign17310_e11812_d_n9, assign17310_e11812_d_n10, assign17310_e11812_d_n11, assign17310_e11812_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17310_e11809: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign17310_e11810: f64 = (1.6021918e-19 / assign17310_e11809);
        (assign17310_e11810, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn11)) / (assign17310_e11809 * assign17310_e11809))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn14)) / (assign17310_e11809 * assign17310_e11809))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    }
};
        locals.var_beta = assign17310_e11812;
        locals.var_beta_dn0 = assign17310_e11812_d_n0;
        locals.var_beta_dn2 = assign17310_e11812_d_n2;
        locals.var_beta_dn4 = assign17310_e11812_d_n4;
        locals.var_beta_dn5 = assign17310_e11812_d_n5;
        locals.var_beta_dn6 = assign17310_e11812_d_n6;
        locals.var_beta_dn7 = assign17310_e11812_d_n7;
        locals.var_beta_dn8 = assign17310_e11812_d_n8;
        locals.var_beta_dn9 = assign17310_e11812_d_n9;
        locals.var_beta_dn10 = assign17310_e11812_d_n10;
        locals.var_beta_dn11 = assign17310_e11812_d_n11;
        locals.var_beta_dn14 = assign17310_e11812_d_n14;
        locals.var_beta_rv = 0.0;

        let (assign17320_e11818, assign17320_e11818_d_n0, assign17320_e11818_d_n2, assign17320_e11818_d_n4, assign17320_e11818_d_n5, assign17320_e11818_d_n6, assign17320_e11818_d_n7, assign17320_e11818_d_n8, assign17320_e11818_d_n9, assign17320_e11818_d_n10, assign17320_e11818_d_n11, assign17320_e11818_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17320_e11816: f64 = (1.0 / locals.var_beta);
        (assign17320_e11816, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn11 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn14 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn11, locals.var_beta_inv_dn14,)
    }
};
        locals.var_beta_inv = assign17320_e11818;
        locals.var_beta_inv_dn0 = assign17320_e11818_d_n0;
        locals.var_beta_inv_dn2 = assign17320_e11818_d_n2;
        locals.var_beta_inv_dn4 = assign17320_e11818_d_n4;
        locals.var_beta_inv_dn5 = assign17320_e11818_d_n5;
        locals.var_beta_inv_dn6 = assign17320_e11818_d_n6;
        locals.var_beta_inv_dn7 = assign17320_e11818_d_n7;
        locals.var_beta_inv_dn8 = assign17320_e11818_d_n8;
        locals.var_beta_inv_dn9 = assign17320_e11818_d_n9;
        locals.var_beta_inv_dn10 = assign17320_e11818_d_n10;
        locals.var_beta_inv_dn11 = assign17320_e11818_d_n11;
        locals.var_beta_inv_dn14 = assign17320_e11818_d_n14;
        locals.var_beta_inv_rv = 0.0;

        let (assign17330_e11824, assign17330_e11824_d_n0, assign17330_e11824_d_n2, assign17330_e11824_d_n4, assign17330_e11824_d_n5, assign17330_e11824_d_n6, assign17330_e11824_d_n7, assign17330_e11824_d_n8, assign17330_e11824_d_n9, assign17330_e11824_d_n10, assign17330_e11824_d_n11, assign17330_e11824_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17330_e11822: f64 = (locals.var_beta * locals.var_beta);
        (assign17330_e11822, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn11 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn11)), ((locals.var_beta_dn14 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn14)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn11, locals.var_beta2_dn14,)
    }
};
        locals.var_beta2 = assign17330_e11824;
        locals.var_beta2_dn0 = assign17330_e11824_d_n0;
        locals.var_beta2_dn2 = assign17330_e11824_d_n2;
        locals.var_beta2_dn4 = assign17330_e11824_d_n4;
        locals.var_beta2_dn5 = assign17330_e11824_d_n5;
        locals.var_beta2_dn6 = assign17330_e11824_d_n6;
        locals.var_beta2_dn7 = assign17330_e11824_d_n7;
        locals.var_beta2_dn8 = assign17330_e11824_d_n8;
        locals.var_beta2_dn9 = assign17330_e11824_d_n9;
        locals.var_beta2_dn10 = assign17330_e11824_d_n10;
        locals.var_beta2_dn11 = assign17330_e11824_d_n11;
        locals.var_beta2_dn14 = assign17330_e11824_d_n14;
        locals.var_beta2_rv = 0.0;

        let (assign17340_e11832,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17340_e11829: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign17340_e11830: f64 = (1.6021918e-19 / assign17340_e11829);
        (assign17340_e11830,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign17340_e11832;
        locals.var_betatnom_rv = 0.0;

        let (assign17350_e11855, assign17350_e11855_d_n0, assign17350_e11855_d_n2, assign17350_e11855_d_n4, assign17350_e11855_d_n5, assign17350_e11855_d_n6, assign17350_e11855_d_n7, assign17350_e11855_d_n8, assign17350_e11855_d_n9, assign17350_e11855_d_n10, assign17350_e11855_d_n11, assign17350_e11855_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17350_e11837: f64 = (locals.var_log_tratio * 1.5);
        let assign17350_e11838: f64 = (assign17350_e11837).exp();
        let assign17350_e11839: f64 = (1.04e16 * assign17350_e11838);
        let assign17350_e11841: f64 = (-locals.var_eg);
        let assign17350_e11843: f64 = (assign17350_e11841 / 2.0);
        let assign17350_e11845: f64 = (assign17350_e11843 * locals.var_beta);
        let assign17350_e11848: f64 = (locals.var_egtnom / 2.0);
        let assign17350_e11850: f64 = (assign17350_e11848 * locals.var_betatnom);
        let assign17350_e11851: f64 = (assign17350_e11845 + assign17350_e11850);
        let assign17350_e11852: f64 = (assign17350_e11851).exp();
        let assign17350_e11853: f64 = (assign17350_e11839 * assign17350_e11852);
        (assign17350_e11853, (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn0 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn0))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn2 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn2))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn4 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn4))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn5 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn5))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn6 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn6))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn7 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn7))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn8 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn8))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn9 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn9))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn10 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn10))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn11 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn11) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn11))))), (((1.04e16 * (assign17350_e11838 * (locals.var_log_tratio_dn14 * 1.5))) * assign17350_e11852) + (assign17350_e11839 * (assign17350_e11852 * ((((-locals.var_eg_dn14) / 2.0) * locals.var_beta) + (assign17350_e11843 * locals.var_beta_dn14))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn11, locals.var_nin_dn14,)
    }
};
        locals.var_nin = assign17350_e11855;
        locals.var_nin_dn0 = assign17350_e11855_d_n0;
        locals.var_nin_dn2 = assign17350_e11855_d_n2;
        locals.var_nin_dn4 = assign17350_e11855_d_n4;
        locals.var_nin_dn5 = assign17350_e11855_d_n5;
        locals.var_nin_dn6 = assign17350_e11855_d_n6;
        locals.var_nin_dn7 = assign17350_e11855_d_n7;
        locals.var_nin_dn8 = assign17350_e11855_d_n8;
        locals.var_nin_dn9 = assign17350_e11855_d_n9;
        locals.var_nin_dn10 = assign17350_e11855_d_n10;
        locals.var_nin_dn11 = assign17350_e11855_d_n11;
        locals.var_nin_dn14 = assign17350_e11855_d_n14;
        locals.var_nin_rv = 0.0;

        let (assign17360_e11862, assign17360_e11862_d_n0, assign17360_e11862_d_n2, assign17360_e11862_d_n4, assign17360_e11862_d_n5, assign17360_e11862_d_n6, assign17360_e11862_d_n7, assign17360_e11862_d_n8, assign17360_e11862_d_n9, assign17360_e11862_d_n10, assign17360_e11862_d_n11, assign17360_e11862_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17360_e11859: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign17360_e11860: f64 = (assign17360_e11859).exp();
        (assign17360_e11860, (assign17360_e11860 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn11 * locals.var_uc_muetmp)), (assign17360_e11860 * (locals.var_log_tratio_dn14 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17360_e11862;
        locals.var_t1_dn0 = assign17360_e11862_d_n0;
        locals.var_t1_dn2 = assign17360_e11862_d_n2;
        locals.var_t1_dn4 = assign17360_e11862_d_n4;
        locals.var_t1_dn5 = assign17360_e11862_d_n5;
        locals.var_t1_dn6 = assign17360_e11862_d_n6;
        locals.var_t1_dn7 = assign17360_e11862_d_n7;
        locals.var_t1_dn8 = assign17360_e11862_d_n8;
        locals.var_t1_dn9 = assign17360_e11862_d_n9;
        locals.var_t1_dn10 = assign17360_e11862_d_n10;
        locals.var_t1_dn11 = assign17360_e11862_d_n11;
        locals.var_t1_dn14 = assign17360_e11862_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17370_e11868, assign17370_e11868_d_n0, assign17370_e11868_d_n2, assign17370_e11868_d_n4, assign17370_e11868_d_n5, assign17370_e11868_d_n6, assign17370_e11868_d_n7, assign17370_e11868_d_n8, assign17370_e11868_d_n9, assign17370_e11868_d_n10, assign17370_e11868_d_n11, assign17370_e11868_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign17370_e11866: f64 = (locals.var_t1 / locals.var_mueph);
        (assign17370_e11866, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn11 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn11)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn14 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn14)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn11, locals.var_mphn0_dn14,)
    }
};
        locals.var_mphn0 = assign17370_e11868;
        locals.var_mphn0_dn0 = assign17370_e11868_d_n0;
        locals.var_mphn0_dn2 = assign17370_e11868_d_n2;
        locals.var_mphn0_dn4 = assign17370_e11868_d_n4;
        locals.var_mphn0_dn5 = assign17370_e11868_d_n5;
        locals.var_mphn0_dn6 = assign17370_e11868_d_n6;
        locals.var_mphn0_dn7 = assign17370_e11868_d_n7;
        locals.var_mphn0_dn8 = assign17370_e11868_d_n8;
        locals.var_mphn0_dn9 = assign17370_e11868_d_n9;
        locals.var_mphn0_dn10 = assign17370_e11868_d_n10;
        locals.var_mphn0_dn11 = assign17370_e11868_d_n11;
        locals.var_mphn0_dn14 = assign17370_e11868_d_n14;
        locals.var_mphn0_rv = 0.0;

        let assign17380_e11875: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard357 = assign17380_e11875;
        locals.var_guard357_rv = 0.0;

        let (assign17390_e11890, assign17390_e11890_d_n0, assign17390_e11890_d_n2, assign17390_e11890_d_n4, assign17390_e11890_d_n5, assign17390_e11890_d_n6, assign17390_e11890_d_n7, assign17390_e11890_d_n8, assign17390_e11890_d_n9, assign17390_e11890_d_n10, assign17390_e11890_d_n11, assign17390_e11890_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17390_e11881: f64 = (2.0 * 1.034943e-10);
        let assign17390_e11883: f64 = (assign17390_e11881 * 1.6021918e-19);
        let assign17390_e11885: f64 = (assign17390_e11883 * locals.var_uc_ndepm);
        let assign17390_e11887: f64 = (assign17390_e11885 * locals.var_beta_inv);
        let assign17390_e11888: f64 = (assign17390_e11887).sqrt();
        (assign17390_e11888, ((((assign17390_e11883 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn0)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn2)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn4)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn5)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn6)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn7)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn8)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn9)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn10)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn11)) / (2.0 * assign17390_e11888)), ((((assign17390_e11883 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign17390_e11885 * locals.var_beta_inv_dn14)) / (2.0 * assign17390_e11888)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign17390_e11890;
        locals.var_cnst0_dn0 = assign17390_e11890_d_n0;
        locals.var_cnst0_dn2 = assign17390_e11890_d_n2;
        locals.var_cnst0_dn4 = assign17390_e11890_d_n4;
        locals.var_cnst0_dn5 = assign17390_e11890_d_n5;
        locals.var_cnst0_dn6 = assign17390_e11890_d_n6;
        locals.var_cnst0_dn7 = assign17390_e11890_d_n7;
        locals.var_cnst0_dn8 = assign17390_e11890_d_n8;
        locals.var_cnst0_dn9 = assign17390_e11890_d_n9;
        locals.var_cnst0_dn10 = assign17390_e11890_d_n10;
        locals.var_cnst0_dn11 = assign17390_e11890_d_n11;
        locals.var_cnst0_dn14 = assign17390_e11890_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign17400_e11902, assign17400_e11902_d_n0, assign17400_e11902_d_n2, assign17400_e11902_d_n4, assign17400_e11902_d_n5, assign17400_e11902_d_n6, assign17400_e11902_d_n7, assign17400_e11902_d_n8, assign17400_e11902_d_n9, assign17400_e11902_d_n10, assign17400_e11902_d_n11, assign17400_e11902_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17400_e11896: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17400_e11898: f64 = (assign17400_e11896 * __rspice_inv_cse_0);
        let assign17400_e11900: f64 = (assign17400_e11898 * __rspice_inv_cse_0);
        (assign17400_e11900, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign17400_e11896 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17400_e11898 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign17400_e11902;
        locals.var_cnst1_dn0 = assign17400_e11902_d_n0;
        locals.var_cnst1_dn2 = assign17400_e11902_d_n2;
        locals.var_cnst1_dn4 = assign17400_e11902_d_n4;
        locals.var_cnst1_dn5 = assign17400_e11902_d_n5;
        locals.var_cnst1_dn6 = assign17400_e11902_d_n6;
        locals.var_cnst1_dn7 = assign17400_e11902_d_n7;
        locals.var_cnst1_dn8 = assign17400_e11902_d_n8;
        locals.var_cnst1_dn9 = assign17400_e11902_d_n9;
        locals.var_cnst1_dn10 = assign17400_e11902_d_n10;
        locals.var_cnst1_dn11 = assign17400_e11902_d_n11;
        locals.var_cnst1_dn14 = assign17400_e11902_d_n14;
        locals.var_cnst1_rv = 0.0;

        let (assign17410_e11915, assign17410_e11915_d_n0, assign17410_e11915_d_n2, assign17410_e11915_d_n4, assign17410_e11915_d_n5, assign17410_e11915_d_n6, assign17410_e11915_d_n7, assign17410_e11915_d_n8, assign17410_e11915_d_n9, assign17410_e11915_d_n10, assign17410_e11915_d_n11, assign17410_e11915_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17410_e11908: f64 = (2.0 * locals.var_beta_inv);
        let assign17410_e11911: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17410_e11912: f64 = (assign17410_e11911).ln();
        let assign17410_e11913: f64 = (assign17410_e11908 * assign17410_e11912);
        (assign17410_e11913, (((2.0 * locals.var_beta_inv_dn0) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn2) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn4) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn5) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn6) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn7) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn8) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn9) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn10) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn11) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))), (((2.0 * locals.var_beta_inv_dn14) * assign17410_e11912) + (assign17410_e11908 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17410_e11911))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17410_e11915;
        locals.var_pb2n_dn0 = assign17410_e11915_d_n0;
        locals.var_pb2n_dn2 = assign17410_e11915_d_n2;
        locals.var_pb2n_dn4 = assign17410_e11915_d_n4;
        locals.var_pb2n_dn5 = assign17410_e11915_d_n5;
        locals.var_pb2n_dn6 = assign17410_e11915_d_n6;
        locals.var_pb2n_dn7 = assign17410_e11915_d_n7;
        locals.var_pb2n_dn8 = assign17410_e11915_d_n8;
        locals.var_pb2n_dn9 = assign17410_e11915_d_n9;
        locals.var_pb2n_dn10 = assign17410_e11915_d_n10;
        locals.var_pb2n_dn11 = assign17410_e11915_d_n11;
        locals.var_pb2n_dn14 = assign17410_e11915_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign17420_e11930, assign17420_e11930_d_n0, assign17420_e11930_d_n2, assign17420_e11930_d_n4, assign17420_e11930_d_n5, assign17420_e11930_d_n6, assign17420_e11930_d_n7, assign17420_e11930_d_n8, assign17420_e11930_d_n9, assign17420_e11930_d_n10, assign17420_e11930_d_n11, assign17420_e11930_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17420_e11922: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17420_e11924: f64 = (assign17420_e11922 * __rspice_inv_cse_1);
        let assign17420_e11926: f64 = (assign17420_e11924 * __rspice_inv_cse_1);
        let assign17420_e11927: f64 = (assign17420_e11926).ln();
        let assign17420_e11928: f64 = (locals.var_beta_inv * assign17420_e11927);
        (assign17420_e11928, ((locals.var_beta_inv_dn0 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn2 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn4 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn5 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn6 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn7 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn8 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn9 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn10 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn11 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))), ((locals.var_beta_inv_dn14 * assign17420_e11927) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign17420_e11922 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17420_e11924 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17420_e11926))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17420_e11930;
        locals.var_vbipn_dn0 = assign17420_e11930_d_n0;
        locals.var_vbipn_dn2 = assign17420_e11930_d_n2;
        locals.var_vbipn_dn4 = assign17420_e11930_d_n4;
        locals.var_vbipn_dn5 = assign17420_e11930_d_n5;
        locals.var_vbipn_dn6 = assign17420_e11930_d_n6;
        locals.var_vbipn_dn7 = assign17420_e11930_d_n7;
        locals.var_vbipn_dn8 = assign17420_e11930_d_n8;
        locals.var_vbipn_dn9 = assign17420_e11930_d_n9;
        locals.var_vbipn_dn10 = assign17420_e11930_d_n10;
        locals.var_vbipn_dn11 = assign17420_e11930_d_n11;
        locals.var_vbipn_dn14 = assign17420_e11930_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign17430_e11939, assign17430_e11939_d_n0, assign17430_e11939_d_n2, assign17430_e11939_d_n4, assign17430_e11939_d_n5, assign17430_e11939_d_n6, assign17430_e11939_d_n7, assign17430_e11939_d_n8, assign17430_e11939_d_n9, assign17430_e11939_d_n10, assign17430_e11939_d_n11, assign17430_e11939_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17430_e11936: f64 = (locals.var_log_tratio * p.p380);
        let assign17430_e11937: f64 = (assign17430_e11936).exp();
        (assign17430_e11937, (assign17430_e11937 * (locals.var_log_tratio_dn0 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn2 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn4 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn5 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn6 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn7 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn8 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn9 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn10 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn11 * p.p380)), (assign17430_e11937 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17430_e11939;
        locals.var_t1_dn0 = assign17430_e11939_d_n0;
        locals.var_t1_dn2 = assign17430_e11939_d_n2;
        locals.var_t1_dn4 = assign17430_e11939_d_n4;
        locals.var_t1_dn5 = assign17430_e11939_d_n5;
        locals.var_t1_dn6 = assign17430_e11939_d_n6;
        locals.var_t1_dn7 = assign17430_e11939_d_n7;
        locals.var_t1_dn8 = assign17430_e11939_d_n8;
        locals.var_t1_dn9 = assign17430_e11939_d_n9;
        locals.var_t1_dn10 = assign17430_e11939_d_n10;
        locals.var_t1_dn11 = assign17430_e11939_d_n11;
        locals.var_t1_dn14 = assign17430_e11939_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17440_e11947, assign17440_e11947_d_n0, assign17440_e11947_d_n2, assign17440_e11947_d_n4, assign17440_e11947_d_n5, assign17440_e11947_d_n6, assign17440_e11947_d_n7, assign17440_e11947_d_n8, assign17440_e11947_d_n9, assign17440_e11947_d_n10, assign17440_e11947_d_n11, assign17440_e11947_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17440_e11945: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17440_e11945, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17440_e11947;
        locals.var_depmphn0_dn0 = assign17440_e11947_d_n0;
        locals.var_depmphn0_dn2 = assign17440_e11947_d_n2;
        locals.var_depmphn0_dn4 = assign17440_e11947_d_n4;
        locals.var_depmphn0_dn5 = assign17440_e11947_d_n5;
        locals.var_depmphn0_dn6 = assign17440_e11947_d_n6;
        locals.var_depmphn0_dn7 = assign17440_e11947_d_n7;
        locals.var_depmphn0_dn8 = assign17440_e11947_d_n8;
        locals.var_depmphn0_dn9 = assign17440_e11947_d_n9;
        locals.var_depmphn0_dn10 = assign17440_e11947_d_n10;
        locals.var_depmphn0_dn11 = assign17440_e11947_d_n11;
        locals.var_depmphn0_dn14 = assign17440_e11947_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign17450_e11969, assign17450_e11969_d_n0, assign17450_e11969_d_n2, assign17450_e11969_d_n4, assign17450_e11969_d_n5, assign17450_e11969_d_n6, assign17450_e11969_d_n7, assign17450_e11969_d_n8, assign17450_e11969_d_n9, assign17450_e11969_d_n10, assign17450_e11969_d_n11, assign17450_e11969_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17450_e11954: f64 = (0.4 * locals.var_tratio);
        let assign17450_e11955: f64 = (1.8 + assign17450_e11954);
        let assign17450_e11958: f64 = (0.1 * locals.var_tratio);
        let assign17450_e11960: f64 = (assign17450_e11958 * locals.var_tratio);
        let assign17450_e11961: f64 = (assign17450_e11955 + assign17450_e11960);
        let assign17450_e11965: f64 = (1.0 - locals.var_tratio);
        let assign17450_e11966: f64 = (p.p379 * assign17450_e11965);
        let assign17450_e11967: f64 = (assign17450_e11961 - assign17450_e11966);
        (assign17450_e11967, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign17450_e11958 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17450_e11969;
        locals.var_t0_dn0 = assign17450_e11969_d_n0;
        locals.var_t0_dn2 = assign17450_e11969_d_n2;
        locals.var_t0_dn4 = assign17450_e11969_d_n4;
        locals.var_t0_dn5 = assign17450_e11969_d_n5;
        locals.var_t0_dn6 = assign17450_e11969_d_n6;
        locals.var_t0_dn7 = assign17450_e11969_d_n7;
        locals.var_t0_dn8 = assign17450_e11969_d_n8;
        locals.var_t0_dn9 = assign17450_e11969_d_n9;
        locals.var_t0_dn10 = assign17450_e11969_d_n10;
        locals.var_t0_dn11 = assign17450_e11969_d_n11;
        locals.var_t0_dn14 = assign17450_e11969_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign17460_e11977, assign17460_e11977_d_n0, assign17460_e11977_d_n2, assign17460_e11977_d_n4, assign17460_e11977_d_n5, assign17460_e11977_d_n6, assign17460_e11977_d_n7, assign17460_e11977_d_n8, assign17460_e11977_d_n9, assign17460_e11977_d_n10, assign17460_e11977_d_n11, assign17460_e11977_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17460_e11975: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17460_e11975, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17460_e11977;
        locals.var_uc_depvmax_dn0 = assign17460_e11977_d_n0;
        locals.var_uc_depvmax_dn2 = assign17460_e11977_d_n2;
        locals.var_uc_depvmax_dn4 = assign17460_e11977_d_n4;
        locals.var_uc_depvmax_dn5 = assign17460_e11977_d_n5;
        locals.var_uc_depvmax_dn6 = assign17460_e11977_d_n6;
        locals.var_uc_depvmax_dn7 = assign17460_e11977_d_n7;
        locals.var_uc_depvmax_dn8 = assign17460_e11977_d_n8;
        locals.var_uc_depvmax_dn9 = assign17460_e11977_d_n9;
        locals.var_uc_depvmax_dn10 = assign17460_e11977_d_n10;
        locals.var_uc_depvmax_dn11 = assign17460_e11977_d_n11;
        locals.var_uc_depvmax_dn14 = assign17460_e11977_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17480_e11985: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard359 = assign17480_e11985;
        locals.var_guard359_rv = 0.0;

        let (assign17490_e11993, assign17490_e11993_d_n0, assign17490_e11993_d_n2, assign17490_e11993_d_n4, assign17490_e11993_d_n5, assign17490_e11993_d_n6, assign17490_e11993_d_n7, assign17490_e11993_d_n8, assign17490_e11993_d_n9, assign17490_e11993_d_n10, assign17490_e11993_d_n11, assign17490_e11993_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) && (locals.var_guard359 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17490_e11993;
        locals.var_uc_depvmax_dn0 = assign17490_e11993_d_n0;
        locals.var_uc_depvmax_dn2 = assign17490_e11993_d_n2;
        locals.var_uc_depvmax_dn4 = assign17490_e11993_d_n4;
        locals.var_uc_depvmax_dn5 = assign17490_e11993_d_n5;
        locals.var_uc_depvmax_dn6 = assign17490_e11993_d_n6;
        locals.var_uc_depvmax_dn7 = assign17490_e11993_d_n7;
        locals.var_uc_depvmax_dn8 = assign17490_e11993_d_n8;
        locals.var_uc_depvmax_dn9 = assign17490_e11993_d_n9;
        locals.var_uc_depvmax_dn10 = assign17490_e11993_d_n10;
        locals.var_uc_depvmax_dn11 = assign17490_e11993_d_n11;
        locals.var_uc_depvmax_dn14 = assign17490_e11993_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17500_e12003, assign17500_e12003_d_n0, assign17500_e12003_d_n2, assign17500_e12003_d_n4, assign17500_e12003_d_n5, assign17500_e12003_d_n6, assign17500_e12003_d_n7, assign17500_e12003_d_n8, assign17500_e12003_d_n9, assign17500_e12003_d_n10, assign17500_e12003_d_n11, assign17500_e12003_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17500_e12000: f64 = (locals.var_tratio).powf(p.p381);
        let assign17500_e12001: f64 = (locals.var_uc_depmue0 / assign17500_e12000);
        (assign17500_e12001, (((locals.var_uc_depmue0_dn0 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn2 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn4 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn5 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn6 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn7 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn8 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn9 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn10 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn11 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)), (((locals.var_uc_depmue0_dn14 * assign17500_e12000) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17500_e12000 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17500_e12000 * assign17500_e12000)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign17500_e12003;
        locals.var_uc_depmue0_dn0 = assign17500_e12003_d_n0;
        locals.var_uc_depmue0_dn2 = assign17500_e12003_d_n2;
        locals.var_uc_depmue0_dn4 = assign17500_e12003_d_n4;
        locals.var_uc_depmue0_dn5 = assign17500_e12003_d_n5;
        locals.var_uc_depmue0_dn6 = assign17500_e12003_d_n6;
        locals.var_uc_depmue0_dn7 = assign17500_e12003_d_n7;
        locals.var_uc_depmue0_dn8 = assign17500_e12003_d_n8;
        locals.var_uc_depmue0_dn9 = assign17500_e12003_d_n9;
        locals.var_uc_depmue0_dn10 = assign17500_e12003_d_n10;
        locals.var_uc_depmue0_dn11 = assign17500_e12003_d_n11;
        locals.var_uc_depmue0_dn14 = assign17500_e12003_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17510_e12013, assign17510_e12013_d_n0, assign17510_e12013_d_n2, assign17510_e12013_d_n4, assign17510_e12013_d_n5, assign17510_e12013_d_n6, assign17510_e12013_d_n7, assign17510_e12013_d_n8, assign17510_e12013_d_n9, assign17510_e12013_d_n10, assign17510_e12013_d_n11, assign17510_e12013_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17510_e12010: f64 = (locals.var_tratio).powf(p.p382);
        let assign17510_e12011: f64 = (locals.var_uc_depmue2 / assign17510_e12010);
        (assign17510_e12011, (((locals.var_uc_depmue2_dn0 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn2 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn4 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn5 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn6 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn7 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn8 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn9 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn10 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn11 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)), (((locals.var_uc_depmue2_dn14 * assign17510_e12010) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17510_e12010 * (p.p382 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17510_e12010 * assign17510_e12010)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn11, locals.var_uc_depmue2_dn14,)
    }
};
        locals.var_uc_depmue2 = assign17510_e12013;
        locals.var_uc_depmue2_dn0 = assign17510_e12013_d_n0;
        locals.var_uc_depmue2_dn2 = assign17510_e12013_d_n2;
        locals.var_uc_depmue2_dn4 = assign17510_e12013_d_n4;
        locals.var_uc_depmue2_dn5 = assign17510_e12013_d_n5;
        locals.var_uc_depmue2_dn6 = assign17510_e12013_d_n6;
        locals.var_uc_depmue2_dn7 = assign17510_e12013_d_n7;
        locals.var_uc_depmue2_dn8 = assign17510_e12013_d_n8;
        locals.var_uc_depmue2_dn9 = assign17510_e12013_d_n9;
        locals.var_uc_depmue2_dn10 = assign17510_e12013_d_n10;
        locals.var_uc_depmue2_dn11 = assign17510_e12013_d_n11;
        locals.var_uc_depmue2_dn14 = assign17510_e12013_d_n14;
        locals.var_uc_depmue2_rv = 0.0;

        let assign17520_e12016: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign17520_e12016;
        locals.var_guard360_rv = 0.0;

        let (assign17530_e12034, assign17530_e12034_d_n0, assign17530_e12034_d_n2, assign17530_e12034_d_n4, assign17530_e12034_d_n5, assign17530_e12034_d_n6, assign17530_e12034_d_n7, assign17530_e12034_d_n8, assign17530_e12034_d_n9, assign17530_e12034_d_n10, assign17530_e12034_d_n11, assign17530_e12034_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard357 == 0.0)) && (locals.var_guard360 != 0.0)) {
        let assign17530_e12025: f64 = (2.0 * 1.034943e-10);
        let assign17530_e12027: f64 = (assign17530_e12025 * 1.6021918e-19);
        let assign17530_e12029: f64 = (assign17530_e12027 * locals.var_uc_ndepm);
        let assign17530_e12031: f64 = (assign17530_e12029 * locals.var_beta_inv);
        let assign17530_e12032: f64 = (assign17530_e12031).sqrt();
        (assign17530_e12032, ((((assign17530_e12027 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn0)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn2)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn4)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn5)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn6)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn7)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn8)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn9)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn10)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn11)) / (2.0 * assign17530_e12032)), ((((assign17530_e12027 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign17530_e12029 * locals.var_beta_inv_dn14)) / (2.0 * assign17530_e12032)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign17530_e12034;
        locals.var_cnst0_dn0 = assign17530_e12034_d_n0;
        locals.var_cnst0_dn2 = assign17530_e12034_d_n2;
        locals.var_cnst0_dn4 = assign17530_e12034_d_n4;
        locals.var_cnst0_dn5 = assign17530_e12034_d_n5;
        locals.var_cnst0_dn6 = assign17530_e12034_d_n6;
        locals.var_cnst0_dn7 = assign17530_e12034_d_n7;
        locals.var_cnst0_dn8 = assign17530_e12034_d_n8;
        locals.var_cnst0_dn9 = assign17530_e12034_d_n9;
        locals.var_cnst0_dn10 = assign17530_e12034_d_n10;
        locals.var_cnst0_dn11 = assign17530_e12034_d_n11;
        locals.var_cnst0_dn14 = assign17530_e12034_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign17540_e12049, assign17540_e12049_d_n0, assign17540_e12049_d_n2, assign17540_e12049_d_n4, assign17540_e12049_d_n5, assign17540_e12049_d_n6, assign17540_e12049_d_n7, assign17540_e12049_d_n8, assign17540_e12049_d_n9, assign17540_e12049_d_n10, assign17540_e12049_d_n11, assign17540_e12049_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard357 == 0.0)) && (locals.var_guard360 != 0.0)) {
        let assign17540_e12043: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17540_e12045: f64 = (assign17540_e12043 * __rspice_inv_cse_2);
        let assign17540_e12047: f64 = (assign17540_e12045 * __rspice_inv_cse_2);
        (assign17540_e12047, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign17540_e12043 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17540_e12045 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign17540_e12049;
        locals.var_cnst1_dn0 = assign17540_e12049_d_n0;
        locals.var_cnst1_dn2 = assign17540_e12049_d_n2;
        locals.var_cnst1_dn4 = assign17540_e12049_d_n4;
        locals.var_cnst1_dn5 = assign17540_e12049_d_n5;
        locals.var_cnst1_dn6 = assign17540_e12049_d_n6;
        locals.var_cnst1_dn7 = assign17540_e12049_d_n7;
        locals.var_cnst1_dn8 = assign17540_e12049_d_n8;
        locals.var_cnst1_dn9 = assign17540_e12049_d_n9;
        locals.var_cnst1_dn10 = assign17540_e12049_d_n10;
        locals.var_cnst1_dn11 = assign17540_e12049_d_n11;
        locals.var_cnst1_dn14 = assign17540_e12049_d_n14;
        locals.var_cnst1_rv = 0.0;

        let (assign17550_e12065, assign17550_e12065_d_n0, assign17550_e12065_d_n2, assign17550_e12065_d_n4, assign17550_e12065_d_n5, assign17550_e12065_d_n6, assign17550_e12065_d_n7, assign17550_e12065_d_n8, assign17550_e12065_d_n9, assign17550_e12065_d_n10, assign17550_e12065_d_n11, assign17550_e12065_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard357 == 0.0)) && (locals.var_guard360 != 0.0)) {
        let assign17550_e12058: f64 = (2.0 * locals.var_beta_inv);
        let assign17550_e12061: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17550_e12062: f64 = (assign17550_e12061).ln();
        let assign17550_e12063: f64 = (assign17550_e12058 * assign17550_e12062);
        (assign17550_e12063, (((2.0 * locals.var_beta_inv_dn0) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn2) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn4) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn5) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn6) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn7) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn8) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn9) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn10) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn11) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))), (((2.0 * locals.var_beta_inv_dn14) * assign17550_e12062) + (assign17550_e12058 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17550_e12061))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17550_e12065;
        locals.var_pb2n_dn0 = assign17550_e12065_d_n0;
        locals.var_pb2n_dn2 = assign17550_e12065_d_n2;
        locals.var_pb2n_dn4 = assign17550_e12065_d_n4;
        locals.var_pb2n_dn5 = assign17550_e12065_d_n5;
        locals.var_pb2n_dn6 = assign17550_e12065_d_n6;
        locals.var_pb2n_dn7 = assign17550_e12065_d_n7;
        locals.var_pb2n_dn8 = assign17550_e12065_d_n8;
        locals.var_pb2n_dn9 = assign17550_e12065_d_n9;
        locals.var_pb2n_dn10 = assign17550_e12065_d_n10;
        locals.var_pb2n_dn11 = assign17550_e12065_d_n11;
        locals.var_pb2n_dn14 = assign17550_e12065_d_n14;
        locals.var_pb2n_rv = 0.0;

    }
}
