#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17680_e17262, assign17680_e17262_d_n2, assign17680_e17262_d_n3, assign17680_e17262_d_n4, assign17680_e17262_d_n7, assign17680_e17262_d_n10, assign17680_e17262_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard218 == 0.0)) {
        let assign17680_e17257: f64 = (locals.var_fn205_calc_iq__etad).exp();
        let assign17680_e17258: f64 = (1.0 + assign17680_e17257);
        let assign17680_e17259: f64 = (assign17680_e17258).ln();
        let assign17680_e17260: f64 = (locals.var_fn205_calc_iq__qref * assign17680_e17259);
        (assign17680_e17260, (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn2) / assign17680_e17258)), (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn3) / assign17680_e17258)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn4) / assign17680_e17258))), (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn7) / assign17680_e17258)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn10) / assign17680_e17258))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn11) / assign17680_e17258))),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17680_e17262;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17680_e17262_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17680_e17262_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17680_e17262_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17680_e17262_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17680_e17262_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17680_e17262_d_n11;

        let (assign17690_e17270, assign17690_e17270_d_n2, assign17690_e17270_d_n3, assign17690_e17270_d_n4, assign17690_e17270_d_n7, assign17690_e17270_d_n10, assign17690_e17270_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17690_e17266: f64 = (locals.var_fn205_calc_iq__qinvs - locals.var_fn205_calc_iq__qinvd);
        let assign17690_e17268: f64 = (assign17690_e17266 / locals.var_fn205_calc_iq__cgin);
        (assign17690_e17268, ((locals.var_fn205_calc_iq__qinvs_dn2 - locals.var_fn205_calc_iq__qinvd_dn2) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn3 - locals.var_fn205_calc_iq__qinvd_dn3) / locals.var_fn205_calc_iq__cgin), ((((locals.var_fn205_calc_iq__qinvs_dn4 - locals.var_fn205_calc_iq__qinvd_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17690_e17266 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)), ((locals.var_fn205_calc_iq__qinvs_dn7 - locals.var_fn205_calc_iq__qinvd_dn7) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn10 - locals.var_fn205_calc_iq__qinvd_dn10) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn11 - locals.var_fn205_calc_iq__qinvd_dn11) / locals.var_fn205_calc_iq__cgin),)
    } else {
        (locals.var_fn205_calc_iq__vdsc, locals.var_fn205_calc_iq__vdsc_dn2, locals.var_fn205_calc_iq__vdsc_dn3, locals.var_fn205_calc_iq__vdsc_dn4, locals.var_fn205_calc_iq__vdsc_dn7, locals.var_fn205_calc_iq__vdsc_dn10, locals.var_fn205_calc_iq__vdsc_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsc = assign17690_e17270;
        locals.var_fn205_calc_iq__vdsc_dn2 = assign17690_e17270_d_n2;
        locals.var_fn205_calc_iq__vdsc_dn3 = assign17690_e17270_d_n3;
        locals.var_fn205_calc_iq__vdsc_dn4 = assign17690_e17270_d_n4;
        locals.var_fn205_calc_iq__vdsc_dn7 = assign17690_e17270_d_n7;
        locals.var_fn205_calc_iq__vdsc_dn10 = assign17690_e17270_d_n10;
        locals.var_fn205_calc_iq__vdsc_dn11 = assign17690_e17270_d_n11;

        let (assign17700_e17276, assign17700_e17276_d_n2, assign17700_e17276_d_n3, assign17700_e17276_d_n4, assign17700_e17276_d_n7, assign17700_e17276_d_n10, assign17700_e17276_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17700_e17274: f64 = (locals.var_fn205_calc_iq__vdsc / locals.var_fn205_calc_iq__vdsat);
        (assign17700_e17274, (((locals.var_fn205_calc_iq__vdsc_dn2 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn2)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn3 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn3)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn4 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn4)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn7 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn7)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn10 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn10)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn11 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn11)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)),)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17700_e17276;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17700_e17276_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17700_e17276_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17700_e17276_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17700_e17276_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17700_e17276_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17700_e17276_d_n11;

        let (assign17710_e17313, assign17710_e17313_d_n2, assign17710_e17313_d_n3, assign17710_e17313_d_n4, assign17710_e17313_d_n7, assign17710_e17313_d_n10, assign17710_e17313_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17710_e17303, assign17710_e17303_d_n2, assign17710_e17303_d_n3, assign17710_e17303_d_n4, assign17710_e17303_d_n7, assign17710_e17303_d_n10, assign17710_e17303_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17710_e17287: f64 = (0.001 / p.p53);
                let assign17710_e17289: f64 = (assign17710_e17287 * locals.var_fn205_calc_iq__myarg);
                let assign17710_e17290: f64 = (assign17710_e17289).tanh();
                let assign17710_e17291: f64 = (locals.var_fn205_calc_iq__myarg * assign17710_e17290);
                (assign17710_e17291, ((locals.var_fn205_calc_iq__myarg_dn2 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn2) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))), ((locals.var_fn205_calc_iq__myarg_dn3 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn3) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))), ((locals.var_fn205_calc_iq__myarg_dn4 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn4) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))), ((locals.var_fn205_calc_iq__myarg_dn7 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn7) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))), ((locals.var_fn205_calc_iq__myarg_dn10 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn10) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))), ((locals.var_fn205_calc_iq__myarg_dn11 * assign17710_e17290) + (locals.var_fn205_calc_iq__myarg * ((assign17710_e17287 * locals.var_fn205_calc_iq__myarg_dn11) / ((assign17710_e17289).cosh() * (assign17710_e17289).cosh())))),)
            } else {
                let (assign17710_e17302, assign17710_e17302_d_n2, assign17710_e17302_d_n3, assign17710_e17302_d_n4, assign17710_e17302_d_n7, assign17710_e17302_d_n10, assign17710_e17302_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17710_e17297: f64 = (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg);
                        let assign17710_e17299: f64 = (assign17710_e17297 + p.p53);
                        let assign17710_e17300: f64 = (assign17710_e17299).sqrt();
                        (assign17710_e17300, (((locals.var_fn205_calc_iq__myarg_dn2 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn2)) / (2.0 * assign17710_e17300)), (((locals.var_fn205_calc_iq__myarg_dn3 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn3)) / (2.0 * assign17710_e17300)), (((locals.var_fn205_calc_iq__myarg_dn4 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn4)) / (2.0 * assign17710_e17300)), (((locals.var_fn205_calc_iq__myarg_dn7 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn7)) / (2.0 * assign17710_e17300)), (((locals.var_fn205_calc_iq__myarg_dn10 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn10)) / (2.0 * assign17710_e17300)), (((locals.var_fn205_calc_iq__myarg_dn11 * locals.var_fn205_calc_iq__myarg) + (locals.var_fn205_calc_iq__myarg * locals.var_fn205_calc_iq__myarg_dn11)) / (2.0 * assign17710_e17300)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17710_e17302, assign17710_e17302_d_n2, assign17710_e17302_d_n3, assign17710_e17302_d_n4, assign17710_e17302_d_n7, assign17710_e17302_d_n10, assign17710_e17302_d_n11,)
            }
        };
        let assign17710_e17305: f64 = (assign17710_e17303).powf(locals.var_fn205_calc_iq__beta);
        let assign17710_e17306: f64 = (1.0 + assign17710_e17305);
        let assign17710_e17309: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17710_e17310: f64 = (assign17710_e17306).powf(assign17710_e17309);
        let assign17710_e17311: f64 = (locals.var_fn205_calc_iq__myarg / assign17710_e17310);
        (assign17710_e17311, (((locals.var_fn205_calc_iq__myarg_dn2 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n2)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n2 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n2)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n2 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)), (((locals.var_fn205_calc_iq__myarg_dn3 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n3)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n3 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n3)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n3 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)), (((locals.var_fn205_calc_iq__myarg_dn4 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n4)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n4 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n4)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n4 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)), (((locals.var_fn205_calc_iq__myarg_dn7 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n7)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n7 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n7)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n7 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)), (((locals.var_fn205_calc_iq__myarg_dn10 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n10)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n10 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n10)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n10 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)), (((locals.var_fn205_calc_iq__myarg_dn11 * assign17710_e17310) - (locals.var_fn205_calc_iq__myarg * if 0.0 == 0.0 && ((assign17710_e17309) as f64).is_finite() && ((assign17710_e17309) as f64).fract() == 0.0 { if assign17710_e17309 == 0.0 { 0.0 } else { (assign17710_e17309 * ((assign17710_e17306).powf(assign17710_e17309 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n11)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n11 / assign17710_e17303))) })) } } else { (assign17710_e17310 * (assign17710_e17309 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17710_e17303).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17710_e17303_d_n11)) } } else { (assign17710_e17305 * (locals.var_fn205_calc_iq__beta * (assign17710_e17303_d_n11 / assign17710_e17303))) } / assign17710_e17306))) })) / (assign17710_e17310 * assign17710_e17310)),)
    } else {
        (locals.var_fn205_calc_iq__fsat, locals.var_fn205_calc_iq__fsat_dn2, locals.var_fn205_calc_iq__fsat_dn3, locals.var_fn205_calc_iq__fsat_dn4, locals.var_fn205_calc_iq__fsat_dn7, locals.var_fn205_calc_iq__fsat_dn10, locals.var_fn205_calc_iq__fsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsat = assign17710_e17313;
        locals.var_fn205_calc_iq__fsat_dn2 = assign17710_e17313_d_n2;
        locals.var_fn205_calc_iq__fsat_dn3 = assign17710_e17313_d_n3;
        locals.var_fn205_calc_iq__fsat_dn4 = assign17710_e17313_d_n4;
        locals.var_fn205_calc_iq__fsat_dn7 = assign17710_e17313_d_n7;
        locals.var_fn205_calc_iq__fsat_dn10 = assign17710_e17313_d_n10;
        locals.var_fn205_calc_iq__fsat_dn11 = assign17710_e17313_d_n11;

        let (assign17720_e17319, assign17720_e17319_d_n2, assign17720_e17319_d_n3, assign17720_e17319_d_n4, assign17720_e17319_d_n7, assign17720_e17319_d_n10, assign17720_e17319_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17720_e17317: f64 = (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat);
        (assign17720_e17317, ((locals.var_fn205_calc_iq__vxf_dn2 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn2)), ((locals.var_fn205_calc_iq__vxf_dn3 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn3)), ((locals.var_fn205_calc_iq__vxf_dn4 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn4)), ((locals.var_fn205_calc_iq__vxf_dn7 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn7)), ((locals.var_fn205_calc_iq__vxf_dn10 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn10)), ((locals.var_fn205_calc_iq__vxf_dn11 * locals.var_fn205_calc_iq__fsat) + (locals.var_fn205_calc_iq__vxf * locals.var_fn205_calc_iq__fsat_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vel, locals.var_fn205_calc_iq__vel_dn2, locals.var_fn205_calc_iq__vel_dn3, locals.var_fn205_calc_iq__vel_dn4, locals.var_fn205_calc_iq__vel_dn7, locals.var_fn205_calc_iq__vel_dn10, locals.var_fn205_calc_iq__vel_dn11,)
    }
};
        locals.var_fn205_calc_iq__vel = assign17720_e17319;
        locals.var_fn205_calc_iq__vel_dn2 = assign17720_e17319_d_n2;
        locals.var_fn205_calc_iq__vel_dn3 = assign17720_e17319_d_n3;
        locals.var_fn205_calc_iq__vel_dn4 = assign17720_e17319_d_n4;
        locals.var_fn205_calc_iq__vel_dn7 = assign17720_e17319_d_n7;
        locals.var_fn205_calc_iq__vel_dn10 = assign17720_e17319_d_n10;
        locals.var_fn205_calc_iq__vel_dn11 = assign17720_e17319_d_n11;

        let (assign17730_e17337, assign17730_e17337_d_n2, assign17730_e17337_d_n3, assign17730_e17337_d_n4, assign17730_e17337_d_n7, assign17730_e17337_d_n10, assign17730_e17337_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17730_e17323: f64 = (locals.var_fn205_calc_iq__type * locals.var_fn205_calc_iq__w);
        let assign17730_e17325: f64 = (assign17730_e17323 * locals.var_fn205_calc_iq__ngf);
        let assign17730_e17327: f64 = (assign17730_e17325 * 0.5);
        let assign17730_e17330: f64 = (locals.var_fn205_calc_iq__qinvs + locals.var_fn205_calc_iq__qinvd);
        let assign17730_e17331: f64 = (assign17730_e17327 * assign17730_e17330);
        let assign17730_e17333: f64 = (assign17730_e17331 * locals.var_fn205_calc_iq__vel);
        let assign17730_e17335: f64 = (assign17730_e17333 * locals.var_fn205_calc_iq__trapfracdl);
        (assign17730_e17335, ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn2 + locals.var_fn205_calc_iq__qinvd_dn2)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn2)) * locals.var_fn205_calc_iq__trapfracdl), ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn3 + locals.var_fn205_calc_iq__qinvd_dn3)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn3)) * locals.var_fn205_calc_iq__trapfracdl), ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn4 + locals.var_fn205_calc_iq__qinvd_dn4)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn7 + locals.var_fn205_calc_iq__qinvd_dn7)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn7)) * locals.var_fn205_calc_iq__trapfracdl), ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn10 + locals.var_fn205_calc_iq__qinvd_dn10)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn10)) * locals.var_fn205_calc_iq__trapfracdl), ((((assign17730_e17327 * (locals.var_fn205_calc_iq__qinvs_dn11 + locals.var_fn205_calc_iq__qinvd_dn11)) * locals.var_fn205_calc_iq__vel) + (assign17730_e17331 * locals.var_fn205_calc_iq__vel_dn11)) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__idsout, locals.var_fn205_calc_iq__idsout_dn2, locals.var_fn205_calc_iq__idsout_dn3, locals.var_fn205_calc_iq__idsout_dn4, locals.var_fn205_calc_iq__idsout_dn7, locals.var_fn205_calc_iq__idsout_dn10, locals.var_fn205_calc_iq__idsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__idsout = assign17730_e17337;
        locals.var_fn205_calc_iq__idsout_dn2 = assign17730_e17337_d_n2;
        locals.var_fn205_calc_iq__idsout_dn3 = assign17730_e17337_d_n3;
        locals.var_fn205_calc_iq__idsout_dn4 = assign17730_e17337_d_n4;
        locals.var_fn205_calc_iq__idsout_dn7 = assign17730_e17337_d_n7;
        locals.var_fn205_calc_iq__idsout_dn10 = assign17730_e17337_d_n10;
        locals.var_fn205_calc_iq__idsout_dn11 = assign17730_e17337_d_n11;

        let (assign17740_e17345, assign17740_e17345_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17740_e17342: f64 = (2.302585092994046 * locals.var_fn205_calc_iq__phitin);
        let assign17740_e17343: f64 = (locals.var_fn205_calc_iq__ss / assign17740_e17342);
        (assign17740_e17343, (-((locals.var_fn205_calc_iq__ss * (2.302585092994046 * locals.var_fn205_calc_iq__phitin_dn4)) / (assign17740_e17342 * assign17740_e17342))),)
    } else {
        (locals.var_fn205_calc_iq__n0, locals.var_fn205_calc_iq__n0_dn4,)
    }
};
        locals.var_fn205_calc_iq__n0 = assign17740_e17345;
        locals.var_fn205_calc_iq__n0_dn4 = assign17740_e17345_d_n4;

        let (assign17750_e17353, assign17750_e17353_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17750_e17349: f64 = (2.0 * locals.var_fn205_calc_iq__n0);
        let assign17750_e17351: f64 = (assign17750_e17349 * locals.var_fn205_calc_iq__phitin);
        (assign17750_e17351, (((2.0 * locals.var_fn205_calc_iq__n0_dn4) * locals.var_fn205_calc_iq__phitin) + (assign17750_e17349 * locals.var_fn205_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit0, locals.var_fn205_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit0 = assign17750_e17353;
        locals.var_fn205_calc_iq__two_n_phit0_dn4 = assign17750_e17353_d_n4;

        let (assign17760_e17359, assign17760_e17359_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17760_e17357: f64 = (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit0);
        (assign17760_e17357, ((locals.var_fn205_calc_iq__cgin_dn4 * locals.var_fn205_calc_iq__two_n_phit0) + (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn205_calc_iq__qref0, locals.var_fn205_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn205_calc_iq__qref0 = assign17760_e17359;
        locals.var_fn205_calc_iq__qref0_dn4 = assign17760_e17359_d_n4;

        let (assign17770_e17369, assign17770_e17369_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17770_e17364: f64 = (p.p51 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17770_e17366: f64 = (assign17770_e17364 / 2.0);
        let assign17770_e17367: f64 = (locals.var_fn205_calc_iq__vtof - assign17770_e17366);
        (assign17770_e17367, (locals.var_fn205_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn205_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn205_calc_iq__myarg0, locals.var_fn205_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn205_calc_iq__myarg0 = assign17770_e17369;
        locals.var_fn205_calc_iq__myarg0_dn4 = assign17770_e17369_d_n4;

        let (assign17780_e17420, assign17780_e17420_d_n2, assign17780_e17420_d_n4, assign17780_e17420_d_n7, assign17780_e17420_d_n10, assign17780_e17420_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17780_e17414, assign17780_e17414_d_n2, assign17780_e17414_d_n7, assign17780_e17414_d_n10, assign17780_e17414_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17780_e17378: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17381: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17384: f64 = (0.001 / p.p53);
                let assign17780_e17387: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17388: f64 = (assign17780_e17384 * assign17780_e17387);
                let assign17780_e17389: f64 = (assign17780_e17388).tanh();
                let assign17780_e17390: f64 = (assign17780_e17381 * assign17780_e17389);
                let assign17780_e17391: f64 = (assign17780_e17378 + assign17780_e17390);
                let assign17780_e17392: f64 = (0.5 * assign17780_e17391);
                (assign17780_e17392, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))),)
            } else {
                let (assign17780_e17413, assign17780_e17413_d_n2, assign17780_e17413_d_n7, assign17780_e17413_d_n10, assign17780_e17413_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17780_e17399: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17402: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17405: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17406: f64 = (assign17780_e17402 * assign17780_e17405);
                        let assign17780_e17408: f64 = (assign17780_e17406 + p.p53);
                        let assign17780_e17409: f64 = (assign17780_e17408).sqrt();
                        let assign17780_e17410: f64 = (assign17780_e17399 + assign17780_e17409);
                        let assign17780_e17411: f64 = (0.5 * assign17780_e17410);
                        (assign17780_e17411, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17780_e17409)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17780_e17409)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17780_e17405) + (assign17780_e17402 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17780_e17409)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17780_e17409)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17780_e17413, assign17780_e17413_d_n2, assign17780_e17413_d_n7, assign17780_e17413_d_n10, assign17780_e17413_d_n11,)
            }
        };
        let assign17780_e17416: f64 = (assign17780_e17414 - locals.var_fn205_calc_iq__myarg0);
        let assign17780_e17418: f64 = (assign17780_e17416 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17780_e17418, (assign17780_e17414_d_n2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17780_e17416 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (assign17780_e17414_d_n7 / locals.var_fn205_calc_iq__alpha_phit), (assign17780_e17414_d_n10 / locals.var_fn205_calc_iq__alpha_phit), (assign17780_e17414_d_n11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17780_e17420;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17780_e17420_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17780_e17420_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17780_e17420_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17780_e17420_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17780_e17420_d_n11;

        let assign17790_e17423: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard219 = assign17790_e17423;

        let (assign17800_e17429, assign17800_e17429_d_n2, assign17800_e17429_d_n4, assign17800_e17429_d_n7, assign17800_e17429_d_n10, assign17800_e17429_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard219 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17800_e17429;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17800_e17429_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17800_e17429_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17800_e17429_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17800_e17429_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17800_e17429_d_n11;

        let assign17810_e17432: f64 = (-50.0);
        let assign17810_e17433: f64 = if locals.var_fn205_calc_iq__exparg0 < assign17810_e17432 { 1.0 } else { 0.0 };
        locals.var_guard220 = assign17810_e17433;

        let (assign17820_e17442, assign17820_e17442_d_n2, assign17820_e17442_d_n4, assign17820_e17442_d_n7, assign17820_e17442_d_n10, assign17820_e17442_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17820_e17442;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17820_e17442_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17820_e17442_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17820_e17442_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17820_e17442_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17820_e17442_d_n11;

        let (assign17830_e17457, assign17830_e17457_d_n2, assign17830_e17457_d_n4, assign17830_e17457_d_n7, assign17830_e17457_d_n10, assign17830_e17457_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign17830_e17453: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign17830_e17454: f64 = (1.0 + assign17830_e17453);
        let assign17830_e17455: f64 = (1.0 / assign17830_e17454);
        (assign17830_e17455, (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign17830_e17454 * assign17830_e17454))),)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17830_e17457;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17830_e17457_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17830_e17457_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17830_e17457_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17830_e17457_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17830_e17457_d_n11;

        let (assign17840_e17516, assign17840_e17516_d_n2, assign17840_e17516_d_n4, assign17840_e17516_d_n7, assign17840_e17516_d_n10, assign17840_e17516_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17840_e17502, assign17840_e17502_d_n2, assign17840_e17502_d_n7, assign17840_e17502_d_n10, assign17840_e17502_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17840_e17466: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17469: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17472: f64 = (0.001 / p.p53);
                let assign17840_e17475: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17476: f64 = (assign17840_e17472 * assign17840_e17475);
                let assign17840_e17477: f64 = (assign17840_e17476).tanh();
                let assign17840_e17478: f64 = (assign17840_e17469 * assign17840_e17477);
                let assign17840_e17479: f64 = (assign17840_e17466 + assign17840_e17478);
                let assign17840_e17480: f64 = (0.5 * assign17840_e17479);
                (assign17840_e17480, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))),)
            } else {
                let (assign17840_e17501, assign17840_e17501_d_n2, assign17840_e17501_d_n7, assign17840_e17501_d_n10, assign17840_e17501_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17840_e17487: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17490: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17493: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17494: f64 = (assign17840_e17490 * assign17840_e17493);
                        let assign17840_e17496: f64 = (assign17840_e17494 + p.p53);
                        let assign17840_e17497: f64 = (assign17840_e17496).sqrt();
                        let assign17840_e17498: f64 = (assign17840_e17487 + assign17840_e17497);
                        let assign17840_e17499: f64 = (0.5 * assign17840_e17498);
                        (assign17840_e17499, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17840_e17497)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17840_e17497)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17840_e17493) + (assign17840_e17490 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17840_e17497)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17840_e17497)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17840_e17501, assign17840_e17501_d_n2, assign17840_e17501_d_n7, assign17840_e17501_d_n10, assign17840_e17501_d_n11,)
            }
        };
        let assign17840_e17506: f64 = (p.p51 * 0.1);
        let assign17840_e17508: f64 = (assign17840_e17506 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17840_e17510: f64 = (assign17840_e17508 * locals.var_fn205_calc_iq__ff0);
        let assign17840_e17511: f64 = (locals.var_fn205_calc_iq__vtof - assign17840_e17510);
        let assign17840_e17512: f64 = (assign17840_e17502 - assign17840_e17511);
        let assign17840_e17514: f64 = (assign17840_e17512 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign17840_e17514, ((assign17840_e17502_d_n2 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (((assign17840_e17506 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ff0) + (assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign17840_e17512 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), ((assign17840_e17502_d_n7 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), ((assign17840_e17502_d_n10 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), ((assign17840_e17502_d_n11 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__eta0, locals.var_fn205_calc_iq__eta0_dn2, locals.var_fn205_calc_iq__eta0_dn4, locals.var_fn205_calc_iq__eta0_dn7, locals.var_fn205_calc_iq__eta0_dn10, locals.var_fn205_calc_iq__eta0_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta0 = assign17840_e17516;
        locals.var_fn205_calc_iq__eta0_dn2 = assign17840_e17516_d_n2;
        locals.var_fn205_calc_iq__eta0_dn4 = assign17840_e17516_d_n4;
        locals.var_fn205_calc_iq__eta0_dn7 = assign17840_e17516_d_n7;
        locals.var_fn205_calc_iq__eta0_dn10 = assign17840_e17516_d_n10;
        locals.var_fn205_calc_iq__eta0_dn11 = assign17840_e17516_d_n11;

        let assign17850_e17519: f64 = if locals.var_fn205_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard221 = assign17850_e17519;

        let (assign17860_e17527, assign17860_e17527_d_n2, assign17860_e17527_d_n4, assign17860_e17527_d_n7, assign17860_e17527_d_n10, assign17860_e17527_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard221 != 0.0)) {
        let assign17860_e17525: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0);
        (assign17860_e17525, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__eta0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17860_e17527;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17860_e17527_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17860_e17527_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17860_e17527_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17860_e17527_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17860_e17527_d_n11;

        let assign17870_e17530: f64 = (-50.0);
        let assign17870_e17531: f64 = if locals.var_fn205_calc_iq__eta0 < assign17870_e17530 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign17870_e17531;

        let (assign17880_e17543, assign17880_e17543_d_n2, assign17880_e17543_d_n4, assign17880_e17543_d_n7, assign17880_e17543_d_n10, assign17880_e17543_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard221 == 0.0)) && (locals.var_guard222 != 0.0)) {
        let assign17880_e17540: f64 = (locals.var_fn205_calc_iq__eta0).exp();
        let assign17880_e17541: f64 = (locals.var_fn205_calc_iq__qref0 * assign17880_e17540);
        (assign17880_e17541, (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign17880_e17540) + (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17880_e17543;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17880_e17543_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17880_e17543_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17880_e17543_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17880_e17543_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17880_e17543_d_n11;

        let (assign17890_e17559, assign17890_e17559_d_n2, assign17890_e17559_d_n4, assign17890_e17559_d_n7, assign17890_e17559_d_n10, assign17890_e17559_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard221 == 0.0)) && (locals.var_guard222 == 0.0)) {
        let assign17890_e17554: f64 = (locals.var_fn205_calc_iq__eta0).exp();
        let assign17890_e17555: f64 = (1.0 + assign17890_e17554);
        let assign17890_e17556: f64 = (assign17890_e17555).ln();
        let assign17890_e17557: f64 = (locals.var_fn205_calc_iq__qref0 * assign17890_e17556);
        (assign17890_e17557, (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn2) / assign17890_e17555)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign17890_e17556) + (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn4) / assign17890_e17555))), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn7) / assign17890_e17555)), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn10) / assign17890_e17555)), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn11) / assign17890_e17555)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17890_e17559;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17890_e17559_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17890_e17559_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17890_e17559_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17890_e17559_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17890_e17559_d_n11;

        let (assign17900_e17565, assign17900_e17565_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17900_e17563: f64 = (locals.var_fn205_calc_iq__mu0 / locals.var_fn205_calc_iq__tfacmobin);
        (assign17900_e17563, (-((locals.var_fn205_calc_iq__mu0 * locals.var_fn205_calc_iq__tfacmobin_dn4) / (locals.var_fn205_calc_iq__tfacmobin * locals.var_fn205_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn205_calc_iq__muf0, locals.var_fn205_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn205_calc_iq__muf0 = assign17900_e17565;
        locals.var_fn205_calc_iq__muf0_dn4 = assign17900_e17565_d_n4;

        let (assign17910_e17581, assign17910_e17581_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17910_e17571: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tnomin);
        let assign17910_e17572: f64 = (1.0 + assign17910_e17571);
        let assign17910_e17576: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin);
        let assign17910_e17577: f64 = (1.0 + assign17910_e17576);
        let assign17910_e17578: f64 = (assign17910_e17572 / assign17910_e17577);
        let assign17910_e17579: f64 = (locals.var_fn205_calc_iq__vel0 * assign17910_e17578);
        (assign17910_e17579, (locals.var_fn205_calc_iq__vel0 * (-((assign17910_e17572 * (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin_dn4)) / (assign17910_e17577 * assign17910_e17577)))),)
    } else {
        (locals.var_fn205_calc_iq__vx0, locals.var_fn205_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vx0 = assign17910_e17581;
        locals.var_fn205_calc_iq__vx0_dn4 = assign17910_e17581_d_n4;

        let (assign17920_e17589, assign17920_e17589_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17920_e17585: f64 = (locals.var_fn205_calc_iq__vx0 * locals.var_fn205_calc_iq__lin);
        let assign17920_e17587: f64 = (assign17920_e17585 / locals.var_fn205_calc_iq__muf0);
        (assign17920_e17587, ((((locals.var_fn205_calc_iq__vx0_dn4 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf0) - (assign17920_e17585 * locals.var_fn205_calc_iq__muf0_dn4)) / (locals.var_fn205_calc_iq__muf0 * locals.var_fn205_calc_iq__muf0)),)
    } else {
        (locals.var_fn205_calc_iq__vdsats0, locals.var_fn205_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vdsats0 = assign17920_e17589;
        locals.var_fn205_calc_iq__vdsats0_dn4 = assign17920_e17589_d_n4;

        let (assign17930_e17606, assign17930_e17606_d_n2, assign17930_e17606_d_n4, assign17930_e17606_d_n7, assign17930_e17606_d_n10, assign17930_e17606_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17930_e17595: f64 = (2.0 * locals.var_fn205_calc_iq__qinvv0);
        let assign17930_e17597: f64 = (assign17930_e17595 / locals.var_fn205_calc_iq__cgin);
        let assign17930_e17599: f64 = (assign17930_e17597 / locals.var_fn205_calc_iq__vdsats0);
        let assign17930_e17600: f64 = (1.0 + assign17930_e17599);
        let assign17930_e17601: f64 = (assign17930_e17600).sqrt();
        let assign17930_e17602: f64 = (locals.var_fn205_calc_iq__vdsats0 * assign17930_e17601);
        let assign17930_e17604: f64 = (assign17930_e17602 - locals.var_fn205_calc_iq__vdsats0);
        (assign17930_e17604, (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn2) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (((locals.var_fn205_calc_iq__vdsats0_dn4 * assign17930_e17601) + (locals.var_fn205_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17930_e17595 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)) * locals.var_fn205_calc_iq__vdsats0) - (assign17930_e17597 * locals.var_fn205_calc_iq__vdsats0_dn4)) / (locals.var_fn205_calc_iq__vdsats0 * locals.var_fn205_calc_iq__vdsats0)) / (2.0 * assign17930_e17601)))) - locals.var_fn205_calc_iq__vdsats0_dn4), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn7) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn10) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn11) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))),)
    } else {
        (locals.var_fn205_calc_iq__vdsats10, locals.var_fn205_calc_iq__vdsats10_dn2, locals.var_fn205_calc_iq__vdsats10_dn4, locals.var_fn205_calc_iq__vdsats10_dn7, locals.var_fn205_calc_iq__vdsats10_dn10, locals.var_fn205_calc_iq__vdsats10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats10 = assign17930_e17606;
        locals.var_fn205_calc_iq__vdsats10_dn2 = assign17930_e17606_d_n2;
        locals.var_fn205_calc_iq__vdsats10_dn4 = assign17930_e17606_d_n4;
        locals.var_fn205_calc_iq__vdsats10_dn7 = assign17930_e17606_d_n7;
        locals.var_fn205_calc_iq__vdsats10_dn10 = assign17930_e17606_d_n10;
        locals.var_fn205_calc_iq__vdsats10_dn11 = assign17930_e17606_d_n11;

        let (assign17940_e17618, assign17940_e17618_d_n2, assign17940_e17618_d_n4, assign17940_e17618_d_n7, assign17940_e17618_d_n10, assign17940_e17618_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17940_e17611: f64 = (1.0 - locals.var_fn205_calc_iq__ff0);
        let assign17940_e17612: f64 = (locals.var_fn205_calc_iq__vdsats10 * assign17940_e17611);
        let assign17940_e17615: f64 = (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0);
        let assign17940_e17616: f64 = (assign17940_e17612 + assign17940_e17615);
        (assign17940_e17616, (((locals.var_fn205_calc_iq__vdsats10_dn2 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn2))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn2)), (((locals.var_fn205_calc_iq__vdsats10_dn4 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit0_dn4 * locals.var_fn205_calc_iq__ff0) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn4))), (((locals.var_fn205_calc_iq__vdsats10_dn7 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn7))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn7)), (((locals.var_fn205_calc_iq__vdsats10_dn10 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn10))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn10)), (((locals.var_fn205_calc_iq__vdsats10_dn11 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn11))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdsat10, locals.var_fn205_calc_iq__vdsat10_dn2, locals.var_fn205_calc_iq__vdsat10_dn4, locals.var_fn205_calc_iq__vdsat10_dn7, locals.var_fn205_calc_iq__vdsat10_dn10, locals.var_fn205_calc_iq__vdsat10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat10 = assign17940_e17618;
        locals.var_fn205_calc_iq__vdsat10_dn2 = assign17940_e17618_d_n2;
        locals.var_fn205_calc_iq__vdsat10_dn4 = assign17940_e17618_d_n4;
        locals.var_fn205_calc_iq__vdsat10_dn7 = assign17940_e17618_d_n7;
        locals.var_fn205_calc_iq__vdsat10_dn10 = assign17940_e17618_d_n10;
        locals.var_fn205_calc_iq__vdsat10_dn11 = assign17940_e17618_d_n11;

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17950_e17687, assign17950_e17687_d_n2, assign17950_e17687_d_n4, assign17950_e17687_d_n7, assign17950_e17687_d_n10, assign17950_e17687_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17950_e17677, assign17950_e17677_d_n2, assign17950_e17677_d_n4, assign17950_e17677_d_n7, assign17950_e17677_d_n10, assign17950_e17677_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17950_e17630: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17631: f64 = assign17950_e17630;
                let assign17950_e17635: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17636: f64 = (-assign17950_e17635);
                let assign17950_e17639: f64 = (0.001 / p.p53);
                let assign17950_e17643: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17644: f64 = (-assign17950_e17643);
                let assign17950_e17645: f64 = (assign17950_e17639 * assign17950_e17644);
                let assign17950_e17646: f64 = (assign17950_e17645).tanh();
                let assign17950_e17647: f64 = (assign17950_e17636 * assign17950_e17646);
                let assign17950_e17648: f64 = (assign17950_e17631 + assign17950_e17647);
                let assign17950_e17649: f64 = (0.5 * assign17950_e17648);
                (assign17950_e17649, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))),)
            } else {
                let (assign17950_e17676, assign17950_e17676_d_n2, assign17950_e17676_d_n4, assign17950_e17676_d_n7, assign17950_e17676_d_n10, assign17950_e17676_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17950_e17657: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17658: f64 = assign17950_e17657;
                        let assign17950_e17662: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17663: f64 = (-assign17950_e17662);
                        let assign17950_e17667: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17668: f64 = (-assign17950_e17667);
                        let assign17950_e17669: f64 = (assign17950_e17663 * assign17950_e17668);
                        let assign17950_e17671: f64 = (assign17950_e17669 + p.p53);
                        let assign17950_e17672: f64 = (assign17950_e17671).sqrt();
                        let assign17950_e17673: f64 = (assign17950_e17658 + assign17950_e17672);
                        let assign17950_e17674: f64 = (0.5 * assign17950_e17673);
                        (assign17950_e17674, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17668) + (assign17950_e17663 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17950_e17672)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17668) + (assign17950_e17663 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17950_e17672)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17950_e17676, assign17950_e17676_d_n2, assign17950_e17676_d_n4, assign17950_e17676_d_n7, assign17950_e17676_d_n10, assign17950_e17676_d_n11,)
            }
        };
        let assign17950_e17679: f64 = (assign17950_e17677).powf(locals.var_fn205_calc_iq__beta);
        let assign17950_e17680: f64 = (1.0 + assign17950_e17679);
        let assign17950_e17683: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17950_e17684: f64 = (assign17950_e17680).powf(assign17950_e17683);
        let assign17950_e17685: f64 = (1.0 / assign17950_e17684);
        (assign17950_e17685, (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n2)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n2 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n2)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n2 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n4)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n4 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n4)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n4 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n7)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n7 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n7)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n7 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n10)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n10 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n10)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n10 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n11)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n11 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n11)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n11 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))),)
    } else {
        (locals.var_fn205_calc_iq__fsd0, locals.var_fn205_calc_iq__fsd0_dn2, locals.var_fn205_calc_iq__fsd0_dn4, locals.var_fn205_calc_iq__fsd0_dn7, locals.var_fn205_calc_iq__fsd0_dn10, locals.var_fn205_calc_iq__fsd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd0 = assign17950_e17687;
        locals.var_fn205_calc_iq__fsd0_dn2 = assign17950_e17687_d_n2;
        locals.var_fn205_calc_iq__fsd0_dn4 = assign17950_e17687_d_n4;
        locals.var_fn205_calc_iq__fsd0_dn7 = assign17950_e17687_d_n7;
        locals.var_fn205_calc_iq__fsd0_dn10 = assign17950_e17687_d_n10;
        locals.var_fn205_calc_iq__fsd0_dn11 = assign17950_e17687_d_n11;

        let (assign17960_e17693, assign17960_e17693_d_n2, assign17960_e17693_d_n4, assign17960_e17693_d_n7, assign17960_e17693_d_n10, assign17960_e17693_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17960_e17691: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0);
        (assign17960_e17691, (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn2), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn4), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn7), ((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__fsd0) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn10)), ((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__fsd0) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdx0, locals.var_fn205_calc_iq__vdx0_dn2, locals.var_fn205_calc_iq__vdx0_dn4, locals.var_fn205_calc_iq__vdx0_dn7, locals.var_fn205_calc_iq__vdx0_dn10, locals.var_fn205_calc_iq__vdx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx0 = assign17960_e17693;
        locals.var_fn205_calc_iq__vdx0_dn2 = assign17960_e17693_d_n2;
        locals.var_fn205_calc_iq__vdx0_dn4 = assign17960_e17693_d_n4;
        locals.var_fn205_calc_iq__vdx0_dn7 = assign17960_e17693_d_n7;
        locals.var_fn205_calc_iq__vdx0_dn10 = assign17960_e17693_d_n10;
        locals.var_fn205_calc_iq__vdx0_dn11 = assign17960_e17693_d_n11;

        let (assign17970_e17768, assign17970_e17768_d_n2, assign17970_e17768_d_n4, assign17970_e17768_d_n7, assign17970_e17768_d_n10, assign17970_e17768_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17970_e17758, assign17970_e17758_d_n2, assign17970_e17758_d_n4, assign17970_e17758_d_n7, assign17970_e17758_d_n10, assign17970_e17758_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17970_e17704: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17706: f64 = (assign17970_e17704 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17707: f64 = assign17970_e17706;
                let assign17970_e17710: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17712: f64 = (assign17970_e17710 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17713: f64 = (-assign17970_e17712);
                let assign17970_e17716: f64 = (0.001 / p.p53);
                let assign17970_e17719: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17721: f64 = (assign17970_e17719 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17722: f64 = (-assign17970_e17721);
                let assign17970_e17723: f64 = (assign17970_e17716 * assign17970_e17722);
                let assign17970_e17724: f64 = (assign17970_e17723).tanh();
                let assign17970_e17725: f64 = (assign17970_e17713 * assign17970_e17724);
                let assign17970_e17726: f64 = (assign17970_e17707 + assign17970_e17725);
                let assign17970_e17727: f64 = (0.5 * assign17970_e17726);
                (assign17970_e17727, (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))),)
            } else {
                let (assign17970_e17757, assign17970_e17757_d_n2, assign17970_e17757_d_n4, assign17970_e17757_d_n7, assign17970_e17757_d_n10, assign17970_e17757_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17970_e17734: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17736: f64 = (assign17970_e17734 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17737: f64 = assign17970_e17736;
                        let assign17970_e17740: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17742: f64 = (assign17970_e17740 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17743: f64 = (-assign17970_e17742);
                        let assign17970_e17746: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17748: f64 = (assign17970_e17746 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17749: f64 = (-assign17970_e17748);
                        let assign17970_e17750: f64 = (assign17970_e17743 * assign17970_e17749);
                        let assign17970_e17752: f64 = (assign17970_e17750 + p.p53);
                        let assign17970_e17753: f64 = (assign17970_e17752).sqrt();
                        let assign17970_e17754: f64 = (assign17970_e17737 + assign17970_e17753);
                        let assign17970_e17755: f64 = (0.5 * assign17970_e17754);
                        (assign17970_e17755, (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17749) + (assign17970_e17743 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17970_e17753)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17749) + (assign17970_e17743 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17970_e17753)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17970_e17757, assign17970_e17757_d_n2, assign17970_e17757_d_n4, assign17970_e17757_d_n7, assign17970_e17757_d_n10, assign17970_e17757_d_n11,)
            }
        };
        let assign17970_e17760: f64 = (assign17970_e17758).powf(locals.var_fn205_calc_iq__beta);
        let assign17970_e17761: f64 = (1.0 + assign17970_e17760);
        let assign17970_e17764: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17970_e17765: f64 = (assign17970_e17761).powf(assign17970_e17764);
        let assign17970_e17766: f64 = (1.0 / assign17970_e17765);
        (assign17970_e17766, (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n2)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n2 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n2)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n2 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n4)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n4 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n4)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n4 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n7)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n7 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n7)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n7 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n10)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n10 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n10)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n10 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n11)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n11 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n11)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n11 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))),)
    } else {
        (locals.var_fn205_calc_iq__fds0, locals.var_fn205_calc_iq__fds0_dn2, locals.var_fn205_calc_iq__fds0_dn4, locals.var_fn205_calc_iq__fds0_dn7, locals.var_fn205_calc_iq__fds0_dn10, locals.var_fn205_calc_iq__fds0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds0 = assign17970_e17768;
        locals.var_fn205_calc_iq__fds0_dn2 = assign17970_e17768_d_n2;
        locals.var_fn205_calc_iq__fds0_dn4 = assign17970_e17768_d_n4;
        locals.var_fn205_calc_iq__fds0_dn7 = assign17970_e17768_d_n7;
        locals.var_fn205_calc_iq__fds0_dn10 = assign17970_e17768_d_n10;
        locals.var_fn205_calc_iq__fds0_dn11 = assign17970_e17768_d_n11;

        let (assign17980_e17775, assign17980_e17775_d_n2, assign17980_e17775_d_n4, assign17980_e17775_d_n7, assign17980_e17775_d_n10, assign17980_e17775_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17980_e17771: f64 = (-locals.var_fn205_calc_iq__vdsin);
        let assign17980_e17773: f64 = (assign17980_e17771 * locals.var_fn205_calc_iq__fds0);
        (assign17980_e17773, (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn2), (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn4), (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn7), (((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__fds0) + (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn10)), (((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__fds0) + (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vsx0, locals.var_fn205_calc_iq__vsx0_dn2, locals.var_fn205_calc_iq__vsx0_dn4, locals.var_fn205_calc_iq__vsx0_dn7, locals.var_fn205_calc_iq__vsx0_dn10, locals.var_fn205_calc_iq__vsx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx0 = assign17980_e17775;
        locals.var_fn205_calc_iq__vsx0_dn2 = assign17980_e17775_d_n2;
        locals.var_fn205_calc_iq__vsx0_dn4 = assign17980_e17775_d_n4;
        locals.var_fn205_calc_iq__vsx0_dn7 = assign17980_e17775_d_n7;
        locals.var_fn205_calc_iq__vsx0_dn10 = assign17980_e17775_d_n10;
        locals.var_fn205_calc_iq__vsx0_dn11 = assign17980_e17775_d_n11;

        let (assign17990_e17783, assign17990_e17783_d_n2, assign17990_e17783_d_n4, assign17990_e17783_d_n7, assign17990_e17783_d_n10, assign17990_e17783_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17990_e17779: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__myarg0);
        let assign17990_e17781: f64 = (assign17990_e17779 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17990_e17781, (locals.var_fn205_calc_iq__vgsin_dn2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17990_e17779 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (locals.var_fn205_calc_iq__vgsin_dn7 / locals.var_fn205_calc_iq__alpha_phit), 0.0, (locals.var_fn205_calc_iq__vgsin_dn11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17990_e17783;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17990_e17783_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17990_e17783_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17990_e17783_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17990_e17783_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17990_e17783_d_n11;

        let assign18000_e17786: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign18000_e17786;

        let (assign18010_e17792, assign18010_e17792_d_n2, assign18010_e17792_d_n4, assign18010_e17792_d_n7, assign18010_e17792_d_n10, assign18010_e17792_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18010_e17792;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18010_e17792_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18010_e17792_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18010_e17792_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18010_e17792_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18010_e17792_d_n11;

        let assign18020_e17795: f64 = (-50.0);
        let assign18020_e17796: f64 = if locals.var_fn205_calc_iq__exparg0 < assign18020_e17795 { 1.0 } else { 0.0 };
        locals.var_guard224 = assign18020_e17796;

        let (assign18030_e17805, assign18030_e17805_d_n2, assign18030_e17805_d_n4, assign18030_e17805_d_n7, assign18030_e17805_d_n10, assign18030_e17805_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18030_e17805;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18030_e17805_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18030_e17805_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18030_e17805_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18030_e17805_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18030_e17805_d_n11;

        let (assign18040_e17820, assign18040_e17820_d_n2, assign18040_e17820_d_n4, assign18040_e17820_d_n7, assign18040_e17820_d_n10, assign18040_e17820_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign18040_e17816: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign18040_e17817: f64 = (1.0 + assign18040_e17816);
        let assign18040_e17818: f64 = (1.0 / assign18040_e17817);
        (assign18040_e17818, (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign18040_e17817 * assign18040_e17817))),)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18040_e17820;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18040_e17820_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18040_e17820_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18040_e17820_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18040_e17820_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18040_e17820_d_n11;

        let (assign18050_e17838, assign18050_e17838_d_n2, assign18050_e17838_d_n4, assign18050_e17838_d_n7, assign18050_e17838_d_n10, assign18050_e17838_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18050_e17824: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__vsx0);
        let assign18050_e17828: f64 = (p.p51 * 0.1);
        let assign18050_e17830: f64 = (assign18050_e17828 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18050_e17832: f64 = (assign18050_e17830 * locals.var_fn205_calc_iq__ffs0);
        let assign18050_e17833: f64 = (locals.var_fn205_calc_iq__vtof - assign18050_e17832);
        let assign18050_e17834: f64 = (assign18050_e17824 - assign18050_e17833);
        let assign18050_e17836: f64 = (assign18050_e17834 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18050_e17836, (((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__vsx0_dn2) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), (((((-locals.var_fn205_calc_iq__vsx0_dn4) - (locals.var_fn205_calc_iq__vtof_dn4 - (((assign18050_e17828 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffs0) + (assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18050_e17834 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__vsx0_dn7) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__vsx0_dn10) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__vsx0_dn11) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etas0, locals.var_fn205_calc_iq__etas0_dn2, locals.var_fn205_calc_iq__etas0_dn4, locals.var_fn205_calc_iq__etas0_dn7, locals.var_fn205_calc_iq__etas0_dn10, locals.var_fn205_calc_iq__etas0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas0 = assign18050_e17838;
        locals.var_fn205_calc_iq__etas0_dn2 = assign18050_e17838_d_n2;
        locals.var_fn205_calc_iq__etas0_dn4 = assign18050_e17838_d_n4;
        locals.var_fn205_calc_iq__etas0_dn7 = assign18050_e17838_d_n7;
        locals.var_fn205_calc_iq__etas0_dn10 = assign18050_e17838_d_n10;
        locals.var_fn205_calc_iq__etas0_dn11 = assign18050_e17838_d_n11;

        let assign18060_e17841: f64 = if locals.var_fn205_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign18060_e17841;

        let (assign18070_e17849, assign18070_e17849_d_n2, assign18070_e17849_d_n4, assign18070_e17849_d_n7, assign18070_e17849_d_n10, assign18070_e17849_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard225 != 0.0)) {
        let assign18070_e17847: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0);
        (assign18070_e17847, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__etas0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18070_e17849;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18070_e17849_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18070_e17849_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18070_e17849_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18070_e17849_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18070_e17849_d_n11;

        let assign18080_e17852: f64 = (-50.0);
        let assign18080_e17853: f64 = if locals.var_fn205_calc_iq__etas0 < assign18080_e17852 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign18080_e17853;

        let (assign18090_e17865, assign18090_e17865_d_n2, assign18090_e17865_d_n4, assign18090_e17865_d_n7, assign18090_e17865_d_n10, assign18090_e17865_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard225 == 0.0)) && (locals.var_guard226 != 0.0)) {
        let assign18090_e17862: f64 = (locals.var_fn205_calc_iq__etas0).exp();
        let assign18090_e17863: f64 = (locals.var_fn205_calc_iq__qref0 * assign18090_e17862);
        (assign18090_e17863, (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18090_e17862) + (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18090_e17865;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18090_e17865_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18090_e17865_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18090_e17865_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18090_e17865_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18090_e17865_d_n11;

        let (assign18100_e17881, assign18100_e17881_d_n2, assign18100_e17881_d_n4, assign18100_e17881_d_n7, assign18100_e17881_d_n10, assign18100_e17881_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard225 == 0.0)) && (locals.var_guard226 == 0.0)) {
        let assign18100_e17876: f64 = (locals.var_fn205_calc_iq__etas0).exp();
        let assign18100_e17877: f64 = (1.0 + assign18100_e17876);
        let assign18100_e17878: f64 = (assign18100_e17877).ln();
        let assign18100_e17879: f64 = (locals.var_fn205_calc_iq__qref0 * assign18100_e17878);
        (assign18100_e17879, (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn2) / assign18100_e17877)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18100_e17878) + (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn4) / assign18100_e17877))), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn7) / assign18100_e17877)), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn10) / assign18100_e17877)), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn11) / assign18100_e17877)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18100_e17881;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18100_e17881_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18100_e17881_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18100_e17881_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18100_e17881_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18100_e17881_d_n11;

        let (assign18110_e17889, assign18110_e17889_d_n2, assign18110_e17889_d_n4, assign18110_e17889_d_n7, assign18110_e17889_d_n10, assign18110_e17889_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18110_e17885: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__myarg0);
        let assign18110_e17887: f64 = (assign18110_e17885 / locals.var_fn205_calc_iq__alpha_phit);
        (assign18110_e17887, (locals.var_fn205_calc_iq__vgdin_dn2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign18110_e17885 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (locals.var_fn205_calc_iq__vgdin_dn7 / locals.var_fn205_calc_iq__alpha_phit), (locals.var_fn205_calc_iq__vgdin_dn10 / locals.var_fn205_calc_iq__alpha_phit), (locals.var_fn205_calc_iq__vgdin_dn11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign18110_e17889;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign18110_e17889_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign18110_e17889_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign18110_e17889_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign18110_e17889_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign18110_e17889_d_n11;

        let assign18120_e17892: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign18120_e17892;

        let (assign18130_e17898, assign18130_e17898_d_n2, assign18130_e17898_d_n4, assign18130_e17898_d_n7, assign18130_e17898_d_n10, assign18130_e17898_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18130_e17898;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18130_e17898_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18130_e17898_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18130_e17898_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18130_e17898_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18130_e17898_d_n11;

        let assign18140_e17901: f64 = (-50.0);
        let assign18140_e17902: f64 = if locals.var_fn205_calc_iq__exparg0 < assign18140_e17901 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign18140_e17902;

        let (assign18150_e17911, assign18150_e17911_d_n2, assign18150_e17911_d_n4, assign18150_e17911_d_n7, assign18150_e17911_d_n10, assign18150_e17911_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard227 == 0.0)) && (locals.var_guard228 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18150_e17911;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18150_e17911_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18150_e17911_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18150_e17911_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18150_e17911_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18150_e17911_d_n11;

        let (assign18160_e17926, assign18160_e17926_d_n2, assign18160_e17926_d_n4, assign18160_e17926_d_n7, assign18160_e17926_d_n10, assign18160_e17926_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard227 == 0.0)) && (locals.var_guard228 == 0.0)) {
        let assign18160_e17922: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign18160_e17923: f64 = (1.0 + assign18160_e17922);
        let assign18160_e17924: f64 = (1.0 / assign18160_e17923);
        (assign18160_e17924, (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign18160_e17923 * assign18160_e17923))),)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18160_e17926;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18160_e17926_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18160_e17926_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18160_e17926_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18160_e17926_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18160_e17926_d_n11;

        let (assign18170_e17944, assign18170_e17944_d_n2, assign18170_e17944_d_n4, assign18170_e17944_d_n7, assign18170_e17944_d_n10, assign18170_e17944_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18170_e17930: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdx0);
        let assign18170_e17934: f64 = (p.p51 * 0.1);
        let assign18170_e17936: f64 = (assign18170_e17934 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18170_e17938: f64 = (assign18170_e17936 * locals.var_fn205_calc_iq__ffd0);
        let assign18170_e17939: f64 = (locals.var_fn205_calc_iq__vtof - assign18170_e17938);
        let assign18170_e17940: f64 = (assign18170_e17930 - assign18170_e17939);
        let assign18170_e17942: f64 = (assign18170_e17940 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18170_e17942, (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vdx0_dn2) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), (((((-locals.var_fn205_calc_iq__vdx0_dn4) - (locals.var_fn205_calc_iq__vtof_dn4 - (((assign18170_e17934 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffd0) + (assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18170_e17940 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vdx0_dn7) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), (((-locals.var_fn205_calc_iq__vdx0_dn10) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdx0_dn11) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etad0, locals.var_fn205_calc_iq__etad0_dn2, locals.var_fn205_calc_iq__etad0_dn4, locals.var_fn205_calc_iq__etad0_dn7, locals.var_fn205_calc_iq__etad0_dn10, locals.var_fn205_calc_iq__etad0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad0 = assign18170_e17944;
        locals.var_fn205_calc_iq__etad0_dn2 = assign18170_e17944_d_n2;
        locals.var_fn205_calc_iq__etad0_dn4 = assign18170_e17944_d_n4;
        locals.var_fn205_calc_iq__etad0_dn7 = assign18170_e17944_d_n7;
        locals.var_fn205_calc_iq__etad0_dn10 = assign18170_e17944_d_n10;
        locals.var_fn205_calc_iq__etad0_dn11 = assign18170_e17944_d_n11;

        let assign18180_e17947: f64 = if locals.var_fn205_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard229 = assign18180_e17947;

        let (assign18190_e17955, assign18190_e17955_d_n2, assign18190_e17955_d_n4, assign18190_e17955_d_n7, assign18190_e17955_d_n10, assign18190_e17955_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard229 != 0.0)) {
        let assign18190_e17953: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0);
        (assign18190_e17953, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__etad0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18190_e17955;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18190_e17955_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18190_e17955_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18190_e17955_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18190_e17955_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18190_e17955_d_n11;

        let assign18200_e17958: f64 = (-50.0);
        let assign18200_e17959: f64 = if locals.var_fn205_calc_iq__etad0 < assign18200_e17958 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign18200_e17959;

        let (assign18210_e17971, assign18210_e17971_d_n2, assign18210_e17971_d_n4, assign18210_e17971_d_n7, assign18210_e17971_d_n10, assign18210_e17971_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard229 == 0.0)) && (locals.var_guard230 != 0.0)) {
        let assign18210_e17968: f64 = (locals.var_fn205_calc_iq__etad0).exp();
        let assign18210_e17969: f64 = (locals.var_fn205_calc_iq__qref0 * assign18210_e17968);
        (assign18210_e17969, (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18210_e17968) + (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18210_e17971;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18210_e17971_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18210_e17971_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18210_e17971_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18210_e17971_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18210_e17971_d_n11;

        let (assign18220_e17987, assign18220_e17987_d_n2, assign18220_e17987_d_n4, assign18220_e17987_d_n7, assign18220_e17987_d_n10, assign18220_e17987_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard229 == 0.0)) && (locals.var_guard230 == 0.0)) {
        let assign18220_e17982: f64 = (locals.var_fn205_calc_iq__etad0).exp();
        let assign18220_e17983: f64 = (1.0 + assign18220_e17982);
        let assign18220_e17984: f64 = (assign18220_e17983).ln();
        let assign18220_e17985: f64 = (locals.var_fn205_calc_iq__qref0 * assign18220_e17984);
        (assign18220_e17985, (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn2) / assign18220_e17983)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18220_e17984) + (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn4) / assign18220_e17983))), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn7) / assign18220_e17983)), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn10) / assign18220_e17983)), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn11) / assign18220_e17983)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18220_e17987;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18220_e17987_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18220_e17987_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18220_e17987_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18220_e17987_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18220_e17987_d_n11;

        let (assign18230_e17995, assign18230_e17995_d_n2, assign18230_e17995_d_n4, assign18230_e17995_d_n7, assign18230_e17995_d_n10, assign18230_e17995_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18230_e17991: f64 = (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0);
        let assign18230_e17993: f64 = (assign18230_e17991 + 1e-38);
        (assign18230_e17993, ((locals.var_fn205_calc_iq__qinvs0_dn2 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn2)), ((locals.var_fn205_calc_iq__qinvs0_dn4 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn4)), ((locals.var_fn205_calc_iq__qinvs0_dn7 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn7)), ((locals.var_fn205_calc_iq__qinvs0_dn10 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn10)), ((locals.var_fn205_calc_iq__qinvs0_dn11 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qs2, locals.var_fn205_calc_iq__qs2_dn2, locals.var_fn205_calc_iq__qs2_dn4, locals.var_fn205_calc_iq__qs2_dn7, locals.var_fn205_calc_iq__qs2_dn10, locals.var_fn205_calc_iq__qs2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs2 = assign18230_e17995;
        locals.var_fn205_calc_iq__qs2_dn2 = assign18230_e17995_d_n2;
        locals.var_fn205_calc_iq__qs2_dn4 = assign18230_e17995_d_n4;
        locals.var_fn205_calc_iq__qs2_dn7 = assign18230_e17995_d_n7;
        locals.var_fn205_calc_iq__qs2_dn10 = assign18230_e17995_d_n10;
        locals.var_fn205_calc_iq__qs2_dn11 = assign18230_e17995_d_n11;

        let (assign18240_e18003, assign18240_e18003_d_n2, assign18240_e18003_d_n4, assign18240_e18003_d_n7, assign18240_e18003_d_n10, assign18240_e18003_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18240_e17999: f64 = (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0);
        let assign18240_e18001: f64 = (assign18240_e17999 + 1e-57);
        (assign18240_e18001, ((locals.var_fn205_calc_iq__qs2_dn2 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn2)), ((locals.var_fn205_calc_iq__qs2_dn4 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn4)), ((locals.var_fn205_calc_iq__qs2_dn7 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn7)), ((locals.var_fn205_calc_iq__qs2_dn10 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn10)), ((locals.var_fn205_calc_iq__qs2_dn11 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qs3, locals.var_fn205_calc_iq__qs3_dn2, locals.var_fn205_calc_iq__qs3_dn4, locals.var_fn205_calc_iq__qs3_dn7, locals.var_fn205_calc_iq__qs3_dn10, locals.var_fn205_calc_iq__qs3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs3 = assign18240_e18003;
        locals.var_fn205_calc_iq__qs3_dn2 = assign18240_e18003_d_n2;
        locals.var_fn205_calc_iq__qs3_dn4 = assign18240_e18003_d_n4;
        locals.var_fn205_calc_iq__qs3_dn7 = assign18240_e18003_d_n7;
        locals.var_fn205_calc_iq__qs3_dn10 = assign18240_e18003_d_n10;
        locals.var_fn205_calc_iq__qs3_dn11 = assign18240_e18003_d_n11;

        let (assign18250_e18011, assign18250_e18011_d_n2, assign18250_e18011_d_n4, assign18250_e18011_d_n7, assign18250_e18011_d_n10, assign18250_e18011_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18250_e18007: f64 = (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0);
        let assign18250_e18009: f64 = (assign18250_e18007 + 1e-38);
        (assign18250_e18009, ((locals.var_fn205_calc_iq__qinvd0_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qinvd0_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qinvd0_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qinvd0_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qinvd0_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qd2, locals.var_fn205_calc_iq__qd2_dn2, locals.var_fn205_calc_iq__qd2_dn4, locals.var_fn205_calc_iq__qd2_dn7, locals.var_fn205_calc_iq__qd2_dn10, locals.var_fn205_calc_iq__qd2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd2 = assign18250_e18011;
        locals.var_fn205_calc_iq__qd2_dn2 = assign18250_e18011_d_n2;
        locals.var_fn205_calc_iq__qd2_dn4 = assign18250_e18011_d_n4;
        locals.var_fn205_calc_iq__qd2_dn7 = assign18250_e18011_d_n7;
        locals.var_fn205_calc_iq__qd2_dn10 = assign18250_e18011_d_n10;
        locals.var_fn205_calc_iq__qd2_dn11 = assign18250_e18011_d_n11;

        let (assign18260_e18019, assign18260_e18019_d_n2, assign18260_e18019_d_n4, assign18260_e18019_d_n7, assign18260_e18019_d_n10, assign18260_e18019_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18260_e18015: f64 = (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0);
        let assign18260_e18017: f64 = (assign18260_e18015 + 1e-57);
        (assign18260_e18017, ((locals.var_fn205_calc_iq__qd2_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qd2_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qd2_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qd2_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qd2_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qd3, locals.var_fn205_calc_iq__qd3_dn2, locals.var_fn205_calc_iq__qd3_dn4, locals.var_fn205_calc_iq__qd3_dn7, locals.var_fn205_calc_iq__qd3_dn10, locals.var_fn205_calc_iq__qd3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd3 = assign18260_e18019;
        locals.var_fn205_calc_iq__qd3_dn2 = assign18260_e18019_d_n2;
        locals.var_fn205_calc_iq__qd3_dn4 = assign18260_e18019_d_n4;
        locals.var_fn205_calc_iq__qd3_dn7 = assign18260_e18019_d_n7;
        locals.var_fn205_calc_iq__qd3_dn10 = assign18260_e18019_d_n10;
        locals.var_fn205_calc_iq__qd3_dn11 = assign18260_e18019_d_n11;

    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18270_e18027, assign18270_e18027_d_n2, assign18270_e18027_d_n4, assign18270_e18027_d_n7, assign18270_e18027_d_n10, assign18270_e18027_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18270_e18023: f64 = (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0);
        let assign18270_e18025: f64 = (assign18270_e18023 + 1e-38);
        (assign18270_e18025, ((locals.var_fn205_calc_iq__qinvs0_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qinvs0_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qinvs0_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qinvs0_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qinvs0_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qsqd, locals.var_fn205_calc_iq__qsqd_dn2, locals.var_fn205_calc_iq__qsqd_dn4, locals.var_fn205_calc_iq__qsqd_dn7, locals.var_fn205_calc_iq__qsqd_dn10, locals.var_fn205_calc_iq__qsqd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsqd = assign18270_e18027;
        locals.var_fn205_calc_iq__qsqd_dn2 = assign18270_e18027_d_n2;
        locals.var_fn205_calc_iq__qsqd_dn4 = assign18270_e18027_d_n4;
        locals.var_fn205_calc_iq__qsqd_dn7 = assign18270_e18027_d_n7;
        locals.var_fn205_calc_iq__qsqd_dn10 = assign18270_e18027_d_n10;
        locals.var_fn205_calc_iq__qsqd_dn11 = assign18270_e18027_d_n11;

        let (assign18280_e18045, assign18280_e18045_d_n2, assign18280_e18045_d_n4, assign18280_e18045_d_n7, assign18280_e18045_d_n10, assign18280_e18045_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18280_e18031: f64 = (2.0 / 3.0);
        let assign18280_e18034: f64 = (locals.var_fn205_calc_iq__qs2 + locals.var_fn205_calc_iq__qd2);
        let assign18280_e18036: f64 = (assign18280_e18034 + locals.var_fn205_calc_iq__qsqd);
        let assign18280_e18037: f64 = (assign18280_e18031 * assign18280_e18036);
        let assign18280_e18040: f64 = (locals.var_fn205_calc_iq__qinvs0 + locals.var_fn205_calc_iq__qinvd0);
        let assign18280_e18042: f64 = (assign18280_e18040 + 2e-19);
        let assign18280_e18043: f64 = (assign18280_e18037 / assign18280_e18042);
        (assign18280_e18043, ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn2 + locals.var_fn205_calc_iq__qd2_dn2) + locals.var_fn205_calc_iq__qsqd_dn2)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn2 + locals.var_fn205_calc_iq__qinvd0_dn2))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn4 + locals.var_fn205_calc_iq__qd2_dn4) + locals.var_fn205_calc_iq__qsqd_dn4)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn4 + locals.var_fn205_calc_iq__qinvd0_dn4))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn7 + locals.var_fn205_calc_iq__qd2_dn7) + locals.var_fn205_calc_iq__qsqd_dn7)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn7 + locals.var_fn205_calc_iq__qinvd0_dn7))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn10 + locals.var_fn205_calc_iq__qd2_dn10) + locals.var_fn205_calc_iq__qsqd_dn10)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn10 + locals.var_fn205_calc_iq__qinvd0_dn10))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn11 + locals.var_fn205_calc_iq__qd2_dn11) + locals.var_fn205_calc_iq__qsqd_dn11)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn11 + locals.var_fn205_calc_iq__qinvd0_dn11))) / (assign18280_e18042 * assign18280_e18042)),)
    } else {
        (locals.var_fn205_calc_iq__qinvdd, locals.var_fn205_calc_iq__qinvdd_dn2, locals.var_fn205_calc_iq__qinvdd_dn4, locals.var_fn205_calc_iq__qinvdd_dn7, locals.var_fn205_calc_iq__qinvdd_dn10, locals.var_fn205_calc_iq__qinvdd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvdd = assign18280_e18045;
        locals.var_fn205_calc_iq__qinvdd_dn2 = assign18280_e18045_d_n2;
        locals.var_fn205_calc_iq__qinvdd_dn4 = assign18280_e18045_d_n4;
        locals.var_fn205_calc_iq__qinvdd_dn7 = assign18280_e18045_d_n7;
        locals.var_fn205_calc_iq__qinvdd_dn10 = assign18280_e18045_d_n10;
        locals.var_fn205_calc_iq__qinvdd_dn11 = assign18280_e18045_d_n11;

        let (assign18290_e18079, assign18290_e18079_d_n2, assign18290_e18079_d_n4, assign18290_e18079_d_n7, assign18290_e18079_d_n10, assign18290_e18079_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18290_e18050: f64 = (2.0 * locals.var_fn205_calc_iq__qs3);
        let assign18290_e18053: f64 = (3.0 * locals.var_fn205_calc_iq__qd3);
        let assign18290_e18054: f64 = (assign18290_e18050 + assign18290_e18053);
        let assign18290_e18057: f64 = (4.0 * locals.var_fn205_calc_iq__qs2);
        let assign18290_e18059: f64 = (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0);
        let assign18290_e18060: f64 = (assign18290_e18054 + assign18290_e18059);
        let assign18290_e18063: f64 = (6.0 * locals.var_fn205_calc_iq__qd2);
        let assign18290_e18065: f64 = (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0);
        let assign18290_e18066: f64 = (assign18290_e18060 + assign18290_e18065);
        let assign18290_e18067: f64 = (2.0 * assign18290_e18066);
        let assign18290_e18071: f64 = (locals.var_fn205_calc_iq__qs2 + locals.var_fn205_calc_iq__qd2);
        let assign18290_e18074: f64 = (2.0 * locals.var_fn205_calc_iq__qsqd);
        let assign18290_e18075: f64 = (assign18290_e18071 + assign18290_e18074);
        let assign18290_e18076: f64 = (15.0 * assign18290_e18075);
        let assign18290_e18077: f64 = (assign18290_e18067 / assign18290_e18076);
        (assign18290_e18077, ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn2) + (3.0 * locals.var_fn205_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn2) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn2) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn2)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn2 + locals.var_fn205_calc_iq__qd2_dn2) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn2))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn4) + (3.0 * locals.var_fn205_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn4) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn4) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn4)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn4 + locals.var_fn205_calc_iq__qd2_dn4) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn4))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn7) + (3.0 * locals.var_fn205_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn7) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn7) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn7)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn7 + locals.var_fn205_calc_iq__qd2_dn7) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn7))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn10) + (3.0 * locals.var_fn205_calc_iq__qd3_dn10)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn10) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn10))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn10) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn10)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn10 + locals.var_fn205_calc_iq__qd2_dn10) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn10))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn11) + (3.0 * locals.var_fn205_calc_iq__qd3_dn11)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn11) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn11))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn11) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn11)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn11 + locals.var_fn205_calc_iq__qd2_dn11) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn11))))) / (assign18290_e18076 * assign18290_e18076)),)
    } else {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd1 = assign18290_e18079;
        locals.var_fn205_calc_iq__qd1_dn2 = assign18290_e18079_d_n2;
        locals.var_fn205_calc_iq__qd1_dn4 = assign18290_e18079_d_n4;
        locals.var_fn205_calc_iq__qd1_dn7 = assign18290_e18079_d_n7;
        locals.var_fn205_calc_iq__qd1_dn10 = assign18290_e18079_d_n10;
        locals.var_fn205_calc_iq__qd1_dn11 = assign18290_e18079_d_n11;

        let (assign18300_e18085, assign18300_e18085_d_n2, assign18300_e18085_d_n4, assign18300_e18085_d_n7, assign18300_e18085_d_n10, assign18300_e18085_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18300_e18083: f64 = (locals.var_fn205_calc_iq__qinvdd - locals.var_fn205_calc_iq__qd1);
        (assign18300_e18083, (locals.var_fn205_calc_iq__qinvdd_dn2 - locals.var_fn205_calc_iq__qd1_dn2), (locals.var_fn205_calc_iq__qinvdd_dn4 - locals.var_fn205_calc_iq__qd1_dn4), (locals.var_fn205_calc_iq__qinvdd_dn7 - locals.var_fn205_calc_iq__qd1_dn7), (locals.var_fn205_calc_iq__qinvdd_dn10 - locals.var_fn205_calc_iq__qd1_dn10), (locals.var_fn205_calc_iq__qinvdd_dn11 - locals.var_fn205_calc_iq__qd1_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qs, locals.var_fn205_calc_iq__qs_dn2, locals.var_fn205_calc_iq__qs_dn4, locals.var_fn205_calc_iq__qs_dn7, locals.var_fn205_calc_iq__qs_dn10, locals.var_fn205_calc_iq__qs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs = assign18300_e18085;
        locals.var_fn205_calc_iq__qs_dn2 = assign18300_e18085_d_n2;
        locals.var_fn205_calc_iq__qs_dn4 = assign18300_e18085_d_n4;
        locals.var_fn205_calc_iq__qs_dn7 = assign18300_e18085_d_n7;
        locals.var_fn205_calc_iq__qs_dn10 = assign18300_e18085_d_n10;
        locals.var_fn205_calc_iq__qs_dn11 = assign18300_e18085_d_n11;

        let (assign18310_e18089, assign18310_e18089_d_n2, assign18310_e18089_d_n4, assign18310_e18089_d_n7, assign18310_e18089_d_n10, assign18310_e18089_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    } else {
        (locals.var_fn205_calc_iq__qd, locals.var_fn205_calc_iq__qd_dn2, locals.var_fn205_calc_iq__qd_dn4, locals.var_fn205_calc_iq__qd_dn7, locals.var_fn205_calc_iq__qd_dn10, locals.var_fn205_calc_iq__qd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd = assign18310_e18089;
        locals.var_fn205_calc_iq__qd_dn2 = assign18310_e18089_d_n2;
        locals.var_fn205_calc_iq__qd_dn4 = assign18310_e18089_d_n4;
        locals.var_fn205_calc_iq__qd_dn7 = assign18310_e18089_d_n7;
        locals.var_fn205_calc_iq__qd_dn10 = assign18310_e18089_d_n10;
        locals.var_fn205_calc_iq__qd_dn11 = assign18310_e18089_d_n11;

        let (assign18320_e18103, assign18320_e18103_d_n2, assign18320_e18103_d_n4, assign18320_e18103_d_n7, assign18320_e18103_d_n10, assign18320_e18103_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18320_e18093: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18320_e18095: f64 = (assign18320_e18093 * locals.var_fn205_calc_iq__lin);
        let assign18320_e18097: f64 = (assign18320_e18095 * locals.var_fn205_calc_iq__type);
        let assign18320_e18099: f64 = (assign18320_e18097 * locals.var_fn205_calc_iq__qs);
        let assign18320_e18101: f64 = (assign18320_e18099 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18320_e18101, ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn4) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgsout = assign18320_e18103;
        locals.var_fn205_calc_iq__qgsout_dn2 = assign18320_e18103_d_n2;
        locals.var_fn205_calc_iq__qgsout_dn4 = assign18320_e18103_d_n4;
        locals.var_fn205_calc_iq__qgsout_dn7 = assign18320_e18103_d_n7;
        locals.var_fn205_calc_iq__qgsout_dn10 = assign18320_e18103_d_n10;
        locals.var_fn205_calc_iq__qgsout_dn11 = assign18320_e18103_d_n11;

        let (assign18330_e18117, assign18330_e18117_d_n2, assign18330_e18117_d_n4, assign18330_e18117_d_n7, assign18330_e18117_d_n10, assign18330_e18117_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18330_e18107: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18330_e18109: f64 = (assign18330_e18107 * locals.var_fn205_calc_iq__lin);
        let assign18330_e18111: f64 = (assign18330_e18109 * locals.var_fn205_calc_iq__type);
        let assign18330_e18113: f64 = (assign18330_e18111 * locals.var_fn205_calc_iq__qd);
        let assign18330_e18115: f64 = (assign18330_e18113 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18330_e18115, ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn4) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgdout = assign18330_e18117;
        locals.var_fn205_calc_iq__qgdout_dn2 = assign18330_e18117_d_n2;
        locals.var_fn205_calc_iq__qgdout_dn4 = assign18330_e18117_d_n4;
        locals.var_fn205_calc_iq__qgdout_dn7 = assign18330_e18117_d_n7;
        locals.var_fn205_calc_iq__qgdout_dn10 = assign18330_e18117_d_n10;
        locals.var_fn205_calc_iq__qgdout_dn11 = assign18330_e18117_d_n11;

        let assign18340_e18120: f64 = if locals.var_fn205_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign18340_e18120;

        let (assign18350_e18136, assign18350_e18136_d_n2, assign18350_e18136_d_n4, assign18350_e18136_d_n7, assign18350_e18136_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18350_e18128: f64 = (p.p51 * 0.5);
        let assign18350_e18130: f64 = (assign18350_e18128 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18350_e18131: f64 = (locals.var_fn205_calc_iq__vtof - assign18350_e18130);
        let assign18350_e18132: f64 = (locals.var_fn205_calc_iq__vcin - assign18350_e18131);
        let assign18350_e18134: f64 = (assign18350_e18132 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18350_e18134, (locals.var_fn205_calc_iq__vcin_dn2 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18350_e18128 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18350_e18132 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vcin_dn7 / locals.var_fn205_calc_iq__two_n_phit0), (locals.var_fn205_calc_iq__vcin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, locals.var_fn205_calc_iq__etac_dn11,)
    }
};
        locals.var_fn205_calc_iq__etac = assign18350_e18136;
        locals.var_fn205_calc_iq__etac_dn2 = assign18350_e18136_d_n2;
        locals.var_fn205_calc_iq__etac_dn4 = assign18350_e18136_d_n4;
        locals.var_fn205_calc_iq__etac_dn7 = assign18350_e18136_d_n7;
        locals.var_fn205_calc_iq__etac_dn11 = assign18350_e18136_d_n11;

        let assign18360_e18139: f64 = if locals.var_fn205_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign18360_e18139;

        let (assign18370_e18147, assign18370_e18147_d_n2, assign18370_e18147_d_n3, assign18370_e18147_d_n4, assign18370_e18147_d_n7, assign18370_e18147_d_n10, assign18370_e18147_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 != 0.0)) {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, 0.0, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, 0.0, locals.var_fn205_calc_iq__etac_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18370_e18147;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18370_e18147_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18370_e18147_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18370_e18147_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18370_e18147_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18370_e18147_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18370_e18147_d_n11;

        let assign18380_e18150: f64 = (-50.0);
        let assign18380_e18151: f64 = if locals.var_fn205_calc_iq__etac < assign18380_e18150 { 1.0 } else { 0.0 };
        locals.var_guard233 = assign18380_e18151;

        let (assign18390_e18163, assign18390_e18163_d_n2, assign18390_e18163_d_n3, assign18390_e18163_d_n4, assign18390_e18163_d_n7, assign18390_e18163_d_n10, assign18390_e18163_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 == 0.0)) && (locals.var_guard233 != 0.0)) {
        let assign18390_e18161: f64 = (locals.var_fn205_calc_iq__etac).exp();
        (assign18390_e18161, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn2), 0.0, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn4), (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn7), 0.0, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18390_e18163;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18390_e18163_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18390_e18163_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18390_e18163_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18390_e18163_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18390_e18163_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18390_e18163_d_n11;

        let (assign18400_e18179, assign18400_e18179_d_n2, assign18400_e18179_d_n3, assign18400_e18179_d_n4, assign18400_e18179_d_n7, assign18400_e18179_d_n10, assign18400_e18179_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 == 0.0)) && (locals.var_guard233 == 0.0)) {
        let assign18400_e18175: f64 = (locals.var_fn205_calc_iq__etac).exp();
        let assign18400_e18176: f64 = (1.0 + assign18400_e18175);
        let assign18400_e18177: f64 = (assign18400_e18176).ln();
        (assign18400_e18177, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn2) / assign18400_e18176), 0.0, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn4) / assign18400_e18176), ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn7) / assign18400_e18176), 0.0, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn11) / assign18400_e18176),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18400_e18179;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18400_e18179_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18400_e18179_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18400_e18179_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18400_e18179_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18400_e18179_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18400_e18179_d_n11;

        let (assign18410_e18197, assign18410_e18197_d_n2, assign18410_e18197_d_n3, assign18410_e18197_d_n4, assign18410_e18197_d_n7, assign18410_e18197_d_n10, assign18410_e18197_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18410_e18185: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18410_e18187: f64 = (assign18410_e18185 * locals.var_fn205_calc_iq__type);
        let assign18410_e18189: f64 = (assign18410_e18187 * locals.var_fn205_calc_iq__cc);
        let assign18410_e18191: f64 = (assign18410_e18189 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18410_e18193: f64 = (assign18410_e18191 * locals.var_fn205_calc_iq__exparg);
        let assign18410_e18195: f64 = (assign18410_e18193 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18410_e18195, ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((((assign18410_e18187 * locals.var_fn205_calc_iq__cc_dn4) * locals.var_fn205_calc_iq__two_n_phit0) + (assign18410_e18189 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) * locals.var_fn205_calc_iq__exparg) + (assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign18410_e18197;
        locals.var_fn205_calc_iq__qcout_dn2 = assign18410_e18197_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign18410_e18197_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign18410_e18197_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign18410_e18197_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign18410_e18197_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign18410_e18197_d_n11;

        let (assign18420_e18213, assign18420_e18213_d_n3, assign18420_e18213_d_n4, assign18420_e18213_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18420_e18205: f64 = (p.p51 * 0.5);
        let assign18420_e18207: f64 = (assign18420_e18205 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18420_e18208: f64 = (locals.var_fn205_calc_iq__vtof - assign18420_e18207);
        let assign18420_e18209: f64 = (locals.var_fn205_calc_iq__vbin - assign18420_e18208);
        let assign18420_e18211: f64 = (assign18420_e18209 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18420_e18211, (locals.var_fn205_calc_iq__vbin_dn3 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18420_e18205 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18420_e18209 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vbin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etab, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, locals.var_fn205_calc_iq__etab_dn11,)
    }
};
        locals.var_fn205_calc_iq__etab = assign18420_e18213;
        locals.var_fn205_calc_iq__etab_dn3 = assign18420_e18213_d_n3;
        locals.var_fn205_calc_iq__etab_dn4 = assign18420_e18213_d_n4;
        locals.var_fn205_calc_iq__etab_dn11 = assign18420_e18213_d_n11;

        let assign18430_e18216: f64 = if locals.var_fn205_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign18430_e18216;

        let (assign18440_e18224, assign18440_e18224_d_n2, assign18440_e18224_d_n3, assign18440_e18224_d_n4, assign18440_e18224_d_n7, assign18440_e18224_d_n10, assign18440_e18224_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        (locals.var_fn205_calc_iq__etab, 0.0, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn205_calc_iq__etab_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18440_e18224;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18440_e18224_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18440_e18224_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18440_e18224_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18440_e18224_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18440_e18224_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18440_e18224_d_n11;

        let assign18450_e18227: f64 = (-50.0);
        let assign18450_e18228: f64 = if locals.var_fn205_calc_iq__etab < assign18450_e18227 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign18450_e18228;

        let (assign18460_e18240, assign18460_e18240_d_n2, assign18460_e18240_d_n3, assign18460_e18240_d_n4, assign18460_e18240_d_n7, assign18460_e18240_d_n10, assign18460_e18240_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) && (locals.var_guard235 != 0.0)) {
        let assign18460_e18238: f64 = (locals.var_fn205_calc_iq__etab).exp();
        (assign18460_e18238, 0.0, (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn3), (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn4), 0.0, 0.0, (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18460_e18240;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18460_e18240_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18460_e18240_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18460_e18240_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18460_e18240_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18460_e18240_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18460_e18240_d_n11;

        let (assign18470_e18256, assign18470_e18256_d_n2, assign18470_e18256_d_n3, assign18470_e18256_d_n4, assign18470_e18256_d_n7, assign18470_e18256_d_n10, assign18470_e18256_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) && (locals.var_guard235 == 0.0)) {
        let assign18470_e18252: f64 = (locals.var_fn205_calc_iq__etab).exp();
        let assign18470_e18253: f64 = (1.0 + assign18470_e18252);
        let assign18470_e18254: f64 = (assign18470_e18253).ln();
        (assign18470_e18254, 0.0, ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn3) / assign18470_e18253), ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn4) / assign18470_e18253), 0.0, 0.0, ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn11) / assign18470_e18253),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18470_e18256;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18470_e18256_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18470_e18256_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18470_e18256_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18470_e18256_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18470_e18256_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18470_e18256_d_n11;

        let (assign18480_e18274, assign18480_e18274_d_n2, assign18480_e18274_d_n3, assign18480_e18274_d_n4, assign18480_e18274_d_n7, assign18480_e18274_d_n10, assign18480_e18274_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18480_e18262: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18480_e18264: f64 = (assign18480_e18262 * locals.var_fn205_calc_iq__type);
        let assign18480_e18266: f64 = (assign18480_e18264 * locals.var_fn205_calc_iq__cb);
        let assign18480_e18268: f64 = (assign18480_e18266 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18480_e18270: f64 = (assign18480_e18268 * locals.var_fn205_calc_iq__exparg);
        let assign18480_e18272: f64 = (assign18480_e18270 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18480_e18272, ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((((assign18480_e18264 * locals.var_fn205_calc_iq__cb_dn4) * locals.var_fn205_calc_iq__two_n_phit0) + (assign18480_e18266 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) * locals.var_fn205_calc_iq__exparg) + (assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign18480_e18274;
        locals.var_fn205_calc_iq__qbout_dn2 = assign18480_e18274_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign18480_e18274_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign18480_e18274_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign18480_e18274_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign18480_e18274_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign18480_e18274_d_n11;

        let (assign18490_e18281, assign18490_e18281_d_n2, assign18490_e18281_d_n3, assign18490_e18281_d_n4, assign18490_e18281_d_n7, assign18490_e18281_d_n10, assign18490_e18281_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign18490_e18281;
        locals.var_fn205_calc_iq__qcout_dn2 = assign18490_e18281_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign18490_e18281_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign18490_e18281_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign18490_e18281_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign18490_e18281_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign18490_e18281_d_n11;

        let (assign18500_e18288, assign18500_e18288_d_n2, assign18500_e18288_d_n3, assign18500_e18288_d_n4, assign18500_e18288_d_n7, assign18500_e18288_d_n10, assign18500_e18288_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign18500_e18288;
        locals.var_fn205_calc_iq__qbout_dn2 = assign18500_e18288_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign18500_e18288_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign18500_e18288_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign18500_e18288_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign18500_e18288_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign18500_e18288_d_n11;

        let assign18510_e18291: f64 = if locals.var_fn205_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard236 = assign18510_e18291;

        let (assign18520_e18307, assign18520_e18307_d_n2, assign18520_e18307_d_n4, assign18520_e18307_d_n7, assign18520_e18307_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) {
        let assign18520_e18299: f64 = (p.p51 * 0.5);
        let assign18520_e18301: f64 = (assign18520_e18299 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18520_e18302: f64 = (locals.var_fn205_calc_iq__vtof - assign18520_e18301);
        let assign18520_e18303: f64 = (locals.var_fn205_calc_iq__vgsin - assign18520_e18302);
        let assign18520_e18305: f64 = (assign18520_e18303 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18520_e18305, (locals.var_fn205_calc_iq__vgsin_dn2 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18520_e18299 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18520_e18303 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vgsin_dn7 / locals.var_fn205_calc_iq__two_n_phit0), (locals.var_fn205_calc_iq__vgsin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, locals.var_fn205_calc_iq__etags_dn11,)
    }
};
        locals.var_fn205_calc_iq__etags = assign18520_e18307;
        locals.var_fn205_calc_iq__etags_dn2 = assign18520_e18307_d_n2;
        locals.var_fn205_calc_iq__etags_dn4 = assign18520_e18307_d_n4;
        locals.var_fn205_calc_iq__etags_dn7 = assign18520_e18307_d_n7;
        locals.var_fn205_calc_iq__etags_dn11 = assign18520_e18307_d_n11;

        let assign18530_e18310: f64 = if locals.var_fn205_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign18530_e18310;

        let (assign18540_e18318, assign18540_e18318_d_n2, assign18540_e18318_d_n3, assign18540_e18318_d_n4, assign18540_e18318_d_n7, assign18540_e18318_d_n10, assign18540_e18318_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 != 0.0)) {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, 0.0, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, 0.0, locals.var_fn205_calc_iq__etags_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18540_e18318;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18540_e18318_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18540_e18318_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18540_e18318_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18540_e18318_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18540_e18318_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18540_e18318_d_n11;

        let assign18550_e18321: f64 = (-50.0);
        let assign18550_e18322: f64 = if locals.var_fn205_calc_iq__etags < assign18550_e18321 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign18550_e18322;

        let (assign18560_e18334, assign18560_e18334_d_n2, assign18560_e18334_d_n3, assign18560_e18334_d_n4, assign18560_e18334_d_n7, assign18560_e18334_d_n10, assign18560_e18334_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 == 0.0)) && (locals.var_guard238 != 0.0)) {
        let assign18560_e18332: f64 = (locals.var_fn205_calc_iq__etags).exp();
        (assign18560_e18332, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn2), 0.0, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn4), (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn7), 0.0, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18560_e18334;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18560_e18334_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18560_e18334_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18560_e18334_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18560_e18334_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18560_e18334_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18560_e18334_d_n11;

        let (assign18570_e18350, assign18570_e18350_d_n2, assign18570_e18350_d_n3, assign18570_e18350_d_n4, assign18570_e18350_d_n7, assign18570_e18350_d_n10, assign18570_e18350_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 == 0.0)) && (locals.var_guard238 == 0.0)) {
        let assign18570_e18346: f64 = (locals.var_fn205_calc_iq__etags).exp();
        let assign18570_e18347: f64 = (1.0 + assign18570_e18346);
        let assign18570_e18348: f64 = (assign18570_e18347).ln();
        (assign18570_e18348, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn2) / assign18570_e18347), 0.0, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn4) / assign18570_e18347), ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn7) / assign18570_e18347), 0.0, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn11) / assign18570_e18347),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18570_e18350;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18570_e18350_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18570_e18350_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18570_e18350_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18570_e18350_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18570_e18350_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18570_e18350_d_n11;

        let (assign18580_e18368, assign18580_e18368_d_n2, assign18580_e18368_d_n3, assign18580_e18368_d_n4, assign18580_e18368_d_n7, assign18580_e18368_d_n10, assign18580_e18368_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) {
        let assign18580_e18356: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18580_e18358: f64 = (assign18580_e18356 * locals.var_fn205_calc_iq__type);
        let assign18580_e18360: f64 = (assign18580_e18358 * locals.var_fn205_calc_iq__cs);
        let assign18580_e18362: f64 = (assign18580_e18360 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18580_e18364: f64 = (assign18580_e18362 * locals.var_fn205_calc_iq__exparg);
        let assign18580_e18366: f64 = (assign18580_e18364 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18580_e18366, ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((assign18580_e18360 * locals.var_fn205_calc_iq__two_n_phit0_dn4) * locals.var_fn205_calc_iq__exparg) + (assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign18580_e18368;
        locals.var_fn205_calc_iq__qsout_dn2 = assign18580_e18368_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign18580_e18368_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign18580_e18368_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign18580_e18368_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign18580_e18368_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign18580_e18368_d_n11;

        let (assign18590_e18375, assign18590_e18375_d_n2, assign18590_e18375_d_n3, assign18590_e18375_d_n4, assign18590_e18375_d_n7, assign18590_e18375_d_n10, assign18590_e18375_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign18590_e18375;
        locals.var_fn205_calc_iq__qsout_dn2 = assign18590_e18375_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign18590_e18375_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign18590_e18375_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign18590_e18375_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign18590_e18375_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign18590_e18375_d_n11;

        let (assign18600_e18379, assign18600_e18379_d_n2, assign18600_e18379_d_n3, assign18600_e18379_d_n4, assign18600_e18379_d_n7, assign18600_e18379_d_n10, assign18600_e18379_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__idsout, locals.var_fn205_calc_iq__idsout_dn2, locals.var_fn205_calc_iq__idsout_dn3, locals.var_fn205_calc_iq__idsout_dn4, locals.var_fn205_calc_iq__idsout_dn7, locals.var_fn205_calc_iq__idsout_dn10, locals.var_fn205_calc_iq__idsout_dn11,)
    } else {
        (locals.var_fn205_calc_iq__return, locals.var_fn205_calc_iq__return_dn2, locals.var_fn205_calc_iq__return_dn3, locals.var_fn205_calc_iq__return_dn4, locals.var_fn205_calc_iq__return_dn7, locals.var_fn205_calc_iq__return_dn10, locals.var_fn205_calc_iq__return_dn11,)
    }
};
        locals.var_fn205_calc_iq__return = assign18600_e18379;
        locals.var_fn205_calc_iq__return_dn2 = assign18600_e18379_d_n2;
        locals.var_fn205_calc_iq__return_dn3 = assign18600_e18379_d_n3;
        locals.var_fn205_calc_iq__return_dn4 = assign18600_e18379_d_n4;
        locals.var_fn205_calc_iq__return_dn7 = assign18600_e18379_d_n7;
        locals.var_fn205_calc_iq__return_dn10 = assign18600_e18379_d_n10;
        locals.var_fn205_calc_iq__return_dn11 = assign18600_e18379_d_n11;

        let (assign18610_e18383, assign18610_e18383_d_n2, assign18610_e18383_d_n3, assign18610_e18383_d_n4, assign18610_e18383_d_n7, assign18610_e18383_d_n10, assign18610_e18383_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__idsout, locals.var_fn205_calc_iq__idsout_dn2, locals.var_fn205_calc_iq__idsout_dn3, locals.var_fn205_calc_iq__idsout_dn4, locals.var_fn205_calc_iq__idsout_dn7, locals.var_fn205_calc_iq__idsout_dn10, locals.var_fn205_calc_iq__idsout_dn11,)
    } else {
        (locals.var_idsfps2, locals.var_idsfps2_dn2, locals.var_idsfps2_dn3, locals.var_idsfps2_dn4, locals.var_idsfps2_dn7, locals.var_idsfps2_dn10, locals.var_idsfps2_dn11,)
    }
};
        locals.var_idsfps2 = assign18610_e18383;
        locals.var_idsfps2_dn2 = assign18610_e18383_d_n2;
        locals.var_idsfps2_dn3 = assign18610_e18383_d_n3;
        locals.var_idsfps2_dn4 = assign18610_e18383_d_n4;
        locals.var_idsfps2_dn7 = assign18610_e18383_d_n7;
        locals.var_idsfps2_dn10 = assign18610_e18383_d_n10;
        locals.var_idsfps2_dn11 = assign18610_e18383_d_n11;

        let (assign18620_e18387, assign18620_e18387_d_n2, assign18620_e18387_d_n4, assign18620_e18387_d_n7, assign18620_e18387_d_n10, assign18620_e18387_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    } else {
        (locals.var_qgsfps2, locals.var_qgsfps2_dn2, locals.var_qgsfps2_dn4, locals.var_qgsfps2_dn7, locals.var_qgsfps2_dn10, locals.var_qgsfps2_dn11,)
    }
};
        locals.var_qgsfps2 = assign18620_e18387;
        locals.var_qgsfps2_dn2 = assign18620_e18387_d_n2;
        locals.var_qgsfps2_dn4 = assign18620_e18387_d_n4;
        locals.var_qgsfps2_dn7 = assign18620_e18387_d_n7;
        locals.var_qgsfps2_dn10 = assign18620_e18387_d_n10;
        locals.var_qgsfps2_dn11 = assign18620_e18387_d_n11;

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18630_e18391, assign18630_e18391_d_n2, assign18630_e18391_d_n4, assign18630_e18391_d_n7, assign18630_e18391_d_n10, assign18630_e18391_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    } else {
        (locals.var_qgdfps2, locals.var_qgdfps2_dn2, locals.var_qgdfps2_dn4, locals.var_qgdfps2_dn7, locals.var_qgdfps2_dn10, locals.var_qgdfps2_dn11,)
    }
};
        locals.var_qgdfps2 = assign18630_e18391;
        locals.var_qgdfps2_dn2 = assign18630_e18391_d_n2;
        locals.var_qgdfps2_dn4 = assign18630_e18391_d_n4;
        locals.var_qgdfps2_dn7 = assign18630_e18391_d_n7;
        locals.var_qgdfps2_dn10 = assign18630_e18391_d_n10;
        locals.var_qgdfps2_dn11 = assign18630_e18391_d_n11;

        let (assign18640_e18395, assign18640_e18395_d_n2, assign18640_e18395_d_n3, assign18640_e18395_d_n4, assign18640_e18395_d_n7, assign18640_e18395_d_n10, assign18640_e18395_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    } else {
        (locals.var_qcfps2, locals.var_qcfps2_dn2, locals.var_qcfps2_dn3, locals.var_qcfps2_dn4, locals.var_qcfps2_dn7, locals.var_qcfps2_dn10, locals.var_qcfps2_dn11,)
    }
};
        locals.var_qcfps2 = assign18640_e18395;
        locals.var_qcfps2_dn2 = assign18640_e18395_d_n2;
        locals.var_qcfps2_dn3 = assign18640_e18395_d_n3;
        locals.var_qcfps2_dn4 = assign18640_e18395_d_n4;
        locals.var_qcfps2_dn7 = assign18640_e18395_d_n7;
        locals.var_qcfps2_dn10 = assign18640_e18395_d_n10;
        locals.var_qcfps2_dn11 = assign18640_e18395_d_n11;

        let (assign18650_e18399, assign18650_e18399_d_n2, assign18650_e18399_d_n3, assign18650_e18399_d_n4, assign18650_e18399_d_n7, assign18650_e18399_d_n10, assign18650_e18399_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    } else {
        (locals.var_qbfps2, locals.var_qbfps2_dn2, locals.var_qbfps2_dn3, locals.var_qbfps2_dn4, locals.var_qbfps2_dn7, locals.var_qbfps2_dn10, locals.var_qbfps2_dn11,)
    }
};
        locals.var_qbfps2 = assign18650_e18399;
        locals.var_qbfps2_dn2 = assign18650_e18399_d_n2;
        locals.var_qbfps2_dn3 = assign18650_e18399_d_n3;
        locals.var_qbfps2_dn4 = assign18650_e18399_d_n4;
        locals.var_qbfps2_dn7 = assign18650_e18399_d_n7;
        locals.var_qbfps2_dn10 = assign18650_e18399_d_n10;
        locals.var_qbfps2_dn11 = assign18650_e18399_d_n11;

        let (assign18660_e18403, assign18660_e18403_d_n2, assign18660_e18403_d_n3, assign18660_e18403_d_n4, assign18660_e18403_d_n7, assign18660_e18403_d_n10, assign18660_e18403_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    } else {
        (locals.var_qsfps2, locals.var_qsfps2_dn2, locals.var_qsfps2_dn3, locals.var_qsfps2_dn4, locals.var_qsfps2_dn7, locals.var_qsfps2_dn10, locals.var_qsfps2_dn11,)
    }
};
        locals.var_qsfps2 = assign18660_e18403;
        locals.var_qsfps2_dn2 = assign18660_e18403_d_n2;
        locals.var_qsfps2_dn3 = assign18660_e18403_d_n3;
        locals.var_qsfps2_dn4 = assign18660_e18403_d_n4;
        locals.var_qsfps2_dn7 = assign18660_e18403_d_n7;
        locals.var_qsfps2_dn10 = assign18660_e18403_d_n10;
        locals.var_qsfps2_dn11 = assign18660_e18403_d_n11;

        let (assign18690_e18415, assign18690_e18415_d_n2, assign18690_e18415_d_n3, assign18690_e18415_d_n4, assign18690_e18415_d_n7, assign18690_e18415_d_n10, assign18690_e18415_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__return, locals.var_fn205_calc_iq__return_dn2, locals.var_fn205_calc_iq__return_dn3, locals.var_fn205_calc_iq__return_dn4, locals.var_fn205_calc_iq__return_dn7, locals.var_fn205_calc_iq__return_dn10, locals.var_fn205_calc_iq__return_dn11,)
    } else {
        (locals.var_idsfps2, locals.var_idsfps2_dn2, locals.var_idsfps2_dn3, locals.var_idsfps2_dn4, locals.var_idsfps2_dn7, locals.var_idsfps2_dn10, locals.var_idsfps2_dn11,)
    }
};
        locals.var_idsfps2 = assign18690_e18415;
        locals.var_idsfps2_dn2 = assign18690_e18415_d_n2;
        locals.var_idsfps2_dn3 = assign18690_e18415_d_n3;
        locals.var_idsfps2_dn4 = assign18690_e18415_d_n4;
        locals.var_idsfps2_dn7 = assign18690_e18415_d_n7;
        locals.var_idsfps2_dn10 = assign18690_e18415_d_n10;
        locals.var_idsfps2_dn11 = assign18690_e18415_d_n11;

        let assign18700_e18418: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign18700_e18418;

        locals.var_idsfps3 = 0.0;
        locals.var_idsfps3_dn2 = 0.0;
        locals.var_idsfps3_dn3 = 0.0;
        locals.var_idsfps3_dn4 = 0.0;
        locals.var_idsfps3_dn7 = 0.0;
        locals.var_idsfps3_dn11 = 0.0;
        locals.var_idsfps3_dn12 = 0.0;

        locals.var_qgsfps3 = 0.0;
        locals.var_qgsfps3_dn2 = 0.0;
        locals.var_qgsfps3_dn4 = 0.0;
        locals.var_qgsfps3_dn7 = 0.0;
        locals.var_qgsfps3_dn11 = 0.0;
        locals.var_qgsfps3_dn12 = 0.0;

        locals.var_qgdfps3 = 0.0;
        locals.var_qgdfps3_dn2 = 0.0;
        locals.var_qgdfps3_dn4 = 0.0;
        locals.var_qgdfps3_dn7 = 0.0;
        locals.var_qgdfps3_dn11 = 0.0;
        locals.var_qgdfps3_dn12 = 0.0;

        locals.var_qcfps3 = 0.0;
        locals.var_qcfps3_dn2 = 0.0;
        locals.var_qcfps3_dn3 = 0.0;
        locals.var_qcfps3_dn4 = 0.0;
        locals.var_qcfps3_dn7 = 0.0;
        locals.var_qcfps3_dn11 = 0.0;
        locals.var_qcfps3_dn12 = 0.0;

        locals.var_qbfps3 = 0.0;
        locals.var_qbfps3_dn2 = 0.0;
        locals.var_qbfps3_dn3 = 0.0;
        locals.var_qbfps3_dn4 = 0.0;
        locals.var_qbfps3_dn7 = 0.0;
        locals.var_qbfps3_dn11 = 0.0;
        locals.var_qbfps3_dn12 = 0.0;

        locals.var_qsfps3 = 0.0;
        locals.var_qsfps3_dn2 = 0.0;
        locals.var_qsfps3_dn3 = 0.0;
        locals.var_qsfps3_dn4 = 0.0;
        locals.var_qsfps3_dn7 = 0.0;
        locals.var_qsfps3_dn11 = 0.0;
        locals.var_qsfps3_dn12 = 0.0;

        let assign18790_e18429: f64 = if p.p123 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign18790_e18429;

        let (assign18800_e18433, assign18800_e18433_d_n2, assign18800_e18433_d_n3, assign18800_e18433_d_n4, assign18800_e18433_d_n7, assign18800_e18433_d_n11, assign18800_e18433_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__return, locals.var_fn241_calc_iq__return_dn2, locals.var_fn241_calc_iq__return_dn3, locals.var_fn241_calc_iq__return_dn4, locals.var_fn241_calc_iq__return_dn7, locals.var_fn241_calc_iq__return_dn11, locals.var_fn241_calc_iq__return_dn12,)
    }
};
        locals.var_fn241_calc_iq__return = assign18800_e18433;
        locals.var_fn241_calc_iq__return_dn2 = assign18800_e18433_d_n2;
        locals.var_fn241_calc_iq__return_dn3 = assign18800_e18433_d_n3;
        locals.var_fn241_calc_iq__return_dn4 = assign18800_e18433_d_n4;
        locals.var_fn241_calc_iq__return_dn7 = assign18800_e18433_d_n7;
        locals.var_fn241_calc_iq__return_dn11 = assign18800_e18433_d_n11;
        locals.var_fn241_calc_iq__return_dn12 = assign18800_e18433_d_n12;

        let (assign18810_e18437, assign18810_e18437_d_n2, assign18810_e18437_d_n3, assign18810_e18437_d_n4, assign18810_e18437_d_n7, assign18810_e18437_d_n11, assign18810_e18437_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__idsout, locals.var_fn241_calc_iq__idsout_dn2, locals.var_fn241_calc_iq__idsout_dn3, locals.var_fn241_calc_iq__idsout_dn4, locals.var_fn241_calc_iq__idsout_dn7, locals.var_fn241_calc_iq__idsout_dn11, locals.var_fn241_calc_iq__idsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__idsout = assign18810_e18437;
        locals.var_fn241_calc_iq__idsout_dn2 = assign18810_e18437_d_n2;
        locals.var_fn241_calc_iq__idsout_dn3 = assign18810_e18437_d_n3;
        locals.var_fn241_calc_iq__idsout_dn4 = assign18810_e18437_d_n4;
        locals.var_fn241_calc_iq__idsout_dn7 = assign18810_e18437_d_n7;
        locals.var_fn241_calc_iq__idsout_dn11 = assign18810_e18437_d_n11;
        locals.var_fn241_calc_iq__idsout_dn12 = assign18810_e18437_d_n12;

        let (assign18820_e18441, assign18820_e18441_d_n2, assign18820_e18441_d_n4, assign18820_e18441_d_n7, assign18820_e18441_d_n11, assign18820_e18441_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgsout = assign18820_e18441;
        locals.var_fn241_calc_iq__qgsout_dn2 = assign18820_e18441_d_n2;
        locals.var_fn241_calc_iq__qgsout_dn4 = assign18820_e18441_d_n4;
        locals.var_fn241_calc_iq__qgsout_dn7 = assign18820_e18441_d_n7;
        locals.var_fn241_calc_iq__qgsout_dn11 = assign18820_e18441_d_n11;
        locals.var_fn241_calc_iq__qgsout_dn12 = assign18820_e18441_d_n12;

        let (assign18830_e18445, assign18830_e18445_d_n2, assign18830_e18445_d_n4, assign18830_e18445_d_n7, assign18830_e18445_d_n11, assign18830_e18445_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgdout = assign18830_e18445;
        locals.var_fn241_calc_iq__qgdout_dn2 = assign18830_e18445_d_n2;
        locals.var_fn241_calc_iq__qgdout_dn4 = assign18830_e18445_d_n4;
        locals.var_fn241_calc_iq__qgdout_dn7 = assign18830_e18445_d_n7;
        locals.var_fn241_calc_iq__qgdout_dn11 = assign18830_e18445_d_n11;
        locals.var_fn241_calc_iq__qgdout_dn12 = assign18830_e18445_d_n12;

        let (assign18840_e18449, assign18840_e18449_d_n2, assign18840_e18449_d_n3, assign18840_e18449_d_n4, assign18840_e18449_d_n7, assign18840_e18449_d_n11, assign18840_e18449_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign18840_e18449;
        locals.var_fn241_calc_iq__qcout_dn2 = assign18840_e18449_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign18840_e18449_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign18840_e18449_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign18840_e18449_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign18840_e18449_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign18840_e18449_d_n12;

        let (assign18850_e18453, assign18850_e18453_d_n2, assign18850_e18453_d_n3, assign18850_e18453_d_n4, assign18850_e18453_d_n7, assign18850_e18453_d_n11, assign18850_e18453_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign18850_e18453;
        locals.var_fn241_calc_iq__qbout_dn2 = assign18850_e18453_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign18850_e18453_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign18850_e18453_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign18850_e18453_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign18850_e18453_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign18850_e18453_d_n12;

        let (assign18860_e18457, assign18860_e18457_d_n2, assign18860_e18457_d_n3, assign18860_e18457_d_n4, assign18860_e18457_d_n7, assign18860_e18457_d_n11, assign18860_e18457_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign18860_e18457;
        locals.var_fn241_calc_iq__qsout_dn2 = assign18860_e18457_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign18860_e18457_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign18860_e18457_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign18860_e18457_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign18860_e18457_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign18860_e18457_d_n12;

        let (assign18870_e18461, assign18870_e18461_d_n4, assign18870_e18461_d_n11, assign18870_e18461_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vtdibl, locals.var_fn241_calc_iq__vtdibl_dn4, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vtdibl = assign18870_e18461;
        locals.var_fn241_calc_iq__vtdibl_dn4 = assign18870_e18461_d_n4;
        locals.var_fn241_calc_iq__vtdibl_dn11 = assign18870_e18461_d_n11;
        locals.var_fn241_calc_iq__vtdibl_dn12 = assign18870_e18461_d_n12;

        let (assign18880_e18465, assign18880_e18465_d_n2, assign18880_e18465_d_n3, assign18880_e18465_d_n4, assign18880_e18465_d_n7, assign18880_e18465_d_n11, assign18880_e18465_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat1, locals.var_fn241_calc_iq__vdsat1_dn2, locals.var_fn241_calc_iq__vdsat1_dn3, locals.var_fn241_calc_iq__vdsat1_dn4, locals.var_fn241_calc_iq__vdsat1_dn7, locals.var_fn241_calc_iq__vdsat1_dn11, locals.var_fn241_calc_iq__vdsat1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat1 = assign18880_e18465;
        locals.var_fn241_calc_iq__vdsat1_dn2 = assign18880_e18465_d_n2;
        locals.var_fn241_calc_iq__vdsat1_dn3 = assign18880_e18465_d_n3;
        locals.var_fn241_calc_iq__vdsat1_dn4 = assign18880_e18465_d_n4;
        locals.var_fn241_calc_iq__vdsat1_dn7 = assign18880_e18465_d_n7;
        locals.var_fn241_calc_iq__vdsat1_dn11 = assign18880_e18465_d_n11;
        locals.var_fn241_calc_iq__vdsat1_dn12 = assign18880_e18465_d_n12;

        let (assign18890_e18469, assign18890_e18469_d_n2, assign18890_e18469_d_n7, assign18890_e18469_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vgsin, locals.var_fn241_calc_iq__vgsin_dn2, locals.var_fn241_calc_iq__vgsin_dn7, locals.var_fn241_calc_iq__vgsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgsin = assign18890_e18469;
        locals.var_fn241_calc_iq__vgsin_dn2 = assign18890_e18469_d_n2;
        locals.var_fn241_calc_iq__vgsin_dn7 = assign18890_e18469_d_n7;
        locals.var_fn241_calc_iq__vgsin_dn12 = assign18890_e18469_d_n12;

        let (assign18900_e18473, assign18900_e18473_d_n11, assign18900_e18473_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vdsfps3, locals.var_vdsfps3_dn11, locals.var_vdsfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vdsin, locals.var_fn241_calc_iq__vdsin_dn11, locals.var_fn241_calc_iq__vdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsin = assign18900_e18473;
        locals.var_fn241_calc_iq__vdsin_dn11 = assign18900_e18473_d_n11;
        locals.var_fn241_calc_iq__vdsin_dn12 = assign18900_e18473_d_n12;

        let (assign18910_e18477,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p129,)
    } else {
        (locals.var_fn241_calc_iq__qcbflag,)
    }
};
        locals.var_fn241_calc_iq__qcbflag = assign18910_e18477;

        let (assign18920_e18481, assign18920_e18481_d_n2, assign18920_e18481_d_n7, assign18920_e18481_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vcin, locals.var_fn241_calc_iq__vcin_dn2, locals.var_fn241_calc_iq__vcin_dn7, locals.var_fn241_calc_iq__vcin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vcin = assign18920_e18481;
        locals.var_fn241_calc_iq__vcin_dn2 = assign18920_e18481_d_n2;
        locals.var_fn241_calc_iq__vcin_dn7 = assign18920_e18481_d_n7;
        locals.var_fn241_calc_iq__vcin_dn12 = assign18920_e18481_d_n12;

        let (assign18930_e18485, assign18930_e18485_d_n3, assign18930_e18485_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vbfps3, locals.var_vbfps3_dn3, locals.var_vbfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vbin, locals.var_fn241_calc_iq__vbin_dn3, locals.var_fn241_calc_iq__vbin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vbin = assign18930_e18485;
        locals.var_fn241_calc_iq__vbin_dn3 = assign18930_e18485_d_n3;
        locals.var_fn241_calc_iq__vbin_dn12 = assign18930_e18485_d_n12;

        let (assign18940_e18489,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p127,)
    } else {
        (locals.var_fn241_calc_iq__qgsflag,)
    }
};
        locals.var_fn241_calc_iq__qgsflag = assign18940_e18489;

        let (assign18950_e18493, assign18950_e18493_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn241_calc_iq__tambin, locals.var_fn241_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tambin = assign18950_e18493;
        locals.var_fn241_calc_iq__tambin_dn4 = assign18950_e18493_d_n4;

        let (assign18960_e18497,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn241_calc_iq__tnomin,)
    }
};
        locals.var_fn241_calc_iq__tnomin = assign18960_e18497;

        let (assign18970_e18501, assign18970_e18501_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn241_calc_iq__phitin, locals.var_fn241_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn241_calc_iq__phitin = assign18970_e18501;
        locals.var_fn241_calc_iq__phitin_dn4 = assign18970_e18501_d_n4;

        let (assign18980_e18505,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn241_calc_iq__w,)
    }
};
        locals.var_fn241_calc_iq__w = assign18980_e18505;

        let (assign18990_e18509,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_fn241_calc_iq__lin,)
    }
};
        locals.var_fn241_calc_iq__lin = assign18990_e18509;

        let (assign19000_e18513, assign19000_e18513_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_cgfps3t, locals.var_cgfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cgin, locals.var_fn241_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn241_calc_iq__cgin = assign19000_e18513;
        locals.var_fn241_calc_iq__cgin_dn4 = assign19000_e18513_d_n4;

        let (assign19010_e18517,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p128,)
    } else {
        (locals.var_fn241_calc_iq__cs,)
    }
};
        locals.var_fn241_calc_iq__cs = assign19010_e18517;

        let (assign19020_e18521, assign19020_e18521_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_ccfps3t, locals.var_ccfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cc, locals.var_fn241_calc_iq__cc_dn4,)
    }
};
        locals.var_fn241_calc_iq__cc = assign19020_e18521;
        locals.var_fn241_calc_iq__cc_dn4 = assign19020_e18521_d_n4;

        let (assign19030_e18525, assign19030_e18525_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_cbfps3t, locals.var_cbfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cb, locals.var_fn241_calc_iq__cb_dn4,)
    }
};
        locals.var_fn241_calc_iq__cb = assign19030_e18525;
        locals.var_fn241_calc_iq__cb_dn4 = assign19030_e18525_d_n4;

        let (assign19040_e18529,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_fn241_calc_iq__vto,)
    }
};
        locals.var_fn241_calc_iq__vto = assign19040_e18529;

        let (assign19050_e18533,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_fn241_calc_iq__ss,)
    }
};
        locals.var_fn241_calc_iq__ss = assign19050_e18533;

        let (assign19060_e18537,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_fn241_calc_iq__delta1,)
    }
};
        locals.var_fn241_calc_iq__delta1 = assign19060_e18537;

        let (assign19070_e18541,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn241_calc_iq__delta2,)
    }
};
        locals.var_fn241_calc_iq__delta2 = assign19070_e18541;

        let (assign19080_e18545,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_fn241_calc_iq__nd,)
    }
};
        locals.var_fn241_calc_iq__nd = assign19080_e18545;

        let (assign19090_e18549,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p143,)
    } else {
        (locals.var_fn241_calc_iq__alpha,)
    }
};
        locals.var_fn241_calc_iq__alpha = assign19090_e18549;

        let (assign19100_e18553,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p134,)
    } else {
        (locals.var_fn241_calc_iq__vel0,)
    }
};
        locals.var_fn241_calc_iq__vel0 = assign19100_e18553;

        let (assign19110_e18557,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p135,)
    } else {
        (locals.var_fn241_calc_iq__mu0,)
    }
};
        locals.var_fn241_calc_iq__mu0 = assign19110_e18557;

        let (assign19120_e18561,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p136,)
    } else {
        (locals.var_fn241_calc_iq__beta,)
    }
};
        locals.var_fn241_calc_iq__beta = assign19120_e18561;

        let (assign19130_e18565,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p142,)
    } else {
        (locals.var_fn241_calc_iq__mtheta,)
    }
};
        locals.var_fn241_calc_iq__mtheta = assign19130_e18565;

        let (assign19140_e18569,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p141,)
    } else {
        (locals.var_fn241_calc_iq__vtheta,)
    }
};
        locals.var_fn241_calc_iq__vtheta = assign19140_e18569;

    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19150_e18573,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p140,)
    } else {
        (locals.var_fn241_calc_iq__vtzeta,)
    }
};
        locals.var_fn241_calc_iq__vtzeta = assign19150_e18573;

        let (assign19160_e18577,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn241_calc_iq__dibsat,)
    }
};
        locals.var_fn241_calc_iq__dibsat = assign19160_e18577;

        let (assign19170_e18581,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn241_calc_iq__epsilon,)
    }
};
        locals.var_fn241_calc_iq__epsilon = assign19170_e18581;

        let (assign19180_e18585,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn241_calc_iq__vzeta,)
    }
};
        locals.var_fn241_calc_iq__vzeta = assign19180_e18585;

        let (assign19190_e18589,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn241_calc_iq__lambda,)
    }
};
        locals.var_fn241_calc_iq__lambda = assign19190_e18589;

        let (assign19200_e18593,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn241_calc_iq__ngf,)
    }
};
        locals.var_fn241_calc_iq__ngf = assign19200_e18593;

        let (assign19210_e18597,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn241_calc_iq__type,)
    }
};
        locals.var_fn241_calc_iq__type = assign19210_e18597;

        let (assign19220_e18601,) = {
    if (locals.var_guard240 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn241_calc_iq__trapfracdl,)
    }
};
        locals.var_fn241_calc_iq__trapfracdl = assign19220_e18601;

        let (assign19230_e18605, assign19230_e18605_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__alpha_phit, locals.var_fn241_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn241_calc_iq__alpha_phit = assign19230_e18605;
        locals.var_fn241_calc_iq__alpha_phit_dn4 = assign19230_e18605_d_n4;

        let (assign19240_e18609, assign19240_e18609_d_n11, assign19240_e18609_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__delta, locals.var_fn241_calc_iq__delta_dn11, locals.var_fn241_calc_iq__delta_dn12,)
    }
};
        locals.var_fn241_calc_iq__delta = assign19240_e18609;
        locals.var_fn241_calc_iq__delta_dn11 = assign19240_e18609_d_n11;
        locals.var_fn241_calc_iq__delta_dn12 = assign19240_e18609_d_n12;

        let (assign19250_e18613, assign19250_e18613_d_n4, assign19250_e18613_d_n11, assign19250_e18613_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__n, locals.var_fn241_calc_iq__n_dn4, locals.var_fn241_calc_iq__n_dn11, locals.var_fn241_calc_iq__n_dn12,)
    }
};
        locals.var_fn241_calc_iq__n = assign19250_e18613;
        locals.var_fn241_calc_iq__n_dn4 = assign19250_e18613_d_n4;
        locals.var_fn241_calc_iq__n_dn11 = assign19250_e18613_d_n11;
        locals.var_fn241_calc_iq__n_dn12 = assign19250_e18613_d_n12;

        let (assign19260_e18617, assign19260_e18617_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vtof, locals.var_fn241_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn241_calc_iq__vtof = assign19260_e18617;
        locals.var_fn241_calc_iq__vtof_dn4 = assign19260_e18617_d_n4;

        let (assign19270_e18621, assign19270_e18621_d_n11, assign19270_e18621_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign19270_e18621;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign19270_e18621_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign19270_e18621_d_n12;

        let (assign19280_e18625, assign19280_e18625_d_n2, assign19280_e18625_d_n3, assign19280_e18625_d_n4, assign19280_e18625_d_n7, assign19280_e18625_d_n11, assign19280_e18625_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign19280_e18625;
        locals.var_fn241_calc_iq__ffs_dn2 = assign19280_e18625_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign19280_e18625_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign19280_e18625_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign19280_e18625_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign19280_e18625_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign19280_e18625_d_n12;

        let (assign19290_e18629, assign19290_e18629_d_n4, assign19290_e18629_d_n11, assign19290_e18629_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit, locals.var_fn241_calc_iq__two_n_phit_dn4, locals.var_fn241_calc_iq__two_n_phit_dn11, locals.var_fn241_calc_iq__two_n_phit_dn12,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit = assign19290_e18629;
        locals.var_fn241_calc_iq__two_n_phit_dn4 = assign19290_e18629_d_n4;
        locals.var_fn241_calc_iq__two_n_phit_dn11 = assign19290_e18629_d_n11;
        locals.var_fn241_calc_iq__two_n_phit_dn12 = assign19290_e18629_d_n12;

        let (assign19300_e18633, assign19300_e18633_d_n4, assign19300_e18633_d_n11, assign19300_e18633_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qref, locals.var_fn241_calc_iq__qref_dn4, locals.var_fn241_calc_iq__qref_dn11, locals.var_fn241_calc_iq__qref_dn12,)
    }
};
        locals.var_fn241_calc_iq__qref = assign19300_e18633;
        locals.var_fn241_calc_iq__qref_dn4 = assign19300_e18633_d_n4;
        locals.var_fn241_calc_iq__qref_dn11 = assign19300_e18633_d_n11;
        locals.var_fn241_calc_iq__qref_dn12 = assign19300_e18633_d_n12;

        let (assign19310_e18637, assign19310_e18637_d_n2, assign19310_e18637_d_n3, assign19310_e18637_d_n4, assign19310_e18637_d_n7, assign19310_e18637_d_n11, assign19310_e18637_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etas, locals.var_fn241_calc_iq__etas_dn2, locals.var_fn241_calc_iq__etas_dn3, locals.var_fn241_calc_iq__etas_dn4, locals.var_fn241_calc_iq__etas_dn7, locals.var_fn241_calc_iq__etas_dn11, locals.var_fn241_calc_iq__etas_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas = assign19310_e18637;
        locals.var_fn241_calc_iq__etas_dn2 = assign19310_e18637_d_n2;
        locals.var_fn241_calc_iq__etas_dn3 = assign19310_e18637_d_n3;
        locals.var_fn241_calc_iq__etas_dn4 = assign19310_e18637_d_n4;
        locals.var_fn241_calc_iq__etas_dn7 = assign19310_e18637_d_n7;
        locals.var_fn241_calc_iq__etas_dn11 = assign19310_e18637_d_n11;
        locals.var_fn241_calc_iq__etas_dn12 = assign19310_e18637_d_n12;

        let (assign19320_e18641, assign19320_e18641_d_n2, assign19320_e18641_d_n3, assign19320_e18641_d_n4, assign19320_e18641_d_n7, assign19320_e18641_d_n11, assign19320_e18641_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign19320_e18641;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign19320_e18641_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign19320_e18641_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign19320_e18641_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign19320_e18641_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign19320_e18641_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign19320_e18641_d_n12;

        let (assign19330_e18645, assign19330_e18645_d_n2, assign19330_e18645_d_n3, assign19330_e18645_d_n4, assign19330_e18645_d_n7, assign19330_e18645_d_n11, assign19330_e18645_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__muf, locals.var_fn241_calc_iq__muf_dn2, locals.var_fn241_calc_iq__muf_dn3, locals.var_fn241_calc_iq__muf_dn4, locals.var_fn241_calc_iq__muf_dn7, locals.var_fn241_calc_iq__muf_dn11, locals.var_fn241_calc_iq__muf_dn12,)
    }
};
        locals.var_fn241_calc_iq__muf = assign19330_e18645;
        locals.var_fn241_calc_iq__muf_dn2 = assign19330_e18645_d_n2;
        locals.var_fn241_calc_iq__muf_dn3 = assign19330_e18645_d_n3;
        locals.var_fn241_calc_iq__muf_dn4 = assign19330_e18645_d_n4;
        locals.var_fn241_calc_iq__muf_dn7 = assign19330_e18645_d_n7;
        locals.var_fn241_calc_iq__muf_dn11 = assign19330_e18645_d_n11;
        locals.var_fn241_calc_iq__muf_dn12 = assign19330_e18645_d_n12;

        let (assign19340_e18649, assign19340_e18649_d_n2, assign19340_e18649_d_n3, assign19340_e18649_d_n4, assign19340_e18649_d_n7, assign19340_e18649_d_n11, assign19340_e18649_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vx, locals.var_fn241_calc_iq__vx_dn2, locals.var_fn241_calc_iq__vx_dn3, locals.var_fn241_calc_iq__vx_dn4, locals.var_fn241_calc_iq__vx_dn7, locals.var_fn241_calc_iq__vx_dn11, locals.var_fn241_calc_iq__vx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vx = assign19340_e18649;
        locals.var_fn241_calc_iq__vx_dn2 = assign19340_e18649_d_n2;
        locals.var_fn241_calc_iq__vx_dn3 = assign19340_e18649_d_n3;
        locals.var_fn241_calc_iq__vx_dn4 = assign19340_e18649_d_n4;
        locals.var_fn241_calc_iq__vx_dn7 = assign19340_e18649_d_n7;
        locals.var_fn241_calc_iq__vx_dn11 = assign19340_e18649_d_n11;
        locals.var_fn241_calc_iq__vx_dn12 = assign19340_e18649_d_n12;

        let (assign19350_e18653, assign19350_e18653_d_n2, assign19350_e18653_d_n3, assign19350_e18653_d_n4, assign19350_e18653_d_n7, assign19350_e18653_d_n11, assign19350_e18653_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vxf, locals.var_fn241_calc_iq__vxf_dn2, locals.var_fn241_calc_iq__vxf_dn3, locals.var_fn241_calc_iq__vxf_dn4, locals.var_fn241_calc_iq__vxf_dn7, locals.var_fn241_calc_iq__vxf_dn11, locals.var_fn241_calc_iq__vxf_dn12,)
    }
};
        locals.var_fn241_calc_iq__vxf = assign19350_e18653;
        locals.var_fn241_calc_iq__vxf_dn2 = assign19350_e18653_d_n2;
        locals.var_fn241_calc_iq__vxf_dn3 = assign19350_e18653_d_n3;
        locals.var_fn241_calc_iq__vxf_dn4 = assign19350_e18653_d_n4;
        locals.var_fn241_calc_iq__vxf_dn7 = assign19350_e18653_d_n7;
        locals.var_fn241_calc_iq__vxf_dn11 = assign19350_e18653_d_n11;
        locals.var_fn241_calc_iq__vxf_dn12 = assign19350_e18653_d_n12;

        let (assign19360_e18657, assign19360_e18657_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__n0, locals.var_fn241_calc_iq__n0_dn4,)
    }
};
        locals.var_fn241_calc_iq__n0 = assign19360_e18657;
        locals.var_fn241_calc_iq__n0_dn4 = assign19360_e18657_d_n4;

        let (assign19370_e18661, assign19370_e18661_d_n2, assign19370_e18661_d_n4, assign19370_e18661_d_n7, assign19370_e18661_d_n11, assign19370_e18661_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign19370_e18661;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign19370_e18661_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign19370_e18661_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign19370_e18661_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign19370_e18661_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign19370_e18661_d_n12;

        let (assign19380_e18665, assign19380_e18665_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit0, locals.var_fn241_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit0 = assign19380_e18665;
        locals.var_fn241_calc_iq__two_n_phit0_dn4 = assign19380_e18665_d_n4;

        let (assign19390_e18669, assign19390_e18669_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qref0, locals.var_fn241_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn241_calc_iq__qref0 = assign19390_e18669;
        locals.var_fn241_calc_iq__qref0_dn4 = assign19390_e18669_d_n4;

        let (assign19400_e18673, assign19400_e18673_d_n2, assign19400_e18673_d_n4, assign19400_e18673_d_n7, assign19400_e18673_d_n11, assign19400_e18673_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etas0, locals.var_fn241_calc_iq__etas0_dn2, locals.var_fn241_calc_iq__etas0_dn4, locals.var_fn241_calc_iq__etas0_dn7, locals.var_fn241_calc_iq__etas0_dn11, locals.var_fn241_calc_iq__etas0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas0 = assign19400_e18673;
        locals.var_fn241_calc_iq__etas0_dn2 = assign19400_e18673_d_n2;
        locals.var_fn241_calc_iq__etas0_dn4 = assign19400_e18673_d_n4;
        locals.var_fn241_calc_iq__etas0_dn7 = assign19400_e18673_d_n7;
        locals.var_fn241_calc_iq__etas0_dn11 = assign19400_e18673_d_n11;
        locals.var_fn241_calc_iq__etas0_dn12 = assign19400_e18673_d_n12;

        let (assign19410_e18677, assign19410_e18677_d_n2, assign19410_e18677_d_n4, assign19410_e18677_d_n7, assign19410_e18677_d_n11, assign19410_e18677_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign19410_e18677;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign19410_e18677_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign19410_e18677_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign19410_e18677_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign19410_e18677_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign19410_e18677_d_n12;

        let (assign19420_e18681, assign19420_e18681_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__muf0, locals.var_fn241_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn241_calc_iq__muf0 = assign19420_e18681;
        locals.var_fn241_calc_iq__muf0_dn4 = assign19420_e18681_d_n4;

        let (assign19430_e18685, assign19430_e18685_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vx0, locals.var_fn241_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vx0 = assign19430_e18685;
        locals.var_fn241_calc_iq__vx0_dn4 = assign19430_e18685_d_n4;

        let (assign19440_e18689, assign19440_e18689_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__tfacmobin, locals.var_fn241_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tfacmobin = assign19440_e18689;
        locals.var_fn241_calc_iq__tfacmobin_dn4 = assign19440_e18689_d_n4;

        let (assign19450_e18693, assign19450_e18693_d_n2, assign19450_e18693_d_n3, assign19450_e18693_d_n4, assign19450_e18693_d_n7, assign19450_e18693_d_n11, assign19450_e18693_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign19450_e18693;
        locals.var_fn241_calc_iq__ff_dn2 = assign19450_e18693_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign19450_e18693_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign19450_e18693_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign19450_e18693_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign19450_e18693_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign19450_e18693_d_n12;

        let (assign19460_e18697, assign19460_e18697_d_n2, assign19460_e18697_d_n3, assign19460_e18697_d_n4, assign19460_e18697_d_n7, assign19460_e18697_d_n11, assign19460_e18697_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__eta, locals.var_fn241_calc_iq__eta_dn2, locals.var_fn241_calc_iq__eta_dn3, locals.var_fn241_calc_iq__eta_dn4, locals.var_fn241_calc_iq__eta_dn7, locals.var_fn241_calc_iq__eta_dn11, locals.var_fn241_calc_iq__eta_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta = assign19460_e18697;
        locals.var_fn241_calc_iq__eta_dn2 = assign19460_e18697_d_n2;
        locals.var_fn241_calc_iq__eta_dn3 = assign19460_e18697_d_n3;
        locals.var_fn241_calc_iq__eta_dn4 = assign19460_e18697_d_n4;
        locals.var_fn241_calc_iq__eta_dn7 = assign19460_e18697_d_n7;
        locals.var_fn241_calc_iq__eta_dn11 = assign19460_e18697_d_n11;
        locals.var_fn241_calc_iq__eta_dn12 = assign19460_e18697_d_n12;

        let (assign19470_e18701, assign19470_e18701_d_n2, assign19470_e18701_d_n3, assign19470_e18701_d_n4, assign19470_e18701_d_n7, assign19470_e18701_d_n11, assign19470_e18701_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign19470_e18701;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign19470_e18701_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign19470_e18701_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign19470_e18701_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign19470_e18701_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign19470_e18701_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign19470_e18701_d_n12;

        let (assign19480_e18705, assign19480_e18705_d_n2, assign19480_e18705_d_n4, assign19480_e18705_d_n7, assign19480_e18705_d_n11, assign19480_e18705_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign19480_e18705;
        locals.var_fn241_calc_iq__ff0_dn2 = assign19480_e18705_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign19480_e18705_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign19480_e18705_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign19480_e18705_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign19480_e18705_d_n12;

        let (assign19490_e18709, assign19490_e18709_d_n2, assign19490_e18709_d_n4, assign19490_e18709_d_n7, assign19490_e18709_d_n11, assign19490_e18709_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__eta0, locals.var_fn241_calc_iq__eta0_dn2, locals.var_fn241_calc_iq__eta0_dn4, locals.var_fn241_calc_iq__eta0_dn7, locals.var_fn241_calc_iq__eta0_dn11, locals.var_fn241_calc_iq__eta0_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta0 = assign19490_e18709;
        locals.var_fn241_calc_iq__eta0_dn2 = assign19490_e18709_d_n2;
        locals.var_fn241_calc_iq__eta0_dn4 = assign19490_e18709_d_n4;
        locals.var_fn241_calc_iq__eta0_dn7 = assign19490_e18709_d_n7;
        locals.var_fn241_calc_iq__eta0_dn11 = assign19490_e18709_d_n11;
        locals.var_fn241_calc_iq__eta0_dn12 = assign19490_e18709_d_n12;

        let (assign19500_e18713, assign19500_e18713_d_n2, assign19500_e18713_d_n4, assign19500_e18713_d_n7, assign19500_e18713_d_n11, assign19500_e18713_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign19500_e18713;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign19500_e18713_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign19500_e18713_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign19500_e18713_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign19500_e18713_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign19500_e18713_d_n12;

        let (assign19510_e18717, assign19510_e18717_d_n2, assign19510_e18717_d_n3, assign19510_e18717_d_n4, assign19510_e18717_d_n7, assign19510_e18717_d_n11, assign19510_e18717_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats, locals.var_fn241_calc_iq__vdsats_dn2, locals.var_fn241_calc_iq__vdsats_dn3, locals.var_fn241_calc_iq__vdsats_dn4, locals.var_fn241_calc_iq__vdsats_dn7, locals.var_fn241_calc_iq__vdsats_dn11, locals.var_fn241_calc_iq__vdsats_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats = assign19510_e18717;
        locals.var_fn241_calc_iq__vdsats_dn2 = assign19510_e18717_d_n2;
        locals.var_fn241_calc_iq__vdsats_dn3 = assign19510_e18717_d_n3;
        locals.var_fn241_calc_iq__vdsats_dn4 = assign19510_e18717_d_n4;
        locals.var_fn241_calc_iq__vdsats_dn7 = assign19510_e18717_d_n7;
        locals.var_fn241_calc_iq__vdsats_dn11 = assign19510_e18717_d_n11;
        locals.var_fn241_calc_iq__vdsats_dn12 = assign19510_e18717_d_n12;

        let (assign19520_e18721, assign19520_e18721_d_n2, assign19520_e18721_d_n3, assign19520_e18721_d_n4, assign19520_e18721_d_n7, assign19520_e18721_d_n11, assign19520_e18721_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats1, locals.var_fn241_calc_iq__vdsats1_dn2, locals.var_fn241_calc_iq__vdsats1_dn3, locals.var_fn241_calc_iq__vdsats1_dn4, locals.var_fn241_calc_iq__vdsats1_dn7, locals.var_fn241_calc_iq__vdsats1_dn11, locals.var_fn241_calc_iq__vdsats1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats1 = assign19520_e18721;
        locals.var_fn241_calc_iq__vdsats1_dn2 = assign19520_e18721_d_n2;
        locals.var_fn241_calc_iq__vdsats1_dn3 = assign19520_e18721_d_n3;
        locals.var_fn241_calc_iq__vdsats1_dn4 = assign19520_e18721_d_n4;
        locals.var_fn241_calc_iq__vdsats1_dn7 = assign19520_e18721_d_n7;
        locals.var_fn241_calc_iq__vdsats1_dn11 = assign19520_e18721_d_n11;
        locals.var_fn241_calc_iq__vdsats1_dn12 = assign19520_e18721_d_n12;

        let (assign19530_e18725, assign19530_e18725_d_n2, assign19530_e18725_d_n3, assign19530_e18725_d_n4, assign19530_e18725_d_n7, assign19530_e18725_d_n11, assign19530_e18725_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat, locals.var_fn241_calc_iq__vdsat_dn2, locals.var_fn241_calc_iq__vdsat_dn3, locals.var_fn241_calc_iq__vdsat_dn4, locals.var_fn241_calc_iq__vdsat_dn7, locals.var_fn241_calc_iq__vdsat_dn11, locals.var_fn241_calc_iq__vdsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat = assign19530_e18725;
        locals.var_fn241_calc_iq__vdsat_dn2 = assign19530_e18725_d_n2;
        locals.var_fn241_calc_iq__vdsat_dn3 = assign19530_e18725_d_n3;
        locals.var_fn241_calc_iq__vdsat_dn4 = assign19530_e18725_d_n4;
        locals.var_fn241_calc_iq__vdsat_dn7 = assign19530_e18725_d_n7;
        locals.var_fn241_calc_iq__vdsat_dn11 = assign19530_e18725_d_n11;
        locals.var_fn241_calc_iq__vdsat_dn12 = assign19530_e18725_d_n12;

        let (assign19540_e18729, assign19540_e18729_d_n2, assign19540_e18729_d_n3, assign19540_e18729_d_n4, assign19540_e18729_d_n7, assign19540_e18729_d_n11, assign19540_e18729_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fsd, locals.var_fn241_calc_iq__fsd_dn2, locals.var_fn241_calc_iq__fsd_dn3, locals.var_fn241_calc_iq__fsd_dn4, locals.var_fn241_calc_iq__fsd_dn7, locals.var_fn241_calc_iq__fsd_dn11, locals.var_fn241_calc_iq__fsd_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd = assign19540_e18729;
        locals.var_fn241_calc_iq__fsd_dn2 = assign19540_e18729_d_n2;
        locals.var_fn241_calc_iq__fsd_dn3 = assign19540_e18729_d_n3;
        locals.var_fn241_calc_iq__fsd_dn4 = assign19540_e18729_d_n4;
        locals.var_fn241_calc_iq__fsd_dn7 = assign19540_e18729_d_n7;
        locals.var_fn241_calc_iq__fsd_dn11 = assign19540_e18729_d_n11;
        locals.var_fn241_calc_iq__fsd_dn12 = assign19540_e18729_d_n12;

        let (assign19550_e18733, assign19550_e18733_d_n2, assign19550_e18733_d_n3, assign19550_e18733_d_n4, assign19550_e18733_d_n7, assign19550_e18733_d_n11, assign19550_e18733_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdx, locals.var_fn241_calc_iq__vdx_dn2, locals.var_fn241_calc_iq__vdx_dn3, locals.var_fn241_calc_iq__vdx_dn4, locals.var_fn241_calc_iq__vdx_dn7, locals.var_fn241_calc_iq__vdx_dn11, locals.var_fn241_calc_iq__vdx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx = assign19550_e18733;
        locals.var_fn241_calc_iq__vdx_dn2 = assign19550_e18733_d_n2;
        locals.var_fn241_calc_iq__vdx_dn3 = assign19550_e18733_d_n3;
        locals.var_fn241_calc_iq__vdx_dn4 = assign19550_e18733_d_n4;
        locals.var_fn241_calc_iq__vdx_dn7 = assign19550_e18733_d_n7;
        locals.var_fn241_calc_iq__vdx_dn11 = assign19550_e18733_d_n11;
        locals.var_fn241_calc_iq__vdx_dn12 = assign19550_e18733_d_n12;

    }

    pub(super) fn stamp_transient_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign19560_e18737, assign19560_e18737_d_n2, assign19560_e18737_d_n3, assign19560_e18737_d_n4, assign19560_e18737_d_n7, assign19560_e18737_d_n11, assign19560_e18737_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fds, locals.var_fn241_calc_iq__fds_dn2, locals.var_fn241_calc_iq__fds_dn3, locals.var_fn241_calc_iq__fds_dn4, locals.var_fn241_calc_iq__fds_dn7, locals.var_fn241_calc_iq__fds_dn11, locals.var_fn241_calc_iq__fds_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds = assign19560_e18737;
        locals.var_fn241_calc_iq__fds_dn2 = assign19560_e18737_d_n2;
        locals.var_fn241_calc_iq__fds_dn3 = assign19560_e18737_d_n3;
        locals.var_fn241_calc_iq__fds_dn4 = assign19560_e18737_d_n4;
        locals.var_fn241_calc_iq__fds_dn7 = assign19560_e18737_d_n7;
        locals.var_fn241_calc_iq__fds_dn11 = assign19560_e18737_d_n11;
        locals.var_fn241_calc_iq__fds_dn12 = assign19560_e18737_d_n12;

        let (assign19570_e18741, assign19570_e18741_d_n2, assign19570_e18741_d_n3, assign19570_e18741_d_n4, assign19570_e18741_d_n7, assign19570_e18741_d_n11, assign19570_e18741_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsx, locals.var_fn241_calc_iq__vsx_dn2, locals.var_fn241_calc_iq__vsx_dn3, locals.var_fn241_calc_iq__vsx_dn4, locals.var_fn241_calc_iq__vsx_dn7, locals.var_fn241_calc_iq__vsx_dn11, locals.var_fn241_calc_iq__vsx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx = assign19570_e18741;
        locals.var_fn241_calc_iq__vsx_dn2 = assign19570_e18741_d_n2;
        locals.var_fn241_calc_iq__vsx_dn3 = assign19570_e18741_d_n3;
        locals.var_fn241_calc_iq__vsx_dn4 = assign19570_e18741_d_n4;
        locals.var_fn241_calc_iq__vsx_dn7 = assign19570_e18741_d_n7;
        locals.var_fn241_calc_iq__vsx_dn11 = assign19570_e18741_d_n11;
        locals.var_fn241_calc_iq__vsx_dn12 = assign19570_e18741_d_n12;

        let (assign19580_e18745, assign19580_e18745_d_n2, assign19580_e18745_d_n3, assign19580_e18745_d_n4, assign19580_e18745_d_n7, assign19580_e18745_d_n11, assign19580_e18745_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign19580_e18745;
        locals.var_fn241_calc_iq__ffd_dn2 = assign19580_e18745_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign19580_e18745_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign19580_e18745_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign19580_e18745_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign19580_e18745_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign19580_e18745_d_n12;

        let (assign19590_e18749, assign19590_e18749_d_n2, assign19590_e18749_d_n3, assign19590_e18749_d_n4, assign19590_e18749_d_n7, assign19590_e18749_d_n11, assign19590_e18749_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etad, locals.var_fn241_calc_iq__etad_dn2, locals.var_fn241_calc_iq__etad_dn3, locals.var_fn241_calc_iq__etad_dn4, locals.var_fn241_calc_iq__etad_dn7, locals.var_fn241_calc_iq__etad_dn11, locals.var_fn241_calc_iq__etad_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad = assign19590_e18749;
        locals.var_fn241_calc_iq__etad_dn2 = assign19590_e18749_d_n2;
        locals.var_fn241_calc_iq__etad_dn3 = assign19590_e18749_d_n3;
        locals.var_fn241_calc_iq__etad_dn4 = assign19590_e18749_d_n4;
        locals.var_fn241_calc_iq__etad_dn7 = assign19590_e18749_d_n7;
        locals.var_fn241_calc_iq__etad_dn11 = assign19590_e18749_d_n11;
        locals.var_fn241_calc_iq__etad_dn12 = assign19590_e18749_d_n12;

        let (assign19600_e18753, assign19600_e18753_d_n2, assign19600_e18753_d_n3, assign19600_e18753_d_n4, assign19600_e18753_d_n7, assign19600_e18753_d_n11, assign19600_e18753_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign19600_e18753;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign19600_e18753_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign19600_e18753_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign19600_e18753_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign19600_e18753_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign19600_e18753_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign19600_e18753_d_n12;

        let (assign19610_e18757, assign19610_e18757_d_n2, assign19610_e18757_d_n3, assign19610_e18757_d_n4, assign19610_e18757_d_n7, assign19610_e18757_d_n11, assign19610_e18757_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsc, locals.var_fn241_calc_iq__vdsc_dn2, locals.var_fn241_calc_iq__vdsc_dn3, locals.var_fn241_calc_iq__vdsc_dn4, locals.var_fn241_calc_iq__vdsc_dn7, locals.var_fn241_calc_iq__vdsc_dn11, locals.var_fn241_calc_iq__vdsc_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsc = assign19610_e18757;
        locals.var_fn241_calc_iq__vdsc_dn2 = assign19610_e18757_d_n2;
        locals.var_fn241_calc_iq__vdsc_dn3 = assign19610_e18757_d_n3;
        locals.var_fn241_calc_iq__vdsc_dn4 = assign19610_e18757_d_n4;
        locals.var_fn241_calc_iq__vdsc_dn7 = assign19610_e18757_d_n7;
        locals.var_fn241_calc_iq__vdsc_dn11 = assign19610_e18757_d_n11;
        locals.var_fn241_calc_iq__vdsc_dn12 = assign19610_e18757_d_n12;

        let (assign19620_e18761, assign19620_e18761_d_n2, assign19620_e18761_d_n3, assign19620_e18761_d_n4, assign19620_e18761_d_n7, assign19620_e18761_d_n11, assign19620_e18761_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fsat, locals.var_fn241_calc_iq__fsat_dn2, locals.var_fn241_calc_iq__fsat_dn3, locals.var_fn241_calc_iq__fsat_dn4, locals.var_fn241_calc_iq__fsat_dn7, locals.var_fn241_calc_iq__fsat_dn11, locals.var_fn241_calc_iq__fsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsat = assign19620_e18761;
        locals.var_fn241_calc_iq__fsat_dn2 = assign19620_e18761_d_n2;
        locals.var_fn241_calc_iq__fsat_dn3 = assign19620_e18761_d_n3;
        locals.var_fn241_calc_iq__fsat_dn4 = assign19620_e18761_d_n4;
        locals.var_fn241_calc_iq__fsat_dn7 = assign19620_e18761_d_n7;
        locals.var_fn241_calc_iq__fsat_dn11 = assign19620_e18761_d_n11;
        locals.var_fn241_calc_iq__fsat_dn12 = assign19620_e18761_d_n12;

        let (assign19630_e18765, assign19630_e18765_d_n2, assign19630_e18765_d_n3, assign19630_e18765_d_n4, assign19630_e18765_d_n7, assign19630_e18765_d_n11, assign19630_e18765_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vel, locals.var_fn241_calc_iq__vel_dn2, locals.var_fn241_calc_iq__vel_dn3, locals.var_fn241_calc_iq__vel_dn4, locals.var_fn241_calc_iq__vel_dn7, locals.var_fn241_calc_iq__vel_dn11, locals.var_fn241_calc_iq__vel_dn12,)
    }
};
        locals.var_fn241_calc_iq__vel = assign19630_e18765;
        locals.var_fn241_calc_iq__vel_dn2 = assign19630_e18765_d_n2;
        locals.var_fn241_calc_iq__vel_dn3 = assign19630_e18765_d_n3;
        locals.var_fn241_calc_iq__vel_dn4 = assign19630_e18765_d_n4;
        locals.var_fn241_calc_iq__vel_dn7 = assign19630_e18765_d_n7;
        locals.var_fn241_calc_iq__vel_dn11 = assign19630_e18765_d_n11;
        locals.var_fn241_calc_iq__vel_dn12 = assign19630_e18765_d_n12;

        let (assign19640_e18769, assign19640_e18769_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats0, locals.var_fn241_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vdsats0 = assign19640_e18769;
        locals.var_fn241_calc_iq__vdsats0_dn4 = assign19640_e18769_d_n4;

        let (assign19650_e18773, assign19650_e18773_d_n2, assign19650_e18773_d_n4, assign19650_e18773_d_n7, assign19650_e18773_d_n11, assign19650_e18773_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats10, locals.var_fn241_calc_iq__vdsats10_dn2, locals.var_fn241_calc_iq__vdsats10_dn4, locals.var_fn241_calc_iq__vdsats10_dn7, locals.var_fn241_calc_iq__vdsats10_dn11, locals.var_fn241_calc_iq__vdsats10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats10 = assign19650_e18773;
        locals.var_fn241_calc_iq__vdsats10_dn2 = assign19650_e18773_d_n2;
        locals.var_fn241_calc_iq__vdsats10_dn4 = assign19650_e18773_d_n4;
        locals.var_fn241_calc_iq__vdsats10_dn7 = assign19650_e18773_d_n7;
        locals.var_fn241_calc_iq__vdsats10_dn11 = assign19650_e18773_d_n11;
        locals.var_fn241_calc_iq__vdsats10_dn12 = assign19650_e18773_d_n12;

        let (assign19660_e18777, assign19660_e18777_d_n2, assign19660_e18777_d_n4, assign19660_e18777_d_n7, assign19660_e18777_d_n11, assign19660_e18777_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat10, locals.var_fn241_calc_iq__vdsat10_dn2, locals.var_fn241_calc_iq__vdsat10_dn4, locals.var_fn241_calc_iq__vdsat10_dn7, locals.var_fn241_calc_iq__vdsat10_dn11, locals.var_fn241_calc_iq__vdsat10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat10 = assign19660_e18777;
        locals.var_fn241_calc_iq__vdsat10_dn2 = assign19660_e18777_d_n2;
        locals.var_fn241_calc_iq__vdsat10_dn4 = assign19660_e18777_d_n4;
        locals.var_fn241_calc_iq__vdsat10_dn7 = assign19660_e18777_d_n7;
        locals.var_fn241_calc_iq__vdsat10_dn11 = assign19660_e18777_d_n11;
        locals.var_fn241_calc_iq__vdsat10_dn12 = assign19660_e18777_d_n12;

        let (assign19670_e18781, assign19670_e18781_d_n2, assign19670_e18781_d_n4, assign19670_e18781_d_n7, assign19670_e18781_d_n11, assign19670_e18781_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fsd0, locals.var_fn241_calc_iq__fsd0_dn2, locals.var_fn241_calc_iq__fsd0_dn4, locals.var_fn241_calc_iq__fsd0_dn7, locals.var_fn241_calc_iq__fsd0_dn11, locals.var_fn241_calc_iq__fsd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd0 = assign19670_e18781;
        locals.var_fn241_calc_iq__fsd0_dn2 = assign19670_e18781_d_n2;
        locals.var_fn241_calc_iq__fsd0_dn4 = assign19670_e18781_d_n4;
        locals.var_fn241_calc_iq__fsd0_dn7 = assign19670_e18781_d_n7;
        locals.var_fn241_calc_iq__fsd0_dn11 = assign19670_e18781_d_n11;
        locals.var_fn241_calc_iq__fsd0_dn12 = assign19670_e18781_d_n12;

        let (assign19680_e18785, assign19680_e18785_d_n2, assign19680_e18785_d_n4, assign19680_e18785_d_n7, assign19680_e18785_d_n11, assign19680_e18785_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdx0, locals.var_fn241_calc_iq__vdx0_dn2, locals.var_fn241_calc_iq__vdx0_dn4, locals.var_fn241_calc_iq__vdx0_dn7, locals.var_fn241_calc_iq__vdx0_dn11, locals.var_fn241_calc_iq__vdx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx0 = assign19680_e18785;
        locals.var_fn241_calc_iq__vdx0_dn2 = assign19680_e18785_d_n2;
        locals.var_fn241_calc_iq__vdx0_dn4 = assign19680_e18785_d_n4;
        locals.var_fn241_calc_iq__vdx0_dn7 = assign19680_e18785_d_n7;
        locals.var_fn241_calc_iq__vdx0_dn11 = assign19680_e18785_d_n11;
        locals.var_fn241_calc_iq__vdx0_dn12 = assign19680_e18785_d_n12;

        let (assign19690_e18789, assign19690_e18789_d_n2, assign19690_e18789_d_n4, assign19690_e18789_d_n7, assign19690_e18789_d_n11, assign19690_e18789_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fds0, locals.var_fn241_calc_iq__fds0_dn2, locals.var_fn241_calc_iq__fds0_dn4, locals.var_fn241_calc_iq__fds0_dn7, locals.var_fn241_calc_iq__fds0_dn11, locals.var_fn241_calc_iq__fds0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds0 = assign19690_e18789;
        locals.var_fn241_calc_iq__fds0_dn2 = assign19690_e18789_d_n2;
        locals.var_fn241_calc_iq__fds0_dn4 = assign19690_e18789_d_n4;
        locals.var_fn241_calc_iq__fds0_dn7 = assign19690_e18789_d_n7;
        locals.var_fn241_calc_iq__fds0_dn11 = assign19690_e18789_d_n11;
        locals.var_fn241_calc_iq__fds0_dn12 = assign19690_e18789_d_n12;

        let (assign19700_e18793, assign19700_e18793_d_n2, assign19700_e18793_d_n4, assign19700_e18793_d_n7, assign19700_e18793_d_n11, assign19700_e18793_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsx0, locals.var_fn241_calc_iq__vsx0_dn2, locals.var_fn241_calc_iq__vsx0_dn4, locals.var_fn241_calc_iq__vsx0_dn7, locals.var_fn241_calc_iq__vsx0_dn11, locals.var_fn241_calc_iq__vsx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx0 = assign19700_e18793;
        locals.var_fn241_calc_iq__vsx0_dn2 = assign19700_e18793_d_n2;
        locals.var_fn241_calc_iq__vsx0_dn4 = assign19700_e18793_d_n4;
        locals.var_fn241_calc_iq__vsx0_dn7 = assign19700_e18793_d_n7;
        locals.var_fn241_calc_iq__vsx0_dn11 = assign19700_e18793_d_n11;
        locals.var_fn241_calc_iq__vsx0_dn12 = assign19700_e18793_d_n12;

        let (assign19710_e18797, assign19710_e18797_d_n2, assign19710_e18797_d_n4, assign19710_e18797_d_n7, assign19710_e18797_d_n11, assign19710_e18797_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign19710_e18797;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign19710_e18797_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign19710_e18797_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign19710_e18797_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign19710_e18797_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign19710_e18797_d_n12;

        let (assign19720_e18801, assign19720_e18801_d_n2, assign19720_e18801_d_n4, assign19720_e18801_d_n7, assign19720_e18801_d_n11, assign19720_e18801_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etad0, locals.var_fn241_calc_iq__etad0_dn2, locals.var_fn241_calc_iq__etad0_dn4, locals.var_fn241_calc_iq__etad0_dn7, locals.var_fn241_calc_iq__etad0_dn11, locals.var_fn241_calc_iq__etad0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad0 = assign19720_e18801;
        locals.var_fn241_calc_iq__etad0_dn2 = assign19720_e18801_d_n2;
        locals.var_fn241_calc_iq__etad0_dn4 = assign19720_e18801_d_n4;
        locals.var_fn241_calc_iq__etad0_dn7 = assign19720_e18801_d_n7;
        locals.var_fn241_calc_iq__etad0_dn11 = assign19720_e18801_d_n11;
        locals.var_fn241_calc_iq__etad0_dn12 = assign19720_e18801_d_n12;

        let (assign19730_e18805, assign19730_e18805_d_n2, assign19730_e18805_d_n4, assign19730_e18805_d_n7, assign19730_e18805_d_n11, assign19730_e18805_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign19730_e18805;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign19730_e18805_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign19730_e18805_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign19730_e18805_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign19730_e18805_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign19730_e18805_d_n12;

        let (assign19740_e18809, assign19740_e18809_d_n2, assign19740_e18809_d_n4, assign19740_e18809_d_n7, assign19740_e18809_d_n11, assign19740_e18809_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs2, locals.var_fn241_calc_iq__qs2_dn2, locals.var_fn241_calc_iq__qs2_dn4, locals.var_fn241_calc_iq__qs2_dn7, locals.var_fn241_calc_iq__qs2_dn11, locals.var_fn241_calc_iq__qs2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs2 = assign19740_e18809;
        locals.var_fn241_calc_iq__qs2_dn2 = assign19740_e18809_d_n2;
        locals.var_fn241_calc_iq__qs2_dn4 = assign19740_e18809_d_n4;
        locals.var_fn241_calc_iq__qs2_dn7 = assign19740_e18809_d_n7;
        locals.var_fn241_calc_iq__qs2_dn11 = assign19740_e18809_d_n11;
        locals.var_fn241_calc_iq__qs2_dn12 = assign19740_e18809_d_n12;

        let (assign19750_e18813, assign19750_e18813_d_n2, assign19750_e18813_d_n4, assign19750_e18813_d_n7, assign19750_e18813_d_n11, assign19750_e18813_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs3, locals.var_fn241_calc_iq__qs3_dn2, locals.var_fn241_calc_iq__qs3_dn4, locals.var_fn241_calc_iq__qs3_dn7, locals.var_fn241_calc_iq__qs3_dn11, locals.var_fn241_calc_iq__qs3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs3 = assign19750_e18813;
        locals.var_fn241_calc_iq__qs3_dn2 = assign19750_e18813_d_n2;
        locals.var_fn241_calc_iq__qs3_dn4 = assign19750_e18813_d_n4;
        locals.var_fn241_calc_iq__qs3_dn7 = assign19750_e18813_d_n7;
        locals.var_fn241_calc_iq__qs3_dn11 = assign19750_e18813_d_n11;
        locals.var_fn241_calc_iq__qs3_dn12 = assign19750_e18813_d_n12;

        let (assign19760_e18817, assign19760_e18817_d_n2, assign19760_e18817_d_n4, assign19760_e18817_d_n7, assign19760_e18817_d_n11, assign19760_e18817_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd2, locals.var_fn241_calc_iq__qd2_dn2, locals.var_fn241_calc_iq__qd2_dn4, locals.var_fn241_calc_iq__qd2_dn7, locals.var_fn241_calc_iq__qd2_dn11, locals.var_fn241_calc_iq__qd2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd2 = assign19760_e18817;
        locals.var_fn241_calc_iq__qd2_dn2 = assign19760_e18817_d_n2;
        locals.var_fn241_calc_iq__qd2_dn4 = assign19760_e18817_d_n4;
        locals.var_fn241_calc_iq__qd2_dn7 = assign19760_e18817_d_n7;
        locals.var_fn241_calc_iq__qd2_dn11 = assign19760_e18817_d_n11;
        locals.var_fn241_calc_iq__qd2_dn12 = assign19760_e18817_d_n12;

        let (assign19770_e18821, assign19770_e18821_d_n2, assign19770_e18821_d_n4, assign19770_e18821_d_n7, assign19770_e18821_d_n11, assign19770_e18821_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd3, locals.var_fn241_calc_iq__qd3_dn2, locals.var_fn241_calc_iq__qd3_dn4, locals.var_fn241_calc_iq__qd3_dn7, locals.var_fn241_calc_iq__qd3_dn11, locals.var_fn241_calc_iq__qd3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd3 = assign19770_e18821;
        locals.var_fn241_calc_iq__qd3_dn2 = assign19770_e18821_d_n2;
        locals.var_fn241_calc_iq__qd3_dn4 = assign19770_e18821_d_n4;
        locals.var_fn241_calc_iq__qd3_dn7 = assign19770_e18821_d_n7;
        locals.var_fn241_calc_iq__qd3_dn11 = assign19770_e18821_d_n11;
        locals.var_fn241_calc_iq__qd3_dn12 = assign19770_e18821_d_n12;

        let (assign19780_e18825, assign19780_e18825_d_n2, assign19780_e18825_d_n4, assign19780_e18825_d_n7, assign19780_e18825_d_n11, assign19780_e18825_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsqd, locals.var_fn241_calc_iq__qsqd_dn2, locals.var_fn241_calc_iq__qsqd_dn4, locals.var_fn241_calc_iq__qsqd_dn7, locals.var_fn241_calc_iq__qsqd_dn11, locals.var_fn241_calc_iq__qsqd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsqd = assign19780_e18825;
        locals.var_fn241_calc_iq__qsqd_dn2 = assign19780_e18825_d_n2;
        locals.var_fn241_calc_iq__qsqd_dn4 = assign19780_e18825_d_n4;
        locals.var_fn241_calc_iq__qsqd_dn7 = assign19780_e18825_d_n7;
        locals.var_fn241_calc_iq__qsqd_dn11 = assign19780_e18825_d_n11;
        locals.var_fn241_calc_iq__qsqd_dn12 = assign19780_e18825_d_n12;

        let (assign19790_e18829, assign19790_e18829_d_n2, assign19790_e18829_d_n4, assign19790_e18829_d_n7, assign19790_e18829_d_n11, assign19790_e18829_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvdd, locals.var_fn241_calc_iq__qinvdd_dn2, locals.var_fn241_calc_iq__qinvdd_dn4, locals.var_fn241_calc_iq__qinvdd_dn7, locals.var_fn241_calc_iq__qinvdd_dn11, locals.var_fn241_calc_iq__qinvdd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvdd = assign19790_e18829;
        locals.var_fn241_calc_iq__qinvdd_dn2 = assign19790_e18829_d_n2;
        locals.var_fn241_calc_iq__qinvdd_dn4 = assign19790_e18829_d_n4;
        locals.var_fn241_calc_iq__qinvdd_dn7 = assign19790_e18829_d_n7;
        locals.var_fn241_calc_iq__qinvdd_dn11 = assign19790_e18829_d_n11;
        locals.var_fn241_calc_iq__qinvdd_dn12 = assign19790_e18829_d_n12;

        let (assign19800_e18833, assign19800_e18833_d_n2, assign19800_e18833_d_n4, assign19800_e18833_d_n7, assign19800_e18833_d_n11, assign19800_e18833_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd1 = assign19800_e18833;
        locals.var_fn241_calc_iq__qd1_dn2 = assign19800_e18833_d_n2;
        locals.var_fn241_calc_iq__qd1_dn4 = assign19800_e18833_d_n4;
        locals.var_fn241_calc_iq__qd1_dn7 = assign19800_e18833_d_n7;
        locals.var_fn241_calc_iq__qd1_dn11 = assign19800_e18833_d_n11;
        locals.var_fn241_calc_iq__qd1_dn12 = assign19800_e18833_d_n12;

        let (assign19810_e18837, assign19810_e18837_d_n2, assign19810_e18837_d_n4, assign19810_e18837_d_n7, assign19810_e18837_d_n11, assign19810_e18837_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs, locals.var_fn241_calc_iq__qs_dn2, locals.var_fn241_calc_iq__qs_dn4, locals.var_fn241_calc_iq__qs_dn7, locals.var_fn241_calc_iq__qs_dn11, locals.var_fn241_calc_iq__qs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs = assign19810_e18837;
        locals.var_fn241_calc_iq__qs_dn2 = assign19810_e18837_d_n2;
        locals.var_fn241_calc_iq__qs_dn4 = assign19810_e18837_d_n4;
        locals.var_fn241_calc_iq__qs_dn7 = assign19810_e18837_d_n7;
        locals.var_fn241_calc_iq__qs_dn11 = assign19810_e18837_d_n11;
        locals.var_fn241_calc_iq__qs_dn12 = assign19810_e18837_d_n12;

        let (assign19820_e18841, assign19820_e18841_d_n2, assign19820_e18841_d_n4, assign19820_e18841_d_n7, assign19820_e18841_d_n11, assign19820_e18841_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd, locals.var_fn241_calc_iq__qd_dn2, locals.var_fn241_calc_iq__qd_dn4, locals.var_fn241_calc_iq__qd_dn7, locals.var_fn241_calc_iq__qd_dn11, locals.var_fn241_calc_iq__qd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd = assign19820_e18841;
        locals.var_fn241_calc_iq__qd_dn2 = assign19820_e18841_d_n2;
        locals.var_fn241_calc_iq__qd_dn4 = assign19820_e18841_d_n4;
        locals.var_fn241_calc_iq__qd_dn7 = assign19820_e18841_d_n7;
        locals.var_fn241_calc_iq__qd_dn11 = assign19820_e18841_d_n11;
        locals.var_fn241_calc_iq__qd_dn12 = assign19820_e18841_d_n12;

        let (assign19830_e18845, assign19830_e18845_d_n2, assign19830_e18845_d_n4, assign19830_e18845_d_n7, assign19830_e18845_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, locals.var_fn241_calc_iq__etac_dn12,)
    }
};
        locals.var_fn241_calc_iq__etac = assign19830_e18845;
        locals.var_fn241_calc_iq__etac_dn2 = assign19830_e18845_d_n2;
        locals.var_fn241_calc_iq__etac_dn4 = assign19830_e18845_d_n4;
        locals.var_fn241_calc_iq__etac_dn7 = assign19830_e18845_d_n7;
        locals.var_fn241_calc_iq__etac_dn12 = assign19830_e18845_d_n12;

        let (assign19840_e18849, assign19840_e18849_d_n3, assign19840_e18849_d_n4, assign19840_e18849_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etab, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, locals.var_fn241_calc_iq__etab_dn12,)
    }
};
        locals.var_fn241_calc_iq__etab = assign19840_e18849;
        locals.var_fn241_calc_iq__etab_dn3 = assign19840_e18849_d_n3;
        locals.var_fn241_calc_iq__etab_dn4 = assign19840_e18849_d_n4;
        locals.var_fn241_calc_iq__etab_dn12 = assign19840_e18849_d_n12;

        let (assign19850_e18853, assign19850_e18853_d_n2, assign19850_e18853_d_n4, assign19850_e18853_d_n7, assign19850_e18853_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, locals.var_fn241_calc_iq__etags_dn12,)
    }
};
        locals.var_fn241_calc_iq__etags = assign19850_e18853;
        locals.var_fn241_calc_iq__etags_dn2 = assign19850_e18853_d_n2;
        locals.var_fn241_calc_iq__etags_dn4 = assign19850_e18853_d_n4;
        locals.var_fn241_calc_iq__etags_dn7 = assign19850_e18853_d_n7;
        locals.var_fn241_calc_iq__etags_dn12 = assign19850_e18853_d_n12;

        let (assign19860_e18857, assign19860_e18857_d_n2, assign19860_e18857_d_n3, assign19860_e18857_d_n4, assign19860_e18857_d_n7, assign19860_e18857_d_n11, assign19860_e18857_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign19860_e18857;
        locals.var_fn241_calc_iq__exparg_dn2 = assign19860_e18857_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign19860_e18857_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign19860_e18857_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign19860_e18857_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign19860_e18857_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign19860_e18857_d_n12;

        let (assign19870_e18861, assign19870_e18861_d_n2, assign19870_e18861_d_n3, assign19870_e18861_d_n4, assign19870_e18861_d_n7, assign19870_e18861_d_n11, assign19870_e18861_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign19870_e18861;
        locals.var_fn241_calc_iq__myarg_dn2 = assign19870_e18861_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign19870_e18861_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign19870_e18861_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign19870_e18861_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign19870_e18861_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign19870_e18861_d_n12;

        let (assign19880_e18865, assign19880_e18865_d_n11, assign19880_e18865_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__absvdsin, locals.var_fn241_calc_iq__absvdsin_dn11, locals.var_fn241_calc_iq__absvdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__absvdsin = assign19880_e18865;
        locals.var_fn241_calc_iq__absvdsin_dn11 = assign19880_e18865_d_n11;
        locals.var_fn241_calc_iq__absvdsin_dn12 = assign19880_e18865_d_n12;

        let (assign19890_e18869, assign19890_e18869_d_n2, assign19890_e18869_d_n7, assign19890_e18869_d_n11, assign19890_e18869_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vgdin, locals.var_fn241_calc_iq__vgdin_dn2, locals.var_fn241_calc_iq__vgdin_dn7, locals.var_fn241_calc_iq__vgdin_dn11, locals.var_fn241_calc_iq__vgdin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgdin = assign19890_e18869;
        locals.var_fn241_calc_iq__vgdin_dn2 = assign19890_e18869_d_n2;
        locals.var_fn241_calc_iq__vgdin_dn7 = assign19890_e18869_d_n7;
        locals.var_fn241_calc_iq__vgdin_dn11 = assign19890_e18869_d_n11;
        locals.var_fn241_calc_iq__vgdin_dn12 = assign19890_e18869_d_n12;

        let (assign19900_e18873, assign19900_e18873_d_n2, assign19900_e18873_d_n4, assign19900_e18873_d_n7, assign19900_e18873_d_n11, assign19900_e18873_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign19900_e18873;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign19900_e18873_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign19900_e18873_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign19900_e18873_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign19900_e18873_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign19900_e18873_d_n12;

        let (assign19910_e18877, assign19910_e18877_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__myarg0, locals.var_fn241_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn241_calc_iq__myarg0 = assign19910_e18877;
        locals.var_fn241_calc_iq__myarg0_dn4 = assign19910_e18877_d_n4;

    }

    pub(super) fn stamp_transient_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19920_e18904, assign19920_e18904_d_n11, assign19920_e18904_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign19920_e18902, assign19920_e18902_d_n11, assign19920_e18902_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign19920_e18886: f64 = (0.001 / p.p53);
                let assign19920_e18888: f64 = (assign19920_e18886 * locals.var_fn241_calc_iq__vdsin);
                let assign19920_e18889: f64 = (assign19920_e18888).tanh();
                let assign19920_e18890: f64 = (locals.var_fn241_calc_iq__vdsin * assign19920_e18889);
                (assign19920_e18890, ((locals.var_fn241_calc_iq__vdsin_dn11 * assign19920_e18889) + (locals.var_fn241_calc_iq__vdsin * ((assign19920_e18886 * locals.var_fn241_calc_iq__vdsin_dn11) / ((assign19920_e18888).cosh() * (assign19920_e18888).cosh())))), ((locals.var_fn241_calc_iq__vdsin_dn12 * assign19920_e18889) + (locals.var_fn241_calc_iq__vdsin * ((assign19920_e18886 * locals.var_fn241_calc_iq__vdsin_dn12) / ((assign19920_e18888).cosh() * (assign19920_e18888).cosh())))),)
            } else {
                let (assign19920_e18901, assign19920_e18901_d_n11, assign19920_e18901_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign19920_e18896: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin);
                        let assign19920_e18898: f64 = (assign19920_e18896 + p.p53);
                        let assign19920_e18899: f64 = (assign19920_e18898).sqrt();
                        (assign19920_e18899, (((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsin) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin_dn11)) / (2.0 * assign19920_e18899)), (((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsin) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin_dn12)) / (2.0 * assign19920_e18899)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign19920_e18901, assign19920_e18901_d_n11, assign19920_e18901_d_n12,)
            }
        };
        (assign19920_e18902, assign19920_e18902_d_n11, assign19920_e18902_d_n12,)
    } else {
        (locals.var_fn241_calc_iq__absvdsin, locals.var_fn241_calc_iq__absvdsin_dn11, locals.var_fn241_calc_iq__absvdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__absvdsin = assign19920_e18904;
        locals.var_fn241_calc_iq__absvdsin_dn11 = assign19920_e18904_d_n11;
        locals.var_fn241_calc_iq__absvdsin_dn12 = assign19920_e18904_d_n12;

        let (assign19930_e18910, assign19930_e18910_d_n2, assign19930_e18910_d_n7, assign19930_e18910_d_n11, assign19930_e18910_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19930_e18908: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdsin);
        (assign19930_e18908, locals.var_fn241_calc_iq__vgsin_dn2, locals.var_fn241_calc_iq__vgsin_dn7, (-locals.var_fn241_calc_iq__vdsin_dn11), (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdsin_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vgdin, locals.var_fn241_calc_iq__vgdin_dn2, locals.var_fn241_calc_iq__vgdin_dn7, locals.var_fn241_calc_iq__vgdin_dn11, locals.var_fn241_calc_iq__vgdin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgdin = assign19930_e18910;
        locals.var_fn241_calc_iq__vgdin_dn2 = assign19930_e18910_d_n2;
        locals.var_fn241_calc_iq__vgdin_dn7 = assign19930_e18910_d_n7;
        locals.var_fn241_calc_iq__vgdin_dn11 = assign19930_e18910_d_n11;
        locals.var_fn241_calc_iq__vgdin_dn12 = assign19930_e18910_d_n12;

        let (assign19940_e18916, assign19940_e18916_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19940_e18914: f64 = (locals.var_fn241_calc_iq__alpha * locals.var_fn241_calc_iq__phitin);
        (assign19940_e18914, (locals.var_fn241_calc_iq__alpha * locals.var_fn241_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn241_calc_iq__alpha_phit, locals.var_fn241_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn241_calc_iq__alpha_phit = assign19940_e18916;
        locals.var_fn241_calc_iq__alpha_phit_dn4 = assign19940_e18916_d_n4;

        let (assign19950_e18928, assign19950_e18928_d_n4, assign19950_e18928_d_n11, assign19950_e18928_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19950_e18921: f64 = (2.302585092994046 * locals.var_fn241_calc_iq__phitin);
        let assign19950_e18922: f64 = (locals.var_fn241_calc_iq__ss / assign19950_e18921);
        let assign19950_e18925: f64 = (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin);
        let assign19950_e18926: f64 = (assign19950_e18922 + assign19950_e18925);
        (assign19950_e18926, (-((locals.var_fn241_calc_iq__ss * (2.302585092994046 * locals.var_fn241_calc_iq__phitin_dn4)) / (assign19950_e18921 * assign19950_e18921))), (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin_dn11), (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin_dn12),)
    } else {
        (locals.var_fn241_calc_iq__n, locals.var_fn241_calc_iq__n_dn4, locals.var_fn241_calc_iq__n_dn11, locals.var_fn241_calc_iq__n_dn12,)
    }
};
        locals.var_fn241_calc_iq__n = assign19950_e18928;
        locals.var_fn241_calc_iq__n_dn4 = assign19950_e18928_d_n4;
        locals.var_fn241_calc_iq__n_dn11 = assign19950_e18928_d_n11;
        locals.var_fn241_calc_iq__n_dn12 = assign19950_e18928_d_n12;

        let (assign19960_e18938, assign19960_e18938_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19960_e18934: f64 = (locals.var_fn241_calc_iq__tambin - locals.var_fn241_calc_iq__tnomin);
        let assign19960_e18935: f64 = (locals.var_fn241_calc_iq__vtzeta * assign19960_e18934);
        let assign19960_e18936: f64 = (locals.var_fn241_calc_iq__vto + assign19960_e18935);
        (assign19960_e18936, (locals.var_fn241_calc_iq__vtzeta * locals.var_fn241_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn241_calc_iq__vtof, locals.var_fn241_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn241_calc_iq__vtof = assign19960_e18938;
        locals.var_fn241_calc_iq__vtof_dn4 = assign19960_e18938_d_n4;

        let (assign19970_e18946, assign19970_e18946_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19970_e18942: f64 = (locals.var_fn241_calc_iq__tambin / locals.var_fn241_calc_iq__tnomin);
        let assign19970_e18944: f64 = (assign19970_e18942).powf(locals.var_fn241_calc_iq__epsilon);
        (assign19970_e18944, if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn241_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__epsilon * ((assign19970_e18942).powf(locals.var_fn241_calc_iq__epsilon - 1.0) * (locals.var_fn241_calc_iq__tambin_dn4 / locals.var_fn241_calc_iq__tnomin))) } } else { (assign19970_e18944 * (locals.var_fn241_calc_iq__epsilon * ((locals.var_fn241_calc_iq__tambin_dn4 / locals.var_fn241_calc_iq__tnomin) / assign19970_e18942))) },)
    } else {
        (locals.var_fn241_calc_iq__tfacmobin, locals.var_fn241_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tfacmobin = assign19970_e18946;
        locals.var_fn241_calc_iq__tfacmobin_dn4 = assign19970_e18946_d_n4;

        let assign19980_e18949: f64 = if locals.var_fn241_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign19980_e18949;

        let (assign19990_e18967, assign19990_e18967_d_n11, assign19990_e18967_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard242 != 0.0)) {
        let assign19990_e18957: f64 = (locals.var_fn241_calc_iq__absvdsin / locals.var_fn241_calc_iq__dibsat);
        let assign19990_e18959: f64 = (assign19990_e18957).powf(locals.var_fn241_calc_iq__beta);
        let assign19990_e18960: f64 = (1.0 + assign19990_e18959);
        let assign19990_e18963: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign19990_e18964: f64 = (assign19990_e18960).powf(assign19990_e18963);
        let assign19990_e18965: f64 = (locals.var_fn241_calc_iq__absvdsin / assign19990_e18964);
        (assign19990_e18965, (((locals.var_fn241_calc_iq__absvdsin_dn11 * assign19990_e18964) - (locals.var_fn241_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign19990_e18963) as f64).is_finite() && ((assign19990_e18963) as f64).fract() == 0.0 { if assign19990_e18963 == 0.0 { 0.0 } else { (assign19990_e18963 * ((assign19990_e18960).powf(assign19990_e18963 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) })) } } else { (assign19990_e18964 * (assign19990_e18963 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) } / assign19990_e18960))) })) / (assign19990_e18964 * assign19990_e18964)), (((locals.var_fn241_calc_iq__absvdsin_dn12 * assign19990_e18964) - (locals.var_fn241_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign19990_e18963) as f64).is_finite() && ((assign19990_e18963) as f64).fract() == 0.0 { if assign19990_e18963 == 0.0 { 0.0 } else { (assign19990_e18963 * ((assign19990_e18960).powf(assign19990_e18963 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) })) } } else { (assign19990_e18964 * (assign19990_e18963 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) } / assign19990_e18960))) })) / (assign19990_e18964 * assign19990_e18964)),)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign19990_e18967;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign19990_e18967_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign19990_e18967_d_n12;

        let (assign20000_e18974, assign20000_e18974_d_n11, assign20000_e18974_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard242 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign20000_e18974;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign20000_e18974_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign20000_e18974_d_n12;

        let (assign20010_e18984, assign20010_e18984_d_n11, assign20010_e18984_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20010_e18979: f64 = (locals.var_fn241_calc_iq__vsatdibl * locals.var_fn241_calc_iq__delta2);
        let assign20010_e18980: f64 = (locals.var_fn241_calc_iq__delta1 - assign20010_e18979);
        let assign20010_e18982: f64 = (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin);
        (assign20010_e18982, (((-(locals.var_fn241_calc_iq__vsatdibl_dn11 * locals.var_fn241_calc_iq__delta2)) * locals.var_fn241_calc_iq__absvdsin) + (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin_dn11)), (((-(locals.var_fn241_calc_iq__vsatdibl_dn12 * locals.var_fn241_calc_iq__delta2)) * locals.var_fn241_calc_iq__absvdsin) + (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__delta, locals.var_fn241_calc_iq__delta_dn11, locals.var_fn241_calc_iq__delta_dn12,)
    }
};
        locals.var_fn241_calc_iq__delta = assign20010_e18984;
        locals.var_fn241_calc_iq__delta_dn11 = assign20010_e18984_d_n11;
        locals.var_fn241_calc_iq__delta_dn12 = assign20010_e18984_d_n12;

        let (assign20020_e18990, assign20020_e18990_d_n4, assign20020_e18990_d_n11, assign20020_e18990_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20020_e18988: f64 = (locals.var_fn241_calc_iq__vtof - locals.var_fn241_calc_iq__delta);
        (assign20020_e18988, locals.var_fn241_calc_iq__vtof_dn4, (-locals.var_fn241_calc_iq__delta_dn11), (-locals.var_fn241_calc_iq__delta_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vtdibl, locals.var_fn241_calc_iq__vtdibl_dn4, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vtdibl = assign20020_e18990;
        locals.var_fn241_calc_iq__vtdibl_dn4 = assign20020_e18990_d_n4;
        locals.var_fn241_calc_iq__vtdibl_dn11 = assign20020_e18990_d_n11;
        locals.var_fn241_calc_iq__vtdibl_dn12 = assign20020_e18990_d_n12;

        let (assign20030_e18998, assign20030_e18998_d_n4, assign20030_e18998_d_n11, assign20030_e18998_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20030_e18994: f64 = (2.0 * locals.var_fn241_calc_iq__n);
        let assign20030_e18996: f64 = (assign20030_e18994 * locals.var_fn241_calc_iq__phitin);
        (assign20030_e18996, (((2.0 * locals.var_fn241_calc_iq__n_dn4) * locals.var_fn241_calc_iq__phitin) + (assign20030_e18994 * locals.var_fn241_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn241_calc_iq__n_dn11) * locals.var_fn241_calc_iq__phitin), ((2.0 * locals.var_fn241_calc_iq__n_dn12) * locals.var_fn241_calc_iq__phitin),)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit, locals.var_fn241_calc_iq__two_n_phit_dn4, locals.var_fn241_calc_iq__two_n_phit_dn11, locals.var_fn241_calc_iq__two_n_phit_dn12,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit = assign20030_e18998;
        locals.var_fn241_calc_iq__two_n_phit_dn4 = assign20030_e18998_d_n4;
        locals.var_fn241_calc_iq__two_n_phit_dn11 = assign20030_e18998_d_n11;
        locals.var_fn241_calc_iq__two_n_phit_dn12 = assign20030_e18998_d_n12;

        let (assign20040_e19004, assign20040_e19004_d_n4, assign20040_e19004_d_n11, assign20040_e19004_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20040_e19002: f64 = (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit);
        (assign20040_e19002, ((locals.var_fn241_calc_iq__cgin_dn4 * locals.var_fn241_calc_iq__two_n_phit) + (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn4)), (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn11), (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qref, locals.var_fn241_calc_iq__qref_dn4, locals.var_fn241_calc_iq__qref_dn11, locals.var_fn241_calc_iq__qref_dn12,)
    }
};
        locals.var_fn241_calc_iq__qref = assign20040_e19004;
        locals.var_fn241_calc_iq__qref_dn4 = assign20040_e19004_d_n4;
        locals.var_fn241_calc_iq__qref_dn11 = assign20040_e19004_d_n11;
        locals.var_fn241_calc_iq__qref_dn12 = assign20040_e19004_d_n12;

        let (assign20050_e19014, assign20050_e19014_d_n2, assign20050_e19014_d_n3, assign20050_e19014_d_n4, assign20050_e19014_d_n7, assign20050_e19014_d_n11, assign20050_e19014_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20050_e19009: f64 = (p.p51 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20050_e19011: f64 = (assign20050_e19009 / 2.0);
        let assign20050_e19012: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20050_e19011);
        (assign20050_e19012, 0.0, 0.0, (locals.var_fn241_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn241_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign20050_e19014;
        locals.var_fn241_calc_iq__myarg_dn2 = assign20050_e19014_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign20050_e19014_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign20050_e19014_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign20050_e19014_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign20050_e19014_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign20050_e19014_d_n12;

        let (assign20060_e19065, assign20060_e19065_d_n2, assign20060_e19065_d_n3, assign20060_e19065_d_n4, assign20060_e19065_d_n7, assign20060_e19065_d_n11, assign20060_e19065_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20060_e19059, assign20060_e19059_d_n2, assign20060_e19059_d_n7, assign20060_e19059_d_n11, assign20060_e19059_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20060_e19023: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19026: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19029: f64 = (0.001 / p.p53);
                let assign20060_e19032: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19033: f64 = (assign20060_e19029 * assign20060_e19032);
                let assign20060_e19034: f64 = (assign20060_e19033).tanh();
                let assign20060_e19035: f64 = (assign20060_e19026 * assign20060_e19034);
                let assign20060_e19036: f64 = (assign20060_e19023 + assign20060_e19035);
                let assign20060_e19037: f64 = (0.5 * assign20060_e19036);
                (assign20060_e19037, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))),)
            } else {
                let (assign20060_e19058, assign20060_e19058_d_n2, assign20060_e19058_d_n7, assign20060_e19058_d_n11, assign20060_e19058_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20060_e19044: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19047: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19050: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19051: f64 = (assign20060_e19047 * assign20060_e19050);
                        let assign20060_e19053: f64 = (assign20060_e19051 + p.p53);
                        let assign20060_e19054: f64 = (assign20060_e19053).sqrt();
                        let assign20060_e19055: f64 = (assign20060_e19044 + assign20060_e19054);
                        let assign20060_e19056: f64 = (0.5 * assign20060_e19055);
                        (assign20060_e19056, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20060_e19054)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20060_e19054)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20060_e19050) + (assign20060_e19047 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20060_e19054)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20060_e19054)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20060_e19058, assign20060_e19058_d_n2, assign20060_e19058_d_n7, assign20060_e19058_d_n11, assign20060_e19058_d_n12,)
            }
        };
        let assign20060_e19061: f64 = (assign20060_e19059 - locals.var_fn241_calc_iq__myarg);
        let assign20060_e19063: f64 = (assign20060_e19061 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20060_e19063, ((assign20060_e19059_d_n2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20060_e19061 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((assign20060_e19059_d_n7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((assign20060_e19059_d_n11 - locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((assign20060_e19059_d_n12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20060_e19065;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20060_e19065_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20060_e19065_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20060_e19065_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20060_e19065_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20060_e19065_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20060_e19065_d_n12;

        let assign20070_e19068: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign20070_e19068;

        let (assign20080_e19074, assign20080_e19074_d_n2, assign20080_e19074_d_n3, assign20080_e19074_d_n4, assign20080_e19074_d_n7, assign20080_e19074_d_n11, assign20080_e19074_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20080_e19074;
        locals.var_fn241_calc_iq__ff_dn2 = assign20080_e19074_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20080_e19074_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20080_e19074_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20080_e19074_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20080_e19074_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20080_e19074_d_n12;

        let assign20090_e19077: f64 = (-50.0);
        let assign20090_e19078: f64 = if locals.var_fn241_calc_iq__exparg < assign20090_e19077 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign20090_e19078;

        let (assign20100_e19087, assign20100_e19087_d_n2, assign20100_e19087_d_n3, assign20100_e19087_d_n4, assign20100_e19087_d_n7, assign20100_e19087_d_n11, assign20100_e19087_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20100_e19087;
        locals.var_fn241_calc_iq__ff_dn2 = assign20100_e19087_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20100_e19087_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20100_e19087_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20100_e19087_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20100_e19087_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20100_e19087_d_n12;

        let (assign20110_e19102, assign20110_e19102_d_n2, assign20110_e19102_d_n3, assign20110_e19102_d_n4, assign20110_e19102_d_n7, assign20110_e19102_d_n11, assign20110_e19102_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 == 0.0)) {
        let assign20110_e19098: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20110_e19099: f64 = (1.0 + assign20110_e19098);
        let assign20110_e19100: f64 = (1.0 / assign20110_e19099);
        (assign20110_e19100, (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20110_e19099 * assign20110_e19099))),)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20110_e19102;
        locals.var_fn241_calc_iq__ff_dn2 = assign20110_e19102_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20110_e19102_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20110_e19102_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20110_e19102_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20110_e19102_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20110_e19102_d_n12;

        let (assign20120_e19161, assign20120_e19161_d_n2, assign20120_e19161_d_n3, assign20120_e19161_d_n4, assign20120_e19161_d_n7, assign20120_e19161_d_n11, assign20120_e19161_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20120_e19147, assign20120_e19147_d_n2, assign20120_e19147_d_n7, assign20120_e19147_d_n11, assign20120_e19147_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20120_e19111: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19114: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19117: f64 = (0.001 / p.p53);
                let assign20120_e19120: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19121: f64 = (assign20120_e19117 * assign20120_e19120);
                let assign20120_e19122: f64 = (assign20120_e19121).tanh();
                let assign20120_e19123: f64 = (assign20120_e19114 * assign20120_e19122);
                let assign20120_e19124: f64 = (assign20120_e19111 + assign20120_e19123);
                let assign20120_e19125: f64 = (0.5 * assign20120_e19124);
                (assign20120_e19125, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))),)
            } else {
                let (assign20120_e19146, assign20120_e19146_d_n2, assign20120_e19146_d_n7, assign20120_e19146_d_n11, assign20120_e19146_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20120_e19132: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19135: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19138: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19139: f64 = (assign20120_e19135 * assign20120_e19138);
                        let assign20120_e19141: f64 = (assign20120_e19139 + p.p53);
                        let assign20120_e19142: f64 = (assign20120_e19141).sqrt();
                        let assign20120_e19143: f64 = (assign20120_e19132 + assign20120_e19142);
                        let assign20120_e19144: f64 = (0.5 * assign20120_e19143);
                        (assign20120_e19144, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20120_e19142)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20120_e19142)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20120_e19138) + (assign20120_e19135 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20120_e19142)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20120_e19142)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20120_e19146, assign20120_e19146_d_n2, assign20120_e19146_d_n7, assign20120_e19146_d_n11, assign20120_e19146_d_n12,)
            }
        };
        let assign20120_e19151: f64 = (p.p51 * 0.1);
        let assign20120_e19153: f64 = (assign20120_e19151 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20120_e19155: f64 = (assign20120_e19153 * locals.var_fn241_calc_iq__ff);
        let assign20120_e19156: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20120_e19155);
        let assign20120_e19157: f64 = (assign20120_e19147 - assign20120_e19156);
        let assign20120_e19159: f64 = (assign20120_e19157 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20120_e19159, ((assign20120_e19147_d_n2 - (-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn2))) / locals.var_fn241_calc_iq__two_n_phit), ((-(-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn3))) / locals.var_fn241_calc_iq__two_n_phit), ((((-(locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20120_e19151 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ff) + (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), ((assign20120_e19147_d_n7 - (-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn7))) / locals.var_fn241_calc_iq__two_n_phit), ((((assign20120_e19147_d_n11 - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), ((((assign20120_e19147_d_n12 - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__eta, locals.var_fn241_calc_iq__eta_dn2, locals.var_fn241_calc_iq__eta_dn3, locals.var_fn241_calc_iq__eta_dn4, locals.var_fn241_calc_iq__eta_dn7, locals.var_fn241_calc_iq__eta_dn11, locals.var_fn241_calc_iq__eta_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta = assign20120_e19161;
        locals.var_fn241_calc_iq__eta_dn2 = assign20120_e19161_d_n2;
        locals.var_fn241_calc_iq__eta_dn3 = assign20120_e19161_d_n3;
        locals.var_fn241_calc_iq__eta_dn4 = assign20120_e19161_d_n4;
        locals.var_fn241_calc_iq__eta_dn7 = assign20120_e19161_d_n7;
        locals.var_fn241_calc_iq__eta_dn11 = assign20120_e19161_d_n11;
        locals.var_fn241_calc_iq__eta_dn12 = assign20120_e19161_d_n12;

        let assign20130_e19164: f64 = if locals.var_fn241_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign20130_e19164;

        let (assign20140_e19172, assign20140_e19172_d_n2, assign20140_e19172_d_n3, assign20140_e19172_d_n4, assign20140_e19172_d_n7, assign20140_e19172_d_n11, assign20140_e19172_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign20140_e19170: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta);
        (assign20140_e19170, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20140_e19172;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20140_e19172_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20140_e19172_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20140_e19172_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20140_e19172_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20140_e19172_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20140_e19172_d_n12;

        let assign20150_e19175: f64 = (-50.0);
        let assign20150_e19176: f64 = if locals.var_fn241_calc_iq__eta < assign20150_e19175 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign20150_e19176;

        let (assign20160_e19188, assign20160_e19188_d_n2, assign20160_e19188_d_n3, assign20160_e19188_d_n4, assign20160_e19188_d_n7, assign20160_e19188_d_n11, assign20160_e19188_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard245 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign20160_e19185: f64 = (locals.var_fn241_calc_iq__eta).exp();
        let assign20160_e19186: f64 = (locals.var_fn241_calc_iq__qref * assign20160_e19185);
        (assign20160_e19186, (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn2)), (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn4))), (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20160_e19188;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20160_e19188_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20160_e19188_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20160_e19188_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20160_e19188_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20160_e19188_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20160_e19188_d_n12;

        let (assign20170_e19204, assign20170_e19204_d_n2, assign20170_e19204_d_n3, assign20170_e19204_d_n4, assign20170_e19204_d_n7, assign20170_e19204_d_n11, assign20170_e19204_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard245 == 0.0)) && (locals.var_guard246 == 0.0)) {
        let assign20170_e19199: f64 = (locals.var_fn241_calc_iq__eta).exp();
        let assign20170_e19200: f64 = (1.0 + assign20170_e19199);
        let assign20170_e19201: f64 = (assign20170_e19200).ln();
        let assign20170_e19202: f64 = (locals.var_fn241_calc_iq__qref * assign20170_e19201);
        (assign20170_e19202, (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn2) / assign20170_e19200)), (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn3) / assign20170_e19200)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn4) / assign20170_e19200))), (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn7) / assign20170_e19200)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn11) / assign20170_e19200))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn12) / assign20170_e19200))),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20170_e19204;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20170_e19204_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20170_e19204_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20170_e19204_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20170_e19204_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20170_e19204_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20170_e19204_d_n12;

        let (assign20180_e19218, assign20180_e19218_d_n2, assign20180_e19218_d_n3, assign20180_e19218_d_n4, assign20180_e19218_d_n7, assign20180_e19218_d_n11, assign20180_e19218_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20180_e19211: f64 = (locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv);
        let assign20180_e19213: f64 = (assign20180_e19211 / locals.var_fn241_calc_iq__cgin);
        let assign20180_e19214: f64 = (1.0 + assign20180_e19213);
        let assign20180_e19215: f64 = (locals.var_fn241_calc_iq__tfacmobin * assign20180_e19214);
        let assign20180_e19216: f64 = (locals.var_fn241_calc_iq__mu0 / assign20180_e19215);
        (assign20180_e19216, (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * ((locals.var_fn241_calc_iq__tfacmobin_dn4 * assign20180_e19214) + (locals.var_fn241_calc_iq__tfacmobin * ((((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20180_e19211 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin))))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))),)
    } else {
        (locals.var_fn241_calc_iq__muf, locals.var_fn241_calc_iq__muf_dn2, locals.var_fn241_calc_iq__muf_dn3, locals.var_fn241_calc_iq__muf_dn4, locals.var_fn241_calc_iq__muf_dn7, locals.var_fn241_calc_iq__muf_dn11, locals.var_fn241_calc_iq__muf_dn12,)
    }
};
        locals.var_fn241_calc_iq__muf = assign20180_e19218;
        locals.var_fn241_calc_iq__muf_dn2 = assign20180_e19218_d_n2;
        locals.var_fn241_calc_iq__muf_dn3 = assign20180_e19218_d_n3;
        locals.var_fn241_calc_iq__muf_dn4 = assign20180_e19218_d_n4;
        locals.var_fn241_calc_iq__muf_dn7 = assign20180_e19218_d_n7;
        locals.var_fn241_calc_iq__muf_dn11 = assign20180_e19218_d_n11;
        locals.var_fn241_calc_iq__muf_dn12 = assign20180_e19218_d_n12;

        let (assign20190_e19250, assign20190_e19250_d_n2, assign20190_e19250_d_n3, assign20190_e19250_d_n4, assign20190_e19250_d_n7, assign20190_e19250_d_n11, assign20190_e19250_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20190_e19224: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tnomin);
        let assign20190_e19225: f64 = (1.0 + assign20190_e19224);
        let assign20190_e19229: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin);
        let assign20190_e19230: f64 = (1.0 + assign20190_e19229);
        let assign20190_e19231: f64 = (assign20190_e19225 / assign20190_e19230);
        let assign20190_e19232: f64 = (locals.var_fn241_calc_iq__vel0 * assign20190_e19231);
        let assign20190_e19236: f64 = (locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin);
        let assign20190_e19238: f64 = (assign20190_e19236 / locals.var_fn241_calc_iq__lin);
        let assign20190_e19239: f64 = (1.0 + assign20190_e19238);
        let assign20190_e19240: f64 = (assign20190_e19232 * assign20190_e19239);
        let assign20190_e19244: f64 = (locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv);
        let assign20190_e19246: f64 = (assign20190_e19244 / locals.var_fn241_calc_iq__cgin);
        let assign20190_e19247: f64 = (1.0 + assign20190_e19246);
        let assign20190_e19248: f64 = (assign20190_e19240 / assign20190_e19247);
        (assign20190_e19248, (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), (((((locals.var_fn241_calc_iq__vel0 * (-((assign20190_e19225 * (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin_dn4)) / (assign20190_e19230 * assign20190_e19230)))) * assign20190_e19239) * assign20190_e19247) - (assign20190_e19240 * ((((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20190_e19244 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)))) / (assign20190_e19247 * assign20190_e19247)), (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), ((((assign20190_e19232 * ((locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin_dn11) / locals.var_fn241_calc_iq__lin)) * assign20190_e19247) - (assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin))) / (assign20190_e19247 * assign20190_e19247)), ((((assign20190_e19232 * ((locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin_dn12) / locals.var_fn241_calc_iq__lin)) * assign20190_e19247) - (assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin))) / (assign20190_e19247 * assign20190_e19247)),)
    } else {
        (locals.var_fn241_calc_iq__vx, locals.var_fn241_calc_iq__vx_dn2, locals.var_fn241_calc_iq__vx_dn3, locals.var_fn241_calc_iq__vx_dn4, locals.var_fn241_calc_iq__vx_dn7, locals.var_fn241_calc_iq__vx_dn11, locals.var_fn241_calc_iq__vx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vx = assign20190_e19250;
        locals.var_fn241_calc_iq__vx_dn2 = assign20190_e19250_d_n2;
        locals.var_fn241_calc_iq__vx_dn3 = assign20190_e19250_d_n3;
        locals.var_fn241_calc_iq__vx_dn4 = assign20190_e19250_d_n4;
        locals.var_fn241_calc_iq__vx_dn7 = assign20190_e19250_d_n7;
        locals.var_fn241_calc_iq__vx_dn11 = assign20190_e19250_d_n11;
        locals.var_fn241_calc_iq__vx_dn12 = assign20190_e19250_d_n12;

        let (assign20200_e19268, assign20200_e19268_d_n2, assign20200_e19268_d_n3, assign20200_e19268_d_n4, assign20200_e19268_d_n7, assign20200_e19268_d_n11, assign20200_e19268_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20200_e19254: f64 = (2.0 * locals.var_fn241_calc_iq__ff);
        let assign20200_e19256: f64 = (assign20200_e19254 * locals.var_fn241_calc_iq__phitin);
        let assign20200_e19258: f64 = (assign20200_e19256 * locals.var_fn241_calc_iq__muf);
        let assign20200_e19260: f64 = (assign20200_e19258 / locals.var_fn241_calc_iq__lin);
        let assign20200_e19263: f64 = (1.0 - locals.var_fn241_calc_iq__ff);
        let assign20200_e19265: f64 = (assign20200_e19263 * locals.var_fn241_calc_iq__vx);
        let assign20200_e19266: f64 = (assign20200_e19260 + assign20200_e19265);
        (assign20200_e19266, ((((((2.0 * locals.var_fn241_calc_iq__ff_dn2) * locals.var_fn241_calc_iq__phitin) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn2)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn2) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn241_calc_iq__ff_dn3) * locals.var_fn241_calc_iq__phitin) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn3)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn3) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn241_calc_iq__ff_dn4) * locals.var_fn241_calc_iq__phitin) + (assign20200_e19254 * locals.var_fn241_calc_iq__phitin_dn4)) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn4)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn4) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn241_calc_iq__ff_dn7) * locals.var_fn241_calc_iq__phitin) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn7)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn7) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn241_calc_iq__ff_dn11) * locals.var_fn241_calc_iq__phitin) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn11)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn11) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn11))), ((((((2.0 * locals.var_fn241_calc_iq__ff_dn12) * locals.var_fn241_calc_iq__phitin) * locals.var_fn241_calc_iq__muf) + (assign20200_e19256 * locals.var_fn241_calc_iq__muf_dn12)) / locals.var_fn241_calc_iq__lin) + (((-locals.var_fn241_calc_iq__ff_dn12) * locals.var_fn241_calc_iq__vx) + (assign20200_e19263 * locals.var_fn241_calc_iq__vx_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__vxf, locals.var_fn241_calc_iq__vxf_dn2, locals.var_fn241_calc_iq__vxf_dn3, locals.var_fn241_calc_iq__vxf_dn4, locals.var_fn241_calc_iq__vxf_dn7, locals.var_fn241_calc_iq__vxf_dn11, locals.var_fn241_calc_iq__vxf_dn12,)
    }
};
        locals.var_fn241_calc_iq__vxf = assign20200_e19268;
        locals.var_fn241_calc_iq__vxf_dn2 = assign20200_e19268_d_n2;
        locals.var_fn241_calc_iq__vxf_dn3 = assign20200_e19268_d_n3;
        locals.var_fn241_calc_iq__vxf_dn4 = assign20200_e19268_d_n4;
        locals.var_fn241_calc_iq__vxf_dn7 = assign20200_e19268_d_n7;
        locals.var_fn241_calc_iq__vxf_dn11 = assign20200_e19268_d_n11;
        locals.var_fn241_calc_iq__vxf_dn12 = assign20200_e19268_d_n12;

        let (assign20210_e19276, assign20210_e19276_d_n2, assign20210_e19276_d_n3, assign20210_e19276_d_n4, assign20210_e19276_d_n7, assign20210_e19276_d_n11, assign20210_e19276_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20210_e19272: f64 = (locals.var_fn241_calc_iq__vx * locals.var_fn241_calc_iq__lin);
        let assign20210_e19274: f64 = (assign20210_e19272 / locals.var_fn241_calc_iq__muf);
        (assign20210_e19274, ((((locals.var_fn241_calc_iq__vx_dn2 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn2)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn3 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn3)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn4 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn4)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn7 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn7)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn11 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn11)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn12 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn12)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)),)
    } else {
        (locals.var_fn241_calc_iq__vdsats, locals.var_fn241_calc_iq__vdsats_dn2, locals.var_fn241_calc_iq__vdsats_dn3, locals.var_fn241_calc_iq__vdsats_dn4, locals.var_fn241_calc_iq__vdsats_dn7, locals.var_fn241_calc_iq__vdsats_dn11, locals.var_fn241_calc_iq__vdsats_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats = assign20210_e19276;
        locals.var_fn241_calc_iq__vdsats_dn2 = assign20210_e19276_d_n2;
        locals.var_fn241_calc_iq__vdsats_dn3 = assign20210_e19276_d_n3;
        locals.var_fn241_calc_iq__vdsats_dn4 = assign20210_e19276_d_n4;
        locals.var_fn241_calc_iq__vdsats_dn7 = assign20210_e19276_d_n7;
        locals.var_fn241_calc_iq__vdsats_dn11 = assign20210_e19276_d_n11;
        locals.var_fn241_calc_iq__vdsats_dn12 = assign20210_e19276_d_n12;

    }

    pub(super) fn stamp_transient_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20220_e19293, assign20220_e19293_d_n2, assign20220_e19293_d_n3, assign20220_e19293_d_n4, assign20220_e19293_d_n7, assign20220_e19293_d_n11, assign20220_e19293_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20220_e19282: f64 = (2.0 * locals.var_fn241_calc_iq__qinvv);
        let assign20220_e19284: f64 = (assign20220_e19282 / locals.var_fn241_calc_iq__cgin);
        let assign20220_e19286: f64 = (assign20220_e19284 / locals.var_fn241_calc_iq__vdsats);
        let assign20220_e19287: f64 = (1.0 + assign20220_e19286);
        let assign20220_e19288: f64 = (assign20220_e19287).sqrt();
        let assign20220_e19289: f64 = (locals.var_fn241_calc_iq__vdsats * assign20220_e19288);
        let assign20220_e19291: f64 = (assign20220_e19289 - locals.var_fn241_calc_iq__vdsats);
        (assign20220_e19291, (((locals.var_fn241_calc_iq__vdsats_dn2 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn2)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn2), (((locals.var_fn241_calc_iq__vdsats_dn3 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn3)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn3), (((locals.var_fn241_calc_iq__vdsats_dn4 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20220_e19282 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn4)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn4), (((locals.var_fn241_calc_iq__vdsats_dn7 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn7)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn7), (((locals.var_fn241_calc_iq__vdsats_dn11 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn11)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn11), (((locals.var_fn241_calc_iq__vdsats_dn12 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn12)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vdsats1, locals.var_fn241_calc_iq__vdsats1_dn2, locals.var_fn241_calc_iq__vdsats1_dn3, locals.var_fn241_calc_iq__vdsats1_dn4, locals.var_fn241_calc_iq__vdsats1_dn7, locals.var_fn241_calc_iq__vdsats1_dn11, locals.var_fn241_calc_iq__vdsats1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats1 = assign20220_e19293;
        locals.var_fn241_calc_iq__vdsats1_dn2 = assign20220_e19293_d_n2;
        locals.var_fn241_calc_iq__vdsats1_dn3 = assign20220_e19293_d_n3;
        locals.var_fn241_calc_iq__vdsats1_dn4 = assign20220_e19293_d_n4;
        locals.var_fn241_calc_iq__vdsats1_dn7 = assign20220_e19293_d_n7;
        locals.var_fn241_calc_iq__vdsats1_dn11 = assign20220_e19293_d_n11;
        locals.var_fn241_calc_iq__vdsats1_dn12 = assign20220_e19293_d_n12;

        let (assign20230_e19305, assign20230_e19305_d_n2, assign20230_e19305_d_n3, assign20230_e19305_d_n4, assign20230_e19305_d_n7, assign20230_e19305_d_n11, assign20230_e19305_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20230_e19298: f64 = (1.0 - locals.var_fn241_calc_iq__ff);
        let assign20230_e19299: f64 = (locals.var_fn241_calc_iq__vdsats * assign20230_e19298);
        let assign20230_e19302: f64 = (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff);
        let assign20230_e19303: f64 = (assign20230_e19299 + assign20230_e19302);
        (assign20230_e19303, (((locals.var_fn241_calc_iq__vdsats_dn2 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn2))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn2)), (((locals.var_fn241_calc_iq__vdsats_dn3 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn3))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn3)), (((locals.var_fn241_calc_iq__vdsats_dn4 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit_dn4 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn4))), (((locals.var_fn241_calc_iq__vdsats_dn7 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn7))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn7)), (((locals.var_fn241_calc_iq__vdsats_dn11 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn11))) + ((locals.var_fn241_calc_iq__two_n_phit_dn11 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn11))), (((locals.var_fn241_calc_iq__vdsats_dn12 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn12))) + ((locals.var_fn241_calc_iq__two_n_phit_dn12 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__vdsat, locals.var_fn241_calc_iq__vdsat_dn2, locals.var_fn241_calc_iq__vdsat_dn3, locals.var_fn241_calc_iq__vdsat_dn4, locals.var_fn241_calc_iq__vdsat_dn7, locals.var_fn241_calc_iq__vdsat_dn11, locals.var_fn241_calc_iq__vdsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat = assign20230_e19305;
        locals.var_fn241_calc_iq__vdsat_dn2 = assign20230_e19305_d_n2;
        locals.var_fn241_calc_iq__vdsat_dn3 = assign20230_e19305_d_n3;
        locals.var_fn241_calc_iq__vdsat_dn4 = assign20230_e19305_d_n4;
        locals.var_fn241_calc_iq__vdsat_dn7 = assign20230_e19305_d_n7;
        locals.var_fn241_calc_iq__vdsat_dn11 = assign20230_e19305_d_n11;
        locals.var_fn241_calc_iq__vdsat_dn12 = assign20230_e19305_d_n12;

        let (assign20240_e19317, assign20240_e19317_d_n2, assign20240_e19317_d_n3, assign20240_e19317_d_n4, assign20240_e19317_d_n7, assign20240_e19317_d_n11, assign20240_e19317_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20240_e19310: f64 = (1.0 - locals.var_fn241_calc_iq__ff);
        let assign20240_e19311: f64 = (locals.var_fn241_calc_iq__vdsats1 * assign20240_e19310);
        let assign20240_e19314: f64 = (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff);
        let assign20240_e19315: f64 = (assign20240_e19311 + assign20240_e19314);
        (assign20240_e19315, (((locals.var_fn241_calc_iq__vdsats1_dn2 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn2))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn2)), (((locals.var_fn241_calc_iq__vdsats1_dn3 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn3))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn3)), (((locals.var_fn241_calc_iq__vdsats1_dn4 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit_dn4 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn4))), (((locals.var_fn241_calc_iq__vdsats1_dn7 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn7))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn7)), (((locals.var_fn241_calc_iq__vdsats1_dn11 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn11))) + ((locals.var_fn241_calc_iq__two_n_phit_dn11 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn11))), (((locals.var_fn241_calc_iq__vdsats1_dn12 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn12))) + ((locals.var_fn241_calc_iq__two_n_phit_dn12 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__vdsat1, locals.var_fn241_calc_iq__vdsat1_dn2, locals.var_fn241_calc_iq__vdsat1_dn3, locals.var_fn241_calc_iq__vdsat1_dn4, locals.var_fn241_calc_iq__vdsat1_dn7, locals.var_fn241_calc_iq__vdsat1_dn11, locals.var_fn241_calc_iq__vdsat1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat1 = assign20240_e19317;
        locals.var_fn241_calc_iq__vdsat1_dn2 = assign20240_e19317_d_n2;
        locals.var_fn241_calc_iq__vdsat1_dn3 = assign20240_e19317_d_n3;
        locals.var_fn241_calc_iq__vdsat1_dn4 = assign20240_e19317_d_n4;
        locals.var_fn241_calc_iq__vdsat1_dn7 = assign20240_e19317_d_n7;
        locals.var_fn241_calc_iq__vdsat1_dn11 = assign20240_e19317_d_n11;
        locals.var_fn241_calc_iq__vdsat1_dn12 = assign20240_e19317_d_n12;

        let (assign20250_e19386, assign20250_e19386_d_n2, assign20250_e19386_d_n3, assign20250_e19386_d_n4, assign20250_e19386_d_n7, assign20250_e19386_d_n11, assign20250_e19386_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20250_e19376, assign20250_e19376_d_n2, assign20250_e19376_d_n3, assign20250_e19376_d_n4, assign20250_e19376_d_n7, assign20250_e19376_d_n11, assign20250_e19376_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20250_e19329: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19330: f64 = assign20250_e19329;
                let assign20250_e19334: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19335: f64 = (-assign20250_e19334);
                let assign20250_e19338: f64 = (0.001 / p.p53);
                let assign20250_e19342: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19343: f64 = (-assign20250_e19342);
                let assign20250_e19344: f64 = (assign20250_e19338 * assign20250_e19343);
                let assign20250_e19345: f64 = (assign20250_e19344).tanh();
                let assign20250_e19346: f64 = (assign20250_e19335 * assign20250_e19345);
                let assign20250_e19347: f64 = (assign20250_e19330 + assign20250_e19346);
                let assign20250_e19348: f64 = (0.5 * assign20250_e19347);
                (assign20250_e19348, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))),)
            } else {
                let (assign20250_e19375, assign20250_e19375_d_n2, assign20250_e19375_d_n3, assign20250_e19375_d_n4, assign20250_e19375_d_n7, assign20250_e19375_d_n11, assign20250_e19375_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20250_e19356: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19357: f64 = assign20250_e19356;
                        let assign20250_e19361: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19362: f64 = (-assign20250_e19361);
                        let assign20250_e19366: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19367: f64 = (-assign20250_e19366);
                        let assign20250_e19368: f64 = (assign20250_e19362 * assign20250_e19367);
                        let assign20250_e19370: f64 = (assign20250_e19368 + p.p53);
                        let assign20250_e19371: f64 = (assign20250_e19370).sqrt();
                        let assign20250_e19372: f64 = (assign20250_e19357 + assign20250_e19371);
                        let assign20250_e19373: f64 = (0.5 * assign20250_e19372);
                        (assign20250_e19373, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19367) + (assign20250_e19362 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20250_e19371)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19367) + (assign20250_e19362 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20250_e19371)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20250_e19375, assign20250_e19375_d_n2, assign20250_e19375_d_n3, assign20250_e19375_d_n4, assign20250_e19375_d_n7, assign20250_e19375_d_n11, assign20250_e19375_d_n12,)
            }
        };
        let assign20250_e19378: f64 = (assign20250_e19376).powf(locals.var_fn241_calc_iq__beta);
        let assign20250_e19379: f64 = (1.0 + assign20250_e19378);
        let assign20250_e19382: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20250_e19383: f64 = (assign20250_e19379).powf(assign20250_e19382);
        let assign20250_e19384: f64 = (1.0 / assign20250_e19383);
        (assign20250_e19384, (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n2)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n2 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n2)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n2 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n3)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n3 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n3)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n3 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n4)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n4 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n4)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n4 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n7)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n7 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n7)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n7 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n11)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n11 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n11)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n11 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n12)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n12 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n12)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n12 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))),)
    } else {
        (locals.var_fn241_calc_iq__fsd, locals.var_fn241_calc_iq__fsd_dn2, locals.var_fn241_calc_iq__fsd_dn3, locals.var_fn241_calc_iq__fsd_dn4, locals.var_fn241_calc_iq__fsd_dn7, locals.var_fn241_calc_iq__fsd_dn11, locals.var_fn241_calc_iq__fsd_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd = assign20250_e19386;
        locals.var_fn241_calc_iq__fsd_dn2 = assign20250_e19386_d_n2;
        locals.var_fn241_calc_iq__fsd_dn3 = assign20250_e19386_d_n3;
        locals.var_fn241_calc_iq__fsd_dn4 = assign20250_e19386_d_n4;
        locals.var_fn241_calc_iq__fsd_dn7 = assign20250_e19386_d_n7;
        locals.var_fn241_calc_iq__fsd_dn11 = assign20250_e19386_d_n11;
        locals.var_fn241_calc_iq__fsd_dn12 = assign20250_e19386_d_n12;

        let (assign20260_e19392, assign20260_e19392_d_n2, assign20260_e19392_d_n3, assign20260_e19392_d_n4, assign20260_e19392_d_n7, assign20260_e19392_d_n11, assign20260_e19392_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20260_e19390: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd);
        (assign20260_e19390, (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn2), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn3), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn4), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn7), ((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__fsd) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn11)), ((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__fsd) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdx, locals.var_fn241_calc_iq__vdx_dn2, locals.var_fn241_calc_iq__vdx_dn3, locals.var_fn241_calc_iq__vdx_dn4, locals.var_fn241_calc_iq__vdx_dn7, locals.var_fn241_calc_iq__vdx_dn11, locals.var_fn241_calc_iq__vdx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx = assign20260_e19392;
        locals.var_fn241_calc_iq__vdx_dn2 = assign20260_e19392_d_n2;
        locals.var_fn241_calc_iq__vdx_dn3 = assign20260_e19392_d_n3;
        locals.var_fn241_calc_iq__vdx_dn4 = assign20260_e19392_d_n4;
        locals.var_fn241_calc_iq__vdx_dn7 = assign20260_e19392_d_n7;
        locals.var_fn241_calc_iq__vdx_dn11 = assign20260_e19392_d_n11;
        locals.var_fn241_calc_iq__vdx_dn12 = assign20260_e19392_d_n12;

        let (assign20270_e19467, assign20270_e19467_d_n2, assign20270_e19467_d_n3, assign20270_e19467_d_n4, assign20270_e19467_d_n7, assign20270_e19467_d_n11, assign20270_e19467_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20270_e19457, assign20270_e19457_d_n2, assign20270_e19457_d_n3, assign20270_e19457_d_n4, assign20270_e19457_d_n7, assign20270_e19457_d_n11, assign20270_e19457_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20270_e19403: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19405: f64 = (assign20270_e19403 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19406: f64 = assign20270_e19405;
                let assign20270_e19409: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19411: f64 = (assign20270_e19409 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19412: f64 = (-assign20270_e19411);
                let assign20270_e19415: f64 = (0.001 / p.p53);
                let assign20270_e19418: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19420: f64 = (assign20270_e19418 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19421: f64 = (-assign20270_e19420);
                let assign20270_e19422: f64 = (assign20270_e19415 * assign20270_e19421);
                let assign20270_e19423: f64 = (assign20270_e19422).tanh();
                let assign20270_e19424: f64 = (assign20270_e19412 * assign20270_e19423);
                let assign20270_e19425: f64 = (assign20270_e19406 + assign20270_e19424);
                let assign20270_e19426: f64 = (0.5 * assign20270_e19425);
                (assign20270_e19426, (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))),)
            } else {
                let (assign20270_e19456, assign20270_e19456_d_n2, assign20270_e19456_d_n3, assign20270_e19456_d_n4, assign20270_e19456_d_n7, assign20270_e19456_d_n11, assign20270_e19456_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20270_e19433: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19435: f64 = (assign20270_e19433 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19436: f64 = assign20270_e19435;
                        let assign20270_e19439: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19441: f64 = (assign20270_e19439 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19442: f64 = (-assign20270_e19441);
                        let assign20270_e19445: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19447: f64 = (assign20270_e19445 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19448: f64 = (-assign20270_e19447);
                        let assign20270_e19449: f64 = (assign20270_e19442 * assign20270_e19448);
                        let assign20270_e19451: f64 = (assign20270_e19449 + p.p53);
                        let assign20270_e19452: f64 = (assign20270_e19451).sqrt();
                        let assign20270_e19453: f64 = (assign20270_e19436 + assign20270_e19452);
                        let assign20270_e19454: f64 = (0.5 * assign20270_e19453);
                        (assign20270_e19454, (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19448) + (assign20270_e19442 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20270_e19452)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19448) + (assign20270_e19442 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20270_e19452)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20270_e19456, assign20270_e19456_d_n2, assign20270_e19456_d_n3, assign20270_e19456_d_n4, assign20270_e19456_d_n7, assign20270_e19456_d_n11, assign20270_e19456_d_n12,)
            }
        };
        let assign20270_e19459: f64 = (assign20270_e19457).powf(locals.var_fn241_calc_iq__beta);
        let assign20270_e19460: f64 = (1.0 + assign20270_e19459);
        let assign20270_e19463: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20270_e19464: f64 = (assign20270_e19460).powf(assign20270_e19463);
        let assign20270_e19465: f64 = (1.0 / assign20270_e19464);
        (assign20270_e19465, (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n2)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n2 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n2)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n2 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n3)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n3 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n3)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n3 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n4)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n4 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n4)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n4 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n7)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n7 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n7)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n7 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n11)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n11 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n11)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n11 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n12)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n12 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n12)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n12 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))),)
    } else {
        (locals.var_fn241_calc_iq__fds, locals.var_fn241_calc_iq__fds_dn2, locals.var_fn241_calc_iq__fds_dn3, locals.var_fn241_calc_iq__fds_dn4, locals.var_fn241_calc_iq__fds_dn7, locals.var_fn241_calc_iq__fds_dn11, locals.var_fn241_calc_iq__fds_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds = assign20270_e19467;
        locals.var_fn241_calc_iq__fds_dn2 = assign20270_e19467_d_n2;
        locals.var_fn241_calc_iq__fds_dn3 = assign20270_e19467_d_n3;
        locals.var_fn241_calc_iq__fds_dn4 = assign20270_e19467_d_n4;
        locals.var_fn241_calc_iq__fds_dn7 = assign20270_e19467_d_n7;
        locals.var_fn241_calc_iq__fds_dn11 = assign20270_e19467_d_n11;
        locals.var_fn241_calc_iq__fds_dn12 = assign20270_e19467_d_n12;

        let (assign20280_e19474, assign20280_e19474_d_n2, assign20280_e19474_d_n3, assign20280_e19474_d_n4, assign20280_e19474_d_n7, assign20280_e19474_d_n11, assign20280_e19474_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20280_e19470: f64 = (-locals.var_fn241_calc_iq__vdsin);
        let assign20280_e19472: f64 = (assign20280_e19470 * locals.var_fn241_calc_iq__fds);
        (assign20280_e19472, (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn2), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn3), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn4), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn7), (((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__fds) + (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn11)), (((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__fds) + (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vsx, locals.var_fn241_calc_iq__vsx_dn2, locals.var_fn241_calc_iq__vsx_dn3, locals.var_fn241_calc_iq__vsx_dn4, locals.var_fn241_calc_iq__vsx_dn7, locals.var_fn241_calc_iq__vsx_dn11, locals.var_fn241_calc_iq__vsx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx = assign20280_e19474;
        locals.var_fn241_calc_iq__vsx_dn2 = assign20280_e19474_d_n2;
        locals.var_fn241_calc_iq__vsx_dn3 = assign20280_e19474_d_n3;
        locals.var_fn241_calc_iq__vsx_dn4 = assign20280_e19474_d_n4;
        locals.var_fn241_calc_iq__vsx_dn7 = assign20280_e19474_d_n7;
        locals.var_fn241_calc_iq__vsx_dn11 = assign20280_e19474_d_n11;
        locals.var_fn241_calc_iq__vsx_dn12 = assign20280_e19474_d_n12;

        let (assign20290_e19482, assign20290_e19482_d_n2, assign20290_e19482_d_n3, assign20290_e19482_d_n4, assign20290_e19482_d_n7, assign20290_e19482_d_n11, assign20290_e19482_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20290_e19478: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__myarg);
        let assign20290_e19480: f64 = (assign20290_e19478 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20290_e19480, ((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20290_e19478 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20290_e19482;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20290_e19482_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20290_e19482_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20290_e19482_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20290_e19482_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20290_e19482_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20290_e19482_d_n12;

        let assign20300_e19485: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign20300_e19485;

        let (assign20310_e19491, assign20310_e19491_d_n2, assign20310_e19491_d_n3, assign20310_e19491_d_n4, assign20310_e19491_d_n7, assign20310_e19491_d_n11, assign20310_e19491_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard247 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20310_e19491;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20310_e19491_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20310_e19491_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20310_e19491_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20310_e19491_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20310_e19491_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20310_e19491_d_n12;

        let assign20320_e19494: f64 = (-50.0);
        let assign20320_e19495: f64 = if locals.var_fn241_calc_iq__exparg < assign20320_e19494 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign20320_e19495;

        let (assign20330_e19504, assign20330_e19504_d_n2, assign20330_e19504_d_n3, assign20330_e19504_d_n4, assign20330_e19504_d_n7, assign20330_e19504_d_n11, assign20330_e19504_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard247 == 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20330_e19504;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20330_e19504_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20330_e19504_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20330_e19504_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20330_e19504_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20330_e19504_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20330_e19504_d_n12;

        let (assign20340_e19519, assign20340_e19519_d_n2, assign20340_e19519_d_n3, assign20340_e19519_d_n4, assign20340_e19519_d_n7, assign20340_e19519_d_n11, assign20340_e19519_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard247 == 0.0)) && (locals.var_guard248 == 0.0)) {
        let assign20340_e19515: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20340_e19516: f64 = (1.0 + assign20340_e19515);
        let assign20340_e19517: f64 = (1.0 / assign20340_e19516);
        (assign20340_e19517, (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20340_e19516 * assign20340_e19516))),)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20340_e19519;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20340_e19519_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20340_e19519_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20340_e19519_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20340_e19519_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20340_e19519_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20340_e19519_d_n12;

        let (assign20350_e19537, assign20350_e19537_d_n2, assign20350_e19537_d_n3, assign20350_e19537_d_n4, assign20350_e19537_d_n7, assign20350_e19537_d_n11, assign20350_e19537_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20350_e19523: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__vsx);
        let assign20350_e19527: f64 = (p.p51 * 0.1);
        let assign20350_e19529: f64 = (assign20350_e19527 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20350_e19531: f64 = (assign20350_e19529 * locals.var_fn241_calc_iq__ffs);
        let assign20350_e19532: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20350_e19531);
        let assign20350_e19533: f64 = (assign20350_e19523 - assign20350_e19532);
        let assign20350_e19535: f64 = (assign20350_e19533 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20350_e19535, (((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__vsx_dn2) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn2))) / locals.var_fn241_calc_iq__two_n_phit), (((-locals.var_fn241_calc_iq__vsx_dn3) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn3))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vsx_dn4) - (locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20350_e19527 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffs) + (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__vsx_dn7) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn7))) / locals.var_fn241_calc_iq__two_n_phit), (((((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__vsx_dn11) - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__vsx_dn12) - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__etas, locals.var_fn241_calc_iq__etas_dn2, locals.var_fn241_calc_iq__etas_dn3, locals.var_fn241_calc_iq__etas_dn4, locals.var_fn241_calc_iq__etas_dn7, locals.var_fn241_calc_iq__etas_dn11, locals.var_fn241_calc_iq__etas_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas = assign20350_e19537;
        locals.var_fn241_calc_iq__etas_dn2 = assign20350_e19537_d_n2;
        locals.var_fn241_calc_iq__etas_dn3 = assign20350_e19537_d_n3;
        locals.var_fn241_calc_iq__etas_dn4 = assign20350_e19537_d_n4;
        locals.var_fn241_calc_iq__etas_dn7 = assign20350_e19537_d_n7;
        locals.var_fn241_calc_iq__etas_dn11 = assign20350_e19537_d_n11;
        locals.var_fn241_calc_iq__etas_dn12 = assign20350_e19537_d_n12;

        let assign20360_e19540: f64 = if locals.var_fn241_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign20360_e19540;

        let (assign20370_e19548, assign20370_e19548_d_n2, assign20370_e19548_d_n3, assign20370_e19548_d_n4, assign20370_e19548_d_n7, assign20370_e19548_d_n11, assign20370_e19548_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard249 != 0.0)) {
        let assign20370_e19546: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas);
        (assign20370_e19546, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20370_e19548;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20370_e19548_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20370_e19548_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20370_e19548_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20370_e19548_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20370_e19548_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20370_e19548_d_n12;

        let assign20380_e19551: f64 = (-50.0);
        let assign20380_e19552: f64 = if locals.var_fn241_calc_iq__etas < assign20380_e19551 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign20380_e19552;

        let (assign20390_e19564, assign20390_e19564_d_n2, assign20390_e19564_d_n3, assign20390_e19564_d_n4, assign20390_e19564_d_n7, assign20390_e19564_d_n11, assign20390_e19564_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign20390_e19561: f64 = (locals.var_fn241_calc_iq__etas).exp();
        let assign20390_e19562: f64 = (locals.var_fn241_calc_iq__qref * assign20390_e19561);
        (assign20390_e19562, (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn2)), (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn4))), (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20390_e19564;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20390_e19564_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20390_e19564_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20390_e19564_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20390_e19564_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20390_e19564_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20390_e19564_d_n12;

        let (assign20400_e19580, assign20400_e19580_d_n2, assign20400_e19580_d_n3, assign20400_e19580_d_n4, assign20400_e19580_d_n7, assign20400_e19580_d_n11, assign20400_e19580_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign20400_e19575: f64 = (locals.var_fn241_calc_iq__etas).exp();
        let assign20400_e19576: f64 = (1.0 + assign20400_e19575);
        let assign20400_e19577: f64 = (assign20400_e19576).ln();
        let assign20400_e19578: f64 = (locals.var_fn241_calc_iq__qref * assign20400_e19577);
        (assign20400_e19578, (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn2) / assign20400_e19576)), (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn3) / assign20400_e19576)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn4) / assign20400_e19576))), (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn7) / assign20400_e19576)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn11) / assign20400_e19576))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn12) / assign20400_e19576))),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20400_e19580;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20400_e19580_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20400_e19580_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20400_e19580_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20400_e19580_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20400_e19580_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20400_e19580_d_n12;

        let (assign20410_e19588, assign20410_e19588_d_n2, assign20410_e19588_d_n3, assign20410_e19588_d_n4, assign20410_e19588_d_n7, assign20410_e19588_d_n11, assign20410_e19588_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20410_e19584: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__myarg);
        let assign20410_e19586: f64 = (assign20410_e19584 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20410_e19586, ((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20410_e19584 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20410_e19588;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20410_e19588_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20410_e19588_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20410_e19588_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20410_e19588_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20410_e19588_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20410_e19588_d_n12;

        let assign20420_e19591: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign20420_e19591;

        let (assign20430_e19597, assign20430_e19597_d_n2, assign20430_e19597_d_n3, assign20430_e19597_d_n4, assign20430_e19597_d_n7, assign20430_e19597_d_n11, assign20430_e19597_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard251 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20430_e19597;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20430_e19597_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20430_e19597_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20430_e19597_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20430_e19597_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20430_e19597_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20430_e19597_d_n12;

        let assign20440_e19600: f64 = (-50.0);
        let assign20440_e19601: f64 = if locals.var_fn241_calc_iq__exparg < assign20440_e19600 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign20440_e19601;

        let (assign20450_e19610, assign20450_e19610_d_n2, assign20450_e19610_d_n3, assign20450_e19610_d_n4, assign20450_e19610_d_n7, assign20450_e19610_d_n11, assign20450_e19610_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard251 == 0.0)) && (locals.var_guard252 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20450_e19610;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20450_e19610_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20450_e19610_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20450_e19610_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20450_e19610_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20450_e19610_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20450_e19610_d_n12;

        let (assign20460_e19625, assign20460_e19625_d_n2, assign20460_e19625_d_n3, assign20460_e19625_d_n4, assign20460_e19625_d_n7, assign20460_e19625_d_n11, assign20460_e19625_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard251 == 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign20460_e19621: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20460_e19622: f64 = (1.0 + assign20460_e19621);
        let assign20460_e19623: f64 = (1.0 / assign20460_e19622);
        (assign20460_e19623, (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20460_e19622 * assign20460_e19622))),)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20460_e19625;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20460_e19625_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20460_e19625_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20460_e19625_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20460_e19625_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20460_e19625_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20460_e19625_d_n12;

        let (assign20470_e19643, assign20470_e19643_d_n2, assign20470_e19643_d_n3, assign20470_e19643_d_n4, assign20470_e19643_d_n7, assign20470_e19643_d_n11, assign20470_e19643_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20470_e19629: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdx);
        let assign20470_e19633: f64 = (p.p51 * 0.1);
        let assign20470_e19635: f64 = (assign20470_e19633 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20470_e19637: f64 = (assign20470_e19635 * locals.var_fn241_calc_iq__ffd);
        let assign20470_e19638: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20470_e19637);
        let assign20470_e19639: f64 = (assign20470_e19629 - assign20470_e19638);
        let assign20470_e19641: f64 = (assign20470_e19639 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20470_e19641, (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vdx_dn2) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn2))) / locals.var_fn241_calc_iq__two_n_phit), (((-locals.var_fn241_calc_iq__vdx_dn3) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn3))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vdx_dn4) - (locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20470_e19633 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffd) + (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vdx_dn7) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn7))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vdx_dn11) - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdx_dn12) - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__etad, locals.var_fn241_calc_iq__etad_dn2, locals.var_fn241_calc_iq__etad_dn3, locals.var_fn241_calc_iq__etad_dn4, locals.var_fn241_calc_iq__etad_dn7, locals.var_fn241_calc_iq__etad_dn11, locals.var_fn241_calc_iq__etad_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad = assign20470_e19643;
        locals.var_fn241_calc_iq__etad_dn2 = assign20470_e19643_d_n2;
        locals.var_fn241_calc_iq__etad_dn3 = assign20470_e19643_d_n3;
        locals.var_fn241_calc_iq__etad_dn4 = assign20470_e19643_d_n4;
        locals.var_fn241_calc_iq__etad_dn7 = assign20470_e19643_d_n7;
        locals.var_fn241_calc_iq__etad_dn11 = assign20470_e19643_d_n11;
        locals.var_fn241_calc_iq__etad_dn12 = assign20470_e19643_d_n12;

        let assign20480_e19646: f64 = if locals.var_fn241_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign20480_e19646;

        let (assign20490_e19654, assign20490_e19654_d_n2, assign20490_e19654_d_n3, assign20490_e19654_d_n4, assign20490_e19654_d_n7, assign20490_e19654_d_n11, assign20490_e19654_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard253 != 0.0)) {
        let assign20490_e19652: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad);
        (assign20490_e19652, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20490_e19654;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20490_e19654_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20490_e19654_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20490_e19654_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20490_e19654_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20490_e19654_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20490_e19654_d_n12;

        let assign20500_e19657: f64 = (-50.0);
        let assign20500_e19658: f64 = if locals.var_fn241_calc_iq__etad < assign20500_e19657 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign20500_e19658;

        let (assign20510_e19670, assign20510_e19670_d_n2, assign20510_e19670_d_n3, assign20510_e19670_d_n4, assign20510_e19670_d_n7, assign20510_e19670_d_n11, assign20510_e19670_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 != 0.0)) {
        let assign20510_e19667: f64 = (locals.var_fn241_calc_iq__etad).exp();
        let assign20510_e19668: f64 = (locals.var_fn241_calc_iq__qref * assign20510_e19667);
        (assign20510_e19668, (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn2)), (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn4))), (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20510_e19670;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20510_e19670_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20510_e19670_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20510_e19670_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20510_e19670_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20510_e19670_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20510_e19670_d_n12;

    }

    pub(super) fn stamp_transient_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20520_e19686, assign20520_e19686_d_n2, assign20520_e19686_d_n3, assign20520_e19686_d_n4, assign20520_e19686_d_n7, assign20520_e19686_d_n11, assign20520_e19686_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
        let assign20520_e19681: f64 = (locals.var_fn241_calc_iq__etad).exp();
        let assign20520_e19682: f64 = (1.0 + assign20520_e19681);
        let assign20520_e19683: f64 = (assign20520_e19682).ln();
        let assign20520_e19684: f64 = (locals.var_fn241_calc_iq__qref * assign20520_e19683);
        (assign20520_e19684, (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn2) / assign20520_e19682)), (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn3) / assign20520_e19682)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn4) / assign20520_e19682))), (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn7) / assign20520_e19682)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn11) / assign20520_e19682))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn12) / assign20520_e19682))),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20520_e19686;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20520_e19686_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20520_e19686_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20520_e19686_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20520_e19686_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20520_e19686_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20520_e19686_d_n12;

        let (assign20530_e19694, assign20530_e19694_d_n2, assign20530_e19694_d_n3, assign20530_e19694_d_n4, assign20530_e19694_d_n7, assign20530_e19694_d_n11, assign20530_e19694_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20530_e19690: f64 = (locals.var_fn241_calc_iq__qinvs - locals.var_fn241_calc_iq__qinvd);
        let assign20530_e19692: f64 = (assign20530_e19690 / locals.var_fn241_calc_iq__cgin);
        (assign20530_e19692, ((locals.var_fn241_calc_iq__qinvs_dn2 - locals.var_fn241_calc_iq__qinvd_dn2) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn3 - locals.var_fn241_calc_iq__qinvd_dn3) / locals.var_fn241_calc_iq__cgin), ((((locals.var_fn241_calc_iq__qinvs_dn4 - locals.var_fn241_calc_iq__qinvd_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20530_e19690 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)), ((locals.var_fn241_calc_iq__qinvs_dn7 - locals.var_fn241_calc_iq__qinvd_dn7) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn11 - locals.var_fn241_calc_iq__qinvd_dn11) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn12 - locals.var_fn241_calc_iq__qinvd_dn12) / locals.var_fn241_calc_iq__cgin),)
    } else {
        (locals.var_fn241_calc_iq__vdsc, locals.var_fn241_calc_iq__vdsc_dn2, locals.var_fn241_calc_iq__vdsc_dn3, locals.var_fn241_calc_iq__vdsc_dn4, locals.var_fn241_calc_iq__vdsc_dn7, locals.var_fn241_calc_iq__vdsc_dn11, locals.var_fn241_calc_iq__vdsc_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsc = assign20530_e19694;
        locals.var_fn241_calc_iq__vdsc_dn2 = assign20530_e19694_d_n2;
        locals.var_fn241_calc_iq__vdsc_dn3 = assign20530_e19694_d_n3;
        locals.var_fn241_calc_iq__vdsc_dn4 = assign20530_e19694_d_n4;
        locals.var_fn241_calc_iq__vdsc_dn7 = assign20530_e19694_d_n7;
        locals.var_fn241_calc_iq__vdsc_dn11 = assign20530_e19694_d_n11;
        locals.var_fn241_calc_iq__vdsc_dn12 = assign20530_e19694_d_n12;

        let (assign20540_e19700, assign20540_e19700_d_n2, assign20540_e19700_d_n3, assign20540_e19700_d_n4, assign20540_e19700_d_n7, assign20540_e19700_d_n11, assign20540_e19700_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20540_e19698: f64 = (locals.var_fn241_calc_iq__vdsc / locals.var_fn241_calc_iq__vdsat);
        (assign20540_e19698, (((locals.var_fn241_calc_iq__vdsc_dn2 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn2)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn3 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn3)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn4 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn4)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn7 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn7)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn11 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn11)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn12 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn12)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)),)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign20540_e19700;
        locals.var_fn241_calc_iq__myarg_dn2 = assign20540_e19700_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign20540_e19700_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign20540_e19700_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign20540_e19700_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign20540_e19700_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign20540_e19700_d_n12;

        let (assign20550_e19737, assign20550_e19737_d_n2, assign20550_e19737_d_n3, assign20550_e19737_d_n4, assign20550_e19737_d_n7, assign20550_e19737_d_n11, assign20550_e19737_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20550_e19727, assign20550_e19727_d_n2, assign20550_e19727_d_n3, assign20550_e19727_d_n4, assign20550_e19727_d_n7, assign20550_e19727_d_n11, assign20550_e19727_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20550_e19711: f64 = (0.001 / p.p53);
                let assign20550_e19713: f64 = (assign20550_e19711 * locals.var_fn241_calc_iq__myarg);
                let assign20550_e19714: f64 = (assign20550_e19713).tanh();
                let assign20550_e19715: f64 = (locals.var_fn241_calc_iq__myarg * assign20550_e19714);
                (assign20550_e19715, ((locals.var_fn241_calc_iq__myarg_dn2 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn2) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))), ((locals.var_fn241_calc_iq__myarg_dn3 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn3) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))), ((locals.var_fn241_calc_iq__myarg_dn4 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn4) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))), ((locals.var_fn241_calc_iq__myarg_dn7 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn7) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))), ((locals.var_fn241_calc_iq__myarg_dn11 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn11) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))), ((locals.var_fn241_calc_iq__myarg_dn12 * assign20550_e19714) + (locals.var_fn241_calc_iq__myarg * ((assign20550_e19711 * locals.var_fn241_calc_iq__myarg_dn12) / ((assign20550_e19713).cosh() * (assign20550_e19713).cosh())))),)
            } else {
                let (assign20550_e19726, assign20550_e19726_d_n2, assign20550_e19726_d_n3, assign20550_e19726_d_n4, assign20550_e19726_d_n7, assign20550_e19726_d_n11, assign20550_e19726_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20550_e19721: f64 = (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg);
                        let assign20550_e19723: f64 = (assign20550_e19721 + p.p53);
                        let assign20550_e19724: f64 = (assign20550_e19723).sqrt();
                        (assign20550_e19724, (((locals.var_fn241_calc_iq__myarg_dn2 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn2)) / (2.0 * assign20550_e19724)), (((locals.var_fn241_calc_iq__myarg_dn3 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn3)) / (2.0 * assign20550_e19724)), (((locals.var_fn241_calc_iq__myarg_dn4 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn4)) / (2.0 * assign20550_e19724)), (((locals.var_fn241_calc_iq__myarg_dn7 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn7)) / (2.0 * assign20550_e19724)), (((locals.var_fn241_calc_iq__myarg_dn11 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn11)) / (2.0 * assign20550_e19724)), (((locals.var_fn241_calc_iq__myarg_dn12 * locals.var_fn241_calc_iq__myarg) + (locals.var_fn241_calc_iq__myarg * locals.var_fn241_calc_iq__myarg_dn12)) / (2.0 * assign20550_e19724)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20550_e19726, assign20550_e19726_d_n2, assign20550_e19726_d_n3, assign20550_e19726_d_n4, assign20550_e19726_d_n7, assign20550_e19726_d_n11, assign20550_e19726_d_n12,)
            }
        };
        let assign20550_e19729: f64 = (assign20550_e19727).powf(locals.var_fn241_calc_iq__beta);
        let assign20550_e19730: f64 = (1.0 + assign20550_e19729);
        let assign20550_e19733: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20550_e19734: f64 = (assign20550_e19730).powf(assign20550_e19733);
        let assign20550_e19735: f64 = (locals.var_fn241_calc_iq__myarg / assign20550_e19734);
        (assign20550_e19735, (((locals.var_fn241_calc_iq__myarg_dn2 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n2)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n2 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n2)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n2 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)), (((locals.var_fn241_calc_iq__myarg_dn3 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n3)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n3 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n3)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n3 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)), (((locals.var_fn241_calc_iq__myarg_dn4 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n4)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n4 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n4)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n4 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)), (((locals.var_fn241_calc_iq__myarg_dn7 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n7)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n7 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n7)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n7 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)), (((locals.var_fn241_calc_iq__myarg_dn11 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n11)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n11 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n11)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n11 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)), (((locals.var_fn241_calc_iq__myarg_dn12 * assign20550_e19734) - (locals.var_fn241_calc_iq__myarg * if 0.0 == 0.0 && ((assign20550_e19733) as f64).is_finite() && ((assign20550_e19733) as f64).fract() == 0.0 { if assign20550_e19733 == 0.0 { 0.0 } else { (assign20550_e19733 * ((assign20550_e19730).powf(assign20550_e19733 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n12)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n12 / assign20550_e19727))) })) } } else { (assign20550_e19734 * (assign20550_e19733 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20550_e19727).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20550_e19727_d_n12)) } } else { (assign20550_e19729 * (locals.var_fn241_calc_iq__beta * (assign20550_e19727_d_n12 / assign20550_e19727))) } / assign20550_e19730))) })) / (assign20550_e19734 * assign20550_e19734)),)
    } else {
        (locals.var_fn241_calc_iq__fsat, locals.var_fn241_calc_iq__fsat_dn2, locals.var_fn241_calc_iq__fsat_dn3, locals.var_fn241_calc_iq__fsat_dn4, locals.var_fn241_calc_iq__fsat_dn7, locals.var_fn241_calc_iq__fsat_dn11, locals.var_fn241_calc_iq__fsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsat = assign20550_e19737;
        locals.var_fn241_calc_iq__fsat_dn2 = assign20550_e19737_d_n2;
        locals.var_fn241_calc_iq__fsat_dn3 = assign20550_e19737_d_n3;
        locals.var_fn241_calc_iq__fsat_dn4 = assign20550_e19737_d_n4;
        locals.var_fn241_calc_iq__fsat_dn7 = assign20550_e19737_d_n7;
        locals.var_fn241_calc_iq__fsat_dn11 = assign20550_e19737_d_n11;
        locals.var_fn241_calc_iq__fsat_dn12 = assign20550_e19737_d_n12;

        let (assign20560_e19743, assign20560_e19743_d_n2, assign20560_e19743_d_n3, assign20560_e19743_d_n4, assign20560_e19743_d_n7, assign20560_e19743_d_n11, assign20560_e19743_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20560_e19741: f64 = (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat);
        (assign20560_e19741, ((locals.var_fn241_calc_iq__vxf_dn2 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn2)), ((locals.var_fn241_calc_iq__vxf_dn3 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn3)), ((locals.var_fn241_calc_iq__vxf_dn4 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn4)), ((locals.var_fn241_calc_iq__vxf_dn7 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn7)), ((locals.var_fn241_calc_iq__vxf_dn11 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn11)), ((locals.var_fn241_calc_iq__vxf_dn12 * locals.var_fn241_calc_iq__fsat) + (locals.var_fn241_calc_iq__vxf * locals.var_fn241_calc_iq__fsat_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vel, locals.var_fn241_calc_iq__vel_dn2, locals.var_fn241_calc_iq__vel_dn3, locals.var_fn241_calc_iq__vel_dn4, locals.var_fn241_calc_iq__vel_dn7, locals.var_fn241_calc_iq__vel_dn11, locals.var_fn241_calc_iq__vel_dn12,)
    }
};
        locals.var_fn241_calc_iq__vel = assign20560_e19743;
        locals.var_fn241_calc_iq__vel_dn2 = assign20560_e19743_d_n2;
        locals.var_fn241_calc_iq__vel_dn3 = assign20560_e19743_d_n3;
        locals.var_fn241_calc_iq__vel_dn4 = assign20560_e19743_d_n4;
        locals.var_fn241_calc_iq__vel_dn7 = assign20560_e19743_d_n7;
        locals.var_fn241_calc_iq__vel_dn11 = assign20560_e19743_d_n11;
        locals.var_fn241_calc_iq__vel_dn12 = assign20560_e19743_d_n12;

        let (assign20570_e19761, assign20570_e19761_d_n2, assign20570_e19761_d_n3, assign20570_e19761_d_n4, assign20570_e19761_d_n7, assign20570_e19761_d_n11, assign20570_e19761_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20570_e19747: f64 = (locals.var_fn241_calc_iq__type * locals.var_fn241_calc_iq__w);
        let assign20570_e19749: f64 = (assign20570_e19747 * locals.var_fn241_calc_iq__ngf);
        let assign20570_e19751: f64 = (assign20570_e19749 * 0.5);
        let assign20570_e19754: f64 = (locals.var_fn241_calc_iq__qinvs + locals.var_fn241_calc_iq__qinvd);
        let assign20570_e19755: f64 = (assign20570_e19751 * assign20570_e19754);
        let assign20570_e19757: f64 = (assign20570_e19755 * locals.var_fn241_calc_iq__vel);
        let assign20570_e19759: f64 = (assign20570_e19757 * locals.var_fn241_calc_iq__trapfracdl);
        (assign20570_e19759, ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn2 + locals.var_fn241_calc_iq__qinvd_dn2)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn2)) * locals.var_fn241_calc_iq__trapfracdl), ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn3 + locals.var_fn241_calc_iq__qinvd_dn3)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn3)) * locals.var_fn241_calc_iq__trapfracdl), ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn4 + locals.var_fn241_calc_iq__qinvd_dn4)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn7 + locals.var_fn241_calc_iq__qinvd_dn7)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn7)) * locals.var_fn241_calc_iq__trapfracdl), ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn11 + locals.var_fn241_calc_iq__qinvd_dn11)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn11)) * locals.var_fn241_calc_iq__trapfracdl), ((((assign20570_e19751 * (locals.var_fn241_calc_iq__qinvs_dn12 + locals.var_fn241_calc_iq__qinvd_dn12)) * locals.var_fn241_calc_iq__vel) + (assign20570_e19755 * locals.var_fn241_calc_iq__vel_dn12)) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__idsout, locals.var_fn241_calc_iq__idsout_dn2, locals.var_fn241_calc_iq__idsout_dn3, locals.var_fn241_calc_iq__idsout_dn4, locals.var_fn241_calc_iq__idsout_dn7, locals.var_fn241_calc_iq__idsout_dn11, locals.var_fn241_calc_iq__idsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__idsout = assign20570_e19761;
        locals.var_fn241_calc_iq__idsout_dn2 = assign20570_e19761_d_n2;
        locals.var_fn241_calc_iq__idsout_dn3 = assign20570_e19761_d_n3;
        locals.var_fn241_calc_iq__idsout_dn4 = assign20570_e19761_d_n4;
        locals.var_fn241_calc_iq__idsout_dn7 = assign20570_e19761_d_n7;
        locals.var_fn241_calc_iq__idsout_dn11 = assign20570_e19761_d_n11;
        locals.var_fn241_calc_iq__idsout_dn12 = assign20570_e19761_d_n12;

        let (assign20580_e19769, assign20580_e19769_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20580_e19766: f64 = (2.302585092994046 * locals.var_fn241_calc_iq__phitin);
        let assign20580_e19767: f64 = (locals.var_fn241_calc_iq__ss / assign20580_e19766);
        (assign20580_e19767, (-((locals.var_fn241_calc_iq__ss * (2.302585092994046 * locals.var_fn241_calc_iq__phitin_dn4)) / (assign20580_e19766 * assign20580_e19766))),)
    } else {
        (locals.var_fn241_calc_iq__n0, locals.var_fn241_calc_iq__n0_dn4,)
    }
};
        locals.var_fn241_calc_iq__n0 = assign20580_e19769;
        locals.var_fn241_calc_iq__n0_dn4 = assign20580_e19769_d_n4;

        let (assign20590_e19777, assign20590_e19777_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20590_e19773: f64 = (2.0 * locals.var_fn241_calc_iq__n0);
        let assign20590_e19775: f64 = (assign20590_e19773 * locals.var_fn241_calc_iq__phitin);
        (assign20590_e19775, (((2.0 * locals.var_fn241_calc_iq__n0_dn4) * locals.var_fn241_calc_iq__phitin) + (assign20590_e19773 * locals.var_fn241_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit0, locals.var_fn241_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit0 = assign20590_e19777;
        locals.var_fn241_calc_iq__two_n_phit0_dn4 = assign20590_e19777_d_n4;

        let (assign20600_e19783, assign20600_e19783_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20600_e19781: f64 = (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit0);
        (assign20600_e19781, ((locals.var_fn241_calc_iq__cgin_dn4 * locals.var_fn241_calc_iq__two_n_phit0) + (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn241_calc_iq__qref0, locals.var_fn241_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn241_calc_iq__qref0 = assign20600_e19783;
        locals.var_fn241_calc_iq__qref0_dn4 = assign20600_e19783_d_n4;

        let (assign20610_e19793, assign20610_e19793_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20610_e19788: f64 = (p.p51 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20610_e19790: f64 = (assign20610_e19788 / 2.0);
        let assign20610_e19791: f64 = (locals.var_fn241_calc_iq__vtof - assign20610_e19790);
        (assign20610_e19791, (locals.var_fn241_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn241_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn241_calc_iq__myarg0, locals.var_fn241_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn241_calc_iq__myarg0 = assign20610_e19793;
        locals.var_fn241_calc_iq__myarg0_dn4 = assign20610_e19793_d_n4;

        let (assign20620_e19844, assign20620_e19844_d_n2, assign20620_e19844_d_n4, assign20620_e19844_d_n7, assign20620_e19844_d_n11, assign20620_e19844_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20620_e19838, assign20620_e19838_d_n2, assign20620_e19838_d_n7, assign20620_e19838_d_n11, assign20620_e19838_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20620_e19802: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19805: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19808: f64 = (0.001 / p.p53);
                let assign20620_e19811: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19812: f64 = (assign20620_e19808 * assign20620_e19811);
                let assign20620_e19813: f64 = (assign20620_e19812).tanh();
                let assign20620_e19814: f64 = (assign20620_e19805 * assign20620_e19813);
                let assign20620_e19815: f64 = (assign20620_e19802 + assign20620_e19814);
                let assign20620_e19816: f64 = (0.5 * assign20620_e19815);
                (assign20620_e19816, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))),)
            } else {
                let (assign20620_e19837, assign20620_e19837_d_n2, assign20620_e19837_d_n7, assign20620_e19837_d_n11, assign20620_e19837_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20620_e19823: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19826: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19829: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19830: f64 = (assign20620_e19826 * assign20620_e19829);
                        let assign20620_e19832: f64 = (assign20620_e19830 + p.p53);
                        let assign20620_e19833: f64 = (assign20620_e19832).sqrt();
                        let assign20620_e19834: f64 = (assign20620_e19823 + assign20620_e19833);
                        let assign20620_e19835: f64 = (0.5 * assign20620_e19834);
                        (assign20620_e19835, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20620_e19833)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20620_e19833)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20620_e19829) + (assign20620_e19826 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20620_e19833)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20620_e19833)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20620_e19837, assign20620_e19837_d_n2, assign20620_e19837_d_n7, assign20620_e19837_d_n11, assign20620_e19837_d_n12,)
            }
        };
        let assign20620_e19840: f64 = (assign20620_e19838 - locals.var_fn241_calc_iq__myarg0);
        let assign20620_e19842: f64 = (assign20620_e19840 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20620_e19842, (assign20620_e19838_d_n2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20620_e19840 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (assign20620_e19838_d_n7 / locals.var_fn241_calc_iq__alpha_phit), (assign20620_e19838_d_n11 / locals.var_fn241_calc_iq__alpha_phit), (assign20620_e19838_d_n12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20620_e19844;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20620_e19844_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20620_e19844_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20620_e19844_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20620_e19844_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20620_e19844_d_n12;

        let assign20630_e19847: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign20630_e19847;

        let (assign20640_e19853, assign20640_e19853_d_n2, assign20640_e19853_d_n4, assign20640_e19853_d_n7, assign20640_e19853_d_n11, assign20640_e19853_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard255 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20640_e19853;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20640_e19853_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20640_e19853_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20640_e19853_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20640_e19853_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20640_e19853_d_n12;

        let assign20650_e19856: f64 = (-50.0);
        let assign20650_e19857: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20650_e19856 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign20650_e19857;

        let (assign20660_e19866, assign20660_e19866_d_n2, assign20660_e19866_d_n4, assign20660_e19866_d_n7, assign20660_e19866_d_n11, assign20660_e19866_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard255 == 0.0)) && (locals.var_guard256 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20660_e19866;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20660_e19866_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20660_e19866_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20660_e19866_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20660_e19866_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20660_e19866_d_n12;

        let (assign20670_e19881, assign20670_e19881_d_n2, assign20670_e19881_d_n4, assign20670_e19881_d_n7, assign20670_e19881_d_n11, assign20670_e19881_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard255 == 0.0)) && (locals.var_guard256 == 0.0)) {
        let assign20670_e19877: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign20670_e19878: f64 = (1.0 + assign20670_e19877);
        let assign20670_e19879: f64 = (1.0 / assign20670_e19878);
        (assign20670_e19879, (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign20670_e19878 * assign20670_e19878))),)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20670_e19881;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20670_e19881_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20670_e19881_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20670_e19881_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20670_e19881_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20670_e19881_d_n12;

        let (assign20680_e19940, assign20680_e19940_d_n2, assign20680_e19940_d_n4, assign20680_e19940_d_n7, assign20680_e19940_d_n11, assign20680_e19940_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20680_e19926, assign20680_e19926_d_n2, assign20680_e19926_d_n7, assign20680_e19926_d_n11, assign20680_e19926_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20680_e19890: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19893: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19896: f64 = (0.001 / p.p53);
                let assign20680_e19899: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19900: f64 = (assign20680_e19896 * assign20680_e19899);
                let assign20680_e19901: f64 = (assign20680_e19900).tanh();
                let assign20680_e19902: f64 = (assign20680_e19893 * assign20680_e19901);
                let assign20680_e19903: f64 = (assign20680_e19890 + assign20680_e19902);
                let assign20680_e19904: f64 = (0.5 * assign20680_e19903);
                (assign20680_e19904, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))),)
            } else {
                let (assign20680_e19925, assign20680_e19925_d_n2, assign20680_e19925_d_n7, assign20680_e19925_d_n11, assign20680_e19925_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20680_e19911: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19914: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19917: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19918: f64 = (assign20680_e19914 * assign20680_e19917);
                        let assign20680_e19920: f64 = (assign20680_e19918 + p.p53);
                        let assign20680_e19921: f64 = (assign20680_e19920).sqrt();
                        let assign20680_e19922: f64 = (assign20680_e19911 + assign20680_e19921);
                        let assign20680_e19923: f64 = (0.5 * assign20680_e19922);
                        (assign20680_e19923, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20680_e19921)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20680_e19921)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20680_e19917) + (assign20680_e19914 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20680_e19921)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20680_e19921)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20680_e19925, assign20680_e19925_d_n2, assign20680_e19925_d_n7, assign20680_e19925_d_n11, assign20680_e19925_d_n12,)
            }
        };
        let assign20680_e19930: f64 = (p.p51 * 0.1);
        let assign20680_e19932: f64 = (assign20680_e19930 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20680_e19934: f64 = (assign20680_e19932 * locals.var_fn241_calc_iq__ff0);
        let assign20680_e19935: f64 = (locals.var_fn241_calc_iq__vtof - assign20680_e19934);
        let assign20680_e19936: f64 = (assign20680_e19926 - assign20680_e19935);
        let assign20680_e19938: f64 = (assign20680_e19936 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign20680_e19938, ((assign20680_e19926_d_n2 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (((assign20680_e19930 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ff0) + (assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign20680_e19936 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), ((assign20680_e19926_d_n7 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), ((assign20680_e19926_d_n11 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), ((assign20680_e19926_d_n12 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__eta0, locals.var_fn241_calc_iq__eta0_dn2, locals.var_fn241_calc_iq__eta0_dn4, locals.var_fn241_calc_iq__eta0_dn7, locals.var_fn241_calc_iq__eta0_dn11, locals.var_fn241_calc_iq__eta0_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta0 = assign20680_e19940;
        locals.var_fn241_calc_iq__eta0_dn2 = assign20680_e19940_d_n2;
        locals.var_fn241_calc_iq__eta0_dn4 = assign20680_e19940_d_n4;
        locals.var_fn241_calc_iq__eta0_dn7 = assign20680_e19940_d_n7;
        locals.var_fn241_calc_iq__eta0_dn11 = assign20680_e19940_d_n11;
        locals.var_fn241_calc_iq__eta0_dn12 = assign20680_e19940_d_n12;

        let assign20690_e19943: f64 = if locals.var_fn241_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign20690_e19943;

        let (assign20700_e19951, assign20700_e19951_d_n2, assign20700_e19951_d_n4, assign20700_e19951_d_n7, assign20700_e19951_d_n11, assign20700_e19951_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard257 != 0.0)) {
        let assign20700_e19949: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0);
        (assign20700_e19949, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__eta0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20700_e19951;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20700_e19951_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20700_e19951_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20700_e19951_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20700_e19951_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20700_e19951_d_n12;

        let assign20710_e19954: f64 = (-50.0);
        let assign20710_e19955: f64 = if locals.var_fn241_calc_iq__eta0 < assign20710_e19954 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign20710_e19955;

        let (assign20720_e19967, assign20720_e19967_d_n2, assign20720_e19967_d_n4, assign20720_e19967_d_n7, assign20720_e19967_d_n11, assign20720_e19967_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 != 0.0)) {
        let assign20720_e19964: f64 = (locals.var_fn241_calc_iq__eta0).exp();
        let assign20720_e19965: f64 = (locals.var_fn241_calc_iq__qref0 * assign20720_e19964);
        (assign20720_e19965, (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20720_e19964) + (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20720_e19967;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20720_e19967_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20720_e19967_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20720_e19967_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20720_e19967_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20720_e19967_d_n12;

        let (assign20730_e19983, assign20730_e19983_d_n2, assign20730_e19983_d_n4, assign20730_e19983_d_n7, assign20730_e19983_d_n11, assign20730_e19983_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 == 0.0)) {
        let assign20730_e19978: f64 = (locals.var_fn241_calc_iq__eta0).exp();
        let assign20730_e19979: f64 = (1.0 + assign20730_e19978);
        let assign20730_e19980: f64 = (assign20730_e19979).ln();
        let assign20730_e19981: f64 = (locals.var_fn241_calc_iq__qref0 * assign20730_e19980);
        (assign20730_e19981, (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn2) / assign20730_e19979)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20730_e19980) + (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn4) / assign20730_e19979))), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn7) / assign20730_e19979)), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn11) / assign20730_e19979)), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn12) / assign20730_e19979)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20730_e19983;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20730_e19983_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20730_e19983_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20730_e19983_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20730_e19983_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20730_e19983_d_n12;

        let (assign20740_e19989, assign20740_e19989_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20740_e19987: f64 = (locals.var_fn241_calc_iq__mu0 / locals.var_fn241_calc_iq__tfacmobin);
        (assign20740_e19987, (-((locals.var_fn241_calc_iq__mu0 * locals.var_fn241_calc_iq__tfacmobin_dn4) / (locals.var_fn241_calc_iq__tfacmobin * locals.var_fn241_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn241_calc_iq__muf0, locals.var_fn241_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn241_calc_iq__muf0 = assign20740_e19989;
        locals.var_fn241_calc_iq__muf0_dn4 = assign20740_e19989_d_n4;

        let (assign20750_e20005, assign20750_e20005_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20750_e19995: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tnomin);
        let assign20750_e19996: f64 = (1.0 + assign20750_e19995);
        let assign20750_e20000: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin);
        let assign20750_e20001: f64 = (1.0 + assign20750_e20000);
        let assign20750_e20002: f64 = (assign20750_e19996 / assign20750_e20001);
        let assign20750_e20003: f64 = (locals.var_fn241_calc_iq__vel0 * assign20750_e20002);
        (assign20750_e20003, (locals.var_fn241_calc_iq__vel0 * (-((assign20750_e19996 * (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin_dn4)) / (assign20750_e20001 * assign20750_e20001)))),)
    } else {
        (locals.var_fn241_calc_iq__vx0, locals.var_fn241_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vx0 = assign20750_e20005;
        locals.var_fn241_calc_iq__vx0_dn4 = assign20750_e20005_d_n4;

        let (assign20760_e20013, assign20760_e20013_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20760_e20009: f64 = (locals.var_fn241_calc_iq__vx0 * locals.var_fn241_calc_iq__lin);
        let assign20760_e20011: f64 = (assign20760_e20009 / locals.var_fn241_calc_iq__muf0);
        (assign20760_e20011, ((((locals.var_fn241_calc_iq__vx0_dn4 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf0) - (assign20760_e20009 * locals.var_fn241_calc_iq__muf0_dn4)) / (locals.var_fn241_calc_iq__muf0 * locals.var_fn241_calc_iq__muf0)),)
    } else {
        (locals.var_fn241_calc_iq__vdsats0, locals.var_fn241_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vdsats0 = assign20760_e20013;
        locals.var_fn241_calc_iq__vdsats0_dn4 = assign20760_e20013_d_n4;

        let (assign20770_e20030, assign20770_e20030_d_n2, assign20770_e20030_d_n4, assign20770_e20030_d_n7, assign20770_e20030_d_n11, assign20770_e20030_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20770_e20019: f64 = (2.0 * locals.var_fn241_calc_iq__qinvv0);
        let assign20770_e20021: f64 = (assign20770_e20019 / locals.var_fn241_calc_iq__cgin);
        let assign20770_e20023: f64 = (assign20770_e20021 / locals.var_fn241_calc_iq__vdsats0);
        let assign20770_e20024: f64 = (1.0 + assign20770_e20023);
        let assign20770_e20025: f64 = (assign20770_e20024).sqrt();
        let assign20770_e20026: f64 = (locals.var_fn241_calc_iq__vdsats0 * assign20770_e20025);
        let assign20770_e20028: f64 = (assign20770_e20026 - locals.var_fn241_calc_iq__vdsats0);
        (assign20770_e20028, (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn2) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (((locals.var_fn241_calc_iq__vdsats0_dn4 * assign20770_e20025) + (locals.var_fn241_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20770_e20019 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)) * locals.var_fn241_calc_iq__vdsats0) - (assign20770_e20021 * locals.var_fn241_calc_iq__vdsats0_dn4)) / (locals.var_fn241_calc_iq__vdsats0 * locals.var_fn241_calc_iq__vdsats0)) / (2.0 * assign20770_e20025)))) - locals.var_fn241_calc_iq__vdsats0_dn4), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn7) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn11) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn12) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))),)
    } else {
        (locals.var_fn241_calc_iq__vdsats10, locals.var_fn241_calc_iq__vdsats10_dn2, locals.var_fn241_calc_iq__vdsats10_dn4, locals.var_fn241_calc_iq__vdsats10_dn7, locals.var_fn241_calc_iq__vdsats10_dn11, locals.var_fn241_calc_iq__vdsats10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats10 = assign20770_e20030;
        locals.var_fn241_calc_iq__vdsats10_dn2 = assign20770_e20030_d_n2;
        locals.var_fn241_calc_iq__vdsats10_dn4 = assign20770_e20030_d_n4;
        locals.var_fn241_calc_iq__vdsats10_dn7 = assign20770_e20030_d_n7;
        locals.var_fn241_calc_iq__vdsats10_dn11 = assign20770_e20030_d_n11;
        locals.var_fn241_calc_iq__vdsats10_dn12 = assign20770_e20030_d_n12;

        let (assign20780_e20042, assign20780_e20042_d_n2, assign20780_e20042_d_n4, assign20780_e20042_d_n7, assign20780_e20042_d_n11, assign20780_e20042_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20780_e20035: f64 = (1.0 - locals.var_fn241_calc_iq__ff0);
        let assign20780_e20036: f64 = (locals.var_fn241_calc_iq__vdsats10 * assign20780_e20035);
        let assign20780_e20039: f64 = (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0);
        let assign20780_e20040: f64 = (assign20780_e20036 + assign20780_e20039);
        (assign20780_e20040, (((locals.var_fn241_calc_iq__vdsats10_dn2 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn2))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn2)), (((locals.var_fn241_calc_iq__vdsats10_dn4 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit0_dn4 * locals.var_fn241_calc_iq__ff0) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn4))), (((locals.var_fn241_calc_iq__vdsats10_dn7 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn7))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn7)), (((locals.var_fn241_calc_iq__vdsats10_dn11 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn11))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn11)), (((locals.var_fn241_calc_iq__vdsats10_dn12 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn12))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdsat10, locals.var_fn241_calc_iq__vdsat10_dn2, locals.var_fn241_calc_iq__vdsat10_dn4, locals.var_fn241_calc_iq__vdsat10_dn7, locals.var_fn241_calc_iq__vdsat10_dn11, locals.var_fn241_calc_iq__vdsat10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat10 = assign20780_e20042;
        locals.var_fn241_calc_iq__vdsat10_dn2 = assign20780_e20042_d_n2;
        locals.var_fn241_calc_iq__vdsat10_dn4 = assign20780_e20042_d_n4;
        locals.var_fn241_calc_iq__vdsat10_dn7 = assign20780_e20042_d_n7;
        locals.var_fn241_calc_iq__vdsat10_dn11 = assign20780_e20042_d_n11;
        locals.var_fn241_calc_iq__vdsat10_dn12 = assign20780_e20042_d_n12;

    }

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20790_e20111, assign20790_e20111_d_n2, assign20790_e20111_d_n4, assign20790_e20111_d_n7, assign20790_e20111_d_n11, assign20790_e20111_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20790_e20101, assign20790_e20101_d_n2, assign20790_e20101_d_n4, assign20790_e20101_d_n7, assign20790_e20101_d_n11, assign20790_e20101_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20790_e20054: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20055: f64 = assign20790_e20054;
                let assign20790_e20059: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20060: f64 = (-assign20790_e20059);
                let assign20790_e20063: f64 = (0.001 / p.p53);
                let assign20790_e20067: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20068: f64 = (-assign20790_e20067);
                let assign20790_e20069: f64 = (assign20790_e20063 * assign20790_e20068);
                let assign20790_e20070: f64 = (assign20790_e20069).tanh();
                let assign20790_e20071: f64 = (assign20790_e20060 * assign20790_e20070);
                let assign20790_e20072: f64 = (assign20790_e20055 + assign20790_e20071);
                let assign20790_e20073: f64 = (0.5 * assign20790_e20072);
                (assign20790_e20073, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))),)
            } else {
                let (assign20790_e20100, assign20790_e20100_d_n2, assign20790_e20100_d_n4, assign20790_e20100_d_n7, assign20790_e20100_d_n11, assign20790_e20100_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20790_e20081: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20082: f64 = assign20790_e20081;
                        let assign20790_e20086: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20087: f64 = (-assign20790_e20086);
                        let assign20790_e20091: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20092: f64 = (-assign20790_e20091);
                        let assign20790_e20093: f64 = (assign20790_e20087 * assign20790_e20092);
                        let assign20790_e20095: f64 = (assign20790_e20093 + p.p53);
                        let assign20790_e20096: f64 = (assign20790_e20095).sqrt();
                        let assign20790_e20097: f64 = (assign20790_e20082 + assign20790_e20096);
                        let assign20790_e20098: f64 = (0.5 * assign20790_e20097);
                        (assign20790_e20098, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20092) + (assign20790_e20087 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20790_e20096)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20092) + (assign20790_e20087 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20790_e20096)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20790_e20100, assign20790_e20100_d_n2, assign20790_e20100_d_n4, assign20790_e20100_d_n7, assign20790_e20100_d_n11, assign20790_e20100_d_n12,)
            }
        };
        let assign20790_e20103: f64 = (assign20790_e20101).powf(locals.var_fn241_calc_iq__beta);
        let assign20790_e20104: f64 = (1.0 + assign20790_e20103);
        let assign20790_e20107: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20790_e20108: f64 = (assign20790_e20104).powf(assign20790_e20107);
        let assign20790_e20109: f64 = (1.0 / assign20790_e20108);
        (assign20790_e20109, (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n2)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n2 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n2)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n2 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n4)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n4 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n4)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n4 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n7)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n7 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n7)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n7 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n11)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n11 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n11)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n11 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n12)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n12 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n12)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n12 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))),)
    } else {
        (locals.var_fn241_calc_iq__fsd0, locals.var_fn241_calc_iq__fsd0_dn2, locals.var_fn241_calc_iq__fsd0_dn4, locals.var_fn241_calc_iq__fsd0_dn7, locals.var_fn241_calc_iq__fsd0_dn11, locals.var_fn241_calc_iq__fsd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd0 = assign20790_e20111;
        locals.var_fn241_calc_iq__fsd0_dn2 = assign20790_e20111_d_n2;
        locals.var_fn241_calc_iq__fsd0_dn4 = assign20790_e20111_d_n4;
        locals.var_fn241_calc_iq__fsd0_dn7 = assign20790_e20111_d_n7;
        locals.var_fn241_calc_iq__fsd0_dn11 = assign20790_e20111_d_n11;
        locals.var_fn241_calc_iq__fsd0_dn12 = assign20790_e20111_d_n12;

        let (assign20800_e20117, assign20800_e20117_d_n2, assign20800_e20117_d_n4, assign20800_e20117_d_n7, assign20800_e20117_d_n11, assign20800_e20117_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20800_e20115: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0);
        (assign20800_e20115, (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn2), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn4), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn7), ((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__fsd0) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn11)), ((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__fsd0) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdx0, locals.var_fn241_calc_iq__vdx0_dn2, locals.var_fn241_calc_iq__vdx0_dn4, locals.var_fn241_calc_iq__vdx0_dn7, locals.var_fn241_calc_iq__vdx0_dn11, locals.var_fn241_calc_iq__vdx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx0 = assign20800_e20117;
        locals.var_fn241_calc_iq__vdx0_dn2 = assign20800_e20117_d_n2;
        locals.var_fn241_calc_iq__vdx0_dn4 = assign20800_e20117_d_n4;
        locals.var_fn241_calc_iq__vdx0_dn7 = assign20800_e20117_d_n7;
        locals.var_fn241_calc_iq__vdx0_dn11 = assign20800_e20117_d_n11;
        locals.var_fn241_calc_iq__vdx0_dn12 = assign20800_e20117_d_n12;

        let (assign20810_e20192, assign20810_e20192_d_n2, assign20810_e20192_d_n4, assign20810_e20192_d_n7, assign20810_e20192_d_n11, assign20810_e20192_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20810_e20182, assign20810_e20182_d_n2, assign20810_e20182_d_n4, assign20810_e20182_d_n7, assign20810_e20182_d_n11, assign20810_e20182_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20810_e20128: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20130: f64 = (assign20810_e20128 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20131: f64 = assign20810_e20130;
                let assign20810_e20134: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20136: f64 = (assign20810_e20134 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20137: f64 = (-assign20810_e20136);
                let assign20810_e20140: f64 = (0.001 / p.p53);
                let assign20810_e20143: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20145: f64 = (assign20810_e20143 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20146: f64 = (-assign20810_e20145);
                let assign20810_e20147: f64 = (assign20810_e20140 * assign20810_e20146);
                let assign20810_e20148: f64 = (assign20810_e20147).tanh();
                let assign20810_e20149: f64 = (assign20810_e20137 * assign20810_e20148);
                let assign20810_e20150: f64 = (assign20810_e20131 + assign20810_e20149);
                let assign20810_e20151: f64 = (0.5 * assign20810_e20150);
                (assign20810_e20151, (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))),)
            } else {
                let (assign20810_e20181, assign20810_e20181_d_n2, assign20810_e20181_d_n4, assign20810_e20181_d_n7, assign20810_e20181_d_n11, assign20810_e20181_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20810_e20158: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20160: f64 = (assign20810_e20158 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20161: f64 = assign20810_e20160;
                        let assign20810_e20164: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20166: f64 = (assign20810_e20164 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20167: f64 = (-assign20810_e20166);
                        let assign20810_e20170: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20172: f64 = (assign20810_e20170 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20173: f64 = (-assign20810_e20172);
                        let assign20810_e20174: f64 = (assign20810_e20167 * assign20810_e20173);
                        let assign20810_e20176: f64 = (assign20810_e20174 + p.p53);
                        let assign20810_e20177: f64 = (assign20810_e20176).sqrt();
                        let assign20810_e20178: f64 = (assign20810_e20161 + assign20810_e20177);
                        let assign20810_e20179: f64 = (0.5 * assign20810_e20178);
                        (assign20810_e20179, (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20173) + (assign20810_e20167 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20810_e20177)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20173) + (assign20810_e20167 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20810_e20177)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20810_e20181, assign20810_e20181_d_n2, assign20810_e20181_d_n4, assign20810_e20181_d_n7, assign20810_e20181_d_n11, assign20810_e20181_d_n12,)
            }
        };
        let assign20810_e20184: f64 = (assign20810_e20182).powf(locals.var_fn241_calc_iq__beta);
        let assign20810_e20185: f64 = (1.0 + assign20810_e20184);
        let assign20810_e20188: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20810_e20189: f64 = (assign20810_e20185).powf(assign20810_e20188);
        let assign20810_e20190: f64 = (1.0 / assign20810_e20189);
        (assign20810_e20190, (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n2)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n2 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n2)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n2 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n4)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n4 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n4)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n4 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n7)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n7 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n7)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n7 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n11)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n11 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n11)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n11 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n12)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n12 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n12)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n12 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))),)
    } else {
        (locals.var_fn241_calc_iq__fds0, locals.var_fn241_calc_iq__fds0_dn2, locals.var_fn241_calc_iq__fds0_dn4, locals.var_fn241_calc_iq__fds0_dn7, locals.var_fn241_calc_iq__fds0_dn11, locals.var_fn241_calc_iq__fds0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds0 = assign20810_e20192;
        locals.var_fn241_calc_iq__fds0_dn2 = assign20810_e20192_d_n2;
        locals.var_fn241_calc_iq__fds0_dn4 = assign20810_e20192_d_n4;
        locals.var_fn241_calc_iq__fds0_dn7 = assign20810_e20192_d_n7;
        locals.var_fn241_calc_iq__fds0_dn11 = assign20810_e20192_d_n11;
        locals.var_fn241_calc_iq__fds0_dn12 = assign20810_e20192_d_n12;

        let (assign20820_e20199, assign20820_e20199_d_n2, assign20820_e20199_d_n4, assign20820_e20199_d_n7, assign20820_e20199_d_n11, assign20820_e20199_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20820_e20195: f64 = (-locals.var_fn241_calc_iq__vdsin);
        let assign20820_e20197: f64 = (assign20820_e20195 * locals.var_fn241_calc_iq__fds0);
        (assign20820_e20197, (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn2), (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn4), (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn7), (((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__fds0) + (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn11)), (((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__fds0) + (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vsx0, locals.var_fn241_calc_iq__vsx0_dn2, locals.var_fn241_calc_iq__vsx0_dn4, locals.var_fn241_calc_iq__vsx0_dn7, locals.var_fn241_calc_iq__vsx0_dn11, locals.var_fn241_calc_iq__vsx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx0 = assign20820_e20199;
        locals.var_fn241_calc_iq__vsx0_dn2 = assign20820_e20199_d_n2;
        locals.var_fn241_calc_iq__vsx0_dn4 = assign20820_e20199_d_n4;
        locals.var_fn241_calc_iq__vsx0_dn7 = assign20820_e20199_d_n7;
        locals.var_fn241_calc_iq__vsx0_dn11 = assign20820_e20199_d_n11;
        locals.var_fn241_calc_iq__vsx0_dn12 = assign20820_e20199_d_n12;

        let (assign20830_e20207, assign20830_e20207_d_n2, assign20830_e20207_d_n4, assign20830_e20207_d_n7, assign20830_e20207_d_n11, assign20830_e20207_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20830_e20203: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__myarg0);
        let assign20830_e20205: f64 = (assign20830_e20203 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20830_e20205, (locals.var_fn241_calc_iq__vgsin_dn2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20830_e20203 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (locals.var_fn241_calc_iq__vgsin_dn7 / locals.var_fn241_calc_iq__alpha_phit), 0.0, (locals.var_fn241_calc_iq__vgsin_dn12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20830_e20207;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20830_e20207_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20830_e20207_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20830_e20207_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20830_e20207_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20830_e20207_d_n12;

        let assign20840_e20210: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign20840_e20210;

        let (assign20850_e20216, assign20850_e20216_d_n2, assign20850_e20216_d_n4, assign20850_e20216_d_n7, assign20850_e20216_d_n11, assign20850_e20216_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard259 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20850_e20216;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20850_e20216_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20850_e20216_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20850_e20216_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20850_e20216_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20850_e20216_d_n12;

        let assign20860_e20219: f64 = (-50.0);
        let assign20860_e20220: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20860_e20219 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign20860_e20220;

        let (assign20870_e20229, assign20870_e20229_d_n2, assign20870_e20229_d_n4, assign20870_e20229_d_n7, assign20870_e20229_d_n11, assign20870_e20229_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard260 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20870_e20229;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20870_e20229_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20870_e20229_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20870_e20229_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20870_e20229_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20870_e20229_d_n12;

        let (assign20880_e20244, assign20880_e20244_d_n2, assign20880_e20244_d_n4, assign20880_e20244_d_n7, assign20880_e20244_d_n11, assign20880_e20244_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard260 == 0.0)) {
        let assign20880_e20240: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign20880_e20241: f64 = (1.0 + assign20880_e20240);
        let assign20880_e20242: f64 = (1.0 / assign20880_e20241);
        (assign20880_e20242, (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign20880_e20241 * assign20880_e20241))),)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20880_e20244;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20880_e20244_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20880_e20244_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20880_e20244_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20880_e20244_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20880_e20244_d_n12;

        let (assign20890_e20262, assign20890_e20262_d_n2, assign20890_e20262_d_n4, assign20890_e20262_d_n7, assign20890_e20262_d_n11, assign20890_e20262_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20890_e20248: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__vsx0);
        let assign20890_e20252: f64 = (p.p51 * 0.1);
        let assign20890_e20254: f64 = (assign20890_e20252 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20890_e20256: f64 = (assign20890_e20254 * locals.var_fn241_calc_iq__ffs0);
        let assign20890_e20257: f64 = (locals.var_fn241_calc_iq__vtof - assign20890_e20256);
        let assign20890_e20258: f64 = (assign20890_e20248 - assign20890_e20257);
        let assign20890_e20260: f64 = (assign20890_e20258 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign20890_e20260, (((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__vsx0_dn2) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), (((((-locals.var_fn241_calc_iq__vsx0_dn4) - (locals.var_fn241_calc_iq__vtof_dn4 - (((assign20890_e20252 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffs0) + (assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign20890_e20258 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__vsx0_dn7) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__vsx0_dn11) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__vsx0_dn12) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etas0, locals.var_fn241_calc_iq__etas0_dn2, locals.var_fn241_calc_iq__etas0_dn4, locals.var_fn241_calc_iq__etas0_dn7, locals.var_fn241_calc_iq__etas0_dn11, locals.var_fn241_calc_iq__etas0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas0 = assign20890_e20262;
        locals.var_fn241_calc_iq__etas0_dn2 = assign20890_e20262_d_n2;
        locals.var_fn241_calc_iq__etas0_dn4 = assign20890_e20262_d_n4;
        locals.var_fn241_calc_iq__etas0_dn7 = assign20890_e20262_d_n7;
        locals.var_fn241_calc_iq__etas0_dn11 = assign20890_e20262_d_n11;
        locals.var_fn241_calc_iq__etas0_dn12 = assign20890_e20262_d_n12;

        let assign20900_e20265: f64 = if locals.var_fn241_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign20900_e20265;

        let (assign20910_e20273, assign20910_e20273_d_n2, assign20910_e20273_d_n4, assign20910_e20273_d_n7, assign20910_e20273_d_n11, assign20910_e20273_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard261 != 0.0)) {
        let assign20910_e20271: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0);
        (assign20910_e20271, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__etas0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20910_e20273;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20910_e20273_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20910_e20273_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20910_e20273_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20910_e20273_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20910_e20273_d_n12;

        let assign20920_e20276: f64 = (-50.0);
        let assign20920_e20277: f64 = if locals.var_fn241_calc_iq__etas0 < assign20920_e20276 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign20920_e20277;

        let (assign20930_e20289, assign20930_e20289_d_n2, assign20930_e20289_d_n4, assign20930_e20289_d_n7, assign20930_e20289_d_n11, assign20930_e20289_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign20930_e20286: f64 = (locals.var_fn241_calc_iq__etas0).exp();
        let assign20930_e20287: f64 = (locals.var_fn241_calc_iq__qref0 * assign20930_e20286);
        (assign20930_e20287, (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20930_e20286) + (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20930_e20289;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20930_e20289_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20930_e20289_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20930_e20289_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20930_e20289_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20930_e20289_d_n12;

        let (assign20940_e20305, assign20940_e20305_d_n2, assign20940_e20305_d_n4, assign20940_e20305_d_n7, assign20940_e20305_d_n11, assign20940_e20305_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 == 0.0)) {
        let assign20940_e20300: f64 = (locals.var_fn241_calc_iq__etas0).exp();
        let assign20940_e20301: f64 = (1.0 + assign20940_e20300);
        let assign20940_e20302: f64 = (assign20940_e20301).ln();
        let assign20940_e20303: f64 = (locals.var_fn241_calc_iq__qref0 * assign20940_e20302);
        (assign20940_e20303, (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn2) / assign20940_e20301)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20940_e20302) + (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn4) / assign20940_e20301))), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn7) / assign20940_e20301)), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn11) / assign20940_e20301)), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn12) / assign20940_e20301)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20940_e20305;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20940_e20305_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20940_e20305_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20940_e20305_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20940_e20305_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20940_e20305_d_n12;

        let (assign20950_e20313, assign20950_e20313_d_n2, assign20950_e20313_d_n4, assign20950_e20313_d_n7, assign20950_e20313_d_n11, assign20950_e20313_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20950_e20309: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__myarg0);
        let assign20950_e20311: f64 = (assign20950_e20309 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20950_e20311, (locals.var_fn241_calc_iq__vgdin_dn2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20950_e20309 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (locals.var_fn241_calc_iq__vgdin_dn7 / locals.var_fn241_calc_iq__alpha_phit), (locals.var_fn241_calc_iq__vgdin_dn11 / locals.var_fn241_calc_iq__alpha_phit), (locals.var_fn241_calc_iq__vgdin_dn12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20950_e20313;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20950_e20313_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20950_e20313_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20950_e20313_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20950_e20313_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20950_e20313_d_n12;

        let assign20960_e20316: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign20960_e20316;

        let (assign20970_e20322, assign20970_e20322_d_n2, assign20970_e20322_d_n4, assign20970_e20322_d_n7, assign20970_e20322_d_n11, assign20970_e20322_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign20970_e20322;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign20970_e20322_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign20970_e20322_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign20970_e20322_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign20970_e20322_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign20970_e20322_d_n12;

        let assign20980_e20325: f64 = (-50.0);
        let assign20980_e20326: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20980_e20325 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign20980_e20326;

        let (assign20990_e20335, assign20990_e20335_d_n2, assign20990_e20335_d_n4, assign20990_e20335_d_n7, assign20990_e20335_d_n11, assign20990_e20335_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign20990_e20335;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign20990_e20335_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign20990_e20335_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign20990_e20335_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign20990_e20335_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign20990_e20335_d_n12;

        let (assign21000_e20350, assign21000_e20350_d_n2, assign21000_e20350_d_n4, assign21000_e20350_d_n7, assign21000_e20350_d_n11, assign21000_e20350_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 == 0.0)) {
        let assign21000_e20346: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign21000_e20347: f64 = (1.0 + assign21000_e20346);
        let assign21000_e20348: f64 = (1.0 / assign21000_e20347);
        (assign21000_e20348, (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign21000_e20347 * assign21000_e20347))),)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign21000_e20350;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign21000_e20350_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign21000_e20350_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign21000_e20350_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign21000_e20350_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign21000_e20350_d_n12;

        let (assign21010_e20368, assign21010_e20368_d_n2, assign21010_e20368_d_n4, assign21010_e20368_d_n7, assign21010_e20368_d_n11, assign21010_e20368_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21010_e20354: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdx0);
        let assign21010_e20358: f64 = (p.p51 * 0.1);
        let assign21010_e20360: f64 = (assign21010_e20358 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21010_e20362: f64 = (assign21010_e20360 * locals.var_fn241_calc_iq__ffd0);
        let assign21010_e20363: f64 = (locals.var_fn241_calc_iq__vtof - assign21010_e20362);
        let assign21010_e20364: f64 = (assign21010_e20354 - assign21010_e20363);
        let assign21010_e20366: f64 = (assign21010_e20364 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21010_e20366, (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vdx0_dn2) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), (((((-locals.var_fn241_calc_iq__vdx0_dn4) - (locals.var_fn241_calc_iq__vtof_dn4 - (((assign21010_e20358 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffd0) + (assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21010_e20364 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vdx0_dn7) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), (((-locals.var_fn241_calc_iq__vdx0_dn11) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdx0_dn12) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etad0, locals.var_fn241_calc_iq__etad0_dn2, locals.var_fn241_calc_iq__etad0_dn4, locals.var_fn241_calc_iq__etad0_dn7, locals.var_fn241_calc_iq__etad0_dn11, locals.var_fn241_calc_iq__etad0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad0 = assign21010_e20368;
        locals.var_fn241_calc_iq__etad0_dn2 = assign21010_e20368_d_n2;
        locals.var_fn241_calc_iq__etad0_dn4 = assign21010_e20368_d_n4;
        locals.var_fn241_calc_iq__etad0_dn7 = assign21010_e20368_d_n7;
        locals.var_fn241_calc_iq__etad0_dn11 = assign21010_e20368_d_n11;
        locals.var_fn241_calc_iq__etad0_dn12 = assign21010_e20368_d_n12;

        let assign21020_e20371: f64 = if locals.var_fn241_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign21020_e20371;

        let (assign21030_e20379, assign21030_e20379_d_n2, assign21030_e20379_d_n4, assign21030_e20379_d_n7, assign21030_e20379_d_n11, assign21030_e20379_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard265 != 0.0)) {
        let assign21030_e20377: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0);
        (assign21030_e20377, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__etad0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21030_e20379;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21030_e20379_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21030_e20379_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21030_e20379_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21030_e20379_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21030_e20379_d_n12;

        let assign21040_e20382: f64 = (-50.0);
        let assign21040_e20383: f64 = if locals.var_fn241_calc_iq__etad0 < assign21040_e20382 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign21040_e20383;

        let (assign21050_e20395, assign21050_e20395_d_n2, assign21050_e20395_d_n4, assign21050_e20395_d_n7, assign21050_e20395_d_n11, assign21050_e20395_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign21050_e20392: f64 = (locals.var_fn241_calc_iq__etad0).exp();
        let assign21050_e20393: f64 = (locals.var_fn241_calc_iq__qref0 * assign21050_e20392);
        (assign21050_e20393, (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign21050_e20392) + (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21050_e20395;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21050_e20395_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21050_e20395_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21050_e20395_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21050_e20395_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21050_e20395_d_n12;

        let (assign21060_e20411, assign21060_e20411_d_n2, assign21060_e20411_d_n4, assign21060_e20411_d_n7, assign21060_e20411_d_n11, assign21060_e20411_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign21060_e20406: f64 = (locals.var_fn241_calc_iq__etad0).exp();
        let assign21060_e20407: f64 = (1.0 + assign21060_e20406);
        let assign21060_e20408: f64 = (assign21060_e20407).ln();
        let assign21060_e20409: f64 = (locals.var_fn241_calc_iq__qref0 * assign21060_e20408);
        (assign21060_e20409, (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn2) / assign21060_e20407)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign21060_e20408) + (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn4) / assign21060_e20407))), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn7) / assign21060_e20407)), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn11) / assign21060_e20407)), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn12) / assign21060_e20407)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21060_e20411;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21060_e20411_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21060_e20411_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21060_e20411_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21060_e20411_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21060_e20411_d_n12;

        let (assign21070_e20419, assign21070_e20419_d_n2, assign21070_e20419_d_n4, assign21070_e20419_d_n7, assign21070_e20419_d_n11, assign21070_e20419_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21070_e20415: f64 = (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0);
        let assign21070_e20417: f64 = (assign21070_e20415 + 1e-38);
        (assign21070_e20417, ((locals.var_fn241_calc_iq__qinvs0_dn2 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn2)), ((locals.var_fn241_calc_iq__qinvs0_dn4 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn4)), ((locals.var_fn241_calc_iq__qinvs0_dn7 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn7)), ((locals.var_fn241_calc_iq__qinvs0_dn11 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn11)), ((locals.var_fn241_calc_iq__qinvs0_dn12 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qs2, locals.var_fn241_calc_iq__qs2_dn2, locals.var_fn241_calc_iq__qs2_dn4, locals.var_fn241_calc_iq__qs2_dn7, locals.var_fn241_calc_iq__qs2_dn11, locals.var_fn241_calc_iq__qs2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs2 = assign21070_e20419;
        locals.var_fn241_calc_iq__qs2_dn2 = assign21070_e20419_d_n2;
        locals.var_fn241_calc_iq__qs2_dn4 = assign21070_e20419_d_n4;
        locals.var_fn241_calc_iq__qs2_dn7 = assign21070_e20419_d_n7;
        locals.var_fn241_calc_iq__qs2_dn11 = assign21070_e20419_d_n11;
        locals.var_fn241_calc_iq__qs2_dn12 = assign21070_e20419_d_n12;

        let (assign21080_e20427, assign21080_e20427_d_n2, assign21080_e20427_d_n4, assign21080_e20427_d_n7, assign21080_e20427_d_n11, assign21080_e20427_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21080_e20423: f64 = (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0);
        let assign21080_e20425: f64 = (assign21080_e20423 + 1e-57);
        (assign21080_e20425, ((locals.var_fn241_calc_iq__qs2_dn2 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn2)), ((locals.var_fn241_calc_iq__qs2_dn4 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn4)), ((locals.var_fn241_calc_iq__qs2_dn7 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn7)), ((locals.var_fn241_calc_iq__qs2_dn11 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn11)), ((locals.var_fn241_calc_iq__qs2_dn12 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qs3, locals.var_fn241_calc_iq__qs3_dn2, locals.var_fn241_calc_iq__qs3_dn4, locals.var_fn241_calc_iq__qs3_dn7, locals.var_fn241_calc_iq__qs3_dn11, locals.var_fn241_calc_iq__qs3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs3 = assign21080_e20427;
        locals.var_fn241_calc_iq__qs3_dn2 = assign21080_e20427_d_n2;
        locals.var_fn241_calc_iq__qs3_dn4 = assign21080_e20427_d_n4;
        locals.var_fn241_calc_iq__qs3_dn7 = assign21080_e20427_d_n7;
        locals.var_fn241_calc_iq__qs3_dn11 = assign21080_e20427_d_n11;
        locals.var_fn241_calc_iq__qs3_dn12 = assign21080_e20427_d_n12;

        let (assign21090_e20435, assign21090_e20435_d_n2, assign21090_e20435_d_n4, assign21090_e20435_d_n7, assign21090_e20435_d_n11, assign21090_e20435_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21090_e20431: f64 = (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0);
        let assign21090_e20433: f64 = (assign21090_e20431 + 1e-38);
        (assign21090_e20433, ((locals.var_fn241_calc_iq__qinvd0_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qinvd0_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qinvd0_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qinvd0_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qinvd0_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qd2, locals.var_fn241_calc_iq__qd2_dn2, locals.var_fn241_calc_iq__qd2_dn4, locals.var_fn241_calc_iq__qd2_dn7, locals.var_fn241_calc_iq__qd2_dn11, locals.var_fn241_calc_iq__qd2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd2 = assign21090_e20435;
        locals.var_fn241_calc_iq__qd2_dn2 = assign21090_e20435_d_n2;
        locals.var_fn241_calc_iq__qd2_dn4 = assign21090_e20435_d_n4;
        locals.var_fn241_calc_iq__qd2_dn7 = assign21090_e20435_d_n7;
        locals.var_fn241_calc_iq__qd2_dn11 = assign21090_e20435_d_n11;
        locals.var_fn241_calc_iq__qd2_dn12 = assign21090_e20435_d_n12;

        let (assign21100_e20443, assign21100_e20443_d_n2, assign21100_e20443_d_n4, assign21100_e20443_d_n7, assign21100_e20443_d_n11, assign21100_e20443_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21100_e20439: f64 = (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0);
        let assign21100_e20441: f64 = (assign21100_e20439 + 1e-57);
        (assign21100_e20441, ((locals.var_fn241_calc_iq__qd2_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qd2_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qd2_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qd2_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qd2_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qd3, locals.var_fn241_calc_iq__qd3_dn2, locals.var_fn241_calc_iq__qd3_dn4, locals.var_fn241_calc_iq__qd3_dn7, locals.var_fn241_calc_iq__qd3_dn11, locals.var_fn241_calc_iq__qd3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd3 = assign21100_e20443;
        locals.var_fn241_calc_iq__qd3_dn2 = assign21100_e20443_d_n2;
        locals.var_fn241_calc_iq__qd3_dn4 = assign21100_e20443_d_n4;
        locals.var_fn241_calc_iq__qd3_dn7 = assign21100_e20443_d_n7;
        locals.var_fn241_calc_iq__qd3_dn11 = assign21100_e20443_d_n11;
        locals.var_fn241_calc_iq__qd3_dn12 = assign21100_e20443_d_n12;

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21110_e20451, assign21110_e20451_d_n2, assign21110_e20451_d_n4, assign21110_e20451_d_n7, assign21110_e20451_d_n11, assign21110_e20451_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21110_e20447: f64 = (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0);
        let assign21110_e20449: f64 = (assign21110_e20447 + 1e-38);
        (assign21110_e20449, ((locals.var_fn241_calc_iq__qinvs0_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qinvs0_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qinvs0_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qinvs0_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qinvs0_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qsqd, locals.var_fn241_calc_iq__qsqd_dn2, locals.var_fn241_calc_iq__qsqd_dn4, locals.var_fn241_calc_iq__qsqd_dn7, locals.var_fn241_calc_iq__qsqd_dn11, locals.var_fn241_calc_iq__qsqd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsqd = assign21110_e20451;
        locals.var_fn241_calc_iq__qsqd_dn2 = assign21110_e20451_d_n2;
        locals.var_fn241_calc_iq__qsqd_dn4 = assign21110_e20451_d_n4;
        locals.var_fn241_calc_iq__qsqd_dn7 = assign21110_e20451_d_n7;
        locals.var_fn241_calc_iq__qsqd_dn11 = assign21110_e20451_d_n11;
        locals.var_fn241_calc_iq__qsqd_dn12 = assign21110_e20451_d_n12;

        let (assign21120_e20469, assign21120_e20469_d_n2, assign21120_e20469_d_n4, assign21120_e20469_d_n7, assign21120_e20469_d_n11, assign21120_e20469_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21120_e20455: f64 = (2.0 / 3.0);
        let assign21120_e20458: f64 = (locals.var_fn241_calc_iq__qs2 + locals.var_fn241_calc_iq__qd2);
        let assign21120_e20460: f64 = (assign21120_e20458 + locals.var_fn241_calc_iq__qsqd);
        let assign21120_e20461: f64 = (assign21120_e20455 * assign21120_e20460);
        let assign21120_e20464: f64 = (locals.var_fn241_calc_iq__qinvs0 + locals.var_fn241_calc_iq__qinvd0);
        let assign21120_e20466: f64 = (assign21120_e20464 + 2e-19);
        let assign21120_e20467: f64 = (assign21120_e20461 / assign21120_e20466);
        (assign21120_e20467, ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn2 + locals.var_fn241_calc_iq__qd2_dn2) + locals.var_fn241_calc_iq__qsqd_dn2)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn2 + locals.var_fn241_calc_iq__qinvd0_dn2))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn4 + locals.var_fn241_calc_iq__qd2_dn4) + locals.var_fn241_calc_iq__qsqd_dn4)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn4 + locals.var_fn241_calc_iq__qinvd0_dn4))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn7 + locals.var_fn241_calc_iq__qd2_dn7) + locals.var_fn241_calc_iq__qsqd_dn7)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn7 + locals.var_fn241_calc_iq__qinvd0_dn7))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn11 + locals.var_fn241_calc_iq__qd2_dn11) + locals.var_fn241_calc_iq__qsqd_dn11)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn11 + locals.var_fn241_calc_iq__qinvd0_dn11))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn12 + locals.var_fn241_calc_iq__qd2_dn12) + locals.var_fn241_calc_iq__qsqd_dn12)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn12 + locals.var_fn241_calc_iq__qinvd0_dn12))) / (assign21120_e20466 * assign21120_e20466)),)
    } else {
        (locals.var_fn241_calc_iq__qinvdd, locals.var_fn241_calc_iq__qinvdd_dn2, locals.var_fn241_calc_iq__qinvdd_dn4, locals.var_fn241_calc_iq__qinvdd_dn7, locals.var_fn241_calc_iq__qinvdd_dn11, locals.var_fn241_calc_iq__qinvdd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvdd = assign21120_e20469;
        locals.var_fn241_calc_iq__qinvdd_dn2 = assign21120_e20469_d_n2;
        locals.var_fn241_calc_iq__qinvdd_dn4 = assign21120_e20469_d_n4;
        locals.var_fn241_calc_iq__qinvdd_dn7 = assign21120_e20469_d_n7;
        locals.var_fn241_calc_iq__qinvdd_dn11 = assign21120_e20469_d_n11;
        locals.var_fn241_calc_iq__qinvdd_dn12 = assign21120_e20469_d_n12;

        let (assign21130_e20503, assign21130_e20503_d_n2, assign21130_e20503_d_n4, assign21130_e20503_d_n7, assign21130_e20503_d_n11, assign21130_e20503_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21130_e20474: f64 = (2.0 * locals.var_fn241_calc_iq__qs3);
        let assign21130_e20477: f64 = (3.0 * locals.var_fn241_calc_iq__qd3);
        let assign21130_e20478: f64 = (assign21130_e20474 + assign21130_e20477);
        let assign21130_e20481: f64 = (4.0 * locals.var_fn241_calc_iq__qs2);
        let assign21130_e20483: f64 = (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0);
        let assign21130_e20484: f64 = (assign21130_e20478 + assign21130_e20483);
        let assign21130_e20487: f64 = (6.0 * locals.var_fn241_calc_iq__qd2);
        let assign21130_e20489: f64 = (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0);
        let assign21130_e20490: f64 = (assign21130_e20484 + assign21130_e20489);
        let assign21130_e20491: f64 = (2.0 * assign21130_e20490);
        let assign21130_e20495: f64 = (locals.var_fn241_calc_iq__qs2 + locals.var_fn241_calc_iq__qd2);
        let assign21130_e20498: f64 = (2.0 * locals.var_fn241_calc_iq__qsqd);
        let assign21130_e20499: f64 = (assign21130_e20495 + assign21130_e20498);
        let assign21130_e20500: f64 = (15.0 * assign21130_e20499);
        let assign21130_e20501: f64 = (assign21130_e20491 / assign21130_e20500);
        (assign21130_e20501, ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn2) + (3.0 * locals.var_fn241_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn2) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn2) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn2)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn2 + locals.var_fn241_calc_iq__qd2_dn2) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn2))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn4) + (3.0 * locals.var_fn241_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn4) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn4) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn4)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn4 + locals.var_fn241_calc_iq__qd2_dn4) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn4))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn7) + (3.0 * locals.var_fn241_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn7) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn7) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn7)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn7 + locals.var_fn241_calc_iq__qd2_dn7) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn7))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn11) + (3.0 * locals.var_fn241_calc_iq__qd3_dn11)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn11) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn11))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn11) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn11)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn11 + locals.var_fn241_calc_iq__qd2_dn11) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn11))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn12) + (3.0 * locals.var_fn241_calc_iq__qd3_dn12)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn12) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn12))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn12) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn12)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn12 + locals.var_fn241_calc_iq__qd2_dn12) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn12))))) / (assign21130_e20500 * assign21130_e20500)),)
    } else {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd1 = assign21130_e20503;
        locals.var_fn241_calc_iq__qd1_dn2 = assign21130_e20503_d_n2;
        locals.var_fn241_calc_iq__qd1_dn4 = assign21130_e20503_d_n4;
        locals.var_fn241_calc_iq__qd1_dn7 = assign21130_e20503_d_n7;
        locals.var_fn241_calc_iq__qd1_dn11 = assign21130_e20503_d_n11;
        locals.var_fn241_calc_iq__qd1_dn12 = assign21130_e20503_d_n12;

        let (assign21140_e20509, assign21140_e20509_d_n2, assign21140_e20509_d_n4, assign21140_e20509_d_n7, assign21140_e20509_d_n11, assign21140_e20509_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21140_e20507: f64 = (locals.var_fn241_calc_iq__qinvdd - locals.var_fn241_calc_iq__qd1);
        (assign21140_e20507, (locals.var_fn241_calc_iq__qinvdd_dn2 - locals.var_fn241_calc_iq__qd1_dn2), (locals.var_fn241_calc_iq__qinvdd_dn4 - locals.var_fn241_calc_iq__qd1_dn4), (locals.var_fn241_calc_iq__qinvdd_dn7 - locals.var_fn241_calc_iq__qd1_dn7), (locals.var_fn241_calc_iq__qinvdd_dn11 - locals.var_fn241_calc_iq__qd1_dn11), (locals.var_fn241_calc_iq__qinvdd_dn12 - locals.var_fn241_calc_iq__qd1_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qs, locals.var_fn241_calc_iq__qs_dn2, locals.var_fn241_calc_iq__qs_dn4, locals.var_fn241_calc_iq__qs_dn7, locals.var_fn241_calc_iq__qs_dn11, locals.var_fn241_calc_iq__qs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs = assign21140_e20509;
        locals.var_fn241_calc_iq__qs_dn2 = assign21140_e20509_d_n2;
        locals.var_fn241_calc_iq__qs_dn4 = assign21140_e20509_d_n4;
        locals.var_fn241_calc_iq__qs_dn7 = assign21140_e20509_d_n7;
        locals.var_fn241_calc_iq__qs_dn11 = assign21140_e20509_d_n11;
        locals.var_fn241_calc_iq__qs_dn12 = assign21140_e20509_d_n12;

        let (assign21150_e20513, assign21150_e20513_d_n2, assign21150_e20513_d_n4, assign21150_e20513_d_n7, assign21150_e20513_d_n11, assign21150_e20513_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    } else {
        (locals.var_fn241_calc_iq__qd, locals.var_fn241_calc_iq__qd_dn2, locals.var_fn241_calc_iq__qd_dn4, locals.var_fn241_calc_iq__qd_dn7, locals.var_fn241_calc_iq__qd_dn11, locals.var_fn241_calc_iq__qd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd = assign21150_e20513;
        locals.var_fn241_calc_iq__qd_dn2 = assign21150_e20513_d_n2;
        locals.var_fn241_calc_iq__qd_dn4 = assign21150_e20513_d_n4;
        locals.var_fn241_calc_iq__qd_dn7 = assign21150_e20513_d_n7;
        locals.var_fn241_calc_iq__qd_dn11 = assign21150_e20513_d_n11;
        locals.var_fn241_calc_iq__qd_dn12 = assign21150_e20513_d_n12;

        let (assign21160_e20527, assign21160_e20527_d_n2, assign21160_e20527_d_n4, assign21160_e20527_d_n7, assign21160_e20527_d_n11, assign21160_e20527_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21160_e20517: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21160_e20519: f64 = (assign21160_e20517 * locals.var_fn241_calc_iq__lin);
        let assign21160_e20521: f64 = (assign21160_e20519 * locals.var_fn241_calc_iq__type);
        let assign21160_e20523: f64 = (assign21160_e20521 * locals.var_fn241_calc_iq__qs);
        let assign21160_e20525: f64 = (assign21160_e20523 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21160_e20525, ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn4) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgsout = assign21160_e20527;
        locals.var_fn241_calc_iq__qgsout_dn2 = assign21160_e20527_d_n2;
        locals.var_fn241_calc_iq__qgsout_dn4 = assign21160_e20527_d_n4;
        locals.var_fn241_calc_iq__qgsout_dn7 = assign21160_e20527_d_n7;
        locals.var_fn241_calc_iq__qgsout_dn11 = assign21160_e20527_d_n11;
        locals.var_fn241_calc_iq__qgsout_dn12 = assign21160_e20527_d_n12;

        let (assign21170_e20541, assign21170_e20541_d_n2, assign21170_e20541_d_n4, assign21170_e20541_d_n7, assign21170_e20541_d_n11, assign21170_e20541_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21170_e20531: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21170_e20533: f64 = (assign21170_e20531 * locals.var_fn241_calc_iq__lin);
        let assign21170_e20535: f64 = (assign21170_e20533 * locals.var_fn241_calc_iq__type);
        let assign21170_e20537: f64 = (assign21170_e20535 * locals.var_fn241_calc_iq__qd);
        let assign21170_e20539: f64 = (assign21170_e20537 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21170_e20539, ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn4) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgdout = assign21170_e20541;
        locals.var_fn241_calc_iq__qgdout_dn2 = assign21170_e20541_d_n2;
        locals.var_fn241_calc_iq__qgdout_dn4 = assign21170_e20541_d_n4;
        locals.var_fn241_calc_iq__qgdout_dn7 = assign21170_e20541_d_n7;
        locals.var_fn241_calc_iq__qgdout_dn11 = assign21170_e20541_d_n11;
        locals.var_fn241_calc_iq__qgdout_dn12 = assign21170_e20541_d_n12;

        let assign21180_e20544: f64 = if locals.var_fn241_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign21180_e20544;

        let (assign21190_e20560, assign21190_e20560_d_n2, assign21190_e20560_d_n4, assign21190_e20560_d_n7, assign21190_e20560_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21190_e20552: f64 = (p.p51 * 0.5);
        let assign21190_e20554: f64 = (assign21190_e20552 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21190_e20555: f64 = (locals.var_fn241_calc_iq__vtof - assign21190_e20554);
        let assign21190_e20556: f64 = (locals.var_fn241_calc_iq__vcin - assign21190_e20555);
        let assign21190_e20558: f64 = (assign21190_e20556 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21190_e20558, (locals.var_fn241_calc_iq__vcin_dn2 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21190_e20552 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21190_e20556 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vcin_dn7 / locals.var_fn241_calc_iq__two_n_phit0), (locals.var_fn241_calc_iq__vcin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, locals.var_fn241_calc_iq__etac_dn12,)
    }
};
        locals.var_fn241_calc_iq__etac = assign21190_e20560;
        locals.var_fn241_calc_iq__etac_dn2 = assign21190_e20560_d_n2;
        locals.var_fn241_calc_iq__etac_dn4 = assign21190_e20560_d_n4;
        locals.var_fn241_calc_iq__etac_dn7 = assign21190_e20560_d_n7;
        locals.var_fn241_calc_iq__etac_dn12 = assign21190_e20560_d_n12;

        let assign21200_e20563: f64 = if locals.var_fn241_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign21200_e20563;

        let (assign21210_e20571, assign21210_e20571_d_n2, assign21210_e20571_d_n3, assign21210_e20571_d_n4, assign21210_e20571_d_n7, assign21210_e20571_d_n11, assign21210_e20571_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 != 0.0)) {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, 0.0, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, 0.0, locals.var_fn241_calc_iq__etac_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21210_e20571;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21210_e20571_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21210_e20571_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21210_e20571_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21210_e20571_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21210_e20571_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21210_e20571_d_n12;

        let assign21220_e20574: f64 = (-50.0);
        let assign21220_e20575: f64 = if locals.var_fn241_calc_iq__etac < assign21220_e20574 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign21220_e20575;

        let (assign21230_e20587, assign21230_e20587_d_n2, assign21230_e20587_d_n3, assign21230_e20587_d_n4, assign21230_e20587_d_n7, assign21230_e20587_d_n11, assign21230_e20587_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 != 0.0)) {
        let assign21230_e20585: f64 = (locals.var_fn241_calc_iq__etac).exp();
        (assign21230_e20585, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn2), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn4), (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn7), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21230_e20587;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21230_e20587_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21230_e20587_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21230_e20587_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21230_e20587_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21230_e20587_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21230_e20587_d_n12;

        let (assign21240_e20603, assign21240_e20603_d_n2, assign21240_e20603_d_n3, assign21240_e20603_d_n4, assign21240_e20603_d_n7, assign21240_e20603_d_n11, assign21240_e20603_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign21240_e20599: f64 = (locals.var_fn241_calc_iq__etac).exp();
        let assign21240_e20600: f64 = (1.0 + assign21240_e20599);
        let assign21240_e20601: f64 = (assign21240_e20600).ln();
        (assign21240_e20601, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn2) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn4) / assign21240_e20600), ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn7) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn12) / assign21240_e20600),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21240_e20603;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21240_e20603_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21240_e20603_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21240_e20603_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21240_e20603_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21240_e20603_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21240_e20603_d_n12;

        let (assign21250_e20621, assign21250_e20621_d_n2, assign21250_e20621_d_n3, assign21250_e20621_d_n4, assign21250_e20621_d_n7, assign21250_e20621_d_n11, assign21250_e20621_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21250_e20609: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21250_e20611: f64 = (assign21250_e20609 * locals.var_fn241_calc_iq__type);
        let assign21250_e20613: f64 = (assign21250_e20611 * locals.var_fn241_calc_iq__cc);
        let assign21250_e20615: f64 = (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21250_e20617: f64 = (assign21250_e20615 * locals.var_fn241_calc_iq__exparg);
        let assign21250_e20619: f64 = (assign21250_e20617 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21250_e20619, ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21250_e20611 * locals.var_fn241_calc_iq__cc_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21250_e20621;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21250_e20621_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21250_e20621_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21250_e20621_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21250_e20621_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21250_e20621_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21250_e20621_d_n12;

        let (assign21260_e20637, assign21260_e20637_d_n3, assign21260_e20637_d_n4, assign21260_e20637_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21260_e20629: f64 = (p.p51 * 0.5);
        let assign21260_e20631: f64 = (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21260_e20632: f64 = (locals.var_fn241_calc_iq__vtof - assign21260_e20631);
        let assign21260_e20633: f64 = (locals.var_fn241_calc_iq__vbin - assign21260_e20632);
        let assign21260_e20635: f64 = (assign21260_e20633 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21260_e20635, (locals.var_fn241_calc_iq__vbin_dn3 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21260_e20633 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vbin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etab, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, locals.var_fn241_calc_iq__etab_dn12,)
    }
};
        locals.var_fn241_calc_iq__etab = assign21260_e20637;
        locals.var_fn241_calc_iq__etab_dn3 = assign21260_e20637_d_n3;
        locals.var_fn241_calc_iq__etab_dn4 = assign21260_e20637_d_n4;
        locals.var_fn241_calc_iq__etab_dn12 = assign21260_e20637_d_n12;

        let assign21270_e20640: f64 = if locals.var_fn241_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign21270_e20640;

        let (assign21280_e20648, assign21280_e20648_d_n2, assign21280_e20648_d_n3, assign21280_e20648_d_n4, assign21280_e20648_d_n7, assign21280_e20648_d_n11, assign21280_e20648_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 != 0.0)) {
        (locals.var_fn241_calc_iq__etab, 0.0, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn241_calc_iq__etab_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21280_e20648;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21280_e20648_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21280_e20648_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21280_e20648_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21280_e20648_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21280_e20648_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21280_e20648_d_n12;

        let assign21290_e20651: f64 = (-50.0);
        let assign21290_e20652: f64 = if locals.var_fn241_calc_iq__etab < assign21290_e20651 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign21290_e20652;

        let (assign21300_e20664, assign21300_e20664_d_n2, assign21300_e20664_d_n3, assign21300_e20664_d_n4, assign21300_e20664_d_n7, assign21300_e20664_d_n11, assign21300_e20664_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 != 0.0)) {
        let assign21300_e20662: f64 = (locals.var_fn241_calc_iq__etab).exp();
        (assign21300_e20662, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn3), (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn4), 0.0, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21300_e20664;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21300_e20664_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21300_e20664_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21300_e20664_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21300_e20664_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21300_e20664_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21300_e20664_d_n12;

        let (assign21310_e20680, assign21310_e20680_d_n2, assign21310_e20680_d_n3, assign21310_e20680_d_n4, assign21310_e20680_d_n7, assign21310_e20680_d_n11, assign21310_e20680_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
        let assign21310_e20676: f64 = (locals.var_fn241_calc_iq__etab).exp();
        let assign21310_e20677: f64 = (1.0 + assign21310_e20676);
        let assign21310_e20678: f64 = (assign21310_e20677).ln();
        (assign21310_e20678, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn3) / assign21310_e20677), ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn4) / assign21310_e20677), 0.0, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn12) / assign21310_e20677),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21310_e20680;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21310_e20680_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21310_e20680_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21310_e20680_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21310_e20680_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21310_e20680_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21310_e20680_d_n12;

        let (assign21320_e20698, assign21320_e20698_d_n2, assign21320_e20698_d_n3, assign21320_e20698_d_n4, assign21320_e20698_d_n7, assign21320_e20698_d_n11, assign21320_e20698_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21320_e20686: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21320_e20688: f64 = (assign21320_e20686 * locals.var_fn241_calc_iq__type);
        let assign21320_e20690: f64 = (assign21320_e20688 * locals.var_fn241_calc_iq__cb);
        let assign21320_e20692: f64 = (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21320_e20694: f64 = (assign21320_e20692 * locals.var_fn241_calc_iq__exparg);
        let assign21320_e20696: f64 = (assign21320_e20694 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21320_e20696, ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21320_e20688 * locals.var_fn241_calc_iq__cb_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21320_e20698;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21320_e20698_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21320_e20698_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21320_e20698_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21320_e20698_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21320_e20698_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21320_e20698_d_n12;

        let (assign21330_e20705, assign21330_e20705_d_n2, assign21330_e20705_d_n3, assign21330_e20705_d_n4, assign21330_e20705_d_n7, assign21330_e20705_d_n11, assign21330_e20705_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21330_e20705;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21330_e20705_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21330_e20705_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21330_e20705_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21330_e20705_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21330_e20705_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21330_e20705_d_n12;

        let (assign21340_e20712, assign21340_e20712_d_n2, assign21340_e20712_d_n3, assign21340_e20712_d_n4, assign21340_e20712_d_n7, assign21340_e20712_d_n11, assign21340_e20712_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21340_e20712;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21340_e20712_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21340_e20712_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21340_e20712_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21340_e20712_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21340_e20712_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21340_e20712_d_n12;

        let assign21350_e20715: f64 = if locals.var_fn241_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign21350_e20715;

        let (assign21360_e20731, assign21360_e20731_d_n2, assign21360_e20731_d_n4, assign21360_e20731_d_n7, assign21360_e20731_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21360_e20723: f64 = (p.p51 * 0.5);
        let assign21360_e20725: f64 = (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21360_e20726: f64 = (locals.var_fn241_calc_iq__vtof - assign21360_e20725);
        let assign21360_e20727: f64 = (locals.var_fn241_calc_iq__vgsin - assign21360_e20726);
        let assign21360_e20729: f64 = (assign21360_e20727 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21360_e20729, (locals.var_fn241_calc_iq__vgsin_dn2 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21360_e20727 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vgsin_dn7 / locals.var_fn241_calc_iq__two_n_phit0), (locals.var_fn241_calc_iq__vgsin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, locals.var_fn241_calc_iq__etags_dn12,)
    }
};
        locals.var_fn241_calc_iq__etags = assign21360_e20731;
        locals.var_fn241_calc_iq__etags_dn2 = assign21360_e20731_d_n2;
        locals.var_fn241_calc_iq__etags_dn4 = assign21360_e20731_d_n4;
        locals.var_fn241_calc_iq__etags_dn7 = assign21360_e20731_d_n7;
        locals.var_fn241_calc_iq__etags_dn12 = assign21360_e20731_d_n12;

        let assign21370_e20734: f64 = if locals.var_fn241_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign21370_e20734;

        let (assign21380_e20742, assign21380_e20742_d_n2, assign21380_e20742_d_n3, assign21380_e20742_d_n4, assign21380_e20742_d_n7, assign21380_e20742_d_n11, assign21380_e20742_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 != 0.0)) {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, 0.0, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, 0.0, locals.var_fn241_calc_iq__etags_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21380_e20742;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21380_e20742_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21380_e20742_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21380_e20742_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21380_e20742_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21380_e20742_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21380_e20742_d_n12;

        let assign21390_e20745: f64 = (-50.0);
        let assign21390_e20746: f64 = if locals.var_fn241_calc_iq__etags < assign21390_e20745 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign21390_e20746;

        let (assign21400_e20758, assign21400_e20758_d_n2, assign21400_e20758_d_n3, assign21400_e20758_d_n4, assign21400_e20758_d_n7, assign21400_e20758_d_n11, assign21400_e20758_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 != 0.0)) {
        let assign21400_e20756: f64 = (locals.var_fn241_calc_iq__etags).exp();
        (assign21400_e20756, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn2), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn4), (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn7), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21400_e20758;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21400_e20758_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21400_e20758_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21400_e20758_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21400_e20758_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21400_e20758_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21400_e20758_d_n12;

        let (assign21410_e20774, assign21410_e20774_d_n2, assign21410_e20774_d_n3, assign21410_e20774_d_n4, assign21410_e20774_d_n7, assign21410_e20774_d_n11, assign21410_e20774_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 == 0.0)) {
        let assign21410_e20770: f64 = (locals.var_fn241_calc_iq__etags).exp();
        let assign21410_e20771: f64 = (1.0 + assign21410_e20770);
        let assign21410_e20772: f64 = (assign21410_e20771).ln();
        (assign21410_e20772, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn2) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn4) / assign21410_e20771), ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn7) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn12) / assign21410_e20771),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21410_e20774;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21410_e20774_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21410_e20774_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21410_e20774_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21410_e20774_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21410_e20774_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21410_e20774_d_n12;

        let (assign21420_e20792, assign21420_e20792_d_n2, assign21420_e20792_d_n3, assign21420_e20792_d_n4, assign21420_e20792_d_n7, assign21420_e20792_d_n11, assign21420_e20792_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21420_e20780: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21420_e20782: f64 = (assign21420_e20780 * locals.var_fn241_calc_iq__type);
        let assign21420_e20784: f64 = (assign21420_e20782 * locals.var_fn241_calc_iq__cs);
        let assign21420_e20786: f64 = (assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21420_e20788: f64 = (assign21420_e20786 * locals.var_fn241_calc_iq__exparg);
        let assign21420_e20790: f64 = (assign21420_e20788 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21420_e20790, ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0_dn4) * locals.var_fn241_calc_iq__exparg) + (assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21420_e20792;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21420_e20792_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21420_e20792_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21420_e20792_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21420_e20792_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21420_e20792_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21420_e20792_d_n12;

        let (assign21430_e20799, assign21430_e20799_d_n2, assign21430_e20799_d_n3, assign21430_e20799_d_n4, assign21430_e20799_d_n7, assign21430_e20799_d_n11, assign21430_e20799_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21430_e20799;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21430_e20799_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21430_e20799_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21430_e20799_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21430_e20799_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21430_e20799_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21430_e20799_d_n12;

        let (assign21440_e20803, assign21440_e20803_d_n2, assign21440_e20803_d_n3, assign21440_e20803_d_n4, assign21440_e20803_d_n7, assign21440_e20803_d_n11, assign21440_e20803_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__idsout, locals.var_fn241_calc_iq__idsout_dn2, locals.var_fn241_calc_iq__idsout_dn3, locals.var_fn241_calc_iq__idsout_dn4, locals.var_fn241_calc_iq__idsout_dn7, locals.var_fn241_calc_iq__idsout_dn11, locals.var_fn241_calc_iq__idsout_dn12,)
    } else {
        (locals.var_fn241_calc_iq__return, locals.var_fn241_calc_iq__return_dn2, locals.var_fn241_calc_iq__return_dn3, locals.var_fn241_calc_iq__return_dn4, locals.var_fn241_calc_iq__return_dn7, locals.var_fn241_calc_iq__return_dn11, locals.var_fn241_calc_iq__return_dn12,)
    }
};
        locals.var_fn241_calc_iq__return = assign21440_e20803;
        locals.var_fn241_calc_iq__return_dn2 = assign21440_e20803_d_n2;
        locals.var_fn241_calc_iq__return_dn3 = assign21440_e20803_d_n3;
        locals.var_fn241_calc_iq__return_dn4 = assign21440_e20803_d_n4;
        locals.var_fn241_calc_iq__return_dn7 = assign21440_e20803_d_n7;
        locals.var_fn241_calc_iq__return_dn11 = assign21440_e20803_d_n11;
        locals.var_fn241_calc_iq__return_dn12 = assign21440_e20803_d_n12;

        let (assign21450_e20807, assign21450_e20807_d_n2, assign21450_e20807_d_n3, assign21450_e20807_d_n4, assign21450_e20807_d_n7, assign21450_e20807_d_n11, assign21450_e20807_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__idsout, locals.var_fn241_calc_iq__idsout_dn2, locals.var_fn241_calc_iq__idsout_dn3, locals.var_fn241_calc_iq__idsout_dn4, locals.var_fn241_calc_iq__idsout_dn7, locals.var_fn241_calc_iq__idsout_dn11, locals.var_fn241_calc_iq__idsout_dn12,)
    } else {
        (locals.var_idsfps3, locals.var_idsfps3_dn2, locals.var_idsfps3_dn3, locals.var_idsfps3_dn4, locals.var_idsfps3_dn7, locals.var_idsfps3_dn11, locals.var_idsfps3_dn12,)
    }
};
        locals.var_idsfps3 = assign21450_e20807;
        locals.var_idsfps3_dn2 = assign21450_e20807_d_n2;
        locals.var_idsfps3_dn3 = assign21450_e20807_d_n3;
        locals.var_idsfps3_dn4 = assign21450_e20807_d_n4;
        locals.var_idsfps3_dn7 = assign21450_e20807_d_n7;
        locals.var_idsfps3_dn11 = assign21450_e20807_d_n11;
        locals.var_idsfps3_dn12 = assign21450_e20807_d_n12;

        let (assign21460_e20811, assign21460_e20811_d_n2, assign21460_e20811_d_n4, assign21460_e20811_d_n7, assign21460_e20811_d_n11, assign21460_e20811_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    } else {
        (locals.var_qgsfps3, locals.var_qgsfps3_dn2, locals.var_qgsfps3_dn4, locals.var_qgsfps3_dn7, locals.var_qgsfps3_dn11, locals.var_qgsfps3_dn12,)
    }
};
        locals.var_qgsfps3 = assign21460_e20811;
        locals.var_qgsfps3_dn2 = assign21460_e20811_d_n2;
        locals.var_qgsfps3_dn4 = assign21460_e20811_d_n4;
        locals.var_qgsfps3_dn7 = assign21460_e20811_d_n7;
        locals.var_qgsfps3_dn11 = assign21460_e20811_d_n11;
        locals.var_qgsfps3_dn12 = assign21460_e20811_d_n12;

    }

    pub(super) fn stamp_transient_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21470_e20815, assign21470_e20815_d_n2, assign21470_e20815_d_n4, assign21470_e20815_d_n7, assign21470_e20815_d_n11, assign21470_e20815_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    } else {
        (locals.var_qgdfps3, locals.var_qgdfps3_dn2, locals.var_qgdfps3_dn4, locals.var_qgdfps3_dn7, locals.var_qgdfps3_dn11, locals.var_qgdfps3_dn12,)
    }
};
        locals.var_qgdfps3 = assign21470_e20815;
        locals.var_qgdfps3_dn2 = assign21470_e20815_d_n2;
        locals.var_qgdfps3_dn4 = assign21470_e20815_d_n4;
        locals.var_qgdfps3_dn7 = assign21470_e20815_d_n7;
        locals.var_qgdfps3_dn11 = assign21470_e20815_d_n11;
        locals.var_qgdfps3_dn12 = assign21470_e20815_d_n12;

        let (assign21480_e20819, assign21480_e20819_d_n2, assign21480_e20819_d_n3, assign21480_e20819_d_n4, assign21480_e20819_d_n7, assign21480_e20819_d_n11, assign21480_e20819_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    } else {
        (locals.var_qcfps3, locals.var_qcfps3_dn2, locals.var_qcfps3_dn3, locals.var_qcfps3_dn4, locals.var_qcfps3_dn7, locals.var_qcfps3_dn11, locals.var_qcfps3_dn12,)
    }
};
        locals.var_qcfps3 = assign21480_e20819;
        locals.var_qcfps3_dn2 = assign21480_e20819_d_n2;
        locals.var_qcfps3_dn3 = assign21480_e20819_d_n3;
        locals.var_qcfps3_dn4 = assign21480_e20819_d_n4;
        locals.var_qcfps3_dn7 = assign21480_e20819_d_n7;
        locals.var_qcfps3_dn11 = assign21480_e20819_d_n11;
        locals.var_qcfps3_dn12 = assign21480_e20819_d_n12;

        let (assign21490_e20823, assign21490_e20823_d_n2, assign21490_e20823_d_n3, assign21490_e20823_d_n4, assign21490_e20823_d_n7, assign21490_e20823_d_n11, assign21490_e20823_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    } else {
        (locals.var_qbfps3, locals.var_qbfps3_dn2, locals.var_qbfps3_dn3, locals.var_qbfps3_dn4, locals.var_qbfps3_dn7, locals.var_qbfps3_dn11, locals.var_qbfps3_dn12,)
    }
};
        locals.var_qbfps3 = assign21490_e20823;
        locals.var_qbfps3_dn2 = assign21490_e20823_d_n2;
        locals.var_qbfps3_dn3 = assign21490_e20823_d_n3;
        locals.var_qbfps3_dn4 = assign21490_e20823_d_n4;
        locals.var_qbfps3_dn7 = assign21490_e20823_d_n7;
        locals.var_qbfps3_dn11 = assign21490_e20823_d_n11;
        locals.var_qbfps3_dn12 = assign21490_e20823_d_n12;

        let (assign21500_e20827, assign21500_e20827_d_n2, assign21500_e20827_d_n3, assign21500_e20827_d_n4, assign21500_e20827_d_n7, assign21500_e20827_d_n11, assign21500_e20827_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    } else {
        (locals.var_qsfps3, locals.var_qsfps3_dn2, locals.var_qsfps3_dn3, locals.var_qsfps3_dn4, locals.var_qsfps3_dn7, locals.var_qsfps3_dn11, locals.var_qsfps3_dn12,)
    }
};
        locals.var_qsfps3 = assign21500_e20827;
        locals.var_qsfps3_dn2 = assign21500_e20827_d_n2;
        locals.var_qsfps3_dn3 = assign21500_e20827_d_n3;
        locals.var_qsfps3_dn4 = assign21500_e20827_d_n4;
        locals.var_qsfps3_dn7 = assign21500_e20827_d_n7;
        locals.var_qsfps3_dn11 = assign21500_e20827_d_n11;
        locals.var_qsfps3_dn12 = assign21500_e20827_d_n12;

        let (assign21530_e20839, assign21530_e20839_d_n2, assign21530_e20839_d_n3, assign21530_e20839_d_n4, assign21530_e20839_d_n7, assign21530_e20839_d_n11, assign21530_e20839_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__return, locals.var_fn241_calc_iq__return_dn2, locals.var_fn241_calc_iq__return_dn3, locals.var_fn241_calc_iq__return_dn4, locals.var_fn241_calc_iq__return_dn7, locals.var_fn241_calc_iq__return_dn11, locals.var_fn241_calc_iq__return_dn12,)
    } else {
        (locals.var_idsfps3, locals.var_idsfps3_dn2, locals.var_idsfps3_dn3, locals.var_idsfps3_dn4, locals.var_idsfps3_dn7, locals.var_idsfps3_dn11, locals.var_idsfps3_dn12,)
    }
};
        locals.var_idsfps3 = assign21530_e20839;
        locals.var_idsfps3_dn2 = assign21530_e20839_d_n2;
        locals.var_idsfps3_dn3 = assign21530_e20839_d_n3;
        locals.var_idsfps3_dn4 = assign21530_e20839_d_n4;
        locals.var_idsfps3_dn7 = assign21530_e20839_d_n7;
        locals.var_idsfps3_dn11 = assign21530_e20839_d_n11;
        locals.var_idsfps3_dn12 = assign21530_e20839_d_n12;

        let assign21540_e20842: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign21540_e20842;

        locals.var_idsfps4 = 0.0;
        locals.var_idsfps4_dn2 = 0.0;
        locals.var_idsfps4_dn3 = 0.0;
        locals.var_idsfps4_dn4 = 0.0;
        locals.var_idsfps4_dn7 = 0.0;
        locals.var_idsfps4_dn12 = 0.0;
        locals.var_idsfps4_dn13 = 0.0;

        locals.var_qgsfps4 = 0.0;
        locals.var_qgsfps4_dn2 = 0.0;
        locals.var_qgsfps4_dn4 = 0.0;
        locals.var_qgsfps4_dn7 = 0.0;
        locals.var_qgsfps4_dn12 = 0.0;
        locals.var_qgsfps4_dn13 = 0.0;

        locals.var_qgdfps4 = 0.0;
        locals.var_qgdfps4_dn2 = 0.0;
        locals.var_qgdfps4_dn4 = 0.0;
        locals.var_qgdfps4_dn7 = 0.0;
        locals.var_qgdfps4_dn12 = 0.0;
        locals.var_qgdfps4_dn13 = 0.0;

        locals.var_qcfps4 = 0.0;
        locals.var_qcfps4_dn2 = 0.0;
        locals.var_qcfps4_dn3 = 0.0;
        locals.var_qcfps4_dn4 = 0.0;
        locals.var_qcfps4_dn7 = 0.0;
        locals.var_qcfps4_dn12 = 0.0;
        locals.var_qcfps4_dn13 = 0.0;

        locals.var_qbfps4 = 0.0;
        locals.var_qbfps4_dn2 = 0.0;
        locals.var_qbfps4_dn3 = 0.0;
        locals.var_qbfps4_dn4 = 0.0;
        locals.var_qbfps4_dn7 = 0.0;
        locals.var_qbfps4_dn12 = 0.0;
        locals.var_qbfps4_dn13 = 0.0;

        locals.var_qsfps4 = 0.0;
        locals.var_qsfps4_dn2 = 0.0;
        locals.var_qsfps4_dn3 = 0.0;
        locals.var_qsfps4_dn4 = 0.0;
        locals.var_qsfps4_dn7 = 0.0;
        locals.var_qsfps4_dn12 = 0.0;
        locals.var_qsfps4_dn13 = 0.0;

        let assign21630_e20853: f64 = if p.p145 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign21630_e20853;

        let (assign21640_e20857, assign21640_e20857_d_n2, assign21640_e20857_d_n3, assign21640_e20857_d_n4, assign21640_e20857_d_n7, assign21640_e20857_d_n12, assign21640_e20857_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__return, locals.var_fn277_calc_iq__return_dn2, locals.var_fn277_calc_iq__return_dn3, locals.var_fn277_calc_iq__return_dn4, locals.var_fn277_calc_iq__return_dn7, locals.var_fn277_calc_iq__return_dn12, locals.var_fn277_calc_iq__return_dn13,)
    }
};
        locals.var_fn277_calc_iq__return = assign21640_e20857;
        locals.var_fn277_calc_iq__return_dn2 = assign21640_e20857_d_n2;
        locals.var_fn277_calc_iq__return_dn3 = assign21640_e20857_d_n3;
        locals.var_fn277_calc_iq__return_dn4 = assign21640_e20857_d_n4;
        locals.var_fn277_calc_iq__return_dn7 = assign21640_e20857_d_n7;
        locals.var_fn277_calc_iq__return_dn12 = assign21640_e20857_d_n12;
        locals.var_fn277_calc_iq__return_dn13 = assign21640_e20857_d_n13;

        let (assign21650_e20861, assign21650_e20861_d_n2, assign21650_e20861_d_n3, assign21650_e20861_d_n4, assign21650_e20861_d_n7, assign21650_e20861_d_n12, assign21650_e20861_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__idsout, locals.var_fn277_calc_iq__idsout_dn2, locals.var_fn277_calc_iq__idsout_dn3, locals.var_fn277_calc_iq__idsout_dn4, locals.var_fn277_calc_iq__idsout_dn7, locals.var_fn277_calc_iq__idsout_dn12, locals.var_fn277_calc_iq__idsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__idsout = assign21650_e20861;
        locals.var_fn277_calc_iq__idsout_dn2 = assign21650_e20861_d_n2;
        locals.var_fn277_calc_iq__idsout_dn3 = assign21650_e20861_d_n3;
        locals.var_fn277_calc_iq__idsout_dn4 = assign21650_e20861_d_n4;
        locals.var_fn277_calc_iq__idsout_dn7 = assign21650_e20861_d_n7;
        locals.var_fn277_calc_iq__idsout_dn12 = assign21650_e20861_d_n12;
        locals.var_fn277_calc_iq__idsout_dn13 = assign21650_e20861_d_n13;

        let (assign21660_e20865, assign21660_e20865_d_n2, assign21660_e20865_d_n4, assign21660_e20865_d_n7, assign21660_e20865_d_n12, assign21660_e20865_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgsout = assign21660_e20865;
        locals.var_fn277_calc_iq__qgsout_dn2 = assign21660_e20865_d_n2;
        locals.var_fn277_calc_iq__qgsout_dn4 = assign21660_e20865_d_n4;
        locals.var_fn277_calc_iq__qgsout_dn7 = assign21660_e20865_d_n7;
        locals.var_fn277_calc_iq__qgsout_dn12 = assign21660_e20865_d_n12;
        locals.var_fn277_calc_iq__qgsout_dn13 = assign21660_e20865_d_n13;

        let (assign21670_e20869, assign21670_e20869_d_n2, assign21670_e20869_d_n4, assign21670_e20869_d_n7, assign21670_e20869_d_n12, assign21670_e20869_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgdout = assign21670_e20869;
        locals.var_fn277_calc_iq__qgdout_dn2 = assign21670_e20869_d_n2;
        locals.var_fn277_calc_iq__qgdout_dn4 = assign21670_e20869_d_n4;
        locals.var_fn277_calc_iq__qgdout_dn7 = assign21670_e20869_d_n7;
        locals.var_fn277_calc_iq__qgdout_dn12 = assign21670_e20869_d_n12;
        locals.var_fn277_calc_iq__qgdout_dn13 = assign21670_e20869_d_n13;

        let (assign21680_e20873, assign21680_e20873_d_n2, assign21680_e20873_d_n3, assign21680_e20873_d_n4, assign21680_e20873_d_n7, assign21680_e20873_d_n12, assign21680_e20873_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign21680_e20873;
        locals.var_fn277_calc_iq__qcout_dn2 = assign21680_e20873_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign21680_e20873_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign21680_e20873_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign21680_e20873_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign21680_e20873_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign21680_e20873_d_n13;

        let (assign21690_e20877, assign21690_e20877_d_n2, assign21690_e20877_d_n3, assign21690_e20877_d_n4, assign21690_e20877_d_n7, assign21690_e20877_d_n12, assign21690_e20877_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign21690_e20877;
        locals.var_fn277_calc_iq__qbout_dn2 = assign21690_e20877_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign21690_e20877_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign21690_e20877_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign21690_e20877_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign21690_e20877_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign21690_e20877_d_n13;

        let (assign21700_e20881, assign21700_e20881_d_n2, assign21700_e20881_d_n3, assign21700_e20881_d_n4, assign21700_e20881_d_n7, assign21700_e20881_d_n12, assign21700_e20881_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign21700_e20881;
        locals.var_fn277_calc_iq__qsout_dn2 = assign21700_e20881_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign21700_e20881_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign21700_e20881_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign21700_e20881_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign21700_e20881_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign21700_e20881_d_n13;

        let (assign21710_e20885, assign21710_e20885_d_n4, assign21710_e20885_d_n12, assign21710_e20885_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign21710_e20885;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign21710_e20885_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign21710_e20885_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign21710_e20885_d_n13;

        let (assign21720_e20889, assign21720_e20889_d_n2, assign21720_e20889_d_n3, assign21720_e20889_d_n4, assign21720_e20889_d_n7, assign21720_e20889_d_n12, assign21720_e20889_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign21720_e20889;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign21720_e20889_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign21720_e20889_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign21720_e20889_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign21720_e20889_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign21720_e20889_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign21720_e20889_d_n13;

        let (assign21730_e20893, assign21730_e20893_d_n2, assign21730_e20893_d_n7, assign21730_e20893_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vgsin, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, locals.var_fn277_calc_iq__vgsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgsin = assign21730_e20893;
        locals.var_fn277_calc_iq__vgsin_dn2 = assign21730_e20893_d_n2;
        locals.var_fn277_calc_iq__vgsin_dn7 = assign21730_e20893_d_n7;
        locals.var_fn277_calc_iq__vgsin_dn13 = assign21730_e20893_d_n13;

        let (assign21740_e20897, assign21740_e20897_d_n12, assign21740_e20897_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vdsfps4, locals.var_vdsfps4_dn12, locals.var_vdsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vdsin, locals.var_fn277_calc_iq__vdsin_dn12, locals.var_fn277_calc_iq__vdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsin = assign21740_e20897;
        locals.var_fn277_calc_iq__vdsin_dn12 = assign21740_e20897_d_n12;
        locals.var_fn277_calc_iq__vdsin_dn13 = assign21740_e20897_d_n13;

        let (assign21750_e20901,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_fn277_calc_iq__qcbflag,)
    }
};
        locals.var_fn277_calc_iq__qcbflag = assign21750_e20901;

        let (assign21760_e20905, assign21760_e20905_d_n2, assign21760_e20905_d_n7, assign21760_e20905_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vcin, locals.var_fn277_calc_iq__vcin_dn2, locals.var_fn277_calc_iq__vcin_dn7, locals.var_fn277_calc_iq__vcin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vcin = assign21760_e20905;
        locals.var_fn277_calc_iq__vcin_dn2 = assign21760_e20905_d_n2;
        locals.var_fn277_calc_iq__vcin_dn7 = assign21760_e20905_d_n7;
        locals.var_fn277_calc_iq__vcin_dn13 = assign21760_e20905_d_n13;

        let (assign21770_e20909, assign21770_e20909_d_n3, assign21770_e20909_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vbfps4, locals.var_vbfps4_dn3, locals.var_vbfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vbin, locals.var_fn277_calc_iq__vbin_dn3, locals.var_fn277_calc_iq__vbin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vbin = assign21770_e20909;
        locals.var_fn277_calc_iq__vbin_dn3 = assign21770_e20909_d_n3;
        locals.var_fn277_calc_iq__vbin_dn13 = assign21770_e20909_d_n13;

        let (assign21780_e20913,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_fn277_calc_iq__qgsflag,)
    }
};
        locals.var_fn277_calc_iq__qgsflag = assign21780_e20913;

        let (assign21790_e20917, assign21790_e20917_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn277_calc_iq__tambin, locals.var_fn277_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tambin = assign21790_e20917;
        locals.var_fn277_calc_iq__tambin_dn4 = assign21790_e20917_d_n4;

        let (assign21800_e20921,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn277_calc_iq__tnomin,)
    }
};
        locals.var_fn277_calc_iq__tnomin = assign21800_e20921;

        let (assign21810_e20925, assign21810_e20925_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn277_calc_iq__phitin, locals.var_fn277_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn277_calc_iq__phitin = assign21810_e20925;
        locals.var_fn277_calc_iq__phitin_dn4 = assign21810_e20925_d_n4;

        let (assign21820_e20929,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn277_calc_iq__w,)
    }
};
        locals.var_fn277_calc_iq__w = assign21820_e20929;

        let (assign21830_e20933,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p145,)
    } else {
        (locals.var_fn277_calc_iq__lin,)
    }
};
        locals.var_fn277_calc_iq__lin = assign21830_e20933;

        let (assign21840_e20937, assign21840_e20937_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cgfps4t, locals.var_cgfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cgin, locals.var_fn277_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn277_calc_iq__cgin = assign21840_e20937;
        locals.var_fn277_calc_iq__cgin_dn4 = assign21840_e20937_d_n4;

        let (assign21850_e20941,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_fn277_calc_iq__cs,)
    }
};
        locals.var_fn277_calc_iq__cs = assign21850_e20941;

        let (assign21860_e20945, assign21860_e20945_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_ccfps4t, locals.var_ccfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cc, locals.var_fn277_calc_iq__cc_dn4,)
    }
};
        locals.var_fn277_calc_iq__cc = assign21860_e20945;
        locals.var_fn277_calc_iq__cc_dn4 = assign21860_e20945_d_n4;

        let (assign21870_e20949, assign21870_e20949_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cbfps4t, locals.var_cbfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cb, locals.var_fn277_calc_iq__cb_dn4,)
    }
};
        locals.var_fn277_calc_iq__cb = assign21870_e20949;
        locals.var_fn277_calc_iq__cb_dn4 = assign21870_e20949_d_n4;

        let (assign21880_e20953,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_fn277_calc_iq__vto,)
    }
};
        locals.var_fn277_calc_iq__vto = assign21880_e20953;

        let (assign21890_e20957,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p160,)
    } else {
        (locals.var_fn277_calc_iq__ss,)
    }
};
        locals.var_fn277_calc_iq__ss = assign21890_e20957;

        let (assign21900_e20961,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p159,)
    } else {
        (locals.var_fn277_calc_iq__delta1,)
    }
};
        locals.var_fn277_calc_iq__delta1 = assign21900_e20961;

        let (assign21910_e20965,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta2,)
    }
};
        locals.var_fn277_calc_iq__delta2 = assign21910_e20965;

        let (assign21920_e20969,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p161,)
    } else {
        (locals.var_fn277_calc_iq__nd,)
    }
};
        locals.var_fn277_calc_iq__nd = assign21920_e20969;

        let (assign21930_e20973,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p165,)
    } else {
        (locals.var_fn277_calc_iq__alpha,)
    }
};
        locals.var_fn277_calc_iq__alpha = assign21930_e20973;

        let (assign21940_e20977,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p156,)
    } else {
        (locals.var_fn277_calc_iq__vel0,)
    }
};
        locals.var_fn277_calc_iq__vel0 = assign21940_e20977;

        let (assign21950_e20981,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p157,)
    } else {
        (locals.var_fn277_calc_iq__mu0,)
    }
};
        locals.var_fn277_calc_iq__mu0 = assign21950_e20981;

        let (assign21960_e20985,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p158,)
    } else {
        (locals.var_fn277_calc_iq__beta,)
    }
};
        locals.var_fn277_calc_iq__beta = assign21960_e20985;

        let (assign21970_e20989,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p164,)
    } else {
        (locals.var_fn277_calc_iq__mtheta,)
    }
};
        locals.var_fn277_calc_iq__mtheta = assign21970_e20989;

        let (assign21980_e20993,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p163,)
    } else {
        (locals.var_fn277_calc_iq__vtheta,)
    }
};
        locals.var_fn277_calc_iq__vtheta = assign21980_e20993;

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21990_e20997,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p162,)
    } else {
        (locals.var_fn277_calc_iq__vtzeta,)
    }
};
        locals.var_fn277_calc_iq__vtzeta = assign21990_e20997;

        let (assign22000_e21001,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn277_calc_iq__dibsat,)
    }
};
        locals.var_fn277_calc_iq__dibsat = assign22000_e21001;

        let (assign22010_e21005,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn277_calc_iq__epsilon,)
    }
};
        locals.var_fn277_calc_iq__epsilon = assign22010_e21005;

        let (assign22020_e21009,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn277_calc_iq__vzeta,)
    }
};
        locals.var_fn277_calc_iq__vzeta = assign22020_e21009;

        let (assign22030_e21013,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn277_calc_iq__lambda,)
    }
};
        locals.var_fn277_calc_iq__lambda = assign22030_e21013;

        let (assign22040_e21017,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn277_calc_iq__ngf,)
    }
};
        locals.var_fn277_calc_iq__ngf = assign22040_e21017;

        let (assign22050_e21021,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn277_calc_iq__type,)
    }
};
        locals.var_fn277_calc_iq__type = assign22050_e21021;

        let (assign22060_e21025,) = {
    if (locals.var_guard276 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn277_calc_iq__trapfracdl,)
    }
};
        locals.var_fn277_calc_iq__trapfracdl = assign22060_e21025;

        let (assign22070_e21029, assign22070_e21029_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22070_e21029;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22070_e21029_d_n4;

        let (assign22080_e21033, assign22080_e21033_d_n12, assign22080_e21033_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22080_e21033;
        locals.var_fn277_calc_iq__delta_dn12 = assign22080_e21033_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22080_e21033_d_n13;

        let (assign22090_e21037, assign22090_e21037_d_n4, assign22090_e21037_d_n12, assign22090_e21037_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22090_e21037;
        locals.var_fn277_calc_iq__n_dn4 = assign22090_e21037_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22090_e21037_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22090_e21037_d_n13;

        let (assign22100_e21041, assign22100_e21041_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22100_e21041;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22100_e21041_d_n4;

        let (assign22110_e21045, assign22110_e21045_d_n12, assign22110_e21045_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22110_e21045;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22110_e21045_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22110_e21045_d_n13;

        let (assign22120_e21049, assign22120_e21049_d_n2, assign22120_e21049_d_n3, assign22120_e21049_d_n4, assign22120_e21049_d_n7, assign22120_e21049_d_n12, assign22120_e21049_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign22120_e21049;
        locals.var_fn277_calc_iq__ffs_dn2 = assign22120_e21049_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign22120_e21049_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign22120_e21049_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign22120_e21049_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign22120_e21049_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign22120_e21049_d_n13;

        let (assign22130_e21053, assign22130_e21053_d_n4, assign22130_e21053_d_n12, assign22130_e21053_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22130_e21053;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22130_e21053_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22130_e21053_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22130_e21053_d_n13;

        let (assign22140_e21057, assign22140_e21057_d_n4, assign22140_e21057_d_n12, assign22140_e21057_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22140_e21057;
        locals.var_fn277_calc_iq__qref_dn4 = assign22140_e21057_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22140_e21057_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22140_e21057_d_n13;

        let (assign22150_e21061, assign22150_e21061_d_n2, assign22150_e21061_d_n3, assign22150_e21061_d_n4, assign22150_e21061_d_n7, assign22150_e21061_d_n12, assign22150_e21061_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign22150_e21061;
        locals.var_fn277_calc_iq__etas_dn2 = assign22150_e21061_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign22150_e21061_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign22150_e21061_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign22150_e21061_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign22150_e21061_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign22150_e21061_d_n13;

        let (assign22160_e21065, assign22160_e21065_d_n2, assign22160_e21065_d_n3, assign22160_e21065_d_n4, assign22160_e21065_d_n7, assign22160_e21065_d_n12, assign22160_e21065_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign22160_e21065;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign22160_e21065_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign22160_e21065_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign22160_e21065_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign22160_e21065_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign22160_e21065_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign22160_e21065_d_n13;

        let (assign22170_e21069, assign22170_e21069_d_n2, assign22170_e21069_d_n3, assign22170_e21069_d_n4, assign22170_e21069_d_n7, assign22170_e21069_d_n12, assign22170_e21069_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign22170_e21069;
        locals.var_fn277_calc_iq__muf_dn2 = assign22170_e21069_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign22170_e21069_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign22170_e21069_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign22170_e21069_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign22170_e21069_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign22170_e21069_d_n13;

        let (assign22180_e21073, assign22180_e21073_d_n2, assign22180_e21073_d_n3, assign22180_e21073_d_n4, assign22180_e21073_d_n7, assign22180_e21073_d_n12, assign22180_e21073_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign22180_e21073;
        locals.var_fn277_calc_iq__vx_dn2 = assign22180_e21073_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign22180_e21073_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign22180_e21073_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign22180_e21073_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign22180_e21073_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign22180_e21073_d_n13;

        let (assign22190_e21077, assign22190_e21077_d_n2, assign22190_e21077_d_n3, assign22190_e21077_d_n4, assign22190_e21077_d_n7, assign22190_e21077_d_n12, assign22190_e21077_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vxf, locals.var_fn277_calc_iq__vxf_dn2, locals.var_fn277_calc_iq__vxf_dn3, locals.var_fn277_calc_iq__vxf_dn4, locals.var_fn277_calc_iq__vxf_dn7, locals.var_fn277_calc_iq__vxf_dn12, locals.var_fn277_calc_iq__vxf_dn13,)
    }
};
        locals.var_fn277_calc_iq__vxf = assign22190_e21077;
        locals.var_fn277_calc_iq__vxf_dn2 = assign22190_e21077_d_n2;
        locals.var_fn277_calc_iq__vxf_dn3 = assign22190_e21077_d_n3;
        locals.var_fn277_calc_iq__vxf_dn4 = assign22190_e21077_d_n4;
        locals.var_fn277_calc_iq__vxf_dn7 = assign22190_e21077_d_n7;
        locals.var_fn277_calc_iq__vxf_dn12 = assign22190_e21077_d_n12;
        locals.var_fn277_calc_iq__vxf_dn13 = assign22190_e21077_d_n13;

        let (assign22200_e21081, assign22200_e21081_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n0, locals.var_fn277_calc_iq__n0_dn4,)
    }
};
        locals.var_fn277_calc_iq__n0 = assign22200_e21081;
        locals.var_fn277_calc_iq__n0_dn4 = assign22200_e21081_d_n4;

        let (assign22210_e21085, assign22210_e21085_d_n2, assign22210_e21085_d_n4, assign22210_e21085_d_n7, assign22210_e21085_d_n12, assign22210_e21085_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign22210_e21085;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign22210_e21085_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign22210_e21085_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign22210_e21085_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign22210_e21085_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign22210_e21085_d_n13;

        let (assign22220_e21089, assign22220_e21089_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit0, locals.var_fn277_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit0 = assign22220_e21089;
        locals.var_fn277_calc_iq__two_n_phit0_dn4 = assign22220_e21089_d_n4;

        let (assign22230_e21093, assign22230_e21093_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref0, locals.var_fn277_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn277_calc_iq__qref0 = assign22230_e21093;
        locals.var_fn277_calc_iq__qref0_dn4 = assign22230_e21093_d_n4;

        let (assign22240_e21097, assign22240_e21097_d_n2, assign22240_e21097_d_n4, assign22240_e21097_d_n7, assign22240_e21097_d_n12, assign22240_e21097_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas0, locals.var_fn277_calc_iq__etas0_dn2, locals.var_fn277_calc_iq__etas0_dn4, locals.var_fn277_calc_iq__etas0_dn7, locals.var_fn277_calc_iq__etas0_dn12, locals.var_fn277_calc_iq__etas0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas0 = assign22240_e21097;
        locals.var_fn277_calc_iq__etas0_dn2 = assign22240_e21097_d_n2;
        locals.var_fn277_calc_iq__etas0_dn4 = assign22240_e21097_d_n4;
        locals.var_fn277_calc_iq__etas0_dn7 = assign22240_e21097_d_n7;
        locals.var_fn277_calc_iq__etas0_dn12 = assign22240_e21097_d_n12;
        locals.var_fn277_calc_iq__etas0_dn13 = assign22240_e21097_d_n13;

        let (assign22250_e21101, assign22250_e21101_d_n2, assign22250_e21101_d_n4, assign22250_e21101_d_n7, assign22250_e21101_d_n12, assign22250_e21101_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign22250_e21101;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign22250_e21101_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign22250_e21101_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign22250_e21101_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign22250_e21101_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign22250_e21101_d_n13;

        let (assign22260_e21105, assign22260_e21105_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf0, locals.var_fn277_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn277_calc_iq__muf0 = assign22260_e21105;
        locals.var_fn277_calc_iq__muf0_dn4 = assign22260_e21105_d_n4;

        let (assign22270_e21109, assign22270_e21109_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx0, locals.var_fn277_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vx0 = assign22270_e21109;
        locals.var_fn277_calc_iq__vx0_dn4 = assign22270_e21109_d_n4;

        let (assign22280_e21113, assign22280_e21113_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22280_e21113;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22280_e21113_d_n4;

        let (assign22290_e21117, assign22290_e21117_d_n2, assign22290_e21117_d_n3, assign22290_e21117_d_n4, assign22290_e21117_d_n7, assign22290_e21117_d_n12, assign22290_e21117_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22290_e21117;
        locals.var_fn277_calc_iq__ff_dn2 = assign22290_e21117_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22290_e21117_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22290_e21117_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22290_e21117_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22290_e21117_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22290_e21117_d_n13;

        let (assign22300_e21121, assign22300_e21121_d_n2, assign22300_e21121_d_n3, assign22300_e21121_d_n4, assign22300_e21121_d_n7, assign22300_e21121_d_n12, assign22300_e21121_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22300_e21121;
        locals.var_fn277_calc_iq__eta_dn2 = assign22300_e21121_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22300_e21121_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22300_e21121_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22300_e21121_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22300_e21121_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22300_e21121_d_n13;

        let (assign22310_e21125, assign22310_e21125_d_n2, assign22310_e21125_d_n3, assign22310_e21125_d_n4, assign22310_e21125_d_n7, assign22310_e21125_d_n12, assign22310_e21125_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22310_e21125;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22310_e21125_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22310_e21125_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22310_e21125_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22310_e21125_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22310_e21125_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22310_e21125_d_n13;

        let (assign22320_e21129, assign22320_e21129_d_n2, assign22320_e21129_d_n4, assign22320_e21129_d_n7, assign22320_e21129_d_n12, assign22320_e21129_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign22320_e21129;
        locals.var_fn277_calc_iq__ff0_dn2 = assign22320_e21129_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign22320_e21129_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign22320_e21129_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign22320_e21129_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign22320_e21129_d_n13;

        let (assign22330_e21133, assign22330_e21133_d_n2, assign22330_e21133_d_n4, assign22330_e21133_d_n7, assign22330_e21133_d_n12, assign22330_e21133_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta0, locals.var_fn277_calc_iq__eta0_dn2, locals.var_fn277_calc_iq__eta0_dn4, locals.var_fn277_calc_iq__eta0_dn7, locals.var_fn277_calc_iq__eta0_dn12, locals.var_fn277_calc_iq__eta0_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta0 = assign22330_e21133;
        locals.var_fn277_calc_iq__eta0_dn2 = assign22330_e21133_d_n2;
        locals.var_fn277_calc_iq__eta0_dn4 = assign22330_e21133_d_n4;
        locals.var_fn277_calc_iq__eta0_dn7 = assign22330_e21133_d_n7;
        locals.var_fn277_calc_iq__eta0_dn12 = assign22330_e21133_d_n12;
        locals.var_fn277_calc_iq__eta0_dn13 = assign22330_e21133_d_n13;

        let (assign22340_e21137, assign22340_e21137_d_n2, assign22340_e21137_d_n4, assign22340_e21137_d_n7, assign22340_e21137_d_n12, assign22340_e21137_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign22340_e21137;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign22340_e21137_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign22340_e21137_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign22340_e21137_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign22340_e21137_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign22340_e21137_d_n13;

        let (assign22350_e21141, assign22350_e21141_d_n2, assign22350_e21141_d_n3, assign22350_e21141_d_n4, assign22350_e21141_d_n7, assign22350_e21141_d_n12, assign22350_e21141_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign22350_e21141;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign22350_e21141_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign22350_e21141_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign22350_e21141_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign22350_e21141_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign22350_e21141_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign22350_e21141_d_n13;

        let (assign22360_e21145, assign22360_e21145_d_n2, assign22360_e21145_d_n3, assign22360_e21145_d_n4, assign22360_e21145_d_n7, assign22360_e21145_d_n12, assign22360_e21145_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign22360_e21145;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign22360_e21145_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign22360_e21145_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign22360_e21145_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign22360_e21145_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign22360_e21145_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign22360_e21145_d_n13;

        let (assign22370_e21149, assign22370_e21149_d_n2, assign22370_e21149_d_n3, assign22370_e21149_d_n4, assign22370_e21149_d_n7, assign22370_e21149_d_n12, assign22370_e21149_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign22370_e21149;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign22370_e21149_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign22370_e21149_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign22370_e21149_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign22370_e21149_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign22370_e21149_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign22370_e21149_d_n13;

        let (assign22380_e21153, assign22380_e21153_d_n2, assign22380_e21153_d_n3, assign22380_e21153_d_n4, assign22380_e21153_d_n7, assign22380_e21153_d_n12, assign22380_e21153_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign22380_e21153;
        locals.var_fn277_calc_iq__fsd_dn2 = assign22380_e21153_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign22380_e21153_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign22380_e21153_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign22380_e21153_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign22380_e21153_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign22380_e21153_d_n13;

        let (assign22390_e21157, assign22390_e21157_d_n2, assign22390_e21157_d_n3, assign22390_e21157_d_n4, assign22390_e21157_d_n7, assign22390_e21157_d_n12, assign22390_e21157_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign22390_e21157;
        locals.var_fn277_calc_iq__vdx_dn2 = assign22390_e21157_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign22390_e21157_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign22390_e21157_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign22390_e21157_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign22390_e21157_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign22390_e21157_d_n13;

    }

    pub(super) fn stamp_transient_block_61(
        locals: &mut StampLocals,
    ) {
        let (assign22400_e21161, assign22400_e21161_d_n2, assign22400_e21161_d_n3, assign22400_e21161_d_n4, assign22400_e21161_d_n7, assign22400_e21161_d_n12, assign22400_e21161_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign22400_e21161;
        locals.var_fn277_calc_iq__fds_dn2 = assign22400_e21161_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign22400_e21161_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign22400_e21161_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign22400_e21161_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign22400_e21161_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign22400_e21161_d_n13;

        let (assign22410_e21165, assign22410_e21165_d_n2, assign22410_e21165_d_n3, assign22410_e21165_d_n4, assign22410_e21165_d_n7, assign22410_e21165_d_n12, assign22410_e21165_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign22410_e21165;
        locals.var_fn277_calc_iq__vsx_dn2 = assign22410_e21165_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign22410_e21165_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign22410_e21165_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign22410_e21165_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign22410_e21165_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign22410_e21165_d_n13;

        let (assign22420_e21169, assign22420_e21169_d_n2, assign22420_e21169_d_n3, assign22420_e21169_d_n4, assign22420_e21169_d_n7, assign22420_e21169_d_n12, assign22420_e21169_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign22420_e21169;
        locals.var_fn277_calc_iq__ffd_dn2 = assign22420_e21169_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign22420_e21169_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign22420_e21169_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign22420_e21169_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign22420_e21169_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign22420_e21169_d_n13;

        let (assign22430_e21173, assign22430_e21173_d_n2, assign22430_e21173_d_n3, assign22430_e21173_d_n4, assign22430_e21173_d_n7, assign22430_e21173_d_n12, assign22430_e21173_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign22430_e21173;
        locals.var_fn277_calc_iq__etad_dn2 = assign22430_e21173_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign22430_e21173_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign22430_e21173_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign22430_e21173_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign22430_e21173_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign22430_e21173_d_n13;

        let (assign22440_e21177, assign22440_e21177_d_n2, assign22440_e21177_d_n3, assign22440_e21177_d_n4, assign22440_e21177_d_n7, assign22440_e21177_d_n12, assign22440_e21177_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign22440_e21177;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign22440_e21177_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign22440_e21177_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign22440_e21177_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign22440_e21177_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign22440_e21177_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign22440_e21177_d_n13;

        let (assign22450_e21181, assign22450_e21181_d_n2, assign22450_e21181_d_n3, assign22450_e21181_d_n4, assign22450_e21181_d_n7, assign22450_e21181_d_n12, assign22450_e21181_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsc, locals.var_fn277_calc_iq__vdsc_dn2, locals.var_fn277_calc_iq__vdsc_dn3, locals.var_fn277_calc_iq__vdsc_dn4, locals.var_fn277_calc_iq__vdsc_dn7, locals.var_fn277_calc_iq__vdsc_dn12, locals.var_fn277_calc_iq__vdsc_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsc = assign22450_e21181;
        locals.var_fn277_calc_iq__vdsc_dn2 = assign22450_e21181_d_n2;
        locals.var_fn277_calc_iq__vdsc_dn3 = assign22450_e21181_d_n3;
        locals.var_fn277_calc_iq__vdsc_dn4 = assign22450_e21181_d_n4;
        locals.var_fn277_calc_iq__vdsc_dn7 = assign22450_e21181_d_n7;
        locals.var_fn277_calc_iq__vdsc_dn12 = assign22450_e21181_d_n12;
        locals.var_fn277_calc_iq__vdsc_dn13 = assign22450_e21181_d_n13;

        let (assign22460_e21185, assign22460_e21185_d_n2, assign22460_e21185_d_n3, assign22460_e21185_d_n4, assign22460_e21185_d_n7, assign22460_e21185_d_n12, assign22460_e21185_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsat, locals.var_fn277_calc_iq__fsat_dn2, locals.var_fn277_calc_iq__fsat_dn3, locals.var_fn277_calc_iq__fsat_dn4, locals.var_fn277_calc_iq__fsat_dn7, locals.var_fn277_calc_iq__fsat_dn12, locals.var_fn277_calc_iq__fsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsat = assign22460_e21185;
        locals.var_fn277_calc_iq__fsat_dn2 = assign22460_e21185_d_n2;
        locals.var_fn277_calc_iq__fsat_dn3 = assign22460_e21185_d_n3;
        locals.var_fn277_calc_iq__fsat_dn4 = assign22460_e21185_d_n4;
        locals.var_fn277_calc_iq__fsat_dn7 = assign22460_e21185_d_n7;
        locals.var_fn277_calc_iq__fsat_dn12 = assign22460_e21185_d_n12;
        locals.var_fn277_calc_iq__fsat_dn13 = assign22460_e21185_d_n13;

        let (assign22470_e21189, assign22470_e21189_d_n2, assign22470_e21189_d_n3, assign22470_e21189_d_n4, assign22470_e21189_d_n7, assign22470_e21189_d_n12, assign22470_e21189_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vel, locals.var_fn277_calc_iq__vel_dn2, locals.var_fn277_calc_iq__vel_dn3, locals.var_fn277_calc_iq__vel_dn4, locals.var_fn277_calc_iq__vel_dn7, locals.var_fn277_calc_iq__vel_dn12, locals.var_fn277_calc_iq__vel_dn13,)
    }
};
        locals.var_fn277_calc_iq__vel = assign22470_e21189;
        locals.var_fn277_calc_iq__vel_dn2 = assign22470_e21189_d_n2;
        locals.var_fn277_calc_iq__vel_dn3 = assign22470_e21189_d_n3;
        locals.var_fn277_calc_iq__vel_dn4 = assign22470_e21189_d_n4;
        locals.var_fn277_calc_iq__vel_dn7 = assign22470_e21189_d_n7;
        locals.var_fn277_calc_iq__vel_dn12 = assign22470_e21189_d_n12;
        locals.var_fn277_calc_iq__vel_dn13 = assign22470_e21189_d_n13;

        let (assign22480_e21193, assign22480_e21193_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats0, locals.var_fn277_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vdsats0 = assign22480_e21193;
        locals.var_fn277_calc_iq__vdsats0_dn4 = assign22480_e21193_d_n4;

        let (assign22490_e21197, assign22490_e21197_d_n2, assign22490_e21197_d_n4, assign22490_e21197_d_n7, assign22490_e21197_d_n12, assign22490_e21197_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats10, locals.var_fn277_calc_iq__vdsats10_dn2, locals.var_fn277_calc_iq__vdsats10_dn4, locals.var_fn277_calc_iq__vdsats10_dn7, locals.var_fn277_calc_iq__vdsats10_dn12, locals.var_fn277_calc_iq__vdsats10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats10 = assign22490_e21197;
        locals.var_fn277_calc_iq__vdsats10_dn2 = assign22490_e21197_d_n2;
        locals.var_fn277_calc_iq__vdsats10_dn4 = assign22490_e21197_d_n4;
        locals.var_fn277_calc_iq__vdsats10_dn7 = assign22490_e21197_d_n7;
        locals.var_fn277_calc_iq__vdsats10_dn12 = assign22490_e21197_d_n12;
        locals.var_fn277_calc_iq__vdsats10_dn13 = assign22490_e21197_d_n13;

        let (assign22500_e21201, assign22500_e21201_d_n2, assign22500_e21201_d_n4, assign22500_e21201_d_n7, assign22500_e21201_d_n12, assign22500_e21201_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat10, locals.var_fn277_calc_iq__vdsat10_dn2, locals.var_fn277_calc_iq__vdsat10_dn4, locals.var_fn277_calc_iq__vdsat10_dn7, locals.var_fn277_calc_iq__vdsat10_dn12, locals.var_fn277_calc_iq__vdsat10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat10 = assign22500_e21201;
        locals.var_fn277_calc_iq__vdsat10_dn2 = assign22500_e21201_d_n2;
        locals.var_fn277_calc_iq__vdsat10_dn4 = assign22500_e21201_d_n4;
        locals.var_fn277_calc_iq__vdsat10_dn7 = assign22500_e21201_d_n7;
        locals.var_fn277_calc_iq__vdsat10_dn12 = assign22500_e21201_d_n12;
        locals.var_fn277_calc_iq__vdsat10_dn13 = assign22500_e21201_d_n13;

        let (assign22510_e21205, assign22510_e21205_d_n2, assign22510_e21205_d_n4, assign22510_e21205_d_n7, assign22510_e21205_d_n12, assign22510_e21205_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd0, locals.var_fn277_calc_iq__fsd0_dn2, locals.var_fn277_calc_iq__fsd0_dn4, locals.var_fn277_calc_iq__fsd0_dn7, locals.var_fn277_calc_iq__fsd0_dn12, locals.var_fn277_calc_iq__fsd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd0 = assign22510_e21205;
        locals.var_fn277_calc_iq__fsd0_dn2 = assign22510_e21205_d_n2;
        locals.var_fn277_calc_iq__fsd0_dn4 = assign22510_e21205_d_n4;
        locals.var_fn277_calc_iq__fsd0_dn7 = assign22510_e21205_d_n7;
        locals.var_fn277_calc_iq__fsd0_dn12 = assign22510_e21205_d_n12;
        locals.var_fn277_calc_iq__fsd0_dn13 = assign22510_e21205_d_n13;

        let (assign22520_e21209, assign22520_e21209_d_n2, assign22520_e21209_d_n4, assign22520_e21209_d_n7, assign22520_e21209_d_n12, assign22520_e21209_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx0, locals.var_fn277_calc_iq__vdx0_dn2, locals.var_fn277_calc_iq__vdx0_dn4, locals.var_fn277_calc_iq__vdx0_dn7, locals.var_fn277_calc_iq__vdx0_dn12, locals.var_fn277_calc_iq__vdx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx0 = assign22520_e21209;
        locals.var_fn277_calc_iq__vdx0_dn2 = assign22520_e21209_d_n2;
        locals.var_fn277_calc_iq__vdx0_dn4 = assign22520_e21209_d_n4;
        locals.var_fn277_calc_iq__vdx0_dn7 = assign22520_e21209_d_n7;
        locals.var_fn277_calc_iq__vdx0_dn12 = assign22520_e21209_d_n12;
        locals.var_fn277_calc_iq__vdx0_dn13 = assign22520_e21209_d_n13;

        let (assign22530_e21213, assign22530_e21213_d_n2, assign22530_e21213_d_n4, assign22530_e21213_d_n7, assign22530_e21213_d_n12, assign22530_e21213_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds0, locals.var_fn277_calc_iq__fds0_dn2, locals.var_fn277_calc_iq__fds0_dn4, locals.var_fn277_calc_iq__fds0_dn7, locals.var_fn277_calc_iq__fds0_dn12, locals.var_fn277_calc_iq__fds0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds0 = assign22530_e21213;
        locals.var_fn277_calc_iq__fds0_dn2 = assign22530_e21213_d_n2;
        locals.var_fn277_calc_iq__fds0_dn4 = assign22530_e21213_d_n4;
        locals.var_fn277_calc_iq__fds0_dn7 = assign22530_e21213_d_n7;
        locals.var_fn277_calc_iq__fds0_dn12 = assign22530_e21213_d_n12;
        locals.var_fn277_calc_iq__fds0_dn13 = assign22530_e21213_d_n13;

        let (assign22540_e21217, assign22540_e21217_d_n2, assign22540_e21217_d_n4, assign22540_e21217_d_n7, assign22540_e21217_d_n12, assign22540_e21217_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx0, locals.var_fn277_calc_iq__vsx0_dn2, locals.var_fn277_calc_iq__vsx0_dn4, locals.var_fn277_calc_iq__vsx0_dn7, locals.var_fn277_calc_iq__vsx0_dn12, locals.var_fn277_calc_iq__vsx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx0 = assign22540_e21217;
        locals.var_fn277_calc_iq__vsx0_dn2 = assign22540_e21217_d_n2;
        locals.var_fn277_calc_iq__vsx0_dn4 = assign22540_e21217_d_n4;
        locals.var_fn277_calc_iq__vsx0_dn7 = assign22540_e21217_d_n7;
        locals.var_fn277_calc_iq__vsx0_dn12 = assign22540_e21217_d_n12;
        locals.var_fn277_calc_iq__vsx0_dn13 = assign22540_e21217_d_n13;

        let (assign22550_e21221, assign22550_e21221_d_n2, assign22550_e21221_d_n4, assign22550_e21221_d_n7, assign22550_e21221_d_n12, assign22550_e21221_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign22550_e21221;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign22550_e21221_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign22550_e21221_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign22550_e21221_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign22550_e21221_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign22550_e21221_d_n13;

        let (assign22560_e21225, assign22560_e21225_d_n2, assign22560_e21225_d_n4, assign22560_e21225_d_n7, assign22560_e21225_d_n12, assign22560_e21225_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad0, locals.var_fn277_calc_iq__etad0_dn2, locals.var_fn277_calc_iq__etad0_dn4, locals.var_fn277_calc_iq__etad0_dn7, locals.var_fn277_calc_iq__etad0_dn12, locals.var_fn277_calc_iq__etad0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad0 = assign22560_e21225;
        locals.var_fn277_calc_iq__etad0_dn2 = assign22560_e21225_d_n2;
        locals.var_fn277_calc_iq__etad0_dn4 = assign22560_e21225_d_n4;
        locals.var_fn277_calc_iq__etad0_dn7 = assign22560_e21225_d_n7;
        locals.var_fn277_calc_iq__etad0_dn12 = assign22560_e21225_d_n12;
        locals.var_fn277_calc_iq__etad0_dn13 = assign22560_e21225_d_n13;

        let (assign22570_e21229, assign22570_e21229_d_n2, assign22570_e21229_d_n4, assign22570_e21229_d_n7, assign22570_e21229_d_n12, assign22570_e21229_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign22570_e21229;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign22570_e21229_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign22570_e21229_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign22570_e21229_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign22570_e21229_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign22570_e21229_d_n13;

        let (assign22580_e21233, assign22580_e21233_d_n2, assign22580_e21233_d_n4, assign22580_e21233_d_n7, assign22580_e21233_d_n12, assign22580_e21233_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs2, locals.var_fn277_calc_iq__qs2_dn2, locals.var_fn277_calc_iq__qs2_dn4, locals.var_fn277_calc_iq__qs2_dn7, locals.var_fn277_calc_iq__qs2_dn12, locals.var_fn277_calc_iq__qs2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs2 = assign22580_e21233;
        locals.var_fn277_calc_iq__qs2_dn2 = assign22580_e21233_d_n2;
        locals.var_fn277_calc_iq__qs2_dn4 = assign22580_e21233_d_n4;
        locals.var_fn277_calc_iq__qs2_dn7 = assign22580_e21233_d_n7;
        locals.var_fn277_calc_iq__qs2_dn12 = assign22580_e21233_d_n12;
        locals.var_fn277_calc_iq__qs2_dn13 = assign22580_e21233_d_n13;

        let (assign22590_e21237, assign22590_e21237_d_n2, assign22590_e21237_d_n4, assign22590_e21237_d_n7, assign22590_e21237_d_n12, assign22590_e21237_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs3, locals.var_fn277_calc_iq__qs3_dn2, locals.var_fn277_calc_iq__qs3_dn4, locals.var_fn277_calc_iq__qs3_dn7, locals.var_fn277_calc_iq__qs3_dn12, locals.var_fn277_calc_iq__qs3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs3 = assign22590_e21237;
        locals.var_fn277_calc_iq__qs3_dn2 = assign22590_e21237_d_n2;
        locals.var_fn277_calc_iq__qs3_dn4 = assign22590_e21237_d_n4;
        locals.var_fn277_calc_iq__qs3_dn7 = assign22590_e21237_d_n7;
        locals.var_fn277_calc_iq__qs3_dn12 = assign22590_e21237_d_n12;
        locals.var_fn277_calc_iq__qs3_dn13 = assign22590_e21237_d_n13;

        let (assign22600_e21241, assign22600_e21241_d_n2, assign22600_e21241_d_n4, assign22600_e21241_d_n7, assign22600_e21241_d_n12, assign22600_e21241_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd2, locals.var_fn277_calc_iq__qd2_dn2, locals.var_fn277_calc_iq__qd2_dn4, locals.var_fn277_calc_iq__qd2_dn7, locals.var_fn277_calc_iq__qd2_dn12, locals.var_fn277_calc_iq__qd2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd2 = assign22600_e21241;
        locals.var_fn277_calc_iq__qd2_dn2 = assign22600_e21241_d_n2;
        locals.var_fn277_calc_iq__qd2_dn4 = assign22600_e21241_d_n4;
        locals.var_fn277_calc_iq__qd2_dn7 = assign22600_e21241_d_n7;
        locals.var_fn277_calc_iq__qd2_dn12 = assign22600_e21241_d_n12;
        locals.var_fn277_calc_iq__qd2_dn13 = assign22600_e21241_d_n13;

        let (assign22610_e21245, assign22610_e21245_d_n2, assign22610_e21245_d_n4, assign22610_e21245_d_n7, assign22610_e21245_d_n12, assign22610_e21245_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd3, locals.var_fn277_calc_iq__qd3_dn2, locals.var_fn277_calc_iq__qd3_dn4, locals.var_fn277_calc_iq__qd3_dn7, locals.var_fn277_calc_iq__qd3_dn12, locals.var_fn277_calc_iq__qd3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd3 = assign22610_e21245;
        locals.var_fn277_calc_iq__qd3_dn2 = assign22610_e21245_d_n2;
        locals.var_fn277_calc_iq__qd3_dn4 = assign22610_e21245_d_n4;
        locals.var_fn277_calc_iq__qd3_dn7 = assign22610_e21245_d_n7;
        locals.var_fn277_calc_iq__qd3_dn12 = assign22610_e21245_d_n12;
        locals.var_fn277_calc_iq__qd3_dn13 = assign22610_e21245_d_n13;

        let (assign22620_e21249, assign22620_e21249_d_n2, assign22620_e21249_d_n4, assign22620_e21249_d_n7, assign22620_e21249_d_n12, assign22620_e21249_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsqd, locals.var_fn277_calc_iq__qsqd_dn2, locals.var_fn277_calc_iq__qsqd_dn4, locals.var_fn277_calc_iq__qsqd_dn7, locals.var_fn277_calc_iq__qsqd_dn12, locals.var_fn277_calc_iq__qsqd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsqd = assign22620_e21249;
        locals.var_fn277_calc_iq__qsqd_dn2 = assign22620_e21249_d_n2;
        locals.var_fn277_calc_iq__qsqd_dn4 = assign22620_e21249_d_n4;
        locals.var_fn277_calc_iq__qsqd_dn7 = assign22620_e21249_d_n7;
        locals.var_fn277_calc_iq__qsqd_dn12 = assign22620_e21249_d_n12;
        locals.var_fn277_calc_iq__qsqd_dn13 = assign22620_e21249_d_n13;

        let (assign22630_e21253, assign22630_e21253_d_n2, assign22630_e21253_d_n4, assign22630_e21253_d_n7, assign22630_e21253_d_n12, assign22630_e21253_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvdd, locals.var_fn277_calc_iq__qinvdd_dn2, locals.var_fn277_calc_iq__qinvdd_dn4, locals.var_fn277_calc_iq__qinvdd_dn7, locals.var_fn277_calc_iq__qinvdd_dn12, locals.var_fn277_calc_iq__qinvdd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvdd = assign22630_e21253;
        locals.var_fn277_calc_iq__qinvdd_dn2 = assign22630_e21253_d_n2;
        locals.var_fn277_calc_iq__qinvdd_dn4 = assign22630_e21253_d_n4;
        locals.var_fn277_calc_iq__qinvdd_dn7 = assign22630_e21253_d_n7;
        locals.var_fn277_calc_iq__qinvdd_dn12 = assign22630_e21253_d_n12;
        locals.var_fn277_calc_iq__qinvdd_dn13 = assign22630_e21253_d_n13;

        let (assign22640_e21257, assign22640_e21257_d_n2, assign22640_e21257_d_n4, assign22640_e21257_d_n7, assign22640_e21257_d_n12, assign22640_e21257_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd1 = assign22640_e21257;
        locals.var_fn277_calc_iq__qd1_dn2 = assign22640_e21257_d_n2;
        locals.var_fn277_calc_iq__qd1_dn4 = assign22640_e21257_d_n4;
        locals.var_fn277_calc_iq__qd1_dn7 = assign22640_e21257_d_n7;
        locals.var_fn277_calc_iq__qd1_dn12 = assign22640_e21257_d_n12;
        locals.var_fn277_calc_iq__qd1_dn13 = assign22640_e21257_d_n13;

        let (assign22650_e21261, assign22650_e21261_d_n2, assign22650_e21261_d_n4, assign22650_e21261_d_n7, assign22650_e21261_d_n12, assign22650_e21261_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs, locals.var_fn277_calc_iq__qs_dn2, locals.var_fn277_calc_iq__qs_dn4, locals.var_fn277_calc_iq__qs_dn7, locals.var_fn277_calc_iq__qs_dn12, locals.var_fn277_calc_iq__qs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs = assign22650_e21261;
        locals.var_fn277_calc_iq__qs_dn2 = assign22650_e21261_d_n2;
        locals.var_fn277_calc_iq__qs_dn4 = assign22650_e21261_d_n4;
        locals.var_fn277_calc_iq__qs_dn7 = assign22650_e21261_d_n7;
        locals.var_fn277_calc_iq__qs_dn12 = assign22650_e21261_d_n12;
        locals.var_fn277_calc_iq__qs_dn13 = assign22650_e21261_d_n13;

        let (assign22660_e21265, assign22660_e21265_d_n2, assign22660_e21265_d_n4, assign22660_e21265_d_n7, assign22660_e21265_d_n12, assign22660_e21265_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd, locals.var_fn277_calc_iq__qd_dn2, locals.var_fn277_calc_iq__qd_dn4, locals.var_fn277_calc_iq__qd_dn7, locals.var_fn277_calc_iq__qd_dn12, locals.var_fn277_calc_iq__qd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd = assign22660_e21265;
        locals.var_fn277_calc_iq__qd_dn2 = assign22660_e21265_d_n2;
        locals.var_fn277_calc_iq__qd_dn4 = assign22660_e21265_d_n4;
        locals.var_fn277_calc_iq__qd_dn7 = assign22660_e21265_d_n7;
        locals.var_fn277_calc_iq__qd_dn12 = assign22660_e21265_d_n12;
        locals.var_fn277_calc_iq__qd_dn13 = assign22660_e21265_d_n13;

        let (assign22670_e21269, assign22670_e21269_d_n2, assign22670_e21269_d_n4, assign22670_e21269_d_n7, assign22670_e21269_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, locals.var_fn277_calc_iq__etac_dn13,)
    }
};
        locals.var_fn277_calc_iq__etac = assign22670_e21269;
        locals.var_fn277_calc_iq__etac_dn2 = assign22670_e21269_d_n2;
        locals.var_fn277_calc_iq__etac_dn4 = assign22670_e21269_d_n4;
        locals.var_fn277_calc_iq__etac_dn7 = assign22670_e21269_d_n7;
        locals.var_fn277_calc_iq__etac_dn13 = assign22670_e21269_d_n13;

        let (assign22680_e21273, assign22680_e21273_d_n3, assign22680_e21273_d_n4, assign22680_e21273_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etab, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, locals.var_fn277_calc_iq__etab_dn13,)
    }
};
        locals.var_fn277_calc_iq__etab = assign22680_e21273;
        locals.var_fn277_calc_iq__etab_dn3 = assign22680_e21273_d_n3;
        locals.var_fn277_calc_iq__etab_dn4 = assign22680_e21273_d_n4;
        locals.var_fn277_calc_iq__etab_dn13 = assign22680_e21273_d_n13;

        let (assign22690_e21277, assign22690_e21277_d_n2, assign22690_e21277_d_n4, assign22690_e21277_d_n7, assign22690_e21277_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, locals.var_fn277_calc_iq__etags_dn13,)
    }
};
        locals.var_fn277_calc_iq__etags = assign22690_e21277;
        locals.var_fn277_calc_iq__etags_dn2 = assign22690_e21277_d_n2;
        locals.var_fn277_calc_iq__etags_dn4 = assign22690_e21277_d_n4;
        locals.var_fn277_calc_iq__etags_dn7 = assign22690_e21277_d_n7;
        locals.var_fn277_calc_iq__etags_dn13 = assign22690_e21277_d_n13;

        let (assign22700_e21281, assign22700_e21281_d_n2, assign22700_e21281_d_n3, assign22700_e21281_d_n4, assign22700_e21281_d_n7, assign22700_e21281_d_n12, assign22700_e21281_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22700_e21281;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22700_e21281_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22700_e21281_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22700_e21281_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22700_e21281_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22700_e21281_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22700_e21281_d_n13;

        let (assign22710_e21285, assign22710_e21285_d_n2, assign22710_e21285_d_n3, assign22710_e21285_d_n4, assign22710_e21285_d_n7, assign22710_e21285_d_n12, assign22710_e21285_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22710_e21285;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22710_e21285_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22710_e21285_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22710_e21285_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22710_e21285_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22710_e21285_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22710_e21285_d_n13;

        let (assign22720_e21289, assign22720_e21289_d_n12, assign22720_e21289_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22720_e21289;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22720_e21289_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22720_e21289_d_n13;

        let (assign22730_e21293, assign22730_e21293_d_n2, assign22730_e21293_d_n7, assign22730_e21293_d_n12, assign22730_e21293_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22730_e21293;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22730_e21293_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22730_e21293_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22730_e21293_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22730_e21293_d_n13;

        let (assign22740_e21297, assign22740_e21297_d_n2, assign22740_e21297_d_n4, assign22740_e21297_d_n7, assign22740_e21297_d_n12, assign22740_e21297_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign22740_e21297;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign22740_e21297_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign22740_e21297_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign22740_e21297_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign22740_e21297_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign22740_e21297_d_n13;

        let (assign22750_e21301, assign22750_e21301_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg0, locals.var_fn277_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn277_calc_iq__myarg0 = assign22750_e21301;
        locals.var_fn277_calc_iq__myarg0_dn4 = assign22750_e21301_d_n4;

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22760_e21328, assign22760_e21328_d_n12, assign22760_e21328_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22760_e21310: f64 = (0.001 / p.p53);
                let assign22760_e21312: f64 = (assign22760_e21310 * locals.var_fn277_calc_iq__vdsin);
                let assign22760_e21313: f64 = (assign22760_e21312).tanh();
                let assign22760_e21314: f64 = (locals.var_fn277_calc_iq__vdsin * assign22760_e21313);
                (assign22760_e21314, ((locals.var_fn277_calc_iq__vdsin_dn12 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn12) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))), ((locals.var_fn277_calc_iq__vdsin_dn13 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn13) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))),)
            } else {
                let (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22760_e21320: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin);
                        let assign22760_e21322: f64 = (assign22760_e21320 + p.p53);
                        let assign22760_e21323: f64 = (assign22760_e21322).sqrt();
                        (assign22760_e21323, (((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn12)) / (2.0 * assign22760_e21323)), (((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn13)) / (2.0 * assign22760_e21323)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,)
            }
        };
        (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22760_e21328;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22760_e21328_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22760_e21328_d_n13;

        let (assign22770_e21334, assign22770_e21334_d_n2, assign22770_e21334_d_n7, assign22770_e21334_d_n12, assign22770_e21334_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22770_e21332: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdsin);
        (assign22770_e21332, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, (-locals.var_fn277_calc_iq__vdsin_dn12), (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22770_e21334;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22770_e21334_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22770_e21334_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22770_e21334_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22770_e21334_d_n13;

        let (assign22780_e21340, assign22780_e21340_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22780_e21338: f64 = (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin);
        (assign22780_e21338, (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22780_e21340;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22780_e21340_d_n4;

        let (assign22790_e21352, assign22790_e21352_d_n4, assign22790_e21352_d_n12, assign22790_e21352_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22790_e21345: f64 = (2.302585092994046 * locals.var_fn277_calc_iq__phitin);
        let assign22790_e21346: f64 = (locals.var_fn277_calc_iq__ss / assign22790_e21345);
        let assign22790_e21349: f64 = (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin);
        let assign22790_e21350: f64 = (assign22790_e21346 + assign22790_e21349);
        (assign22790_e21350, (-((locals.var_fn277_calc_iq__ss * (2.302585092994046 * locals.var_fn277_calc_iq__phitin_dn4)) / (assign22790_e21345 * assign22790_e21345))), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn12), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22790_e21352;
        locals.var_fn277_calc_iq__n_dn4 = assign22790_e21352_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22790_e21352_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22790_e21352_d_n13;

        let (assign22800_e21362, assign22800_e21362_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22800_e21358: f64 = (locals.var_fn277_calc_iq__tambin - locals.var_fn277_calc_iq__tnomin);
        let assign22800_e21359: f64 = (locals.var_fn277_calc_iq__vtzeta * assign22800_e21358);
        let assign22800_e21360: f64 = (locals.var_fn277_calc_iq__vto + assign22800_e21359);
        (assign22800_e21360, (locals.var_fn277_calc_iq__vtzeta * locals.var_fn277_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22800_e21362;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22800_e21362_d_n4;

        let (assign22810_e21370, assign22810_e21370_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22810_e21366: f64 = (locals.var_fn277_calc_iq__tambin / locals.var_fn277_calc_iq__tnomin);
        let assign22810_e21368: f64 = (assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon);
        (assign22810_e21368, if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn277_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__epsilon * ((assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon - 1.0) * (locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin))) } } else { (assign22810_e21368 * (locals.var_fn277_calc_iq__epsilon * ((locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin) / assign22810_e21366))) },)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22810_e21370;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22810_e21370_d_n4;

        let assign22820_e21373: f64 = if locals.var_fn277_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign22820_e21373;

        let (assign22830_e21391, assign22830_e21391_d_n12, assign22830_e21391_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 != 0.0)) {
        let assign22830_e21381: f64 = (locals.var_fn277_calc_iq__absvdsin / locals.var_fn277_calc_iq__dibsat);
        let assign22830_e21383: f64 = (assign22830_e21381).powf(locals.var_fn277_calc_iq__beta);
        let assign22830_e21384: f64 = (1.0 + assign22830_e21383);
        let assign22830_e21387: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign22830_e21388: f64 = (assign22830_e21384).powf(assign22830_e21387);
        let assign22830_e21389: f64 = (locals.var_fn277_calc_iq__absvdsin / assign22830_e21388);
        (assign22830_e21389, (((locals.var_fn277_calc_iq__absvdsin_dn12 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)), (((locals.var_fn277_calc_iq__absvdsin_dn13 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)),)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22830_e21391;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22830_e21391_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22830_e21391_d_n13;

        let (assign22840_e21398, assign22840_e21398_d_n12, assign22840_e21398_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22840_e21398;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22840_e21398_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22840_e21398_d_n13;

        let (assign22850_e21408, assign22850_e21408_d_n12, assign22850_e21408_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22850_e21403: f64 = (locals.var_fn277_calc_iq__vsatdibl * locals.var_fn277_calc_iq__delta2);
        let assign22850_e21404: f64 = (locals.var_fn277_calc_iq__delta1 - assign22850_e21403);
        let assign22850_e21406: f64 = (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin);
        (assign22850_e21406, (((-(locals.var_fn277_calc_iq__vsatdibl_dn12 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn12)), (((-(locals.var_fn277_calc_iq__vsatdibl_dn13 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22850_e21408;
        locals.var_fn277_calc_iq__delta_dn12 = assign22850_e21408_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22850_e21408_d_n13;

        let (assign22860_e21414, assign22860_e21414_d_n4, assign22860_e21414_d_n12, assign22860_e21414_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22860_e21412: f64 = (locals.var_fn277_calc_iq__vtof - locals.var_fn277_calc_iq__delta);
        (assign22860_e21412, locals.var_fn277_calc_iq__vtof_dn4, (-locals.var_fn277_calc_iq__delta_dn12), (-locals.var_fn277_calc_iq__delta_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign22860_e21414;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign22860_e21414_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign22860_e21414_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign22860_e21414_d_n13;

        let (assign22870_e21422, assign22870_e21422_d_n4, assign22870_e21422_d_n12, assign22870_e21422_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22870_e21418: f64 = (2.0 * locals.var_fn277_calc_iq__n);
        let assign22870_e21420: f64 = (assign22870_e21418 * locals.var_fn277_calc_iq__phitin);
        (assign22870_e21420, (((2.0 * locals.var_fn277_calc_iq__n_dn4) * locals.var_fn277_calc_iq__phitin) + (assign22870_e21418 * locals.var_fn277_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn277_calc_iq__n_dn12) * locals.var_fn277_calc_iq__phitin), ((2.0 * locals.var_fn277_calc_iq__n_dn13) * locals.var_fn277_calc_iq__phitin),)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22870_e21422;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22870_e21422_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22870_e21422_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22870_e21422_d_n13;

        let (assign22880_e21428, assign22880_e21428_d_n4, assign22880_e21428_d_n12, assign22880_e21428_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22880_e21426: f64 = (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit);
        (assign22880_e21426, ((locals.var_fn277_calc_iq__cgin_dn4 * locals.var_fn277_calc_iq__two_n_phit) + (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn4)), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn12), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22880_e21428;
        locals.var_fn277_calc_iq__qref_dn4 = assign22880_e21428_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22880_e21428_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22880_e21428_d_n13;

        let (assign22890_e21438, assign22890_e21438_d_n2, assign22890_e21438_d_n3, assign22890_e21438_d_n4, assign22890_e21438_d_n7, assign22890_e21438_d_n12, assign22890_e21438_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22890_e21433: f64 = (p.p51 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22890_e21435: f64 = (assign22890_e21433 / 2.0);
        let assign22890_e21436: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22890_e21435);
        (assign22890_e21436, 0.0, 0.0, (locals.var_fn277_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn277_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22890_e21438;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22890_e21438_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22890_e21438_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22890_e21438_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22890_e21438_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22890_e21438_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22890_e21438_d_n13;

        let (assign22900_e21489, assign22900_e21489_d_n2, assign22900_e21489_d_n3, assign22900_e21489_d_n4, assign22900_e21489_d_n7, assign22900_e21489_d_n12, assign22900_e21489_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22900_e21483, assign22900_e21483_d_n2, assign22900_e21483_d_n7, assign22900_e21483_d_n12, assign22900_e21483_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22900_e21447: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21450: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21453: f64 = (0.001 / p.p53);
                let assign22900_e21456: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21457: f64 = (assign22900_e21453 * assign22900_e21456);
                let assign22900_e21458: f64 = (assign22900_e21457).tanh();
                let assign22900_e21459: f64 = (assign22900_e21450 * assign22900_e21458);
                let assign22900_e21460: f64 = (assign22900_e21447 + assign22900_e21459);
                let assign22900_e21461: f64 = (0.5 * assign22900_e21460);
                (assign22900_e21461, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))),)
            } else {
                let (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22900_e21468: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21471: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21474: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21475: f64 = (assign22900_e21471 * assign22900_e21474);
                        let assign22900_e21477: f64 = (assign22900_e21475 + p.p53);
                        let assign22900_e21478: f64 = (assign22900_e21477).sqrt();
                        let assign22900_e21479: f64 = (assign22900_e21468 + assign22900_e21478);
                        let assign22900_e21480: f64 = (0.5 * assign22900_e21479);
                        (assign22900_e21480, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22900_e21478)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21474) + (assign22900_e21471 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22900_e21478)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,)
            }
        };
        let assign22900_e21485: f64 = (assign22900_e21483 - locals.var_fn277_calc_iq__myarg);
        let assign22900_e21487: f64 = (assign22900_e21485 / locals.var_fn277_calc_iq__alpha_phit);
        (assign22900_e21487, ((assign22900_e21483_d_n2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign22900_e21485 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((assign22900_e21483_d_n7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22900_e21489;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22900_e21489_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22900_e21489_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22900_e21489_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22900_e21489_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22900_e21489_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22900_e21489_d_n13;

        let assign22910_e21492: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign22910_e21492;

        let (assign22920_e21498, assign22920_e21498_d_n2, assign22920_e21498_d_n3, assign22920_e21498_d_n4, assign22920_e21498_d_n7, assign22920_e21498_d_n12, assign22920_e21498_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard279 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22920_e21498;
        locals.var_fn277_calc_iq__ff_dn2 = assign22920_e21498_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22920_e21498_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22920_e21498_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22920_e21498_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22920_e21498_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22920_e21498_d_n13;

        let assign22930_e21501: f64 = (-50.0);
        let assign22930_e21502: f64 = if locals.var_fn277_calc_iq__exparg < assign22930_e21501 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign22930_e21502;

        let (assign22940_e21511, assign22940_e21511_d_n2, assign22940_e21511_d_n3, assign22940_e21511_d_n4, assign22940_e21511_d_n7, assign22940_e21511_d_n12, assign22940_e21511_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22940_e21511;
        locals.var_fn277_calc_iq__ff_dn2 = assign22940_e21511_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22940_e21511_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22940_e21511_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22940_e21511_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22940_e21511_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22940_e21511_d_n13;

        let (assign22950_e21526, assign22950_e21526_d_n2, assign22950_e21526_d_n3, assign22950_e21526_d_n4, assign22950_e21526_d_n7, assign22950_e21526_d_n12, assign22950_e21526_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign22950_e21522: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign22950_e21523: f64 = (1.0 + assign22950_e21522);
        let assign22950_e21524: f64 = (1.0 / assign22950_e21523);
        (assign22950_e21524, (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn2) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn3) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn4) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn7) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn12) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn13) / (assign22950_e21523 * assign22950_e21523))),)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22950_e21526;
        locals.var_fn277_calc_iq__ff_dn2 = assign22950_e21526_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22950_e21526_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22950_e21526_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22950_e21526_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22950_e21526_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22950_e21526_d_n13;

        let (assign22960_e21585, assign22960_e21585_d_n2, assign22960_e21585_d_n3, assign22960_e21585_d_n4, assign22960_e21585_d_n7, assign22960_e21585_d_n12, assign22960_e21585_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22960_e21571, assign22960_e21571_d_n2, assign22960_e21571_d_n7, assign22960_e21571_d_n12, assign22960_e21571_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22960_e21535: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21538: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21541: f64 = (0.001 / p.p53);
                let assign22960_e21544: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21545: f64 = (assign22960_e21541 * assign22960_e21544);
                let assign22960_e21546: f64 = (assign22960_e21545).tanh();
                let assign22960_e21547: f64 = (assign22960_e21538 * assign22960_e21546);
                let assign22960_e21548: f64 = (assign22960_e21535 + assign22960_e21547);
                let assign22960_e21549: f64 = (0.5 * assign22960_e21548);
                (assign22960_e21549, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))),)
            } else {
                let (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22960_e21556: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21559: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21562: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21563: f64 = (assign22960_e21559 * assign22960_e21562);
                        let assign22960_e21565: f64 = (assign22960_e21563 + p.p53);
                        let assign22960_e21566: f64 = (assign22960_e21565).sqrt();
                        let assign22960_e21567: f64 = (assign22960_e21556 + assign22960_e21566);
                        let assign22960_e21568: f64 = (0.5 * assign22960_e21567);
                        (assign22960_e21568, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22960_e21566)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21562) + (assign22960_e21559 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22960_e21566)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,)
            }
        };
        let assign22960_e21575: f64 = (p.p51 * 0.1);
        let assign22960_e21577: f64 = (assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22960_e21579: f64 = (assign22960_e21577 * locals.var_fn277_calc_iq__ff);
        let assign22960_e21580: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22960_e21579);
        let assign22960_e21581: f64 = (assign22960_e21571 - assign22960_e21580);
        let assign22960_e21583: f64 = (assign22960_e21581 / locals.var_fn277_calc_iq__two_n_phit);
        (assign22960_e21583, ((assign22960_e21571_d_n2 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn2))) / locals.var_fn277_calc_iq__two_n_phit), ((-(-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn3))) / locals.var_fn277_calc_iq__two_n_phit), ((((-(locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ff) + (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((assign22960_e21571_d_n7 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn7))) / locals.var_fn277_calc_iq__two_n_phit), ((((assign22960_e21571_d_n12 - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((((assign22960_e21571_d_n13 - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22960_e21585;
        locals.var_fn277_calc_iq__eta_dn2 = assign22960_e21585_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22960_e21585_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22960_e21585_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22960_e21585_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22960_e21585_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22960_e21585_d_n13;

        let assign22970_e21588: f64 = if locals.var_fn277_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign22970_e21588;

        let (assign22980_e21596, assign22980_e21596_d_n2, assign22980_e21596_d_n3, assign22980_e21596_d_n4, assign22980_e21596_d_n7, assign22980_e21596_d_n12, assign22980_e21596_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign22980_e21594: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta);
        (assign22980_e21594, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22980_e21596;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22980_e21596_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22980_e21596_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22980_e21596_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22980_e21596_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22980_e21596_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22980_e21596_d_n13;

        let assign22990_e21599: f64 = (-50.0);
        let assign22990_e21600: f64 = if locals.var_fn277_calc_iq__eta < assign22990_e21599 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign22990_e21600;

        let (assign23000_e21612, assign23000_e21612_d_n2, assign23000_e21612_d_n3, assign23000_e21612_d_n4, assign23000_e21612_d_n7, assign23000_e21612_d_n12, assign23000_e21612_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign23000_e21609: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23000_e21610: f64 = (locals.var_fn277_calc_iq__qref * assign23000_e21609);
        (assign23000_e21610, (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn2)), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn4))), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23000_e21612;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23000_e21612_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23000_e21612_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23000_e21612_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23000_e21612_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23000_e21612_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23000_e21612_d_n13;

        let (assign23010_e21628, assign23010_e21628_d_n2, assign23010_e21628_d_n3, assign23010_e21628_d_n4, assign23010_e21628_d_n7, assign23010_e21628_d_n12, assign23010_e21628_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let assign23010_e21623: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23010_e21624: f64 = (1.0 + assign23010_e21623);
        let assign23010_e21625: f64 = (assign23010_e21624).ln();
        let assign23010_e21626: f64 = (locals.var_fn277_calc_iq__qref * assign23010_e21625);
        (assign23010_e21626, (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn2) / assign23010_e21624)), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn3) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn4) / assign23010_e21624))), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn7) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn12) / assign23010_e21624))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn13) / assign23010_e21624))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23010_e21628;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23010_e21628_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23010_e21628_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23010_e21628_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23010_e21628_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23010_e21628_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23010_e21628_d_n13;

        let (assign23020_e21642, assign23020_e21642_d_n2, assign23020_e21642_d_n3, assign23020_e21642_d_n4, assign23020_e21642_d_n7, assign23020_e21642_d_n12, assign23020_e21642_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23020_e21635: f64 = (locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23020_e21637: f64 = (assign23020_e21635 / locals.var_fn277_calc_iq__cgin);
        let assign23020_e21638: f64 = (1.0 + assign23020_e21637);
        let assign23020_e21639: f64 = (locals.var_fn277_calc_iq__tfacmobin * assign23020_e21638);
        let assign23020_e21640: f64 = (locals.var_fn277_calc_iq__mu0 / assign23020_e21639);
        (assign23020_e21640, (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * ((locals.var_fn277_calc_iq__tfacmobin_dn4 * assign23020_e21638) + (locals.var_fn277_calc_iq__tfacmobin * ((((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23020_e21635 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin))))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))),)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign23020_e21642;
        locals.var_fn277_calc_iq__muf_dn2 = assign23020_e21642_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign23020_e21642_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign23020_e21642_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign23020_e21642_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign23020_e21642_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign23020_e21642_d_n13;

        let (assign23030_e21674, assign23030_e21674_d_n2, assign23030_e21674_d_n3, assign23030_e21674_d_n4, assign23030_e21674_d_n7, assign23030_e21674_d_n12, assign23030_e21674_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23030_e21648: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tnomin);
        let assign23030_e21649: f64 = (1.0 + assign23030_e21648);
        let assign23030_e21653: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin);
        let assign23030_e21654: f64 = (1.0 + assign23030_e21653);
        let assign23030_e21655: f64 = (assign23030_e21649 / assign23030_e21654);
        let assign23030_e21656: f64 = (locals.var_fn277_calc_iq__vel0 * assign23030_e21655);
        let assign23030_e21660: f64 = (locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin);
        let assign23030_e21662: f64 = (assign23030_e21660 / locals.var_fn277_calc_iq__lin);
        let assign23030_e21663: f64 = (1.0 + assign23030_e21662);
        let assign23030_e21664: f64 = (assign23030_e21656 * assign23030_e21663);
        let assign23030_e21668: f64 = (locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23030_e21670: f64 = (assign23030_e21668 / locals.var_fn277_calc_iq__cgin);
        let assign23030_e21671: f64 = (1.0 + assign23030_e21670);
        let assign23030_e21672: f64 = (assign23030_e21664 / assign23030_e21671);
        (assign23030_e21672, (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (((((locals.var_fn277_calc_iq__vel0 * (-((assign23030_e21649 * (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin_dn4)) / (assign23030_e21654 * assign23030_e21654)))) * assign23030_e21663) * assign23030_e21671) - (assign23030_e21664 * ((((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23030_e21668 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)))) / (assign23030_e21671 * assign23030_e21671)), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn12) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn13) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)),)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign23030_e21674;
        locals.var_fn277_calc_iq__vx_dn2 = assign23030_e21674_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign23030_e21674_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign23030_e21674_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign23030_e21674_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign23030_e21674_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign23030_e21674_d_n13;

        let (assign23040_e21692, assign23040_e21692_d_n2, assign23040_e21692_d_n3, assign23040_e21692_d_n4, assign23040_e21692_d_n7, assign23040_e21692_d_n12, assign23040_e21692_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23040_e21678: f64 = (2.0 * locals.var_fn277_calc_iq__ff);
        let assign23040_e21680: f64 = (assign23040_e21678 * locals.var_fn277_calc_iq__phitin);
        let assign23040_e21682: f64 = (assign23040_e21680 * locals.var_fn277_calc_iq__muf);
        let assign23040_e21684: f64 = (assign23040_e21682 / locals.var_fn277_calc_iq__lin);
        let assign23040_e21687: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23040_e21689: f64 = (assign23040_e21687 * locals.var_fn277_calc_iq__vx);
        let assign23040_e21690: f64 = (assign23040_e21684 + assign23040_e21689);
        (assign23040_e21690, ((((((2.0 * locals.var_fn277_calc_iq__ff_dn2) * locals.var_fn277_calc_iq__phitin) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn2)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn2) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn277_calc_iq__ff_dn3) * locals.var_fn277_calc_iq__phitin) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn3)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn3) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn277_calc_iq__ff_dn4) * locals.var_fn277_calc_iq__phitin) + (assign23040_e21678 * locals.var_fn277_calc_iq__phitin_dn4)) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn4)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn4) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn277_calc_iq__ff_dn7) * locals.var_fn277_calc_iq__phitin) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn7)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn7) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn277_calc_iq__ff_dn12) * locals.var_fn277_calc_iq__phitin) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn12)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn12) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn12))), ((((((2.0 * locals.var_fn277_calc_iq__ff_dn13) * locals.var_fn277_calc_iq__phitin) * locals.var_fn277_calc_iq__muf) + (assign23040_e21680 * locals.var_fn277_calc_iq__muf_dn13)) / locals.var_fn277_calc_iq__lin) + (((-locals.var_fn277_calc_iq__ff_dn13) * locals.var_fn277_calc_iq__vx) + (assign23040_e21687 * locals.var_fn277_calc_iq__vx_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vxf, locals.var_fn277_calc_iq__vxf_dn2, locals.var_fn277_calc_iq__vxf_dn3, locals.var_fn277_calc_iq__vxf_dn4, locals.var_fn277_calc_iq__vxf_dn7, locals.var_fn277_calc_iq__vxf_dn12, locals.var_fn277_calc_iq__vxf_dn13,)
    }
};
        locals.var_fn277_calc_iq__vxf = assign23040_e21692;
        locals.var_fn277_calc_iq__vxf_dn2 = assign23040_e21692_d_n2;
        locals.var_fn277_calc_iq__vxf_dn3 = assign23040_e21692_d_n3;
        locals.var_fn277_calc_iq__vxf_dn4 = assign23040_e21692_d_n4;
        locals.var_fn277_calc_iq__vxf_dn7 = assign23040_e21692_d_n7;
        locals.var_fn277_calc_iq__vxf_dn12 = assign23040_e21692_d_n12;
        locals.var_fn277_calc_iq__vxf_dn13 = assign23040_e21692_d_n13;

        let (assign23050_e21700, assign23050_e21700_d_n2, assign23050_e21700_d_n3, assign23050_e21700_d_n4, assign23050_e21700_d_n7, assign23050_e21700_d_n12, assign23050_e21700_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23050_e21696: f64 = (locals.var_fn277_calc_iq__vx * locals.var_fn277_calc_iq__lin);
        let assign23050_e21698: f64 = (assign23050_e21696 / locals.var_fn277_calc_iq__muf);
        (assign23050_e21698, ((((locals.var_fn277_calc_iq__vx_dn2 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn2)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn3 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn3)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn4 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn4)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn7 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn7)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn12 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn12)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn13 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn13)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)),)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign23050_e21700;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign23050_e21700_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign23050_e21700_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign23050_e21700_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign23050_e21700_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign23050_e21700_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign23050_e21700_d_n13;

    }

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23060_e21717, assign23060_e21717_d_n2, assign23060_e21717_d_n3, assign23060_e21717_d_n4, assign23060_e21717_d_n7, assign23060_e21717_d_n12, assign23060_e21717_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23060_e21706: f64 = (2.0 * locals.var_fn277_calc_iq__qinvv);
        let assign23060_e21708: f64 = (assign23060_e21706 / locals.var_fn277_calc_iq__cgin);
        let assign23060_e21710: f64 = (assign23060_e21708 / locals.var_fn277_calc_iq__vdsats);
        let assign23060_e21711: f64 = (1.0 + assign23060_e21710);
        let assign23060_e21712: f64 = (assign23060_e21711).sqrt();
        let assign23060_e21713: f64 = (locals.var_fn277_calc_iq__vdsats * assign23060_e21712);
        let assign23060_e21715: f64 = (assign23060_e21713 - locals.var_fn277_calc_iq__vdsats);
        (assign23060_e21715, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn2)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn2), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn3)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn3), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23060_e21706 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn4)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn4), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn7)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn7), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn12)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn12), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn13)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign23060_e21717;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign23060_e21717_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign23060_e21717_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign23060_e21717_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign23060_e21717_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign23060_e21717_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign23060_e21717_d_n13;

        let (assign23070_e21729, assign23070_e21729_d_n2, assign23070_e21729_d_n3, assign23070_e21729_d_n4, assign23070_e21729_d_n7, assign23070_e21729_d_n12, assign23070_e21729_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23070_e21722: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23070_e21723: f64 = (locals.var_fn277_calc_iq__vdsats * assign23070_e21722);
        let assign23070_e21726: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23070_e21727: f64 = (assign23070_e21723 + assign23070_e21726);
        (assign23070_e21727, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign23070_e21729;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign23070_e21729_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign23070_e21729_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign23070_e21729_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign23070_e21729_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign23070_e21729_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign23070_e21729_d_n13;

        let (assign23080_e21741, assign23080_e21741_d_n2, assign23080_e21741_d_n3, assign23080_e21741_d_n4, assign23080_e21741_d_n7, assign23080_e21741_d_n12, assign23080_e21741_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23080_e21734: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23080_e21735: f64 = (locals.var_fn277_calc_iq__vdsats1 * assign23080_e21734);
        let assign23080_e21738: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23080_e21739: f64 = (assign23080_e21735 + assign23080_e21738);
        (assign23080_e21739, (((locals.var_fn277_calc_iq__vdsats1_dn2 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats1_dn3 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats1_dn4 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats1_dn7 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats1_dn12 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats1_dn13 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign23080_e21741;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign23080_e21741_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign23080_e21741_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign23080_e21741_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign23080_e21741_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign23080_e21741_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign23080_e21741_d_n13;

        let (assign23090_e21810, assign23090_e21810_d_n2, assign23090_e21810_d_n3, assign23090_e21810_d_n4, assign23090_e21810_d_n7, assign23090_e21810_d_n12, assign23090_e21810_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23090_e21800, assign23090_e21800_d_n2, assign23090_e21800_d_n3, assign23090_e21800_d_n4, assign23090_e21800_d_n7, assign23090_e21800_d_n12, assign23090_e21800_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23090_e21753: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21754: f64 = assign23090_e21753;
                let assign23090_e21758: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21759: f64 = (-assign23090_e21758);
                let assign23090_e21762: f64 = (0.001 / p.p53);
                let assign23090_e21766: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21767: f64 = (-assign23090_e21766);
                let assign23090_e21768: f64 = (assign23090_e21762 * assign23090_e21767);
                let assign23090_e21769: f64 = (assign23090_e21768).tanh();
                let assign23090_e21770: f64 = (assign23090_e21759 * assign23090_e21769);
                let assign23090_e21771: f64 = (assign23090_e21754 + assign23090_e21770);
                let assign23090_e21772: f64 = (0.5 * assign23090_e21771);
                (assign23090_e21772, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))),)
            } else {
                let (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23090_e21780: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21781: f64 = assign23090_e21780;
                        let assign23090_e21785: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21786: f64 = (-assign23090_e21785);
                        let assign23090_e21790: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21791: f64 = (-assign23090_e21790);
                        let assign23090_e21792: f64 = (assign23090_e21786 * assign23090_e21791);
                        let assign23090_e21794: f64 = (assign23090_e21792 + p.p53);
                        let assign23090_e21795: f64 = (assign23090_e21794).sqrt();
                        let assign23090_e21796: f64 = (assign23090_e21781 + assign23090_e21795);
                        let assign23090_e21797: f64 = (0.5 * assign23090_e21796);
                        (assign23090_e21797, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,)
            }
        };
        let assign23090_e21802: f64 = (assign23090_e21800).powf(locals.var_fn277_calc_iq__beta);
        let assign23090_e21803: f64 = (1.0 + assign23090_e21802);
        let assign23090_e21806: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23090_e21807: f64 = (assign23090_e21803).powf(assign23090_e21806);
        let assign23090_e21808: f64 = (1.0 / assign23090_e21807);
        (assign23090_e21808, (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))),)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign23090_e21810;
        locals.var_fn277_calc_iq__fsd_dn2 = assign23090_e21810_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign23090_e21810_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign23090_e21810_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign23090_e21810_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign23090_e21810_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign23090_e21810_d_n13;

        let (assign23100_e21816, assign23100_e21816_d_n2, assign23100_e21816_d_n3, assign23100_e21816_d_n4, assign23100_e21816_d_n7, assign23100_e21816_d_n12, assign23100_e21816_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23100_e21814: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd);
        (assign23100_e21814, (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn2), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn3), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn4), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn7), ((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn12)), ((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign23100_e21816;
        locals.var_fn277_calc_iq__vdx_dn2 = assign23100_e21816_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign23100_e21816_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign23100_e21816_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign23100_e21816_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign23100_e21816_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign23100_e21816_d_n13;

        let (assign23110_e21891, assign23110_e21891_d_n2, assign23110_e21891_d_n3, assign23110_e21891_d_n4, assign23110_e21891_d_n7, assign23110_e21891_d_n12, assign23110_e21891_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23110_e21881, assign23110_e21881_d_n2, assign23110_e21881_d_n3, assign23110_e21881_d_n4, assign23110_e21881_d_n7, assign23110_e21881_d_n12, assign23110_e21881_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23110_e21827: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21829: f64 = (assign23110_e21827 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21830: f64 = assign23110_e21829;
                let assign23110_e21833: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21835: f64 = (assign23110_e21833 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21836: f64 = (-assign23110_e21835);
                let assign23110_e21839: f64 = (0.001 / p.p53);
                let assign23110_e21842: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21844: f64 = (assign23110_e21842 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21845: f64 = (-assign23110_e21844);
                let assign23110_e21846: f64 = (assign23110_e21839 * assign23110_e21845);
                let assign23110_e21847: f64 = (assign23110_e21846).tanh();
                let assign23110_e21848: f64 = (assign23110_e21836 * assign23110_e21847);
                let assign23110_e21849: f64 = (assign23110_e21830 + assign23110_e21848);
                let assign23110_e21850: f64 = (0.5 * assign23110_e21849);
                (assign23110_e21850, (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))),)
            } else {
                let (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23110_e21857: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21859: f64 = (assign23110_e21857 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21860: f64 = assign23110_e21859;
                        let assign23110_e21863: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21865: f64 = (assign23110_e21863 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21866: f64 = (-assign23110_e21865);
                        let assign23110_e21869: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21871: f64 = (assign23110_e21869 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21872: f64 = (-assign23110_e21871);
                        let assign23110_e21873: f64 = (assign23110_e21866 * assign23110_e21872);
                        let assign23110_e21875: f64 = (assign23110_e21873 + p.p53);
                        let assign23110_e21876: f64 = (assign23110_e21875).sqrt();
                        let assign23110_e21877: f64 = (assign23110_e21860 + assign23110_e21876);
                        let assign23110_e21878: f64 = (0.5 * assign23110_e21877);
                        (assign23110_e21878, (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,)
            }
        };
        let assign23110_e21883: f64 = (assign23110_e21881).powf(locals.var_fn277_calc_iq__beta);
        let assign23110_e21884: f64 = (1.0 + assign23110_e21883);
        let assign23110_e21887: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23110_e21888: f64 = (assign23110_e21884).powf(assign23110_e21887);
        let assign23110_e21889: f64 = (1.0 / assign23110_e21888);
        (assign23110_e21889, (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))),)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign23110_e21891;
        locals.var_fn277_calc_iq__fds_dn2 = assign23110_e21891_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign23110_e21891_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign23110_e21891_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign23110_e21891_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign23110_e21891_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign23110_e21891_d_n13;

        let (assign23120_e21898, assign23120_e21898_d_n2, assign23120_e21898_d_n3, assign23120_e21898_d_n4, assign23120_e21898_d_n7, assign23120_e21898_d_n12, assign23120_e21898_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23120_e21894: f64 = (-locals.var_fn277_calc_iq__vdsin);
        let assign23120_e21896: f64 = (assign23120_e21894 * locals.var_fn277_calc_iq__fds);
        (assign23120_e21896, (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn2), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn3), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn4), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn7), (((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn12)), (((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign23120_e21898;
        locals.var_fn277_calc_iq__vsx_dn2 = assign23120_e21898_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign23120_e21898_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign23120_e21898_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign23120_e21898_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign23120_e21898_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign23120_e21898_d_n13;

        let (assign23130_e21906, assign23130_e21906_d_n2, assign23130_e21906_d_n3, assign23130_e21906_d_n4, assign23130_e21906_d_n7, assign23130_e21906_d_n12, assign23130_e21906_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23130_e21902: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__myarg);
        let assign23130_e21904: f64 = (assign23130_e21902 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23130_e21904, ((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23130_e21902 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23130_e21906;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23130_e21906_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23130_e21906_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23130_e21906_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23130_e21906_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23130_e21906_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23130_e21906_d_n13;

        let assign23140_e21909: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign23140_e21909;

        let (assign23150_e21915, assign23150_e21915_d_n2, assign23150_e21915_d_n3, assign23150_e21915_d_n4, assign23150_e21915_d_n7, assign23150_e21915_d_n12, assign23150_e21915_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard283 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23150_e21915;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23150_e21915_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23150_e21915_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23150_e21915_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23150_e21915_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23150_e21915_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23150_e21915_d_n13;

        let assign23160_e21918: f64 = (-50.0);
        let assign23160_e21919: f64 = if locals.var_fn277_calc_iq__exparg < assign23160_e21918 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign23160_e21919;

        let (assign23170_e21928, assign23170_e21928_d_n2, assign23170_e21928_d_n3, assign23170_e21928_d_n4, assign23170_e21928_d_n7, assign23170_e21928_d_n12, assign23170_e21928_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23170_e21928;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23170_e21928_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23170_e21928_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23170_e21928_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23170_e21928_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23170_e21928_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23170_e21928_d_n13;

        let (assign23180_e21943, assign23180_e21943_d_n2, assign23180_e21943_d_n3, assign23180_e21943_d_n4, assign23180_e21943_d_n7, assign23180_e21943_d_n12, assign23180_e21943_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign23180_e21939: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23180_e21940: f64 = (1.0 + assign23180_e21939);
        let assign23180_e21941: f64 = (1.0 / assign23180_e21940);
        (assign23180_e21941, (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23180_e21940 * assign23180_e21940))),)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23180_e21943;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23180_e21943_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23180_e21943_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23180_e21943_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23180_e21943_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23180_e21943_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23180_e21943_d_n13;

        let (assign23190_e21961, assign23190_e21961_d_n2, assign23190_e21961_d_n3, assign23190_e21961_d_n4, assign23190_e21961_d_n7, assign23190_e21961_d_n12, assign23190_e21961_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23190_e21947: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__vsx);
        let assign23190_e21951: f64 = (p.p51 * 0.1);
        let assign23190_e21953: f64 = (assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23190_e21955: f64 = (assign23190_e21953 * locals.var_fn277_calc_iq__ffs);
        let assign23190_e21956: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23190_e21955);
        let assign23190_e21957: f64 = (assign23190_e21947 - assign23190_e21956);
        let assign23190_e21959: f64 = (assign23190_e21957 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23190_e21959, (((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__vsx_dn2) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vsx_dn3) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vsx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffs) + (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__vsx_dn7) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__vsx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__vsx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign23190_e21961;
        locals.var_fn277_calc_iq__etas_dn2 = assign23190_e21961_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign23190_e21961_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign23190_e21961_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign23190_e21961_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign23190_e21961_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign23190_e21961_d_n13;

        let assign23200_e21964: f64 = if locals.var_fn277_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign23200_e21964;

        let (assign23210_e21972, assign23210_e21972_d_n2, assign23210_e21972_d_n3, assign23210_e21972_d_n4, assign23210_e21972_d_n7, assign23210_e21972_d_n12, assign23210_e21972_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard285 != 0.0)) {
        let assign23210_e21970: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas);
        (assign23210_e21970, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23210_e21972;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23210_e21972_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23210_e21972_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23210_e21972_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23210_e21972_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23210_e21972_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23210_e21972_d_n13;

        let assign23220_e21975: f64 = (-50.0);
        let assign23220_e21976: f64 = if locals.var_fn277_calc_iq__etas < assign23220_e21975 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign23220_e21976;

        let (assign23230_e21988, assign23230_e21988_d_n2, assign23230_e21988_d_n3, assign23230_e21988_d_n4, assign23230_e21988_d_n7, assign23230_e21988_d_n12, assign23230_e21988_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 != 0.0)) {
        let assign23230_e21985: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23230_e21986: f64 = (locals.var_fn277_calc_iq__qref * assign23230_e21985);
        (assign23230_e21986, (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn2)), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn4))), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23230_e21988;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23230_e21988_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23230_e21988_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23230_e21988_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23230_e21988_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23230_e21988_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23230_e21988_d_n13;

        let (assign23240_e22004, assign23240_e22004_d_n2, assign23240_e22004_d_n3, assign23240_e22004_d_n4, assign23240_e22004_d_n7, assign23240_e22004_d_n12, assign23240_e22004_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 == 0.0)) {
        let assign23240_e21999: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23240_e22000: f64 = (1.0 + assign23240_e21999);
        let assign23240_e22001: f64 = (assign23240_e22000).ln();
        let assign23240_e22002: f64 = (locals.var_fn277_calc_iq__qref * assign23240_e22001);
        (assign23240_e22002, (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn2) / assign23240_e22000)), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn3) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn4) / assign23240_e22000))), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn7) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn12) / assign23240_e22000))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn13) / assign23240_e22000))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23240_e22004;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23240_e22004_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23240_e22004_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23240_e22004_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23240_e22004_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23240_e22004_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23240_e22004_d_n13;

        let (assign23250_e22012, assign23250_e22012_d_n2, assign23250_e22012_d_n3, assign23250_e22012_d_n4, assign23250_e22012_d_n7, assign23250_e22012_d_n12, assign23250_e22012_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23250_e22008: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__myarg);
        let assign23250_e22010: f64 = (assign23250_e22008 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23250_e22010, ((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23250_e22008 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23250_e22012;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23250_e22012_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23250_e22012_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23250_e22012_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23250_e22012_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23250_e22012_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23250_e22012_d_n13;

        let assign23260_e22015: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign23260_e22015;

        let (assign23270_e22021, assign23270_e22021_d_n2, assign23270_e22021_d_n3, assign23270_e22021_d_n4, assign23270_e22021_d_n7, assign23270_e22021_d_n12, assign23270_e22021_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23270_e22021;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23270_e22021_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23270_e22021_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23270_e22021_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23270_e22021_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23270_e22021_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23270_e22021_d_n13;

        let assign23280_e22024: f64 = (-50.0);
        let assign23280_e22025: f64 = if locals.var_fn277_calc_iq__exparg < assign23280_e22024 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign23280_e22025;

        let (assign23290_e22034, assign23290_e22034_d_n2, assign23290_e22034_d_n3, assign23290_e22034_d_n4, assign23290_e22034_d_n7, assign23290_e22034_d_n12, assign23290_e22034_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23290_e22034;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23290_e22034_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23290_e22034_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23290_e22034_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23290_e22034_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23290_e22034_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23290_e22034_d_n13;

        let (assign23300_e22049, assign23300_e22049_d_n2, assign23300_e22049_d_n3, assign23300_e22049_d_n4, assign23300_e22049_d_n7, assign23300_e22049_d_n12, assign23300_e22049_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign23300_e22045: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23300_e22046: f64 = (1.0 + assign23300_e22045);
        let assign23300_e22047: f64 = (1.0 / assign23300_e22046);
        (assign23300_e22047, (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23300_e22046 * assign23300_e22046))),)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23300_e22049;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23300_e22049_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23300_e22049_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23300_e22049_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23300_e22049_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23300_e22049_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23300_e22049_d_n13;

        let (assign23310_e22067, assign23310_e22067_d_n2, assign23310_e22067_d_n3, assign23310_e22067_d_n4, assign23310_e22067_d_n7, assign23310_e22067_d_n12, assign23310_e22067_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23310_e22053: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdx);
        let assign23310_e22057: f64 = (p.p51 * 0.1);
        let assign23310_e22059: f64 = (assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23310_e22061: f64 = (assign23310_e22059 * locals.var_fn277_calc_iq__ffd);
        let assign23310_e22062: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23310_e22061);
        let assign23310_e22063: f64 = (assign23310_e22053 - assign23310_e22062);
        let assign23310_e22065: f64 = (assign23310_e22063 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23310_e22065, (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vdx_dn2) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vdx_dn3) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffd) + (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vdx_dn7) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign23310_e22067;
        locals.var_fn277_calc_iq__etad_dn2 = assign23310_e22067_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign23310_e22067_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign23310_e22067_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign23310_e22067_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign23310_e22067_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign23310_e22067_d_n13;

        let assign23320_e22070: f64 = if locals.var_fn277_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign23320_e22070;

        let (assign23330_e22078, assign23330_e22078_d_n2, assign23330_e22078_d_n3, assign23330_e22078_d_n4, assign23330_e22078_d_n7, assign23330_e22078_d_n12, assign23330_e22078_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard289 != 0.0)) {
        let assign23330_e22076: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad);
        (assign23330_e22076, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23330_e22078;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23330_e22078_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23330_e22078_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23330_e22078_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23330_e22078_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23330_e22078_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23330_e22078_d_n13;

        let assign23340_e22081: f64 = (-50.0);
        let assign23340_e22082: f64 = if locals.var_fn277_calc_iq__etad < assign23340_e22081 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign23340_e22082;

        let (assign23350_e22094, assign23350_e22094_d_n2, assign23350_e22094_d_n3, assign23350_e22094_d_n4, assign23350_e22094_d_n7, assign23350_e22094_d_n12, assign23350_e22094_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard289 == 0.0)) && (locals.var_guard290 != 0.0)) {
        let assign23350_e22091: f64 = (locals.var_fn277_calc_iq__etad).exp();
        let assign23350_e22092: f64 = (locals.var_fn277_calc_iq__qref * assign23350_e22091);
        (assign23350_e22092, (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn2)), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn4))), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23350_e22094;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23350_e22094_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23350_e22094_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23350_e22094_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23350_e22094_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23350_e22094_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23350_e22094_d_n13;

    }
}
