#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12000_e12414, assign12000_e12414_d_n2, assign12000_e12414_d_n3, assign12000_e12414_d_n4, assign12000_e12414_d_n5, assign12000_e12414_d_n7, assign12000_e12414_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard145 == 0.0)) && (locals.var_guard146 == 0.0)) {
        let assign12000_e12409: f64 = (locals.var_fn133_calc_iq__etad).exp();
        let assign12000_e12410: f64 = (1.0 + assign12000_e12409);
        let assign12000_e12411: f64 = (assign12000_e12410).ln();
        let assign12000_e12412: f64 = (locals.var_fn133_calc_iq__qref * assign12000_e12411);
        (assign12000_e12412, (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn2) / assign12000_e12410)), (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn3) / assign12000_e12410)), ((locals.var_fn133_calc_iq__qref_dn4 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn4) / assign12000_e12410))), ((locals.var_fn133_calc_iq__qref_dn5 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn5) / assign12000_e12410))), (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn7) / assign12000_e12410)), ((locals.var_fn133_calc_iq__qref_dn14 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn14) / assign12000_e12410))),)
    } else {
        (locals.var_fn133_calc_iq__qinvd, locals.var_fn133_calc_iq__qinvd_dn2, locals.var_fn133_calc_iq__qinvd_dn3, locals.var_fn133_calc_iq__qinvd_dn4, locals.var_fn133_calc_iq__qinvd_dn5, locals.var_fn133_calc_iq__qinvd_dn7, locals.var_fn133_calc_iq__qinvd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd = assign12000_e12414;
        locals.var_fn133_calc_iq__qinvd_dn2 = assign12000_e12414_d_n2;
        locals.var_fn133_calc_iq__qinvd_dn3 = assign12000_e12414_d_n3;
        locals.var_fn133_calc_iq__qinvd_dn4 = assign12000_e12414_d_n4;
        locals.var_fn133_calc_iq__qinvd_dn5 = assign12000_e12414_d_n5;
        locals.var_fn133_calc_iq__qinvd_dn7 = assign12000_e12414_d_n7;
        locals.var_fn133_calc_iq__qinvd_dn14 = assign12000_e12414_d_n14;

        let (assign12010_e12422, assign12010_e12422_d_n2, assign12010_e12422_d_n3, assign12010_e12422_d_n4, assign12010_e12422_d_n5, assign12010_e12422_d_n7, assign12010_e12422_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12010_e12418: f64 = (locals.var_fn133_calc_iq__qinvs - locals.var_fn133_calc_iq__qinvd);
        let assign12010_e12420: f64 = (assign12010_e12418 / locals.var_fn133_calc_iq__cgin);
        (assign12010_e12420, ((locals.var_fn133_calc_iq__qinvs_dn2 - locals.var_fn133_calc_iq__qinvd_dn2) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn3 - locals.var_fn133_calc_iq__qinvd_dn3) / locals.var_fn133_calc_iq__cgin), ((((locals.var_fn133_calc_iq__qinvs_dn4 - locals.var_fn133_calc_iq__qinvd_dn4) * locals.var_fn133_calc_iq__cgin) - (assign12010_e12418 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)), ((locals.var_fn133_calc_iq__qinvs_dn5 - locals.var_fn133_calc_iq__qinvd_dn5) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn7 - locals.var_fn133_calc_iq__qinvd_dn7) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn14 - locals.var_fn133_calc_iq__qinvd_dn14) / locals.var_fn133_calc_iq__cgin),)
    } else {
        (locals.var_fn133_calc_iq__vdsc, locals.var_fn133_calc_iq__vdsc_dn2, locals.var_fn133_calc_iq__vdsc_dn3, locals.var_fn133_calc_iq__vdsc_dn4, locals.var_fn133_calc_iq__vdsc_dn5, locals.var_fn133_calc_iq__vdsc_dn7, locals.var_fn133_calc_iq__vdsc_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsc = assign12010_e12422;
        locals.var_fn133_calc_iq__vdsc_dn2 = assign12010_e12422_d_n2;
        locals.var_fn133_calc_iq__vdsc_dn3 = assign12010_e12422_d_n3;
        locals.var_fn133_calc_iq__vdsc_dn4 = assign12010_e12422_d_n4;
        locals.var_fn133_calc_iq__vdsc_dn5 = assign12010_e12422_d_n5;
        locals.var_fn133_calc_iq__vdsc_dn7 = assign12010_e12422_d_n7;
        locals.var_fn133_calc_iq__vdsc_dn14 = assign12010_e12422_d_n14;

        let (assign12020_e12428, assign12020_e12428_d_n2, assign12020_e12428_d_n3, assign12020_e12428_d_n4, assign12020_e12428_d_n5, assign12020_e12428_d_n7, assign12020_e12428_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12020_e12426: f64 = (locals.var_fn133_calc_iq__vdsc / locals.var_fn133_calc_iq__vdsat);
        (assign12020_e12426, (((locals.var_fn133_calc_iq__vdsc_dn2 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn2)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn3 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn3)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn4 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn4)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn5 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn5)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn7 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn7)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn14 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn14)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)),)
    } else {
        (locals.var_fn133_calc_iq__myarg, locals.var_fn133_calc_iq__myarg_dn2, locals.var_fn133_calc_iq__myarg_dn3, locals.var_fn133_calc_iq__myarg_dn4, locals.var_fn133_calc_iq__myarg_dn5, locals.var_fn133_calc_iq__myarg_dn7, locals.var_fn133_calc_iq__myarg_dn14,)
    }
};
        locals.var_fn133_calc_iq__myarg = assign12020_e12428;
        locals.var_fn133_calc_iq__myarg_dn2 = assign12020_e12428_d_n2;
        locals.var_fn133_calc_iq__myarg_dn3 = assign12020_e12428_d_n3;
        locals.var_fn133_calc_iq__myarg_dn4 = assign12020_e12428_d_n4;
        locals.var_fn133_calc_iq__myarg_dn5 = assign12020_e12428_d_n5;
        locals.var_fn133_calc_iq__myarg_dn7 = assign12020_e12428_d_n7;
        locals.var_fn133_calc_iq__myarg_dn14 = assign12020_e12428_d_n14;

        let (assign12030_e12465, assign12030_e12465_d_n2, assign12030_e12465_d_n3, assign12030_e12465_d_n4, assign12030_e12465_d_n5, assign12030_e12465_d_n7, assign12030_e12465_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12030_e12455, assign12030_e12455_d_n2, assign12030_e12455_d_n3, assign12030_e12455_d_n4, assign12030_e12455_d_n5, assign12030_e12455_d_n7, assign12030_e12455_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12030_e12439: f64 = (0.001 / p.p53);
                let assign12030_e12441: f64 = (assign12030_e12439 * locals.var_fn133_calc_iq__myarg);
                let assign12030_e12442: f64 = (assign12030_e12441).tanh();
                let assign12030_e12443: f64 = (locals.var_fn133_calc_iq__myarg * assign12030_e12442);
                (assign12030_e12443, ((locals.var_fn133_calc_iq__myarg_dn2 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn2) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))), ((locals.var_fn133_calc_iq__myarg_dn3 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn3) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))), ((locals.var_fn133_calc_iq__myarg_dn4 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn4) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))), ((locals.var_fn133_calc_iq__myarg_dn5 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn5) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))), ((locals.var_fn133_calc_iq__myarg_dn7 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn7) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))), ((locals.var_fn133_calc_iq__myarg_dn14 * assign12030_e12442) + (locals.var_fn133_calc_iq__myarg * ((assign12030_e12439 * locals.var_fn133_calc_iq__myarg_dn14) / ((assign12030_e12441).cosh() * (assign12030_e12441).cosh())))),)
            } else {
                let (assign12030_e12454, assign12030_e12454_d_n2, assign12030_e12454_d_n3, assign12030_e12454_d_n4, assign12030_e12454_d_n5, assign12030_e12454_d_n7, assign12030_e12454_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12030_e12449: f64 = (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg);
                        let assign12030_e12451: f64 = (assign12030_e12449 + p.p53);
                        let assign12030_e12452: f64 = (assign12030_e12451).sqrt();
                        (assign12030_e12452, (((locals.var_fn133_calc_iq__myarg_dn2 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn2)) / (2.0 * assign12030_e12452)), (((locals.var_fn133_calc_iq__myarg_dn3 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn3)) / (2.0 * assign12030_e12452)), (((locals.var_fn133_calc_iq__myarg_dn4 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn4)) / (2.0 * assign12030_e12452)), (((locals.var_fn133_calc_iq__myarg_dn5 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn5)) / (2.0 * assign12030_e12452)), (((locals.var_fn133_calc_iq__myarg_dn7 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn7)) / (2.0 * assign12030_e12452)), (((locals.var_fn133_calc_iq__myarg_dn14 * locals.var_fn133_calc_iq__myarg) + (locals.var_fn133_calc_iq__myarg * locals.var_fn133_calc_iq__myarg_dn14)) / (2.0 * assign12030_e12452)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12030_e12454, assign12030_e12454_d_n2, assign12030_e12454_d_n3, assign12030_e12454_d_n4, assign12030_e12454_d_n5, assign12030_e12454_d_n7, assign12030_e12454_d_n14,)
            }
        };
        let assign12030_e12457: f64 = (assign12030_e12455).powf(locals.var_fn133_calc_iq__beta);
        let assign12030_e12458: f64 = (1.0 + assign12030_e12457);
        let assign12030_e12461: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign12030_e12462: f64 = (assign12030_e12458).powf(assign12030_e12461);
        let assign12030_e12463: f64 = (locals.var_fn133_calc_iq__myarg / assign12030_e12462);
        (assign12030_e12463, (((locals.var_fn133_calc_iq__myarg_dn2 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n2)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n2 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n2)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n2 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)), (((locals.var_fn133_calc_iq__myarg_dn3 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n3)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n3 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n3)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n3 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)), (((locals.var_fn133_calc_iq__myarg_dn4 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n4)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n4 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n4)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n4 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)), (((locals.var_fn133_calc_iq__myarg_dn5 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n5)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n5 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n5)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n5 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)), (((locals.var_fn133_calc_iq__myarg_dn7 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n7)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n7 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n7)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n7 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)), (((locals.var_fn133_calc_iq__myarg_dn14 * assign12030_e12462) - (locals.var_fn133_calc_iq__myarg * if 0.0 == 0.0 && ((assign12030_e12461) as f64).is_finite() && ((assign12030_e12461) as f64).fract() == 0.0 { if assign12030_e12461 == 0.0 { 0.0 } else { (assign12030_e12461 * ((assign12030_e12458).powf(assign12030_e12461 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n14)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n14 / assign12030_e12455))) })) } } else { (assign12030_e12462 * (assign12030_e12461 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12030_e12455).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12030_e12455_d_n14)) } } else { (assign12030_e12457 * (locals.var_fn133_calc_iq__beta * (assign12030_e12455_d_n14 / assign12030_e12455))) } / assign12030_e12458))) })) / (assign12030_e12462 * assign12030_e12462)),)
    } else {
        (locals.var_fn133_calc_iq__fsat, locals.var_fn133_calc_iq__fsat_dn2, locals.var_fn133_calc_iq__fsat_dn3, locals.var_fn133_calc_iq__fsat_dn4, locals.var_fn133_calc_iq__fsat_dn5, locals.var_fn133_calc_iq__fsat_dn7, locals.var_fn133_calc_iq__fsat_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsat = assign12030_e12465;
        locals.var_fn133_calc_iq__fsat_dn2 = assign12030_e12465_d_n2;
        locals.var_fn133_calc_iq__fsat_dn3 = assign12030_e12465_d_n3;
        locals.var_fn133_calc_iq__fsat_dn4 = assign12030_e12465_d_n4;
        locals.var_fn133_calc_iq__fsat_dn5 = assign12030_e12465_d_n5;
        locals.var_fn133_calc_iq__fsat_dn7 = assign12030_e12465_d_n7;
        locals.var_fn133_calc_iq__fsat_dn14 = assign12030_e12465_d_n14;

        let (assign12040_e12471, assign12040_e12471_d_n2, assign12040_e12471_d_n3, assign12040_e12471_d_n4, assign12040_e12471_d_n5, assign12040_e12471_d_n7, assign12040_e12471_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12040_e12469: f64 = (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat);
        (assign12040_e12469, ((locals.var_fn133_calc_iq__vxf_dn2 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn2)), ((locals.var_fn133_calc_iq__vxf_dn3 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn3)), ((locals.var_fn133_calc_iq__vxf_dn4 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn4)), ((locals.var_fn133_calc_iq__vxf_dn5 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn5)), ((locals.var_fn133_calc_iq__vxf_dn7 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn7)), ((locals.var_fn133_calc_iq__vxf_dn14 * locals.var_fn133_calc_iq__fsat) + (locals.var_fn133_calc_iq__vxf * locals.var_fn133_calc_iq__fsat_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vel, locals.var_fn133_calc_iq__vel_dn2, locals.var_fn133_calc_iq__vel_dn3, locals.var_fn133_calc_iq__vel_dn4, locals.var_fn133_calc_iq__vel_dn5, locals.var_fn133_calc_iq__vel_dn7, locals.var_fn133_calc_iq__vel_dn14,)
    }
};
        locals.var_fn133_calc_iq__vel = assign12040_e12471;
        locals.var_fn133_calc_iq__vel_dn2 = assign12040_e12471_d_n2;
        locals.var_fn133_calc_iq__vel_dn3 = assign12040_e12471_d_n3;
        locals.var_fn133_calc_iq__vel_dn4 = assign12040_e12471_d_n4;
        locals.var_fn133_calc_iq__vel_dn5 = assign12040_e12471_d_n5;
        locals.var_fn133_calc_iq__vel_dn7 = assign12040_e12471_d_n7;
        locals.var_fn133_calc_iq__vel_dn14 = assign12040_e12471_d_n14;

        let (assign12050_e12489, assign12050_e12489_d_n2, assign12050_e12489_d_n3, assign12050_e12489_d_n4, assign12050_e12489_d_n5, assign12050_e12489_d_n7, assign12050_e12489_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12050_e12475: f64 = (locals.var_fn133_calc_iq__type * locals.var_fn133_calc_iq__w);
        let assign12050_e12477: f64 = (assign12050_e12475 * locals.var_fn133_calc_iq__ngf);
        let assign12050_e12479: f64 = (assign12050_e12477 * 0.5);
        let assign12050_e12482: f64 = (locals.var_fn133_calc_iq__qinvs + locals.var_fn133_calc_iq__qinvd);
        let assign12050_e12483: f64 = (assign12050_e12479 * assign12050_e12482);
        let assign12050_e12485: f64 = (assign12050_e12483 * locals.var_fn133_calc_iq__vel);
        let assign12050_e12487: f64 = (assign12050_e12485 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12050_e12487, ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn2 + locals.var_fn133_calc_iq__qinvd_dn2)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn2)) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn3 + locals.var_fn133_calc_iq__qinvd_dn3)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn3)) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn4 + locals.var_fn133_calc_iq__qinvd_dn4)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn5 + locals.var_fn133_calc_iq__qinvd_dn5)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn5)) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn7 + locals.var_fn133_calc_iq__qinvd_dn7)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn7)) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12050_e12479 * (locals.var_fn133_calc_iq__qinvs_dn14 + locals.var_fn133_calc_iq__qinvd_dn14)) * locals.var_fn133_calc_iq__vel) + (assign12050_e12483 * locals.var_fn133_calc_iq__vel_dn14)) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__idsout, locals.var_fn133_calc_iq__idsout_dn2, locals.var_fn133_calc_iq__idsout_dn3, locals.var_fn133_calc_iq__idsout_dn4, locals.var_fn133_calc_iq__idsout_dn5, locals.var_fn133_calc_iq__idsout_dn7, locals.var_fn133_calc_iq__idsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__idsout = assign12050_e12489;
        locals.var_fn133_calc_iq__idsout_dn2 = assign12050_e12489_d_n2;
        locals.var_fn133_calc_iq__idsout_dn3 = assign12050_e12489_d_n3;
        locals.var_fn133_calc_iq__idsout_dn4 = assign12050_e12489_d_n4;
        locals.var_fn133_calc_iq__idsout_dn5 = assign12050_e12489_d_n5;
        locals.var_fn133_calc_iq__idsout_dn7 = assign12050_e12489_d_n7;
        locals.var_fn133_calc_iq__idsout_dn14 = assign12050_e12489_d_n14;

        let (assign12060_e12497, assign12060_e12497_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12060_e12494: f64 = (2.302585092994046 * locals.var_fn133_calc_iq__phitin);
        let assign12060_e12495: f64 = (locals.var_fn133_calc_iq__ss / assign12060_e12494);
        (assign12060_e12495, (-((locals.var_fn133_calc_iq__ss * (2.302585092994046 * locals.var_fn133_calc_iq__phitin_dn4)) / (assign12060_e12494 * assign12060_e12494))),)
    } else {
        (locals.var_fn133_calc_iq__n0, locals.var_fn133_calc_iq__n0_dn4,)
    }
};
        locals.var_fn133_calc_iq__n0 = assign12060_e12497;
        locals.var_fn133_calc_iq__n0_dn4 = assign12060_e12497_d_n4;

        let (assign12070_e12505, assign12070_e12505_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12070_e12501: f64 = (2.0 * locals.var_fn133_calc_iq__n0);
        let assign12070_e12503: f64 = (assign12070_e12501 * locals.var_fn133_calc_iq__phitin);
        (assign12070_e12503, (((2.0 * locals.var_fn133_calc_iq__n0_dn4) * locals.var_fn133_calc_iq__phitin) + (assign12070_e12501 * locals.var_fn133_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit0, locals.var_fn133_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit0 = assign12070_e12505;
        locals.var_fn133_calc_iq__two_n_phit0_dn4 = assign12070_e12505_d_n4;

        let (assign12080_e12511, assign12080_e12511_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12080_e12509: f64 = (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit0);
        (assign12080_e12509, ((locals.var_fn133_calc_iq__cgin_dn4 * locals.var_fn133_calc_iq__two_n_phit0) + (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn133_calc_iq__qref0, locals.var_fn133_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn133_calc_iq__qref0 = assign12080_e12511;
        locals.var_fn133_calc_iq__qref0_dn4 = assign12080_e12511_d_n4;

        let (assign12090_e12521, assign12090_e12521_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12090_e12516: f64 = (p.p51 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12090_e12518: f64 = (assign12090_e12516 / 2.0);
        let assign12090_e12519: f64 = (locals.var_fn133_calc_iq__vtof - assign12090_e12518);
        (assign12090_e12519, (locals.var_fn133_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn133_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn133_calc_iq__myarg0, locals.var_fn133_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn133_calc_iq__myarg0 = assign12090_e12521;
        locals.var_fn133_calc_iq__myarg0_dn4 = assign12090_e12521_d_n4;

        let (assign12100_e12572, assign12100_e12572_d_n2, assign12100_e12572_d_n4, assign12100_e12572_d_n5, assign12100_e12572_d_n7, assign12100_e12572_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12100_e12566, assign12100_e12566_d_n2, assign12100_e12566_d_n5, assign12100_e12566_d_n7, assign12100_e12566_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12100_e12530: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12533: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12536: f64 = (0.001 / p.p53);
                let assign12100_e12539: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12540: f64 = (assign12100_e12536 * assign12100_e12539);
                let assign12100_e12541: f64 = (assign12100_e12540).tanh();
                let assign12100_e12542: f64 = (assign12100_e12533 * assign12100_e12541);
                let assign12100_e12543: f64 = (assign12100_e12530 + assign12100_e12542);
                let assign12100_e12544: f64 = (0.5 * assign12100_e12543);
                (assign12100_e12544, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))),)
            } else {
                let (assign12100_e12565, assign12100_e12565_d_n2, assign12100_e12565_d_n5, assign12100_e12565_d_n7, assign12100_e12565_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12100_e12551: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12554: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12557: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12558: f64 = (assign12100_e12554 * assign12100_e12557);
                        let assign12100_e12560: f64 = (assign12100_e12558 + p.p53);
                        let assign12100_e12561: f64 = (assign12100_e12560).sqrt();
                        let assign12100_e12562: f64 = (assign12100_e12551 + assign12100_e12561);
                        let assign12100_e12563: f64 = (0.5 * assign12100_e12562);
                        (assign12100_e12563, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign12100_e12561)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign12100_e12561)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign12100_e12561)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12100_e12557) + (assign12100_e12554 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign12100_e12561)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12100_e12565, assign12100_e12565_d_n2, assign12100_e12565_d_n5, assign12100_e12565_d_n7, assign12100_e12565_d_n14,)
            }
        };
        let assign12100_e12568: f64 = (assign12100_e12566 - locals.var_fn133_calc_iq__myarg0);
        let assign12100_e12570: f64 = (assign12100_e12568 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12100_e12570, (assign12100_e12566_d_n2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12100_e12568 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (assign12100_e12566_d_n5 / locals.var_fn133_calc_iq__alpha_phit), (assign12100_e12566_d_n7 / locals.var_fn133_calc_iq__alpha_phit), (assign12100_e12566_d_n14 / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12100_e12572;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12100_e12572_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12100_e12572_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12100_e12572_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12100_e12572_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12100_e12572_d_n14;

        let assign12110_e12575: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign12110_e12575;

        let (assign12120_e12581, assign12120_e12581_d_n2, assign12120_e12581_d_n4, assign12120_e12581_d_n5, assign12120_e12581_d_n7, assign12120_e12581_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard147 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12120_e12581;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12120_e12581_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12120_e12581_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12120_e12581_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12120_e12581_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12120_e12581_d_n14;

        let assign12130_e12584: f64 = (-50.0);
        let assign12130_e12585: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12130_e12584 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign12130_e12585;

        let (assign12140_e12594, assign12140_e12594_d_n2, assign12140_e12594_d_n4, assign12140_e12594_d_n5, assign12140_e12594_d_n7, assign12140_e12594_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12140_e12594;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12140_e12594_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12140_e12594_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12140_e12594_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12140_e12594_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12140_e12594_d_n14;

        let (assign12150_e12609, assign12150_e12609_d_n2, assign12150_e12609_d_n4, assign12150_e12609_d_n5, assign12150_e12609_d_n7, assign12150_e12609_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign12150_e12605: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12150_e12606: f64 = (1.0 + assign12150_e12605);
        let assign12150_e12607: f64 = (1.0 / assign12150_e12606);
        (assign12150_e12607, (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12150_e12606 * assign12150_e12606))),)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12150_e12609;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12150_e12609_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12150_e12609_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12150_e12609_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12150_e12609_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12150_e12609_d_n14;

        let (assign12160_e12668, assign12160_e12668_d_n2, assign12160_e12668_d_n4, assign12160_e12668_d_n5, assign12160_e12668_d_n7, assign12160_e12668_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12160_e12654, assign12160_e12654_d_n2, assign12160_e12654_d_n5, assign12160_e12654_d_n7, assign12160_e12654_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12160_e12618: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12621: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12624: f64 = (0.001 / p.p53);
                let assign12160_e12627: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12628: f64 = (assign12160_e12624 * assign12160_e12627);
                let assign12160_e12629: f64 = (assign12160_e12628).tanh();
                let assign12160_e12630: f64 = (assign12160_e12621 * assign12160_e12629);
                let assign12160_e12631: f64 = (assign12160_e12618 + assign12160_e12630);
                let assign12160_e12632: f64 = (0.5 * assign12160_e12631);
                (assign12160_e12632, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))),)
            } else {
                let (assign12160_e12653, assign12160_e12653_d_n2, assign12160_e12653_d_n5, assign12160_e12653_d_n7, assign12160_e12653_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12160_e12639: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12642: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12645: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12646: f64 = (assign12160_e12642 * assign12160_e12645);
                        let assign12160_e12648: f64 = (assign12160_e12646 + p.p53);
                        let assign12160_e12649: f64 = (assign12160_e12648).sqrt();
                        let assign12160_e12650: f64 = (assign12160_e12639 + assign12160_e12649);
                        let assign12160_e12651: f64 = (0.5 * assign12160_e12650);
                        (assign12160_e12651, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign12160_e12649)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign12160_e12649)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign12160_e12649)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12160_e12645) + (assign12160_e12642 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign12160_e12649)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12160_e12653, assign12160_e12653_d_n2, assign12160_e12653_d_n5, assign12160_e12653_d_n7, assign12160_e12653_d_n14,)
            }
        };
        let assign12160_e12658: f64 = (p.p51 * 0.1);
        let assign12160_e12660: f64 = (assign12160_e12658 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12160_e12662: f64 = (assign12160_e12660 * locals.var_fn133_calc_iq__ff0);
        let assign12160_e12663: f64 = (locals.var_fn133_calc_iq__vtof - assign12160_e12662);
        let assign12160_e12664: f64 = (assign12160_e12654 - assign12160_e12663);
        let assign12160_e12666: f64 = (assign12160_e12664 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12160_e12666, ((assign12160_e12654_d_n2 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (((assign12160_e12658 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ff0) + (assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12160_e12664 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), ((assign12160_e12654_d_n5 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), ((assign12160_e12654_d_n7 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), ((assign12160_e12654_d_n14 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__eta0, locals.var_fn133_calc_iq__eta0_dn2, locals.var_fn133_calc_iq__eta0_dn4, locals.var_fn133_calc_iq__eta0_dn5, locals.var_fn133_calc_iq__eta0_dn7, locals.var_fn133_calc_iq__eta0_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta0 = assign12160_e12668;
        locals.var_fn133_calc_iq__eta0_dn2 = assign12160_e12668_d_n2;
        locals.var_fn133_calc_iq__eta0_dn4 = assign12160_e12668_d_n4;
        locals.var_fn133_calc_iq__eta0_dn5 = assign12160_e12668_d_n5;
        locals.var_fn133_calc_iq__eta0_dn7 = assign12160_e12668_d_n7;
        locals.var_fn133_calc_iq__eta0_dn14 = assign12160_e12668_d_n14;

        let assign12170_e12671: f64 = if locals.var_fn133_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign12170_e12671;

        let (assign12180_e12679, assign12180_e12679_d_n2, assign12180_e12679_d_n4, assign12180_e12679_d_n5, assign12180_e12679_d_n7, assign12180_e12679_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard149 != 0.0)) {
        let assign12180_e12677: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0);
        (assign12180_e12677, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__eta0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12180_e12679;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12180_e12679_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12180_e12679_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12180_e12679_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12180_e12679_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12180_e12679_d_n14;

        let assign12190_e12682: f64 = (-50.0);
        let assign12190_e12683: f64 = if locals.var_fn133_calc_iq__eta0 < assign12190_e12682 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign12190_e12683;

        let (assign12200_e12695, assign12200_e12695_d_n2, assign12200_e12695_d_n4, assign12200_e12695_d_n5, assign12200_e12695_d_n7, assign12200_e12695_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign12200_e12692: f64 = (locals.var_fn133_calc_iq__eta0).exp();
        let assign12200_e12693: f64 = (locals.var_fn133_calc_iq__qref0 * assign12200_e12692);
        (assign12200_e12693, (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12200_e12692) + (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12200_e12695;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12200_e12695_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12200_e12695_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12200_e12695_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12200_e12695_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12200_e12695_d_n14;

        let (assign12210_e12711, assign12210_e12711_d_n2, assign12210_e12711_d_n4, assign12210_e12711_d_n5, assign12210_e12711_d_n7, assign12210_e12711_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign12210_e12706: f64 = (locals.var_fn133_calc_iq__eta0).exp();
        let assign12210_e12707: f64 = (1.0 + assign12210_e12706);
        let assign12210_e12708: f64 = (assign12210_e12707).ln();
        let assign12210_e12709: f64 = (locals.var_fn133_calc_iq__qref0 * assign12210_e12708);
        (assign12210_e12709, (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn2) / assign12210_e12707)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12210_e12708) + (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn4) / assign12210_e12707))), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn5) / assign12210_e12707)), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn7) / assign12210_e12707)), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn14) / assign12210_e12707)),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12210_e12711;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12210_e12711_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12210_e12711_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12210_e12711_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12210_e12711_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12210_e12711_d_n14;

        let (assign12220_e12717, assign12220_e12717_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12220_e12715: f64 = (locals.var_fn133_calc_iq__mu0 / locals.var_fn133_calc_iq__tfacmobin);
        (assign12220_e12715, (-((locals.var_fn133_calc_iq__mu0 * locals.var_fn133_calc_iq__tfacmobin_dn4) / (locals.var_fn133_calc_iq__tfacmobin * locals.var_fn133_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn133_calc_iq__muf0, locals.var_fn133_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn133_calc_iq__muf0 = assign12220_e12717;
        locals.var_fn133_calc_iq__muf0_dn4 = assign12220_e12717_d_n4;

        let (assign12230_e12733, assign12230_e12733_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12230_e12723: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tnomin);
        let assign12230_e12724: f64 = (1.0 + assign12230_e12723);
        let assign12230_e12728: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin);
        let assign12230_e12729: f64 = (1.0 + assign12230_e12728);
        let assign12230_e12730: f64 = (assign12230_e12724 / assign12230_e12729);
        let assign12230_e12731: f64 = (locals.var_fn133_calc_iq__vel0 * assign12230_e12730);
        (assign12230_e12731, (locals.var_fn133_calc_iq__vel0 * (-((assign12230_e12724 * (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin_dn4)) / (assign12230_e12729 * assign12230_e12729)))),)
    } else {
        (locals.var_fn133_calc_iq__vx0, locals.var_fn133_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vx0 = assign12230_e12733;
        locals.var_fn133_calc_iq__vx0_dn4 = assign12230_e12733_d_n4;

        let (assign12240_e12741, assign12240_e12741_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12240_e12737: f64 = (locals.var_fn133_calc_iq__vx0 * locals.var_fn133_calc_iq__lin);
        let assign12240_e12739: f64 = (assign12240_e12737 / locals.var_fn133_calc_iq__muf0);
        (assign12240_e12739, ((((locals.var_fn133_calc_iq__vx0_dn4 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf0) - (assign12240_e12737 * locals.var_fn133_calc_iq__muf0_dn4)) / (locals.var_fn133_calc_iq__muf0 * locals.var_fn133_calc_iq__muf0)),)
    } else {
        (locals.var_fn133_calc_iq__vdsats0, locals.var_fn133_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vdsats0 = assign12240_e12741;
        locals.var_fn133_calc_iq__vdsats0_dn4 = assign12240_e12741_d_n4;

        let (assign12250_e12758, assign12250_e12758_d_n2, assign12250_e12758_d_n4, assign12250_e12758_d_n5, assign12250_e12758_d_n7, assign12250_e12758_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12250_e12747: f64 = (2.0 * locals.var_fn133_calc_iq__qinvv0);
        let assign12250_e12749: f64 = (assign12250_e12747 / locals.var_fn133_calc_iq__cgin);
        let assign12250_e12751: f64 = (assign12250_e12749 / locals.var_fn133_calc_iq__vdsats0);
        let assign12250_e12752: f64 = (1.0 + assign12250_e12751);
        let assign12250_e12753: f64 = (assign12250_e12752).sqrt();
        let assign12250_e12754: f64 = (locals.var_fn133_calc_iq__vdsats0 * assign12250_e12753);
        let assign12250_e12756: f64 = (assign12250_e12754 - locals.var_fn133_calc_iq__vdsats0);
        (assign12250_e12756, (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn2) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (((locals.var_fn133_calc_iq__vdsats0_dn4 * assign12250_e12753) + (locals.var_fn133_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn4) * locals.var_fn133_calc_iq__cgin) - (assign12250_e12747 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)) * locals.var_fn133_calc_iq__vdsats0) - (assign12250_e12749 * locals.var_fn133_calc_iq__vdsats0_dn4)) / (locals.var_fn133_calc_iq__vdsats0 * locals.var_fn133_calc_iq__vdsats0)) / (2.0 * assign12250_e12753)))) - locals.var_fn133_calc_iq__vdsats0_dn4), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn5) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn7) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn14) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))),)
    } else {
        (locals.var_fn133_calc_iq__vdsats10, locals.var_fn133_calc_iq__vdsats10_dn2, locals.var_fn133_calc_iq__vdsats10_dn4, locals.var_fn133_calc_iq__vdsats10_dn5, locals.var_fn133_calc_iq__vdsats10_dn7, locals.var_fn133_calc_iq__vdsats10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats10 = assign12250_e12758;
        locals.var_fn133_calc_iq__vdsats10_dn2 = assign12250_e12758_d_n2;
        locals.var_fn133_calc_iq__vdsats10_dn4 = assign12250_e12758_d_n4;
        locals.var_fn133_calc_iq__vdsats10_dn5 = assign12250_e12758_d_n5;
        locals.var_fn133_calc_iq__vdsats10_dn7 = assign12250_e12758_d_n7;
        locals.var_fn133_calc_iq__vdsats10_dn14 = assign12250_e12758_d_n14;

        let (assign12260_e12770, assign12260_e12770_d_n2, assign12260_e12770_d_n4, assign12260_e12770_d_n5, assign12260_e12770_d_n7, assign12260_e12770_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12260_e12763: f64 = (1.0 - locals.var_fn133_calc_iq__ff0);
        let assign12260_e12764: f64 = (locals.var_fn133_calc_iq__vdsats10 * assign12260_e12763);
        let assign12260_e12767: f64 = (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0);
        let assign12260_e12768: f64 = (assign12260_e12764 + assign12260_e12767);
        (assign12260_e12768, (((locals.var_fn133_calc_iq__vdsats10_dn2 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn2))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn2)), (((locals.var_fn133_calc_iq__vdsats10_dn4 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn4))) + ((locals.var_fn133_calc_iq__two_n_phit0_dn4 * locals.var_fn133_calc_iq__ff0) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn4))), (((locals.var_fn133_calc_iq__vdsats10_dn5 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn5))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn5)), (((locals.var_fn133_calc_iq__vdsats10_dn7 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn7))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn7)), (((locals.var_fn133_calc_iq__vdsats10_dn14 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn14))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vdsat10, locals.var_fn133_calc_iq__vdsat10_dn2, locals.var_fn133_calc_iq__vdsat10_dn4, locals.var_fn133_calc_iq__vdsat10_dn5, locals.var_fn133_calc_iq__vdsat10_dn7, locals.var_fn133_calc_iq__vdsat10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat10 = assign12260_e12770;
        locals.var_fn133_calc_iq__vdsat10_dn2 = assign12260_e12770_d_n2;
        locals.var_fn133_calc_iq__vdsat10_dn4 = assign12260_e12770_d_n4;
        locals.var_fn133_calc_iq__vdsat10_dn5 = assign12260_e12770_d_n5;
        locals.var_fn133_calc_iq__vdsat10_dn7 = assign12260_e12770_d_n7;
        locals.var_fn133_calc_iq__vdsat10_dn14 = assign12260_e12770_d_n14;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12270_e12839, assign12270_e12839_d_n2, assign12270_e12839_d_n4, assign12270_e12839_d_n5, assign12270_e12839_d_n7, assign12270_e12839_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12270_e12829, assign12270_e12829_d_n2, assign12270_e12829_d_n4, assign12270_e12829_d_n5, assign12270_e12829_d_n7, assign12270_e12829_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12270_e12782: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12783: f64 = assign12270_e12782;
                let assign12270_e12787: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12788: f64 = (-assign12270_e12787);
                let assign12270_e12791: f64 = (0.001 / p.p53);
                let assign12270_e12795: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12796: f64 = (-assign12270_e12795);
                let assign12270_e12797: f64 = (assign12270_e12791 * assign12270_e12796);
                let assign12270_e12798: f64 = (assign12270_e12797).tanh();
                let assign12270_e12799: f64 = (assign12270_e12788 * assign12270_e12798);
                let assign12270_e12800: f64 = (assign12270_e12783 + assign12270_e12799);
                let assign12270_e12801: f64 = (0.5 * assign12270_e12800);
                (assign12270_e12801, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))),)
            } else {
                let (assign12270_e12828, assign12270_e12828_d_n2, assign12270_e12828_d_n4, assign12270_e12828_d_n5, assign12270_e12828_d_n7, assign12270_e12828_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12270_e12809: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12810: f64 = assign12270_e12809;
                        let assign12270_e12814: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12815: f64 = (-assign12270_e12814);
                        let assign12270_e12819: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12820: f64 = (-assign12270_e12819);
                        let assign12270_e12821: f64 = (assign12270_e12815 * assign12270_e12820);
                        let assign12270_e12823: f64 = (assign12270_e12821 + p.p53);
                        let assign12270_e12824: f64 = (assign12270_e12823).sqrt();
                        let assign12270_e12825: f64 = (assign12270_e12810 + assign12270_e12824);
                        let assign12270_e12826: f64 = (0.5 * assign12270_e12825);
                        (assign12270_e12826, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12820) + (assign12270_e12815 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12270_e12824)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12820) + (assign12270_e12815 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12270_e12824)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12270_e12828, assign12270_e12828_d_n2, assign12270_e12828_d_n4, assign12270_e12828_d_n5, assign12270_e12828_d_n7, assign12270_e12828_d_n14,)
            }
        };
        let assign12270_e12831: f64 = (assign12270_e12829).powf(locals.var_fn133_calc_iq__beta);
        let assign12270_e12832: f64 = (1.0 + assign12270_e12831);
        let assign12270_e12835: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign12270_e12836: f64 = (assign12270_e12832).powf(assign12270_e12835);
        let assign12270_e12837: f64 = (1.0 / assign12270_e12836);
        (assign12270_e12837, (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n2)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n2 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n2)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n2 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n4)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n4 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n4)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n4 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n5)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n5 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n5)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n5 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n7)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n7 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n7)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n7 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n14)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n14 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n14)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n14 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))),)
    } else {
        (locals.var_fn133_calc_iq__fsd0, locals.var_fn133_calc_iq__fsd0_dn2, locals.var_fn133_calc_iq__fsd0_dn4, locals.var_fn133_calc_iq__fsd0_dn5, locals.var_fn133_calc_iq__fsd0_dn7, locals.var_fn133_calc_iq__fsd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsd0 = assign12270_e12839;
        locals.var_fn133_calc_iq__fsd0_dn2 = assign12270_e12839_d_n2;
        locals.var_fn133_calc_iq__fsd0_dn4 = assign12270_e12839_d_n4;
        locals.var_fn133_calc_iq__fsd0_dn5 = assign12270_e12839_d_n5;
        locals.var_fn133_calc_iq__fsd0_dn7 = assign12270_e12839_d_n7;
        locals.var_fn133_calc_iq__fsd0_dn14 = assign12270_e12839_d_n14;

        let (assign12280_e12845, assign12280_e12845_d_n2, assign12280_e12845_d_n4, assign12280_e12845_d_n5, assign12280_e12845_d_n7, assign12280_e12845_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12280_e12843: f64 = (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0);
        (assign12280_e12843, (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn2), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn4), ((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__fsd0) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn5)), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn7), ((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__fsd0) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vdx0, locals.var_fn133_calc_iq__vdx0_dn2, locals.var_fn133_calc_iq__vdx0_dn4, locals.var_fn133_calc_iq__vdx0_dn5, locals.var_fn133_calc_iq__vdx0_dn7, locals.var_fn133_calc_iq__vdx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdx0 = assign12280_e12845;
        locals.var_fn133_calc_iq__vdx0_dn2 = assign12280_e12845_d_n2;
        locals.var_fn133_calc_iq__vdx0_dn4 = assign12280_e12845_d_n4;
        locals.var_fn133_calc_iq__vdx0_dn5 = assign12280_e12845_d_n5;
        locals.var_fn133_calc_iq__vdx0_dn7 = assign12280_e12845_d_n7;
        locals.var_fn133_calc_iq__vdx0_dn14 = assign12280_e12845_d_n14;

        let (assign12290_e12920, assign12290_e12920_d_n2, assign12290_e12920_d_n4, assign12290_e12920_d_n5, assign12290_e12920_d_n7, assign12290_e12920_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12290_e12910, assign12290_e12910_d_n2, assign12290_e12910_d_n4, assign12290_e12910_d_n5, assign12290_e12910_d_n7, assign12290_e12910_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12290_e12856: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12858: f64 = (assign12290_e12856 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12859: f64 = assign12290_e12858;
                let assign12290_e12862: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12864: f64 = (assign12290_e12862 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12865: f64 = (-assign12290_e12864);
                let assign12290_e12868: f64 = (0.001 / p.p53);
                let assign12290_e12871: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12873: f64 = (assign12290_e12871 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12874: f64 = (-assign12290_e12873);
                let assign12290_e12875: f64 = (assign12290_e12868 * assign12290_e12874);
                let assign12290_e12876: f64 = (assign12290_e12875).tanh();
                let assign12290_e12877: f64 = (assign12290_e12865 * assign12290_e12876);
                let assign12290_e12878: f64 = (assign12290_e12859 + assign12290_e12877);
                let assign12290_e12879: f64 = (0.5 * assign12290_e12878);
                (assign12290_e12879, (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))),)
            } else {
                let (assign12290_e12909, assign12290_e12909_d_n2, assign12290_e12909_d_n4, assign12290_e12909_d_n5, assign12290_e12909_d_n7, assign12290_e12909_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12290_e12886: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12888: f64 = (assign12290_e12886 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12889: f64 = assign12290_e12888;
                        let assign12290_e12892: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12894: f64 = (assign12290_e12892 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12895: f64 = (-assign12290_e12894);
                        let assign12290_e12898: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12900: f64 = (assign12290_e12898 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12901: f64 = (-assign12290_e12900);
                        let assign12290_e12902: f64 = (assign12290_e12895 * assign12290_e12901);
                        let assign12290_e12904: f64 = (assign12290_e12902 + p.p53);
                        let assign12290_e12905: f64 = (assign12290_e12904).sqrt();
                        let assign12290_e12906: f64 = (assign12290_e12889 + assign12290_e12905);
                        let assign12290_e12907: f64 = (0.5 * assign12290_e12906);
                        (assign12290_e12907, (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12901) + (assign12290_e12895 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12290_e12905)))), (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12901) + (assign12290_e12895 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12290_e12905)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12290_e12909, assign12290_e12909_d_n2, assign12290_e12909_d_n4, assign12290_e12909_d_n5, assign12290_e12909_d_n7, assign12290_e12909_d_n14,)
            }
        };
        let assign12290_e12912: f64 = (assign12290_e12910).powf(locals.var_fn133_calc_iq__beta);
        let assign12290_e12913: f64 = (1.0 + assign12290_e12912);
        let assign12290_e12916: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign12290_e12917: f64 = (assign12290_e12913).powf(assign12290_e12916);
        let assign12290_e12918: f64 = (1.0 / assign12290_e12917);
        (assign12290_e12918, (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n2)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n2 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n2)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n2 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n4)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n4 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n4)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n4 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n5)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n5 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n5)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n5 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n7)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n7 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n7)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n7 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n14)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n14 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n14)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n14 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))),)
    } else {
        (locals.var_fn133_calc_iq__fds0, locals.var_fn133_calc_iq__fds0_dn2, locals.var_fn133_calc_iq__fds0_dn4, locals.var_fn133_calc_iq__fds0_dn5, locals.var_fn133_calc_iq__fds0_dn7, locals.var_fn133_calc_iq__fds0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fds0 = assign12290_e12920;
        locals.var_fn133_calc_iq__fds0_dn2 = assign12290_e12920_d_n2;
        locals.var_fn133_calc_iq__fds0_dn4 = assign12290_e12920_d_n4;
        locals.var_fn133_calc_iq__fds0_dn5 = assign12290_e12920_d_n5;
        locals.var_fn133_calc_iq__fds0_dn7 = assign12290_e12920_d_n7;
        locals.var_fn133_calc_iq__fds0_dn14 = assign12290_e12920_d_n14;

        let (assign12300_e12927, assign12300_e12927_d_n2, assign12300_e12927_d_n4, assign12300_e12927_d_n5, assign12300_e12927_d_n7, assign12300_e12927_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12300_e12923: f64 = (-locals.var_fn133_calc_iq__vdsin);
        let assign12300_e12925: f64 = (assign12300_e12923 * locals.var_fn133_calc_iq__fds0);
        (assign12300_e12925, (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn2), (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn4), (((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__fds0) + (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn5)), (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn7), (((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__fds0) + (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vsx0, locals.var_fn133_calc_iq__vsx0_dn2, locals.var_fn133_calc_iq__vsx0_dn4, locals.var_fn133_calc_iq__vsx0_dn5, locals.var_fn133_calc_iq__vsx0_dn7, locals.var_fn133_calc_iq__vsx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsx0 = assign12300_e12927;
        locals.var_fn133_calc_iq__vsx0_dn2 = assign12300_e12927_d_n2;
        locals.var_fn133_calc_iq__vsx0_dn4 = assign12300_e12927_d_n4;
        locals.var_fn133_calc_iq__vsx0_dn5 = assign12300_e12927_d_n5;
        locals.var_fn133_calc_iq__vsx0_dn7 = assign12300_e12927_d_n7;
        locals.var_fn133_calc_iq__vsx0_dn14 = assign12300_e12927_d_n14;

        let (assign12310_e12935, assign12310_e12935_d_n2, assign12310_e12935_d_n4, assign12310_e12935_d_n5, assign12310_e12935_d_n7, assign12310_e12935_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12310_e12931: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__myarg0);
        let assign12310_e12933: f64 = (assign12310_e12931 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12310_e12933, (locals.var_fn133_calc_iq__vgsin_dn2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12310_e12931 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (locals.var_fn133_calc_iq__vgsin_dn5 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgsin_dn7 / locals.var_fn133_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12310_e12935;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12310_e12935_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12310_e12935_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12310_e12935_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12310_e12935_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12310_e12935_d_n14;

        let assign12320_e12938: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign12320_e12938;

        let (assign12330_e12944, assign12330_e12944_d_n2, assign12330_e12944_d_n4, assign12330_e12944_d_n5, assign12330_e12944_d_n7, assign12330_e12944_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard151 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12330_e12944;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12330_e12944_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12330_e12944_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12330_e12944_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12330_e12944_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12330_e12944_d_n14;

        let assign12340_e12947: f64 = (-50.0);
        let assign12340_e12948: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12340_e12947 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign12340_e12948;

        let (assign12350_e12957, assign12350_e12957_d_n2, assign12350_e12957_d_n4, assign12350_e12957_d_n5, assign12350_e12957_d_n7, assign12350_e12957_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard151 == 0.0)) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12350_e12957;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12350_e12957_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12350_e12957_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12350_e12957_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12350_e12957_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12350_e12957_d_n14;

        let (assign12360_e12972, assign12360_e12972_d_n2, assign12360_e12972_d_n4, assign12360_e12972_d_n5, assign12360_e12972_d_n7, assign12360_e12972_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard151 == 0.0)) && (locals.var_guard152 == 0.0)) {
        let assign12360_e12968: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12360_e12969: f64 = (1.0 + assign12360_e12968);
        let assign12360_e12970: f64 = (1.0 / assign12360_e12969);
        (assign12360_e12970, (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12360_e12969 * assign12360_e12969))),)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12360_e12972;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12360_e12972_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12360_e12972_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12360_e12972_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12360_e12972_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12360_e12972_d_n14;

        let (assign12370_e12990, assign12370_e12990_d_n2, assign12370_e12990_d_n4, assign12370_e12990_d_n5, assign12370_e12990_d_n7, assign12370_e12990_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12370_e12976: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__vsx0);
        let assign12370_e12980: f64 = (p.p51 * 0.1);
        let assign12370_e12982: f64 = (assign12370_e12980 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12370_e12984: f64 = (assign12370_e12982 * locals.var_fn133_calc_iq__ffs0);
        let assign12370_e12985: f64 = (locals.var_fn133_calc_iq__vtof - assign12370_e12984);
        let assign12370_e12986: f64 = (assign12370_e12976 - assign12370_e12985);
        let assign12370_e12988: f64 = (assign12370_e12986 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12370_e12988, (((locals.var_fn133_calc_iq__vgdin_dn2 - locals.var_fn133_calc_iq__vsx0_dn2) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), (((((-locals.var_fn133_calc_iq__vsx0_dn4) - (locals.var_fn133_calc_iq__vtof_dn4 - (((assign12370_e12980 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffs0) + (assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12370_e12986 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (((locals.var_fn133_calc_iq__vgdin_dn5 - locals.var_fn133_calc_iq__vsx0_dn5) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgdin_dn7 - locals.var_fn133_calc_iq__vsx0_dn7) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgdin_dn14 - locals.var_fn133_calc_iq__vsx0_dn14) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etas0, locals.var_fn133_calc_iq__etas0_dn2, locals.var_fn133_calc_iq__etas0_dn4, locals.var_fn133_calc_iq__etas0_dn5, locals.var_fn133_calc_iq__etas0_dn7, locals.var_fn133_calc_iq__etas0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas0 = assign12370_e12990;
        locals.var_fn133_calc_iq__etas0_dn2 = assign12370_e12990_d_n2;
        locals.var_fn133_calc_iq__etas0_dn4 = assign12370_e12990_d_n4;
        locals.var_fn133_calc_iq__etas0_dn5 = assign12370_e12990_d_n5;
        locals.var_fn133_calc_iq__etas0_dn7 = assign12370_e12990_d_n7;
        locals.var_fn133_calc_iq__etas0_dn14 = assign12370_e12990_d_n14;

        let assign12380_e12993: f64 = if locals.var_fn133_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign12380_e12993;

        let (assign12390_e13001, assign12390_e13001_d_n2, assign12390_e13001_d_n4, assign12390_e13001_d_n5, assign12390_e13001_d_n7, assign12390_e13001_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard153 != 0.0)) {
        let assign12390_e12999: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0);
        (assign12390_e12999, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__etas0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12390_e13001;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12390_e13001_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12390_e13001_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12390_e13001_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12390_e13001_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12390_e13001_d_n14;

        let assign12400_e13004: f64 = (-50.0);
        let assign12400_e13005: f64 = if locals.var_fn133_calc_iq__etas0 < assign12400_e13004 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign12400_e13005;

        let (assign12410_e13017, assign12410_e13017_d_n2, assign12410_e13017_d_n4, assign12410_e13017_d_n5, assign12410_e13017_d_n7, assign12410_e13017_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign12410_e13014: f64 = (locals.var_fn133_calc_iq__etas0).exp();
        let assign12410_e13015: f64 = (locals.var_fn133_calc_iq__qref0 * assign12410_e13014);
        (assign12410_e13015, (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12410_e13014) + (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12410_e13017;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12410_e13017_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12410_e13017_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12410_e13017_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12410_e13017_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12410_e13017_d_n14;

        let (assign12420_e13033, assign12420_e13033_d_n2, assign12420_e13033_d_n4, assign12420_e13033_d_n5, assign12420_e13033_d_n7, assign12420_e13033_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign12420_e13028: f64 = (locals.var_fn133_calc_iq__etas0).exp();
        let assign12420_e13029: f64 = (1.0 + assign12420_e13028);
        let assign12420_e13030: f64 = (assign12420_e13029).ln();
        let assign12420_e13031: f64 = (locals.var_fn133_calc_iq__qref0 * assign12420_e13030);
        (assign12420_e13031, (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn2) / assign12420_e13029)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12420_e13030) + (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn4) / assign12420_e13029))), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn5) / assign12420_e13029)), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn7) / assign12420_e13029)), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn14) / assign12420_e13029)),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12420_e13033;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12420_e13033_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12420_e13033_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12420_e13033_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12420_e13033_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12420_e13033_d_n14;

        let (assign12430_e13041, assign12430_e13041_d_n2, assign12430_e13041_d_n4, assign12430_e13041_d_n5, assign12430_e13041_d_n7, assign12430_e13041_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12430_e13037: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__myarg0);
        let assign12430_e13039: f64 = (assign12430_e13037 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12430_e13039, (locals.var_fn133_calc_iq__vgdin_dn2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12430_e13037 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (locals.var_fn133_calc_iq__vgdin_dn5 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgdin_dn7 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgdin_dn14 / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12430_e13041;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12430_e13041_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12430_e13041_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12430_e13041_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12430_e13041_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12430_e13041_d_n14;

        let assign12440_e13044: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign12440_e13044;

        let (assign12450_e13050, assign12450_e13050_d_n2, assign12450_e13050_d_n4, assign12450_e13050_d_n5, assign12450_e13050_d_n7, assign12450_e13050_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard155 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12450_e13050;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12450_e13050_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12450_e13050_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12450_e13050_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12450_e13050_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12450_e13050_d_n14;

        let assign12460_e13053: f64 = (-50.0);
        let assign12460_e13054: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12460_e13053 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign12460_e13054;

        let (assign12470_e13063, assign12470_e13063_d_n2, assign12470_e13063_d_n4, assign12470_e13063_d_n5, assign12470_e13063_d_n7, assign12470_e13063_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12470_e13063;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12470_e13063_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12470_e13063_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12470_e13063_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12470_e13063_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12470_e13063_d_n14;

        let (assign12480_e13078, assign12480_e13078_d_n2, assign12480_e13078_d_n4, assign12480_e13078_d_n5, assign12480_e13078_d_n7, assign12480_e13078_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
        let assign12480_e13074: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12480_e13075: f64 = (1.0 + assign12480_e13074);
        let assign12480_e13076: f64 = (1.0 / assign12480_e13075);
        (assign12480_e13076, (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12480_e13075 * assign12480_e13075))),)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12480_e13078;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12480_e13078_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12480_e13078_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12480_e13078_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12480_e13078_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12480_e13078_d_n14;

        let (assign12490_e13096, assign12490_e13096_d_n2, assign12490_e13096_d_n4, assign12490_e13096_d_n5, assign12490_e13096_d_n7, assign12490_e13096_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12490_e13082: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vdx0);
        let assign12490_e13086: f64 = (p.p51 * 0.1);
        let assign12490_e13088: f64 = (assign12490_e13086 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12490_e13090: f64 = (assign12490_e13088 * locals.var_fn133_calc_iq__ffd0);
        let assign12490_e13091: f64 = (locals.var_fn133_calc_iq__vtof - assign12490_e13090);
        let assign12490_e13092: f64 = (assign12490_e13082 - assign12490_e13091);
        let assign12490_e13094: f64 = (assign12490_e13092 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12490_e13094, (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vdx0_dn2) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), (((((-locals.var_fn133_calc_iq__vdx0_dn4) - (locals.var_fn133_calc_iq__vtof_dn4 - (((assign12490_e13086 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffd0) + (assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12490_e13092 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vdx0_dn5) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vdx0_dn7) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), (((-locals.var_fn133_calc_iq__vdx0_dn14) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etad0, locals.var_fn133_calc_iq__etad0_dn2, locals.var_fn133_calc_iq__etad0_dn4, locals.var_fn133_calc_iq__etad0_dn5, locals.var_fn133_calc_iq__etad0_dn7, locals.var_fn133_calc_iq__etad0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etad0 = assign12490_e13096;
        locals.var_fn133_calc_iq__etad0_dn2 = assign12490_e13096_d_n2;
        locals.var_fn133_calc_iq__etad0_dn4 = assign12490_e13096_d_n4;
        locals.var_fn133_calc_iq__etad0_dn5 = assign12490_e13096_d_n5;
        locals.var_fn133_calc_iq__etad0_dn7 = assign12490_e13096_d_n7;
        locals.var_fn133_calc_iq__etad0_dn14 = assign12490_e13096_d_n14;

        let assign12500_e13099: f64 = if locals.var_fn133_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign12500_e13099;

        let (assign12510_e13107, assign12510_e13107_d_n2, assign12510_e13107_d_n4, assign12510_e13107_d_n5, assign12510_e13107_d_n7, assign12510_e13107_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard157 != 0.0)) {
        let assign12510_e13105: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0);
        (assign12510_e13105, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__etad0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12510_e13107;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12510_e13107_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12510_e13107_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12510_e13107_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12510_e13107_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12510_e13107_d_n14;

        let assign12520_e13110: f64 = (-50.0);
        let assign12520_e13111: f64 = if locals.var_fn133_calc_iq__etad0 < assign12520_e13110 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign12520_e13111;

        let (assign12530_e13123, assign12530_e13123_d_n2, assign12530_e13123_d_n4, assign12530_e13123_d_n5, assign12530_e13123_d_n7, assign12530_e13123_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard157 == 0.0)) && (locals.var_guard158 != 0.0)) {
        let assign12530_e13120: f64 = (locals.var_fn133_calc_iq__etad0).exp();
        let assign12530_e13121: f64 = (locals.var_fn133_calc_iq__qref0 * assign12530_e13120);
        (assign12530_e13121, (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12530_e13120) + (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12530_e13123;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12530_e13123_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12530_e13123_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12530_e13123_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12530_e13123_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12530_e13123_d_n14;

        let (assign12540_e13139, assign12540_e13139_d_n2, assign12540_e13139_d_n4, assign12540_e13139_d_n5, assign12540_e13139_d_n7, assign12540_e13139_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard157 == 0.0)) && (locals.var_guard158 == 0.0)) {
        let assign12540_e13134: f64 = (locals.var_fn133_calc_iq__etad0).exp();
        let assign12540_e13135: f64 = (1.0 + assign12540_e13134);
        let assign12540_e13136: f64 = (assign12540_e13135).ln();
        let assign12540_e13137: f64 = (locals.var_fn133_calc_iq__qref0 * assign12540_e13136);
        (assign12540_e13137, (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn2) / assign12540_e13135)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12540_e13136) + (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn4) / assign12540_e13135))), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn5) / assign12540_e13135)), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn7) / assign12540_e13135)), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn14) / assign12540_e13135)),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12540_e13139;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12540_e13139_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12540_e13139_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12540_e13139_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12540_e13139_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12540_e13139_d_n14;

        let (assign12550_e13147, assign12550_e13147_d_n2, assign12550_e13147_d_n4, assign12550_e13147_d_n5, assign12550_e13147_d_n7, assign12550_e13147_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12550_e13143: f64 = (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0);
        let assign12550_e13145: f64 = (assign12550_e13143 + 1e-38);
        (assign12550_e13145, ((locals.var_fn133_calc_iq__qinvs0_dn2 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn2)), ((locals.var_fn133_calc_iq__qinvs0_dn4 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn4)), ((locals.var_fn133_calc_iq__qinvs0_dn5 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn5)), ((locals.var_fn133_calc_iq__qinvs0_dn7 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn7)), ((locals.var_fn133_calc_iq__qinvs0_dn14 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qs2, locals.var_fn133_calc_iq__qs2_dn2, locals.var_fn133_calc_iq__qs2_dn4, locals.var_fn133_calc_iq__qs2_dn5, locals.var_fn133_calc_iq__qs2_dn7, locals.var_fn133_calc_iq__qs2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs2 = assign12550_e13147;
        locals.var_fn133_calc_iq__qs2_dn2 = assign12550_e13147_d_n2;
        locals.var_fn133_calc_iq__qs2_dn4 = assign12550_e13147_d_n4;
        locals.var_fn133_calc_iq__qs2_dn5 = assign12550_e13147_d_n5;
        locals.var_fn133_calc_iq__qs2_dn7 = assign12550_e13147_d_n7;
        locals.var_fn133_calc_iq__qs2_dn14 = assign12550_e13147_d_n14;

        let (assign12560_e13155, assign12560_e13155_d_n2, assign12560_e13155_d_n4, assign12560_e13155_d_n5, assign12560_e13155_d_n7, assign12560_e13155_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12560_e13151: f64 = (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0);
        let assign12560_e13153: f64 = (assign12560_e13151 + 1e-57);
        (assign12560_e13153, ((locals.var_fn133_calc_iq__qs2_dn2 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn2)), ((locals.var_fn133_calc_iq__qs2_dn4 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn4)), ((locals.var_fn133_calc_iq__qs2_dn5 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn5)), ((locals.var_fn133_calc_iq__qs2_dn7 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn7)), ((locals.var_fn133_calc_iq__qs2_dn14 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qs3, locals.var_fn133_calc_iq__qs3_dn2, locals.var_fn133_calc_iq__qs3_dn4, locals.var_fn133_calc_iq__qs3_dn5, locals.var_fn133_calc_iq__qs3_dn7, locals.var_fn133_calc_iq__qs3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs3 = assign12560_e13155;
        locals.var_fn133_calc_iq__qs3_dn2 = assign12560_e13155_d_n2;
        locals.var_fn133_calc_iq__qs3_dn4 = assign12560_e13155_d_n4;
        locals.var_fn133_calc_iq__qs3_dn5 = assign12560_e13155_d_n5;
        locals.var_fn133_calc_iq__qs3_dn7 = assign12560_e13155_d_n7;
        locals.var_fn133_calc_iq__qs3_dn14 = assign12560_e13155_d_n14;

        let (assign12570_e13163, assign12570_e13163_d_n2, assign12570_e13163_d_n4, assign12570_e13163_d_n5, assign12570_e13163_d_n7, assign12570_e13163_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12570_e13159: f64 = (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0);
        let assign12570_e13161: f64 = (assign12570_e13159 + 1e-38);
        (assign12570_e13161, ((locals.var_fn133_calc_iq__qinvd0_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qinvd0_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qinvd0_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qinvd0_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qinvd0_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qd2, locals.var_fn133_calc_iq__qd2_dn2, locals.var_fn133_calc_iq__qd2_dn4, locals.var_fn133_calc_iq__qd2_dn5, locals.var_fn133_calc_iq__qd2_dn7, locals.var_fn133_calc_iq__qd2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd2 = assign12570_e13163;
        locals.var_fn133_calc_iq__qd2_dn2 = assign12570_e13163_d_n2;
        locals.var_fn133_calc_iq__qd2_dn4 = assign12570_e13163_d_n4;
        locals.var_fn133_calc_iq__qd2_dn5 = assign12570_e13163_d_n5;
        locals.var_fn133_calc_iq__qd2_dn7 = assign12570_e13163_d_n7;
        locals.var_fn133_calc_iq__qd2_dn14 = assign12570_e13163_d_n14;

        let (assign12580_e13171, assign12580_e13171_d_n2, assign12580_e13171_d_n4, assign12580_e13171_d_n5, assign12580_e13171_d_n7, assign12580_e13171_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12580_e13167: f64 = (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0);
        let assign12580_e13169: f64 = (assign12580_e13167 + 1e-57);
        (assign12580_e13169, ((locals.var_fn133_calc_iq__qd2_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qd2_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qd2_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qd2_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qd2_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qd3, locals.var_fn133_calc_iq__qd3_dn2, locals.var_fn133_calc_iq__qd3_dn4, locals.var_fn133_calc_iq__qd3_dn5, locals.var_fn133_calc_iq__qd3_dn7, locals.var_fn133_calc_iq__qd3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd3 = assign12580_e13171;
        locals.var_fn133_calc_iq__qd3_dn2 = assign12580_e13171_d_n2;
        locals.var_fn133_calc_iq__qd3_dn4 = assign12580_e13171_d_n4;
        locals.var_fn133_calc_iq__qd3_dn5 = assign12580_e13171_d_n5;
        locals.var_fn133_calc_iq__qd3_dn7 = assign12580_e13171_d_n7;
        locals.var_fn133_calc_iq__qd3_dn14 = assign12580_e13171_d_n14;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12590_e13179, assign12590_e13179_d_n2, assign12590_e13179_d_n4, assign12590_e13179_d_n5, assign12590_e13179_d_n7, assign12590_e13179_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12590_e13175: f64 = (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0);
        let assign12590_e13177: f64 = (assign12590_e13175 + 1e-38);
        (assign12590_e13177, ((locals.var_fn133_calc_iq__qinvs0_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qinvs0_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qinvs0_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qinvs0_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qinvs0_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qsqd, locals.var_fn133_calc_iq__qsqd_dn2, locals.var_fn133_calc_iq__qsqd_dn4, locals.var_fn133_calc_iq__qsqd_dn5, locals.var_fn133_calc_iq__qsqd_dn7, locals.var_fn133_calc_iq__qsqd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsqd = assign12590_e13179;
        locals.var_fn133_calc_iq__qsqd_dn2 = assign12590_e13179_d_n2;
        locals.var_fn133_calc_iq__qsqd_dn4 = assign12590_e13179_d_n4;
        locals.var_fn133_calc_iq__qsqd_dn5 = assign12590_e13179_d_n5;
        locals.var_fn133_calc_iq__qsqd_dn7 = assign12590_e13179_d_n7;
        locals.var_fn133_calc_iq__qsqd_dn14 = assign12590_e13179_d_n14;

        let (assign12600_e13197, assign12600_e13197_d_n2, assign12600_e13197_d_n4, assign12600_e13197_d_n5, assign12600_e13197_d_n7, assign12600_e13197_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12600_e13183: f64 = (2.0 / 3.0);
        let assign12600_e13186: f64 = (locals.var_fn133_calc_iq__qs2 + locals.var_fn133_calc_iq__qd2);
        let assign12600_e13188: f64 = (assign12600_e13186 + locals.var_fn133_calc_iq__qsqd);
        let assign12600_e13189: f64 = (assign12600_e13183 * assign12600_e13188);
        let assign12600_e13192: f64 = (locals.var_fn133_calc_iq__qinvs0 + locals.var_fn133_calc_iq__qinvd0);
        let assign12600_e13194: f64 = (assign12600_e13192 + 2e-19);
        let assign12600_e13195: f64 = (assign12600_e13189 / assign12600_e13194);
        (assign12600_e13195, ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn2 + locals.var_fn133_calc_iq__qd2_dn2) + locals.var_fn133_calc_iq__qsqd_dn2)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn2 + locals.var_fn133_calc_iq__qinvd0_dn2))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn4 + locals.var_fn133_calc_iq__qd2_dn4) + locals.var_fn133_calc_iq__qsqd_dn4)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn4 + locals.var_fn133_calc_iq__qinvd0_dn4))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn5 + locals.var_fn133_calc_iq__qd2_dn5) + locals.var_fn133_calc_iq__qsqd_dn5)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn5 + locals.var_fn133_calc_iq__qinvd0_dn5))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn7 + locals.var_fn133_calc_iq__qd2_dn7) + locals.var_fn133_calc_iq__qsqd_dn7)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn7 + locals.var_fn133_calc_iq__qinvd0_dn7))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn14 + locals.var_fn133_calc_iq__qd2_dn14) + locals.var_fn133_calc_iq__qsqd_dn14)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn14 + locals.var_fn133_calc_iq__qinvd0_dn14))) / (assign12600_e13194 * assign12600_e13194)),)
    } else {
        (locals.var_fn133_calc_iq__qinvdd, locals.var_fn133_calc_iq__qinvdd_dn2, locals.var_fn133_calc_iq__qinvdd_dn4, locals.var_fn133_calc_iq__qinvdd_dn5, locals.var_fn133_calc_iq__qinvdd_dn7, locals.var_fn133_calc_iq__qinvdd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvdd = assign12600_e13197;
        locals.var_fn133_calc_iq__qinvdd_dn2 = assign12600_e13197_d_n2;
        locals.var_fn133_calc_iq__qinvdd_dn4 = assign12600_e13197_d_n4;
        locals.var_fn133_calc_iq__qinvdd_dn5 = assign12600_e13197_d_n5;
        locals.var_fn133_calc_iq__qinvdd_dn7 = assign12600_e13197_d_n7;
        locals.var_fn133_calc_iq__qinvdd_dn14 = assign12600_e13197_d_n14;

        let (assign12610_e13231, assign12610_e13231_d_n2, assign12610_e13231_d_n4, assign12610_e13231_d_n5, assign12610_e13231_d_n7, assign12610_e13231_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12610_e13202: f64 = (2.0 * locals.var_fn133_calc_iq__qs3);
        let assign12610_e13205: f64 = (3.0 * locals.var_fn133_calc_iq__qd3);
        let assign12610_e13206: f64 = (assign12610_e13202 + assign12610_e13205);
        let assign12610_e13209: f64 = (4.0 * locals.var_fn133_calc_iq__qs2);
        let assign12610_e13211: f64 = (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0);
        let assign12610_e13212: f64 = (assign12610_e13206 + assign12610_e13211);
        let assign12610_e13215: f64 = (6.0 * locals.var_fn133_calc_iq__qd2);
        let assign12610_e13217: f64 = (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0);
        let assign12610_e13218: f64 = (assign12610_e13212 + assign12610_e13217);
        let assign12610_e13219: f64 = (2.0 * assign12610_e13218);
        let assign12610_e13223: f64 = (locals.var_fn133_calc_iq__qs2 + locals.var_fn133_calc_iq__qd2);
        let assign12610_e13226: f64 = (2.0 * locals.var_fn133_calc_iq__qsqd);
        let assign12610_e13227: f64 = (assign12610_e13223 + assign12610_e13226);
        let assign12610_e13228: f64 = (15.0 * assign12610_e13227);
        let assign12610_e13229: f64 = (assign12610_e13219 / assign12610_e13228);
        (assign12610_e13229, ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn2) + (3.0 * locals.var_fn133_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn2) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn2) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn2)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn2 + locals.var_fn133_calc_iq__qd2_dn2) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn2))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn4) + (3.0 * locals.var_fn133_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn4) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn4) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn4)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn4 + locals.var_fn133_calc_iq__qd2_dn4) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn4))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn5) + (3.0 * locals.var_fn133_calc_iq__qd3_dn5)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn5) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn5))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn5) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn5)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn5 + locals.var_fn133_calc_iq__qd2_dn5) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn5))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn7) + (3.0 * locals.var_fn133_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn7) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn7) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn7)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn7 + locals.var_fn133_calc_iq__qd2_dn7) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn7))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn14) + (3.0 * locals.var_fn133_calc_iq__qd3_dn14)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn14) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn14))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn14) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn14)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn14 + locals.var_fn133_calc_iq__qd2_dn14) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn14))))) / (assign12610_e13228 * assign12610_e13228)),)
    } else {
        (locals.var_fn133_calc_iq__qd1, locals.var_fn133_calc_iq__qd1_dn2, locals.var_fn133_calc_iq__qd1_dn4, locals.var_fn133_calc_iq__qd1_dn5, locals.var_fn133_calc_iq__qd1_dn7, locals.var_fn133_calc_iq__qd1_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd1 = assign12610_e13231;
        locals.var_fn133_calc_iq__qd1_dn2 = assign12610_e13231_d_n2;
        locals.var_fn133_calc_iq__qd1_dn4 = assign12610_e13231_d_n4;
        locals.var_fn133_calc_iq__qd1_dn5 = assign12610_e13231_d_n5;
        locals.var_fn133_calc_iq__qd1_dn7 = assign12610_e13231_d_n7;
        locals.var_fn133_calc_iq__qd1_dn14 = assign12610_e13231_d_n14;

        let (assign12620_e13237, assign12620_e13237_d_n2, assign12620_e13237_d_n4, assign12620_e13237_d_n5, assign12620_e13237_d_n7, assign12620_e13237_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12620_e13235: f64 = (locals.var_fn133_calc_iq__qinvdd - locals.var_fn133_calc_iq__qd1);
        (assign12620_e13235, (locals.var_fn133_calc_iq__qinvdd_dn2 - locals.var_fn133_calc_iq__qd1_dn2), (locals.var_fn133_calc_iq__qinvdd_dn4 - locals.var_fn133_calc_iq__qd1_dn4), (locals.var_fn133_calc_iq__qinvdd_dn5 - locals.var_fn133_calc_iq__qd1_dn5), (locals.var_fn133_calc_iq__qinvdd_dn7 - locals.var_fn133_calc_iq__qd1_dn7), (locals.var_fn133_calc_iq__qinvdd_dn14 - locals.var_fn133_calc_iq__qd1_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qs, locals.var_fn133_calc_iq__qs_dn2, locals.var_fn133_calc_iq__qs_dn4, locals.var_fn133_calc_iq__qs_dn5, locals.var_fn133_calc_iq__qs_dn7, locals.var_fn133_calc_iq__qs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs = assign12620_e13237;
        locals.var_fn133_calc_iq__qs_dn2 = assign12620_e13237_d_n2;
        locals.var_fn133_calc_iq__qs_dn4 = assign12620_e13237_d_n4;
        locals.var_fn133_calc_iq__qs_dn5 = assign12620_e13237_d_n5;
        locals.var_fn133_calc_iq__qs_dn7 = assign12620_e13237_d_n7;
        locals.var_fn133_calc_iq__qs_dn14 = assign12620_e13237_d_n14;

        let (assign12630_e13241, assign12630_e13241_d_n2, assign12630_e13241_d_n4, assign12630_e13241_d_n5, assign12630_e13241_d_n7, assign12630_e13241_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qd1, locals.var_fn133_calc_iq__qd1_dn2, locals.var_fn133_calc_iq__qd1_dn4, locals.var_fn133_calc_iq__qd1_dn5, locals.var_fn133_calc_iq__qd1_dn7, locals.var_fn133_calc_iq__qd1_dn14,)
    } else {
        (locals.var_fn133_calc_iq__qd, locals.var_fn133_calc_iq__qd_dn2, locals.var_fn133_calc_iq__qd_dn4, locals.var_fn133_calc_iq__qd_dn5, locals.var_fn133_calc_iq__qd_dn7, locals.var_fn133_calc_iq__qd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd = assign12630_e13241;
        locals.var_fn133_calc_iq__qd_dn2 = assign12630_e13241_d_n2;
        locals.var_fn133_calc_iq__qd_dn4 = assign12630_e13241_d_n4;
        locals.var_fn133_calc_iq__qd_dn5 = assign12630_e13241_d_n5;
        locals.var_fn133_calc_iq__qd_dn7 = assign12630_e13241_d_n7;
        locals.var_fn133_calc_iq__qd_dn14 = assign12630_e13241_d_n14;

        let (assign12640_e13255, assign12640_e13255_d_n2, assign12640_e13255_d_n4, assign12640_e13255_d_n5, assign12640_e13255_d_n7, assign12640_e13255_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12640_e13245: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12640_e13247: f64 = (assign12640_e13245 * locals.var_fn133_calc_iq__lin);
        let assign12640_e13249: f64 = (assign12640_e13247 * locals.var_fn133_calc_iq__type);
        let assign12640_e13251: f64 = (assign12640_e13249 * locals.var_fn133_calc_iq__qs);
        let assign12640_e13253: f64 = (assign12640_e13251 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12640_e13253, ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn4) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgsout = assign12640_e13255;
        locals.var_fn133_calc_iq__qgsout_dn2 = assign12640_e13255_d_n2;
        locals.var_fn133_calc_iq__qgsout_dn4 = assign12640_e13255_d_n4;
        locals.var_fn133_calc_iq__qgsout_dn5 = assign12640_e13255_d_n5;
        locals.var_fn133_calc_iq__qgsout_dn7 = assign12640_e13255_d_n7;
        locals.var_fn133_calc_iq__qgsout_dn14 = assign12640_e13255_d_n14;

        let (assign12650_e13269, assign12650_e13269_d_n2, assign12650_e13269_d_n4, assign12650_e13269_d_n5, assign12650_e13269_d_n7, assign12650_e13269_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12650_e13259: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12650_e13261: f64 = (assign12650_e13259 * locals.var_fn133_calc_iq__lin);
        let assign12650_e13263: f64 = (assign12650_e13261 * locals.var_fn133_calc_iq__type);
        let assign12650_e13265: f64 = (assign12650_e13263 * locals.var_fn133_calc_iq__qd);
        let assign12650_e13267: f64 = (assign12650_e13265 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12650_e13267, ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn4) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgdout = assign12650_e13269;
        locals.var_fn133_calc_iq__qgdout_dn2 = assign12650_e13269_d_n2;
        locals.var_fn133_calc_iq__qgdout_dn4 = assign12650_e13269_d_n4;
        locals.var_fn133_calc_iq__qgdout_dn5 = assign12650_e13269_d_n5;
        locals.var_fn133_calc_iq__qgdout_dn7 = assign12650_e13269_d_n7;
        locals.var_fn133_calc_iq__qgdout_dn14 = assign12650_e13269_d_n14;

        let assign12660_e13272: f64 = if locals.var_fn133_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign12660_e13272;

        let (assign12670_e13288, assign12670_e13288_d_n2, assign12670_e13288_d_n4, assign12670_e13288_d_n5, assign12670_e13288_d_n7,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12670_e13280: f64 = (p.p51 * 0.5);
        let assign12670_e13282: f64 = (assign12670_e13280 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12670_e13283: f64 = (locals.var_fn133_calc_iq__vtof - assign12670_e13282);
        let assign12670_e13284: f64 = (locals.var_fn133_calc_iq__vcin - assign12670_e13283);
        let assign12670_e13286: f64 = (assign12670_e13284 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12670_e13286, (locals.var_fn133_calc_iq__vcin_dn2 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12670_e13280 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12670_e13284 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vcin_dn5 / locals.var_fn133_calc_iq__two_n_phit0), (locals.var_fn133_calc_iq__vcin_dn7 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etac, locals.var_fn133_calc_iq__etac_dn2, locals.var_fn133_calc_iq__etac_dn4, locals.var_fn133_calc_iq__etac_dn5, locals.var_fn133_calc_iq__etac_dn7,)
    }
};
        locals.var_fn133_calc_iq__etac = assign12670_e13288;
        locals.var_fn133_calc_iq__etac_dn2 = assign12670_e13288_d_n2;
        locals.var_fn133_calc_iq__etac_dn4 = assign12670_e13288_d_n4;
        locals.var_fn133_calc_iq__etac_dn5 = assign12670_e13288_d_n5;
        locals.var_fn133_calc_iq__etac_dn7 = assign12670_e13288_d_n7;

        let assign12680_e13291: f64 = if locals.var_fn133_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign12680_e13291;

        let (assign12690_e13299, assign12690_e13299_d_n2, assign12690_e13299_d_n3, assign12690_e13299_d_n4, assign12690_e13299_d_n5, assign12690_e13299_d_n7, assign12690_e13299_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 != 0.0)) {
        (locals.var_fn133_calc_iq__etac, locals.var_fn133_calc_iq__etac_dn2, 0.0, locals.var_fn133_calc_iq__etac_dn4, locals.var_fn133_calc_iq__etac_dn5, locals.var_fn133_calc_iq__etac_dn7, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12690_e13299;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12690_e13299_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12690_e13299_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12690_e13299_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12690_e13299_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12690_e13299_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12690_e13299_d_n14;

        let assign12700_e13302: f64 = (-50.0);
        let assign12700_e13303: f64 = if locals.var_fn133_calc_iq__etac < assign12700_e13302 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign12700_e13303;

        let (assign12710_e13315, assign12710_e13315_d_n2, assign12710_e13315_d_n3, assign12710_e13315_d_n4, assign12710_e13315_d_n5, assign12710_e13315_d_n7, assign12710_e13315_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 == 0.0)) && (locals.var_guard161 != 0.0)) {
        let assign12710_e13313: f64 = (locals.var_fn133_calc_iq__etac).exp();
        (assign12710_e13313, (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn2), 0.0, (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn4), (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn5), (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn7), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12710_e13315;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12710_e13315_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12710_e13315_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12710_e13315_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12710_e13315_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12710_e13315_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12710_e13315_d_n14;

        let (assign12720_e13331, assign12720_e13331_d_n2, assign12720_e13331_d_n3, assign12720_e13331_d_n4, assign12720_e13331_d_n5, assign12720_e13331_d_n7, assign12720_e13331_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 == 0.0)) && (locals.var_guard161 == 0.0)) {
        let assign12720_e13327: f64 = (locals.var_fn133_calc_iq__etac).exp();
        let assign12720_e13328: f64 = (1.0 + assign12720_e13327);
        let assign12720_e13329: f64 = (assign12720_e13328).ln();
        (assign12720_e13329, ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn2) / assign12720_e13328), 0.0, ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn4) / assign12720_e13328), ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn5) / assign12720_e13328), ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn7) / assign12720_e13328), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12720_e13331;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12720_e13331_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12720_e13331_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12720_e13331_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12720_e13331_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12720_e13331_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12720_e13331_d_n14;

        let (assign12730_e13349, assign12730_e13349_d_n2, assign12730_e13349_d_n3, assign12730_e13349_d_n4, assign12730_e13349_d_n5, assign12730_e13349_d_n7, assign12730_e13349_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12730_e13337: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12730_e13339: f64 = (assign12730_e13337 * locals.var_fn133_calc_iq__type);
        let assign12730_e13341: f64 = (assign12730_e13339 * locals.var_fn133_calc_iq__cc);
        let assign12730_e13343: f64 = (assign12730_e13341 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12730_e13345: f64 = (assign12730_e13343 * locals.var_fn133_calc_iq__exparg);
        let assign12730_e13347: f64 = (assign12730_e13345 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12730_e13347, ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((((assign12730_e13339 * locals.var_fn133_calc_iq__cc_dn4) * locals.var_fn133_calc_iq__two_n_phit0) + (assign12730_e13341 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) * locals.var_fn133_calc_iq__exparg) + (assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign12730_e13349;
        locals.var_fn133_calc_iq__qcout_dn2 = assign12730_e13349_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign12730_e13349_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign12730_e13349_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign12730_e13349_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign12730_e13349_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign12730_e13349_d_n14;

        let (assign12740_e13365, assign12740_e13365_d_n3, assign12740_e13365_d_n4, assign12740_e13365_d_n5,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12740_e13357: f64 = (p.p51 * 0.5);
        let assign12740_e13359: f64 = (assign12740_e13357 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12740_e13360: f64 = (locals.var_fn133_calc_iq__vtof - assign12740_e13359);
        let assign12740_e13361: f64 = (locals.var_fn133_calc_iq__vbin - assign12740_e13360);
        let assign12740_e13363: f64 = (assign12740_e13361 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12740_e13363, (locals.var_fn133_calc_iq__vbin_dn3 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12740_e13357 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12740_e13361 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vbin_dn5 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etab, locals.var_fn133_calc_iq__etab_dn3, locals.var_fn133_calc_iq__etab_dn4, locals.var_fn133_calc_iq__etab_dn5,)
    }
};
        locals.var_fn133_calc_iq__etab = assign12740_e13365;
        locals.var_fn133_calc_iq__etab_dn3 = assign12740_e13365_d_n3;
        locals.var_fn133_calc_iq__etab_dn4 = assign12740_e13365_d_n4;
        locals.var_fn133_calc_iq__etab_dn5 = assign12740_e13365_d_n5;

        let assign12750_e13368: f64 = if locals.var_fn133_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign12750_e13368;

        let (assign12760_e13376, assign12760_e13376_d_n2, assign12760_e13376_d_n3, assign12760_e13376_d_n4, assign12760_e13376_d_n5, assign12760_e13376_d_n7, assign12760_e13376_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 != 0.0)) {
        (locals.var_fn133_calc_iq__etab, 0.0, locals.var_fn133_calc_iq__etab_dn3, locals.var_fn133_calc_iq__etab_dn4, locals.var_fn133_calc_iq__etab_dn5, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12760_e13376;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12760_e13376_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12760_e13376_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12760_e13376_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12760_e13376_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12760_e13376_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12760_e13376_d_n14;

        let assign12770_e13379: f64 = (-50.0);
        let assign12770_e13380: f64 = if locals.var_fn133_calc_iq__etab < assign12770_e13379 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign12770_e13380;

        let (assign12780_e13392, assign12780_e13392_d_n2, assign12780_e13392_d_n3, assign12780_e13392_d_n4, assign12780_e13392_d_n5, assign12780_e13392_d_n7, assign12780_e13392_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign12780_e13390: f64 = (locals.var_fn133_calc_iq__etab).exp();
        (assign12780_e13390, 0.0, (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn3), (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn4), (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn5), 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12780_e13392;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12780_e13392_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12780_e13392_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12780_e13392_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12780_e13392_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12780_e13392_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12780_e13392_d_n14;

        let (assign12790_e13408, assign12790_e13408_d_n2, assign12790_e13408_d_n3, assign12790_e13408_d_n4, assign12790_e13408_d_n5, assign12790_e13408_d_n7, assign12790_e13408_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) {
        let assign12790_e13404: f64 = (locals.var_fn133_calc_iq__etab).exp();
        let assign12790_e13405: f64 = (1.0 + assign12790_e13404);
        let assign12790_e13406: f64 = (assign12790_e13405).ln();
        (assign12790_e13406, 0.0, ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn3) / assign12790_e13405), ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn4) / assign12790_e13405), ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn5) / assign12790_e13405), 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12790_e13408;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12790_e13408_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12790_e13408_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12790_e13408_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12790_e13408_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12790_e13408_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12790_e13408_d_n14;

        let (assign12800_e13426, assign12800_e13426_d_n2, assign12800_e13426_d_n3, assign12800_e13426_d_n4, assign12800_e13426_d_n5, assign12800_e13426_d_n7, assign12800_e13426_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12800_e13414: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12800_e13416: f64 = (assign12800_e13414 * locals.var_fn133_calc_iq__type);
        let assign12800_e13418: f64 = (assign12800_e13416 * locals.var_fn133_calc_iq__cb);
        let assign12800_e13420: f64 = (assign12800_e13418 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12800_e13422: f64 = (assign12800_e13420 * locals.var_fn133_calc_iq__exparg);
        let assign12800_e13424: f64 = (assign12800_e13422 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12800_e13424, ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((((assign12800_e13416 * locals.var_fn133_calc_iq__cb_dn4) * locals.var_fn133_calc_iq__two_n_phit0) + (assign12800_e13418 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) * locals.var_fn133_calc_iq__exparg) + (assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign12800_e13426;
        locals.var_fn133_calc_iq__qbout_dn2 = assign12800_e13426_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign12800_e13426_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign12800_e13426_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign12800_e13426_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign12800_e13426_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign12800_e13426_d_n14;

        let (assign12810_e13433, assign12810_e13433_d_n2, assign12810_e13433_d_n3, assign12810_e13433_d_n4, assign12810_e13433_d_n5, assign12810_e13433_d_n7, assign12810_e13433_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign12810_e13433;
        locals.var_fn133_calc_iq__qcout_dn2 = assign12810_e13433_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign12810_e13433_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign12810_e13433_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign12810_e13433_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign12810_e13433_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign12810_e13433_d_n14;

        let (assign12820_e13440, assign12820_e13440_d_n2, assign12820_e13440_d_n3, assign12820_e13440_d_n4, assign12820_e13440_d_n5, assign12820_e13440_d_n7, assign12820_e13440_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign12820_e13440;
        locals.var_fn133_calc_iq__qbout_dn2 = assign12820_e13440_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign12820_e13440_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign12820_e13440_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign12820_e13440_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign12820_e13440_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign12820_e13440_d_n14;

        let assign12830_e13443: f64 = if locals.var_fn133_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign12830_e13443;

        let (assign12840_e13459, assign12840_e13459_d_n2, assign12840_e13459_d_n4, assign12840_e13459_d_n5, assign12840_e13459_d_n7,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) {
        let assign12840_e13451: f64 = (p.p51 * 0.5);
        let assign12840_e13453: f64 = (assign12840_e13451 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12840_e13454: f64 = (locals.var_fn133_calc_iq__vtof - assign12840_e13453);
        let assign12840_e13455: f64 = (locals.var_fn133_calc_iq__vgsin - assign12840_e13454);
        let assign12840_e13457: f64 = (assign12840_e13455 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12840_e13457, (locals.var_fn133_calc_iq__vgsin_dn2 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12840_e13451 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12840_e13455 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vgsin_dn5 / locals.var_fn133_calc_iq__two_n_phit0), (locals.var_fn133_calc_iq__vgsin_dn7 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etags, locals.var_fn133_calc_iq__etags_dn2, locals.var_fn133_calc_iq__etags_dn4, locals.var_fn133_calc_iq__etags_dn5, locals.var_fn133_calc_iq__etags_dn7,)
    }
};
        locals.var_fn133_calc_iq__etags = assign12840_e13459;
        locals.var_fn133_calc_iq__etags_dn2 = assign12840_e13459_d_n2;
        locals.var_fn133_calc_iq__etags_dn4 = assign12840_e13459_d_n4;
        locals.var_fn133_calc_iq__etags_dn5 = assign12840_e13459_d_n5;
        locals.var_fn133_calc_iq__etags_dn7 = assign12840_e13459_d_n7;

        let assign12850_e13462: f64 = if locals.var_fn133_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign12850_e13462;

        let (assign12860_e13470, assign12860_e13470_d_n2, assign12860_e13470_d_n3, assign12860_e13470_d_n4, assign12860_e13470_d_n5, assign12860_e13470_d_n7, assign12860_e13470_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) {
        (locals.var_fn133_calc_iq__etags, locals.var_fn133_calc_iq__etags_dn2, 0.0, locals.var_fn133_calc_iq__etags_dn4, locals.var_fn133_calc_iq__etags_dn5, locals.var_fn133_calc_iq__etags_dn7, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12860_e13470;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12860_e13470_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12860_e13470_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12860_e13470_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12860_e13470_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12860_e13470_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12860_e13470_d_n14;

        let assign12870_e13473: f64 = (-50.0);
        let assign12870_e13474: f64 = if locals.var_fn133_calc_iq__etags < assign12870_e13473 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12870_e13474;

        let (assign12880_e13486, assign12880_e13486_d_n2, assign12880_e13486_d_n3, assign12880_e13486_d_n4, assign12880_e13486_d_n5, assign12880_e13486_d_n7, assign12880_e13486_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign12880_e13484: f64 = (locals.var_fn133_calc_iq__etags).exp();
        (assign12880_e13484, (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn2), 0.0, (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn4), (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn5), (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn7), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12880_e13486;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12880_e13486_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12880_e13486_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12880_e13486_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12880_e13486_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12880_e13486_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12880_e13486_d_n14;

        let (assign12890_e13502, assign12890_e13502_d_n2, assign12890_e13502_d_n3, assign12890_e13502_d_n4, assign12890_e13502_d_n5, assign12890_e13502_d_n7, assign12890_e13502_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign12890_e13498: f64 = (locals.var_fn133_calc_iq__etags).exp();
        let assign12890_e13499: f64 = (1.0 + assign12890_e13498);
        let assign12890_e13500: f64 = (assign12890_e13499).ln();
        (assign12890_e13500, ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn2) / assign12890_e13499), 0.0, ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn4) / assign12890_e13499), ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn5) / assign12890_e13499), ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn7) / assign12890_e13499), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12890_e13502;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12890_e13502_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12890_e13502_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12890_e13502_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12890_e13502_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12890_e13502_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12890_e13502_d_n14;

        let (assign12900_e13520, assign12900_e13520_d_n2, assign12900_e13520_d_n3, assign12900_e13520_d_n4, assign12900_e13520_d_n5, assign12900_e13520_d_n7, assign12900_e13520_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) {
        let assign12900_e13508: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12900_e13510: f64 = (assign12900_e13508 * locals.var_fn133_calc_iq__type);
        let assign12900_e13512: f64 = (assign12900_e13510 * locals.var_fn133_calc_iq__cs);
        let assign12900_e13514: f64 = (assign12900_e13512 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12900_e13516: f64 = (assign12900_e13514 * locals.var_fn133_calc_iq__exparg);
        let assign12900_e13518: f64 = (assign12900_e13516 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12900_e13518, ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12900_e13512 * locals.var_fn133_calc_iq__two_n_phit0_dn4) * locals.var_fn133_calc_iq__exparg) + (assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign12900_e13520;
        locals.var_fn133_calc_iq__qsout_dn2 = assign12900_e13520_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign12900_e13520_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign12900_e13520_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign12900_e13520_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign12900_e13520_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign12900_e13520_d_n14;

        let (assign12910_e13527, assign12910_e13527_d_n2, assign12910_e13527_d_n3, assign12910_e13527_d_n4, assign12910_e13527_d_n5, assign12910_e13527_d_n7, assign12910_e13527_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign12910_e13527;
        locals.var_fn133_calc_iq__qsout_dn2 = assign12910_e13527_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign12910_e13527_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign12910_e13527_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign12910_e13527_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign12910_e13527_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign12910_e13527_d_n14;

        let (assign12920_e13531, assign12920_e13531_d_n2, assign12920_e13531_d_n3, assign12920_e13531_d_n4, assign12920_e13531_d_n5, assign12920_e13531_d_n7, assign12920_e13531_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__idsout, locals.var_fn133_calc_iq__idsout_dn2, locals.var_fn133_calc_iq__idsout_dn3, locals.var_fn133_calc_iq__idsout_dn4, locals.var_fn133_calc_iq__idsout_dn5, locals.var_fn133_calc_iq__idsout_dn7, locals.var_fn133_calc_iq__idsout_dn14,)
    } else {
        (locals.var_fn133_calc_iq__return, locals.var_fn133_calc_iq__return_dn2, locals.var_fn133_calc_iq__return_dn3, locals.var_fn133_calc_iq__return_dn4, locals.var_fn133_calc_iq__return_dn5, locals.var_fn133_calc_iq__return_dn7, locals.var_fn133_calc_iq__return_dn14,)
    }
};
        locals.var_fn133_calc_iq__return = assign12920_e13531;
        locals.var_fn133_calc_iq__return_dn2 = assign12920_e13531_d_n2;
        locals.var_fn133_calc_iq__return_dn3 = assign12920_e13531_d_n3;
        locals.var_fn133_calc_iq__return_dn4 = assign12920_e13531_d_n4;
        locals.var_fn133_calc_iq__return_dn5 = assign12920_e13531_d_n5;
        locals.var_fn133_calc_iq__return_dn7 = assign12920_e13531_d_n7;
        locals.var_fn133_calc_iq__return_dn14 = assign12920_e13531_d_n14;

        let (assign12930_e13535, assign12930_e13535_d_n2, assign12930_e13535_d_n3, assign12930_e13535_d_n4, assign12930_e13535_d_n5, assign12930_e13535_d_n7, assign12930_e13535_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__idsout, locals.var_fn133_calc_iq__idsout_dn2, locals.var_fn133_calc_iq__idsout_dn3, locals.var_fn133_calc_iq__idsout_dn4, locals.var_fn133_calc_iq__idsout_dn5, locals.var_fn133_calc_iq__idsout_dn7, locals.var_fn133_calc_iq__idsout_dn14,)
    } else {
        (locals.var_idsfp1, locals.var_idsfp1_dn2, locals.var_idsfp1_dn3, locals.var_idsfp1_dn4, locals.var_idsfp1_dn5, locals.var_idsfp1_dn7, locals.var_idsfp1_dn14,)
    }
};
        locals.var_idsfp1 = assign12930_e13535;
        locals.var_idsfp1_dn2 = assign12930_e13535_d_n2;
        locals.var_idsfp1_dn3 = assign12930_e13535_d_n3;
        locals.var_idsfp1_dn4 = assign12930_e13535_d_n4;
        locals.var_idsfp1_dn5 = assign12930_e13535_d_n5;
        locals.var_idsfp1_dn7 = assign12930_e13535_d_n7;
        locals.var_idsfp1_dn14 = assign12930_e13535_d_n14;

        let (assign12940_e13539, assign12940_e13539_d_n2, assign12940_e13539_d_n4, assign12940_e13539_d_n5, assign12940_e13539_d_n7, assign12940_e13539_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    } else {
        (locals.var_qgsfp1, locals.var_qgsfp1_dn2, locals.var_qgsfp1_dn4, locals.var_qgsfp1_dn5, locals.var_qgsfp1_dn7, locals.var_qgsfp1_dn14,)
    }
};
        locals.var_qgsfp1 = assign12940_e13539;
        locals.var_qgsfp1_dn2 = assign12940_e13539_d_n2;
        locals.var_qgsfp1_dn4 = assign12940_e13539_d_n4;
        locals.var_qgsfp1_dn5 = assign12940_e13539_d_n5;
        locals.var_qgsfp1_dn7 = assign12940_e13539_d_n7;
        locals.var_qgsfp1_dn14 = assign12940_e13539_d_n14;

    }

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12950_e13543, assign12950_e13543_d_n2, assign12950_e13543_d_n4, assign12950_e13543_d_n5, assign12950_e13543_d_n7, assign12950_e13543_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    } else {
        (locals.var_qgdfp1, locals.var_qgdfp1_dn2, locals.var_qgdfp1_dn4, locals.var_qgdfp1_dn5, locals.var_qgdfp1_dn7, locals.var_qgdfp1_dn14,)
    }
};
        locals.var_qgdfp1 = assign12950_e13543;
        locals.var_qgdfp1_dn2 = assign12950_e13543_d_n2;
        locals.var_qgdfp1_dn4 = assign12950_e13543_d_n4;
        locals.var_qgdfp1_dn5 = assign12950_e13543_d_n5;
        locals.var_qgdfp1_dn7 = assign12950_e13543_d_n7;
        locals.var_qgdfp1_dn14 = assign12950_e13543_d_n14;

        let (assign12960_e13547, assign12960_e13547_d_n2, assign12960_e13547_d_n3, assign12960_e13547_d_n4, assign12960_e13547_d_n5, assign12960_e13547_d_n7, assign12960_e13547_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    } else {
        (locals.var_qcfp1, locals.var_qcfp1_dn2, locals.var_qcfp1_dn3, locals.var_qcfp1_dn4, locals.var_qcfp1_dn5, locals.var_qcfp1_dn7, locals.var_qcfp1_dn14,)
    }
};
        locals.var_qcfp1 = assign12960_e13547;
        locals.var_qcfp1_dn2 = assign12960_e13547_d_n2;
        locals.var_qcfp1_dn3 = assign12960_e13547_d_n3;
        locals.var_qcfp1_dn4 = assign12960_e13547_d_n4;
        locals.var_qcfp1_dn5 = assign12960_e13547_d_n5;
        locals.var_qcfp1_dn7 = assign12960_e13547_d_n7;
        locals.var_qcfp1_dn14 = assign12960_e13547_d_n14;

        let (assign12970_e13551, assign12970_e13551_d_n2, assign12970_e13551_d_n3, assign12970_e13551_d_n4, assign12970_e13551_d_n5, assign12970_e13551_d_n7, assign12970_e13551_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    } else {
        (locals.var_qbfp1, locals.var_qbfp1_dn2, locals.var_qbfp1_dn3, locals.var_qbfp1_dn4, locals.var_qbfp1_dn5, locals.var_qbfp1_dn7, locals.var_qbfp1_dn14,)
    }
};
        locals.var_qbfp1 = assign12970_e13551;
        locals.var_qbfp1_dn2 = assign12970_e13551_d_n2;
        locals.var_qbfp1_dn3 = assign12970_e13551_d_n3;
        locals.var_qbfp1_dn4 = assign12970_e13551_d_n4;
        locals.var_qbfp1_dn5 = assign12970_e13551_d_n5;
        locals.var_qbfp1_dn7 = assign12970_e13551_d_n7;
        locals.var_qbfp1_dn14 = assign12970_e13551_d_n14;

        let (assign12980_e13555, assign12980_e13555_d_n2, assign12980_e13555_d_n3, assign12980_e13555_d_n4, assign12980_e13555_d_n5, assign12980_e13555_d_n7, assign12980_e13555_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    } else {
        (locals.var_qsfp1, locals.var_qsfp1_dn2, locals.var_qsfp1_dn3, locals.var_qsfp1_dn4, locals.var_qsfp1_dn5, locals.var_qsfp1_dn7, locals.var_qsfp1_dn14,)
    }
};
        locals.var_qsfp1 = assign12980_e13555;
        locals.var_qsfp1_dn2 = assign12980_e13555_d_n2;
        locals.var_qsfp1_dn3 = assign12980_e13555_d_n3;
        locals.var_qsfp1_dn4 = assign12980_e13555_d_n4;
        locals.var_qsfp1_dn5 = assign12980_e13555_d_n5;
        locals.var_qsfp1_dn7 = assign12980_e13555_d_n7;
        locals.var_qsfp1_dn14 = assign12980_e13555_d_n14;

        let (assign13010_e13567, assign13010_e13567_d_n2, assign13010_e13567_d_n3, assign13010_e13567_d_n4, assign13010_e13567_d_n5, assign13010_e13567_d_n7, assign13010_e13567_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__return, locals.var_fn133_calc_iq__return_dn2, locals.var_fn133_calc_iq__return_dn3, locals.var_fn133_calc_iq__return_dn4, locals.var_fn133_calc_iq__return_dn5, locals.var_fn133_calc_iq__return_dn7, locals.var_fn133_calc_iq__return_dn14,)
    } else {
        (locals.var_idsfp1, locals.var_idsfp1_dn2, locals.var_idsfp1_dn3, locals.var_idsfp1_dn4, locals.var_idsfp1_dn5, locals.var_idsfp1_dn7, locals.var_idsfp1_dn14,)
    }
};
        locals.var_idsfp1 = assign13010_e13567;
        locals.var_idsfp1_dn2 = assign13010_e13567_d_n2;
        locals.var_idsfp1_dn3 = assign13010_e13567_d_n3;
        locals.var_idsfp1_dn4 = assign13010_e13567_d_n4;
        locals.var_idsfp1_dn5 = assign13010_e13567_d_n5;
        locals.var_idsfp1_dn7 = assign13010_e13567_d_n7;
        locals.var_idsfp1_dn14 = assign13010_e13567_d_n14;

        let assign13020_e13570: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign13020_e13570;

        locals.var_idsfps1 = 0.0;
        locals.var_idsfps1_dn2 = 0.0;
        locals.var_idsfps1_dn3 = 0.0;
        locals.var_idsfps1_dn4 = 0.0;
        locals.var_idsfps1_dn7 = 0.0;
        locals.var_idsfps1_dn9 = 0.0;
        locals.var_idsfps1_dn10 = 0.0;

        locals.var_qgsfps1 = 0.0;
        locals.var_qgsfps1_dn2 = 0.0;
        locals.var_qgsfps1_dn4 = 0.0;
        locals.var_qgsfps1_dn7 = 0.0;
        locals.var_qgsfps1_dn9 = 0.0;
        locals.var_qgsfps1_dn10 = 0.0;

        locals.var_qgdfps1 = 0.0;
        locals.var_qgdfps1_dn2 = 0.0;
        locals.var_qgdfps1_dn4 = 0.0;
        locals.var_qgdfps1_dn7 = 0.0;
        locals.var_qgdfps1_dn9 = 0.0;
        locals.var_qgdfps1_dn10 = 0.0;

        locals.var_qcfps1 = 0.0;
        locals.var_qcfps1_dn2 = 0.0;
        locals.var_qcfps1_dn3 = 0.0;
        locals.var_qcfps1_dn4 = 0.0;
        locals.var_qcfps1_dn7 = 0.0;
        locals.var_qcfps1_dn9 = 0.0;
        locals.var_qcfps1_dn10 = 0.0;

        locals.var_qbfps1 = 0.0;
        locals.var_qbfps1_dn2 = 0.0;
        locals.var_qbfps1_dn3 = 0.0;
        locals.var_qbfps1_dn4 = 0.0;
        locals.var_qbfps1_dn7 = 0.0;
        locals.var_qbfps1_dn9 = 0.0;
        locals.var_qbfps1_dn10 = 0.0;

        locals.var_qsfps1 = 0.0;
        locals.var_qsfps1_dn2 = 0.0;
        locals.var_qsfps1_dn3 = 0.0;
        locals.var_qsfps1_dn4 = 0.0;
        locals.var_qsfps1_dn7 = 0.0;
        locals.var_qsfps1_dn9 = 0.0;
        locals.var_qsfps1_dn10 = 0.0;

        let assign13110_e13581: f64 = if p.p79 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign13110_e13581;

        let (assign13120_e13585, assign13120_e13585_d_n2, assign13120_e13585_d_n3, assign13120_e13585_d_n4, assign13120_e13585_d_n7, assign13120_e13585_d_n9, assign13120_e13585_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__return, locals.var_fn169_calc_iq__return_dn2, locals.var_fn169_calc_iq__return_dn3, locals.var_fn169_calc_iq__return_dn4, locals.var_fn169_calc_iq__return_dn7, locals.var_fn169_calc_iq__return_dn9, locals.var_fn169_calc_iq__return_dn10,)
    }
};
        locals.var_fn169_calc_iq__return = assign13120_e13585;
        locals.var_fn169_calc_iq__return_dn2 = assign13120_e13585_d_n2;
        locals.var_fn169_calc_iq__return_dn3 = assign13120_e13585_d_n3;
        locals.var_fn169_calc_iq__return_dn4 = assign13120_e13585_d_n4;
        locals.var_fn169_calc_iq__return_dn7 = assign13120_e13585_d_n7;
        locals.var_fn169_calc_iq__return_dn9 = assign13120_e13585_d_n9;
        locals.var_fn169_calc_iq__return_dn10 = assign13120_e13585_d_n10;

        let (assign13130_e13589, assign13130_e13589_d_n2, assign13130_e13589_d_n3, assign13130_e13589_d_n4, assign13130_e13589_d_n7, assign13130_e13589_d_n9, assign13130_e13589_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__idsout, locals.var_fn169_calc_iq__idsout_dn2, locals.var_fn169_calc_iq__idsout_dn3, locals.var_fn169_calc_iq__idsout_dn4, locals.var_fn169_calc_iq__idsout_dn7, locals.var_fn169_calc_iq__idsout_dn9, locals.var_fn169_calc_iq__idsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__idsout = assign13130_e13589;
        locals.var_fn169_calc_iq__idsout_dn2 = assign13130_e13589_d_n2;
        locals.var_fn169_calc_iq__idsout_dn3 = assign13130_e13589_d_n3;
        locals.var_fn169_calc_iq__idsout_dn4 = assign13130_e13589_d_n4;
        locals.var_fn169_calc_iq__idsout_dn7 = assign13130_e13589_d_n7;
        locals.var_fn169_calc_iq__idsout_dn9 = assign13130_e13589_d_n9;
        locals.var_fn169_calc_iq__idsout_dn10 = assign13130_e13589_d_n10;

        let (assign13140_e13593, assign13140_e13593_d_n2, assign13140_e13593_d_n4, assign13140_e13593_d_n7, assign13140_e13593_d_n9, assign13140_e13593_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgsout = assign13140_e13593;
        locals.var_fn169_calc_iq__qgsout_dn2 = assign13140_e13593_d_n2;
        locals.var_fn169_calc_iq__qgsout_dn4 = assign13140_e13593_d_n4;
        locals.var_fn169_calc_iq__qgsout_dn7 = assign13140_e13593_d_n7;
        locals.var_fn169_calc_iq__qgsout_dn9 = assign13140_e13593_d_n9;
        locals.var_fn169_calc_iq__qgsout_dn10 = assign13140_e13593_d_n10;

        let (assign13150_e13597, assign13150_e13597_d_n2, assign13150_e13597_d_n4, assign13150_e13597_d_n7, assign13150_e13597_d_n9, assign13150_e13597_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgdout = assign13150_e13597;
        locals.var_fn169_calc_iq__qgdout_dn2 = assign13150_e13597_d_n2;
        locals.var_fn169_calc_iq__qgdout_dn4 = assign13150_e13597_d_n4;
        locals.var_fn169_calc_iq__qgdout_dn7 = assign13150_e13597_d_n7;
        locals.var_fn169_calc_iq__qgdout_dn9 = assign13150_e13597_d_n9;
        locals.var_fn169_calc_iq__qgdout_dn10 = assign13150_e13597_d_n10;

        let (assign13160_e13601, assign13160_e13601_d_n2, assign13160_e13601_d_n3, assign13160_e13601_d_n4, assign13160_e13601_d_n7, assign13160_e13601_d_n9, assign13160_e13601_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign13160_e13601;
        locals.var_fn169_calc_iq__qcout_dn2 = assign13160_e13601_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign13160_e13601_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign13160_e13601_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign13160_e13601_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign13160_e13601_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign13160_e13601_d_n10;

        let (assign13170_e13605, assign13170_e13605_d_n2, assign13170_e13605_d_n3, assign13170_e13605_d_n4, assign13170_e13605_d_n7, assign13170_e13605_d_n9, assign13170_e13605_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign13170_e13605;
        locals.var_fn169_calc_iq__qbout_dn2 = assign13170_e13605_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign13170_e13605_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign13170_e13605_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign13170_e13605_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign13170_e13605_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign13170_e13605_d_n10;

        let (assign13180_e13609, assign13180_e13609_d_n2, assign13180_e13609_d_n3, assign13180_e13609_d_n4, assign13180_e13609_d_n7, assign13180_e13609_d_n9, assign13180_e13609_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign13180_e13609;
        locals.var_fn169_calc_iq__qsout_dn2 = assign13180_e13609_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign13180_e13609_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign13180_e13609_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign13180_e13609_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign13180_e13609_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign13180_e13609_d_n10;

        let (assign13190_e13613, assign13190_e13613_d_n4, assign13190_e13613_d_n9, assign13190_e13613_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vtdibl, locals.var_fn169_calc_iq__vtdibl_dn4, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vtdibl = assign13190_e13613;
        locals.var_fn169_calc_iq__vtdibl_dn4 = assign13190_e13613_d_n4;
        locals.var_fn169_calc_iq__vtdibl_dn9 = assign13190_e13613_d_n9;
        locals.var_fn169_calc_iq__vtdibl_dn10 = assign13190_e13613_d_n10;

        let (assign13200_e13617, assign13200_e13617_d_n2, assign13200_e13617_d_n3, assign13200_e13617_d_n4, assign13200_e13617_d_n7, assign13200_e13617_d_n9, assign13200_e13617_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat1, locals.var_fn169_calc_iq__vdsat1_dn2, locals.var_fn169_calc_iq__vdsat1_dn3, locals.var_fn169_calc_iq__vdsat1_dn4, locals.var_fn169_calc_iq__vdsat1_dn7, locals.var_fn169_calc_iq__vdsat1_dn9, locals.var_fn169_calc_iq__vdsat1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat1 = assign13200_e13617;
        locals.var_fn169_calc_iq__vdsat1_dn2 = assign13200_e13617_d_n2;
        locals.var_fn169_calc_iq__vdsat1_dn3 = assign13200_e13617_d_n3;
        locals.var_fn169_calc_iq__vdsat1_dn4 = assign13200_e13617_d_n4;
        locals.var_fn169_calc_iq__vdsat1_dn7 = assign13200_e13617_d_n7;
        locals.var_fn169_calc_iq__vdsat1_dn9 = assign13200_e13617_d_n9;
        locals.var_fn169_calc_iq__vdsat1_dn10 = assign13200_e13617_d_n10;

        let (assign13210_e13621, assign13210_e13621_d_n2, assign13210_e13621_d_n7, assign13210_e13621_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vgsin, locals.var_fn169_calc_iq__vgsin_dn2, locals.var_fn169_calc_iq__vgsin_dn7, locals.var_fn169_calc_iq__vgsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgsin = assign13210_e13621;
        locals.var_fn169_calc_iq__vgsin_dn2 = assign13210_e13621_d_n2;
        locals.var_fn169_calc_iq__vgsin_dn7 = assign13210_e13621_d_n7;
        locals.var_fn169_calc_iq__vgsin_dn10 = assign13210_e13621_d_n10;

        let (assign13220_e13625, assign13220_e13625_d_n9, assign13220_e13625_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vdsfps1, locals.var_vdsfps1_dn9, locals.var_vdsfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vdsin, locals.var_fn169_calc_iq__vdsin_dn9, locals.var_fn169_calc_iq__vdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsin = assign13220_e13625;
        locals.var_fn169_calc_iq__vdsin_dn9 = assign13220_e13625_d_n9;
        locals.var_fn169_calc_iq__vdsin_dn10 = assign13220_e13625_d_n10;

        let (assign13230_e13629,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p85,)
    } else {
        (locals.var_fn169_calc_iq__qcbflag,)
    }
};
        locals.var_fn169_calc_iq__qcbflag = assign13230_e13629;

        let (assign13240_e13633, assign13240_e13633_d_n2, assign13240_e13633_d_n7, assign13240_e13633_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vcin, locals.var_fn169_calc_iq__vcin_dn2, locals.var_fn169_calc_iq__vcin_dn7, locals.var_fn169_calc_iq__vcin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vcin = assign13240_e13633;
        locals.var_fn169_calc_iq__vcin_dn2 = assign13240_e13633_d_n2;
        locals.var_fn169_calc_iq__vcin_dn7 = assign13240_e13633_d_n7;
        locals.var_fn169_calc_iq__vcin_dn10 = assign13240_e13633_d_n10;

        let (assign13250_e13637, assign13250_e13637_d_n3, assign13250_e13637_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vbfps1, locals.var_vbfps1_dn3, locals.var_vbfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vbin, locals.var_fn169_calc_iq__vbin_dn3, locals.var_fn169_calc_iq__vbin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vbin = assign13250_e13637;
        locals.var_fn169_calc_iq__vbin_dn3 = assign13250_e13637_d_n3;
        locals.var_fn169_calc_iq__vbin_dn10 = assign13250_e13637_d_n10;

        let (assign13260_e13641,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p83,)
    } else {
        (locals.var_fn169_calc_iq__qgsflag,)
    }
};
        locals.var_fn169_calc_iq__qgsflag = assign13260_e13641;

        let (assign13270_e13645, assign13270_e13645_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn169_calc_iq__tambin, locals.var_fn169_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tambin = assign13270_e13645;
        locals.var_fn169_calc_iq__tambin_dn4 = assign13270_e13645_d_n4;

        let (assign13280_e13649,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn169_calc_iq__tnomin,)
    }
};
        locals.var_fn169_calc_iq__tnomin = assign13280_e13649;

        let (assign13290_e13653, assign13290_e13653_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn169_calc_iq__phitin, locals.var_fn169_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn169_calc_iq__phitin = assign13290_e13653;
        locals.var_fn169_calc_iq__phitin_dn4 = assign13290_e13653_d_n4;

        let (assign13300_e13657,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn169_calc_iq__w,)
    }
};
        locals.var_fn169_calc_iq__w = assign13300_e13657;

        let (assign13310_e13661,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p79,)
    } else {
        (locals.var_fn169_calc_iq__lin,)
    }
};
        locals.var_fn169_calc_iq__lin = assign13310_e13661;

        let (assign13320_e13665, assign13320_e13665_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_cgfps1t, locals.var_cgfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cgin, locals.var_fn169_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn169_calc_iq__cgin = assign13320_e13665;
        locals.var_fn169_calc_iq__cgin_dn4 = assign13320_e13665_d_n4;

        let (assign13330_e13669,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p84,)
    } else {
        (locals.var_fn169_calc_iq__cs,)
    }
};
        locals.var_fn169_calc_iq__cs = assign13330_e13669;

        let (assign13340_e13673, assign13340_e13673_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_ccfps1t, locals.var_ccfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cc, locals.var_fn169_calc_iq__cc_dn4,)
    }
};
        locals.var_fn169_calc_iq__cc = assign13340_e13673;
        locals.var_fn169_calc_iq__cc_dn4 = assign13340_e13673_d_n4;

        let (assign13350_e13677, assign13350_e13677_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_cbfps1t, locals.var_cbfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cb, locals.var_fn169_calc_iq__cb_dn4,)
    }
};
        locals.var_fn169_calc_iq__cb = assign13350_e13677;
        locals.var_fn169_calc_iq__cb_dn4 = assign13350_e13677_d_n4;

        let (assign13360_e13681,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p80,)
    } else {
        (locals.var_fn169_calc_iq__vto,)
    }
};
        locals.var_fn169_calc_iq__vto = assign13360_e13681;

        let (assign13370_e13685,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p94,)
    } else {
        (locals.var_fn169_calc_iq__ss,)
    }
};
        locals.var_fn169_calc_iq__ss = assign13370_e13685;

        let (assign13380_e13689,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p93,)
    } else {
        (locals.var_fn169_calc_iq__delta1,)
    }
};
        locals.var_fn169_calc_iq__delta1 = assign13380_e13689;

        let (assign13390_e13693,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn169_calc_iq__delta2,)
    }
};
        locals.var_fn169_calc_iq__delta2 = assign13390_e13693;

        let (assign13400_e13697,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_fn169_calc_iq__nd,)
    }
};
        locals.var_fn169_calc_iq__nd = assign13400_e13697;

        let (assign13410_e13701,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p99,)
    } else {
        (locals.var_fn169_calc_iq__alpha,)
    }
};
        locals.var_fn169_calc_iq__alpha = assign13410_e13701;

        let (assign13420_e13705,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p90,)
    } else {
        (locals.var_fn169_calc_iq__vel0,)
    }
};
        locals.var_fn169_calc_iq__vel0 = assign13420_e13705;

        let (assign13430_e13709,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p91,)
    } else {
        (locals.var_fn169_calc_iq__mu0,)
    }
};
        locals.var_fn169_calc_iq__mu0 = assign13430_e13709;

        let (assign13440_e13713,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p92,)
    } else {
        (locals.var_fn169_calc_iq__beta,)
    }
};
        locals.var_fn169_calc_iq__beta = assign13440_e13713;

        let (assign13450_e13717,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p98,)
    } else {
        (locals.var_fn169_calc_iq__mtheta,)
    }
};
        locals.var_fn169_calc_iq__mtheta = assign13450_e13717;

        let (assign13460_e13721,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p97,)
    } else {
        (locals.var_fn169_calc_iq__vtheta,)
    }
};
        locals.var_fn169_calc_iq__vtheta = assign13460_e13721;

    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13470_e13725,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p96,)
    } else {
        (locals.var_fn169_calc_iq__vtzeta,)
    }
};
        locals.var_fn169_calc_iq__vtzeta = assign13470_e13725;

        let (assign13480_e13729,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn169_calc_iq__dibsat,)
    }
};
        locals.var_fn169_calc_iq__dibsat = assign13480_e13729;

        let (assign13490_e13733,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn169_calc_iq__epsilon,)
    }
};
        locals.var_fn169_calc_iq__epsilon = assign13490_e13733;

        let (assign13500_e13737,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn169_calc_iq__vzeta,)
    }
};
        locals.var_fn169_calc_iq__vzeta = assign13500_e13737;

        let (assign13510_e13741,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn169_calc_iq__lambda,)
    }
};
        locals.var_fn169_calc_iq__lambda = assign13510_e13741;

        let (assign13520_e13745,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn169_calc_iq__ngf,)
    }
};
        locals.var_fn169_calc_iq__ngf = assign13520_e13745;

        let (assign13530_e13749,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn169_calc_iq__type,)
    }
};
        locals.var_fn169_calc_iq__type = assign13530_e13749;

        let (assign13540_e13753,) = {
    if (locals.var_guard168 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn169_calc_iq__trapfracdl,)
    }
};
        locals.var_fn169_calc_iq__trapfracdl = assign13540_e13753;

        let (assign13550_e13757, assign13550_e13757_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__alpha_phit, locals.var_fn169_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn169_calc_iq__alpha_phit = assign13550_e13757;
        locals.var_fn169_calc_iq__alpha_phit_dn4 = assign13550_e13757_d_n4;

        let (assign13560_e13761, assign13560_e13761_d_n9, assign13560_e13761_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__delta, locals.var_fn169_calc_iq__delta_dn9, locals.var_fn169_calc_iq__delta_dn10,)
    }
};
        locals.var_fn169_calc_iq__delta = assign13560_e13761;
        locals.var_fn169_calc_iq__delta_dn9 = assign13560_e13761_d_n9;
        locals.var_fn169_calc_iq__delta_dn10 = assign13560_e13761_d_n10;

        let (assign13570_e13765, assign13570_e13765_d_n4, assign13570_e13765_d_n9, assign13570_e13765_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__n, locals.var_fn169_calc_iq__n_dn4, locals.var_fn169_calc_iq__n_dn9, locals.var_fn169_calc_iq__n_dn10,)
    }
};
        locals.var_fn169_calc_iq__n = assign13570_e13765;
        locals.var_fn169_calc_iq__n_dn4 = assign13570_e13765_d_n4;
        locals.var_fn169_calc_iq__n_dn9 = assign13570_e13765_d_n9;
        locals.var_fn169_calc_iq__n_dn10 = assign13570_e13765_d_n10;

        let (assign13580_e13769, assign13580_e13769_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vtof, locals.var_fn169_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn169_calc_iq__vtof = assign13580_e13769;
        locals.var_fn169_calc_iq__vtof_dn4 = assign13580_e13769_d_n4;

        let (assign13590_e13773, assign13590_e13773_d_n9, assign13590_e13773_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign13590_e13773;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign13590_e13773_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign13590_e13773_d_n10;

        let (assign13600_e13777, assign13600_e13777_d_n2, assign13600_e13777_d_n3, assign13600_e13777_d_n4, assign13600_e13777_d_n7, assign13600_e13777_d_n9, assign13600_e13777_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign13600_e13777;
        locals.var_fn169_calc_iq__ffs_dn2 = assign13600_e13777_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign13600_e13777_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign13600_e13777_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign13600_e13777_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign13600_e13777_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign13600_e13777_d_n10;

        let (assign13610_e13781, assign13610_e13781_d_n4, assign13610_e13781_d_n9, assign13610_e13781_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit, locals.var_fn169_calc_iq__two_n_phit_dn4, locals.var_fn169_calc_iq__two_n_phit_dn9, locals.var_fn169_calc_iq__two_n_phit_dn10,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit = assign13610_e13781;
        locals.var_fn169_calc_iq__two_n_phit_dn4 = assign13610_e13781_d_n4;
        locals.var_fn169_calc_iq__two_n_phit_dn9 = assign13610_e13781_d_n9;
        locals.var_fn169_calc_iq__two_n_phit_dn10 = assign13610_e13781_d_n10;

        let (assign13620_e13785, assign13620_e13785_d_n4, assign13620_e13785_d_n9, assign13620_e13785_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qref, locals.var_fn169_calc_iq__qref_dn4, locals.var_fn169_calc_iq__qref_dn9, locals.var_fn169_calc_iq__qref_dn10,)
    }
};
        locals.var_fn169_calc_iq__qref = assign13620_e13785;
        locals.var_fn169_calc_iq__qref_dn4 = assign13620_e13785_d_n4;
        locals.var_fn169_calc_iq__qref_dn9 = assign13620_e13785_d_n9;
        locals.var_fn169_calc_iq__qref_dn10 = assign13620_e13785_d_n10;

        let (assign13630_e13789, assign13630_e13789_d_n2, assign13630_e13789_d_n3, assign13630_e13789_d_n4, assign13630_e13789_d_n7, assign13630_e13789_d_n9, assign13630_e13789_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etas, locals.var_fn169_calc_iq__etas_dn2, locals.var_fn169_calc_iq__etas_dn3, locals.var_fn169_calc_iq__etas_dn4, locals.var_fn169_calc_iq__etas_dn7, locals.var_fn169_calc_iq__etas_dn9, locals.var_fn169_calc_iq__etas_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas = assign13630_e13789;
        locals.var_fn169_calc_iq__etas_dn2 = assign13630_e13789_d_n2;
        locals.var_fn169_calc_iq__etas_dn3 = assign13630_e13789_d_n3;
        locals.var_fn169_calc_iq__etas_dn4 = assign13630_e13789_d_n4;
        locals.var_fn169_calc_iq__etas_dn7 = assign13630_e13789_d_n7;
        locals.var_fn169_calc_iq__etas_dn9 = assign13630_e13789_d_n9;
        locals.var_fn169_calc_iq__etas_dn10 = assign13630_e13789_d_n10;

        let (assign13640_e13793, assign13640_e13793_d_n2, assign13640_e13793_d_n3, assign13640_e13793_d_n4, assign13640_e13793_d_n7, assign13640_e13793_d_n9, assign13640_e13793_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign13640_e13793;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign13640_e13793_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign13640_e13793_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign13640_e13793_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign13640_e13793_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign13640_e13793_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign13640_e13793_d_n10;

        let (assign13650_e13797, assign13650_e13797_d_n2, assign13650_e13797_d_n3, assign13650_e13797_d_n4, assign13650_e13797_d_n7, assign13650_e13797_d_n9, assign13650_e13797_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__muf, locals.var_fn169_calc_iq__muf_dn2, locals.var_fn169_calc_iq__muf_dn3, locals.var_fn169_calc_iq__muf_dn4, locals.var_fn169_calc_iq__muf_dn7, locals.var_fn169_calc_iq__muf_dn9, locals.var_fn169_calc_iq__muf_dn10,)
    }
};
        locals.var_fn169_calc_iq__muf = assign13650_e13797;
        locals.var_fn169_calc_iq__muf_dn2 = assign13650_e13797_d_n2;
        locals.var_fn169_calc_iq__muf_dn3 = assign13650_e13797_d_n3;
        locals.var_fn169_calc_iq__muf_dn4 = assign13650_e13797_d_n4;
        locals.var_fn169_calc_iq__muf_dn7 = assign13650_e13797_d_n7;
        locals.var_fn169_calc_iq__muf_dn9 = assign13650_e13797_d_n9;
        locals.var_fn169_calc_iq__muf_dn10 = assign13650_e13797_d_n10;

        let (assign13660_e13801, assign13660_e13801_d_n2, assign13660_e13801_d_n3, assign13660_e13801_d_n4, assign13660_e13801_d_n7, assign13660_e13801_d_n9, assign13660_e13801_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vx, locals.var_fn169_calc_iq__vx_dn2, locals.var_fn169_calc_iq__vx_dn3, locals.var_fn169_calc_iq__vx_dn4, locals.var_fn169_calc_iq__vx_dn7, locals.var_fn169_calc_iq__vx_dn9, locals.var_fn169_calc_iq__vx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vx = assign13660_e13801;
        locals.var_fn169_calc_iq__vx_dn2 = assign13660_e13801_d_n2;
        locals.var_fn169_calc_iq__vx_dn3 = assign13660_e13801_d_n3;
        locals.var_fn169_calc_iq__vx_dn4 = assign13660_e13801_d_n4;
        locals.var_fn169_calc_iq__vx_dn7 = assign13660_e13801_d_n7;
        locals.var_fn169_calc_iq__vx_dn9 = assign13660_e13801_d_n9;
        locals.var_fn169_calc_iq__vx_dn10 = assign13660_e13801_d_n10;

        let (assign13670_e13805, assign13670_e13805_d_n2, assign13670_e13805_d_n3, assign13670_e13805_d_n4, assign13670_e13805_d_n7, assign13670_e13805_d_n9, assign13670_e13805_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vxf, locals.var_fn169_calc_iq__vxf_dn2, locals.var_fn169_calc_iq__vxf_dn3, locals.var_fn169_calc_iq__vxf_dn4, locals.var_fn169_calc_iq__vxf_dn7, locals.var_fn169_calc_iq__vxf_dn9, locals.var_fn169_calc_iq__vxf_dn10,)
    }
};
        locals.var_fn169_calc_iq__vxf = assign13670_e13805;
        locals.var_fn169_calc_iq__vxf_dn2 = assign13670_e13805_d_n2;
        locals.var_fn169_calc_iq__vxf_dn3 = assign13670_e13805_d_n3;
        locals.var_fn169_calc_iq__vxf_dn4 = assign13670_e13805_d_n4;
        locals.var_fn169_calc_iq__vxf_dn7 = assign13670_e13805_d_n7;
        locals.var_fn169_calc_iq__vxf_dn9 = assign13670_e13805_d_n9;
        locals.var_fn169_calc_iq__vxf_dn10 = assign13670_e13805_d_n10;

        let (assign13680_e13809, assign13680_e13809_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__n0, locals.var_fn169_calc_iq__n0_dn4,)
    }
};
        locals.var_fn169_calc_iq__n0 = assign13680_e13809;
        locals.var_fn169_calc_iq__n0_dn4 = assign13680_e13809_d_n4;

        let (assign13690_e13813, assign13690_e13813_d_n2, assign13690_e13813_d_n4, assign13690_e13813_d_n7, assign13690_e13813_d_n9, assign13690_e13813_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign13690_e13813;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign13690_e13813_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign13690_e13813_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign13690_e13813_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign13690_e13813_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign13690_e13813_d_n10;

        let (assign13700_e13817, assign13700_e13817_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit0, locals.var_fn169_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit0 = assign13700_e13817;
        locals.var_fn169_calc_iq__two_n_phit0_dn4 = assign13700_e13817_d_n4;

        let (assign13710_e13821, assign13710_e13821_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qref0, locals.var_fn169_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn169_calc_iq__qref0 = assign13710_e13821;
        locals.var_fn169_calc_iq__qref0_dn4 = assign13710_e13821_d_n4;

        let (assign13720_e13825, assign13720_e13825_d_n2, assign13720_e13825_d_n4, assign13720_e13825_d_n7, assign13720_e13825_d_n9, assign13720_e13825_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etas0, locals.var_fn169_calc_iq__etas0_dn2, locals.var_fn169_calc_iq__etas0_dn4, locals.var_fn169_calc_iq__etas0_dn7, locals.var_fn169_calc_iq__etas0_dn9, locals.var_fn169_calc_iq__etas0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas0 = assign13720_e13825;
        locals.var_fn169_calc_iq__etas0_dn2 = assign13720_e13825_d_n2;
        locals.var_fn169_calc_iq__etas0_dn4 = assign13720_e13825_d_n4;
        locals.var_fn169_calc_iq__etas0_dn7 = assign13720_e13825_d_n7;
        locals.var_fn169_calc_iq__etas0_dn9 = assign13720_e13825_d_n9;
        locals.var_fn169_calc_iq__etas0_dn10 = assign13720_e13825_d_n10;

        let (assign13730_e13829, assign13730_e13829_d_n2, assign13730_e13829_d_n4, assign13730_e13829_d_n7, assign13730_e13829_d_n9, assign13730_e13829_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign13730_e13829;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign13730_e13829_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign13730_e13829_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign13730_e13829_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign13730_e13829_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign13730_e13829_d_n10;

        let (assign13740_e13833, assign13740_e13833_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__muf0, locals.var_fn169_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn169_calc_iq__muf0 = assign13740_e13833;
        locals.var_fn169_calc_iq__muf0_dn4 = assign13740_e13833_d_n4;

        let (assign13750_e13837, assign13750_e13837_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vx0, locals.var_fn169_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vx0 = assign13750_e13837;
        locals.var_fn169_calc_iq__vx0_dn4 = assign13750_e13837_d_n4;

        let (assign13760_e13841, assign13760_e13841_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__tfacmobin, locals.var_fn169_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tfacmobin = assign13760_e13841;
        locals.var_fn169_calc_iq__tfacmobin_dn4 = assign13760_e13841_d_n4;

        let (assign13770_e13845, assign13770_e13845_d_n2, assign13770_e13845_d_n3, assign13770_e13845_d_n4, assign13770_e13845_d_n7, assign13770_e13845_d_n9, assign13770_e13845_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign13770_e13845;
        locals.var_fn169_calc_iq__ff_dn2 = assign13770_e13845_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign13770_e13845_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign13770_e13845_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign13770_e13845_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign13770_e13845_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign13770_e13845_d_n10;

        let (assign13780_e13849, assign13780_e13849_d_n2, assign13780_e13849_d_n3, assign13780_e13849_d_n4, assign13780_e13849_d_n7, assign13780_e13849_d_n9, assign13780_e13849_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__eta, locals.var_fn169_calc_iq__eta_dn2, locals.var_fn169_calc_iq__eta_dn3, locals.var_fn169_calc_iq__eta_dn4, locals.var_fn169_calc_iq__eta_dn7, locals.var_fn169_calc_iq__eta_dn9, locals.var_fn169_calc_iq__eta_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta = assign13780_e13849;
        locals.var_fn169_calc_iq__eta_dn2 = assign13780_e13849_d_n2;
        locals.var_fn169_calc_iq__eta_dn3 = assign13780_e13849_d_n3;
        locals.var_fn169_calc_iq__eta_dn4 = assign13780_e13849_d_n4;
        locals.var_fn169_calc_iq__eta_dn7 = assign13780_e13849_d_n7;
        locals.var_fn169_calc_iq__eta_dn9 = assign13780_e13849_d_n9;
        locals.var_fn169_calc_iq__eta_dn10 = assign13780_e13849_d_n10;

        let (assign13790_e13853, assign13790_e13853_d_n2, assign13790_e13853_d_n3, assign13790_e13853_d_n4, assign13790_e13853_d_n7, assign13790_e13853_d_n9, assign13790_e13853_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign13790_e13853;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign13790_e13853_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign13790_e13853_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign13790_e13853_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign13790_e13853_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign13790_e13853_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign13790_e13853_d_n10;

        let (assign13800_e13857, assign13800_e13857_d_n2, assign13800_e13857_d_n4, assign13800_e13857_d_n7, assign13800_e13857_d_n9, assign13800_e13857_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign13800_e13857;
        locals.var_fn169_calc_iq__ff0_dn2 = assign13800_e13857_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign13800_e13857_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign13800_e13857_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign13800_e13857_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign13800_e13857_d_n10;

        let (assign13810_e13861, assign13810_e13861_d_n2, assign13810_e13861_d_n4, assign13810_e13861_d_n7, assign13810_e13861_d_n9, assign13810_e13861_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__eta0, locals.var_fn169_calc_iq__eta0_dn2, locals.var_fn169_calc_iq__eta0_dn4, locals.var_fn169_calc_iq__eta0_dn7, locals.var_fn169_calc_iq__eta0_dn9, locals.var_fn169_calc_iq__eta0_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta0 = assign13810_e13861;
        locals.var_fn169_calc_iq__eta0_dn2 = assign13810_e13861_d_n2;
        locals.var_fn169_calc_iq__eta0_dn4 = assign13810_e13861_d_n4;
        locals.var_fn169_calc_iq__eta0_dn7 = assign13810_e13861_d_n7;
        locals.var_fn169_calc_iq__eta0_dn9 = assign13810_e13861_d_n9;
        locals.var_fn169_calc_iq__eta0_dn10 = assign13810_e13861_d_n10;

        let (assign13820_e13865, assign13820_e13865_d_n2, assign13820_e13865_d_n4, assign13820_e13865_d_n7, assign13820_e13865_d_n9, assign13820_e13865_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign13820_e13865;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign13820_e13865_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign13820_e13865_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign13820_e13865_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign13820_e13865_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign13820_e13865_d_n10;

        let (assign13830_e13869, assign13830_e13869_d_n2, assign13830_e13869_d_n3, assign13830_e13869_d_n4, assign13830_e13869_d_n7, assign13830_e13869_d_n9, assign13830_e13869_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats, locals.var_fn169_calc_iq__vdsats_dn2, locals.var_fn169_calc_iq__vdsats_dn3, locals.var_fn169_calc_iq__vdsats_dn4, locals.var_fn169_calc_iq__vdsats_dn7, locals.var_fn169_calc_iq__vdsats_dn9, locals.var_fn169_calc_iq__vdsats_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats = assign13830_e13869;
        locals.var_fn169_calc_iq__vdsats_dn2 = assign13830_e13869_d_n2;
        locals.var_fn169_calc_iq__vdsats_dn3 = assign13830_e13869_d_n3;
        locals.var_fn169_calc_iq__vdsats_dn4 = assign13830_e13869_d_n4;
        locals.var_fn169_calc_iq__vdsats_dn7 = assign13830_e13869_d_n7;
        locals.var_fn169_calc_iq__vdsats_dn9 = assign13830_e13869_d_n9;
        locals.var_fn169_calc_iq__vdsats_dn10 = assign13830_e13869_d_n10;

        let (assign13840_e13873, assign13840_e13873_d_n2, assign13840_e13873_d_n3, assign13840_e13873_d_n4, assign13840_e13873_d_n7, assign13840_e13873_d_n9, assign13840_e13873_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats1, locals.var_fn169_calc_iq__vdsats1_dn2, locals.var_fn169_calc_iq__vdsats1_dn3, locals.var_fn169_calc_iq__vdsats1_dn4, locals.var_fn169_calc_iq__vdsats1_dn7, locals.var_fn169_calc_iq__vdsats1_dn9, locals.var_fn169_calc_iq__vdsats1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats1 = assign13840_e13873;
        locals.var_fn169_calc_iq__vdsats1_dn2 = assign13840_e13873_d_n2;
        locals.var_fn169_calc_iq__vdsats1_dn3 = assign13840_e13873_d_n3;
        locals.var_fn169_calc_iq__vdsats1_dn4 = assign13840_e13873_d_n4;
        locals.var_fn169_calc_iq__vdsats1_dn7 = assign13840_e13873_d_n7;
        locals.var_fn169_calc_iq__vdsats1_dn9 = assign13840_e13873_d_n9;
        locals.var_fn169_calc_iq__vdsats1_dn10 = assign13840_e13873_d_n10;

        let (assign13850_e13877, assign13850_e13877_d_n2, assign13850_e13877_d_n3, assign13850_e13877_d_n4, assign13850_e13877_d_n7, assign13850_e13877_d_n9, assign13850_e13877_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat, locals.var_fn169_calc_iq__vdsat_dn2, locals.var_fn169_calc_iq__vdsat_dn3, locals.var_fn169_calc_iq__vdsat_dn4, locals.var_fn169_calc_iq__vdsat_dn7, locals.var_fn169_calc_iq__vdsat_dn9, locals.var_fn169_calc_iq__vdsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat = assign13850_e13877;
        locals.var_fn169_calc_iq__vdsat_dn2 = assign13850_e13877_d_n2;
        locals.var_fn169_calc_iq__vdsat_dn3 = assign13850_e13877_d_n3;
        locals.var_fn169_calc_iq__vdsat_dn4 = assign13850_e13877_d_n4;
        locals.var_fn169_calc_iq__vdsat_dn7 = assign13850_e13877_d_n7;
        locals.var_fn169_calc_iq__vdsat_dn9 = assign13850_e13877_d_n9;
        locals.var_fn169_calc_iq__vdsat_dn10 = assign13850_e13877_d_n10;

        let (assign13860_e13881, assign13860_e13881_d_n2, assign13860_e13881_d_n3, assign13860_e13881_d_n4, assign13860_e13881_d_n7, assign13860_e13881_d_n9, assign13860_e13881_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fsd, locals.var_fn169_calc_iq__fsd_dn2, locals.var_fn169_calc_iq__fsd_dn3, locals.var_fn169_calc_iq__fsd_dn4, locals.var_fn169_calc_iq__fsd_dn7, locals.var_fn169_calc_iq__fsd_dn9, locals.var_fn169_calc_iq__fsd_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd = assign13860_e13881;
        locals.var_fn169_calc_iq__fsd_dn2 = assign13860_e13881_d_n2;
        locals.var_fn169_calc_iq__fsd_dn3 = assign13860_e13881_d_n3;
        locals.var_fn169_calc_iq__fsd_dn4 = assign13860_e13881_d_n4;
        locals.var_fn169_calc_iq__fsd_dn7 = assign13860_e13881_d_n7;
        locals.var_fn169_calc_iq__fsd_dn9 = assign13860_e13881_d_n9;
        locals.var_fn169_calc_iq__fsd_dn10 = assign13860_e13881_d_n10;

        let (assign13870_e13885, assign13870_e13885_d_n2, assign13870_e13885_d_n3, assign13870_e13885_d_n4, assign13870_e13885_d_n7, assign13870_e13885_d_n9, assign13870_e13885_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdx, locals.var_fn169_calc_iq__vdx_dn2, locals.var_fn169_calc_iq__vdx_dn3, locals.var_fn169_calc_iq__vdx_dn4, locals.var_fn169_calc_iq__vdx_dn7, locals.var_fn169_calc_iq__vdx_dn9, locals.var_fn169_calc_iq__vdx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx = assign13870_e13885;
        locals.var_fn169_calc_iq__vdx_dn2 = assign13870_e13885_d_n2;
        locals.var_fn169_calc_iq__vdx_dn3 = assign13870_e13885_d_n3;
        locals.var_fn169_calc_iq__vdx_dn4 = assign13870_e13885_d_n4;
        locals.var_fn169_calc_iq__vdx_dn7 = assign13870_e13885_d_n7;
        locals.var_fn169_calc_iq__vdx_dn9 = assign13870_e13885_d_n9;
        locals.var_fn169_calc_iq__vdx_dn10 = assign13870_e13885_d_n10;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let (assign13880_e13889, assign13880_e13889_d_n2, assign13880_e13889_d_n3, assign13880_e13889_d_n4, assign13880_e13889_d_n7, assign13880_e13889_d_n9, assign13880_e13889_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fds, locals.var_fn169_calc_iq__fds_dn2, locals.var_fn169_calc_iq__fds_dn3, locals.var_fn169_calc_iq__fds_dn4, locals.var_fn169_calc_iq__fds_dn7, locals.var_fn169_calc_iq__fds_dn9, locals.var_fn169_calc_iq__fds_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds = assign13880_e13889;
        locals.var_fn169_calc_iq__fds_dn2 = assign13880_e13889_d_n2;
        locals.var_fn169_calc_iq__fds_dn3 = assign13880_e13889_d_n3;
        locals.var_fn169_calc_iq__fds_dn4 = assign13880_e13889_d_n4;
        locals.var_fn169_calc_iq__fds_dn7 = assign13880_e13889_d_n7;
        locals.var_fn169_calc_iq__fds_dn9 = assign13880_e13889_d_n9;
        locals.var_fn169_calc_iq__fds_dn10 = assign13880_e13889_d_n10;

        let (assign13890_e13893, assign13890_e13893_d_n2, assign13890_e13893_d_n3, assign13890_e13893_d_n4, assign13890_e13893_d_n7, assign13890_e13893_d_n9, assign13890_e13893_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsx, locals.var_fn169_calc_iq__vsx_dn2, locals.var_fn169_calc_iq__vsx_dn3, locals.var_fn169_calc_iq__vsx_dn4, locals.var_fn169_calc_iq__vsx_dn7, locals.var_fn169_calc_iq__vsx_dn9, locals.var_fn169_calc_iq__vsx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx = assign13890_e13893;
        locals.var_fn169_calc_iq__vsx_dn2 = assign13890_e13893_d_n2;
        locals.var_fn169_calc_iq__vsx_dn3 = assign13890_e13893_d_n3;
        locals.var_fn169_calc_iq__vsx_dn4 = assign13890_e13893_d_n4;
        locals.var_fn169_calc_iq__vsx_dn7 = assign13890_e13893_d_n7;
        locals.var_fn169_calc_iq__vsx_dn9 = assign13890_e13893_d_n9;
        locals.var_fn169_calc_iq__vsx_dn10 = assign13890_e13893_d_n10;

        let (assign13900_e13897, assign13900_e13897_d_n2, assign13900_e13897_d_n3, assign13900_e13897_d_n4, assign13900_e13897_d_n7, assign13900_e13897_d_n9, assign13900_e13897_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign13900_e13897;
        locals.var_fn169_calc_iq__ffd_dn2 = assign13900_e13897_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign13900_e13897_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign13900_e13897_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign13900_e13897_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign13900_e13897_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign13900_e13897_d_n10;

        let (assign13910_e13901, assign13910_e13901_d_n2, assign13910_e13901_d_n3, assign13910_e13901_d_n4, assign13910_e13901_d_n7, assign13910_e13901_d_n9, assign13910_e13901_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etad, locals.var_fn169_calc_iq__etad_dn2, locals.var_fn169_calc_iq__etad_dn3, locals.var_fn169_calc_iq__etad_dn4, locals.var_fn169_calc_iq__etad_dn7, locals.var_fn169_calc_iq__etad_dn9, locals.var_fn169_calc_iq__etad_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad = assign13910_e13901;
        locals.var_fn169_calc_iq__etad_dn2 = assign13910_e13901_d_n2;
        locals.var_fn169_calc_iq__etad_dn3 = assign13910_e13901_d_n3;
        locals.var_fn169_calc_iq__etad_dn4 = assign13910_e13901_d_n4;
        locals.var_fn169_calc_iq__etad_dn7 = assign13910_e13901_d_n7;
        locals.var_fn169_calc_iq__etad_dn9 = assign13910_e13901_d_n9;
        locals.var_fn169_calc_iq__etad_dn10 = assign13910_e13901_d_n10;

        let (assign13920_e13905, assign13920_e13905_d_n2, assign13920_e13905_d_n3, assign13920_e13905_d_n4, assign13920_e13905_d_n7, assign13920_e13905_d_n9, assign13920_e13905_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign13920_e13905;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign13920_e13905_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign13920_e13905_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign13920_e13905_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign13920_e13905_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign13920_e13905_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign13920_e13905_d_n10;

        let (assign13930_e13909, assign13930_e13909_d_n2, assign13930_e13909_d_n3, assign13930_e13909_d_n4, assign13930_e13909_d_n7, assign13930_e13909_d_n9, assign13930_e13909_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsc, locals.var_fn169_calc_iq__vdsc_dn2, locals.var_fn169_calc_iq__vdsc_dn3, locals.var_fn169_calc_iq__vdsc_dn4, locals.var_fn169_calc_iq__vdsc_dn7, locals.var_fn169_calc_iq__vdsc_dn9, locals.var_fn169_calc_iq__vdsc_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsc = assign13930_e13909;
        locals.var_fn169_calc_iq__vdsc_dn2 = assign13930_e13909_d_n2;
        locals.var_fn169_calc_iq__vdsc_dn3 = assign13930_e13909_d_n3;
        locals.var_fn169_calc_iq__vdsc_dn4 = assign13930_e13909_d_n4;
        locals.var_fn169_calc_iq__vdsc_dn7 = assign13930_e13909_d_n7;
        locals.var_fn169_calc_iq__vdsc_dn9 = assign13930_e13909_d_n9;
        locals.var_fn169_calc_iq__vdsc_dn10 = assign13930_e13909_d_n10;

        let (assign13940_e13913, assign13940_e13913_d_n2, assign13940_e13913_d_n3, assign13940_e13913_d_n4, assign13940_e13913_d_n7, assign13940_e13913_d_n9, assign13940_e13913_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fsat, locals.var_fn169_calc_iq__fsat_dn2, locals.var_fn169_calc_iq__fsat_dn3, locals.var_fn169_calc_iq__fsat_dn4, locals.var_fn169_calc_iq__fsat_dn7, locals.var_fn169_calc_iq__fsat_dn9, locals.var_fn169_calc_iq__fsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsat = assign13940_e13913;
        locals.var_fn169_calc_iq__fsat_dn2 = assign13940_e13913_d_n2;
        locals.var_fn169_calc_iq__fsat_dn3 = assign13940_e13913_d_n3;
        locals.var_fn169_calc_iq__fsat_dn4 = assign13940_e13913_d_n4;
        locals.var_fn169_calc_iq__fsat_dn7 = assign13940_e13913_d_n7;
        locals.var_fn169_calc_iq__fsat_dn9 = assign13940_e13913_d_n9;
        locals.var_fn169_calc_iq__fsat_dn10 = assign13940_e13913_d_n10;

        let (assign13950_e13917, assign13950_e13917_d_n2, assign13950_e13917_d_n3, assign13950_e13917_d_n4, assign13950_e13917_d_n7, assign13950_e13917_d_n9, assign13950_e13917_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vel, locals.var_fn169_calc_iq__vel_dn2, locals.var_fn169_calc_iq__vel_dn3, locals.var_fn169_calc_iq__vel_dn4, locals.var_fn169_calc_iq__vel_dn7, locals.var_fn169_calc_iq__vel_dn9, locals.var_fn169_calc_iq__vel_dn10,)
    }
};
        locals.var_fn169_calc_iq__vel = assign13950_e13917;
        locals.var_fn169_calc_iq__vel_dn2 = assign13950_e13917_d_n2;
        locals.var_fn169_calc_iq__vel_dn3 = assign13950_e13917_d_n3;
        locals.var_fn169_calc_iq__vel_dn4 = assign13950_e13917_d_n4;
        locals.var_fn169_calc_iq__vel_dn7 = assign13950_e13917_d_n7;
        locals.var_fn169_calc_iq__vel_dn9 = assign13950_e13917_d_n9;
        locals.var_fn169_calc_iq__vel_dn10 = assign13950_e13917_d_n10;

        let (assign13960_e13921, assign13960_e13921_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats0, locals.var_fn169_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vdsats0 = assign13960_e13921;
        locals.var_fn169_calc_iq__vdsats0_dn4 = assign13960_e13921_d_n4;

        let (assign13970_e13925, assign13970_e13925_d_n2, assign13970_e13925_d_n4, assign13970_e13925_d_n7, assign13970_e13925_d_n9, assign13970_e13925_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats10, locals.var_fn169_calc_iq__vdsats10_dn2, locals.var_fn169_calc_iq__vdsats10_dn4, locals.var_fn169_calc_iq__vdsats10_dn7, locals.var_fn169_calc_iq__vdsats10_dn9, locals.var_fn169_calc_iq__vdsats10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats10 = assign13970_e13925;
        locals.var_fn169_calc_iq__vdsats10_dn2 = assign13970_e13925_d_n2;
        locals.var_fn169_calc_iq__vdsats10_dn4 = assign13970_e13925_d_n4;
        locals.var_fn169_calc_iq__vdsats10_dn7 = assign13970_e13925_d_n7;
        locals.var_fn169_calc_iq__vdsats10_dn9 = assign13970_e13925_d_n9;
        locals.var_fn169_calc_iq__vdsats10_dn10 = assign13970_e13925_d_n10;

        let (assign13980_e13929, assign13980_e13929_d_n2, assign13980_e13929_d_n4, assign13980_e13929_d_n7, assign13980_e13929_d_n9, assign13980_e13929_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat10, locals.var_fn169_calc_iq__vdsat10_dn2, locals.var_fn169_calc_iq__vdsat10_dn4, locals.var_fn169_calc_iq__vdsat10_dn7, locals.var_fn169_calc_iq__vdsat10_dn9, locals.var_fn169_calc_iq__vdsat10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat10 = assign13980_e13929;
        locals.var_fn169_calc_iq__vdsat10_dn2 = assign13980_e13929_d_n2;
        locals.var_fn169_calc_iq__vdsat10_dn4 = assign13980_e13929_d_n4;
        locals.var_fn169_calc_iq__vdsat10_dn7 = assign13980_e13929_d_n7;
        locals.var_fn169_calc_iq__vdsat10_dn9 = assign13980_e13929_d_n9;
        locals.var_fn169_calc_iq__vdsat10_dn10 = assign13980_e13929_d_n10;

        let (assign13990_e13933, assign13990_e13933_d_n2, assign13990_e13933_d_n4, assign13990_e13933_d_n7, assign13990_e13933_d_n9, assign13990_e13933_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fsd0, locals.var_fn169_calc_iq__fsd0_dn2, locals.var_fn169_calc_iq__fsd0_dn4, locals.var_fn169_calc_iq__fsd0_dn7, locals.var_fn169_calc_iq__fsd0_dn9, locals.var_fn169_calc_iq__fsd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd0 = assign13990_e13933;
        locals.var_fn169_calc_iq__fsd0_dn2 = assign13990_e13933_d_n2;
        locals.var_fn169_calc_iq__fsd0_dn4 = assign13990_e13933_d_n4;
        locals.var_fn169_calc_iq__fsd0_dn7 = assign13990_e13933_d_n7;
        locals.var_fn169_calc_iq__fsd0_dn9 = assign13990_e13933_d_n9;
        locals.var_fn169_calc_iq__fsd0_dn10 = assign13990_e13933_d_n10;

        let (assign14000_e13937, assign14000_e13937_d_n2, assign14000_e13937_d_n4, assign14000_e13937_d_n7, assign14000_e13937_d_n9, assign14000_e13937_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdx0, locals.var_fn169_calc_iq__vdx0_dn2, locals.var_fn169_calc_iq__vdx0_dn4, locals.var_fn169_calc_iq__vdx0_dn7, locals.var_fn169_calc_iq__vdx0_dn9, locals.var_fn169_calc_iq__vdx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx0 = assign14000_e13937;
        locals.var_fn169_calc_iq__vdx0_dn2 = assign14000_e13937_d_n2;
        locals.var_fn169_calc_iq__vdx0_dn4 = assign14000_e13937_d_n4;
        locals.var_fn169_calc_iq__vdx0_dn7 = assign14000_e13937_d_n7;
        locals.var_fn169_calc_iq__vdx0_dn9 = assign14000_e13937_d_n9;
        locals.var_fn169_calc_iq__vdx0_dn10 = assign14000_e13937_d_n10;

        let (assign14010_e13941, assign14010_e13941_d_n2, assign14010_e13941_d_n4, assign14010_e13941_d_n7, assign14010_e13941_d_n9, assign14010_e13941_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fds0, locals.var_fn169_calc_iq__fds0_dn2, locals.var_fn169_calc_iq__fds0_dn4, locals.var_fn169_calc_iq__fds0_dn7, locals.var_fn169_calc_iq__fds0_dn9, locals.var_fn169_calc_iq__fds0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds0 = assign14010_e13941;
        locals.var_fn169_calc_iq__fds0_dn2 = assign14010_e13941_d_n2;
        locals.var_fn169_calc_iq__fds0_dn4 = assign14010_e13941_d_n4;
        locals.var_fn169_calc_iq__fds0_dn7 = assign14010_e13941_d_n7;
        locals.var_fn169_calc_iq__fds0_dn9 = assign14010_e13941_d_n9;
        locals.var_fn169_calc_iq__fds0_dn10 = assign14010_e13941_d_n10;

        let (assign14020_e13945, assign14020_e13945_d_n2, assign14020_e13945_d_n4, assign14020_e13945_d_n7, assign14020_e13945_d_n9, assign14020_e13945_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsx0, locals.var_fn169_calc_iq__vsx0_dn2, locals.var_fn169_calc_iq__vsx0_dn4, locals.var_fn169_calc_iq__vsx0_dn7, locals.var_fn169_calc_iq__vsx0_dn9, locals.var_fn169_calc_iq__vsx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx0 = assign14020_e13945;
        locals.var_fn169_calc_iq__vsx0_dn2 = assign14020_e13945_d_n2;
        locals.var_fn169_calc_iq__vsx0_dn4 = assign14020_e13945_d_n4;
        locals.var_fn169_calc_iq__vsx0_dn7 = assign14020_e13945_d_n7;
        locals.var_fn169_calc_iq__vsx0_dn9 = assign14020_e13945_d_n9;
        locals.var_fn169_calc_iq__vsx0_dn10 = assign14020_e13945_d_n10;

        let (assign14030_e13949, assign14030_e13949_d_n2, assign14030_e13949_d_n4, assign14030_e13949_d_n7, assign14030_e13949_d_n9, assign14030_e13949_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign14030_e13949;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign14030_e13949_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign14030_e13949_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign14030_e13949_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign14030_e13949_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign14030_e13949_d_n10;

        let (assign14040_e13953, assign14040_e13953_d_n2, assign14040_e13953_d_n4, assign14040_e13953_d_n7, assign14040_e13953_d_n9, assign14040_e13953_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etad0, locals.var_fn169_calc_iq__etad0_dn2, locals.var_fn169_calc_iq__etad0_dn4, locals.var_fn169_calc_iq__etad0_dn7, locals.var_fn169_calc_iq__etad0_dn9, locals.var_fn169_calc_iq__etad0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad0 = assign14040_e13953;
        locals.var_fn169_calc_iq__etad0_dn2 = assign14040_e13953_d_n2;
        locals.var_fn169_calc_iq__etad0_dn4 = assign14040_e13953_d_n4;
        locals.var_fn169_calc_iq__etad0_dn7 = assign14040_e13953_d_n7;
        locals.var_fn169_calc_iq__etad0_dn9 = assign14040_e13953_d_n9;
        locals.var_fn169_calc_iq__etad0_dn10 = assign14040_e13953_d_n10;

        let (assign14050_e13957, assign14050_e13957_d_n2, assign14050_e13957_d_n4, assign14050_e13957_d_n7, assign14050_e13957_d_n9, assign14050_e13957_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign14050_e13957;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign14050_e13957_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign14050_e13957_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign14050_e13957_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign14050_e13957_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign14050_e13957_d_n10;

        let (assign14060_e13961, assign14060_e13961_d_n2, assign14060_e13961_d_n4, assign14060_e13961_d_n7, assign14060_e13961_d_n9, assign14060_e13961_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs2, locals.var_fn169_calc_iq__qs2_dn2, locals.var_fn169_calc_iq__qs2_dn4, locals.var_fn169_calc_iq__qs2_dn7, locals.var_fn169_calc_iq__qs2_dn9, locals.var_fn169_calc_iq__qs2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs2 = assign14060_e13961;
        locals.var_fn169_calc_iq__qs2_dn2 = assign14060_e13961_d_n2;
        locals.var_fn169_calc_iq__qs2_dn4 = assign14060_e13961_d_n4;
        locals.var_fn169_calc_iq__qs2_dn7 = assign14060_e13961_d_n7;
        locals.var_fn169_calc_iq__qs2_dn9 = assign14060_e13961_d_n9;
        locals.var_fn169_calc_iq__qs2_dn10 = assign14060_e13961_d_n10;

        let (assign14070_e13965, assign14070_e13965_d_n2, assign14070_e13965_d_n4, assign14070_e13965_d_n7, assign14070_e13965_d_n9, assign14070_e13965_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs3, locals.var_fn169_calc_iq__qs3_dn2, locals.var_fn169_calc_iq__qs3_dn4, locals.var_fn169_calc_iq__qs3_dn7, locals.var_fn169_calc_iq__qs3_dn9, locals.var_fn169_calc_iq__qs3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs3 = assign14070_e13965;
        locals.var_fn169_calc_iq__qs3_dn2 = assign14070_e13965_d_n2;
        locals.var_fn169_calc_iq__qs3_dn4 = assign14070_e13965_d_n4;
        locals.var_fn169_calc_iq__qs3_dn7 = assign14070_e13965_d_n7;
        locals.var_fn169_calc_iq__qs3_dn9 = assign14070_e13965_d_n9;
        locals.var_fn169_calc_iq__qs3_dn10 = assign14070_e13965_d_n10;

        let (assign14080_e13969, assign14080_e13969_d_n2, assign14080_e13969_d_n4, assign14080_e13969_d_n7, assign14080_e13969_d_n9, assign14080_e13969_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd2, locals.var_fn169_calc_iq__qd2_dn2, locals.var_fn169_calc_iq__qd2_dn4, locals.var_fn169_calc_iq__qd2_dn7, locals.var_fn169_calc_iq__qd2_dn9, locals.var_fn169_calc_iq__qd2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd2 = assign14080_e13969;
        locals.var_fn169_calc_iq__qd2_dn2 = assign14080_e13969_d_n2;
        locals.var_fn169_calc_iq__qd2_dn4 = assign14080_e13969_d_n4;
        locals.var_fn169_calc_iq__qd2_dn7 = assign14080_e13969_d_n7;
        locals.var_fn169_calc_iq__qd2_dn9 = assign14080_e13969_d_n9;
        locals.var_fn169_calc_iq__qd2_dn10 = assign14080_e13969_d_n10;

        let (assign14090_e13973, assign14090_e13973_d_n2, assign14090_e13973_d_n4, assign14090_e13973_d_n7, assign14090_e13973_d_n9, assign14090_e13973_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd3, locals.var_fn169_calc_iq__qd3_dn2, locals.var_fn169_calc_iq__qd3_dn4, locals.var_fn169_calc_iq__qd3_dn7, locals.var_fn169_calc_iq__qd3_dn9, locals.var_fn169_calc_iq__qd3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd3 = assign14090_e13973;
        locals.var_fn169_calc_iq__qd3_dn2 = assign14090_e13973_d_n2;
        locals.var_fn169_calc_iq__qd3_dn4 = assign14090_e13973_d_n4;
        locals.var_fn169_calc_iq__qd3_dn7 = assign14090_e13973_d_n7;
        locals.var_fn169_calc_iq__qd3_dn9 = assign14090_e13973_d_n9;
        locals.var_fn169_calc_iq__qd3_dn10 = assign14090_e13973_d_n10;

        let (assign14100_e13977, assign14100_e13977_d_n2, assign14100_e13977_d_n4, assign14100_e13977_d_n7, assign14100_e13977_d_n9, assign14100_e13977_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsqd, locals.var_fn169_calc_iq__qsqd_dn2, locals.var_fn169_calc_iq__qsqd_dn4, locals.var_fn169_calc_iq__qsqd_dn7, locals.var_fn169_calc_iq__qsqd_dn9, locals.var_fn169_calc_iq__qsqd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsqd = assign14100_e13977;
        locals.var_fn169_calc_iq__qsqd_dn2 = assign14100_e13977_d_n2;
        locals.var_fn169_calc_iq__qsqd_dn4 = assign14100_e13977_d_n4;
        locals.var_fn169_calc_iq__qsqd_dn7 = assign14100_e13977_d_n7;
        locals.var_fn169_calc_iq__qsqd_dn9 = assign14100_e13977_d_n9;
        locals.var_fn169_calc_iq__qsqd_dn10 = assign14100_e13977_d_n10;

        let (assign14110_e13981, assign14110_e13981_d_n2, assign14110_e13981_d_n4, assign14110_e13981_d_n7, assign14110_e13981_d_n9, assign14110_e13981_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvdd, locals.var_fn169_calc_iq__qinvdd_dn2, locals.var_fn169_calc_iq__qinvdd_dn4, locals.var_fn169_calc_iq__qinvdd_dn7, locals.var_fn169_calc_iq__qinvdd_dn9, locals.var_fn169_calc_iq__qinvdd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvdd = assign14110_e13981;
        locals.var_fn169_calc_iq__qinvdd_dn2 = assign14110_e13981_d_n2;
        locals.var_fn169_calc_iq__qinvdd_dn4 = assign14110_e13981_d_n4;
        locals.var_fn169_calc_iq__qinvdd_dn7 = assign14110_e13981_d_n7;
        locals.var_fn169_calc_iq__qinvdd_dn9 = assign14110_e13981_d_n9;
        locals.var_fn169_calc_iq__qinvdd_dn10 = assign14110_e13981_d_n10;

        let (assign14120_e13985, assign14120_e13985_d_n2, assign14120_e13985_d_n4, assign14120_e13985_d_n7, assign14120_e13985_d_n9, assign14120_e13985_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd1 = assign14120_e13985;
        locals.var_fn169_calc_iq__qd1_dn2 = assign14120_e13985_d_n2;
        locals.var_fn169_calc_iq__qd1_dn4 = assign14120_e13985_d_n4;
        locals.var_fn169_calc_iq__qd1_dn7 = assign14120_e13985_d_n7;
        locals.var_fn169_calc_iq__qd1_dn9 = assign14120_e13985_d_n9;
        locals.var_fn169_calc_iq__qd1_dn10 = assign14120_e13985_d_n10;

        let (assign14130_e13989, assign14130_e13989_d_n2, assign14130_e13989_d_n4, assign14130_e13989_d_n7, assign14130_e13989_d_n9, assign14130_e13989_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs, locals.var_fn169_calc_iq__qs_dn2, locals.var_fn169_calc_iq__qs_dn4, locals.var_fn169_calc_iq__qs_dn7, locals.var_fn169_calc_iq__qs_dn9, locals.var_fn169_calc_iq__qs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs = assign14130_e13989;
        locals.var_fn169_calc_iq__qs_dn2 = assign14130_e13989_d_n2;
        locals.var_fn169_calc_iq__qs_dn4 = assign14130_e13989_d_n4;
        locals.var_fn169_calc_iq__qs_dn7 = assign14130_e13989_d_n7;
        locals.var_fn169_calc_iq__qs_dn9 = assign14130_e13989_d_n9;
        locals.var_fn169_calc_iq__qs_dn10 = assign14130_e13989_d_n10;

        let (assign14140_e13993, assign14140_e13993_d_n2, assign14140_e13993_d_n4, assign14140_e13993_d_n7, assign14140_e13993_d_n9, assign14140_e13993_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd, locals.var_fn169_calc_iq__qd_dn2, locals.var_fn169_calc_iq__qd_dn4, locals.var_fn169_calc_iq__qd_dn7, locals.var_fn169_calc_iq__qd_dn9, locals.var_fn169_calc_iq__qd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd = assign14140_e13993;
        locals.var_fn169_calc_iq__qd_dn2 = assign14140_e13993_d_n2;
        locals.var_fn169_calc_iq__qd_dn4 = assign14140_e13993_d_n4;
        locals.var_fn169_calc_iq__qd_dn7 = assign14140_e13993_d_n7;
        locals.var_fn169_calc_iq__qd_dn9 = assign14140_e13993_d_n9;
        locals.var_fn169_calc_iq__qd_dn10 = assign14140_e13993_d_n10;

        let (assign14150_e13997, assign14150_e13997_d_n2, assign14150_e13997_d_n4, assign14150_e13997_d_n7, assign14150_e13997_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, locals.var_fn169_calc_iq__etac_dn10,)
    }
};
        locals.var_fn169_calc_iq__etac = assign14150_e13997;
        locals.var_fn169_calc_iq__etac_dn2 = assign14150_e13997_d_n2;
        locals.var_fn169_calc_iq__etac_dn4 = assign14150_e13997_d_n4;
        locals.var_fn169_calc_iq__etac_dn7 = assign14150_e13997_d_n7;
        locals.var_fn169_calc_iq__etac_dn10 = assign14150_e13997_d_n10;

        let (assign14160_e14001, assign14160_e14001_d_n3, assign14160_e14001_d_n4, assign14160_e14001_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etab, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, locals.var_fn169_calc_iq__etab_dn10,)
    }
};
        locals.var_fn169_calc_iq__etab = assign14160_e14001;
        locals.var_fn169_calc_iq__etab_dn3 = assign14160_e14001_d_n3;
        locals.var_fn169_calc_iq__etab_dn4 = assign14160_e14001_d_n4;
        locals.var_fn169_calc_iq__etab_dn10 = assign14160_e14001_d_n10;

        let (assign14170_e14005, assign14170_e14005_d_n2, assign14170_e14005_d_n4, assign14170_e14005_d_n7, assign14170_e14005_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, locals.var_fn169_calc_iq__etags_dn10,)
    }
};
        locals.var_fn169_calc_iq__etags = assign14170_e14005;
        locals.var_fn169_calc_iq__etags_dn2 = assign14170_e14005_d_n2;
        locals.var_fn169_calc_iq__etags_dn4 = assign14170_e14005_d_n4;
        locals.var_fn169_calc_iq__etags_dn7 = assign14170_e14005_d_n7;
        locals.var_fn169_calc_iq__etags_dn10 = assign14170_e14005_d_n10;

        let (assign14180_e14009, assign14180_e14009_d_n2, assign14180_e14009_d_n3, assign14180_e14009_d_n4, assign14180_e14009_d_n7, assign14180_e14009_d_n9, assign14180_e14009_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14180_e14009;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14180_e14009_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14180_e14009_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14180_e14009_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14180_e14009_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14180_e14009_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14180_e14009_d_n10;

        let (assign14190_e14013, assign14190_e14013_d_n2, assign14190_e14013_d_n3, assign14190_e14013_d_n4, assign14190_e14013_d_n7, assign14190_e14013_d_n9, assign14190_e14013_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14190_e14013;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14190_e14013_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14190_e14013_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14190_e14013_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14190_e14013_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14190_e14013_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14190_e14013_d_n10;

        let (assign14200_e14017, assign14200_e14017_d_n9, assign14200_e14017_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__absvdsin, locals.var_fn169_calc_iq__absvdsin_dn9, locals.var_fn169_calc_iq__absvdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__absvdsin = assign14200_e14017;
        locals.var_fn169_calc_iq__absvdsin_dn9 = assign14200_e14017_d_n9;
        locals.var_fn169_calc_iq__absvdsin_dn10 = assign14200_e14017_d_n10;

        let (assign14210_e14021, assign14210_e14021_d_n2, assign14210_e14021_d_n7, assign14210_e14021_d_n9, assign14210_e14021_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vgdin, locals.var_fn169_calc_iq__vgdin_dn2, locals.var_fn169_calc_iq__vgdin_dn7, locals.var_fn169_calc_iq__vgdin_dn9, locals.var_fn169_calc_iq__vgdin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgdin = assign14210_e14021;
        locals.var_fn169_calc_iq__vgdin_dn2 = assign14210_e14021_d_n2;
        locals.var_fn169_calc_iq__vgdin_dn7 = assign14210_e14021_d_n7;
        locals.var_fn169_calc_iq__vgdin_dn9 = assign14210_e14021_d_n9;
        locals.var_fn169_calc_iq__vgdin_dn10 = assign14210_e14021_d_n10;

        let (assign14220_e14025, assign14220_e14025_d_n2, assign14220_e14025_d_n4, assign14220_e14025_d_n7, assign14220_e14025_d_n9, assign14220_e14025_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign14220_e14025;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign14220_e14025_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign14220_e14025_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign14220_e14025_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign14220_e14025_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign14220_e14025_d_n10;

        let (assign14230_e14029, assign14230_e14029_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__myarg0, locals.var_fn169_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn169_calc_iq__myarg0 = assign14230_e14029;
        locals.var_fn169_calc_iq__myarg0_dn4 = assign14230_e14029_d_n4;

    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14240_e14056, assign14240_e14056_d_n9, assign14240_e14056_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14240_e14054, assign14240_e14054_d_n9, assign14240_e14054_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14240_e14038: f64 = (0.001 / p.p53);
                let assign14240_e14040: f64 = (assign14240_e14038 * locals.var_fn169_calc_iq__vdsin);
                let assign14240_e14041: f64 = (assign14240_e14040).tanh();
                let assign14240_e14042: f64 = (locals.var_fn169_calc_iq__vdsin * assign14240_e14041);
                (assign14240_e14042, ((locals.var_fn169_calc_iq__vdsin_dn9 * assign14240_e14041) + (locals.var_fn169_calc_iq__vdsin * ((assign14240_e14038 * locals.var_fn169_calc_iq__vdsin_dn9) / ((assign14240_e14040).cosh() * (assign14240_e14040).cosh())))), ((locals.var_fn169_calc_iq__vdsin_dn10 * assign14240_e14041) + (locals.var_fn169_calc_iq__vdsin * ((assign14240_e14038 * locals.var_fn169_calc_iq__vdsin_dn10) / ((assign14240_e14040).cosh() * (assign14240_e14040).cosh())))),)
            } else {
                let (assign14240_e14053, assign14240_e14053_d_n9, assign14240_e14053_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14240_e14048: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin);
                        let assign14240_e14050: f64 = (assign14240_e14048 + p.p53);
                        let assign14240_e14051: f64 = (assign14240_e14050).sqrt();
                        (assign14240_e14051, (((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsin) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin_dn9)) / (2.0 * assign14240_e14051)), (((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsin) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin_dn10)) / (2.0 * assign14240_e14051)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign14240_e14053, assign14240_e14053_d_n9, assign14240_e14053_d_n10,)
            }
        };
        (assign14240_e14054, assign14240_e14054_d_n9, assign14240_e14054_d_n10,)
    } else {
        (locals.var_fn169_calc_iq__absvdsin, locals.var_fn169_calc_iq__absvdsin_dn9, locals.var_fn169_calc_iq__absvdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__absvdsin = assign14240_e14056;
        locals.var_fn169_calc_iq__absvdsin_dn9 = assign14240_e14056_d_n9;
        locals.var_fn169_calc_iq__absvdsin_dn10 = assign14240_e14056_d_n10;

        let (assign14250_e14062, assign14250_e14062_d_n2, assign14250_e14062_d_n7, assign14250_e14062_d_n9, assign14250_e14062_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14250_e14060: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdsin);
        (assign14250_e14060, locals.var_fn169_calc_iq__vgsin_dn2, locals.var_fn169_calc_iq__vgsin_dn7, (-locals.var_fn169_calc_iq__vdsin_dn9), (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdsin_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vgdin, locals.var_fn169_calc_iq__vgdin_dn2, locals.var_fn169_calc_iq__vgdin_dn7, locals.var_fn169_calc_iq__vgdin_dn9, locals.var_fn169_calc_iq__vgdin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgdin = assign14250_e14062;
        locals.var_fn169_calc_iq__vgdin_dn2 = assign14250_e14062_d_n2;
        locals.var_fn169_calc_iq__vgdin_dn7 = assign14250_e14062_d_n7;
        locals.var_fn169_calc_iq__vgdin_dn9 = assign14250_e14062_d_n9;
        locals.var_fn169_calc_iq__vgdin_dn10 = assign14250_e14062_d_n10;

        let (assign14260_e14068, assign14260_e14068_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14260_e14066: f64 = (locals.var_fn169_calc_iq__alpha * locals.var_fn169_calc_iq__phitin);
        (assign14260_e14066, (locals.var_fn169_calc_iq__alpha * locals.var_fn169_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn169_calc_iq__alpha_phit, locals.var_fn169_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn169_calc_iq__alpha_phit = assign14260_e14068;
        locals.var_fn169_calc_iq__alpha_phit_dn4 = assign14260_e14068_d_n4;

        let (assign14270_e14080, assign14270_e14080_d_n4, assign14270_e14080_d_n9, assign14270_e14080_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14270_e14073: f64 = (2.302585092994046 * locals.var_fn169_calc_iq__phitin);
        let assign14270_e14074: f64 = (locals.var_fn169_calc_iq__ss / assign14270_e14073);
        let assign14270_e14077: f64 = (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin);
        let assign14270_e14078: f64 = (assign14270_e14074 + assign14270_e14077);
        (assign14270_e14078, (-((locals.var_fn169_calc_iq__ss * (2.302585092994046 * locals.var_fn169_calc_iq__phitin_dn4)) / (assign14270_e14073 * assign14270_e14073))), (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin_dn9), (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin_dn10),)
    } else {
        (locals.var_fn169_calc_iq__n, locals.var_fn169_calc_iq__n_dn4, locals.var_fn169_calc_iq__n_dn9, locals.var_fn169_calc_iq__n_dn10,)
    }
};
        locals.var_fn169_calc_iq__n = assign14270_e14080;
        locals.var_fn169_calc_iq__n_dn4 = assign14270_e14080_d_n4;
        locals.var_fn169_calc_iq__n_dn9 = assign14270_e14080_d_n9;
        locals.var_fn169_calc_iq__n_dn10 = assign14270_e14080_d_n10;

        let (assign14280_e14090, assign14280_e14090_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14280_e14086: f64 = (locals.var_fn169_calc_iq__tambin - locals.var_fn169_calc_iq__tnomin);
        let assign14280_e14087: f64 = (locals.var_fn169_calc_iq__vtzeta * assign14280_e14086);
        let assign14280_e14088: f64 = (locals.var_fn169_calc_iq__vto + assign14280_e14087);
        (assign14280_e14088, (locals.var_fn169_calc_iq__vtzeta * locals.var_fn169_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn169_calc_iq__vtof, locals.var_fn169_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn169_calc_iq__vtof = assign14280_e14090;
        locals.var_fn169_calc_iq__vtof_dn4 = assign14280_e14090_d_n4;

        let (assign14290_e14098, assign14290_e14098_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14290_e14094: f64 = (locals.var_fn169_calc_iq__tambin / locals.var_fn169_calc_iq__tnomin);
        let assign14290_e14096: f64 = (assign14290_e14094).powf(locals.var_fn169_calc_iq__epsilon);
        (assign14290_e14096, if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn169_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__epsilon * ((assign14290_e14094).powf(locals.var_fn169_calc_iq__epsilon - 1.0) * (locals.var_fn169_calc_iq__tambin_dn4 / locals.var_fn169_calc_iq__tnomin))) } } else { (assign14290_e14096 * (locals.var_fn169_calc_iq__epsilon * ((locals.var_fn169_calc_iq__tambin_dn4 / locals.var_fn169_calc_iq__tnomin) / assign14290_e14094))) },)
    } else {
        (locals.var_fn169_calc_iq__tfacmobin, locals.var_fn169_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tfacmobin = assign14290_e14098;
        locals.var_fn169_calc_iq__tfacmobin_dn4 = assign14290_e14098_d_n4;

        let assign14300_e14101: f64 = if locals.var_fn169_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign14300_e14101;

        let (assign14310_e14119, assign14310_e14119_d_n9, assign14310_e14119_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard170 != 0.0)) {
        let assign14310_e14109: f64 = (locals.var_fn169_calc_iq__absvdsin / locals.var_fn169_calc_iq__dibsat);
        let assign14310_e14111: f64 = (assign14310_e14109).powf(locals.var_fn169_calc_iq__beta);
        let assign14310_e14112: f64 = (1.0 + assign14310_e14111);
        let assign14310_e14115: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14310_e14116: f64 = (assign14310_e14112).powf(assign14310_e14115);
        let assign14310_e14117: f64 = (locals.var_fn169_calc_iq__absvdsin / assign14310_e14116);
        (assign14310_e14117, (((locals.var_fn169_calc_iq__absvdsin_dn9 * assign14310_e14116) - (locals.var_fn169_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign14310_e14115) as f64).is_finite() && ((assign14310_e14115) as f64).fract() == 0.0 { if assign14310_e14115 == 0.0 { 0.0 } else { (assign14310_e14115 * ((assign14310_e14112).powf(assign14310_e14115 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) })) } } else { (assign14310_e14116 * (assign14310_e14115 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) } / assign14310_e14112))) })) / (assign14310_e14116 * assign14310_e14116)), (((locals.var_fn169_calc_iq__absvdsin_dn10 * assign14310_e14116) - (locals.var_fn169_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign14310_e14115) as f64).is_finite() && ((assign14310_e14115) as f64).fract() == 0.0 { if assign14310_e14115 == 0.0 { 0.0 } else { (assign14310_e14115 * ((assign14310_e14112).powf(assign14310_e14115 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) })) } } else { (assign14310_e14116 * (assign14310_e14115 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) } / assign14310_e14112))) })) / (assign14310_e14116 * assign14310_e14116)),)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign14310_e14119;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign14310_e14119_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign14310_e14119_d_n10;

        let (assign14320_e14126, assign14320_e14126_d_n9, assign14320_e14126_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard170 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign14320_e14126;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign14320_e14126_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign14320_e14126_d_n10;

        let (assign14330_e14136, assign14330_e14136_d_n9, assign14330_e14136_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14330_e14131: f64 = (locals.var_fn169_calc_iq__vsatdibl * locals.var_fn169_calc_iq__delta2);
        let assign14330_e14132: f64 = (locals.var_fn169_calc_iq__delta1 - assign14330_e14131);
        let assign14330_e14134: f64 = (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin);
        (assign14330_e14134, (((-(locals.var_fn169_calc_iq__vsatdibl_dn9 * locals.var_fn169_calc_iq__delta2)) * locals.var_fn169_calc_iq__absvdsin) + (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin_dn9)), (((-(locals.var_fn169_calc_iq__vsatdibl_dn10 * locals.var_fn169_calc_iq__delta2)) * locals.var_fn169_calc_iq__absvdsin) + (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__delta, locals.var_fn169_calc_iq__delta_dn9, locals.var_fn169_calc_iq__delta_dn10,)
    }
};
        locals.var_fn169_calc_iq__delta = assign14330_e14136;
        locals.var_fn169_calc_iq__delta_dn9 = assign14330_e14136_d_n9;
        locals.var_fn169_calc_iq__delta_dn10 = assign14330_e14136_d_n10;

        let (assign14340_e14142, assign14340_e14142_d_n4, assign14340_e14142_d_n9, assign14340_e14142_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14340_e14140: f64 = (locals.var_fn169_calc_iq__vtof - locals.var_fn169_calc_iq__delta);
        (assign14340_e14140, locals.var_fn169_calc_iq__vtof_dn4, (-locals.var_fn169_calc_iq__delta_dn9), (-locals.var_fn169_calc_iq__delta_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vtdibl, locals.var_fn169_calc_iq__vtdibl_dn4, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vtdibl = assign14340_e14142;
        locals.var_fn169_calc_iq__vtdibl_dn4 = assign14340_e14142_d_n4;
        locals.var_fn169_calc_iq__vtdibl_dn9 = assign14340_e14142_d_n9;
        locals.var_fn169_calc_iq__vtdibl_dn10 = assign14340_e14142_d_n10;

        let (assign14350_e14150, assign14350_e14150_d_n4, assign14350_e14150_d_n9, assign14350_e14150_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14350_e14146: f64 = (2.0 * locals.var_fn169_calc_iq__n);
        let assign14350_e14148: f64 = (assign14350_e14146 * locals.var_fn169_calc_iq__phitin);
        (assign14350_e14148, (((2.0 * locals.var_fn169_calc_iq__n_dn4) * locals.var_fn169_calc_iq__phitin) + (assign14350_e14146 * locals.var_fn169_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn169_calc_iq__n_dn9) * locals.var_fn169_calc_iq__phitin), ((2.0 * locals.var_fn169_calc_iq__n_dn10) * locals.var_fn169_calc_iq__phitin),)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit, locals.var_fn169_calc_iq__two_n_phit_dn4, locals.var_fn169_calc_iq__two_n_phit_dn9, locals.var_fn169_calc_iq__two_n_phit_dn10,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit = assign14350_e14150;
        locals.var_fn169_calc_iq__two_n_phit_dn4 = assign14350_e14150_d_n4;
        locals.var_fn169_calc_iq__two_n_phit_dn9 = assign14350_e14150_d_n9;
        locals.var_fn169_calc_iq__two_n_phit_dn10 = assign14350_e14150_d_n10;

        let (assign14360_e14156, assign14360_e14156_d_n4, assign14360_e14156_d_n9, assign14360_e14156_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14360_e14154: f64 = (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit);
        (assign14360_e14154, ((locals.var_fn169_calc_iq__cgin_dn4 * locals.var_fn169_calc_iq__two_n_phit) + (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn4)), (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn9), (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qref, locals.var_fn169_calc_iq__qref_dn4, locals.var_fn169_calc_iq__qref_dn9, locals.var_fn169_calc_iq__qref_dn10,)
    }
};
        locals.var_fn169_calc_iq__qref = assign14360_e14156;
        locals.var_fn169_calc_iq__qref_dn4 = assign14360_e14156_d_n4;
        locals.var_fn169_calc_iq__qref_dn9 = assign14360_e14156_d_n9;
        locals.var_fn169_calc_iq__qref_dn10 = assign14360_e14156_d_n10;

        let (assign14370_e14166, assign14370_e14166_d_n2, assign14370_e14166_d_n3, assign14370_e14166_d_n4, assign14370_e14166_d_n7, assign14370_e14166_d_n9, assign14370_e14166_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14370_e14161: f64 = (p.p51 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14370_e14163: f64 = (assign14370_e14161 / 2.0);
        let assign14370_e14164: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14370_e14163);
        (assign14370_e14164, 0.0, 0.0, (locals.var_fn169_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn169_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14370_e14166;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14370_e14166_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14370_e14166_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14370_e14166_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14370_e14166_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14370_e14166_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14370_e14166_d_n10;

        let (assign14380_e14217, assign14380_e14217_d_n2, assign14380_e14217_d_n3, assign14380_e14217_d_n4, assign14380_e14217_d_n7, assign14380_e14217_d_n9, assign14380_e14217_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14380_e14211, assign14380_e14211_d_n2, assign14380_e14211_d_n7, assign14380_e14211_d_n9, assign14380_e14211_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14380_e14175: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14178: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14181: f64 = (0.001 / p.p53);
                let assign14380_e14184: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14185: f64 = (assign14380_e14181 * assign14380_e14184);
                let assign14380_e14186: f64 = (assign14380_e14185).tanh();
                let assign14380_e14187: f64 = (assign14380_e14178 * assign14380_e14186);
                let assign14380_e14188: f64 = (assign14380_e14175 + assign14380_e14187);
                let assign14380_e14189: f64 = (0.5 * assign14380_e14188);
                (assign14380_e14189, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))),)
            } else {
                let (assign14380_e14210, assign14380_e14210_d_n2, assign14380_e14210_d_n7, assign14380_e14210_d_n9, assign14380_e14210_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14380_e14196: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14199: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14202: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14203: f64 = (assign14380_e14199 * assign14380_e14202);
                        let assign14380_e14205: f64 = (assign14380_e14203 + p.p53);
                        let assign14380_e14206: f64 = (assign14380_e14205).sqrt();
                        let assign14380_e14207: f64 = (assign14380_e14196 + assign14380_e14206);
                        let assign14380_e14208: f64 = (0.5 * assign14380_e14207);
                        (assign14380_e14208, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14380_e14206)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14380_e14206)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14380_e14202) + (assign14380_e14199 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14380_e14206)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14380_e14206)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14380_e14210, assign14380_e14210_d_n2, assign14380_e14210_d_n7, assign14380_e14210_d_n9, assign14380_e14210_d_n10,)
            }
        };
        let assign14380_e14213: f64 = (assign14380_e14211 - locals.var_fn169_calc_iq__myarg);
        let assign14380_e14215: f64 = (assign14380_e14213 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14380_e14215, ((assign14380_e14211_d_n2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14380_e14213 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((assign14380_e14211_d_n7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((assign14380_e14211_d_n9 - locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((assign14380_e14211_d_n10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14380_e14217;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14380_e14217_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14380_e14217_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14380_e14217_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14380_e14217_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14380_e14217_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14380_e14217_d_n10;

        let assign14390_e14220: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign14390_e14220;

        let (assign14400_e14226, assign14400_e14226_d_n2, assign14400_e14226_d_n3, assign14400_e14226_d_n4, assign14400_e14226_d_n7, assign14400_e14226_d_n9, assign14400_e14226_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard171 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14400_e14226;
        locals.var_fn169_calc_iq__ff_dn2 = assign14400_e14226_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14400_e14226_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14400_e14226_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14400_e14226_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14400_e14226_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14400_e14226_d_n10;

        let assign14410_e14229: f64 = (-50.0);
        let assign14410_e14230: f64 = if locals.var_fn169_calc_iq__exparg < assign14410_e14229 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign14410_e14230;

        let (assign14420_e14239, assign14420_e14239_d_n2, assign14420_e14239_d_n3, assign14420_e14239_d_n4, assign14420_e14239_d_n7, assign14420_e14239_d_n9, assign14420_e14239_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14420_e14239;
        locals.var_fn169_calc_iq__ff_dn2 = assign14420_e14239_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14420_e14239_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14420_e14239_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14420_e14239_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14420_e14239_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14420_e14239_d_n10;

        let (assign14430_e14254, assign14430_e14254_d_n2, assign14430_e14254_d_n3, assign14430_e14254_d_n4, assign14430_e14254_d_n7, assign14430_e14254_d_n9, assign14430_e14254_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 == 0.0)) {
        let assign14430_e14250: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14430_e14251: f64 = (1.0 + assign14430_e14250);
        let assign14430_e14252: f64 = (1.0 / assign14430_e14251);
        (assign14430_e14252, (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14430_e14251 * assign14430_e14251))),)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14430_e14254;
        locals.var_fn169_calc_iq__ff_dn2 = assign14430_e14254_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14430_e14254_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14430_e14254_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14430_e14254_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14430_e14254_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14430_e14254_d_n10;

        let (assign14440_e14313, assign14440_e14313_d_n2, assign14440_e14313_d_n3, assign14440_e14313_d_n4, assign14440_e14313_d_n7, assign14440_e14313_d_n9, assign14440_e14313_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14440_e14299, assign14440_e14299_d_n2, assign14440_e14299_d_n7, assign14440_e14299_d_n9, assign14440_e14299_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14440_e14263: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14266: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14269: f64 = (0.001 / p.p53);
                let assign14440_e14272: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14273: f64 = (assign14440_e14269 * assign14440_e14272);
                let assign14440_e14274: f64 = (assign14440_e14273).tanh();
                let assign14440_e14275: f64 = (assign14440_e14266 * assign14440_e14274);
                let assign14440_e14276: f64 = (assign14440_e14263 + assign14440_e14275);
                let assign14440_e14277: f64 = (0.5 * assign14440_e14276);
                (assign14440_e14277, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))),)
            } else {
                let (assign14440_e14298, assign14440_e14298_d_n2, assign14440_e14298_d_n7, assign14440_e14298_d_n9, assign14440_e14298_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14440_e14284: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14287: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14290: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14291: f64 = (assign14440_e14287 * assign14440_e14290);
                        let assign14440_e14293: f64 = (assign14440_e14291 + p.p53);
                        let assign14440_e14294: f64 = (assign14440_e14293).sqrt();
                        let assign14440_e14295: f64 = (assign14440_e14284 + assign14440_e14294);
                        let assign14440_e14296: f64 = (0.5 * assign14440_e14295);
                        (assign14440_e14296, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14440_e14294)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14440_e14294)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14440_e14290) + (assign14440_e14287 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14440_e14294)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14440_e14294)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14440_e14298, assign14440_e14298_d_n2, assign14440_e14298_d_n7, assign14440_e14298_d_n9, assign14440_e14298_d_n10,)
            }
        };
        let assign14440_e14303: f64 = (p.p51 * 0.1);
        let assign14440_e14305: f64 = (assign14440_e14303 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14440_e14307: f64 = (assign14440_e14305 * locals.var_fn169_calc_iq__ff);
        let assign14440_e14308: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14440_e14307);
        let assign14440_e14309: f64 = (assign14440_e14299 - assign14440_e14308);
        let assign14440_e14311: f64 = (assign14440_e14309 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14440_e14311, ((assign14440_e14299_d_n2 - (-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn2))) / locals.var_fn169_calc_iq__two_n_phit), ((-(-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn3))) / locals.var_fn169_calc_iq__two_n_phit), ((((-(locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14440_e14303 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ff) + (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), ((assign14440_e14299_d_n7 - (-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn7))) / locals.var_fn169_calc_iq__two_n_phit), ((((assign14440_e14299_d_n9 - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), ((((assign14440_e14299_d_n10 - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__eta, locals.var_fn169_calc_iq__eta_dn2, locals.var_fn169_calc_iq__eta_dn3, locals.var_fn169_calc_iq__eta_dn4, locals.var_fn169_calc_iq__eta_dn7, locals.var_fn169_calc_iq__eta_dn9, locals.var_fn169_calc_iq__eta_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta = assign14440_e14313;
        locals.var_fn169_calc_iq__eta_dn2 = assign14440_e14313_d_n2;
        locals.var_fn169_calc_iq__eta_dn3 = assign14440_e14313_d_n3;
        locals.var_fn169_calc_iq__eta_dn4 = assign14440_e14313_d_n4;
        locals.var_fn169_calc_iq__eta_dn7 = assign14440_e14313_d_n7;
        locals.var_fn169_calc_iq__eta_dn9 = assign14440_e14313_d_n9;
        locals.var_fn169_calc_iq__eta_dn10 = assign14440_e14313_d_n10;

        let assign14450_e14316: f64 = if locals.var_fn169_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign14450_e14316;

        let (assign14460_e14324, assign14460_e14324_d_n2, assign14460_e14324_d_n3, assign14460_e14324_d_n4, assign14460_e14324_d_n7, assign14460_e14324_d_n9, assign14460_e14324_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard173 != 0.0)) {
        let assign14460_e14322: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta);
        (assign14460_e14322, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14460_e14324;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14460_e14324_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14460_e14324_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14460_e14324_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14460_e14324_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14460_e14324_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14460_e14324_d_n10;

        let assign14470_e14327: f64 = (-50.0);
        let assign14470_e14328: f64 = if locals.var_fn169_calc_iq__eta < assign14470_e14327 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign14470_e14328;

        let (assign14480_e14340, assign14480_e14340_d_n2, assign14480_e14340_d_n3, assign14480_e14340_d_n4, assign14480_e14340_d_n7, assign14480_e14340_d_n9, assign14480_e14340_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard173 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign14480_e14337: f64 = (locals.var_fn169_calc_iq__eta).exp();
        let assign14480_e14338: f64 = (locals.var_fn169_calc_iq__qref * assign14480_e14337);
        (assign14480_e14338, (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn2)), (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn4))), (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14480_e14340;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14480_e14340_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14480_e14340_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14480_e14340_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14480_e14340_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14480_e14340_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14480_e14340_d_n10;

        let (assign14490_e14356, assign14490_e14356_d_n2, assign14490_e14356_d_n3, assign14490_e14356_d_n4, assign14490_e14356_d_n7, assign14490_e14356_d_n9, assign14490_e14356_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard173 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign14490_e14351: f64 = (locals.var_fn169_calc_iq__eta).exp();
        let assign14490_e14352: f64 = (1.0 + assign14490_e14351);
        let assign14490_e14353: f64 = (assign14490_e14352).ln();
        let assign14490_e14354: f64 = (locals.var_fn169_calc_iq__qref * assign14490_e14353);
        (assign14490_e14354, (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn2) / assign14490_e14352)), (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn3) / assign14490_e14352)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn4) / assign14490_e14352))), (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn7) / assign14490_e14352)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn9) / assign14490_e14352))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn10) / assign14490_e14352))),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14490_e14356;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14490_e14356_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14490_e14356_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14490_e14356_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14490_e14356_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14490_e14356_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14490_e14356_d_n10;

        let (assign14500_e14370, assign14500_e14370_d_n2, assign14500_e14370_d_n3, assign14500_e14370_d_n4, assign14500_e14370_d_n7, assign14500_e14370_d_n9, assign14500_e14370_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14500_e14363: f64 = (locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv);
        let assign14500_e14365: f64 = (assign14500_e14363 / locals.var_fn169_calc_iq__cgin);
        let assign14500_e14366: f64 = (1.0 + assign14500_e14365);
        let assign14500_e14367: f64 = (locals.var_fn169_calc_iq__tfacmobin * assign14500_e14366);
        let assign14500_e14368: f64 = (locals.var_fn169_calc_iq__mu0 / assign14500_e14367);
        (assign14500_e14368, (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * ((locals.var_fn169_calc_iq__tfacmobin_dn4 * assign14500_e14366) + (locals.var_fn169_calc_iq__tfacmobin * ((((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14500_e14363 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin))))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))),)
    } else {
        (locals.var_fn169_calc_iq__muf, locals.var_fn169_calc_iq__muf_dn2, locals.var_fn169_calc_iq__muf_dn3, locals.var_fn169_calc_iq__muf_dn4, locals.var_fn169_calc_iq__muf_dn7, locals.var_fn169_calc_iq__muf_dn9, locals.var_fn169_calc_iq__muf_dn10,)
    }
};
        locals.var_fn169_calc_iq__muf = assign14500_e14370;
        locals.var_fn169_calc_iq__muf_dn2 = assign14500_e14370_d_n2;
        locals.var_fn169_calc_iq__muf_dn3 = assign14500_e14370_d_n3;
        locals.var_fn169_calc_iq__muf_dn4 = assign14500_e14370_d_n4;
        locals.var_fn169_calc_iq__muf_dn7 = assign14500_e14370_d_n7;
        locals.var_fn169_calc_iq__muf_dn9 = assign14500_e14370_d_n9;
        locals.var_fn169_calc_iq__muf_dn10 = assign14500_e14370_d_n10;

        let (assign14510_e14402, assign14510_e14402_d_n2, assign14510_e14402_d_n3, assign14510_e14402_d_n4, assign14510_e14402_d_n7, assign14510_e14402_d_n9, assign14510_e14402_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14510_e14376: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tnomin);
        let assign14510_e14377: f64 = (1.0 + assign14510_e14376);
        let assign14510_e14381: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin);
        let assign14510_e14382: f64 = (1.0 + assign14510_e14381);
        let assign14510_e14383: f64 = (assign14510_e14377 / assign14510_e14382);
        let assign14510_e14384: f64 = (locals.var_fn169_calc_iq__vel0 * assign14510_e14383);
        let assign14510_e14388: f64 = (locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin);
        let assign14510_e14390: f64 = (assign14510_e14388 / locals.var_fn169_calc_iq__lin);
        let assign14510_e14391: f64 = (1.0 + assign14510_e14390);
        let assign14510_e14392: f64 = (assign14510_e14384 * assign14510_e14391);
        let assign14510_e14396: f64 = (locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv);
        let assign14510_e14398: f64 = (assign14510_e14396 / locals.var_fn169_calc_iq__cgin);
        let assign14510_e14399: f64 = (1.0 + assign14510_e14398);
        let assign14510_e14400: f64 = (assign14510_e14392 / assign14510_e14399);
        (assign14510_e14400, (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), (((((locals.var_fn169_calc_iq__vel0 * (-((assign14510_e14377 * (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin_dn4)) / (assign14510_e14382 * assign14510_e14382)))) * assign14510_e14391) * assign14510_e14399) - (assign14510_e14392 * ((((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14510_e14396 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)))) / (assign14510_e14399 * assign14510_e14399)), (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), ((((assign14510_e14384 * ((locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin_dn9) / locals.var_fn169_calc_iq__lin)) * assign14510_e14399) - (assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin))) / (assign14510_e14399 * assign14510_e14399)), ((((assign14510_e14384 * ((locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin_dn10) / locals.var_fn169_calc_iq__lin)) * assign14510_e14399) - (assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin))) / (assign14510_e14399 * assign14510_e14399)),)
    } else {
        (locals.var_fn169_calc_iq__vx, locals.var_fn169_calc_iq__vx_dn2, locals.var_fn169_calc_iq__vx_dn3, locals.var_fn169_calc_iq__vx_dn4, locals.var_fn169_calc_iq__vx_dn7, locals.var_fn169_calc_iq__vx_dn9, locals.var_fn169_calc_iq__vx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vx = assign14510_e14402;
        locals.var_fn169_calc_iq__vx_dn2 = assign14510_e14402_d_n2;
        locals.var_fn169_calc_iq__vx_dn3 = assign14510_e14402_d_n3;
        locals.var_fn169_calc_iq__vx_dn4 = assign14510_e14402_d_n4;
        locals.var_fn169_calc_iq__vx_dn7 = assign14510_e14402_d_n7;
        locals.var_fn169_calc_iq__vx_dn9 = assign14510_e14402_d_n9;
        locals.var_fn169_calc_iq__vx_dn10 = assign14510_e14402_d_n10;

        let (assign14520_e14420, assign14520_e14420_d_n2, assign14520_e14420_d_n3, assign14520_e14420_d_n4, assign14520_e14420_d_n7, assign14520_e14420_d_n9, assign14520_e14420_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14520_e14406: f64 = (2.0 * locals.var_fn169_calc_iq__ff);
        let assign14520_e14408: f64 = (assign14520_e14406 * locals.var_fn169_calc_iq__phitin);
        let assign14520_e14410: f64 = (assign14520_e14408 * locals.var_fn169_calc_iq__muf);
        let assign14520_e14412: f64 = (assign14520_e14410 / locals.var_fn169_calc_iq__lin);
        let assign14520_e14415: f64 = (1.0 - locals.var_fn169_calc_iq__ff);
        let assign14520_e14417: f64 = (assign14520_e14415 * locals.var_fn169_calc_iq__vx);
        let assign14520_e14418: f64 = (assign14520_e14412 + assign14520_e14417);
        (assign14520_e14418, ((((((2.0 * locals.var_fn169_calc_iq__ff_dn2) * locals.var_fn169_calc_iq__phitin) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn2)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn2) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn169_calc_iq__ff_dn3) * locals.var_fn169_calc_iq__phitin) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn3)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn3) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn169_calc_iq__ff_dn4) * locals.var_fn169_calc_iq__phitin) + (assign14520_e14406 * locals.var_fn169_calc_iq__phitin_dn4)) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn4)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn4) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn169_calc_iq__ff_dn7) * locals.var_fn169_calc_iq__phitin) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn7)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn7) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn169_calc_iq__ff_dn9) * locals.var_fn169_calc_iq__phitin) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn9)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn9) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn9))), ((((((2.0 * locals.var_fn169_calc_iq__ff_dn10) * locals.var_fn169_calc_iq__phitin) * locals.var_fn169_calc_iq__muf) + (assign14520_e14408 * locals.var_fn169_calc_iq__muf_dn10)) / locals.var_fn169_calc_iq__lin) + (((-locals.var_fn169_calc_iq__ff_dn10) * locals.var_fn169_calc_iq__vx) + (assign14520_e14415 * locals.var_fn169_calc_iq__vx_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__vxf, locals.var_fn169_calc_iq__vxf_dn2, locals.var_fn169_calc_iq__vxf_dn3, locals.var_fn169_calc_iq__vxf_dn4, locals.var_fn169_calc_iq__vxf_dn7, locals.var_fn169_calc_iq__vxf_dn9, locals.var_fn169_calc_iq__vxf_dn10,)
    }
};
        locals.var_fn169_calc_iq__vxf = assign14520_e14420;
        locals.var_fn169_calc_iq__vxf_dn2 = assign14520_e14420_d_n2;
        locals.var_fn169_calc_iq__vxf_dn3 = assign14520_e14420_d_n3;
        locals.var_fn169_calc_iq__vxf_dn4 = assign14520_e14420_d_n4;
        locals.var_fn169_calc_iq__vxf_dn7 = assign14520_e14420_d_n7;
        locals.var_fn169_calc_iq__vxf_dn9 = assign14520_e14420_d_n9;
        locals.var_fn169_calc_iq__vxf_dn10 = assign14520_e14420_d_n10;

        let (assign14530_e14428, assign14530_e14428_d_n2, assign14530_e14428_d_n3, assign14530_e14428_d_n4, assign14530_e14428_d_n7, assign14530_e14428_d_n9, assign14530_e14428_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14530_e14424: f64 = (locals.var_fn169_calc_iq__vx * locals.var_fn169_calc_iq__lin);
        let assign14530_e14426: f64 = (assign14530_e14424 / locals.var_fn169_calc_iq__muf);
        (assign14530_e14426, ((((locals.var_fn169_calc_iq__vx_dn2 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn2)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn3 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn3)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn4 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn4)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn7 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn7)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn9 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn9)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn10 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn10)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)),)
    } else {
        (locals.var_fn169_calc_iq__vdsats, locals.var_fn169_calc_iq__vdsats_dn2, locals.var_fn169_calc_iq__vdsats_dn3, locals.var_fn169_calc_iq__vdsats_dn4, locals.var_fn169_calc_iq__vdsats_dn7, locals.var_fn169_calc_iq__vdsats_dn9, locals.var_fn169_calc_iq__vdsats_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats = assign14530_e14428;
        locals.var_fn169_calc_iq__vdsats_dn2 = assign14530_e14428_d_n2;
        locals.var_fn169_calc_iq__vdsats_dn3 = assign14530_e14428_d_n3;
        locals.var_fn169_calc_iq__vdsats_dn4 = assign14530_e14428_d_n4;
        locals.var_fn169_calc_iq__vdsats_dn7 = assign14530_e14428_d_n7;
        locals.var_fn169_calc_iq__vdsats_dn9 = assign14530_e14428_d_n9;
        locals.var_fn169_calc_iq__vdsats_dn10 = assign14530_e14428_d_n10;

    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14540_e14445, assign14540_e14445_d_n2, assign14540_e14445_d_n3, assign14540_e14445_d_n4, assign14540_e14445_d_n7, assign14540_e14445_d_n9, assign14540_e14445_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14540_e14434: f64 = (2.0 * locals.var_fn169_calc_iq__qinvv);
        let assign14540_e14436: f64 = (assign14540_e14434 / locals.var_fn169_calc_iq__cgin);
        let assign14540_e14438: f64 = (assign14540_e14436 / locals.var_fn169_calc_iq__vdsats);
        let assign14540_e14439: f64 = (1.0 + assign14540_e14438);
        let assign14540_e14440: f64 = (assign14540_e14439).sqrt();
        let assign14540_e14441: f64 = (locals.var_fn169_calc_iq__vdsats * assign14540_e14440);
        let assign14540_e14443: f64 = (assign14540_e14441 - locals.var_fn169_calc_iq__vdsats);
        (assign14540_e14443, (((locals.var_fn169_calc_iq__vdsats_dn2 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn2)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn2), (((locals.var_fn169_calc_iq__vdsats_dn3 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn3)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn3), (((locals.var_fn169_calc_iq__vdsats_dn4 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14540_e14434 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn4)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn4), (((locals.var_fn169_calc_iq__vdsats_dn7 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn7)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn7), (((locals.var_fn169_calc_iq__vdsats_dn9 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn9)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn9), (((locals.var_fn169_calc_iq__vdsats_dn10 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn10)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vdsats1, locals.var_fn169_calc_iq__vdsats1_dn2, locals.var_fn169_calc_iq__vdsats1_dn3, locals.var_fn169_calc_iq__vdsats1_dn4, locals.var_fn169_calc_iq__vdsats1_dn7, locals.var_fn169_calc_iq__vdsats1_dn9, locals.var_fn169_calc_iq__vdsats1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats1 = assign14540_e14445;
        locals.var_fn169_calc_iq__vdsats1_dn2 = assign14540_e14445_d_n2;
        locals.var_fn169_calc_iq__vdsats1_dn3 = assign14540_e14445_d_n3;
        locals.var_fn169_calc_iq__vdsats1_dn4 = assign14540_e14445_d_n4;
        locals.var_fn169_calc_iq__vdsats1_dn7 = assign14540_e14445_d_n7;
        locals.var_fn169_calc_iq__vdsats1_dn9 = assign14540_e14445_d_n9;
        locals.var_fn169_calc_iq__vdsats1_dn10 = assign14540_e14445_d_n10;

        let (assign14550_e14457, assign14550_e14457_d_n2, assign14550_e14457_d_n3, assign14550_e14457_d_n4, assign14550_e14457_d_n7, assign14550_e14457_d_n9, assign14550_e14457_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14550_e14450: f64 = (1.0 - locals.var_fn169_calc_iq__ff);
        let assign14550_e14451: f64 = (locals.var_fn169_calc_iq__vdsats * assign14550_e14450);
        let assign14550_e14454: f64 = (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff);
        let assign14550_e14455: f64 = (assign14550_e14451 + assign14550_e14454);
        (assign14550_e14455, (((locals.var_fn169_calc_iq__vdsats_dn2 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn2))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn2)), (((locals.var_fn169_calc_iq__vdsats_dn3 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn3))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn3)), (((locals.var_fn169_calc_iq__vdsats_dn4 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit_dn4 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn4))), (((locals.var_fn169_calc_iq__vdsats_dn7 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn7))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn7)), (((locals.var_fn169_calc_iq__vdsats_dn9 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn9))) + ((locals.var_fn169_calc_iq__two_n_phit_dn9 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn9))), (((locals.var_fn169_calc_iq__vdsats_dn10 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn10))) + ((locals.var_fn169_calc_iq__two_n_phit_dn10 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__vdsat, locals.var_fn169_calc_iq__vdsat_dn2, locals.var_fn169_calc_iq__vdsat_dn3, locals.var_fn169_calc_iq__vdsat_dn4, locals.var_fn169_calc_iq__vdsat_dn7, locals.var_fn169_calc_iq__vdsat_dn9, locals.var_fn169_calc_iq__vdsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat = assign14550_e14457;
        locals.var_fn169_calc_iq__vdsat_dn2 = assign14550_e14457_d_n2;
        locals.var_fn169_calc_iq__vdsat_dn3 = assign14550_e14457_d_n3;
        locals.var_fn169_calc_iq__vdsat_dn4 = assign14550_e14457_d_n4;
        locals.var_fn169_calc_iq__vdsat_dn7 = assign14550_e14457_d_n7;
        locals.var_fn169_calc_iq__vdsat_dn9 = assign14550_e14457_d_n9;
        locals.var_fn169_calc_iq__vdsat_dn10 = assign14550_e14457_d_n10;

        let (assign14560_e14469, assign14560_e14469_d_n2, assign14560_e14469_d_n3, assign14560_e14469_d_n4, assign14560_e14469_d_n7, assign14560_e14469_d_n9, assign14560_e14469_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14560_e14462: f64 = (1.0 - locals.var_fn169_calc_iq__ff);
        let assign14560_e14463: f64 = (locals.var_fn169_calc_iq__vdsats1 * assign14560_e14462);
        let assign14560_e14466: f64 = (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff);
        let assign14560_e14467: f64 = (assign14560_e14463 + assign14560_e14466);
        (assign14560_e14467, (((locals.var_fn169_calc_iq__vdsats1_dn2 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn2))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn2)), (((locals.var_fn169_calc_iq__vdsats1_dn3 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn3))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn3)), (((locals.var_fn169_calc_iq__vdsats1_dn4 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit_dn4 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn4))), (((locals.var_fn169_calc_iq__vdsats1_dn7 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn7))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn7)), (((locals.var_fn169_calc_iq__vdsats1_dn9 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn9))) + ((locals.var_fn169_calc_iq__two_n_phit_dn9 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn9))), (((locals.var_fn169_calc_iq__vdsats1_dn10 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn10))) + ((locals.var_fn169_calc_iq__two_n_phit_dn10 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__vdsat1, locals.var_fn169_calc_iq__vdsat1_dn2, locals.var_fn169_calc_iq__vdsat1_dn3, locals.var_fn169_calc_iq__vdsat1_dn4, locals.var_fn169_calc_iq__vdsat1_dn7, locals.var_fn169_calc_iq__vdsat1_dn9, locals.var_fn169_calc_iq__vdsat1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat1 = assign14560_e14469;
        locals.var_fn169_calc_iq__vdsat1_dn2 = assign14560_e14469_d_n2;
        locals.var_fn169_calc_iq__vdsat1_dn3 = assign14560_e14469_d_n3;
        locals.var_fn169_calc_iq__vdsat1_dn4 = assign14560_e14469_d_n4;
        locals.var_fn169_calc_iq__vdsat1_dn7 = assign14560_e14469_d_n7;
        locals.var_fn169_calc_iq__vdsat1_dn9 = assign14560_e14469_d_n9;
        locals.var_fn169_calc_iq__vdsat1_dn10 = assign14560_e14469_d_n10;

        let (assign14570_e14538, assign14570_e14538_d_n2, assign14570_e14538_d_n3, assign14570_e14538_d_n4, assign14570_e14538_d_n7, assign14570_e14538_d_n9, assign14570_e14538_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14570_e14528, assign14570_e14528_d_n2, assign14570_e14528_d_n3, assign14570_e14528_d_n4, assign14570_e14528_d_n7, assign14570_e14528_d_n9, assign14570_e14528_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14570_e14481: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14482: f64 = assign14570_e14481;
                let assign14570_e14486: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14487: f64 = (-assign14570_e14486);
                let assign14570_e14490: f64 = (0.001 / p.p53);
                let assign14570_e14494: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14495: f64 = (-assign14570_e14494);
                let assign14570_e14496: f64 = (assign14570_e14490 * assign14570_e14495);
                let assign14570_e14497: f64 = (assign14570_e14496).tanh();
                let assign14570_e14498: f64 = (assign14570_e14487 * assign14570_e14497);
                let assign14570_e14499: f64 = (assign14570_e14482 + assign14570_e14498);
                let assign14570_e14500: f64 = (0.5 * assign14570_e14499);
                (assign14570_e14500, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))),)
            } else {
                let (assign14570_e14527, assign14570_e14527_d_n2, assign14570_e14527_d_n3, assign14570_e14527_d_n4, assign14570_e14527_d_n7, assign14570_e14527_d_n9, assign14570_e14527_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14570_e14508: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14509: f64 = assign14570_e14508;
                        let assign14570_e14513: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14514: f64 = (-assign14570_e14513);
                        let assign14570_e14518: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14519: f64 = (-assign14570_e14518);
                        let assign14570_e14520: f64 = (assign14570_e14514 * assign14570_e14519);
                        let assign14570_e14522: f64 = (assign14570_e14520 + p.p53);
                        let assign14570_e14523: f64 = (assign14570_e14522).sqrt();
                        let assign14570_e14524: f64 = (assign14570_e14509 + assign14570_e14523);
                        let assign14570_e14525: f64 = (0.5 * assign14570_e14524);
                        (assign14570_e14525, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14519) + (assign14570_e14514 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14570_e14523)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14519) + (assign14570_e14514 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14570_e14523)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14570_e14527, assign14570_e14527_d_n2, assign14570_e14527_d_n3, assign14570_e14527_d_n4, assign14570_e14527_d_n7, assign14570_e14527_d_n9, assign14570_e14527_d_n10,)
            }
        };
        let assign14570_e14530: f64 = (assign14570_e14528).powf(locals.var_fn169_calc_iq__beta);
        let assign14570_e14531: f64 = (1.0 + assign14570_e14530);
        let assign14570_e14534: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14570_e14535: f64 = (assign14570_e14531).powf(assign14570_e14534);
        let assign14570_e14536: f64 = (1.0 / assign14570_e14535);
        (assign14570_e14536, (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n2)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n2 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n2)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n2 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n3)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n3 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n3)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n3 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n4)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n4 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n4)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n4 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n7)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n7 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n7)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n7 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n9)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n9 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n9)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n9 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n10)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n10 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n10)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n10 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))),)
    } else {
        (locals.var_fn169_calc_iq__fsd, locals.var_fn169_calc_iq__fsd_dn2, locals.var_fn169_calc_iq__fsd_dn3, locals.var_fn169_calc_iq__fsd_dn4, locals.var_fn169_calc_iq__fsd_dn7, locals.var_fn169_calc_iq__fsd_dn9, locals.var_fn169_calc_iq__fsd_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd = assign14570_e14538;
        locals.var_fn169_calc_iq__fsd_dn2 = assign14570_e14538_d_n2;
        locals.var_fn169_calc_iq__fsd_dn3 = assign14570_e14538_d_n3;
        locals.var_fn169_calc_iq__fsd_dn4 = assign14570_e14538_d_n4;
        locals.var_fn169_calc_iq__fsd_dn7 = assign14570_e14538_d_n7;
        locals.var_fn169_calc_iq__fsd_dn9 = assign14570_e14538_d_n9;
        locals.var_fn169_calc_iq__fsd_dn10 = assign14570_e14538_d_n10;

        let (assign14580_e14544, assign14580_e14544_d_n2, assign14580_e14544_d_n3, assign14580_e14544_d_n4, assign14580_e14544_d_n7, assign14580_e14544_d_n9, assign14580_e14544_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14580_e14542: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd);
        (assign14580_e14542, (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn2), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn3), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn4), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn7), ((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__fsd) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn9)), ((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__fsd) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdx, locals.var_fn169_calc_iq__vdx_dn2, locals.var_fn169_calc_iq__vdx_dn3, locals.var_fn169_calc_iq__vdx_dn4, locals.var_fn169_calc_iq__vdx_dn7, locals.var_fn169_calc_iq__vdx_dn9, locals.var_fn169_calc_iq__vdx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx = assign14580_e14544;
        locals.var_fn169_calc_iq__vdx_dn2 = assign14580_e14544_d_n2;
        locals.var_fn169_calc_iq__vdx_dn3 = assign14580_e14544_d_n3;
        locals.var_fn169_calc_iq__vdx_dn4 = assign14580_e14544_d_n4;
        locals.var_fn169_calc_iq__vdx_dn7 = assign14580_e14544_d_n7;
        locals.var_fn169_calc_iq__vdx_dn9 = assign14580_e14544_d_n9;
        locals.var_fn169_calc_iq__vdx_dn10 = assign14580_e14544_d_n10;

        let (assign14590_e14619, assign14590_e14619_d_n2, assign14590_e14619_d_n3, assign14590_e14619_d_n4, assign14590_e14619_d_n7, assign14590_e14619_d_n9, assign14590_e14619_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14590_e14609, assign14590_e14609_d_n2, assign14590_e14609_d_n3, assign14590_e14609_d_n4, assign14590_e14609_d_n7, assign14590_e14609_d_n9, assign14590_e14609_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14590_e14555: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14557: f64 = (assign14590_e14555 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14558: f64 = assign14590_e14557;
                let assign14590_e14561: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14563: f64 = (assign14590_e14561 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14564: f64 = (-assign14590_e14563);
                let assign14590_e14567: f64 = (0.001 / p.p53);
                let assign14590_e14570: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14572: f64 = (assign14590_e14570 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14573: f64 = (-assign14590_e14572);
                let assign14590_e14574: f64 = (assign14590_e14567 * assign14590_e14573);
                let assign14590_e14575: f64 = (assign14590_e14574).tanh();
                let assign14590_e14576: f64 = (assign14590_e14564 * assign14590_e14575);
                let assign14590_e14577: f64 = (assign14590_e14558 + assign14590_e14576);
                let assign14590_e14578: f64 = (0.5 * assign14590_e14577);
                (assign14590_e14578, (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))),)
            } else {
                let (assign14590_e14608, assign14590_e14608_d_n2, assign14590_e14608_d_n3, assign14590_e14608_d_n4, assign14590_e14608_d_n7, assign14590_e14608_d_n9, assign14590_e14608_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14590_e14585: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14587: f64 = (assign14590_e14585 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14588: f64 = assign14590_e14587;
                        let assign14590_e14591: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14593: f64 = (assign14590_e14591 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14594: f64 = (-assign14590_e14593);
                        let assign14590_e14597: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14599: f64 = (assign14590_e14597 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14600: f64 = (-assign14590_e14599);
                        let assign14590_e14601: f64 = (assign14590_e14594 * assign14590_e14600);
                        let assign14590_e14603: f64 = (assign14590_e14601 + p.p53);
                        let assign14590_e14604: f64 = (assign14590_e14603).sqrt();
                        let assign14590_e14605: f64 = (assign14590_e14588 + assign14590_e14604);
                        let assign14590_e14606: f64 = (0.5 * assign14590_e14605);
                        (assign14590_e14606, (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14600) + (assign14590_e14594 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14590_e14604)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14600) + (assign14590_e14594 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14590_e14604)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14590_e14608, assign14590_e14608_d_n2, assign14590_e14608_d_n3, assign14590_e14608_d_n4, assign14590_e14608_d_n7, assign14590_e14608_d_n9, assign14590_e14608_d_n10,)
            }
        };
        let assign14590_e14611: f64 = (assign14590_e14609).powf(locals.var_fn169_calc_iq__beta);
        let assign14590_e14612: f64 = (1.0 + assign14590_e14611);
        let assign14590_e14615: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14590_e14616: f64 = (assign14590_e14612).powf(assign14590_e14615);
        let assign14590_e14617: f64 = (1.0 / assign14590_e14616);
        (assign14590_e14617, (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n2)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n2 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n2)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n2 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n3)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n3 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n3)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n3 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n4)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n4 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n4)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n4 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n7)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n7 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n7)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n7 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n9)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n9 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n9)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n9 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n10)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n10 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n10)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n10 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))),)
    } else {
        (locals.var_fn169_calc_iq__fds, locals.var_fn169_calc_iq__fds_dn2, locals.var_fn169_calc_iq__fds_dn3, locals.var_fn169_calc_iq__fds_dn4, locals.var_fn169_calc_iq__fds_dn7, locals.var_fn169_calc_iq__fds_dn9, locals.var_fn169_calc_iq__fds_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds = assign14590_e14619;
        locals.var_fn169_calc_iq__fds_dn2 = assign14590_e14619_d_n2;
        locals.var_fn169_calc_iq__fds_dn3 = assign14590_e14619_d_n3;
        locals.var_fn169_calc_iq__fds_dn4 = assign14590_e14619_d_n4;
        locals.var_fn169_calc_iq__fds_dn7 = assign14590_e14619_d_n7;
        locals.var_fn169_calc_iq__fds_dn9 = assign14590_e14619_d_n9;
        locals.var_fn169_calc_iq__fds_dn10 = assign14590_e14619_d_n10;

        let (assign14600_e14626, assign14600_e14626_d_n2, assign14600_e14626_d_n3, assign14600_e14626_d_n4, assign14600_e14626_d_n7, assign14600_e14626_d_n9, assign14600_e14626_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14600_e14622: f64 = (-locals.var_fn169_calc_iq__vdsin);
        let assign14600_e14624: f64 = (assign14600_e14622 * locals.var_fn169_calc_iq__fds);
        (assign14600_e14624, (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn2), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn3), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn4), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn7), (((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__fds) + (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn9)), (((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__fds) + (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vsx, locals.var_fn169_calc_iq__vsx_dn2, locals.var_fn169_calc_iq__vsx_dn3, locals.var_fn169_calc_iq__vsx_dn4, locals.var_fn169_calc_iq__vsx_dn7, locals.var_fn169_calc_iq__vsx_dn9, locals.var_fn169_calc_iq__vsx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx = assign14600_e14626;
        locals.var_fn169_calc_iq__vsx_dn2 = assign14600_e14626_d_n2;
        locals.var_fn169_calc_iq__vsx_dn3 = assign14600_e14626_d_n3;
        locals.var_fn169_calc_iq__vsx_dn4 = assign14600_e14626_d_n4;
        locals.var_fn169_calc_iq__vsx_dn7 = assign14600_e14626_d_n7;
        locals.var_fn169_calc_iq__vsx_dn9 = assign14600_e14626_d_n9;
        locals.var_fn169_calc_iq__vsx_dn10 = assign14600_e14626_d_n10;

        let (assign14610_e14634, assign14610_e14634_d_n2, assign14610_e14634_d_n3, assign14610_e14634_d_n4, assign14610_e14634_d_n7, assign14610_e14634_d_n9, assign14610_e14634_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14610_e14630: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__myarg);
        let assign14610_e14632: f64 = (assign14610_e14630 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14610_e14632, ((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14610_e14630 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14610_e14634;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14610_e14634_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14610_e14634_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14610_e14634_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14610_e14634_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14610_e14634_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14610_e14634_d_n10;

        let assign14620_e14637: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign14620_e14637;

        let (assign14630_e14643, assign14630_e14643_d_n2, assign14630_e14643_d_n3, assign14630_e14643_d_n4, assign14630_e14643_d_n7, assign14630_e14643_d_n9, assign14630_e14643_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard175 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14630_e14643;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14630_e14643_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14630_e14643_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14630_e14643_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14630_e14643_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14630_e14643_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14630_e14643_d_n10;

        let assign14640_e14646: f64 = (-50.0);
        let assign14640_e14647: f64 = if locals.var_fn169_calc_iq__exparg < assign14640_e14646 { 1.0 } else { 0.0 };
        locals.var_guard176 = assign14640_e14647;

        let (assign14650_e14656, assign14650_e14656_d_n2, assign14650_e14656_d_n3, assign14650_e14656_d_n4, assign14650_e14656_d_n7, assign14650_e14656_d_n9, assign14650_e14656_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard175 == 0.0)) && (locals.var_guard176 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14650_e14656;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14650_e14656_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14650_e14656_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14650_e14656_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14650_e14656_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14650_e14656_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14650_e14656_d_n10;

        let (assign14660_e14671, assign14660_e14671_d_n2, assign14660_e14671_d_n3, assign14660_e14671_d_n4, assign14660_e14671_d_n7, assign14660_e14671_d_n9, assign14660_e14671_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard175 == 0.0)) && (locals.var_guard176 == 0.0)) {
        let assign14660_e14667: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14660_e14668: f64 = (1.0 + assign14660_e14667);
        let assign14660_e14669: f64 = (1.0 / assign14660_e14668);
        (assign14660_e14669, (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14660_e14668 * assign14660_e14668))),)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14660_e14671;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14660_e14671_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14660_e14671_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14660_e14671_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14660_e14671_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14660_e14671_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14660_e14671_d_n10;

        let (assign14670_e14689, assign14670_e14689_d_n2, assign14670_e14689_d_n3, assign14670_e14689_d_n4, assign14670_e14689_d_n7, assign14670_e14689_d_n9, assign14670_e14689_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14670_e14675: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__vsx);
        let assign14670_e14679: f64 = (p.p51 * 0.1);
        let assign14670_e14681: f64 = (assign14670_e14679 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14670_e14683: f64 = (assign14670_e14681 * locals.var_fn169_calc_iq__ffs);
        let assign14670_e14684: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14670_e14683);
        let assign14670_e14685: f64 = (assign14670_e14675 - assign14670_e14684);
        let assign14670_e14687: f64 = (assign14670_e14685 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14670_e14687, (((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__vsx_dn2) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn2))) / locals.var_fn169_calc_iq__two_n_phit), (((-locals.var_fn169_calc_iq__vsx_dn3) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn3))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vsx_dn4) - (locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14670_e14679 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffs) + (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__vsx_dn7) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn7))) / locals.var_fn169_calc_iq__two_n_phit), (((((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__vsx_dn9) - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__vsx_dn10) - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__etas, locals.var_fn169_calc_iq__etas_dn2, locals.var_fn169_calc_iq__etas_dn3, locals.var_fn169_calc_iq__etas_dn4, locals.var_fn169_calc_iq__etas_dn7, locals.var_fn169_calc_iq__etas_dn9, locals.var_fn169_calc_iq__etas_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas = assign14670_e14689;
        locals.var_fn169_calc_iq__etas_dn2 = assign14670_e14689_d_n2;
        locals.var_fn169_calc_iq__etas_dn3 = assign14670_e14689_d_n3;
        locals.var_fn169_calc_iq__etas_dn4 = assign14670_e14689_d_n4;
        locals.var_fn169_calc_iq__etas_dn7 = assign14670_e14689_d_n7;
        locals.var_fn169_calc_iq__etas_dn9 = assign14670_e14689_d_n9;
        locals.var_fn169_calc_iq__etas_dn10 = assign14670_e14689_d_n10;

        let assign14680_e14692: f64 = if locals.var_fn169_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign14680_e14692;

        let (assign14690_e14700, assign14690_e14700_d_n2, assign14690_e14700_d_n3, assign14690_e14700_d_n4, assign14690_e14700_d_n7, assign14690_e14700_d_n9, assign14690_e14700_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard177 != 0.0)) {
        let assign14690_e14698: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas);
        (assign14690_e14698, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14690_e14700;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14690_e14700_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14690_e14700_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14690_e14700_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14690_e14700_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14690_e14700_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14690_e14700_d_n10;

        let assign14700_e14703: f64 = (-50.0);
        let assign14700_e14704: f64 = if locals.var_fn169_calc_iq__etas < assign14700_e14703 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign14700_e14704;

        let (assign14710_e14716, assign14710_e14716_d_n2, assign14710_e14716_d_n3, assign14710_e14716_d_n4, assign14710_e14716_d_n7, assign14710_e14716_d_n9, assign14710_e14716_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard177 == 0.0)) && (locals.var_guard178 != 0.0)) {
        let assign14710_e14713: f64 = (locals.var_fn169_calc_iq__etas).exp();
        let assign14710_e14714: f64 = (locals.var_fn169_calc_iq__qref * assign14710_e14713);
        (assign14710_e14714, (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn2)), (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn4))), (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14710_e14716;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14710_e14716_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14710_e14716_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14710_e14716_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14710_e14716_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14710_e14716_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14710_e14716_d_n10;

        let (assign14720_e14732, assign14720_e14732_d_n2, assign14720_e14732_d_n3, assign14720_e14732_d_n4, assign14720_e14732_d_n7, assign14720_e14732_d_n9, assign14720_e14732_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard177 == 0.0)) && (locals.var_guard178 == 0.0)) {
        let assign14720_e14727: f64 = (locals.var_fn169_calc_iq__etas).exp();
        let assign14720_e14728: f64 = (1.0 + assign14720_e14727);
        let assign14720_e14729: f64 = (assign14720_e14728).ln();
        let assign14720_e14730: f64 = (locals.var_fn169_calc_iq__qref * assign14720_e14729);
        (assign14720_e14730, (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn2) / assign14720_e14728)), (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn3) / assign14720_e14728)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn4) / assign14720_e14728))), (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn7) / assign14720_e14728)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn9) / assign14720_e14728))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn10) / assign14720_e14728))),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14720_e14732;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14720_e14732_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14720_e14732_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14720_e14732_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14720_e14732_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14720_e14732_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14720_e14732_d_n10;

        let (assign14730_e14740, assign14730_e14740_d_n2, assign14730_e14740_d_n3, assign14730_e14740_d_n4, assign14730_e14740_d_n7, assign14730_e14740_d_n9, assign14730_e14740_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14730_e14736: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__myarg);
        let assign14730_e14738: f64 = (assign14730_e14736 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14730_e14738, ((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14730_e14736 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14730_e14740;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14730_e14740_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14730_e14740_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14730_e14740_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14730_e14740_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14730_e14740_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14730_e14740_d_n10;

        let assign14740_e14743: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard179 = assign14740_e14743;

        let (assign14750_e14749, assign14750_e14749_d_n2, assign14750_e14749_d_n3, assign14750_e14749_d_n4, assign14750_e14749_d_n7, assign14750_e14749_d_n9, assign14750_e14749_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard179 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14750_e14749;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14750_e14749_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14750_e14749_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14750_e14749_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14750_e14749_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14750_e14749_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14750_e14749_d_n10;

        let assign14760_e14752: f64 = (-50.0);
        let assign14760_e14753: f64 = if locals.var_fn169_calc_iq__exparg < assign14760_e14752 { 1.0 } else { 0.0 };
        locals.var_guard180 = assign14760_e14753;

        let (assign14770_e14762, assign14770_e14762_d_n2, assign14770_e14762_d_n3, assign14770_e14762_d_n4, assign14770_e14762_d_n7, assign14770_e14762_d_n9, assign14770_e14762_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard179 == 0.0)) && (locals.var_guard180 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14770_e14762;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14770_e14762_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14770_e14762_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14770_e14762_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14770_e14762_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14770_e14762_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14770_e14762_d_n10;

        let (assign14780_e14777, assign14780_e14777_d_n2, assign14780_e14777_d_n3, assign14780_e14777_d_n4, assign14780_e14777_d_n7, assign14780_e14777_d_n9, assign14780_e14777_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard179 == 0.0)) && (locals.var_guard180 == 0.0)) {
        let assign14780_e14773: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14780_e14774: f64 = (1.0 + assign14780_e14773);
        let assign14780_e14775: f64 = (1.0 / assign14780_e14774);
        (assign14780_e14775, (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14780_e14774 * assign14780_e14774))),)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14780_e14777;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14780_e14777_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14780_e14777_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14780_e14777_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14780_e14777_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14780_e14777_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14780_e14777_d_n10;

        let (assign14790_e14795, assign14790_e14795_d_n2, assign14790_e14795_d_n3, assign14790_e14795_d_n4, assign14790_e14795_d_n7, assign14790_e14795_d_n9, assign14790_e14795_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14790_e14781: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdx);
        let assign14790_e14785: f64 = (p.p51 * 0.1);
        let assign14790_e14787: f64 = (assign14790_e14785 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14790_e14789: f64 = (assign14790_e14787 * locals.var_fn169_calc_iq__ffd);
        let assign14790_e14790: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14790_e14789);
        let assign14790_e14791: f64 = (assign14790_e14781 - assign14790_e14790);
        let assign14790_e14793: f64 = (assign14790_e14791 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14790_e14793, (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vdx_dn2) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn2))) / locals.var_fn169_calc_iq__two_n_phit), (((-locals.var_fn169_calc_iq__vdx_dn3) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn3))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vdx_dn4) - (locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14790_e14785 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffd) + (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vdx_dn7) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn7))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vdx_dn9) - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdx_dn10) - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__etad, locals.var_fn169_calc_iq__etad_dn2, locals.var_fn169_calc_iq__etad_dn3, locals.var_fn169_calc_iq__etad_dn4, locals.var_fn169_calc_iq__etad_dn7, locals.var_fn169_calc_iq__etad_dn9, locals.var_fn169_calc_iq__etad_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad = assign14790_e14795;
        locals.var_fn169_calc_iq__etad_dn2 = assign14790_e14795_d_n2;
        locals.var_fn169_calc_iq__etad_dn3 = assign14790_e14795_d_n3;
        locals.var_fn169_calc_iq__etad_dn4 = assign14790_e14795_d_n4;
        locals.var_fn169_calc_iq__etad_dn7 = assign14790_e14795_d_n7;
        locals.var_fn169_calc_iq__etad_dn9 = assign14790_e14795_d_n9;
        locals.var_fn169_calc_iq__etad_dn10 = assign14790_e14795_d_n10;

        let assign14800_e14798: f64 = if locals.var_fn169_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign14800_e14798;

        let (assign14810_e14806, assign14810_e14806_d_n2, assign14810_e14806_d_n3, assign14810_e14806_d_n4, assign14810_e14806_d_n7, assign14810_e14806_d_n9, assign14810_e14806_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard181 != 0.0)) {
        let assign14810_e14804: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad);
        (assign14810_e14804, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14810_e14806;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14810_e14806_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14810_e14806_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14810_e14806_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14810_e14806_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14810_e14806_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14810_e14806_d_n10;

        let assign14820_e14809: f64 = (-50.0);
        let assign14820_e14810: f64 = if locals.var_fn169_calc_iq__etad < assign14820_e14809 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign14820_e14810;

        let (assign14830_e14822, assign14830_e14822_d_n2, assign14830_e14822_d_n3, assign14830_e14822_d_n4, assign14830_e14822_d_n7, assign14830_e14822_d_n9, assign14830_e14822_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard181 == 0.0)) && (locals.var_guard182 != 0.0)) {
        let assign14830_e14819: f64 = (locals.var_fn169_calc_iq__etad).exp();
        let assign14830_e14820: f64 = (locals.var_fn169_calc_iq__qref * assign14830_e14819);
        (assign14830_e14820, (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn2)), (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn4))), (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14830_e14822;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14830_e14822_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14830_e14822_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14830_e14822_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14830_e14822_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14830_e14822_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14830_e14822_d_n10;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14840_e14838, assign14840_e14838_d_n2, assign14840_e14838_d_n3, assign14840_e14838_d_n4, assign14840_e14838_d_n7, assign14840_e14838_d_n9, assign14840_e14838_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard181 == 0.0)) && (locals.var_guard182 == 0.0)) {
        let assign14840_e14833: f64 = (locals.var_fn169_calc_iq__etad).exp();
        let assign14840_e14834: f64 = (1.0 + assign14840_e14833);
        let assign14840_e14835: f64 = (assign14840_e14834).ln();
        let assign14840_e14836: f64 = (locals.var_fn169_calc_iq__qref * assign14840_e14835);
        (assign14840_e14836, (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn2) / assign14840_e14834)), (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn3) / assign14840_e14834)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn4) / assign14840_e14834))), (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn7) / assign14840_e14834)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn9) / assign14840_e14834))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn10) / assign14840_e14834))),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14840_e14838;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14840_e14838_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14840_e14838_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14840_e14838_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14840_e14838_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14840_e14838_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14840_e14838_d_n10;

        let (assign14850_e14846, assign14850_e14846_d_n2, assign14850_e14846_d_n3, assign14850_e14846_d_n4, assign14850_e14846_d_n7, assign14850_e14846_d_n9, assign14850_e14846_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14850_e14842: f64 = (locals.var_fn169_calc_iq__qinvs - locals.var_fn169_calc_iq__qinvd);
        let assign14850_e14844: f64 = (assign14850_e14842 / locals.var_fn169_calc_iq__cgin);
        (assign14850_e14844, ((locals.var_fn169_calc_iq__qinvs_dn2 - locals.var_fn169_calc_iq__qinvd_dn2) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn3 - locals.var_fn169_calc_iq__qinvd_dn3) / locals.var_fn169_calc_iq__cgin), ((((locals.var_fn169_calc_iq__qinvs_dn4 - locals.var_fn169_calc_iq__qinvd_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14850_e14842 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)), ((locals.var_fn169_calc_iq__qinvs_dn7 - locals.var_fn169_calc_iq__qinvd_dn7) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn9 - locals.var_fn169_calc_iq__qinvd_dn9) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn10 - locals.var_fn169_calc_iq__qinvd_dn10) / locals.var_fn169_calc_iq__cgin),)
    } else {
        (locals.var_fn169_calc_iq__vdsc, locals.var_fn169_calc_iq__vdsc_dn2, locals.var_fn169_calc_iq__vdsc_dn3, locals.var_fn169_calc_iq__vdsc_dn4, locals.var_fn169_calc_iq__vdsc_dn7, locals.var_fn169_calc_iq__vdsc_dn9, locals.var_fn169_calc_iq__vdsc_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsc = assign14850_e14846;
        locals.var_fn169_calc_iq__vdsc_dn2 = assign14850_e14846_d_n2;
        locals.var_fn169_calc_iq__vdsc_dn3 = assign14850_e14846_d_n3;
        locals.var_fn169_calc_iq__vdsc_dn4 = assign14850_e14846_d_n4;
        locals.var_fn169_calc_iq__vdsc_dn7 = assign14850_e14846_d_n7;
        locals.var_fn169_calc_iq__vdsc_dn9 = assign14850_e14846_d_n9;
        locals.var_fn169_calc_iq__vdsc_dn10 = assign14850_e14846_d_n10;

        let (assign14860_e14852, assign14860_e14852_d_n2, assign14860_e14852_d_n3, assign14860_e14852_d_n4, assign14860_e14852_d_n7, assign14860_e14852_d_n9, assign14860_e14852_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14860_e14850: f64 = (locals.var_fn169_calc_iq__vdsc / locals.var_fn169_calc_iq__vdsat);
        (assign14860_e14850, (((locals.var_fn169_calc_iq__vdsc_dn2 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn2)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn3 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn3)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn4 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn4)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn7 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn7)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn9 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn9)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn10 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn10)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)),)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14860_e14852;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14860_e14852_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14860_e14852_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14860_e14852_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14860_e14852_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14860_e14852_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14860_e14852_d_n10;

        let (assign14870_e14889, assign14870_e14889_d_n2, assign14870_e14889_d_n3, assign14870_e14889_d_n4, assign14870_e14889_d_n7, assign14870_e14889_d_n9, assign14870_e14889_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14870_e14879, assign14870_e14879_d_n2, assign14870_e14879_d_n3, assign14870_e14879_d_n4, assign14870_e14879_d_n7, assign14870_e14879_d_n9, assign14870_e14879_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14870_e14863: f64 = (0.001 / p.p53);
                let assign14870_e14865: f64 = (assign14870_e14863 * locals.var_fn169_calc_iq__myarg);
                let assign14870_e14866: f64 = (assign14870_e14865).tanh();
                let assign14870_e14867: f64 = (locals.var_fn169_calc_iq__myarg * assign14870_e14866);
                (assign14870_e14867, ((locals.var_fn169_calc_iq__myarg_dn2 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn2) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))), ((locals.var_fn169_calc_iq__myarg_dn3 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn3) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))), ((locals.var_fn169_calc_iq__myarg_dn4 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn4) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))), ((locals.var_fn169_calc_iq__myarg_dn7 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn7) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))), ((locals.var_fn169_calc_iq__myarg_dn9 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn9) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))), ((locals.var_fn169_calc_iq__myarg_dn10 * assign14870_e14866) + (locals.var_fn169_calc_iq__myarg * ((assign14870_e14863 * locals.var_fn169_calc_iq__myarg_dn10) / ((assign14870_e14865).cosh() * (assign14870_e14865).cosh())))),)
            } else {
                let (assign14870_e14878, assign14870_e14878_d_n2, assign14870_e14878_d_n3, assign14870_e14878_d_n4, assign14870_e14878_d_n7, assign14870_e14878_d_n9, assign14870_e14878_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14870_e14873: f64 = (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg);
                        let assign14870_e14875: f64 = (assign14870_e14873 + p.p53);
                        let assign14870_e14876: f64 = (assign14870_e14875).sqrt();
                        (assign14870_e14876, (((locals.var_fn169_calc_iq__myarg_dn2 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn2)) / (2.0 * assign14870_e14876)), (((locals.var_fn169_calc_iq__myarg_dn3 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn3)) / (2.0 * assign14870_e14876)), (((locals.var_fn169_calc_iq__myarg_dn4 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn4)) / (2.0 * assign14870_e14876)), (((locals.var_fn169_calc_iq__myarg_dn7 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn7)) / (2.0 * assign14870_e14876)), (((locals.var_fn169_calc_iq__myarg_dn9 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn9)) / (2.0 * assign14870_e14876)), (((locals.var_fn169_calc_iq__myarg_dn10 * locals.var_fn169_calc_iq__myarg) + (locals.var_fn169_calc_iq__myarg * locals.var_fn169_calc_iq__myarg_dn10)) / (2.0 * assign14870_e14876)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14870_e14878, assign14870_e14878_d_n2, assign14870_e14878_d_n3, assign14870_e14878_d_n4, assign14870_e14878_d_n7, assign14870_e14878_d_n9, assign14870_e14878_d_n10,)
            }
        };
        let assign14870_e14881: f64 = (assign14870_e14879).powf(locals.var_fn169_calc_iq__beta);
        let assign14870_e14882: f64 = (1.0 + assign14870_e14881);
        let assign14870_e14885: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14870_e14886: f64 = (assign14870_e14882).powf(assign14870_e14885);
        let assign14870_e14887: f64 = (locals.var_fn169_calc_iq__myarg / assign14870_e14886);
        (assign14870_e14887, (((locals.var_fn169_calc_iq__myarg_dn2 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n2)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n2 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n2)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n2 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)), (((locals.var_fn169_calc_iq__myarg_dn3 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n3)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n3 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n3)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n3 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)), (((locals.var_fn169_calc_iq__myarg_dn4 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n4)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n4 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n4)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n4 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)), (((locals.var_fn169_calc_iq__myarg_dn7 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n7)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n7 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n7)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n7 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)), (((locals.var_fn169_calc_iq__myarg_dn9 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n9)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n9 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n9)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n9 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)), (((locals.var_fn169_calc_iq__myarg_dn10 * assign14870_e14886) - (locals.var_fn169_calc_iq__myarg * if 0.0 == 0.0 && ((assign14870_e14885) as f64).is_finite() && ((assign14870_e14885) as f64).fract() == 0.0 { if assign14870_e14885 == 0.0 { 0.0 } else { (assign14870_e14885 * ((assign14870_e14882).powf(assign14870_e14885 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n10)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n10 / assign14870_e14879))) })) } } else { (assign14870_e14886 * (assign14870_e14885 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14870_e14879).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14870_e14879_d_n10)) } } else { (assign14870_e14881 * (locals.var_fn169_calc_iq__beta * (assign14870_e14879_d_n10 / assign14870_e14879))) } / assign14870_e14882))) })) / (assign14870_e14886 * assign14870_e14886)),)
    } else {
        (locals.var_fn169_calc_iq__fsat, locals.var_fn169_calc_iq__fsat_dn2, locals.var_fn169_calc_iq__fsat_dn3, locals.var_fn169_calc_iq__fsat_dn4, locals.var_fn169_calc_iq__fsat_dn7, locals.var_fn169_calc_iq__fsat_dn9, locals.var_fn169_calc_iq__fsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsat = assign14870_e14889;
        locals.var_fn169_calc_iq__fsat_dn2 = assign14870_e14889_d_n2;
        locals.var_fn169_calc_iq__fsat_dn3 = assign14870_e14889_d_n3;
        locals.var_fn169_calc_iq__fsat_dn4 = assign14870_e14889_d_n4;
        locals.var_fn169_calc_iq__fsat_dn7 = assign14870_e14889_d_n7;
        locals.var_fn169_calc_iq__fsat_dn9 = assign14870_e14889_d_n9;
        locals.var_fn169_calc_iq__fsat_dn10 = assign14870_e14889_d_n10;

        let (assign14880_e14895, assign14880_e14895_d_n2, assign14880_e14895_d_n3, assign14880_e14895_d_n4, assign14880_e14895_d_n7, assign14880_e14895_d_n9, assign14880_e14895_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14880_e14893: f64 = (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat);
        (assign14880_e14893, ((locals.var_fn169_calc_iq__vxf_dn2 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn2)), ((locals.var_fn169_calc_iq__vxf_dn3 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn3)), ((locals.var_fn169_calc_iq__vxf_dn4 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn4)), ((locals.var_fn169_calc_iq__vxf_dn7 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn7)), ((locals.var_fn169_calc_iq__vxf_dn9 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn9)), ((locals.var_fn169_calc_iq__vxf_dn10 * locals.var_fn169_calc_iq__fsat) + (locals.var_fn169_calc_iq__vxf * locals.var_fn169_calc_iq__fsat_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vel, locals.var_fn169_calc_iq__vel_dn2, locals.var_fn169_calc_iq__vel_dn3, locals.var_fn169_calc_iq__vel_dn4, locals.var_fn169_calc_iq__vel_dn7, locals.var_fn169_calc_iq__vel_dn9, locals.var_fn169_calc_iq__vel_dn10,)
    }
};
        locals.var_fn169_calc_iq__vel = assign14880_e14895;
        locals.var_fn169_calc_iq__vel_dn2 = assign14880_e14895_d_n2;
        locals.var_fn169_calc_iq__vel_dn3 = assign14880_e14895_d_n3;
        locals.var_fn169_calc_iq__vel_dn4 = assign14880_e14895_d_n4;
        locals.var_fn169_calc_iq__vel_dn7 = assign14880_e14895_d_n7;
        locals.var_fn169_calc_iq__vel_dn9 = assign14880_e14895_d_n9;
        locals.var_fn169_calc_iq__vel_dn10 = assign14880_e14895_d_n10;

        let (assign14890_e14913, assign14890_e14913_d_n2, assign14890_e14913_d_n3, assign14890_e14913_d_n4, assign14890_e14913_d_n7, assign14890_e14913_d_n9, assign14890_e14913_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14890_e14899: f64 = (locals.var_fn169_calc_iq__type * locals.var_fn169_calc_iq__w);
        let assign14890_e14901: f64 = (assign14890_e14899 * locals.var_fn169_calc_iq__ngf);
        let assign14890_e14903: f64 = (assign14890_e14901 * 0.5);
        let assign14890_e14906: f64 = (locals.var_fn169_calc_iq__qinvs + locals.var_fn169_calc_iq__qinvd);
        let assign14890_e14907: f64 = (assign14890_e14903 * assign14890_e14906);
        let assign14890_e14909: f64 = (assign14890_e14907 * locals.var_fn169_calc_iq__vel);
        let assign14890_e14911: f64 = (assign14890_e14909 * locals.var_fn169_calc_iq__trapfracdl);
        (assign14890_e14911, ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn2 + locals.var_fn169_calc_iq__qinvd_dn2)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn2)) * locals.var_fn169_calc_iq__trapfracdl), ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn3 + locals.var_fn169_calc_iq__qinvd_dn3)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn3)) * locals.var_fn169_calc_iq__trapfracdl), ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn4 + locals.var_fn169_calc_iq__qinvd_dn4)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn7 + locals.var_fn169_calc_iq__qinvd_dn7)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn7)) * locals.var_fn169_calc_iq__trapfracdl), ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn9 + locals.var_fn169_calc_iq__qinvd_dn9)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn9)) * locals.var_fn169_calc_iq__trapfracdl), ((((assign14890_e14903 * (locals.var_fn169_calc_iq__qinvs_dn10 + locals.var_fn169_calc_iq__qinvd_dn10)) * locals.var_fn169_calc_iq__vel) + (assign14890_e14907 * locals.var_fn169_calc_iq__vel_dn10)) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__idsout, locals.var_fn169_calc_iq__idsout_dn2, locals.var_fn169_calc_iq__idsout_dn3, locals.var_fn169_calc_iq__idsout_dn4, locals.var_fn169_calc_iq__idsout_dn7, locals.var_fn169_calc_iq__idsout_dn9, locals.var_fn169_calc_iq__idsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__idsout = assign14890_e14913;
        locals.var_fn169_calc_iq__idsout_dn2 = assign14890_e14913_d_n2;
        locals.var_fn169_calc_iq__idsout_dn3 = assign14890_e14913_d_n3;
        locals.var_fn169_calc_iq__idsout_dn4 = assign14890_e14913_d_n4;
        locals.var_fn169_calc_iq__idsout_dn7 = assign14890_e14913_d_n7;
        locals.var_fn169_calc_iq__idsout_dn9 = assign14890_e14913_d_n9;
        locals.var_fn169_calc_iq__idsout_dn10 = assign14890_e14913_d_n10;

        let (assign14900_e14921, assign14900_e14921_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14900_e14918: f64 = (2.302585092994046 * locals.var_fn169_calc_iq__phitin);
        let assign14900_e14919: f64 = (locals.var_fn169_calc_iq__ss / assign14900_e14918);
        (assign14900_e14919, (-((locals.var_fn169_calc_iq__ss * (2.302585092994046 * locals.var_fn169_calc_iq__phitin_dn4)) / (assign14900_e14918 * assign14900_e14918))),)
    } else {
        (locals.var_fn169_calc_iq__n0, locals.var_fn169_calc_iq__n0_dn4,)
    }
};
        locals.var_fn169_calc_iq__n0 = assign14900_e14921;
        locals.var_fn169_calc_iq__n0_dn4 = assign14900_e14921_d_n4;

        let (assign14910_e14929, assign14910_e14929_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14910_e14925: f64 = (2.0 * locals.var_fn169_calc_iq__n0);
        let assign14910_e14927: f64 = (assign14910_e14925 * locals.var_fn169_calc_iq__phitin);
        (assign14910_e14927, (((2.0 * locals.var_fn169_calc_iq__n0_dn4) * locals.var_fn169_calc_iq__phitin) + (assign14910_e14925 * locals.var_fn169_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit0, locals.var_fn169_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit0 = assign14910_e14929;
        locals.var_fn169_calc_iq__two_n_phit0_dn4 = assign14910_e14929_d_n4;

        let (assign14920_e14935, assign14920_e14935_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14920_e14933: f64 = (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit0);
        (assign14920_e14933, ((locals.var_fn169_calc_iq__cgin_dn4 * locals.var_fn169_calc_iq__two_n_phit0) + (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn169_calc_iq__qref0, locals.var_fn169_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn169_calc_iq__qref0 = assign14920_e14935;
        locals.var_fn169_calc_iq__qref0_dn4 = assign14920_e14935_d_n4;

        let (assign14930_e14945, assign14930_e14945_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14930_e14940: f64 = (p.p51 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14930_e14942: f64 = (assign14930_e14940 / 2.0);
        let assign14930_e14943: f64 = (locals.var_fn169_calc_iq__vtof - assign14930_e14942);
        (assign14930_e14943, (locals.var_fn169_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn169_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn169_calc_iq__myarg0, locals.var_fn169_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn169_calc_iq__myarg0 = assign14930_e14945;
        locals.var_fn169_calc_iq__myarg0_dn4 = assign14930_e14945_d_n4;

        let (assign14940_e14996, assign14940_e14996_d_n2, assign14940_e14996_d_n4, assign14940_e14996_d_n7, assign14940_e14996_d_n9, assign14940_e14996_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14940_e14990, assign14940_e14990_d_n2, assign14940_e14990_d_n7, assign14940_e14990_d_n9, assign14940_e14990_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14940_e14954: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14957: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14960: f64 = (0.001 / p.p53);
                let assign14940_e14963: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14964: f64 = (assign14940_e14960 * assign14940_e14963);
                let assign14940_e14965: f64 = (assign14940_e14964).tanh();
                let assign14940_e14966: f64 = (assign14940_e14957 * assign14940_e14965);
                let assign14940_e14967: f64 = (assign14940_e14954 + assign14940_e14966);
                let assign14940_e14968: f64 = (0.5 * assign14940_e14967);
                (assign14940_e14968, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))),)
            } else {
                let (assign14940_e14989, assign14940_e14989_d_n2, assign14940_e14989_d_n7, assign14940_e14989_d_n9, assign14940_e14989_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14940_e14975: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14978: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14981: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14982: f64 = (assign14940_e14978 * assign14940_e14981);
                        let assign14940_e14984: f64 = (assign14940_e14982 + p.p53);
                        let assign14940_e14985: f64 = (assign14940_e14984).sqrt();
                        let assign14940_e14986: f64 = (assign14940_e14975 + assign14940_e14985);
                        let assign14940_e14987: f64 = (0.5 * assign14940_e14986);
                        (assign14940_e14987, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14940_e14985)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14940_e14985)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14940_e14981) + (assign14940_e14978 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14940_e14985)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14940_e14985)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14940_e14989, assign14940_e14989_d_n2, assign14940_e14989_d_n7, assign14940_e14989_d_n9, assign14940_e14989_d_n10,)
            }
        };
        let assign14940_e14992: f64 = (assign14940_e14990 - locals.var_fn169_calc_iq__myarg0);
        let assign14940_e14994: f64 = (assign14940_e14992 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14940_e14994, (assign14940_e14990_d_n2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14940_e14992 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (assign14940_e14990_d_n7 / locals.var_fn169_calc_iq__alpha_phit), (assign14940_e14990_d_n9 / locals.var_fn169_calc_iq__alpha_phit), (assign14940_e14990_d_n10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign14940_e14996;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign14940_e14996_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign14940_e14996_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign14940_e14996_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign14940_e14996_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign14940_e14996_d_n10;

        let assign14950_e14999: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign14950_e14999;

        let (assign14960_e15005, assign14960_e15005_d_n2, assign14960_e15005_d_n4, assign14960_e15005_d_n7, assign14960_e15005_d_n9, assign14960_e15005_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard183 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14960_e15005;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14960_e15005_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14960_e15005_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14960_e15005_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14960_e15005_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14960_e15005_d_n10;

        let assign14970_e15008: f64 = (-50.0);
        let assign14970_e15009: f64 = if locals.var_fn169_calc_iq__exparg0 < assign14970_e15008 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign14970_e15009;

        let (assign14980_e15018, assign14980_e15018_d_n2, assign14980_e15018_d_n4, assign14980_e15018_d_n7, assign14980_e15018_d_n9, assign14980_e15018_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard183 == 0.0)) && (locals.var_guard184 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14980_e15018;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14980_e15018_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14980_e15018_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14980_e15018_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14980_e15018_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14980_e15018_d_n10;

        let (assign14990_e15033, assign14990_e15033_d_n2, assign14990_e15033_d_n4, assign14990_e15033_d_n7, assign14990_e15033_d_n9, assign14990_e15033_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard183 == 0.0)) && (locals.var_guard184 == 0.0)) {
        let assign14990_e15029: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign14990_e15030: f64 = (1.0 + assign14990_e15029);
        let assign14990_e15031: f64 = (1.0 / assign14990_e15030);
        (assign14990_e15031, (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign14990_e15030 * assign14990_e15030))),)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14990_e15033;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14990_e15033_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14990_e15033_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14990_e15033_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14990_e15033_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14990_e15033_d_n10;

        let (assign15000_e15092, assign15000_e15092_d_n2, assign15000_e15092_d_n4, assign15000_e15092_d_n7, assign15000_e15092_d_n9, assign15000_e15092_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15000_e15078, assign15000_e15078_d_n2, assign15000_e15078_d_n7, assign15000_e15078_d_n9, assign15000_e15078_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15000_e15042: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15045: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15048: f64 = (0.001 / p.p53);
                let assign15000_e15051: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15052: f64 = (assign15000_e15048 * assign15000_e15051);
                let assign15000_e15053: f64 = (assign15000_e15052).tanh();
                let assign15000_e15054: f64 = (assign15000_e15045 * assign15000_e15053);
                let assign15000_e15055: f64 = (assign15000_e15042 + assign15000_e15054);
                let assign15000_e15056: f64 = (0.5 * assign15000_e15055);
                (assign15000_e15056, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))),)
            } else {
                let (assign15000_e15077, assign15000_e15077_d_n2, assign15000_e15077_d_n7, assign15000_e15077_d_n9, assign15000_e15077_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15000_e15063: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15066: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15069: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15070: f64 = (assign15000_e15066 * assign15000_e15069);
                        let assign15000_e15072: f64 = (assign15000_e15070 + p.p53);
                        let assign15000_e15073: f64 = (assign15000_e15072).sqrt();
                        let assign15000_e15074: f64 = (assign15000_e15063 + assign15000_e15073);
                        let assign15000_e15075: f64 = (0.5 * assign15000_e15074);
                        (assign15000_e15075, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign15000_e15073)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign15000_e15073)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign15000_e15069) + (assign15000_e15066 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign15000_e15073)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign15000_e15073)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15000_e15077, assign15000_e15077_d_n2, assign15000_e15077_d_n7, assign15000_e15077_d_n9, assign15000_e15077_d_n10,)
            }
        };
        let assign15000_e15082: f64 = (p.p51 * 0.1);
        let assign15000_e15084: f64 = (assign15000_e15082 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15000_e15086: f64 = (assign15000_e15084 * locals.var_fn169_calc_iq__ff0);
        let assign15000_e15087: f64 = (locals.var_fn169_calc_iq__vtof - assign15000_e15086);
        let assign15000_e15088: f64 = (assign15000_e15078 - assign15000_e15087);
        let assign15000_e15090: f64 = (assign15000_e15088 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15000_e15090, ((assign15000_e15078_d_n2 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (((assign15000_e15082 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ff0) + (assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15000_e15088 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), ((assign15000_e15078_d_n7 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), ((assign15000_e15078_d_n9 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), ((assign15000_e15078_d_n10 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__eta0, locals.var_fn169_calc_iq__eta0_dn2, locals.var_fn169_calc_iq__eta0_dn4, locals.var_fn169_calc_iq__eta0_dn7, locals.var_fn169_calc_iq__eta0_dn9, locals.var_fn169_calc_iq__eta0_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta0 = assign15000_e15092;
        locals.var_fn169_calc_iq__eta0_dn2 = assign15000_e15092_d_n2;
        locals.var_fn169_calc_iq__eta0_dn4 = assign15000_e15092_d_n4;
        locals.var_fn169_calc_iq__eta0_dn7 = assign15000_e15092_d_n7;
        locals.var_fn169_calc_iq__eta0_dn9 = assign15000_e15092_d_n9;
        locals.var_fn169_calc_iq__eta0_dn10 = assign15000_e15092_d_n10;

        let assign15010_e15095: f64 = if locals.var_fn169_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign15010_e15095;

        let (assign15020_e15103, assign15020_e15103_d_n2, assign15020_e15103_d_n4, assign15020_e15103_d_n7, assign15020_e15103_d_n9, assign15020_e15103_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard185 != 0.0)) {
        let assign15020_e15101: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0);
        (assign15020_e15101, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__eta0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15020_e15103;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15020_e15103_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15020_e15103_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15020_e15103_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15020_e15103_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15020_e15103_d_n10;

        let assign15030_e15106: f64 = (-50.0);
        let assign15030_e15107: f64 = if locals.var_fn169_calc_iq__eta0 < assign15030_e15106 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign15030_e15107;

        let (assign15040_e15119, assign15040_e15119_d_n2, assign15040_e15119_d_n4, assign15040_e15119_d_n7, assign15040_e15119_d_n9, assign15040_e15119_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard185 == 0.0)) && (locals.var_guard186 != 0.0)) {
        let assign15040_e15116: f64 = (locals.var_fn169_calc_iq__eta0).exp();
        let assign15040_e15117: f64 = (locals.var_fn169_calc_iq__qref0 * assign15040_e15116);
        (assign15040_e15117, (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15040_e15116) + (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15040_e15119;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15040_e15119_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15040_e15119_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15040_e15119_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15040_e15119_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15040_e15119_d_n10;

        let (assign15050_e15135, assign15050_e15135_d_n2, assign15050_e15135_d_n4, assign15050_e15135_d_n7, assign15050_e15135_d_n9, assign15050_e15135_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard185 == 0.0)) && (locals.var_guard186 == 0.0)) {
        let assign15050_e15130: f64 = (locals.var_fn169_calc_iq__eta0).exp();
        let assign15050_e15131: f64 = (1.0 + assign15050_e15130);
        let assign15050_e15132: f64 = (assign15050_e15131).ln();
        let assign15050_e15133: f64 = (locals.var_fn169_calc_iq__qref0 * assign15050_e15132);
        (assign15050_e15133, (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn2) / assign15050_e15131)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15050_e15132) + (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn4) / assign15050_e15131))), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn7) / assign15050_e15131)), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn9) / assign15050_e15131)), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn10) / assign15050_e15131)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15050_e15135;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15050_e15135_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15050_e15135_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15050_e15135_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15050_e15135_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15050_e15135_d_n10;

        let (assign15060_e15141, assign15060_e15141_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15060_e15139: f64 = (locals.var_fn169_calc_iq__mu0 / locals.var_fn169_calc_iq__tfacmobin);
        (assign15060_e15139, (-((locals.var_fn169_calc_iq__mu0 * locals.var_fn169_calc_iq__tfacmobin_dn4) / (locals.var_fn169_calc_iq__tfacmobin * locals.var_fn169_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn169_calc_iq__muf0, locals.var_fn169_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn169_calc_iq__muf0 = assign15060_e15141;
        locals.var_fn169_calc_iq__muf0_dn4 = assign15060_e15141_d_n4;

        let (assign15070_e15157, assign15070_e15157_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15070_e15147: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tnomin);
        let assign15070_e15148: f64 = (1.0 + assign15070_e15147);
        let assign15070_e15152: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin);
        let assign15070_e15153: f64 = (1.0 + assign15070_e15152);
        let assign15070_e15154: f64 = (assign15070_e15148 / assign15070_e15153);
        let assign15070_e15155: f64 = (locals.var_fn169_calc_iq__vel0 * assign15070_e15154);
        (assign15070_e15155, (locals.var_fn169_calc_iq__vel0 * (-((assign15070_e15148 * (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin_dn4)) / (assign15070_e15153 * assign15070_e15153)))),)
    } else {
        (locals.var_fn169_calc_iq__vx0, locals.var_fn169_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vx0 = assign15070_e15157;
        locals.var_fn169_calc_iq__vx0_dn4 = assign15070_e15157_d_n4;

        let (assign15080_e15165, assign15080_e15165_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15080_e15161: f64 = (locals.var_fn169_calc_iq__vx0 * locals.var_fn169_calc_iq__lin);
        let assign15080_e15163: f64 = (assign15080_e15161 / locals.var_fn169_calc_iq__muf0);
        (assign15080_e15163, ((((locals.var_fn169_calc_iq__vx0_dn4 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf0) - (assign15080_e15161 * locals.var_fn169_calc_iq__muf0_dn4)) / (locals.var_fn169_calc_iq__muf0 * locals.var_fn169_calc_iq__muf0)),)
    } else {
        (locals.var_fn169_calc_iq__vdsats0, locals.var_fn169_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vdsats0 = assign15080_e15165;
        locals.var_fn169_calc_iq__vdsats0_dn4 = assign15080_e15165_d_n4;

        let (assign15090_e15182, assign15090_e15182_d_n2, assign15090_e15182_d_n4, assign15090_e15182_d_n7, assign15090_e15182_d_n9, assign15090_e15182_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15090_e15171: f64 = (2.0 * locals.var_fn169_calc_iq__qinvv0);
        let assign15090_e15173: f64 = (assign15090_e15171 / locals.var_fn169_calc_iq__cgin);
        let assign15090_e15175: f64 = (assign15090_e15173 / locals.var_fn169_calc_iq__vdsats0);
        let assign15090_e15176: f64 = (1.0 + assign15090_e15175);
        let assign15090_e15177: f64 = (assign15090_e15176).sqrt();
        let assign15090_e15178: f64 = (locals.var_fn169_calc_iq__vdsats0 * assign15090_e15177);
        let assign15090_e15180: f64 = (assign15090_e15178 - locals.var_fn169_calc_iq__vdsats0);
        (assign15090_e15180, (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn2) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (((locals.var_fn169_calc_iq__vdsats0_dn4 * assign15090_e15177) + (locals.var_fn169_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn4) * locals.var_fn169_calc_iq__cgin) - (assign15090_e15171 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)) * locals.var_fn169_calc_iq__vdsats0) - (assign15090_e15173 * locals.var_fn169_calc_iq__vdsats0_dn4)) / (locals.var_fn169_calc_iq__vdsats0 * locals.var_fn169_calc_iq__vdsats0)) / (2.0 * assign15090_e15177)))) - locals.var_fn169_calc_iq__vdsats0_dn4), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn7) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn9) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn10) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))),)
    } else {
        (locals.var_fn169_calc_iq__vdsats10, locals.var_fn169_calc_iq__vdsats10_dn2, locals.var_fn169_calc_iq__vdsats10_dn4, locals.var_fn169_calc_iq__vdsats10_dn7, locals.var_fn169_calc_iq__vdsats10_dn9, locals.var_fn169_calc_iq__vdsats10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats10 = assign15090_e15182;
        locals.var_fn169_calc_iq__vdsats10_dn2 = assign15090_e15182_d_n2;
        locals.var_fn169_calc_iq__vdsats10_dn4 = assign15090_e15182_d_n4;
        locals.var_fn169_calc_iq__vdsats10_dn7 = assign15090_e15182_d_n7;
        locals.var_fn169_calc_iq__vdsats10_dn9 = assign15090_e15182_d_n9;
        locals.var_fn169_calc_iq__vdsats10_dn10 = assign15090_e15182_d_n10;

        let (assign15100_e15194, assign15100_e15194_d_n2, assign15100_e15194_d_n4, assign15100_e15194_d_n7, assign15100_e15194_d_n9, assign15100_e15194_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15100_e15187: f64 = (1.0 - locals.var_fn169_calc_iq__ff0);
        let assign15100_e15188: f64 = (locals.var_fn169_calc_iq__vdsats10 * assign15100_e15187);
        let assign15100_e15191: f64 = (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0);
        let assign15100_e15192: f64 = (assign15100_e15188 + assign15100_e15191);
        (assign15100_e15192, (((locals.var_fn169_calc_iq__vdsats10_dn2 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn2))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn2)), (((locals.var_fn169_calc_iq__vdsats10_dn4 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit0_dn4 * locals.var_fn169_calc_iq__ff0) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn4))), (((locals.var_fn169_calc_iq__vdsats10_dn7 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn7))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn7)), (((locals.var_fn169_calc_iq__vdsats10_dn9 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn9))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn9)), (((locals.var_fn169_calc_iq__vdsats10_dn10 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn10))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdsat10, locals.var_fn169_calc_iq__vdsat10_dn2, locals.var_fn169_calc_iq__vdsat10_dn4, locals.var_fn169_calc_iq__vdsat10_dn7, locals.var_fn169_calc_iq__vdsat10_dn9, locals.var_fn169_calc_iq__vdsat10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat10 = assign15100_e15194;
        locals.var_fn169_calc_iq__vdsat10_dn2 = assign15100_e15194_d_n2;
        locals.var_fn169_calc_iq__vdsat10_dn4 = assign15100_e15194_d_n4;
        locals.var_fn169_calc_iq__vdsat10_dn7 = assign15100_e15194_d_n7;
        locals.var_fn169_calc_iq__vdsat10_dn9 = assign15100_e15194_d_n9;
        locals.var_fn169_calc_iq__vdsat10_dn10 = assign15100_e15194_d_n10;

    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15110_e15263, assign15110_e15263_d_n2, assign15110_e15263_d_n4, assign15110_e15263_d_n7, assign15110_e15263_d_n9, assign15110_e15263_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15110_e15253, assign15110_e15253_d_n2, assign15110_e15253_d_n4, assign15110_e15253_d_n7, assign15110_e15253_d_n9, assign15110_e15253_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15110_e15206: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15207: f64 = assign15110_e15206;
                let assign15110_e15211: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15212: f64 = (-assign15110_e15211);
                let assign15110_e15215: f64 = (0.001 / p.p53);
                let assign15110_e15219: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15220: f64 = (-assign15110_e15219);
                let assign15110_e15221: f64 = (assign15110_e15215 * assign15110_e15220);
                let assign15110_e15222: f64 = (assign15110_e15221).tanh();
                let assign15110_e15223: f64 = (assign15110_e15212 * assign15110_e15222);
                let assign15110_e15224: f64 = (assign15110_e15207 + assign15110_e15223);
                let assign15110_e15225: f64 = (0.5 * assign15110_e15224);
                (assign15110_e15225, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))),)
            } else {
                let (assign15110_e15252, assign15110_e15252_d_n2, assign15110_e15252_d_n4, assign15110_e15252_d_n7, assign15110_e15252_d_n9, assign15110_e15252_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15110_e15233: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15234: f64 = assign15110_e15233;
                        let assign15110_e15238: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15239: f64 = (-assign15110_e15238);
                        let assign15110_e15243: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15244: f64 = (-assign15110_e15243);
                        let assign15110_e15245: f64 = (assign15110_e15239 * assign15110_e15244);
                        let assign15110_e15247: f64 = (assign15110_e15245 + p.p53);
                        let assign15110_e15248: f64 = (assign15110_e15247).sqrt();
                        let assign15110_e15249: f64 = (assign15110_e15234 + assign15110_e15248);
                        let assign15110_e15250: f64 = (0.5 * assign15110_e15249);
                        (assign15110_e15250, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15244) + (assign15110_e15239 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15110_e15248)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15244) + (assign15110_e15239 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15110_e15248)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15110_e15252, assign15110_e15252_d_n2, assign15110_e15252_d_n4, assign15110_e15252_d_n7, assign15110_e15252_d_n9, assign15110_e15252_d_n10,)
            }
        };
        let assign15110_e15255: f64 = (assign15110_e15253).powf(locals.var_fn169_calc_iq__beta);
        let assign15110_e15256: f64 = (1.0 + assign15110_e15255);
        let assign15110_e15259: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign15110_e15260: f64 = (assign15110_e15256).powf(assign15110_e15259);
        let assign15110_e15261: f64 = (1.0 / assign15110_e15260);
        (assign15110_e15261, (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n2)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n2 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n2)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n2 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n4)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n4 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n4)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n4 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n7)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n7 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n7)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n7 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n9)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n9 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n9)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n9 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n10)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n10 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n10)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n10 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))),)
    } else {
        (locals.var_fn169_calc_iq__fsd0, locals.var_fn169_calc_iq__fsd0_dn2, locals.var_fn169_calc_iq__fsd0_dn4, locals.var_fn169_calc_iq__fsd0_dn7, locals.var_fn169_calc_iq__fsd0_dn9, locals.var_fn169_calc_iq__fsd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd0 = assign15110_e15263;
        locals.var_fn169_calc_iq__fsd0_dn2 = assign15110_e15263_d_n2;
        locals.var_fn169_calc_iq__fsd0_dn4 = assign15110_e15263_d_n4;
        locals.var_fn169_calc_iq__fsd0_dn7 = assign15110_e15263_d_n7;
        locals.var_fn169_calc_iq__fsd0_dn9 = assign15110_e15263_d_n9;
        locals.var_fn169_calc_iq__fsd0_dn10 = assign15110_e15263_d_n10;

        let (assign15120_e15269, assign15120_e15269_d_n2, assign15120_e15269_d_n4, assign15120_e15269_d_n7, assign15120_e15269_d_n9, assign15120_e15269_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15120_e15267: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0);
        (assign15120_e15267, (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn2), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn4), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn7), ((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__fsd0) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn9)), ((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__fsd0) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdx0, locals.var_fn169_calc_iq__vdx0_dn2, locals.var_fn169_calc_iq__vdx0_dn4, locals.var_fn169_calc_iq__vdx0_dn7, locals.var_fn169_calc_iq__vdx0_dn9, locals.var_fn169_calc_iq__vdx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx0 = assign15120_e15269;
        locals.var_fn169_calc_iq__vdx0_dn2 = assign15120_e15269_d_n2;
        locals.var_fn169_calc_iq__vdx0_dn4 = assign15120_e15269_d_n4;
        locals.var_fn169_calc_iq__vdx0_dn7 = assign15120_e15269_d_n7;
        locals.var_fn169_calc_iq__vdx0_dn9 = assign15120_e15269_d_n9;
        locals.var_fn169_calc_iq__vdx0_dn10 = assign15120_e15269_d_n10;

        let (assign15130_e15344, assign15130_e15344_d_n2, assign15130_e15344_d_n4, assign15130_e15344_d_n7, assign15130_e15344_d_n9, assign15130_e15344_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15130_e15334, assign15130_e15334_d_n2, assign15130_e15334_d_n4, assign15130_e15334_d_n7, assign15130_e15334_d_n9, assign15130_e15334_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15130_e15280: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15282: f64 = (assign15130_e15280 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15283: f64 = assign15130_e15282;
                let assign15130_e15286: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15288: f64 = (assign15130_e15286 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15289: f64 = (-assign15130_e15288);
                let assign15130_e15292: f64 = (0.001 / p.p53);
                let assign15130_e15295: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15297: f64 = (assign15130_e15295 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15298: f64 = (-assign15130_e15297);
                let assign15130_e15299: f64 = (assign15130_e15292 * assign15130_e15298);
                let assign15130_e15300: f64 = (assign15130_e15299).tanh();
                let assign15130_e15301: f64 = (assign15130_e15289 * assign15130_e15300);
                let assign15130_e15302: f64 = (assign15130_e15283 + assign15130_e15301);
                let assign15130_e15303: f64 = (0.5 * assign15130_e15302);
                (assign15130_e15303, (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))),)
            } else {
                let (assign15130_e15333, assign15130_e15333_d_n2, assign15130_e15333_d_n4, assign15130_e15333_d_n7, assign15130_e15333_d_n9, assign15130_e15333_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15130_e15310: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15312: f64 = (assign15130_e15310 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15313: f64 = assign15130_e15312;
                        let assign15130_e15316: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15318: f64 = (assign15130_e15316 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15319: f64 = (-assign15130_e15318);
                        let assign15130_e15322: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15324: f64 = (assign15130_e15322 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15325: f64 = (-assign15130_e15324);
                        let assign15130_e15326: f64 = (assign15130_e15319 * assign15130_e15325);
                        let assign15130_e15328: f64 = (assign15130_e15326 + p.p53);
                        let assign15130_e15329: f64 = (assign15130_e15328).sqrt();
                        let assign15130_e15330: f64 = (assign15130_e15313 + assign15130_e15329);
                        let assign15130_e15331: f64 = (0.5 * assign15130_e15330);
                        (assign15130_e15331, (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15325) + (assign15130_e15319 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15130_e15329)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15325) + (assign15130_e15319 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15130_e15329)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15130_e15333, assign15130_e15333_d_n2, assign15130_e15333_d_n4, assign15130_e15333_d_n7, assign15130_e15333_d_n9, assign15130_e15333_d_n10,)
            }
        };
        let assign15130_e15336: f64 = (assign15130_e15334).powf(locals.var_fn169_calc_iq__beta);
        let assign15130_e15337: f64 = (1.0 + assign15130_e15336);
        let assign15130_e15340: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign15130_e15341: f64 = (assign15130_e15337).powf(assign15130_e15340);
        let assign15130_e15342: f64 = (1.0 / assign15130_e15341);
        (assign15130_e15342, (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n2)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n2 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n2)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n2 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n4)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n4 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n4)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n4 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n7)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n7 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n7)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n7 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n9)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n9 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n9)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n9 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n10)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n10 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n10)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n10 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))),)
    } else {
        (locals.var_fn169_calc_iq__fds0, locals.var_fn169_calc_iq__fds0_dn2, locals.var_fn169_calc_iq__fds0_dn4, locals.var_fn169_calc_iq__fds0_dn7, locals.var_fn169_calc_iq__fds0_dn9, locals.var_fn169_calc_iq__fds0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds0 = assign15130_e15344;
        locals.var_fn169_calc_iq__fds0_dn2 = assign15130_e15344_d_n2;
        locals.var_fn169_calc_iq__fds0_dn4 = assign15130_e15344_d_n4;
        locals.var_fn169_calc_iq__fds0_dn7 = assign15130_e15344_d_n7;
        locals.var_fn169_calc_iq__fds0_dn9 = assign15130_e15344_d_n9;
        locals.var_fn169_calc_iq__fds0_dn10 = assign15130_e15344_d_n10;

        let (assign15140_e15351, assign15140_e15351_d_n2, assign15140_e15351_d_n4, assign15140_e15351_d_n7, assign15140_e15351_d_n9, assign15140_e15351_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15140_e15347: f64 = (-locals.var_fn169_calc_iq__vdsin);
        let assign15140_e15349: f64 = (assign15140_e15347 * locals.var_fn169_calc_iq__fds0);
        (assign15140_e15349, (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn2), (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn4), (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn7), (((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__fds0) + (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn9)), (((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__fds0) + (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vsx0, locals.var_fn169_calc_iq__vsx0_dn2, locals.var_fn169_calc_iq__vsx0_dn4, locals.var_fn169_calc_iq__vsx0_dn7, locals.var_fn169_calc_iq__vsx0_dn9, locals.var_fn169_calc_iq__vsx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx0 = assign15140_e15351;
        locals.var_fn169_calc_iq__vsx0_dn2 = assign15140_e15351_d_n2;
        locals.var_fn169_calc_iq__vsx0_dn4 = assign15140_e15351_d_n4;
        locals.var_fn169_calc_iq__vsx0_dn7 = assign15140_e15351_d_n7;
        locals.var_fn169_calc_iq__vsx0_dn9 = assign15140_e15351_d_n9;
        locals.var_fn169_calc_iq__vsx0_dn10 = assign15140_e15351_d_n10;

        let (assign15150_e15359, assign15150_e15359_d_n2, assign15150_e15359_d_n4, assign15150_e15359_d_n7, assign15150_e15359_d_n9, assign15150_e15359_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15150_e15355: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__myarg0);
        let assign15150_e15357: f64 = (assign15150_e15355 / locals.var_fn169_calc_iq__alpha_phit);
        (assign15150_e15357, (locals.var_fn169_calc_iq__vgsin_dn2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign15150_e15355 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (locals.var_fn169_calc_iq__vgsin_dn7 / locals.var_fn169_calc_iq__alpha_phit), 0.0, (locals.var_fn169_calc_iq__vgsin_dn10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign15150_e15359;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign15150_e15359_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign15150_e15359_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign15150_e15359_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign15150_e15359_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign15150_e15359_d_n10;

        let assign15160_e15362: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign15160_e15362;

        let (assign15170_e15368, assign15170_e15368_d_n2, assign15170_e15368_d_n4, assign15170_e15368_d_n7, assign15170_e15368_d_n9, assign15170_e15368_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard187 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15170_e15368;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15170_e15368_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15170_e15368_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15170_e15368_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15170_e15368_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15170_e15368_d_n10;

        let assign15180_e15371: f64 = (-50.0);
        let assign15180_e15372: f64 = if locals.var_fn169_calc_iq__exparg0 < assign15180_e15371 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign15180_e15372;

        let (assign15190_e15381, assign15190_e15381_d_n2, assign15190_e15381_d_n4, assign15190_e15381_d_n7, assign15190_e15381_d_n9, assign15190_e15381_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard187 == 0.0)) && (locals.var_guard188 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15190_e15381;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15190_e15381_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15190_e15381_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15190_e15381_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15190_e15381_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15190_e15381_d_n10;

        let (assign15200_e15396, assign15200_e15396_d_n2, assign15200_e15396_d_n4, assign15200_e15396_d_n7, assign15200_e15396_d_n9, assign15200_e15396_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard187 == 0.0)) && (locals.var_guard188 == 0.0)) {
        let assign15200_e15392: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign15200_e15393: f64 = (1.0 + assign15200_e15392);
        let assign15200_e15394: f64 = (1.0 / assign15200_e15393);
        (assign15200_e15394, (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign15200_e15393 * assign15200_e15393))),)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15200_e15396;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15200_e15396_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15200_e15396_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15200_e15396_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15200_e15396_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15200_e15396_d_n10;

        let (assign15210_e15414, assign15210_e15414_d_n2, assign15210_e15414_d_n4, assign15210_e15414_d_n7, assign15210_e15414_d_n9, assign15210_e15414_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15210_e15400: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__vsx0);
        let assign15210_e15404: f64 = (p.p51 * 0.1);
        let assign15210_e15406: f64 = (assign15210_e15404 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15210_e15408: f64 = (assign15210_e15406 * locals.var_fn169_calc_iq__ffs0);
        let assign15210_e15409: f64 = (locals.var_fn169_calc_iq__vtof - assign15210_e15408);
        let assign15210_e15410: f64 = (assign15210_e15400 - assign15210_e15409);
        let assign15210_e15412: f64 = (assign15210_e15410 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15210_e15412, (((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__vsx0_dn2) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), (((((-locals.var_fn169_calc_iq__vsx0_dn4) - (locals.var_fn169_calc_iq__vtof_dn4 - (((assign15210_e15404 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffs0) + (assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15210_e15410 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__vsx0_dn7) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__vsx0_dn9) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__vsx0_dn10) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etas0, locals.var_fn169_calc_iq__etas0_dn2, locals.var_fn169_calc_iq__etas0_dn4, locals.var_fn169_calc_iq__etas0_dn7, locals.var_fn169_calc_iq__etas0_dn9, locals.var_fn169_calc_iq__etas0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas0 = assign15210_e15414;
        locals.var_fn169_calc_iq__etas0_dn2 = assign15210_e15414_d_n2;
        locals.var_fn169_calc_iq__etas0_dn4 = assign15210_e15414_d_n4;
        locals.var_fn169_calc_iq__etas0_dn7 = assign15210_e15414_d_n7;
        locals.var_fn169_calc_iq__etas0_dn9 = assign15210_e15414_d_n9;
        locals.var_fn169_calc_iq__etas0_dn10 = assign15210_e15414_d_n10;

        let assign15220_e15417: f64 = if locals.var_fn169_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign15220_e15417;

        let (assign15230_e15425, assign15230_e15425_d_n2, assign15230_e15425_d_n4, assign15230_e15425_d_n7, assign15230_e15425_d_n9, assign15230_e15425_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard189 != 0.0)) {
        let assign15230_e15423: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0);
        (assign15230_e15423, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__etas0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15230_e15425;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15230_e15425_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15230_e15425_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15230_e15425_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15230_e15425_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15230_e15425_d_n10;

        let assign15240_e15428: f64 = (-50.0);
        let assign15240_e15429: f64 = if locals.var_fn169_calc_iq__etas0 < assign15240_e15428 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign15240_e15429;

        let (assign15250_e15441, assign15250_e15441_d_n2, assign15250_e15441_d_n4, assign15250_e15441_d_n7, assign15250_e15441_d_n9, assign15250_e15441_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 != 0.0)) {
        let assign15250_e15438: f64 = (locals.var_fn169_calc_iq__etas0).exp();
        let assign15250_e15439: f64 = (locals.var_fn169_calc_iq__qref0 * assign15250_e15438);
        (assign15250_e15439, (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15250_e15438) + (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15250_e15441;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15250_e15441_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15250_e15441_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15250_e15441_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15250_e15441_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15250_e15441_d_n10;

        let (assign15260_e15457, assign15260_e15457_d_n2, assign15260_e15457_d_n4, assign15260_e15457_d_n7, assign15260_e15457_d_n9, assign15260_e15457_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 == 0.0)) {
        let assign15260_e15452: f64 = (locals.var_fn169_calc_iq__etas0).exp();
        let assign15260_e15453: f64 = (1.0 + assign15260_e15452);
        let assign15260_e15454: f64 = (assign15260_e15453).ln();
        let assign15260_e15455: f64 = (locals.var_fn169_calc_iq__qref0 * assign15260_e15454);
        (assign15260_e15455, (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn2) / assign15260_e15453)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15260_e15454) + (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn4) / assign15260_e15453))), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn7) / assign15260_e15453)), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn9) / assign15260_e15453)), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn10) / assign15260_e15453)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15260_e15457;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15260_e15457_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15260_e15457_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15260_e15457_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15260_e15457_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15260_e15457_d_n10;

        let (assign15270_e15465, assign15270_e15465_d_n2, assign15270_e15465_d_n4, assign15270_e15465_d_n7, assign15270_e15465_d_n9, assign15270_e15465_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15270_e15461: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__myarg0);
        let assign15270_e15463: f64 = (assign15270_e15461 / locals.var_fn169_calc_iq__alpha_phit);
        (assign15270_e15463, (locals.var_fn169_calc_iq__vgdin_dn2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign15270_e15461 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (locals.var_fn169_calc_iq__vgdin_dn7 / locals.var_fn169_calc_iq__alpha_phit), (locals.var_fn169_calc_iq__vgdin_dn9 / locals.var_fn169_calc_iq__alpha_phit), (locals.var_fn169_calc_iq__vgdin_dn10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign15270_e15465;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign15270_e15465_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign15270_e15465_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign15270_e15465_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign15270_e15465_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign15270_e15465_d_n10;

        let assign15280_e15468: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign15280_e15468;

        let (assign15290_e15474, assign15290_e15474_d_n2, assign15290_e15474_d_n4, assign15290_e15474_d_n7, assign15290_e15474_d_n9, assign15290_e15474_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15290_e15474;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15290_e15474_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15290_e15474_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15290_e15474_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15290_e15474_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15290_e15474_d_n10;

        let assign15300_e15477: f64 = (-50.0);
        let assign15300_e15478: f64 = if locals.var_fn169_calc_iq__exparg0 < assign15300_e15477 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign15300_e15478;

        let (assign15310_e15487, assign15310_e15487_d_n2, assign15310_e15487_d_n4, assign15310_e15487_d_n7, assign15310_e15487_d_n9, assign15310_e15487_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard191 == 0.0)) && (locals.var_guard192 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15310_e15487;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15310_e15487_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15310_e15487_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15310_e15487_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15310_e15487_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15310_e15487_d_n10;

        let (assign15320_e15502, assign15320_e15502_d_n2, assign15320_e15502_d_n4, assign15320_e15502_d_n7, assign15320_e15502_d_n9, assign15320_e15502_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard191 == 0.0)) && (locals.var_guard192 == 0.0)) {
        let assign15320_e15498: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign15320_e15499: f64 = (1.0 + assign15320_e15498);
        let assign15320_e15500: f64 = (1.0 / assign15320_e15499);
        (assign15320_e15500, (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign15320_e15499 * assign15320_e15499))),)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15320_e15502;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15320_e15502_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15320_e15502_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15320_e15502_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15320_e15502_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15320_e15502_d_n10;

        let (assign15330_e15520, assign15330_e15520_d_n2, assign15330_e15520_d_n4, assign15330_e15520_d_n7, assign15330_e15520_d_n9, assign15330_e15520_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15330_e15506: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdx0);
        let assign15330_e15510: f64 = (p.p51 * 0.1);
        let assign15330_e15512: f64 = (assign15330_e15510 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15330_e15514: f64 = (assign15330_e15512 * locals.var_fn169_calc_iq__ffd0);
        let assign15330_e15515: f64 = (locals.var_fn169_calc_iq__vtof - assign15330_e15514);
        let assign15330_e15516: f64 = (assign15330_e15506 - assign15330_e15515);
        let assign15330_e15518: f64 = (assign15330_e15516 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15330_e15518, (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vdx0_dn2) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), (((((-locals.var_fn169_calc_iq__vdx0_dn4) - (locals.var_fn169_calc_iq__vtof_dn4 - (((assign15330_e15510 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffd0) + (assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15330_e15516 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vdx0_dn7) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), (((-locals.var_fn169_calc_iq__vdx0_dn9) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdx0_dn10) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etad0, locals.var_fn169_calc_iq__etad0_dn2, locals.var_fn169_calc_iq__etad0_dn4, locals.var_fn169_calc_iq__etad0_dn7, locals.var_fn169_calc_iq__etad0_dn9, locals.var_fn169_calc_iq__etad0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad0 = assign15330_e15520;
        locals.var_fn169_calc_iq__etad0_dn2 = assign15330_e15520_d_n2;
        locals.var_fn169_calc_iq__etad0_dn4 = assign15330_e15520_d_n4;
        locals.var_fn169_calc_iq__etad0_dn7 = assign15330_e15520_d_n7;
        locals.var_fn169_calc_iq__etad0_dn9 = assign15330_e15520_d_n9;
        locals.var_fn169_calc_iq__etad0_dn10 = assign15330_e15520_d_n10;

        let assign15340_e15523: f64 = if locals.var_fn169_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign15340_e15523;

        let (assign15350_e15531, assign15350_e15531_d_n2, assign15350_e15531_d_n4, assign15350_e15531_d_n7, assign15350_e15531_d_n9, assign15350_e15531_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard193 != 0.0)) {
        let assign15350_e15529: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0);
        (assign15350_e15529, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__etad0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15350_e15531;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15350_e15531_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15350_e15531_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15350_e15531_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15350_e15531_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15350_e15531_d_n10;

        let assign15360_e15534: f64 = (-50.0);
        let assign15360_e15535: f64 = if locals.var_fn169_calc_iq__etad0 < assign15360_e15534 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign15360_e15535;

        let (assign15370_e15547, assign15370_e15547_d_n2, assign15370_e15547_d_n4, assign15370_e15547_d_n7, assign15370_e15547_d_n9, assign15370_e15547_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard193 == 0.0)) && (locals.var_guard194 != 0.0)) {
        let assign15370_e15544: f64 = (locals.var_fn169_calc_iq__etad0).exp();
        let assign15370_e15545: f64 = (locals.var_fn169_calc_iq__qref0 * assign15370_e15544);
        (assign15370_e15545, (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15370_e15544) + (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15370_e15547;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15370_e15547_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15370_e15547_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15370_e15547_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15370_e15547_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15370_e15547_d_n10;

        let (assign15380_e15563, assign15380_e15563_d_n2, assign15380_e15563_d_n4, assign15380_e15563_d_n7, assign15380_e15563_d_n9, assign15380_e15563_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard193 == 0.0)) && (locals.var_guard194 == 0.0)) {
        let assign15380_e15558: f64 = (locals.var_fn169_calc_iq__etad0).exp();
        let assign15380_e15559: f64 = (1.0 + assign15380_e15558);
        let assign15380_e15560: f64 = (assign15380_e15559).ln();
        let assign15380_e15561: f64 = (locals.var_fn169_calc_iq__qref0 * assign15380_e15560);
        (assign15380_e15561, (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn2) / assign15380_e15559)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15380_e15560) + (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn4) / assign15380_e15559))), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn7) / assign15380_e15559)), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn9) / assign15380_e15559)), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn10) / assign15380_e15559)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15380_e15563;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15380_e15563_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15380_e15563_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15380_e15563_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15380_e15563_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15380_e15563_d_n10;

        let (assign15390_e15571, assign15390_e15571_d_n2, assign15390_e15571_d_n4, assign15390_e15571_d_n7, assign15390_e15571_d_n9, assign15390_e15571_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15390_e15567: f64 = (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0);
        let assign15390_e15569: f64 = (assign15390_e15567 + 1e-38);
        (assign15390_e15569, ((locals.var_fn169_calc_iq__qinvs0_dn2 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn2)), ((locals.var_fn169_calc_iq__qinvs0_dn4 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn4)), ((locals.var_fn169_calc_iq__qinvs0_dn7 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn7)), ((locals.var_fn169_calc_iq__qinvs0_dn9 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn9)), ((locals.var_fn169_calc_iq__qinvs0_dn10 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qs2, locals.var_fn169_calc_iq__qs2_dn2, locals.var_fn169_calc_iq__qs2_dn4, locals.var_fn169_calc_iq__qs2_dn7, locals.var_fn169_calc_iq__qs2_dn9, locals.var_fn169_calc_iq__qs2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs2 = assign15390_e15571;
        locals.var_fn169_calc_iq__qs2_dn2 = assign15390_e15571_d_n2;
        locals.var_fn169_calc_iq__qs2_dn4 = assign15390_e15571_d_n4;
        locals.var_fn169_calc_iq__qs2_dn7 = assign15390_e15571_d_n7;
        locals.var_fn169_calc_iq__qs2_dn9 = assign15390_e15571_d_n9;
        locals.var_fn169_calc_iq__qs2_dn10 = assign15390_e15571_d_n10;

        let (assign15400_e15579, assign15400_e15579_d_n2, assign15400_e15579_d_n4, assign15400_e15579_d_n7, assign15400_e15579_d_n9, assign15400_e15579_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15400_e15575: f64 = (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0);
        let assign15400_e15577: f64 = (assign15400_e15575 + 1e-57);
        (assign15400_e15577, ((locals.var_fn169_calc_iq__qs2_dn2 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn2)), ((locals.var_fn169_calc_iq__qs2_dn4 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn4)), ((locals.var_fn169_calc_iq__qs2_dn7 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn7)), ((locals.var_fn169_calc_iq__qs2_dn9 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn9)), ((locals.var_fn169_calc_iq__qs2_dn10 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qs3, locals.var_fn169_calc_iq__qs3_dn2, locals.var_fn169_calc_iq__qs3_dn4, locals.var_fn169_calc_iq__qs3_dn7, locals.var_fn169_calc_iq__qs3_dn9, locals.var_fn169_calc_iq__qs3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs3 = assign15400_e15579;
        locals.var_fn169_calc_iq__qs3_dn2 = assign15400_e15579_d_n2;
        locals.var_fn169_calc_iq__qs3_dn4 = assign15400_e15579_d_n4;
        locals.var_fn169_calc_iq__qs3_dn7 = assign15400_e15579_d_n7;
        locals.var_fn169_calc_iq__qs3_dn9 = assign15400_e15579_d_n9;
        locals.var_fn169_calc_iq__qs3_dn10 = assign15400_e15579_d_n10;

        let (assign15410_e15587, assign15410_e15587_d_n2, assign15410_e15587_d_n4, assign15410_e15587_d_n7, assign15410_e15587_d_n9, assign15410_e15587_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15410_e15583: f64 = (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0);
        let assign15410_e15585: f64 = (assign15410_e15583 + 1e-38);
        (assign15410_e15585, ((locals.var_fn169_calc_iq__qinvd0_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qinvd0_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qinvd0_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qinvd0_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qinvd0_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qd2, locals.var_fn169_calc_iq__qd2_dn2, locals.var_fn169_calc_iq__qd2_dn4, locals.var_fn169_calc_iq__qd2_dn7, locals.var_fn169_calc_iq__qd2_dn9, locals.var_fn169_calc_iq__qd2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd2 = assign15410_e15587;
        locals.var_fn169_calc_iq__qd2_dn2 = assign15410_e15587_d_n2;
        locals.var_fn169_calc_iq__qd2_dn4 = assign15410_e15587_d_n4;
        locals.var_fn169_calc_iq__qd2_dn7 = assign15410_e15587_d_n7;
        locals.var_fn169_calc_iq__qd2_dn9 = assign15410_e15587_d_n9;
        locals.var_fn169_calc_iq__qd2_dn10 = assign15410_e15587_d_n10;

        let (assign15420_e15595, assign15420_e15595_d_n2, assign15420_e15595_d_n4, assign15420_e15595_d_n7, assign15420_e15595_d_n9, assign15420_e15595_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15420_e15591: f64 = (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0);
        let assign15420_e15593: f64 = (assign15420_e15591 + 1e-57);
        (assign15420_e15593, ((locals.var_fn169_calc_iq__qd2_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qd2_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qd2_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qd2_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qd2_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qd3, locals.var_fn169_calc_iq__qd3_dn2, locals.var_fn169_calc_iq__qd3_dn4, locals.var_fn169_calc_iq__qd3_dn7, locals.var_fn169_calc_iq__qd3_dn9, locals.var_fn169_calc_iq__qd3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd3 = assign15420_e15595;
        locals.var_fn169_calc_iq__qd3_dn2 = assign15420_e15595_d_n2;
        locals.var_fn169_calc_iq__qd3_dn4 = assign15420_e15595_d_n4;
        locals.var_fn169_calc_iq__qd3_dn7 = assign15420_e15595_d_n7;
        locals.var_fn169_calc_iq__qd3_dn9 = assign15420_e15595_d_n9;
        locals.var_fn169_calc_iq__qd3_dn10 = assign15420_e15595_d_n10;

    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15430_e15603, assign15430_e15603_d_n2, assign15430_e15603_d_n4, assign15430_e15603_d_n7, assign15430_e15603_d_n9, assign15430_e15603_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15430_e15599: f64 = (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0);
        let assign15430_e15601: f64 = (assign15430_e15599 + 1e-38);
        (assign15430_e15601, ((locals.var_fn169_calc_iq__qinvs0_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qinvs0_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qinvs0_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qinvs0_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qinvs0_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qsqd, locals.var_fn169_calc_iq__qsqd_dn2, locals.var_fn169_calc_iq__qsqd_dn4, locals.var_fn169_calc_iq__qsqd_dn7, locals.var_fn169_calc_iq__qsqd_dn9, locals.var_fn169_calc_iq__qsqd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsqd = assign15430_e15603;
        locals.var_fn169_calc_iq__qsqd_dn2 = assign15430_e15603_d_n2;
        locals.var_fn169_calc_iq__qsqd_dn4 = assign15430_e15603_d_n4;
        locals.var_fn169_calc_iq__qsqd_dn7 = assign15430_e15603_d_n7;
        locals.var_fn169_calc_iq__qsqd_dn9 = assign15430_e15603_d_n9;
        locals.var_fn169_calc_iq__qsqd_dn10 = assign15430_e15603_d_n10;

        let (assign15440_e15621, assign15440_e15621_d_n2, assign15440_e15621_d_n4, assign15440_e15621_d_n7, assign15440_e15621_d_n9, assign15440_e15621_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15440_e15607: f64 = (2.0 / 3.0);
        let assign15440_e15610: f64 = (locals.var_fn169_calc_iq__qs2 + locals.var_fn169_calc_iq__qd2);
        let assign15440_e15612: f64 = (assign15440_e15610 + locals.var_fn169_calc_iq__qsqd);
        let assign15440_e15613: f64 = (assign15440_e15607 * assign15440_e15612);
        let assign15440_e15616: f64 = (locals.var_fn169_calc_iq__qinvs0 + locals.var_fn169_calc_iq__qinvd0);
        let assign15440_e15618: f64 = (assign15440_e15616 + 2e-19);
        let assign15440_e15619: f64 = (assign15440_e15613 / assign15440_e15618);
        (assign15440_e15619, ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn2 + locals.var_fn169_calc_iq__qd2_dn2) + locals.var_fn169_calc_iq__qsqd_dn2)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn2 + locals.var_fn169_calc_iq__qinvd0_dn2))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn4 + locals.var_fn169_calc_iq__qd2_dn4) + locals.var_fn169_calc_iq__qsqd_dn4)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn4 + locals.var_fn169_calc_iq__qinvd0_dn4))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn7 + locals.var_fn169_calc_iq__qd2_dn7) + locals.var_fn169_calc_iq__qsqd_dn7)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn7 + locals.var_fn169_calc_iq__qinvd0_dn7))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn9 + locals.var_fn169_calc_iq__qd2_dn9) + locals.var_fn169_calc_iq__qsqd_dn9)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn9 + locals.var_fn169_calc_iq__qinvd0_dn9))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn10 + locals.var_fn169_calc_iq__qd2_dn10) + locals.var_fn169_calc_iq__qsqd_dn10)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn10 + locals.var_fn169_calc_iq__qinvd0_dn10))) / (assign15440_e15618 * assign15440_e15618)),)
    } else {
        (locals.var_fn169_calc_iq__qinvdd, locals.var_fn169_calc_iq__qinvdd_dn2, locals.var_fn169_calc_iq__qinvdd_dn4, locals.var_fn169_calc_iq__qinvdd_dn7, locals.var_fn169_calc_iq__qinvdd_dn9, locals.var_fn169_calc_iq__qinvdd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvdd = assign15440_e15621;
        locals.var_fn169_calc_iq__qinvdd_dn2 = assign15440_e15621_d_n2;
        locals.var_fn169_calc_iq__qinvdd_dn4 = assign15440_e15621_d_n4;
        locals.var_fn169_calc_iq__qinvdd_dn7 = assign15440_e15621_d_n7;
        locals.var_fn169_calc_iq__qinvdd_dn9 = assign15440_e15621_d_n9;
        locals.var_fn169_calc_iq__qinvdd_dn10 = assign15440_e15621_d_n10;

        let (assign15450_e15655, assign15450_e15655_d_n2, assign15450_e15655_d_n4, assign15450_e15655_d_n7, assign15450_e15655_d_n9, assign15450_e15655_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15450_e15626: f64 = (2.0 * locals.var_fn169_calc_iq__qs3);
        let assign15450_e15629: f64 = (3.0 * locals.var_fn169_calc_iq__qd3);
        let assign15450_e15630: f64 = (assign15450_e15626 + assign15450_e15629);
        let assign15450_e15633: f64 = (4.0 * locals.var_fn169_calc_iq__qs2);
        let assign15450_e15635: f64 = (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0);
        let assign15450_e15636: f64 = (assign15450_e15630 + assign15450_e15635);
        let assign15450_e15639: f64 = (6.0 * locals.var_fn169_calc_iq__qd2);
        let assign15450_e15641: f64 = (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0);
        let assign15450_e15642: f64 = (assign15450_e15636 + assign15450_e15641);
        let assign15450_e15643: f64 = (2.0 * assign15450_e15642);
        let assign15450_e15647: f64 = (locals.var_fn169_calc_iq__qs2 + locals.var_fn169_calc_iq__qd2);
        let assign15450_e15650: f64 = (2.0 * locals.var_fn169_calc_iq__qsqd);
        let assign15450_e15651: f64 = (assign15450_e15647 + assign15450_e15650);
        let assign15450_e15652: f64 = (15.0 * assign15450_e15651);
        let assign15450_e15653: f64 = (assign15450_e15643 / assign15450_e15652);
        (assign15450_e15653, ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn2) + (3.0 * locals.var_fn169_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn2) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn2) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn2)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn2 + locals.var_fn169_calc_iq__qd2_dn2) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn2))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn4) + (3.0 * locals.var_fn169_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn4) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn4) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn4)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn4 + locals.var_fn169_calc_iq__qd2_dn4) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn4))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn7) + (3.0 * locals.var_fn169_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn7) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn7) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn7)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn7 + locals.var_fn169_calc_iq__qd2_dn7) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn7))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn9) + (3.0 * locals.var_fn169_calc_iq__qd3_dn9)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn9) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn9))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn9) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn9)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn9 + locals.var_fn169_calc_iq__qd2_dn9) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn9))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn10) + (3.0 * locals.var_fn169_calc_iq__qd3_dn10)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn10) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn10))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn10) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn10)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn10 + locals.var_fn169_calc_iq__qd2_dn10) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn10))))) / (assign15450_e15652 * assign15450_e15652)),)
    } else {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd1 = assign15450_e15655;
        locals.var_fn169_calc_iq__qd1_dn2 = assign15450_e15655_d_n2;
        locals.var_fn169_calc_iq__qd1_dn4 = assign15450_e15655_d_n4;
        locals.var_fn169_calc_iq__qd1_dn7 = assign15450_e15655_d_n7;
        locals.var_fn169_calc_iq__qd1_dn9 = assign15450_e15655_d_n9;
        locals.var_fn169_calc_iq__qd1_dn10 = assign15450_e15655_d_n10;

        let (assign15460_e15661, assign15460_e15661_d_n2, assign15460_e15661_d_n4, assign15460_e15661_d_n7, assign15460_e15661_d_n9, assign15460_e15661_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15460_e15659: f64 = (locals.var_fn169_calc_iq__qinvdd - locals.var_fn169_calc_iq__qd1);
        (assign15460_e15659, (locals.var_fn169_calc_iq__qinvdd_dn2 - locals.var_fn169_calc_iq__qd1_dn2), (locals.var_fn169_calc_iq__qinvdd_dn4 - locals.var_fn169_calc_iq__qd1_dn4), (locals.var_fn169_calc_iq__qinvdd_dn7 - locals.var_fn169_calc_iq__qd1_dn7), (locals.var_fn169_calc_iq__qinvdd_dn9 - locals.var_fn169_calc_iq__qd1_dn9), (locals.var_fn169_calc_iq__qinvdd_dn10 - locals.var_fn169_calc_iq__qd1_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qs, locals.var_fn169_calc_iq__qs_dn2, locals.var_fn169_calc_iq__qs_dn4, locals.var_fn169_calc_iq__qs_dn7, locals.var_fn169_calc_iq__qs_dn9, locals.var_fn169_calc_iq__qs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs = assign15460_e15661;
        locals.var_fn169_calc_iq__qs_dn2 = assign15460_e15661_d_n2;
        locals.var_fn169_calc_iq__qs_dn4 = assign15460_e15661_d_n4;
        locals.var_fn169_calc_iq__qs_dn7 = assign15460_e15661_d_n7;
        locals.var_fn169_calc_iq__qs_dn9 = assign15460_e15661_d_n9;
        locals.var_fn169_calc_iq__qs_dn10 = assign15460_e15661_d_n10;

        let (assign15470_e15665, assign15470_e15665_d_n2, assign15470_e15665_d_n4, assign15470_e15665_d_n7, assign15470_e15665_d_n9, assign15470_e15665_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__qd, locals.var_fn169_calc_iq__qd_dn2, locals.var_fn169_calc_iq__qd_dn4, locals.var_fn169_calc_iq__qd_dn7, locals.var_fn169_calc_iq__qd_dn9, locals.var_fn169_calc_iq__qd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd = assign15470_e15665;
        locals.var_fn169_calc_iq__qd_dn2 = assign15470_e15665_d_n2;
        locals.var_fn169_calc_iq__qd_dn4 = assign15470_e15665_d_n4;
        locals.var_fn169_calc_iq__qd_dn7 = assign15470_e15665_d_n7;
        locals.var_fn169_calc_iq__qd_dn9 = assign15470_e15665_d_n9;
        locals.var_fn169_calc_iq__qd_dn10 = assign15470_e15665_d_n10;

        let (assign15480_e15679, assign15480_e15679_d_n2, assign15480_e15679_d_n4, assign15480_e15679_d_n7, assign15480_e15679_d_n9, assign15480_e15679_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15480_e15669: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15480_e15671: f64 = (assign15480_e15669 * locals.var_fn169_calc_iq__lin);
        let assign15480_e15673: f64 = (assign15480_e15671 * locals.var_fn169_calc_iq__type);
        let assign15480_e15675: f64 = (assign15480_e15673 * locals.var_fn169_calc_iq__qs);
        let assign15480_e15677: f64 = (assign15480_e15675 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15480_e15677, ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn4) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgsout = assign15480_e15679;
        locals.var_fn169_calc_iq__qgsout_dn2 = assign15480_e15679_d_n2;
        locals.var_fn169_calc_iq__qgsout_dn4 = assign15480_e15679_d_n4;
        locals.var_fn169_calc_iq__qgsout_dn7 = assign15480_e15679_d_n7;
        locals.var_fn169_calc_iq__qgsout_dn9 = assign15480_e15679_d_n9;
        locals.var_fn169_calc_iq__qgsout_dn10 = assign15480_e15679_d_n10;

        let (assign15490_e15693, assign15490_e15693_d_n2, assign15490_e15693_d_n4, assign15490_e15693_d_n7, assign15490_e15693_d_n9, assign15490_e15693_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15490_e15683: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15490_e15685: f64 = (assign15490_e15683 * locals.var_fn169_calc_iq__lin);
        let assign15490_e15687: f64 = (assign15490_e15685 * locals.var_fn169_calc_iq__type);
        let assign15490_e15689: f64 = (assign15490_e15687 * locals.var_fn169_calc_iq__qd);
        let assign15490_e15691: f64 = (assign15490_e15689 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15490_e15691, ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn4) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgdout = assign15490_e15693;
        locals.var_fn169_calc_iq__qgdout_dn2 = assign15490_e15693_d_n2;
        locals.var_fn169_calc_iq__qgdout_dn4 = assign15490_e15693_d_n4;
        locals.var_fn169_calc_iq__qgdout_dn7 = assign15490_e15693_d_n7;
        locals.var_fn169_calc_iq__qgdout_dn9 = assign15490_e15693_d_n9;
        locals.var_fn169_calc_iq__qgdout_dn10 = assign15490_e15693_d_n10;

        let assign15500_e15696: f64 = if locals.var_fn169_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign15500_e15696;

        let (assign15510_e15712, assign15510_e15712_d_n2, assign15510_e15712_d_n4, assign15510_e15712_d_n7, assign15510_e15712_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15510_e15704: f64 = (p.p51 * 0.5);
        let assign15510_e15706: f64 = (assign15510_e15704 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15510_e15707: f64 = (locals.var_fn169_calc_iq__vtof - assign15510_e15706);
        let assign15510_e15708: f64 = (locals.var_fn169_calc_iq__vcin - assign15510_e15707);
        let assign15510_e15710: f64 = (assign15510_e15708 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15510_e15710, (locals.var_fn169_calc_iq__vcin_dn2 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15510_e15704 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15510_e15708 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vcin_dn7 / locals.var_fn169_calc_iq__two_n_phit0), (locals.var_fn169_calc_iq__vcin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, locals.var_fn169_calc_iq__etac_dn10,)
    }
};
        locals.var_fn169_calc_iq__etac = assign15510_e15712;
        locals.var_fn169_calc_iq__etac_dn2 = assign15510_e15712_d_n2;
        locals.var_fn169_calc_iq__etac_dn4 = assign15510_e15712_d_n4;
        locals.var_fn169_calc_iq__etac_dn7 = assign15510_e15712_d_n7;
        locals.var_fn169_calc_iq__etac_dn10 = assign15510_e15712_d_n10;

        let assign15520_e15715: f64 = if locals.var_fn169_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign15520_e15715;

        let (assign15530_e15723, assign15530_e15723_d_n2, assign15530_e15723_d_n3, assign15530_e15723_d_n4, assign15530_e15723_d_n7, assign15530_e15723_d_n9, assign15530_e15723_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 != 0.0)) {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, 0.0, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, 0.0, locals.var_fn169_calc_iq__etac_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15530_e15723;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15530_e15723_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15530_e15723_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15530_e15723_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15530_e15723_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15530_e15723_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15530_e15723_d_n10;

        let assign15540_e15726: f64 = (-50.0);
        let assign15540_e15727: f64 = if locals.var_fn169_calc_iq__etac < assign15540_e15726 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign15540_e15727;

        let (assign15550_e15739, assign15550_e15739_d_n2, assign15550_e15739_d_n3, assign15550_e15739_d_n4, assign15550_e15739_d_n7, assign15550_e15739_d_n9, assign15550_e15739_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 == 0.0)) && (locals.var_guard197 != 0.0)) {
        let assign15550_e15737: f64 = (locals.var_fn169_calc_iq__etac).exp();
        (assign15550_e15737, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn2), 0.0, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn4), (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn7), 0.0, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15550_e15739;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15550_e15739_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15550_e15739_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15550_e15739_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15550_e15739_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15550_e15739_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15550_e15739_d_n10;

        let (assign15560_e15755, assign15560_e15755_d_n2, assign15560_e15755_d_n3, assign15560_e15755_d_n4, assign15560_e15755_d_n7, assign15560_e15755_d_n9, assign15560_e15755_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 == 0.0)) && (locals.var_guard197 == 0.0)) {
        let assign15560_e15751: f64 = (locals.var_fn169_calc_iq__etac).exp();
        let assign15560_e15752: f64 = (1.0 + assign15560_e15751);
        let assign15560_e15753: f64 = (assign15560_e15752).ln();
        (assign15560_e15753, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn2) / assign15560_e15752), 0.0, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn4) / assign15560_e15752), ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn7) / assign15560_e15752), 0.0, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn10) / assign15560_e15752),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15560_e15755;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15560_e15755_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15560_e15755_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15560_e15755_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15560_e15755_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15560_e15755_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15560_e15755_d_n10;

        let (assign15570_e15773, assign15570_e15773_d_n2, assign15570_e15773_d_n3, assign15570_e15773_d_n4, assign15570_e15773_d_n7, assign15570_e15773_d_n9, assign15570_e15773_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15570_e15761: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15570_e15763: f64 = (assign15570_e15761 * locals.var_fn169_calc_iq__type);
        let assign15570_e15765: f64 = (assign15570_e15763 * locals.var_fn169_calc_iq__cc);
        let assign15570_e15767: f64 = (assign15570_e15765 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15570_e15769: f64 = (assign15570_e15767 * locals.var_fn169_calc_iq__exparg);
        let assign15570_e15771: f64 = (assign15570_e15769 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15570_e15771, ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((((assign15570_e15763 * locals.var_fn169_calc_iq__cc_dn4) * locals.var_fn169_calc_iq__two_n_phit0) + (assign15570_e15765 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) * locals.var_fn169_calc_iq__exparg) + (assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign15570_e15773;
        locals.var_fn169_calc_iq__qcout_dn2 = assign15570_e15773_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign15570_e15773_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign15570_e15773_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign15570_e15773_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign15570_e15773_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign15570_e15773_d_n10;

        let (assign15580_e15789, assign15580_e15789_d_n3, assign15580_e15789_d_n4, assign15580_e15789_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15580_e15781: f64 = (p.p51 * 0.5);
        let assign15580_e15783: f64 = (assign15580_e15781 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15580_e15784: f64 = (locals.var_fn169_calc_iq__vtof - assign15580_e15783);
        let assign15580_e15785: f64 = (locals.var_fn169_calc_iq__vbin - assign15580_e15784);
        let assign15580_e15787: f64 = (assign15580_e15785 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15580_e15787, (locals.var_fn169_calc_iq__vbin_dn3 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15580_e15781 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15580_e15785 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vbin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etab, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, locals.var_fn169_calc_iq__etab_dn10,)
    }
};
        locals.var_fn169_calc_iq__etab = assign15580_e15789;
        locals.var_fn169_calc_iq__etab_dn3 = assign15580_e15789_d_n3;
        locals.var_fn169_calc_iq__etab_dn4 = assign15580_e15789_d_n4;
        locals.var_fn169_calc_iq__etab_dn10 = assign15580_e15789_d_n10;

        let assign15590_e15792: f64 = if locals.var_fn169_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign15590_e15792;

        let (assign15600_e15800, assign15600_e15800_d_n2, assign15600_e15800_d_n3, assign15600_e15800_d_n4, assign15600_e15800_d_n7, assign15600_e15800_d_n9, assign15600_e15800_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 != 0.0)) {
        (locals.var_fn169_calc_iq__etab, 0.0, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn169_calc_iq__etab_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15600_e15800;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15600_e15800_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15600_e15800_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15600_e15800_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15600_e15800_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15600_e15800_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15600_e15800_d_n10;

        let assign15610_e15803: f64 = (-50.0);
        let assign15610_e15804: f64 = if locals.var_fn169_calc_iq__etab < assign15610_e15803 { 1.0 } else { 0.0 };
        locals.var_guard199 = assign15610_e15804;

        let (assign15620_e15816, assign15620_e15816_d_n2, assign15620_e15816_d_n3, assign15620_e15816_d_n4, assign15620_e15816_d_n7, assign15620_e15816_d_n9, assign15620_e15816_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 == 0.0)) && (locals.var_guard199 != 0.0)) {
        let assign15620_e15814: f64 = (locals.var_fn169_calc_iq__etab).exp();
        (assign15620_e15814, 0.0, (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn3), (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn4), 0.0, 0.0, (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15620_e15816;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15620_e15816_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15620_e15816_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15620_e15816_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15620_e15816_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15620_e15816_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15620_e15816_d_n10;

        let (assign15630_e15832, assign15630_e15832_d_n2, assign15630_e15832_d_n3, assign15630_e15832_d_n4, assign15630_e15832_d_n7, assign15630_e15832_d_n9, assign15630_e15832_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 == 0.0)) && (locals.var_guard199 == 0.0)) {
        let assign15630_e15828: f64 = (locals.var_fn169_calc_iq__etab).exp();
        let assign15630_e15829: f64 = (1.0 + assign15630_e15828);
        let assign15630_e15830: f64 = (assign15630_e15829).ln();
        (assign15630_e15830, 0.0, ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn3) / assign15630_e15829), ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn4) / assign15630_e15829), 0.0, 0.0, ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn10) / assign15630_e15829),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15630_e15832;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15630_e15832_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15630_e15832_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15630_e15832_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15630_e15832_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15630_e15832_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15630_e15832_d_n10;

        let (assign15640_e15850, assign15640_e15850_d_n2, assign15640_e15850_d_n3, assign15640_e15850_d_n4, assign15640_e15850_d_n7, assign15640_e15850_d_n9, assign15640_e15850_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15640_e15838: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15640_e15840: f64 = (assign15640_e15838 * locals.var_fn169_calc_iq__type);
        let assign15640_e15842: f64 = (assign15640_e15840 * locals.var_fn169_calc_iq__cb);
        let assign15640_e15844: f64 = (assign15640_e15842 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15640_e15846: f64 = (assign15640_e15844 * locals.var_fn169_calc_iq__exparg);
        let assign15640_e15848: f64 = (assign15640_e15846 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15640_e15848, ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((((assign15640_e15840 * locals.var_fn169_calc_iq__cb_dn4) * locals.var_fn169_calc_iq__two_n_phit0) + (assign15640_e15842 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) * locals.var_fn169_calc_iq__exparg) + (assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign15640_e15850;
        locals.var_fn169_calc_iq__qbout_dn2 = assign15640_e15850_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign15640_e15850_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign15640_e15850_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign15640_e15850_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign15640_e15850_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign15640_e15850_d_n10;

        let (assign15650_e15857, assign15650_e15857_d_n2, assign15650_e15857_d_n3, assign15650_e15857_d_n4, assign15650_e15857_d_n7, assign15650_e15857_d_n9, assign15650_e15857_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign15650_e15857;
        locals.var_fn169_calc_iq__qcout_dn2 = assign15650_e15857_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign15650_e15857_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign15650_e15857_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign15650_e15857_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign15650_e15857_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign15650_e15857_d_n10;

        let (assign15660_e15864, assign15660_e15864_d_n2, assign15660_e15864_d_n3, assign15660_e15864_d_n4, assign15660_e15864_d_n7, assign15660_e15864_d_n9, assign15660_e15864_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign15660_e15864;
        locals.var_fn169_calc_iq__qbout_dn2 = assign15660_e15864_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign15660_e15864_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign15660_e15864_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign15660_e15864_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign15660_e15864_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign15660_e15864_d_n10;

        let assign15670_e15867: f64 = if locals.var_fn169_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard200 = assign15670_e15867;

        let (assign15680_e15883, assign15680_e15883_d_n2, assign15680_e15883_d_n4, assign15680_e15883_d_n7, assign15680_e15883_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) {
        let assign15680_e15875: f64 = (p.p51 * 0.5);
        let assign15680_e15877: f64 = (assign15680_e15875 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15680_e15878: f64 = (locals.var_fn169_calc_iq__vtof - assign15680_e15877);
        let assign15680_e15879: f64 = (locals.var_fn169_calc_iq__vgsin - assign15680_e15878);
        let assign15680_e15881: f64 = (assign15680_e15879 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15680_e15881, (locals.var_fn169_calc_iq__vgsin_dn2 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15680_e15875 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15680_e15879 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vgsin_dn7 / locals.var_fn169_calc_iq__two_n_phit0), (locals.var_fn169_calc_iq__vgsin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, locals.var_fn169_calc_iq__etags_dn10,)
    }
};
        locals.var_fn169_calc_iq__etags = assign15680_e15883;
        locals.var_fn169_calc_iq__etags_dn2 = assign15680_e15883_d_n2;
        locals.var_fn169_calc_iq__etags_dn4 = assign15680_e15883_d_n4;
        locals.var_fn169_calc_iq__etags_dn7 = assign15680_e15883_d_n7;
        locals.var_fn169_calc_iq__etags_dn10 = assign15680_e15883_d_n10;

        let assign15690_e15886: f64 = if locals.var_fn169_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign15690_e15886;

        let (assign15700_e15894, assign15700_e15894_d_n2, assign15700_e15894_d_n3, assign15700_e15894_d_n4, assign15700_e15894_d_n7, assign15700_e15894_d_n9, assign15700_e15894_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, 0.0, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, 0.0, locals.var_fn169_calc_iq__etags_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15700_e15894;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15700_e15894_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15700_e15894_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15700_e15894_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15700_e15894_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15700_e15894_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15700_e15894_d_n10;

        let assign15710_e15897: f64 = (-50.0);
        let assign15710_e15898: f64 = if locals.var_fn169_calc_iq__etags < assign15710_e15897 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign15710_e15898;

        let (assign15720_e15910, assign15720_e15910_d_n2, assign15720_e15910_d_n3, assign15720_e15910_d_n4, assign15720_e15910_d_n7, assign15720_e15910_d_n9, assign15720_e15910_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard202 != 0.0)) {
        let assign15720_e15908: f64 = (locals.var_fn169_calc_iq__etags).exp();
        (assign15720_e15908, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn2), 0.0, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn4), (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn7), 0.0, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15720_e15910;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15720_e15910_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15720_e15910_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15720_e15910_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15720_e15910_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15720_e15910_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15720_e15910_d_n10;

        let (assign15730_e15926, assign15730_e15926_d_n2, assign15730_e15926_d_n3, assign15730_e15926_d_n4, assign15730_e15926_d_n7, assign15730_e15926_d_n9, assign15730_e15926_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard202 == 0.0)) {
        let assign15730_e15922: f64 = (locals.var_fn169_calc_iq__etags).exp();
        let assign15730_e15923: f64 = (1.0 + assign15730_e15922);
        let assign15730_e15924: f64 = (assign15730_e15923).ln();
        (assign15730_e15924, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn2) / assign15730_e15923), 0.0, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn4) / assign15730_e15923), ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn7) / assign15730_e15923), 0.0, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn10) / assign15730_e15923),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15730_e15926;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15730_e15926_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15730_e15926_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15730_e15926_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15730_e15926_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15730_e15926_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15730_e15926_d_n10;

        let (assign15740_e15944, assign15740_e15944_d_n2, assign15740_e15944_d_n3, assign15740_e15944_d_n4, assign15740_e15944_d_n7, assign15740_e15944_d_n9, assign15740_e15944_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) {
        let assign15740_e15932: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15740_e15934: f64 = (assign15740_e15932 * locals.var_fn169_calc_iq__type);
        let assign15740_e15936: f64 = (assign15740_e15934 * locals.var_fn169_calc_iq__cs);
        let assign15740_e15938: f64 = (assign15740_e15936 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15740_e15940: f64 = (assign15740_e15938 * locals.var_fn169_calc_iq__exparg);
        let assign15740_e15942: f64 = (assign15740_e15940 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15740_e15942, ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((assign15740_e15936 * locals.var_fn169_calc_iq__two_n_phit0_dn4) * locals.var_fn169_calc_iq__exparg) + (assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign15740_e15944;
        locals.var_fn169_calc_iq__qsout_dn2 = assign15740_e15944_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign15740_e15944_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign15740_e15944_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign15740_e15944_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign15740_e15944_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign15740_e15944_d_n10;

        let (assign15750_e15951, assign15750_e15951_d_n2, assign15750_e15951_d_n3, assign15750_e15951_d_n4, assign15750_e15951_d_n7, assign15750_e15951_d_n9, assign15750_e15951_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign15750_e15951;
        locals.var_fn169_calc_iq__qsout_dn2 = assign15750_e15951_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign15750_e15951_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign15750_e15951_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign15750_e15951_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign15750_e15951_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign15750_e15951_d_n10;

        let (assign15760_e15955, assign15760_e15955_d_n2, assign15760_e15955_d_n3, assign15760_e15955_d_n4, assign15760_e15955_d_n7, assign15760_e15955_d_n9, assign15760_e15955_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__idsout, locals.var_fn169_calc_iq__idsout_dn2, locals.var_fn169_calc_iq__idsout_dn3, locals.var_fn169_calc_iq__idsout_dn4, locals.var_fn169_calc_iq__idsout_dn7, locals.var_fn169_calc_iq__idsout_dn9, locals.var_fn169_calc_iq__idsout_dn10,)
    } else {
        (locals.var_fn169_calc_iq__return, locals.var_fn169_calc_iq__return_dn2, locals.var_fn169_calc_iq__return_dn3, locals.var_fn169_calc_iq__return_dn4, locals.var_fn169_calc_iq__return_dn7, locals.var_fn169_calc_iq__return_dn9, locals.var_fn169_calc_iq__return_dn10,)
    }
};
        locals.var_fn169_calc_iq__return = assign15760_e15955;
        locals.var_fn169_calc_iq__return_dn2 = assign15760_e15955_d_n2;
        locals.var_fn169_calc_iq__return_dn3 = assign15760_e15955_d_n3;
        locals.var_fn169_calc_iq__return_dn4 = assign15760_e15955_d_n4;
        locals.var_fn169_calc_iq__return_dn7 = assign15760_e15955_d_n7;
        locals.var_fn169_calc_iq__return_dn9 = assign15760_e15955_d_n9;
        locals.var_fn169_calc_iq__return_dn10 = assign15760_e15955_d_n10;

        let (assign15770_e15959, assign15770_e15959_d_n2, assign15770_e15959_d_n3, assign15770_e15959_d_n4, assign15770_e15959_d_n7, assign15770_e15959_d_n9, assign15770_e15959_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__idsout, locals.var_fn169_calc_iq__idsout_dn2, locals.var_fn169_calc_iq__idsout_dn3, locals.var_fn169_calc_iq__idsout_dn4, locals.var_fn169_calc_iq__idsout_dn7, locals.var_fn169_calc_iq__idsout_dn9, locals.var_fn169_calc_iq__idsout_dn10,)
    } else {
        (locals.var_idsfps1, locals.var_idsfps1_dn2, locals.var_idsfps1_dn3, locals.var_idsfps1_dn4, locals.var_idsfps1_dn7, locals.var_idsfps1_dn9, locals.var_idsfps1_dn10,)
    }
};
        locals.var_idsfps1 = assign15770_e15959;
        locals.var_idsfps1_dn2 = assign15770_e15959_d_n2;
        locals.var_idsfps1_dn3 = assign15770_e15959_d_n3;
        locals.var_idsfps1_dn4 = assign15770_e15959_d_n4;
        locals.var_idsfps1_dn7 = assign15770_e15959_d_n7;
        locals.var_idsfps1_dn9 = assign15770_e15959_d_n9;
        locals.var_idsfps1_dn10 = assign15770_e15959_d_n10;

        let (assign15780_e15963, assign15780_e15963_d_n2, assign15780_e15963_d_n4, assign15780_e15963_d_n7, assign15780_e15963_d_n9, assign15780_e15963_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    } else {
        (locals.var_qgsfps1, locals.var_qgsfps1_dn2, locals.var_qgsfps1_dn4, locals.var_qgsfps1_dn7, locals.var_qgsfps1_dn9, locals.var_qgsfps1_dn10,)
    }
};
        locals.var_qgsfps1 = assign15780_e15963;
        locals.var_qgsfps1_dn2 = assign15780_e15963_d_n2;
        locals.var_qgsfps1_dn4 = assign15780_e15963_d_n4;
        locals.var_qgsfps1_dn7 = assign15780_e15963_d_n7;
        locals.var_qgsfps1_dn9 = assign15780_e15963_d_n9;
        locals.var_qgsfps1_dn10 = assign15780_e15963_d_n10;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15790_e15967, assign15790_e15967_d_n2, assign15790_e15967_d_n4, assign15790_e15967_d_n7, assign15790_e15967_d_n9, assign15790_e15967_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    } else {
        (locals.var_qgdfps1, locals.var_qgdfps1_dn2, locals.var_qgdfps1_dn4, locals.var_qgdfps1_dn7, locals.var_qgdfps1_dn9, locals.var_qgdfps1_dn10,)
    }
};
        locals.var_qgdfps1 = assign15790_e15967;
        locals.var_qgdfps1_dn2 = assign15790_e15967_d_n2;
        locals.var_qgdfps1_dn4 = assign15790_e15967_d_n4;
        locals.var_qgdfps1_dn7 = assign15790_e15967_d_n7;
        locals.var_qgdfps1_dn9 = assign15790_e15967_d_n9;
        locals.var_qgdfps1_dn10 = assign15790_e15967_d_n10;

        let (assign15800_e15971, assign15800_e15971_d_n2, assign15800_e15971_d_n3, assign15800_e15971_d_n4, assign15800_e15971_d_n7, assign15800_e15971_d_n9, assign15800_e15971_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    } else {
        (locals.var_qcfps1, locals.var_qcfps1_dn2, locals.var_qcfps1_dn3, locals.var_qcfps1_dn4, locals.var_qcfps1_dn7, locals.var_qcfps1_dn9, locals.var_qcfps1_dn10,)
    }
};
        locals.var_qcfps1 = assign15800_e15971;
        locals.var_qcfps1_dn2 = assign15800_e15971_d_n2;
        locals.var_qcfps1_dn3 = assign15800_e15971_d_n3;
        locals.var_qcfps1_dn4 = assign15800_e15971_d_n4;
        locals.var_qcfps1_dn7 = assign15800_e15971_d_n7;
        locals.var_qcfps1_dn9 = assign15800_e15971_d_n9;
        locals.var_qcfps1_dn10 = assign15800_e15971_d_n10;

        let (assign15810_e15975, assign15810_e15975_d_n2, assign15810_e15975_d_n3, assign15810_e15975_d_n4, assign15810_e15975_d_n7, assign15810_e15975_d_n9, assign15810_e15975_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    } else {
        (locals.var_qbfps1, locals.var_qbfps1_dn2, locals.var_qbfps1_dn3, locals.var_qbfps1_dn4, locals.var_qbfps1_dn7, locals.var_qbfps1_dn9, locals.var_qbfps1_dn10,)
    }
};
        locals.var_qbfps1 = assign15810_e15975;
        locals.var_qbfps1_dn2 = assign15810_e15975_d_n2;
        locals.var_qbfps1_dn3 = assign15810_e15975_d_n3;
        locals.var_qbfps1_dn4 = assign15810_e15975_d_n4;
        locals.var_qbfps1_dn7 = assign15810_e15975_d_n7;
        locals.var_qbfps1_dn9 = assign15810_e15975_d_n9;
        locals.var_qbfps1_dn10 = assign15810_e15975_d_n10;

        let (assign15820_e15979, assign15820_e15979_d_n2, assign15820_e15979_d_n3, assign15820_e15979_d_n4, assign15820_e15979_d_n7, assign15820_e15979_d_n9, assign15820_e15979_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    } else {
        (locals.var_qsfps1, locals.var_qsfps1_dn2, locals.var_qsfps1_dn3, locals.var_qsfps1_dn4, locals.var_qsfps1_dn7, locals.var_qsfps1_dn9, locals.var_qsfps1_dn10,)
    }
};
        locals.var_qsfps1 = assign15820_e15979;
        locals.var_qsfps1_dn2 = assign15820_e15979_d_n2;
        locals.var_qsfps1_dn3 = assign15820_e15979_d_n3;
        locals.var_qsfps1_dn4 = assign15820_e15979_d_n4;
        locals.var_qsfps1_dn7 = assign15820_e15979_d_n7;
        locals.var_qsfps1_dn9 = assign15820_e15979_d_n9;
        locals.var_qsfps1_dn10 = assign15820_e15979_d_n10;

        let (assign15850_e15991, assign15850_e15991_d_n2, assign15850_e15991_d_n3, assign15850_e15991_d_n4, assign15850_e15991_d_n7, assign15850_e15991_d_n9, assign15850_e15991_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__return, locals.var_fn169_calc_iq__return_dn2, locals.var_fn169_calc_iq__return_dn3, locals.var_fn169_calc_iq__return_dn4, locals.var_fn169_calc_iq__return_dn7, locals.var_fn169_calc_iq__return_dn9, locals.var_fn169_calc_iq__return_dn10,)
    } else {
        (locals.var_idsfps1, locals.var_idsfps1_dn2, locals.var_idsfps1_dn3, locals.var_idsfps1_dn4, locals.var_idsfps1_dn7, locals.var_idsfps1_dn9, locals.var_idsfps1_dn10,)
    }
};
        locals.var_idsfps1 = assign15850_e15991;
        locals.var_idsfps1_dn2 = assign15850_e15991_d_n2;
        locals.var_idsfps1_dn3 = assign15850_e15991_d_n3;
        locals.var_idsfps1_dn4 = assign15850_e15991_d_n4;
        locals.var_idsfps1_dn7 = assign15850_e15991_d_n7;
        locals.var_idsfps1_dn9 = assign15850_e15991_d_n9;
        locals.var_idsfps1_dn10 = assign15850_e15991_d_n10;

        let assign15860_e15994: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard203 = assign15860_e15994;

        locals.var_idsfps2 = 0.0;
        locals.var_idsfps2_dn2 = 0.0;
        locals.var_idsfps2_dn3 = 0.0;
        locals.var_idsfps2_dn4 = 0.0;
        locals.var_idsfps2_dn7 = 0.0;
        locals.var_idsfps2_dn10 = 0.0;
        locals.var_idsfps2_dn11 = 0.0;

        locals.var_qgsfps2 = 0.0;
        locals.var_qgsfps2_dn2 = 0.0;
        locals.var_qgsfps2_dn4 = 0.0;
        locals.var_qgsfps2_dn7 = 0.0;
        locals.var_qgsfps2_dn10 = 0.0;
        locals.var_qgsfps2_dn11 = 0.0;

        locals.var_qgdfps2 = 0.0;
        locals.var_qgdfps2_dn2 = 0.0;
        locals.var_qgdfps2_dn4 = 0.0;
        locals.var_qgdfps2_dn7 = 0.0;
        locals.var_qgdfps2_dn10 = 0.0;
        locals.var_qgdfps2_dn11 = 0.0;

        locals.var_qcfps2 = 0.0;
        locals.var_qcfps2_dn2 = 0.0;
        locals.var_qcfps2_dn3 = 0.0;
        locals.var_qcfps2_dn4 = 0.0;
        locals.var_qcfps2_dn7 = 0.0;
        locals.var_qcfps2_dn10 = 0.0;
        locals.var_qcfps2_dn11 = 0.0;

        locals.var_qbfps2 = 0.0;
        locals.var_qbfps2_dn2 = 0.0;
        locals.var_qbfps2_dn3 = 0.0;
        locals.var_qbfps2_dn4 = 0.0;
        locals.var_qbfps2_dn7 = 0.0;
        locals.var_qbfps2_dn10 = 0.0;
        locals.var_qbfps2_dn11 = 0.0;

        locals.var_qsfps2 = 0.0;
        locals.var_qsfps2_dn2 = 0.0;
        locals.var_qsfps2_dn3 = 0.0;
        locals.var_qsfps2_dn4 = 0.0;
        locals.var_qsfps2_dn7 = 0.0;
        locals.var_qsfps2_dn10 = 0.0;
        locals.var_qsfps2_dn11 = 0.0;

        let assign15950_e16005: f64 = if p.p101 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign15950_e16005;

        let (assign15960_e16009, assign15960_e16009_d_n2, assign15960_e16009_d_n3, assign15960_e16009_d_n4, assign15960_e16009_d_n7, assign15960_e16009_d_n10, assign15960_e16009_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__return, locals.var_fn205_calc_iq__return_dn2, locals.var_fn205_calc_iq__return_dn3, locals.var_fn205_calc_iq__return_dn4, locals.var_fn205_calc_iq__return_dn7, locals.var_fn205_calc_iq__return_dn10, locals.var_fn205_calc_iq__return_dn11,)
    }
};
        locals.var_fn205_calc_iq__return = assign15960_e16009;
        locals.var_fn205_calc_iq__return_dn2 = assign15960_e16009_d_n2;
        locals.var_fn205_calc_iq__return_dn3 = assign15960_e16009_d_n3;
        locals.var_fn205_calc_iq__return_dn4 = assign15960_e16009_d_n4;
        locals.var_fn205_calc_iq__return_dn7 = assign15960_e16009_d_n7;
        locals.var_fn205_calc_iq__return_dn10 = assign15960_e16009_d_n10;
        locals.var_fn205_calc_iq__return_dn11 = assign15960_e16009_d_n11;

        let (assign15970_e16013, assign15970_e16013_d_n2, assign15970_e16013_d_n3, assign15970_e16013_d_n4, assign15970_e16013_d_n7, assign15970_e16013_d_n10, assign15970_e16013_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__idsout, locals.var_fn205_calc_iq__idsout_dn2, locals.var_fn205_calc_iq__idsout_dn3, locals.var_fn205_calc_iq__idsout_dn4, locals.var_fn205_calc_iq__idsout_dn7, locals.var_fn205_calc_iq__idsout_dn10, locals.var_fn205_calc_iq__idsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__idsout = assign15970_e16013;
        locals.var_fn205_calc_iq__idsout_dn2 = assign15970_e16013_d_n2;
        locals.var_fn205_calc_iq__idsout_dn3 = assign15970_e16013_d_n3;
        locals.var_fn205_calc_iq__idsout_dn4 = assign15970_e16013_d_n4;
        locals.var_fn205_calc_iq__idsout_dn7 = assign15970_e16013_d_n7;
        locals.var_fn205_calc_iq__idsout_dn10 = assign15970_e16013_d_n10;
        locals.var_fn205_calc_iq__idsout_dn11 = assign15970_e16013_d_n11;

        let (assign15980_e16017, assign15980_e16017_d_n2, assign15980_e16017_d_n4, assign15980_e16017_d_n7, assign15980_e16017_d_n10, assign15980_e16017_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgsout = assign15980_e16017;
        locals.var_fn205_calc_iq__qgsout_dn2 = assign15980_e16017_d_n2;
        locals.var_fn205_calc_iq__qgsout_dn4 = assign15980_e16017_d_n4;
        locals.var_fn205_calc_iq__qgsout_dn7 = assign15980_e16017_d_n7;
        locals.var_fn205_calc_iq__qgsout_dn10 = assign15980_e16017_d_n10;
        locals.var_fn205_calc_iq__qgsout_dn11 = assign15980_e16017_d_n11;

        let (assign15990_e16021, assign15990_e16021_d_n2, assign15990_e16021_d_n4, assign15990_e16021_d_n7, assign15990_e16021_d_n10, assign15990_e16021_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgdout = assign15990_e16021;
        locals.var_fn205_calc_iq__qgdout_dn2 = assign15990_e16021_d_n2;
        locals.var_fn205_calc_iq__qgdout_dn4 = assign15990_e16021_d_n4;
        locals.var_fn205_calc_iq__qgdout_dn7 = assign15990_e16021_d_n7;
        locals.var_fn205_calc_iq__qgdout_dn10 = assign15990_e16021_d_n10;
        locals.var_fn205_calc_iq__qgdout_dn11 = assign15990_e16021_d_n11;

        let (assign16000_e16025, assign16000_e16025_d_n2, assign16000_e16025_d_n3, assign16000_e16025_d_n4, assign16000_e16025_d_n7, assign16000_e16025_d_n10, assign16000_e16025_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign16000_e16025;
        locals.var_fn205_calc_iq__qcout_dn2 = assign16000_e16025_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign16000_e16025_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign16000_e16025_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign16000_e16025_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign16000_e16025_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign16000_e16025_d_n11;

        let (assign16010_e16029, assign16010_e16029_d_n2, assign16010_e16029_d_n3, assign16010_e16029_d_n4, assign16010_e16029_d_n7, assign16010_e16029_d_n10, assign16010_e16029_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign16010_e16029;
        locals.var_fn205_calc_iq__qbout_dn2 = assign16010_e16029_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign16010_e16029_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign16010_e16029_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign16010_e16029_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign16010_e16029_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign16010_e16029_d_n11;

        let (assign16020_e16033, assign16020_e16033_d_n2, assign16020_e16033_d_n3, assign16020_e16033_d_n4, assign16020_e16033_d_n7, assign16020_e16033_d_n10, assign16020_e16033_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign16020_e16033;
        locals.var_fn205_calc_iq__qsout_dn2 = assign16020_e16033_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign16020_e16033_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign16020_e16033_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign16020_e16033_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign16020_e16033_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign16020_e16033_d_n11;

        let (assign16030_e16037, assign16030_e16037_d_n4, assign16030_e16037_d_n10, assign16030_e16037_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vtdibl, locals.var_fn205_calc_iq__vtdibl_dn4, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vtdibl = assign16030_e16037;
        locals.var_fn205_calc_iq__vtdibl_dn4 = assign16030_e16037_d_n4;
        locals.var_fn205_calc_iq__vtdibl_dn10 = assign16030_e16037_d_n10;
        locals.var_fn205_calc_iq__vtdibl_dn11 = assign16030_e16037_d_n11;

        let (assign16040_e16041, assign16040_e16041_d_n2, assign16040_e16041_d_n3, assign16040_e16041_d_n4, assign16040_e16041_d_n7, assign16040_e16041_d_n10, assign16040_e16041_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat1, locals.var_fn205_calc_iq__vdsat1_dn2, locals.var_fn205_calc_iq__vdsat1_dn3, locals.var_fn205_calc_iq__vdsat1_dn4, locals.var_fn205_calc_iq__vdsat1_dn7, locals.var_fn205_calc_iq__vdsat1_dn10, locals.var_fn205_calc_iq__vdsat1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat1 = assign16040_e16041;
        locals.var_fn205_calc_iq__vdsat1_dn2 = assign16040_e16041_d_n2;
        locals.var_fn205_calc_iq__vdsat1_dn3 = assign16040_e16041_d_n3;
        locals.var_fn205_calc_iq__vdsat1_dn4 = assign16040_e16041_d_n4;
        locals.var_fn205_calc_iq__vdsat1_dn7 = assign16040_e16041_d_n7;
        locals.var_fn205_calc_iq__vdsat1_dn10 = assign16040_e16041_d_n10;
        locals.var_fn205_calc_iq__vdsat1_dn11 = assign16040_e16041_d_n11;

        let (assign16050_e16045, assign16050_e16045_d_n2, assign16050_e16045_d_n7, assign16050_e16045_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vgsin, locals.var_fn205_calc_iq__vgsin_dn2, locals.var_fn205_calc_iq__vgsin_dn7, locals.var_fn205_calc_iq__vgsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgsin = assign16050_e16045;
        locals.var_fn205_calc_iq__vgsin_dn2 = assign16050_e16045_d_n2;
        locals.var_fn205_calc_iq__vgsin_dn7 = assign16050_e16045_d_n7;
        locals.var_fn205_calc_iq__vgsin_dn11 = assign16050_e16045_d_n11;

        let (assign16060_e16049, assign16060_e16049_d_n10, assign16060_e16049_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vdsfps2, locals.var_vdsfps2_dn10, locals.var_vdsfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vdsin, locals.var_fn205_calc_iq__vdsin_dn10, locals.var_fn205_calc_iq__vdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsin = assign16060_e16049;
        locals.var_fn205_calc_iq__vdsin_dn10 = assign16060_e16049_d_n10;
        locals.var_fn205_calc_iq__vdsin_dn11 = assign16060_e16049_d_n11;

        let (assign16070_e16053,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p107,)
    } else {
        (locals.var_fn205_calc_iq__qcbflag,)
    }
};
        locals.var_fn205_calc_iq__qcbflag = assign16070_e16053;

        let (assign16080_e16057, assign16080_e16057_d_n2, assign16080_e16057_d_n7, assign16080_e16057_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vcin, locals.var_fn205_calc_iq__vcin_dn2, locals.var_fn205_calc_iq__vcin_dn7, locals.var_fn205_calc_iq__vcin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vcin = assign16080_e16057;
        locals.var_fn205_calc_iq__vcin_dn2 = assign16080_e16057_d_n2;
        locals.var_fn205_calc_iq__vcin_dn7 = assign16080_e16057_d_n7;
        locals.var_fn205_calc_iq__vcin_dn11 = assign16080_e16057_d_n11;

        let (assign16090_e16061, assign16090_e16061_d_n3, assign16090_e16061_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vbfps2, locals.var_vbfps2_dn3, locals.var_vbfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vbin, locals.var_fn205_calc_iq__vbin_dn3, locals.var_fn205_calc_iq__vbin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vbin = assign16090_e16061;
        locals.var_fn205_calc_iq__vbin_dn3 = assign16090_e16061_d_n3;
        locals.var_fn205_calc_iq__vbin_dn11 = assign16090_e16061_d_n11;

        let (assign16100_e16065,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p105,)
    } else {
        (locals.var_fn205_calc_iq__qgsflag,)
    }
};
        locals.var_fn205_calc_iq__qgsflag = assign16100_e16065;

        let (assign16110_e16069, assign16110_e16069_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn205_calc_iq__tambin, locals.var_fn205_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tambin = assign16110_e16069;
        locals.var_fn205_calc_iq__tambin_dn4 = assign16110_e16069_d_n4;

        let (assign16120_e16073,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn205_calc_iq__tnomin,)
    }
};
        locals.var_fn205_calc_iq__tnomin = assign16120_e16073;

        let (assign16130_e16077, assign16130_e16077_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn205_calc_iq__phitin, locals.var_fn205_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn205_calc_iq__phitin = assign16130_e16077;
        locals.var_fn205_calc_iq__phitin_dn4 = assign16130_e16077_d_n4;

        let (assign16140_e16081,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn205_calc_iq__w,)
    }
};
        locals.var_fn205_calc_iq__w = assign16140_e16081;

        let (assign16150_e16085,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p101,)
    } else {
        (locals.var_fn205_calc_iq__lin,)
    }
};
        locals.var_fn205_calc_iq__lin = assign16150_e16085;

        let (assign16160_e16089, assign16160_e16089_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_cgfps2t, locals.var_cgfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cgin, locals.var_fn205_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn205_calc_iq__cgin = assign16160_e16089;
        locals.var_fn205_calc_iq__cgin_dn4 = assign16160_e16089_d_n4;

        let (assign16170_e16093,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p106,)
    } else {
        (locals.var_fn205_calc_iq__cs,)
    }
};
        locals.var_fn205_calc_iq__cs = assign16170_e16093;

        let (assign16180_e16097, assign16180_e16097_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_ccfps2t, locals.var_ccfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cc, locals.var_fn205_calc_iq__cc_dn4,)
    }
};
        locals.var_fn205_calc_iq__cc = assign16180_e16097;
        locals.var_fn205_calc_iq__cc_dn4 = assign16180_e16097_d_n4;

        let (assign16190_e16101, assign16190_e16101_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_cbfps2t, locals.var_cbfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cb, locals.var_fn205_calc_iq__cb_dn4,)
    }
};
        locals.var_fn205_calc_iq__cb = assign16190_e16101;
        locals.var_fn205_calc_iq__cb_dn4 = assign16190_e16101_d_n4;

        let (assign16200_e16105,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p102,)
    } else {
        (locals.var_fn205_calc_iq__vto,)
    }
};
        locals.var_fn205_calc_iq__vto = assign16200_e16105;

        let (assign16210_e16109,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p116,)
    } else {
        (locals.var_fn205_calc_iq__ss,)
    }
};
        locals.var_fn205_calc_iq__ss = assign16210_e16109;

        let (assign16220_e16113,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p115,)
    } else {
        (locals.var_fn205_calc_iq__delta1,)
    }
};
        locals.var_fn205_calc_iq__delta1 = assign16220_e16113;

        let (assign16230_e16117,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn205_calc_iq__delta2,)
    }
};
        locals.var_fn205_calc_iq__delta2 = assign16230_e16117;

        let (assign16240_e16121,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p117,)
    } else {
        (locals.var_fn205_calc_iq__nd,)
    }
};
        locals.var_fn205_calc_iq__nd = assign16240_e16121;

        let (assign16250_e16125,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_fn205_calc_iq__alpha,)
    }
};
        locals.var_fn205_calc_iq__alpha = assign16250_e16125;

        let (assign16260_e16129,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p112,)
    } else {
        (locals.var_fn205_calc_iq__vel0,)
    }
};
        locals.var_fn205_calc_iq__vel0 = assign16260_e16129;

        let (assign16270_e16133,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p113,)
    } else {
        (locals.var_fn205_calc_iq__mu0,)
    }
};
        locals.var_fn205_calc_iq__mu0 = assign16270_e16133;

        let (assign16280_e16137,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p114,)
    } else {
        (locals.var_fn205_calc_iq__beta,)
    }
};
        locals.var_fn205_calc_iq__beta = assign16280_e16137;

        let (assign16290_e16141,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p120,)
    } else {
        (locals.var_fn205_calc_iq__mtheta,)
    }
};
        locals.var_fn205_calc_iq__mtheta = assign16290_e16141;

        let (assign16300_e16145,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p119,)
    } else {
        (locals.var_fn205_calc_iq__vtheta,)
    }
};
        locals.var_fn205_calc_iq__vtheta = assign16300_e16145;

    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16310_e16149,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p118,)
    } else {
        (locals.var_fn205_calc_iq__vtzeta,)
    }
};
        locals.var_fn205_calc_iq__vtzeta = assign16310_e16149;

        let (assign16320_e16153,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn205_calc_iq__dibsat,)
    }
};
        locals.var_fn205_calc_iq__dibsat = assign16320_e16153;

        let (assign16330_e16157,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn205_calc_iq__epsilon,)
    }
};
        locals.var_fn205_calc_iq__epsilon = assign16330_e16157;

        let (assign16340_e16161,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn205_calc_iq__vzeta,)
    }
};
        locals.var_fn205_calc_iq__vzeta = assign16340_e16161;

        let (assign16350_e16165,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn205_calc_iq__lambda,)
    }
};
        locals.var_fn205_calc_iq__lambda = assign16350_e16165;

        let (assign16360_e16169,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn205_calc_iq__ngf,)
    }
};
        locals.var_fn205_calc_iq__ngf = assign16360_e16169;

        let (assign16370_e16173,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn205_calc_iq__type,)
    }
};
        locals.var_fn205_calc_iq__type = assign16370_e16173;

        let (assign16380_e16177,) = {
    if (locals.var_guard204 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn205_calc_iq__trapfracdl,)
    }
};
        locals.var_fn205_calc_iq__trapfracdl = assign16380_e16177;

        let (assign16390_e16181, assign16390_e16181_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__alpha_phit, locals.var_fn205_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn205_calc_iq__alpha_phit = assign16390_e16181;
        locals.var_fn205_calc_iq__alpha_phit_dn4 = assign16390_e16181_d_n4;

        let (assign16400_e16185, assign16400_e16185_d_n10, assign16400_e16185_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__delta, locals.var_fn205_calc_iq__delta_dn10, locals.var_fn205_calc_iq__delta_dn11,)
    }
};
        locals.var_fn205_calc_iq__delta = assign16400_e16185;
        locals.var_fn205_calc_iq__delta_dn10 = assign16400_e16185_d_n10;
        locals.var_fn205_calc_iq__delta_dn11 = assign16400_e16185_d_n11;

        let (assign16410_e16189, assign16410_e16189_d_n4, assign16410_e16189_d_n10, assign16410_e16189_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__n, locals.var_fn205_calc_iq__n_dn4, locals.var_fn205_calc_iq__n_dn10, locals.var_fn205_calc_iq__n_dn11,)
    }
};
        locals.var_fn205_calc_iq__n = assign16410_e16189;
        locals.var_fn205_calc_iq__n_dn4 = assign16410_e16189_d_n4;
        locals.var_fn205_calc_iq__n_dn10 = assign16410_e16189_d_n10;
        locals.var_fn205_calc_iq__n_dn11 = assign16410_e16189_d_n11;

        let (assign16420_e16193, assign16420_e16193_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vtof, locals.var_fn205_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn205_calc_iq__vtof = assign16420_e16193;
        locals.var_fn205_calc_iq__vtof_dn4 = assign16420_e16193_d_n4;

        let (assign16430_e16197, assign16430_e16197_d_n10, assign16430_e16197_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign16430_e16197;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign16430_e16197_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign16430_e16197_d_n11;

        let (assign16440_e16201, assign16440_e16201_d_n2, assign16440_e16201_d_n3, assign16440_e16201_d_n4, assign16440_e16201_d_n7, assign16440_e16201_d_n10, assign16440_e16201_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign16440_e16201;
        locals.var_fn205_calc_iq__ffs_dn2 = assign16440_e16201_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign16440_e16201_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign16440_e16201_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign16440_e16201_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign16440_e16201_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign16440_e16201_d_n11;

        let (assign16450_e16205, assign16450_e16205_d_n4, assign16450_e16205_d_n10, assign16450_e16205_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit, locals.var_fn205_calc_iq__two_n_phit_dn4, locals.var_fn205_calc_iq__two_n_phit_dn10, locals.var_fn205_calc_iq__two_n_phit_dn11,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit = assign16450_e16205;
        locals.var_fn205_calc_iq__two_n_phit_dn4 = assign16450_e16205_d_n4;
        locals.var_fn205_calc_iq__two_n_phit_dn10 = assign16450_e16205_d_n10;
        locals.var_fn205_calc_iq__two_n_phit_dn11 = assign16450_e16205_d_n11;

        let (assign16460_e16209, assign16460_e16209_d_n4, assign16460_e16209_d_n10, assign16460_e16209_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qref, locals.var_fn205_calc_iq__qref_dn4, locals.var_fn205_calc_iq__qref_dn10, locals.var_fn205_calc_iq__qref_dn11,)
    }
};
        locals.var_fn205_calc_iq__qref = assign16460_e16209;
        locals.var_fn205_calc_iq__qref_dn4 = assign16460_e16209_d_n4;
        locals.var_fn205_calc_iq__qref_dn10 = assign16460_e16209_d_n10;
        locals.var_fn205_calc_iq__qref_dn11 = assign16460_e16209_d_n11;

        let (assign16470_e16213, assign16470_e16213_d_n2, assign16470_e16213_d_n3, assign16470_e16213_d_n4, assign16470_e16213_d_n7, assign16470_e16213_d_n10, assign16470_e16213_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etas, locals.var_fn205_calc_iq__etas_dn2, locals.var_fn205_calc_iq__etas_dn3, locals.var_fn205_calc_iq__etas_dn4, locals.var_fn205_calc_iq__etas_dn7, locals.var_fn205_calc_iq__etas_dn10, locals.var_fn205_calc_iq__etas_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas = assign16470_e16213;
        locals.var_fn205_calc_iq__etas_dn2 = assign16470_e16213_d_n2;
        locals.var_fn205_calc_iq__etas_dn3 = assign16470_e16213_d_n3;
        locals.var_fn205_calc_iq__etas_dn4 = assign16470_e16213_d_n4;
        locals.var_fn205_calc_iq__etas_dn7 = assign16470_e16213_d_n7;
        locals.var_fn205_calc_iq__etas_dn10 = assign16470_e16213_d_n10;
        locals.var_fn205_calc_iq__etas_dn11 = assign16470_e16213_d_n11;

        let (assign16480_e16217, assign16480_e16217_d_n2, assign16480_e16217_d_n3, assign16480_e16217_d_n4, assign16480_e16217_d_n7, assign16480_e16217_d_n10, assign16480_e16217_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign16480_e16217;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign16480_e16217_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign16480_e16217_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign16480_e16217_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign16480_e16217_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign16480_e16217_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign16480_e16217_d_n11;

        let (assign16490_e16221, assign16490_e16221_d_n2, assign16490_e16221_d_n3, assign16490_e16221_d_n4, assign16490_e16221_d_n7, assign16490_e16221_d_n10, assign16490_e16221_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__muf, locals.var_fn205_calc_iq__muf_dn2, locals.var_fn205_calc_iq__muf_dn3, locals.var_fn205_calc_iq__muf_dn4, locals.var_fn205_calc_iq__muf_dn7, locals.var_fn205_calc_iq__muf_dn10, locals.var_fn205_calc_iq__muf_dn11,)
    }
};
        locals.var_fn205_calc_iq__muf = assign16490_e16221;
        locals.var_fn205_calc_iq__muf_dn2 = assign16490_e16221_d_n2;
        locals.var_fn205_calc_iq__muf_dn3 = assign16490_e16221_d_n3;
        locals.var_fn205_calc_iq__muf_dn4 = assign16490_e16221_d_n4;
        locals.var_fn205_calc_iq__muf_dn7 = assign16490_e16221_d_n7;
        locals.var_fn205_calc_iq__muf_dn10 = assign16490_e16221_d_n10;
        locals.var_fn205_calc_iq__muf_dn11 = assign16490_e16221_d_n11;

        let (assign16500_e16225, assign16500_e16225_d_n2, assign16500_e16225_d_n3, assign16500_e16225_d_n4, assign16500_e16225_d_n7, assign16500_e16225_d_n10, assign16500_e16225_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vx, locals.var_fn205_calc_iq__vx_dn2, locals.var_fn205_calc_iq__vx_dn3, locals.var_fn205_calc_iq__vx_dn4, locals.var_fn205_calc_iq__vx_dn7, locals.var_fn205_calc_iq__vx_dn10, locals.var_fn205_calc_iq__vx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vx = assign16500_e16225;
        locals.var_fn205_calc_iq__vx_dn2 = assign16500_e16225_d_n2;
        locals.var_fn205_calc_iq__vx_dn3 = assign16500_e16225_d_n3;
        locals.var_fn205_calc_iq__vx_dn4 = assign16500_e16225_d_n4;
        locals.var_fn205_calc_iq__vx_dn7 = assign16500_e16225_d_n7;
        locals.var_fn205_calc_iq__vx_dn10 = assign16500_e16225_d_n10;
        locals.var_fn205_calc_iq__vx_dn11 = assign16500_e16225_d_n11;

        let (assign16510_e16229, assign16510_e16229_d_n2, assign16510_e16229_d_n3, assign16510_e16229_d_n4, assign16510_e16229_d_n7, assign16510_e16229_d_n10, assign16510_e16229_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vxf, locals.var_fn205_calc_iq__vxf_dn2, locals.var_fn205_calc_iq__vxf_dn3, locals.var_fn205_calc_iq__vxf_dn4, locals.var_fn205_calc_iq__vxf_dn7, locals.var_fn205_calc_iq__vxf_dn10, locals.var_fn205_calc_iq__vxf_dn11,)
    }
};
        locals.var_fn205_calc_iq__vxf = assign16510_e16229;
        locals.var_fn205_calc_iq__vxf_dn2 = assign16510_e16229_d_n2;
        locals.var_fn205_calc_iq__vxf_dn3 = assign16510_e16229_d_n3;
        locals.var_fn205_calc_iq__vxf_dn4 = assign16510_e16229_d_n4;
        locals.var_fn205_calc_iq__vxf_dn7 = assign16510_e16229_d_n7;
        locals.var_fn205_calc_iq__vxf_dn10 = assign16510_e16229_d_n10;
        locals.var_fn205_calc_iq__vxf_dn11 = assign16510_e16229_d_n11;

        let (assign16520_e16233, assign16520_e16233_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__n0, locals.var_fn205_calc_iq__n0_dn4,)
    }
};
        locals.var_fn205_calc_iq__n0 = assign16520_e16233;
        locals.var_fn205_calc_iq__n0_dn4 = assign16520_e16233_d_n4;

        let (assign16530_e16237, assign16530_e16237_d_n2, assign16530_e16237_d_n4, assign16530_e16237_d_n7, assign16530_e16237_d_n10, assign16530_e16237_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign16530_e16237;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign16530_e16237_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign16530_e16237_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign16530_e16237_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign16530_e16237_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign16530_e16237_d_n11;

        let (assign16540_e16241, assign16540_e16241_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit0, locals.var_fn205_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit0 = assign16540_e16241;
        locals.var_fn205_calc_iq__two_n_phit0_dn4 = assign16540_e16241_d_n4;

        let (assign16550_e16245, assign16550_e16245_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qref0, locals.var_fn205_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn205_calc_iq__qref0 = assign16550_e16245;
        locals.var_fn205_calc_iq__qref0_dn4 = assign16550_e16245_d_n4;

        let (assign16560_e16249, assign16560_e16249_d_n2, assign16560_e16249_d_n4, assign16560_e16249_d_n7, assign16560_e16249_d_n10, assign16560_e16249_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etas0, locals.var_fn205_calc_iq__etas0_dn2, locals.var_fn205_calc_iq__etas0_dn4, locals.var_fn205_calc_iq__etas0_dn7, locals.var_fn205_calc_iq__etas0_dn10, locals.var_fn205_calc_iq__etas0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas0 = assign16560_e16249;
        locals.var_fn205_calc_iq__etas0_dn2 = assign16560_e16249_d_n2;
        locals.var_fn205_calc_iq__etas0_dn4 = assign16560_e16249_d_n4;
        locals.var_fn205_calc_iq__etas0_dn7 = assign16560_e16249_d_n7;
        locals.var_fn205_calc_iq__etas0_dn10 = assign16560_e16249_d_n10;
        locals.var_fn205_calc_iq__etas0_dn11 = assign16560_e16249_d_n11;

        let (assign16570_e16253, assign16570_e16253_d_n2, assign16570_e16253_d_n4, assign16570_e16253_d_n7, assign16570_e16253_d_n10, assign16570_e16253_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign16570_e16253;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign16570_e16253_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign16570_e16253_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign16570_e16253_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign16570_e16253_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign16570_e16253_d_n11;

        let (assign16580_e16257, assign16580_e16257_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__muf0, locals.var_fn205_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn205_calc_iq__muf0 = assign16580_e16257;
        locals.var_fn205_calc_iq__muf0_dn4 = assign16580_e16257_d_n4;

        let (assign16590_e16261, assign16590_e16261_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vx0, locals.var_fn205_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vx0 = assign16590_e16261;
        locals.var_fn205_calc_iq__vx0_dn4 = assign16590_e16261_d_n4;

        let (assign16600_e16265, assign16600_e16265_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__tfacmobin, locals.var_fn205_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tfacmobin = assign16600_e16265;
        locals.var_fn205_calc_iq__tfacmobin_dn4 = assign16600_e16265_d_n4;

        let (assign16610_e16269, assign16610_e16269_d_n2, assign16610_e16269_d_n3, assign16610_e16269_d_n4, assign16610_e16269_d_n7, assign16610_e16269_d_n10, assign16610_e16269_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign16610_e16269;
        locals.var_fn205_calc_iq__ff_dn2 = assign16610_e16269_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign16610_e16269_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign16610_e16269_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign16610_e16269_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign16610_e16269_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign16610_e16269_d_n11;

        let (assign16620_e16273, assign16620_e16273_d_n2, assign16620_e16273_d_n3, assign16620_e16273_d_n4, assign16620_e16273_d_n7, assign16620_e16273_d_n10, assign16620_e16273_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__eta, locals.var_fn205_calc_iq__eta_dn2, locals.var_fn205_calc_iq__eta_dn3, locals.var_fn205_calc_iq__eta_dn4, locals.var_fn205_calc_iq__eta_dn7, locals.var_fn205_calc_iq__eta_dn10, locals.var_fn205_calc_iq__eta_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta = assign16620_e16273;
        locals.var_fn205_calc_iq__eta_dn2 = assign16620_e16273_d_n2;
        locals.var_fn205_calc_iq__eta_dn3 = assign16620_e16273_d_n3;
        locals.var_fn205_calc_iq__eta_dn4 = assign16620_e16273_d_n4;
        locals.var_fn205_calc_iq__eta_dn7 = assign16620_e16273_d_n7;
        locals.var_fn205_calc_iq__eta_dn10 = assign16620_e16273_d_n10;
        locals.var_fn205_calc_iq__eta_dn11 = assign16620_e16273_d_n11;

        let (assign16630_e16277, assign16630_e16277_d_n2, assign16630_e16277_d_n3, assign16630_e16277_d_n4, assign16630_e16277_d_n7, assign16630_e16277_d_n10, assign16630_e16277_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign16630_e16277;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign16630_e16277_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign16630_e16277_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign16630_e16277_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign16630_e16277_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign16630_e16277_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign16630_e16277_d_n11;

        let (assign16640_e16281, assign16640_e16281_d_n2, assign16640_e16281_d_n4, assign16640_e16281_d_n7, assign16640_e16281_d_n10, assign16640_e16281_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign16640_e16281;
        locals.var_fn205_calc_iq__ff0_dn2 = assign16640_e16281_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign16640_e16281_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign16640_e16281_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign16640_e16281_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign16640_e16281_d_n11;

        let (assign16650_e16285, assign16650_e16285_d_n2, assign16650_e16285_d_n4, assign16650_e16285_d_n7, assign16650_e16285_d_n10, assign16650_e16285_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__eta0, locals.var_fn205_calc_iq__eta0_dn2, locals.var_fn205_calc_iq__eta0_dn4, locals.var_fn205_calc_iq__eta0_dn7, locals.var_fn205_calc_iq__eta0_dn10, locals.var_fn205_calc_iq__eta0_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta0 = assign16650_e16285;
        locals.var_fn205_calc_iq__eta0_dn2 = assign16650_e16285_d_n2;
        locals.var_fn205_calc_iq__eta0_dn4 = assign16650_e16285_d_n4;
        locals.var_fn205_calc_iq__eta0_dn7 = assign16650_e16285_d_n7;
        locals.var_fn205_calc_iq__eta0_dn10 = assign16650_e16285_d_n10;
        locals.var_fn205_calc_iq__eta0_dn11 = assign16650_e16285_d_n11;

        let (assign16660_e16289, assign16660_e16289_d_n2, assign16660_e16289_d_n4, assign16660_e16289_d_n7, assign16660_e16289_d_n10, assign16660_e16289_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign16660_e16289;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign16660_e16289_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign16660_e16289_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign16660_e16289_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign16660_e16289_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign16660_e16289_d_n11;

        let (assign16670_e16293, assign16670_e16293_d_n2, assign16670_e16293_d_n3, assign16670_e16293_d_n4, assign16670_e16293_d_n7, assign16670_e16293_d_n10, assign16670_e16293_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats, locals.var_fn205_calc_iq__vdsats_dn2, locals.var_fn205_calc_iq__vdsats_dn3, locals.var_fn205_calc_iq__vdsats_dn4, locals.var_fn205_calc_iq__vdsats_dn7, locals.var_fn205_calc_iq__vdsats_dn10, locals.var_fn205_calc_iq__vdsats_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats = assign16670_e16293;
        locals.var_fn205_calc_iq__vdsats_dn2 = assign16670_e16293_d_n2;
        locals.var_fn205_calc_iq__vdsats_dn3 = assign16670_e16293_d_n3;
        locals.var_fn205_calc_iq__vdsats_dn4 = assign16670_e16293_d_n4;
        locals.var_fn205_calc_iq__vdsats_dn7 = assign16670_e16293_d_n7;
        locals.var_fn205_calc_iq__vdsats_dn10 = assign16670_e16293_d_n10;
        locals.var_fn205_calc_iq__vdsats_dn11 = assign16670_e16293_d_n11;

        let (assign16680_e16297, assign16680_e16297_d_n2, assign16680_e16297_d_n3, assign16680_e16297_d_n4, assign16680_e16297_d_n7, assign16680_e16297_d_n10, assign16680_e16297_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats1, locals.var_fn205_calc_iq__vdsats1_dn2, locals.var_fn205_calc_iq__vdsats1_dn3, locals.var_fn205_calc_iq__vdsats1_dn4, locals.var_fn205_calc_iq__vdsats1_dn7, locals.var_fn205_calc_iq__vdsats1_dn10, locals.var_fn205_calc_iq__vdsats1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats1 = assign16680_e16297;
        locals.var_fn205_calc_iq__vdsats1_dn2 = assign16680_e16297_d_n2;
        locals.var_fn205_calc_iq__vdsats1_dn3 = assign16680_e16297_d_n3;
        locals.var_fn205_calc_iq__vdsats1_dn4 = assign16680_e16297_d_n4;
        locals.var_fn205_calc_iq__vdsats1_dn7 = assign16680_e16297_d_n7;
        locals.var_fn205_calc_iq__vdsats1_dn10 = assign16680_e16297_d_n10;
        locals.var_fn205_calc_iq__vdsats1_dn11 = assign16680_e16297_d_n11;

        let (assign16690_e16301, assign16690_e16301_d_n2, assign16690_e16301_d_n3, assign16690_e16301_d_n4, assign16690_e16301_d_n7, assign16690_e16301_d_n10, assign16690_e16301_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat, locals.var_fn205_calc_iq__vdsat_dn2, locals.var_fn205_calc_iq__vdsat_dn3, locals.var_fn205_calc_iq__vdsat_dn4, locals.var_fn205_calc_iq__vdsat_dn7, locals.var_fn205_calc_iq__vdsat_dn10, locals.var_fn205_calc_iq__vdsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat = assign16690_e16301;
        locals.var_fn205_calc_iq__vdsat_dn2 = assign16690_e16301_d_n2;
        locals.var_fn205_calc_iq__vdsat_dn3 = assign16690_e16301_d_n3;
        locals.var_fn205_calc_iq__vdsat_dn4 = assign16690_e16301_d_n4;
        locals.var_fn205_calc_iq__vdsat_dn7 = assign16690_e16301_d_n7;
        locals.var_fn205_calc_iq__vdsat_dn10 = assign16690_e16301_d_n10;
        locals.var_fn205_calc_iq__vdsat_dn11 = assign16690_e16301_d_n11;

        let (assign16700_e16305, assign16700_e16305_d_n2, assign16700_e16305_d_n3, assign16700_e16305_d_n4, assign16700_e16305_d_n7, assign16700_e16305_d_n10, assign16700_e16305_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fsd, locals.var_fn205_calc_iq__fsd_dn2, locals.var_fn205_calc_iq__fsd_dn3, locals.var_fn205_calc_iq__fsd_dn4, locals.var_fn205_calc_iq__fsd_dn7, locals.var_fn205_calc_iq__fsd_dn10, locals.var_fn205_calc_iq__fsd_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd = assign16700_e16305;
        locals.var_fn205_calc_iq__fsd_dn2 = assign16700_e16305_d_n2;
        locals.var_fn205_calc_iq__fsd_dn3 = assign16700_e16305_d_n3;
        locals.var_fn205_calc_iq__fsd_dn4 = assign16700_e16305_d_n4;
        locals.var_fn205_calc_iq__fsd_dn7 = assign16700_e16305_d_n7;
        locals.var_fn205_calc_iq__fsd_dn10 = assign16700_e16305_d_n10;
        locals.var_fn205_calc_iq__fsd_dn11 = assign16700_e16305_d_n11;

        let (assign16710_e16309, assign16710_e16309_d_n2, assign16710_e16309_d_n3, assign16710_e16309_d_n4, assign16710_e16309_d_n7, assign16710_e16309_d_n10, assign16710_e16309_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdx, locals.var_fn205_calc_iq__vdx_dn2, locals.var_fn205_calc_iq__vdx_dn3, locals.var_fn205_calc_iq__vdx_dn4, locals.var_fn205_calc_iq__vdx_dn7, locals.var_fn205_calc_iq__vdx_dn10, locals.var_fn205_calc_iq__vdx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx = assign16710_e16309;
        locals.var_fn205_calc_iq__vdx_dn2 = assign16710_e16309_d_n2;
        locals.var_fn205_calc_iq__vdx_dn3 = assign16710_e16309_d_n3;
        locals.var_fn205_calc_iq__vdx_dn4 = assign16710_e16309_d_n4;
        locals.var_fn205_calc_iq__vdx_dn7 = assign16710_e16309_d_n7;
        locals.var_fn205_calc_iq__vdx_dn10 = assign16710_e16309_d_n10;
        locals.var_fn205_calc_iq__vdx_dn11 = assign16710_e16309_d_n11;

    }

    pub(super) fn stamp_transient_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign16720_e16313, assign16720_e16313_d_n2, assign16720_e16313_d_n3, assign16720_e16313_d_n4, assign16720_e16313_d_n7, assign16720_e16313_d_n10, assign16720_e16313_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fds, locals.var_fn205_calc_iq__fds_dn2, locals.var_fn205_calc_iq__fds_dn3, locals.var_fn205_calc_iq__fds_dn4, locals.var_fn205_calc_iq__fds_dn7, locals.var_fn205_calc_iq__fds_dn10, locals.var_fn205_calc_iq__fds_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds = assign16720_e16313;
        locals.var_fn205_calc_iq__fds_dn2 = assign16720_e16313_d_n2;
        locals.var_fn205_calc_iq__fds_dn3 = assign16720_e16313_d_n3;
        locals.var_fn205_calc_iq__fds_dn4 = assign16720_e16313_d_n4;
        locals.var_fn205_calc_iq__fds_dn7 = assign16720_e16313_d_n7;
        locals.var_fn205_calc_iq__fds_dn10 = assign16720_e16313_d_n10;
        locals.var_fn205_calc_iq__fds_dn11 = assign16720_e16313_d_n11;

        let (assign16730_e16317, assign16730_e16317_d_n2, assign16730_e16317_d_n3, assign16730_e16317_d_n4, assign16730_e16317_d_n7, assign16730_e16317_d_n10, assign16730_e16317_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsx, locals.var_fn205_calc_iq__vsx_dn2, locals.var_fn205_calc_iq__vsx_dn3, locals.var_fn205_calc_iq__vsx_dn4, locals.var_fn205_calc_iq__vsx_dn7, locals.var_fn205_calc_iq__vsx_dn10, locals.var_fn205_calc_iq__vsx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx = assign16730_e16317;
        locals.var_fn205_calc_iq__vsx_dn2 = assign16730_e16317_d_n2;
        locals.var_fn205_calc_iq__vsx_dn3 = assign16730_e16317_d_n3;
        locals.var_fn205_calc_iq__vsx_dn4 = assign16730_e16317_d_n4;
        locals.var_fn205_calc_iq__vsx_dn7 = assign16730_e16317_d_n7;
        locals.var_fn205_calc_iq__vsx_dn10 = assign16730_e16317_d_n10;
        locals.var_fn205_calc_iq__vsx_dn11 = assign16730_e16317_d_n11;

        let (assign16740_e16321, assign16740_e16321_d_n2, assign16740_e16321_d_n3, assign16740_e16321_d_n4, assign16740_e16321_d_n7, assign16740_e16321_d_n10, assign16740_e16321_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign16740_e16321;
        locals.var_fn205_calc_iq__ffd_dn2 = assign16740_e16321_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign16740_e16321_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign16740_e16321_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign16740_e16321_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign16740_e16321_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign16740_e16321_d_n11;

        let (assign16750_e16325, assign16750_e16325_d_n2, assign16750_e16325_d_n3, assign16750_e16325_d_n4, assign16750_e16325_d_n7, assign16750_e16325_d_n10, assign16750_e16325_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etad, locals.var_fn205_calc_iq__etad_dn2, locals.var_fn205_calc_iq__etad_dn3, locals.var_fn205_calc_iq__etad_dn4, locals.var_fn205_calc_iq__etad_dn7, locals.var_fn205_calc_iq__etad_dn10, locals.var_fn205_calc_iq__etad_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad = assign16750_e16325;
        locals.var_fn205_calc_iq__etad_dn2 = assign16750_e16325_d_n2;
        locals.var_fn205_calc_iq__etad_dn3 = assign16750_e16325_d_n3;
        locals.var_fn205_calc_iq__etad_dn4 = assign16750_e16325_d_n4;
        locals.var_fn205_calc_iq__etad_dn7 = assign16750_e16325_d_n7;
        locals.var_fn205_calc_iq__etad_dn10 = assign16750_e16325_d_n10;
        locals.var_fn205_calc_iq__etad_dn11 = assign16750_e16325_d_n11;

        let (assign16760_e16329, assign16760_e16329_d_n2, assign16760_e16329_d_n3, assign16760_e16329_d_n4, assign16760_e16329_d_n7, assign16760_e16329_d_n10, assign16760_e16329_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign16760_e16329;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign16760_e16329_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign16760_e16329_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign16760_e16329_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign16760_e16329_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign16760_e16329_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign16760_e16329_d_n11;

        let (assign16770_e16333, assign16770_e16333_d_n2, assign16770_e16333_d_n3, assign16770_e16333_d_n4, assign16770_e16333_d_n7, assign16770_e16333_d_n10, assign16770_e16333_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsc, locals.var_fn205_calc_iq__vdsc_dn2, locals.var_fn205_calc_iq__vdsc_dn3, locals.var_fn205_calc_iq__vdsc_dn4, locals.var_fn205_calc_iq__vdsc_dn7, locals.var_fn205_calc_iq__vdsc_dn10, locals.var_fn205_calc_iq__vdsc_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsc = assign16770_e16333;
        locals.var_fn205_calc_iq__vdsc_dn2 = assign16770_e16333_d_n2;
        locals.var_fn205_calc_iq__vdsc_dn3 = assign16770_e16333_d_n3;
        locals.var_fn205_calc_iq__vdsc_dn4 = assign16770_e16333_d_n4;
        locals.var_fn205_calc_iq__vdsc_dn7 = assign16770_e16333_d_n7;
        locals.var_fn205_calc_iq__vdsc_dn10 = assign16770_e16333_d_n10;
        locals.var_fn205_calc_iq__vdsc_dn11 = assign16770_e16333_d_n11;

        let (assign16780_e16337, assign16780_e16337_d_n2, assign16780_e16337_d_n3, assign16780_e16337_d_n4, assign16780_e16337_d_n7, assign16780_e16337_d_n10, assign16780_e16337_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fsat, locals.var_fn205_calc_iq__fsat_dn2, locals.var_fn205_calc_iq__fsat_dn3, locals.var_fn205_calc_iq__fsat_dn4, locals.var_fn205_calc_iq__fsat_dn7, locals.var_fn205_calc_iq__fsat_dn10, locals.var_fn205_calc_iq__fsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsat = assign16780_e16337;
        locals.var_fn205_calc_iq__fsat_dn2 = assign16780_e16337_d_n2;
        locals.var_fn205_calc_iq__fsat_dn3 = assign16780_e16337_d_n3;
        locals.var_fn205_calc_iq__fsat_dn4 = assign16780_e16337_d_n4;
        locals.var_fn205_calc_iq__fsat_dn7 = assign16780_e16337_d_n7;
        locals.var_fn205_calc_iq__fsat_dn10 = assign16780_e16337_d_n10;
        locals.var_fn205_calc_iq__fsat_dn11 = assign16780_e16337_d_n11;

        let (assign16790_e16341, assign16790_e16341_d_n2, assign16790_e16341_d_n3, assign16790_e16341_d_n4, assign16790_e16341_d_n7, assign16790_e16341_d_n10, assign16790_e16341_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vel, locals.var_fn205_calc_iq__vel_dn2, locals.var_fn205_calc_iq__vel_dn3, locals.var_fn205_calc_iq__vel_dn4, locals.var_fn205_calc_iq__vel_dn7, locals.var_fn205_calc_iq__vel_dn10, locals.var_fn205_calc_iq__vel_dn11,)
    }
};
        locals.var_fn205_calc_iq__vel = assign16790_e16341;
        locals.var_fn205_calc_iq__vel_dn2 = assign16790_e16341_d_n2;
        locals.var_fn205_calc_iq__vel_dn3 = assign16790_e16341_d_n3;
        locals.var_fn205_calc_iq__vel_dn4 = assign16790_e16341_d_n4;
        locals.var_fn205_calc_iq__vel_dn7 = assign16790_e16341_d_n7;
        locals.var_fn205_calc_iq__vel_dn10 = assign16790_e16341_d_n10;
        locals.var_fn205_calc_iq__vel_dn11 = assign16790_e16341_d_n11;

        let (assign16800_e16345, assign16800_e16345_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats0, locals.var_fn205_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vdsats0 = assign16800_e16345;
        locals.var_fn205_calc_iq__vdsats0_dn4 = assign16800_e16345_d_n4;

        let (assign16810_e16349, assign16810_e16349_d_n2, assign16810_e16349_d_n4, assign16810_e16349_d_n7, assign16810_e16349_d_n10, assign16810_e16349_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats10, locals.var_fn205_calc_iq__vdsats10_dn2, locals.var_fn205_calc_iq__vdsats10_dn4, locals.var_fn205_calc_iq__vdsats10_dn7, locals.var_fn205_calc_iq__vdsats10_dn10, locals.var_fn205_calc_iq__vdsats10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats10 = assign16810_e16349;
        locals.var_fn205_calc_iq__vdsats10_dn2 = assign16810_e16349_d_n2;
        locals.var_fn205_calc_iq__vdsats10_dn4 = assign16810_e16349_d_n4;
        locals.var_fn205_calc_iq__vdsats10_dn7 = assign16810_e16349_d_n7;
        locals.var_fn205_calc_iq__vdsats10_dn10 = assign16810_e16349_d_n10;
        locals.var_fn205_calc_iq__vdsats10_dn11 = assign16810_e16349_d_n11;

        let (assign16820_e16353, assign16820_e16353_d_n2, assign16820_e16353_d_n4, assign16820_e16353_d_n7, assign16820_e16353_d_n10, assign16820_e16353_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat10, locals.var_fn205_calc_iq__vdsat10_dn2, locals.var_fn205_calc_iq__vdsat10_dn4, locals.var_fn205_calc_iq__vdsat10_dn7, locals.var_fn205_calc_iq__vdsat10_dn10, locals.var_fn205_calc_iq__vdsat10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat10 = assign16820_e16353;
        locals.var_fn205_calc_iq__vdsat10_dn2 = assign16820_e16353_d_n2;
        locals.var_fn205_calc_iq__vdsat10_dn4 = assign16820_e16353_d_n4;
        locals.var_fn205_calc_iq__vdsat10_dn7 = assign16820_e16353_d_n7;
        locals.var_fn205_calc_iq__vdsat10_dn10 = assign16820_e16353_d_n10;
        locals.var_fn205_calc_iq__vdsat10_dn11 = assign16820_e16353_d_n11;

        let (assign16830_e16357, assign16830_e16357_d_n2, assign16830_e16357_d_n4, assign16830_e16357_d_n7, assign16830_e16357_d_n10, assign16830_e16357_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fsd0, locals.var_fn205_calc_iq__fsd0_dn2, locals.var_fn205_calc_iq__fsd0_dn4, locals.var_fn205_calc_iq__fsd0_dn7, locals.var_fn205_calc_iq__fsd0_dn10, locals.var_fn205_calc_iq__fsd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd0 = assign16830_e16357;
        locals.var_fn205_calc_iq__fsd0_dn2 = assign16830_e16357_d_n2;
        locals.var_fn205_calc_iq__fsd0_dn4 = assign16830_e16357_d_n4;
        locals.var_fn205_calc_iq__fsd0_dn7 = assign16830_e16357_d_n7;
        locals.var_fn205_calc_iq__fsd0_dn10 = assign16830_e16357_d_n10;
        locals.var_fn205_calc_iq__fsd0_dn11 = assign16830_e16357_d_n11;

        let (assign16840_e16361, assign16840_e16361_d_n2, assign16840_e16361_d_n4, assign16840_e16361_d_n7, assign16840_e16361_d_n10, assign16840_e16361_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdx0, locals.var_fn205_calc_iq__vdx0_dn2, locals.var_fn205_calc_iq__vdx0_dn4, locals.var_fn205_calc_iq__vdx0_dn7, locals.var_fn205_calc_iq__vdx0_dn10, locals.var_fn205_calc_iq__vdx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx0 = assign16840_e16361;
        locals.var_fn205_calc_iq__vdx0_dn2 = assign16840_e16361_d_n2;
        locals.var_fn205_calc_iq__vdx0_dn4 = assign16840_e16361_d_n4;
        locals.var_fn205_calc_iq__vdx0_dn7 = assign16840_e16361_d_n7;
        locals.var_fn205_calc_iq__vdx0_dn10 = assign16840_e16361_d_n10;
        locals.var_fn205_calc_iq__vdx0_dn11 = assign16840_e16361_d_n11;

        let (assign16850_e16365, assign16850_e16365_d_n2, assign16850_e16365_d_n4, assign16850_e16365_d_n7, assign16850_e16365_d_n10, assign16850_e16365_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fds0, locals.var_fn205_calc_iq__fds0_dn2, locals.var_fn205_calc_iq__fds0_dn4, locals.var_fn205_calc_iq__fds0_dn7, locals.var_fn205_calc_iq__fds0_dn10, locals.var_fn205_calc_iq__fds0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds0 = assign16850_e16365;
        locals.var_fn205_calc_iq__fds0_dn2 = assign16850_e16365_d_n2;
        locals.var_fn205_calc_iq__fds0_dn4 = assign16850_e16365_d_n4;
        locals.var_fn205_calc_iq__fds0_dn7 = assign16850_e16365_d_n7;
        locals.var_fn205_calc_iq__fds0_dn10 = assign16850_e16365_d_n10;
        locals.var_fn205_calc_iq__fds0_dn11 = assign16850_e16365_d_n11;

        let (assign16860_e16369, assign16860_e16369_d_n2, assign16860_e16369_d_n4, assign16860_e16369_d_n7, assign16860_e16369_d_n10, assign16860_e16369_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsx0, locals.var_fn205_calc_iq__vsx0_dn2, locals.var_fn205_calc_iq__vsx0_dn4, locals.var_fn205_calc_iq__vsx0_dn7, locals.var_fn205_calc_iq__vsx0_dn10, locals.var_fn205_calc_iq__vsx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx0 = assign16860_e16369;
        locals.var_fn205_calc_iq__vsx0_dn2 = assign16860_e16369_d_n2;
        locals.var_fn205_calc_iq__vsx0_dn4 = assign16860_e16369_d_n4;
        locals.var_fn205_calc_iq__vsx0_dn7 = assign16860_e16369_d_n7;
        locals.var_fn205_calc_iq__vsx0_dn10 = assign16860_e16369_d_n10;
        locals.var_fn205_calc_iq__vsx0_dn11 = assign16860_e16369_d_n11;

        let (assign16870_e16373, assign16870_e16373_d_n2, assign16870_e16373_d_n4, assign16870_e16373_d_n7, assign16870_e16373_d_n10, assign16870_e16373_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign16870_e16373;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign16870_e16373_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign16870_e16373_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign16870_e16373_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign16870_e16373_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign16870_e16373_d_n11;

        let (assign16880_e16377, assign16880_e16377_d_n2, assign16880_e16377_d_n4, assign16880_e16377_d_n7, assign16880_e16377_d_n10, assign16880_e16377_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etad0, locals.var_fn205_calc_iq__etad0_dn2, locals.var_fn205_calc_iq__etad0_dn4, locals.var_fn205_calc_iq__etad0_dn7, locals.var_fn205_calc_iq__etad0_dn10, locals.var_fn205_calc_iq__etad0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad0 = assign16880_e16377;
        locals.var_fn205_calc_iq__etad0_dn2 = assign16880_e16377_d_n2;
        locals.var_fn205_calc_iq__etad0_dn4 = assign16880_e16377_d_n4;
        locals.var_fn205_calc_iq__etad0_dn7 = assign16880_e16377_d_n7;
        locals.var_fn205_calc_iq__etad0_dn10 = assign16880_e16377_d_n10;
        locals.var_fn205_calc_iq__etad0_dn11 = assign16880_e16377_d_n11;

        let (assign16890_e16381, assign16890_e16381_d_n2, assign16890_e16381_d_n4, assign16890_e16381_d_n7, assign16890_e16381_d_n10, assign16890_e16381_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign16890_e16381;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign16890_e16381_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign16890_e16381_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign16890_e16381_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign16890_e16381_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign16890_e16381_d_n11;

        let (assign16900_e16385, assign16900_e16385_d_n2, assign16900_e16385_d_n4, assign16900_e16385_d_n7, assign16900_e16385_d_n10, assign16900_e16385_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs2, locals.var_fn205_calc_iq__qs2_dn2, locals.var_fn205_calc_iq__qs2_dn4, locals.var_fn205_calc_iq__qs2_dn7, locals.var_fn205_calc_iq__qs2_dn10, locals.var_fn205_calc_iq__qs2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs2 = assign16900_e16385;
        locals.var_fn205_calc_iq__qs2_dn2 = assign16900_e16385_d_n2;
        locals.var_fn205_calc_iq__qs2_dn4 = assign16900_e16385_d_n4;
        locals.var_fn205_calc_iq__qs2_dn7 = assign16900_e16385_d_n7;
        locals.var_fn205_calc_iq__qs2_dn10 = assign16900_e16385_d_n10;
        locals.var_fn205_calc_iq__qs2_dn11 = assign16900_e16385_d_n11;

        let (assign16910_e16389, assign16910_e16389_d_n2, assign16910_e16389_d_n4, assign16910_e16389_d_n7, assign16910_e16389_d_n10, assign16910_e16389_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs3, locals.var_fn205_calc_iq__qs3_dn2, locals.var_fn205_calc_iq__qs3_dn4, locals.var_fn205_calc_iq__qs3_dn7, locals.var_fn205_calc_iq__qs3_dn10, locals.var_fn205_calc_iq__qs3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs3 = assign16910_e16389;
        locals.var_fn205_calc_iq__qs3_dn2 = assign16910_e16389_d_n2;
        locals.var_fn205_calc_iq__qs3_dn4 = assign16910_e16389_d_n4;
        locals.var_fn205_calc_iq__qs3_dn7 = assign16910_e16389_d_n7;
        locals.var_fn205_calc_iq__qs3_dn10 = assign16910_e16389_d_n10;
        locals.var_fn205_calc_iq__qs3_dn11 = assign16910_e16389_d_n11;

        let (assign16920_e16393, assign16920_e16393_d_n2, assign16920_e16393_d_n4, assign16920_e16393_d_n7, assign16920_e16393_d_n10, assign16920_e16393_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd2, locals.var_fn205_calc_iq__qd2_dn2, locals.var_fn205_calc_iq__qd2_dn4, locals.var_fn205_calc_iq__qd2_dn7, locals.var_fn205_calc_iq__qd2_dn10, locals.var_fn205_calc_iq__qd2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd2 = assign16920_e16393;
        locals.var_fn205_calc_iq__qd2_dn2 = assign16920_e16393_d_n2;
        locals.var_fn205_calc_iq__qd2_dn4 = assign16920_e16393_d_n4;
        locals.var_fn205_calc_iq__qd2_dn7 = assign16920_e16393_d_n7;
        locals.var_fn205_calc_iq__qd2_dn10 = assign16920_e16393_d_n10;
        locals.var_fn205_calc_iq__qd2_dn11 = assign16920_e16393_d_n11;

        let (assign16930_e16397, assign16930_e16397_d_n2, assign16930_e16397_d_n4, assign16930_e16397_d_n7, assign16930_e16397_d_n10, assign16930_e16397_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd3, locals.var_fn205_calc_iq__qd3_dn2, locals.var_fn205_calc_iq__qd3_dn4, locals.var_fn205_calc_iq__qd3_dn7, locals.var_fn205_calc_iq__qd3_dn10, locals.var_fn205_calc_iq__qd3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd3 = assign16930_e16397;
        locals.var_fn205_calc_iq__qd3_dn2 = assign16930_e16397_d_n2;
        locals.var_fn205_calc_iq__qd3_dn4 = assign16930_e16397_d_n4;
        locals.var_fn205_calc_iq__qd3_dn7 = assign16930_e16397_d_n7;
        locals.var_fn205_calc_iq__qd3_dn10 = assign16930_e16397_d_n10;
        locals.var_fn205_calc_iq__qd3_dn11 = assign16930_e16397_d_n11;

        let (assign16940_e16401, assign16940_e16401_d_n2, assign16940_e16401_d_n4, assign16940_e16401_d_n7, assign16940_e16401_d_n10, assign16940_e16401_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsqd, locals.var_fn205_calc_iq__qsqd_dn2, locals.var_fn205_calc_iq__qsqd_dn4, locals.var_fn205_calc_iq__qsqd_dn7, locals.var_fn205_calc_iq__qsqd_dn10, locals.var_fn205_calc_iq__qsqd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsqd = assign16940_e16401;
        locals.var_fn205_calc_iq__qsqd_dn2 = assign16940_e16401_d_n2;
        locals.var_fn205_calc_iq__qsqd_dn4 = assign16940_e16401_d_n4;
        locals.var_fn205_calc_iq__qsqd_dn7 = assign16940_e16401_d_n7;
        locals.var_fn205_calc_iq__qsqd_dn10 = assign16940_e16401_d_n10;
        locals.var_fn205_calc_iq__qsqd_dn11 = assign16940_e16401_d_n11;

        let (assign16950_e16405, assign16950_e16405_d_n2, assign16950_e16405_d_n4, assign16950_e16405_d_n7, assign16950_e16405_d_n10, assign16950_e16405_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvdd, locals.var_fn205_calc_iq__qinvdd_dn2, locals.var_fn205_calc_iq__qinvdd_dn4, locals.var_fn205_calc_iq__qinvdd_dn7, locals.var_fn205_calc_iq__qinvdd_dn10, locals.var_fn205_calc_iq__qinvdd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvdd = assign16950_e16405;
        locals.var_fn205_calc_iq__qinvdd_dn2 = assign16950_e16405_d_n2;
        locals.var_fn205_calc_iq__qinvdd_dn4 = assign16950_e16405_d_n4;
        locals.var_fn205_calc_iq__qinvdd_dn7 = assign16950_e16405_d_n7;
        locals.var_fn205_calc_iq__qinvdd_dn10 = assign16950_e16405_d_n10;
        locals.var_fn205_calc_iq__qinvdd_dn11 = assign16950_e16405_d_n11;

        let (assign16960_e16409, assign16960_e16409_d_n2, assign16960_e16409_d_n4, assign16960_e16409_d_n7, assign16960_e16409_d_n10, assign16960_e16409_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd1 = assign16960_e16409;
        locals.var_fn205_calc_iq__qd1_dn2 = assign16960_e16409_d_n2;
        locals.var_fn205_calc_iq__qd1_dn4 = assign16960_e16409_d_n4;
        locals.var_fn205_calc_iq__qd1_dn7 = assign16960_e16409_d_n7;
        locals.var_fn205_calc_iq__qd1_dn10 = assign16960_e16409_d_n10;
        locals.var_fn205_calc_iq__qd1_dn11 = assign16960_e16409_d_n11;

        let (assign16970_e16413, assign16970_e16413_d_n2, assign16970_e16413_d_n4, assign16970_e16413_d_n7, assign16970_e16413_d_n10, assign16970_e16413_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs, locals.var_fn205_calc_iq__qs_dn2, locals.var_fn205_calc_iq__qs_dn4, locals.var_fn205_calc_iq__qs_dn7, locals.var_fn205_calc_iq__qs_dn10, locals.var_fn205_calc_iq__qs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs = assign16970_e16413;
        locals.var_fn205_calc_iq__qs_dn2 = assign16970_e16413_d_n2;
        locals.var_fn205_calc_iq__qs_dn4 = assign16970_e16413_d_n4;
        locals.var_fn205_calc_iq__qs_dn7 = assign16970_e16413_d_n7;
        locals.var_fn205_calc_iq__qs_dn10 = assign16970_e16413_d_n10;
        locals.var_fn205_calc_iq__qs_dn11 = assign16970_e16413_d_n11;

        let (assign16980_e16417, assign16980_e16417_d_n2, assign16980_e16417_d_n4, assign16980_e16417_d_n7, assign16980_e16417_d_n10, assign16980_e16417_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd, locals.var_fn205_calc_iq__qd_dn2, locals.var_fn205_calc_iq__qd_dn4, locals.var_fn205_calc_iq__qd_dn7, locals.var_fn205_calc_iq__qd_dn10, locals.var_fn205_calc_iq__qd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd = assign16980_e16417;
        locals.var_fn205_calc_iq__qd_dn2 = assign16980_e16417_d_n2;
        locals.var_fn205_calc_iq__qd_dn4 = assign16980_e16417_d_n4;
        locals.var_fn205_calc_iq__qd_dn7 = assign16980_e16417_d_n7;
        locals.var_fn205_calc_iq__qd_dn10 = assign16980_e16417_d_n10;
        locals.var_fn205_calc_iq__qd_dn11 = assign16980_e16417_d_n11;

        let (assign16990_e16421, assign16990_e16421_d_n2, assign16990_e16421_d_n4, assign16990_e16421_d_n7, assign16990_e16421_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, locals.var_fn205_calc_iq__etac_dn11,)
    }
};
        locals.var_fn205_calc_iq__etac = assign16990_e16421;
        locals.var_fn205_calc_iq__etac_dn2 = assign16990_e16421_d_n2;
        locals.var_fn205_calc_iq__etac_dn4 = assign16990_e16421_d_n4;
        locals.var_fn205_calc_iq__etac_dn7 = assign16990_e16421_d_n7;
        locals.var_fn205_calc_iq__etac_dn11 = assign16990_e16421_d_n11;

        let (assign17000_e16425, assign17000_e16425_d_n3, assign17000_e16425_d_n4, assign17000_e16425_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etab, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, locals.var_fn205_calc_iq__etab_dn11,)
    }
};
        locals.var_fn205_calc_iq__etab = assign17000_e16425;
        locals.var_fn205_calc_iq__etab_dn3 = assign17000_e16425_d_n3;
        locals.var_fn205_calc_iq__etab_dn4 = assign17000_e16425_d_n4;
        locals.var_fn205_calc_iq__etab_dn11 = assign17000_e16425_d_n11;

        let (assign17010_e16429, assign17010_e16429_d_n2, assign17010_e16429_d_n4, assign17010_e16429_d_n7, assign17010_e16429_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, locals.var_fn205_calc_iq__etags_dn11,)
    }
};
        locals.var_fn205_calc_iq__etags = assign17010_e16429;
        locals.var_fn205_calc_iq__etags_dn2 = assign17010_e16429_d_n2;
        locals.var_fn205_calc_iq__etags_dn4 = assign17010_e16429_d_n4;
        locals.var_fn205_calc_iq__etags_dn7 = assign17010_e16429_d_n7;
        locals.var_fn205_calc_iq__etags_dn11 = assign17010_e16429_d_n11;

        let (assign17020_e16433, assign17020_e16433_d_n2, assign17020_e16433_d_n3, assign17020_e16433_d_n4, assign17020_e16433_d_n7, assign17020_e16433_d_n10, assign17020_e16433_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17020_e16433;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17020_e16433_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17020_e16433_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17020_e16433_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17020_e16433_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17020_e16433_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17020_e16433_d_n11;

        let (assign17030_e16437, assign17030_e16437_d_n2, assign17030_e16437_d_n3, assign17030_e16437_d_n4, assign17030_e16437_d_n7, assign17030_e16437_d_n10, assign17030_e16437_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17030_e16437;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17030_e16437_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17030_e16437_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17030_e16437_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17030_e16437_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17030_e16437_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17030_e16437_d_n11;

        let (assign17040_e16441, assign17040_e16441_d_n10, assign17040_e16441_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__absvdsin, locals.var_fn205_calc_iq__absvdsin_dn10, locals.var_fn205_calc_iq__absvdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__absvdsin = assign17040_e16441;
        locals.var_fn205_calc_iq__absvdsin_dn10 = assign17040_e16441_d_n10;
        locals.var_fn205_calc_iq__absvdsin_dn11 = assign17040_e16441_d_n11;

        let (assign17050_e16445, assign17050_e16445_d_n2, assign17050_e16445_d_n7, assign17050_e16445_d_n10, assign17050_e16445_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vgdin, locals.var_fn205_calc_iq__vgdin_dn2, locals.var_fn205_calc_iq__vgdin_dn7, locals.var_fn205_calc_iq__vgdin_dn10, locals.var_fn205_calc_iq__vgdin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgdin = assign17050_e16445;
        locals.var_fn205_calc_iq__vgdin_dn2 = assign17050_e16445_d_n2;
        locals.var_fn205_calc_iq__vgdin_dn7 = assign17050_e16445_d_n7;
        locals.var_fn205_calc_iq__vgdin_dn10 = assign17050_e16445_d_n10;
        locals.var_fn205_calc_iq__vgdin_dn11 = assign17050_e16445_d_n11;

        let (assign17060_e16449, assign17060_e16449_d_n2, assign17060_e16449_d_n4, assign17060_e16449_d_n7, assign17060_e16449_d_n10, assign17060_e16449_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17060_e16449;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17060_e16449_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17060_e16449_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17060_e16449_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17060_e16449_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17060_e16449_d_n11;

        let (assign17070_e16453, assign17070_e16453_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__myarg0, locals.var_fn205_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn205_calc_iq__myarg0 = assign17070_e16453;
        locals.var_fn205_calc_iq__myarg0_dn4 = assign17070_e16453_d_n4;

    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17080_e16480, assign17080_e16480_d_n10, assign17080_e16480_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17080_e16478, assign17080_e16478_d_n10, assign17080_e16478_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17080_e16462: f64 = (0.001 / p.p53);
                let assign17080_e16464: f64 = (assign17080_e16462 * locals.var_fn205_calc_iq__vdsin);
                let assign17080_e16465: f64 = (assign17080_e16464).tanh();
                let assign17080_e16466: f64 = (locals.var_fn205_calc_iq__vdsin * assign17080_e16465);
                (assign17080_e16466, ((locals.var_fn205_calc_iq__vdsin_dn10 * assign17080_e16465) + (locals.var_fn205_calc_iq__vdsin * ((assign17080_e16462 * locals.var_fn205_calc_iq__vdsin_dn10) / ((assign17080_e16464).cosh() * (assign17080_e16464).cosh())))), ((locals.var_fn205_calc_iq__vdsin_dn11 * assign17080_e16465) + (locals.var_fn205_calc_iq__vdsin * ((assign17080_e16462 * locals.var_fn205_calc_iq__vdsin_dn11) / ((assign17080_e16464).cosh() * (assign17080_e16464).cosh())))),)
            } else {
                let (assign17080_e16477, assign17080_e16477_d_n10, assign17080_e16477_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17080_e16472: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin);
                        let assign17080_e16474: f64 = (assign17080_e16472 + p.p53);
                        let assign17080_e16475: f64 = (assign17080_e16474).sqrt();
                        (assign17080_e16475, (((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsin) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin_dn10)) / (2.0 * assign17080_e16475)), (((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsin) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin_dn11)) / (2.0 * assign17080_e16475)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign17080_e16477, assign17080_e16477_d_n10, assign17080_e16477_d_n11,)
            }
        };
        (assign17080_e16478, assign17080_e16478_d_n10, assign17080_e16478_d_n11,)
    } else {
        (locals.var_fn205_calc_iq__absvdsin, locals.var_fn205_calc_iq__absvdsin_dn10, locals.var_fn205_calc_iq__absvdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__absvdsin = assign17080_e16480;
        locals.var_fn205_calc_iq__absvdsin_dn10 = assign17080_e16480_d_n10;
        locals.var_fn205_calc_iq__absvdsin_dn11 = assign17080_e16480_d_n11;

        let (assign17090_e16486, assign17090_e16486_d_n2, assign17090_e16486_d_n7, assign17090_e16486_d_n10, assign17090_e16486_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17090_e16484: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdsin);
        (assign17090_e16484, locals.var_fn205_calc_iq__vgsin_dn2, locals.var_fn205_calc_iq__vgsin_dn7, (-locals.var_fn205_calc_iq__vdsin_dn10), (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdsin_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vgdin, locals.var_fn205_calc_iq__vgdin_dn2, locals.var_fn205_calc_iq__vgdin_dn7, locals.var_fn205_calc_iq__vgdin_dn10, locals.var_fn205_calc_iq__vgdin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgdin = assign17090_e16486;
        locals.var_fn205_calc_iq__vgdin_dn2 = assign17090_e16486_d_n2;
        locals.var_fn205_calc_iq__vgdin_dn7 = assign17090_e16486_d_n7;
        locals.var_fn205_calc_iq__vgdin_dn10 = assign17090_e16486_d_n10;
        locals.var_fn205_calc_iq__vgdin_dn11 = assign17090_e16486_d_n11;

        let (assign17100_e16492, assign17100_e16492_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17100_e16490: f64 = (locals.var_fn205_calc_iq__alpha * locals.var_fn205_calc_iq__phitin);
        (assign17100_e16490, (locals.var_fn205_calc_iq__alpha * locals.var_fn205_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn205_calc_iq__alpha_phit, locals.var_fn205_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn205_calc_iq__alpha_phit = assign17100_e16492;
        locals.var_fn205_calc_iq__alpha_phit_dn4 = assign17100_e16492_d_n4;

        let (assign17110_e16504, assign17110_e16504_d_n4, assign17110_e16504_d_n10, assign17110_e16504_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17110_e16497: f64 = (2.302585092994046 * locals.var_fn205_calc_iq__phitin);
        let assign17110_e16498: f64 = (locals.var_fn205_calc_iq__ss / assign17110_e16497);
        let assign17110_e16501: f64 = (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin);
        let assign17110_e16502: f64 = (assign17110_e16498 + assign17110_e16501);
        (assign17110_e16502, (-((locals.var_fn205_calc_iq__ss * (2.302585092994046 * locals.var_fn205_calc_iq__phitin_dn4)) / (assign17110_e16497 * assign17110_e16497))), (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin_dn10), (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin_dn11),)
    } else {
        (locals.var_fn205_calc_iq__n, locals.var_fn205_calc_iq__n_dn4, locals.var_fn205_calc_iq__n_dn10, locals.var_fn205_calc_iq__n_dn11,)
    }
};
        locals.var_fn205_calc_iq__n = assign17110_e16504;
        locals.var_fn205_calc_iq__n_dn4 = assign17110_e16504_d_n4;
        locals.var_fn205_calc_iq__n_dn10 = assign17110_e16504_d_n10;
        locals.var_fn205_calc_iq__n_dn11 = assign17110_e16504_d_n11;

        let (assign17120_e16514, assign17120_e16514_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17120_e16510: f64 = (locals.var_fn205_calc_iq__tambin - locals.var_fn205_calc_iq__tnomin);
        let assign17120_e16511: f64 = (locals.var_fn205_calc_iq__vtzeta * assign17120_e16510);
        let assign17120_e16512: f64 = (locals.var_fn205_calc_iq__vto + assign17120_e16511);
        (assign17120_e16512, (locals.var_fn205_calc_iq__vtzeta * locals.var_fn205_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn205_calc_iq__vtof, locals.var_fn205_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn205_calc_iq__vtof = assign17120_e16514;
        locals.var_fn205_calc_iq__vtof_dn4 = assign17120_e16514_d_n4;

        let (assign17130_e16522, assign17130_e16522_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17130_e16518: f64 = (locals.var_fn205_calc_iq__tambin / locals.var_fn205_calc_iq__tnomin);
        let assign17130_e16520: f64 = (assign17130_e16518).powf(locals.var_fn205_calc_iq__epsilon);
        (assign17130_e16520, if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn205_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__epsilon * ((assign17130_e16518).powf(locals.var_fn205_calc_iq__epsilon - 1.0) * (locals.var_fn205_calc_iq__tambin_dn4 / locals.var_fn205_calc_iq__tnomin))) } } else { (assign17130_e16520 * (locals.var_fn205_calc_iq__epsilon * ((locals.var_fn205_calc_iq__tambin_dn4 / locals.var_fn205_calc_iq__tnomin) / assign17130_e16518))) },)
    } else {
        (locals.var_fn205_calc_iq__tfacmobin, locals.var_fn205_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tfacmobin = assign17130_e16522;
        locals.var_fn205_calc_iq__tfacmobin_dn4 = assign17130_e16522_d_n4;

        let assign17140_e16525: f64 = if locals.var_fn205_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard206 = assign17140_e16525;

        let (assign17150_e16543, assign17150_e16543_d_n10, assign17150_e16543_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard206 != 0.0)) {
        let assign17150_e16533: f64 = (locals.var_fn205_calc_iq__absvdsin / locals.var_fn205_calc_iq__dibsat);
        let assign17150_e16535: f64 = (assign17150_e16533).powf(locals.var_fn205_calc_iq__beta);
        let assign17150_e16536: f64 = (1.0 + assign17150_e16535);
        let assign17150_e16539: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17150_e16540: f64 = (assign17150_e16536).powf(assign17150_e16539);
        let assign17150_e16541: f64 = (locals.var_fn205_calc_iq__absvdsin / assign17150_e16540);
        (assign17150_e16541, (((locals.var_fn205_calc_iq__absvdsin_dn10 * assign17150_e16540) - (locals.var_fn205_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign17150_e16539) as f64).is_finite() && ((assign17150_e16539) as f64).fract() == 0.0 { if assign17150_e16539 == 0.0 { 0.0 } else { (assign17150_e16539 * ((assign17150_e16536).powf(assign17150_e16539 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) })) } } else { (assign17150_e16540 * (assign17150_e16539 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) } / assign17150_e16536))) })) / (assign17150_e16540 * assign17150_e16540)), (((locals.var_fn205_calc_iq__absvdsin_dn11 * assign17150_e16540) - (locals.var_fn205_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign17150_e16539) as f64).is_finite() && ((assign17150_e16539) as f64).fract() == 0.0 { if assign17150_e16539 == 0.0 { 0.0 } else { (assign17150_e16539 * ((assign17150_e16536).powf(assign17150_e16539 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) })) } } else { (assign17150_e16540 * (assign17150_e16539 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) } / assign17150_e16536))) })) / (assign17150_e16540 * assign17150_e16540)),)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign17150_e16543;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign17150_e16543_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign17150_e16543_d_n11;

        let (assign17160_e16550, assign17160_e16550_d_n10, assign17160_e16550_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard206 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign17160_e16550;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign17160_e16550_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign17160_e16550_d_n11;

        let (assign17170_e16560, assign17170_e16560_d_n10, assign17170_e16560_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17170_e16555: f64 = (locals.var_fn205_calc_iq__vsatdibl * locals.var_fn205_calc_iq__delta2);
        let assign17170_e16556: f64 = (locals.var_fn205_calc_iq__delta1 - assign17170_e16555);
        let assign17170_e16558: f64 = (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin);
        (assign17170_e16558, (((-(locals.var_fn205_calc_iq__vsatdibl_dn10 * locals.var_fn205_calc_iq__delta2)) * locals.var_fn205_calc_iq__absvdsin) + (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin_dn10)), (((-(locals.var_fn205_calc_iq__vsatdibl_dn11 * locals.var_fn205_calc_iq__delta2)) * locals.var_fn205_calc_iq__absvdsin) + (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__delta, locals.var_fn205_calc_iq__delta_dn10, locals.var_fn205_calc_iq__delta_dn11,)
    }
};
        locals.var_fn205_calc_iq__delta = assign17170_e16560;
        locals.var_fn205_calc_iq__delta_dn10 = assign17170_e16560_d_n10;
        locals.var_fn205_calc_iq__delta_dn11 = assign17170_e16560_d_n11;

        let (assign17180_e16566, assign17180_e16566_d_n4, assign17180_e16566_d_n10, assign17180_e16566_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17180_e16564: f64 = (locals.var_fn205_calc_iq__vtof - locals.var_fn205_calc_iq__delta);
        (assign17180_e16564, locals.var_fn205_calc_iq__vtof_dn4, (-locals.var_fn205_calc_iq__delta_dn10), (-locals.var_fn205_calc_iq__delta_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vtdibl, locals.var_fn205_calc_iq__vtdibl_dn4, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vtdibl = assign17180_e16566;
        locals.var_fn205_calc_iq__vtdibl_dn4 = assign17180_e16566_d_n4;
        locals.var_fn205_calc_iq__vtdibl_dn10 = assign17180_e16566_d_n10;
        locals.var_fn205_calc_iq__vtdibl_dn11 = assign17180_e16566_d_n11;

        let (assign17190_e16574, assign17190_e16574_d_n4, assign17190_e16574_d_n10, assign17190_e16574_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17190_e16570: f64 = (2.0 * locals.var_fn205_calc_iq__n);
        let assign17190_e16572: f64 = (assign17190_e16570 * locals.var_fn205_calc_iq__phitin);
        (assign17190_e16572, (((2.0 * locals.var_fn205_calc_iq__n_dn4) * locals.var_fn205_calc_iq__phitin) + (assign17190_e16570 * locals.var_fn205_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn205_calc_iq__n_dn10) * locals.var_fn205_calc_iq__phitin), ((2.0 * locals.var_fn205_calc_iq__n_dn11) * locals.var_fn205_calc_iq__phitin),)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit, locals.var_fn205_calc_iq__two_n_phit_dn4, locals.var_fn205_calc_iq__two_n_phit_dn10, locals.var_fn205_calc_iq__two_n_phit_dn11,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit = assign17190_e16574;
        locals.var_fn205_calc_iq__two_n_phit_dn4 = assign17190_e16574_d_n4;
        locals.var_fn205_calc_iq__two_n_phit_dn10 = assign17190_e16574_d_n10;
        locals.var_fn205_calc_iq__two_n_phit_dn11 = assign17190_e16574_d_n11;

        let (assign17200_e16580, assign17200_e16580_d_n4, assign17200_e16580_d_n10, assign17200_e16580_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17200_e16578: f64 = (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit);
        (assign17200_e16578, ((locals.var_fn205_calc_iq__cgin_dn4 * locals.var_fn205_calc_iq__two_n_phit) + (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn4)), (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn10), (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qref, locals.var_fn205_calc_iq__qref_dn4, locals.var_fn205_calc_iq__qref_dn10, locals.var_fn205_calc_iq__qref_dn11,)
    }
};
        locals.var_fn205_calc_iq__qref = assign17200_e16580;
        locals.var_fn205_calc_iq__qref_dn4 = assign17200_e16580_d_n4;
        locals.var_fn205_calc_iq__qref_dn10 = assign17200_e16580_d_n10;
        locals.var_fn205_calc_iq__qref_dn11 = assign17200_e16580_d_n11;

        let (assign17210_e16590, assign17210_e16590_d_n2, assign17210_e16590_d_n3, assign17210_e16590_d_n4, assign17210_e16590_d_n7, assign17210_e16590_d_n10, assign17210_e16590_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17210_e16585: f64 = (p.p51 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17210_e16587: f64 = (assign17210_e16585 / 2.0);
        let assign17210_e16588: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17210_e16587);
        (assign17210_e16588, 0.0, 0.0, (locals.var_fn205_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn205_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17210_e16590;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17210_e16590_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17210_e16590_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17210_e16590_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17210_e16590_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17210_e16590_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17210_e16590_d_n11;

        let (assign17220_e16641, assign17220_e16641_d_n2, assign17220_e16641_d_n3, assign17220_e16641_d_n4, assign17220_e16641_d_n7, assign17220_e16641_d_n10, assign17220_e16641_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17220_e16635, assign17220_e16635_d_n2, assign17220_e16635_d_n7, assign17220_e16635_d_n10, assign17220_e16635_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17220_e16599: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16602: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16605: f64 = (0.001 / p.p53);
                let assign17220_e16608: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16609: f64 = (assign17220_e16605 * assign17220_e16608);
                let assign17220_e16610: f64 = (assign17220_e16609).tanh();
                let assign17220_e16611: f64 = (assign17220_e16602 * assign17220_e16610);
                let assign17220_e16612: f64 = (assign17220_e16599 + assign17220_e16611);
                let assign17220_e16613: f64 = (0.5 * assign17220_e16612);
                (assign17220_e16613, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))),)
            } else {
                let (assign17220_e16634, assign17220_e16634_d_n2, assign17220_e16634_d_n7, assign17220_e16634_d_n10, assign17220_e16634_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17220_e16620: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16623: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16626: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16627: f64 = (assign17220_e16623 * assign17220_e16626);
                        let assign17220_e16629: f64 = (assign17220_e16627 + p.p53);
                        let assign17220_e16630: f64 = (assign17220_e16629).sqrt();
                        let assign17220_e16631: f64 = (assign17220_e16620 + assign17220_e16630);
                        let assign17220_e16632: f64 = (0.5 * assign17220_e16631);
                        (assign17220_e16632, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17220_e16630)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17220_e16630)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17220_e16626) + (assign17220_e16623 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17220_e16630)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17220_e16630)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17220_e16634, assign17220_e16634_d_n2, assign17220_e16634_d_n7, assign17220_e16634_d_n10, assign17220_e16634_d_n11,)
            }
        };
        let assign17220_e16637: f64 = (assign17220_e16635 - locals.var_fn205_calc_iq__myarg);
        let assign17220_e16639: f64 = (assign17220_e16637 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17220_e16639, ((assign17220_e16635_d_n2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17220_e16637 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((assign17220_e16635_d_n7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((assign17220_e16635_d_n10 - locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((assign17220_e16635_d_n11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17220_e16641;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17220_e16641_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17220_e16641_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17220_e16641_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17220_e16641_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17220_e16641_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17220_e16641_d_n11;

        let assign17230_e16644: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard207 = assign17230_e16644;

        let (assign17240_e16650, assign17240_e16650_d_n2, assign17240_e16650_d_n3, assign17240_e16650_d_n4, assign17240_e16650_d_n7, assign17240_e16650_d_n10, assign17240_e16650_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard207 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17240_e16650;
        locals.var_fn205_calc_iq__ff_dn2 = assign17240_e16650_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17240_e16650_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17240_e16650_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17240_e16650_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17240_e16650_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17240_e16650_d_n11;

        let assign17250_e16653: f64 = (-50.0);
        let assign17250_e16654: f64 = if locals.var_fn205_calc_iq__exparg < assign17250_e16653 { 1.0 } else { 0.0 };
        locals.var_guard208 = assign17250_e16654;

        let (assign17260_e16663, assign17260_e16663_d_n2, assign17260_e16663_d_n3, assign17260_e16663_d_n4, assign17260_e16663_d_n7, assign17260_e16663_d_n10, assign17260_e16663_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard207 == 0.0)) && (locals.var_guard208 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17260_e16663;
        locals.var_fn205_calc_iq__ff_dn2 = assign17260_e16663_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17260_e16663_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17260_e16663_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17260_e16663_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17260_e16663_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17260_e16663_d_n11;

        let (assign17270_e16678, assign17270_e16678_d_n2, assign17270_e16678_d_n3, assign17270_e16678_d_n4, assign17270_e16678_d_n7, assign17270_e16678_d_n10, assign17270_e16678_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard207 == 0.0)) && (locals.var_guard208 == 0.0)) {
        let assign17270_e16674: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17270_e16675: f64 = (1.0 + assign17270_e16674);
        let assign17270_e16676: f64 = (1.0 / assign17270_e16675);
        (assign17270_e16676, (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17270_e16675 * assign17270_e16675))),)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17270_e16678;
        locals.var_fn205_calc_iq__ff_dn2 = assign17270_e16678_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17270_e16678_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17270_e16678_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17270_e16678_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17270_e16678_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17270_e16678_d_n11;

        let (assign17280_e16737, assign17280_e16737_d_n2, assign17280_e16737_d_n3, assign17280_e16737_d_n4, assign17280_e16737_d_n7, assign17280_e16737_d_n10, assign17280_e16737_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17280_e16723, assign17280_e16723_d_n2, assign17280_e16723_d_n7, assign17280_e16723_d_n10, assign17280_e16723_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17280_e16687: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16690: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16693: f64 = (0.001 / p.p53);
                let assign17280_e16696: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16697: f64 = (assign17280_e16693 * assign17280_e16696);
                let assign17280_e16698: f64 = (assign17280_e16697).tanh();
                let assign17280_e16699: f64 = (assign17280_e16690 * assign17280_e16698);
                let assign17280_e16700: f64 = (assign17280_e16687 + assign17280_e16699);
                let assign17280_e16701: f64 = (0.5 * assign17280_e16700);
                (assign17280_e16701, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))),)
            } else {
                let (assign17280_e16722, assign17280_e16722_d_n2, assign17280_e16722_d_n7, assign17280_e16722_d_n10, assign17280_e16722_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17280_e16708: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16711: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16714: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16715: f64 = (assign17280_e16711 * assign17280_e16714);
                        let assign17280_e16717: f64 = (assign17280_e16715 + p.p53);
                        let assign17280_e16718: f64 = (assign17280_e16717).sqrt();
                        let assign17280_e16719: f64 = (assign17280_e16708 + assign17280_e16718);
                        let assign17280_e16720: f64 = (0.5 * assign17280_e16719);
                        (assign17280_e16720, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17280_e16718)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17280_e16718)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17280_e16714) + (assign17280_e16711 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17280_e16718)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17280_e16718)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17280_e16722, assign17280_e16722_d_n2, assign17280_e16722_d_n7, assign17280_e16722_d_n10, assign17280_e16722_d_n11,)
            }
        };
        let assign17280_e16727: f64 = (p.p51 * 0.1);
        let assign17280_e16729: f64 = (assign17280_e16727 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17280_e16731: f64 = (assign17280_e16729 * locals.var_fn205_calc_iq__ff);
        let assign17280_e16732: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17280_e16731);
        let assign17280_e16733: f64 = (assign17280_e16723 - assign17280_e16732);
        let assign17280_e16735: f64 = (assign17280_e16733 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17280_e16735, ((assign17280_e16723_d_n2 - (-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn2))) / locals.var_fn205_calc_iq__two_n_phit), ((-(-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn3))) / locals.var_fn205_calc_iq__two_n_phit), ((((-(locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17280_e16727 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ff) + (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), ((assign17280_e16723_d_n7 - (-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn7))) / locals.var_fn205_calc_iq__two_n_phit), ((((assign17280_e16723_d_n10 - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), ((((assign17280_e16723_d_n11 - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__eta, locals.var_fn205_calc_iq__eta_dn2, locals.var_fn205_calc_iq__eta_dn3, locals.var_fn205_calc_iq__eta_dn4, locals.var_fn205_calc_iq__eta_dn7, locals.var_fn205_calc_iq__eta_dn10, locals.var_fn205_calc_iq__eta_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta = assign17280_e16737;
        locals.var_fn205_calc_iq__eta_dn2 = assign17280_e16737_d_n2;
        locals.var_fn205_calc_iq__eta_dn3 = assign17280_e16737_d_n3;
        locals.var_fn205_calc_iq__eta_dn4 = assign17280_e16737_d_n4;
        locals.var_fn205_calc_iq__eta_dn7 = assign17280_e16737_d_n7;
        locals.var_fn205_calc_iq__eta_dn10 = assign17280_e16737_d_n10;
        locals.var_fn205_calc_iq__eta_dn11 = assign17280_e16737_d_n11;

        let assign17290_e16740: f64 = if locals.var_fn205_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign17290_e16740;

        let (assign17300_e16748, assign17300_e16748_d_n2, assign17300_e16748_d_n3, assign17300_e16748_d_n4, assign17300_e16748_d_n7, assign17300_e16748_d_n10, assign17300_e16748_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard209 != 0.0)) {
        let assign17300_e16746: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta);
        (assign17300_e16746, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17300_e16748;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17300_e16748_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17300_e16748_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17300_e16748_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17300_e16748_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17300_e16748_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17300_e16748_d_n11;

        let assign17310_e16751: f64 = (-50.0);
        let assign17310_e16752: f64 = if locals.var_fn205_calc_iq__eta < assign17310_e16751 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign17310_e16752;

        let (assign17320_e16764, assign17320_e16764_d_n2, assign17320_e16764_d_n3, assign17320_e16764_d_n4, assign17320_e16764_d_n7, assign17320_e16764_d_n10, assign17320_e16764_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign17320_e16761: f64 = (locals.var_fn205_calc_iq__eta).exp();
        let assign17320_e16762: f64 = (locals.var_fn205_calc_iq__qref * assign17320_e16761);
        (assign17320_e16762, (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn2)), (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn4))), (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17320_e16764;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17320_e16764_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17320_e16764_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17320_e16764_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17320_e16764_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17320_e16764_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17320_e16764_d_n11;

        let (assign17330_e16780, assign17330_e16780_d_n2, assign17330_e16780_d_n3, assign17330_e16780_d_n4, assign17330_e16780_d_n7, assign17330_e16780_d_n10, assign17330_e16780_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard210 == 0.0)) {
        let assign17330_e16775: f64 = (locals.var_fn205_calc_iq__eta).exp();
        let assign17330_e16776: f64 = (1.0 + assign17330_e16775);
        let assign17330_e16777: f64 = (assign17330_e16776).ln();
        let assign17330_e16778: f64 = (locals.var_fn205_calc_iq__qref * assign17330_e16777);
        (assign17330_e16778, (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn2) / assign17330_e16776)), (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn3) / assign17330_e16776)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn4) / assign17330_e16776))), (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn7) / assign17330_e16776)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn10) / assign17330_e16776))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn11) / assign17330_e16776))),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17330_e16780;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17330_e16780_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17330_e16780_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17330_e16780_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17330_e16780_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17330_e16780_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17330_e16780_d_n11;

        let (assign17340_e16794, assign17340_e16794_d_n2, assign17340_e16794_d_n3, assign17340_e16794_d_n4, assign17340_e16794_d_n7, assign17340_e16794_d_n10, assign17340_e16794_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17340_e16787: f64 = (locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv);
        let assign17340_e16789: f64 = (assign17340_e16787 / locals.var_fn205_calc_iq__cgin);
        let assign17340_e16790: f64 = (1.0 + assign17340_e16789);
        let assign17340_e16791: f64 = (locals.var_fn205_calc_iq__tfacmobin * assign17340_e16790);
        let assign17340_e16792: f64 = (locals.var_fn205_calc_iq__mu0 / assign17340_e16791);
        (assign17340_e16792, (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * ((locals.var_fn205_calc_iq__tfacmobin_dn4 * assign17340_e16790) + (locals.var_fn205_calc_iq__tfacmobin * ((((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17340_e16787 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin))))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))),)
    } else {
        (locals.var_fn205_calc_iq__muf, locals.var_fn205_calc_iq__muf_dn2, locals.var_fn205_calc_iq__muf_dn3, locals.var_fn205_calc_iq__muf_dn4, locals.var_fn205_calc_iq__muf_dn7, locals.var_fn205_calc_iq__muf_dn10, locals.var_fn205_calc_iq__muf_dn11,)
    }
};
        locals.var_fn205_calc_iq__muf = assign17340_e16794;
        locals.var_fn205_calc_iq__muf_dn2 = assign17340_e16794_d_n2;
        locals.var_fn205_calc_iq__muf_dn3 = assign17340_e16794_d_n3;
        locals.var_fn205_calc_iq__muf_dn4 = assign17340_e16794_d_n4;
        locals.var_fn205_calc_iq__muf_dn7 = assign17340_e16794_d_n7;
        locals.var_fn205_calc_iq__muf_dn10 = assign17340_e16794_d_n10;
        locals.var_fn205_calc_iq__muf_dn11 = assign17340_e16794_d_n11;

        let (assign17350_e16826, assign17350_e16826_d_n2, assign17350_e16826_d_n3, assign17350_e16826_d_n4, assign17350_e16826_d_n7, assign17350_e16826_d_n10, assign17350_e16826_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17350_e16800: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tnomin);
        let assign17350_e16801: f64 = (1.0 + assign17350_e16800);
        let assign17350_e16805: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin);
        let assign17350_e16806: f64 = (1.0 + assign17350_e16805);
        let assign17350_e16807: f64 = (assign17350_e16801 / assign17350_e16806);
        let assign17350_e16808: f64 = (locals.var_fn205_calc_iq__vel0 * assign17350_e16807);
        let assign17350_e16812: f64 = (locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin);
        let assign17350_e16814: f64 = (assign17350_e16812 / locals.var_fn205_calc_iq__lin);
        let assign17350_e16815: f64 = (1.0 + assign17350_e16814);
        let assign17350_e16816: f64 = (assign17350_e16808 * assign17350_e16815);
        let assign17350_e16820: f64 = (locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv);
        let assign17350_e16822: f64 = (assign17350_e16820 / locals.var_fn205_calc_iq__cgin);
        let assign17350_e16823: f64 = (1.0 + assign17350_e16822);
        let assign17350_e16824: f64 = (assign17350_e16816 / assign17350_e16823);
        (assign17350_e16824, (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), (((((locals.var_fn205_calc_iq__vel0 * (-((assign17350_e16801 * (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin_dn4)) / (assign17350_e16806 * assign17350_e16806)))) * assign17350_e16815) * assign17350_e16823) - (assign17350_e16816 * ((((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17350_e16820 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)))) / (assign17350_e16823 * assign17350_e16823)), (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), ((((assign17350_e16808 * ((locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin_dn10) / locals.var_fn205_calc_iq__lin)) * assign17350_e16823) - (assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin))) / (assign17350_e16823 * assign17350_e16823)), ((((assign17350_e16808 * ((locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin_dn11) / locals.var_fn205_calc_iq__lin)) * assign17350_e16823) - (assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin))) / (assign17350_e16823 * assign17350_e16823)),)
    } else {
        (locals.var_fn205_calc_iq__vx, locals.var_fn205_calc_iq__vx_dn2, locals.var_fn205_calc_iq__vx_dn3, locals.var_fn205_calc_iq__vx_dn4, locals.var_fn205_calc_iq__vx_dn7, locals.var_fn205_calc_iq__vx_dn10, locals.var_fn205_calc_iq__vx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vx = assign17350_e16826;
        locals.var_fn205_calc_iq__vx_dn2 = assign17350_e16826_d_n2;
        locals.var_fn205_calc_iq__vx_dn3 = assign17350_e16826_d_n3;
        locals.var_fn205_calc_iq__vx_dn4 = assign17350_e16826_d_n4;
        locals.var_fn205_calc_iq__vx_dn7 = assign17350_e16826_d_n7;
        locals.var_fn205_calc_iq__vx_dn10 = assign17350_e16826_d_n10;
        locals.var_fn205_calc_iq__vx_dn11 = assign17350_e16826_d_n11;

        let (assign17360_e16844, assign17360_e16844_d_n2, assign17360_e16844_d_n3, assign17360_e16844_d_n4, assign17360_e16844_d_n7, assign17360_e16844_d_n10, assign17360_e16844_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17360_e16830: f64 = (2.0 * locals.var_fn205_calc_iq__ff);
        let assign17360_e16832: f64 = (assign17360_e16830 * locals.var_fn205_calc_iq__phitin);
        let assign17360_e16834: f64 = (assign17360_e16832 * locals.var_fn205_calc_iq__muf);
        let assign17360_e16836: f64 = (assign17360_e16834 / locals.var_fn205_calc_iq__lin);
        let assign17360_e16839: f64 = (1.0 - locals.var_fn205_calc_iq__ff);
        let assign17360_e16841: f64 = (assign17360_e16839 * locals.var_fn205_calc_iq__vx);
        let assign17360_e16842: f64 = (assign17360_e16836 + assign17360_e16841);
        (assign17360_e16842, ((((((2.0 * locals.var_fn205_calc_iq__ff_dn2) * locals.var_fn205_calc_iq__phitin) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn2)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn2) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn205_calc_iq__ff_dn3) * locals.var_fn205_calc_iq__phitin) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn3)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn3) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn205_calc_iq__ff_dn4) * locals.var_fn205_calc_iq__phitin) + (assign17360_e16830 * locals.var_fn205_calc_iq__phitin_dn4)) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn4)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn4) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn205_calc_iq__ff_dn7) * locals.var_fn205_calc_iq__phitin) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn7)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn7) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn205_calc_iq__ff_dn10) * locals.var_fn205_calc_iq__phitin) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn10)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn10) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn10))), ((((((2.0 * locals.var_fn205_calc_iq__ff_dn11) * locals.var_fn205_calc_iq__phitin) * locals.var_fn205_calc_iq__muf) + (assign17360_e16832 * locals.var_fn205_calc_iq__muf_dn11)) / locals.var_fn205_calc_iq__lin) + (((-locals.var_fn205_calc_iq__ff_dn11) * locals.var_fn205_calc_iq__vx) + (assign17360_e16839 * locals.var_fn205_calc_iq__vx_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__vxf, locals.var_fn205_calc_iq__vxf_dn2, locals.var_fn205_calc_iq__vxf_dn3, locals.var_fn205_calc_iq__vxf_dn4, locals.var_fn205_calc_iq__vxf_dn7, locals.var_fn205_calc_iq__vxf_dn10, locals.var_fn205_calc_iq__vxf_dn11,)
    }
};
        locals.var_fn205_calc_iq__vxf = assign17360_e16844;
        locals.var_fn205_calc_iq__vxf_dn2 = assign17360_e16844_d_n2;
        locals.var_fn205_calc_iq__vxf_dn3 = assign17360_e16844_d_n3;
        locals.var_fn205_calc_iq__vxf_dn4 = assign17360_e16844_d_n4;
        locals.var_fn205_calc_iq__vxf_dn7 = assign17360_e16844_d_n7;
        locals.var_fn205_calc_iq__vxf_dn10 = assign17360_e16844_d_n10;
        locals.var_fn205_calc_iq__vxf_dn11 = assign17360_e16844_d_n11;

        let (assign17370_e16852, assign17370_e16852_d_n2, assign17370_e16852_d_n3, assign17370_e16852_d_n4, assign17370_e16852_d_n7, assign17370_e16852_d_n10, assign17370_e16852_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17370_e16848: f64 = (locals.var_fn205_calc_iq__vx * locals.var_fn205_calc_iq__lin);
        let assign17370_e16850: f64 = (assign17370_e16848 / locals.var_fn205_calc_iq__muf);
        (assign17370_e16850, ((((locals.var_fn205_calc_iq__vx_dn2 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn2)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn3 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn3)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn4 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn4)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn7 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn7)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn10 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn10)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn11 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn11)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)),)
    } else {
        (locals.var_fn205_calc_iq__vdsats, locals.var_fn205_calc_iq__vdsats_dn2, locals.var_fn205_calc_iq__vdsats_dn3, locals.var_fn205_calc_iq__vdsats_dn4, locals.var_fn205_calc_iq__vdsats_dn7, locals.var_fn205_calc_iq__vdsats_dn10, locals.var_fn205_calc_iq__vdsats_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats = assign17370_e16852;
        locals.var_fn205_calc_iq__vdsats_dn2 = assign17370_e16852_d_n2;
        locals.var_fn205_calc_iq__vdsats_dn3 = assign17370_e16852_d_n3;
        locals.var_fn205_calc_iq__vdsats_dn4 = assign17370_e16852_d_n4;
        locals.var_fn205_calc_iq__vdsats_dn7 = assign17370_e16852_d_n7;
        locals.var_fn205_calc_iq__vdsats_dn10 = assign17370_e16852_d_n10;
        locals.var_fn205_calc_iq__vdsats_dn11 = assign17370_e16852_d_n11;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17380_e16869, assign17380_e16869_d_n2, assign17380_e16869_d_n3, assign17380_e16869_d_n4, assign17380_e16869_d_n7, assign17380_e16869_d_n10, assign17380_e16869_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17380_e16858: f64 = (2.0 * locals.var_fn205_calc_iq__qinvv);
        let assign17380_e16860: f64 = (assign17380_e16858 / locals.var_fn205_calc_iq__cgin);
        let assign17380_e16862: f64 = (assign17380_e16860 / locals.var_fn205_calc_iq__vdsats);
        let assign17380_e16863: f64 = (1.0 + assign17380_e16862);
        let assign17380_e16864: f64 = (assign17380_e16863).sqrt();
        let assign17380_e16865: f64 = (locals.var_fn205_calc_iq__vdsats * assign17380_e16864);
        let assign17380_e16867: f64 = (assign17380_e16865 - locals.var_fn205_calc_iq__vdsats);
        (assign17380_e16867, (((locals.var_fn205_calc_iq__vdsats_dn2 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn2)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn2), (((locals.var_fn205_calc_iq__vdsats_dn3 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn3)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn3), (((locals.var_fn205_calc_iq__vdsats_dn4 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17380_e16858 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn4)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn4), (((locals.var_fn205_calc_iq__vdsats_dn7 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn7)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn7), (((locals.var_fn205_calc_iq__vdsats_dn10 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn10)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn10), (((locals.var_fn205_calc_iq__vdsats_dn11 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn11)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vdsats1, locals.var_fn205_calc_iq__vdsats1_dn2, locals.var_fn205_calc_iq__vdsats1_dn3, locals.var_fn205_calc_iq__vdsats1_dn4, locals.var_fn205_calc_iq__vdsats1_dn7, locals.var_fn205_calc_iq__vdsats1_dn10, locals.var_fn205_calc_iq__vdsats1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats1 = assign17380_e16869;
        locals.var_fn205_calc_iq__vdsats1_dn2 = assign17380_e16869_d_n2;
        locals.var_fn205_calc_iq__vdsats1_dn3 = assign17380_e16869_d_n3;
        locals.var_fn205_calc_iq__vdsats1_dn4 = assign17380_e16869_d_n4;
        locals.var_fn205_calc_iq__vdsats1_dn7 = assign17380_e16869_d_n7;
        locals.var_fn205_calc_iq__vdsats1_dn10 = assign17380_e16869_d_n10;
        locals.var_fn205_calc_iq__vdsats1_dn11 = assign17380_e16869_d_n11;

        let (assign17390_e16881, assign17390_e16881_d_n2, assign17390_e16881_d_n3, assign17390_e16881_d_n4, assign17390_e16881_d_n7, assign17390_e16881_d_n10, assign17390_e16881_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17390_e16874: f64 = (1.0 - locals.var_fn205_calc_iq__ff);
        let assign17390_e16875: f64 = (locals.var_fn205_calc_iq__vdsats * assign17390_e16874);
        let assign17390_e16878: f64 = (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff);
        let assign17390_e16879: f64 = (assign17390_e16875 + assign17390_e16878);
        (assign17390_e16879, (((locals.var_fn205_calc_iq__vdsats_dn2 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn2))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn2)), (((locals.var_fn205_calc_iq__vdsats_dn3 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn3))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn3)), (((locals.var_fn205_calc_iq__vdsats_dn4 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit_dn4 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn4))), (((locals.var_fn205_calc_iq__vdsats_dn7 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn7))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn7)), (((locals.var_fn205_calc_iq__vdsats_dn10 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn10))) + ((locals.var_fn205_calc_iq__two_n_phit_dn10 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn10))), (((locals.var_fn205_calc_iq__vdsats_dn11 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn11))) + ((locals.var_fn205_calc_iq__two_n_phit_dn11 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__vdsat, locals.var_fn205_calc_iq__vdsat_dn2, locals.var_fn205_calc_iq__vdsat_dn3, locals.var_fn205_calc_iq__vdsat_dn4, locals.var_fn205_calc_iq__vdsat_dn7, locals.var_fn205_calc_iq__vdsat_dn10, locals.var_fn205_calc_iq__vdsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat = assign17390_e16881;
        locals.var_fn205_calc_iq__vdsat_dn2 = assign17390_e16881_d_n2;
        locals.var_fn205_calc_iq__vdsat_dn3 = assign17390_e16881_d_n3;
        locals.var_fn205_calc_iq__vdsat_dn4 = assign17390_e16881_d_n4;
        locals.var_fn205_calc_iq__vdsat_dn7 = assign17390_e16881_d_n7;
        locals.var_fn205_calc_iq__vdsat_dn10 = assign17390_e16881_d_n10;
        locals.var_fn205_calc_iq__vdsat_dn11 = assign17390_e16881_d_n11;

        let (assign17400_e16893, assign17400_e16893_d_n2, assign17400_e16893_d_n3, assign17400_e16893_d_n4, assign17400_e16893_d_n7, assign17400_e16893_d_n10, assign17400_e16893_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17400_e16886: f64 = (1.0 - locals.var_fn205_calc_iq__ff);
        let assign17400_e16887: f64 = (locals.var_fn205_calc_iq__vdsats1 * assign17400_e16886);
        let assign17400_e16890: f64 = (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff);
        let assign17400_e16891: f64 = (assign17400_e16887 + assign17400_e16890);
        (assign17400_e16891, (((locals.var_fn205_calc_iq__vdsats1_dn2 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn2))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn2)), (((locals.var_fn205_calc_iq__vdsats1_dn3 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn3))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn3)), (((locals.var_fn205_calc_iq__vdsats1_dn4 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit_dn4 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn4))), (((locals.var_fn205_calc_iq__vdsats1_dn7 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn7))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn7)), (((locals.var_fn205_calc_iq__vdsats1_dn10 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn10))) + ((locals.var_fn205_calc_iq__two_n_phit_dn10 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn10))), (((locals.var_fn205_calc_iq__vdsats1_dn11 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn11))) + ((locals.var_fn205_calc_iq__two_n_phit_dn11 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__vdsat1, locals.var_fn205_calc_iq__vdsat1_dn2, locals.var_fn205_calc_iq__vdsat1_dn3, locals.var_fn205_calc_iq__vdsat1_dn4, locals.var_fn205_calc_iq__vdsat1_dn7, locals.var_fn205_calc_iq__vdsat1_dn10, locals.var_fn205_calc_iq__vdsat1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat1 = assign17400_e16893;
        locals.var_fn205_calc_iq__vdsat1_dn2 = assign17400_e16893_d_n2;
        locals.var_fn205_calc_iq__vdsat1_dn3 = assign17400_e16893_d_n3;
        locals.var_fn205_calc_iq__vdsat1_dn4 = assign17400_e16893_d_n4;
        locals.var_fn205_calc_iq__vdsat1_dn7 = assign17400_e16893_d_n7;
        locals.var_fn205_calc_iq__vdsat1_dn10 = assign17400_e16893_d_n10;
        locals.var_fn205_calc_iq__vdsat1_dn11 = assign17400_e16893_d_n11;

        let (assign17410_e16962, assign17410_e16962_d_n2, assign17410_e16962_d_n3, assign17410_e16962_d_n4, assign17410_e16962_d_n7, assign17410_e16962_d_n10, assign17410_e16962_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17410_e16952, assign17410_e16952_d_n2, assign17410_e16952_d_n3, assign17410_e16952_d_n4, assign17410_e16952_d_n7, assign17410_e16952_d_n10, assign17410_e16952_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17410_e16905: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16906: f64 = assign17410_e16905;
                let assign17410_e16910: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16911: f64 = (-assign17410_e16910);
                let assign17410_e16914: f64 = (0.001 / p.p53);
                let assign17410_e16918: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16919: f64 = (-assign17410_e16918);
                let assign17410_e16920: f64 = (assign17410_e16914 * assign17410_e16919);
                let assign17410_e16921: f64 = (assign17410_e16920).tanh();
                let assign17410_e16922: f64 = (assign17410_e16911 * assign17410_e16921);
                let assign17410_e16923: f64 = (assign17410_e16906 + assign17410_e16922);
                let assign17410_e16924: f64 = (0.5 * assign17410_e16923);
                (assign17410_e16924, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))),)
            } else {
                let (assign17410_e16951, assign17410_e16951_d_n2, assign17410_e16951_d_n3, assign17410_e16951_d_n4, assign17410_e16951_d_n7, assign17410_e16951_d_n10, assign17410_e16951_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17410_e16932: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16933: f64 = assign17410_e16932;
                        let assign17410_e16937: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16938: f64 = (-assign17410_e16937);
                        let assign17410_e16942: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16943: f64 = (-assign17410_e16942);
                        let assign17410_e16944: f64 = (assign17410_e16938 * assign17410_e16943);
                        let assign17410_e16946: f64 = (assign17410_e16944 + p.p53);
                        let assign17410_e16947: f64 = (assign17410_e16946).sqrt();
                        let assign17410_e16948: f64 = (assign17410_e16933 + assign17410_e16947);
                        let assign17410_e16949: f64 = (0.5 * assign17410_e16948);
                        (assign17410_e16949, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16943) + (assign17410_e16938 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17410_e16947)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16943) + (assign17410_e16938 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17410_e16947)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17410_e16951, assign17410_e16951_d_n2, assign17410_e16951_d_n3, assign17410_e16951_d_n4, assign17410_e16951_d_n7, assign17410_e16951_d_n10, assign17410_e16951_d_n11,)
            }
        };
        let assign17410_e16954: f64 = (assign17410_e16952).powf(locals.var_fn205_calc_iq__beta);
        let assign17410_e16955: f64 = (1.0 + assign17410_e16954);
        let assign17410_e16958: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17410_e16959: f64 = (assign17410_e16955).powf(assign17410_e16958);
        let assign17410_e16960: f64 = (1.0 / assign17410_e16959);
        (assign17410_e16960, (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n2)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n2 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n2)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n2 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n3)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n3 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n3)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n3 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n4)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n4 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n4)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n4 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n7)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n7 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n7)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n7 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n10)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n10 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n10)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n10 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n11)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n11 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n11)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n11 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))),)
    } else {
        (locals.var_fn205_calc_iq__fsd, locals.var_fn205_calc_iq__fsd_dn2, locals.var_fn205_calc_iq__fsd_dn3, locals.var_fn205_calc_iq__fsd_dn4, locals.var_fn205_calc_iq__fsd_dn7, locals.var_fn205_calc_iq__fsd_dn10, locals.var_fn205_calc_iq__fsd_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd = assign17410_e16962;
        locals.var_fn205_calc_iq__fsd_dn2 = assign17410_e16962_d_n2;
        locals.var_fn205_calc_iq__fsd_dn3 = assign17410_e16962_d_n3;
        locals.var_fn205_calc_iq__fsd_dn4 = assign17410_e16962_d_n4;
        locals.var_fn205_calc_iq__fsd_dn7 = assign17410_e16962_d_n7;
        locals.var_fn205_calc_iq__fsd_dn10 = assign17410_e16962_d_n10;
        locals.var_fn205_calc_iq__fsd_dn11 = assign17410_e16962_d_n11;

        let (assign17420_e16968, assign17420_e16968_d_n2, assign17420_e16968_d_n3, assign17420_e16968_d_n4, assign17420_e16968_d_n7, assign17420_e16968_d_n10, assign17420_e16968_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17420_e16966: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd);
        (assign17420_e16966, (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn2), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn3), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn4), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn7), ((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__fsd) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn10)), ((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__fsd) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdx, locals.var_fn205_calc_iq__vdx_dn2, locals.var_fn205_calc_iq__vdx_dn3, locals.var_fn205_calc_iq__vdx_dn4, locals.var_fn205_calc_iq__vdx_dn7, locals.var_fn205_calc_iq__vdx_dn10, locals.var_fn205_calc_iq__vdx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx = assign17420_e16968;
        locals.var_fn205_calc_iq__vdx_dn2 = assign17420_e16968_d_n2;
        locals.var_fn205_calc_iq__vdx_dn3 = assign17420_e16968_d_n3;
        locals.var_fn205_calc_iq__vdx_dn4 = assign17420_e16968_d_n4;
        locals.var_fn205_calc_iq__vdx_dn7 = assign17420_e16968_d_n7;
        locals.var_fn205_calc_iq__vdx_dn10 = assign17420_e16968_d_n10;
        locals.var_fn205_calc_iq__vdx_dn11 = assign17420_e16968_d_n11;

        let (assign17430_e17043, assign17430_e17043_d_n2, assign17430_e17043_d_n3, assign17430_e17043_d_n4, assign17430_e17043_d_n7, assign17430_e17043_d_n10, assign17430_e17043_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17430_e17033, assign17430_e17033_d_n2, assign17430_e17033_d_n3, assign17430_e17033_d_n4, assign17430_e17033_d_n7, assign17430_e17033_d_n10, assign17430_e17033_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17430_e16979: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16981: f64 = (assign17430_e16979 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16982: f64 = assign17430_e16981;
                let assign17430_e16985: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16987: f64 = (assign17430_e16985 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16988: f64 = (-assign17430_e16987);
                let assign17430_e16991: f64 = (0.001 / p.p53);
                let assign17430_e16994: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16996: f64 = (assign17430_e16994 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16997: f64 = (-assign17430_e16996);
                let assign17430_e16998: f64 = (assign17430_e16991 * assign17430_e16997);
                let assign17430_e16999: f64 = (assign17430_e16998).tanh();
                let assign17430_e17000: f64 = (assign17430_e16988 * assign17430_e16999);
                let assign17430_e17001: f64 = (assign17430_e16982 + assign17430_e17000);
                let assign17430_e17002: f64 = (0.5 * assign17430_e17001);
                (assign17430_e17002, (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))),)
            } else {
                let (assign17430_e17032, assign17430_e17032_d_n2, assign17430_e17032_d_n3, assign17430_e17032_d_n4, assign17430_e17032_d_n7, assign17430_e17032_d_n10, assign17430_e17032_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17430_e17009: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17011: f64 = (assign17430_e17009 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17012: f64 = assign17430_e17011;
                        let assign17430_e17015: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17017: f64 = (assign17430_e17015 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17018: f64 = (-assign17430_e17017);
                        let assign17430_e17021: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17023: f64 = (assign17430_e17021 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17024: f64 = (-assign17430_e17023);
                        let assign17430_e17025: f64 = (assign17430_e17018 * assign17430_e17024);
                        let assign17430_e17027: f64 = (assign17430_e17025 + p.p53);
                        let assign17430_e17028: f64 = (assign17430_e17027).sqrt();
                        let assign17430_e17029: f64 = (assign17430_e17012 + assign17430_e17028);
                        let assign17430_e17030: f64 = (0.5 * assign17430_e17029);
                        (assign17430_e17030, (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e17024) + (assign17430_e17018 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17430_e17028)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e17024) + (assign17430_e17018 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17430_e17028)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17430_e17032, assign17430_e17032_d_n2, assign17430_e17032_d_n3, assign17430_e17032_d_n4, assign17430_e17032_d_n7, assign17430_e17032_d_n10, assign17430_e17032_d_n11,)
            }
        };
        let assign17430_e17035: f64 = (assign17430_e17033).powf(locals.var_fn205_calc_iq__beta);
        let assign17430_e17036: f64 = (1.0 + assign17430_e17035);
        let assign17430_e17039: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17430_e17040: f64 = (assign17430_e17036).powf(assign17430_e17039);
        let assign17430_e17041: f64 = (1.0 / assign17430_e17040);
        (assign17430_e17041, (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n2)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n2 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n2)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n2 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n3)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n3 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n3)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n3 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n4)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n4 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n4)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n4 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n7)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n7 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n7)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n7 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n10)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n10 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n10)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n10 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n11)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n11 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n11)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n11 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))),)
    } else {
        (locals.var_fn205_calc_iq__fds, locals.var_fn205_calc_iq__fds_dn2, locals.var_fn205_calc_iq__fds_dn3, locals.var_fn205_calc_iq__fds_dn4, locals.var_fn205_calc_iq__fds_dn7, locals.var_fn205_calc_iq__fds_dn10, locals.var_fn205_calc_iq__fds_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds = assign17430_e17043;
        locals.var_fn205_calc_iq__fds_dn2 = assign17430_e17043_d_n2;
        locals.var_fn205_calc_iq__fds_dn3 = assign17430_e17043_d_n3;
        locals.var_fn205_calc_iq__fds_dn4 = assign17430_e17043_d_n4;
        locals.var_fn205_calc_iq__fds_dn7 = assign17430_e17043_d_n7;
        locals.var_fn205_calc_iq__fds_dn10 = assign17430_e17043_d_n10;
        locals.var_fn205_calc_iq__fds_dn11 = assign17430_e17043_d_n11;

        let (assign17440_e17050, assign17440_e17050_d_n2, assign17440_e17050_d_n3, assign17440_e17050_d_n4, assign17440_e17050_d_n7, assign17440_e17050_d_n10, assign17440_e17050_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17440_e17046: f64 = (-locals.var_fn205_calc_iq__vdsin);
        let assign17440_e17048: f64 = (assign17440_e17046 * locals.var_fn205_calc_iq__fds);
        (assign17440_e17048, (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn2), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn3), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn4), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn7), (((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__fds) + (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn10)), (((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__fds) + (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vsx, locals.var_fn205_calc_iq__vsx_dn2, locals.var_fn205_calc_iq__vsx_dn3, locals.var_fn205_calc_iq__vsx_dn4, locals.var_fn205_calc_iq__vsx_dn7, locals.var_fn205_calc_iq__vsx_dn10, locals.var_fn205_calc_iq__vsx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx = assign17440_e17050;
        locals.var_fn205_calc_iq__vsx_dn2 = assign17440_e17050_d_n2;
        locals.var_fn205_calc_iq__vsx_dn3 = assign17440_e17050_d_n3;
        locals.var_fn205_calc_iq__vsx_dn4 = assign17440_e17050_d_n4;
        locals.var_fn205_calc_iq__vsx_dn7 = assign17440_e17050_d_n7;
        locals.var_fn205_calc_iq__vsx_dn10 = assign17440_e17050_d_n10;
        locals.var_fn205_calc_iq__vsx_dn11 = assign17440_e17050_d_n11;

        let (assign17450_e17058, assign17450_e17058_d_n2, assign17450_e17058_d_n3, assign17450_e17058_d_n4, assign17450_e17058_d_n7, assign17450_e17058_d_n10, assign17450_e17058_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17450_e17054: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__myarg);
        let assign17450_e17056: f64 = (assign17450_e17054 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17450_e17056, ((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17450_e17054 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17450_e17058;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17450_e17058_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17450_e17058_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17450_e17058_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17450_e17058_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17450_e17058_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17450_e17058_d_n11;

        let assign17460_e17061: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard211 = assign17460_e17061;

        let (assign17470_e17067, assign17470_e17067_d_n2, assign17470_e17067_d_n3, assign17470_e17067_d_n4, assign17470_e17067_d_n7, assign17470_e17067_d_n10, assign17470_e17067_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard211 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17470_e17067;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17470_e17067_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17470_e17067_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17470_e17067_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17470_e17067_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17470_e17067_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17470_e17067_d_n11;

        let assign17480_e17070: f64 = (-50.0);
        let assign17480_e17071: f64 = if locals.var_fn205_calc_iq__exparg < assign17480_e17070 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign17480_e17071;

        let (assign17490_e17080, assign17490_e17080_d_n2, assign17490_e17080_d_n3, assign17490_e17080_d_n4, assign17490_e17080_d_n7, assign17490_e17080_d_n10, assign17490_e17080_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17490_e17080;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17490_e17080_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17490_e17080_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17490_e17080_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17490_e17080_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17490_e17080_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17490_e17080_d_n11;

        let (assign17500_e17095, assign17500_e17095_d_n2, assign17500_e17095_d_n3, assign17500_e17095_d_n4, assign17500_e17095_d_n7, assign17500_e17095_d_n10, assign17500_e17095_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 == 0.0)) {
        let assign17500_e17091: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17500_e17092: f64 = (1.0 + assign17500_e17091);
        let assign17500_e17093: f64 = (1.0 / assign17500_e17092);
        (assign17500_e17093, (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17500_e17092 * assign17500_e17092))),)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17500_e17095;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17500_e17095_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17500_e17095_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17500_e17095_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17500_e17095_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17500_e17095_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17500_e17095_d_n11;

        let (assign17510_e17113, assign17510_e17113_d_n2, assign17510_e17113_d_n3, assign17510_e17113_d_n4, assign17510_e17113_d_n7, assign17510_e17113_d_n10, assign17510_e17113_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17510_e17099: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__vsx);
        let assign17510_e17103: f64 = (p.p51 * 0.1);
        let assign17510_e17105: f64 = (assign17510_e17103 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17510_e17107: f64 = (assign17510_e17105 * locals.var_fn205_calc_iq__ffs);
        let assign17510_e17108: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17510_e17107);
        let assign17510_e17109: f64 = (assign17510_e17099 - assign17510_e17108);
        let assign17510_e17111: f64 = (assign17510_e17109 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17510_e17111, (((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__vsx_dn2) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn2))) / locals.var_fn205_calc_iq__two_n_phit), (((-locals.var_fn205_calc_iq__vsx_dn3) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn3))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vsx_dn4) - (locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17510_e17103 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffs) + (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__vsx_dn7) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn7))) / locals.var_fn205_calc_iq__two_n_phit), (((((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__vsx_dn10) - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__vsx_dn11) - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__etas, locals.var_fn205_calc_iq__etas_dn2, locals.var_fn205_calc_iq__etas_dn3, locals.var_fn205_calc_iq__etas_dn4, locals.var_fn205_calc_iq__etas_dn7, locals.var_fn205_calc_iq__etas_dn10, locals.var_fn205_calc_iq__etas_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas = assign17510_e17113;
        locals.var_fn205_calc_iq__etas_dn2 = assign17510_e17113_d_n2;
        locals.var_fn205_calc_iq__etas_dn3 = assign17510_e17113_d_n3;
        locals.var_fn205_calc_iq__etas_dn4 = assign17510_e17113_d_n4;
        locals.var_fn205_calc_iq__etas_dn7 = assign17510_e17113_d_n7;
        locals.var_fn205_calc_iq__etas_dn10 = assign17510_e17113_d_n10;
        locals.var_fn205_calc_iq__etas_dn11 = assign17510_e17113_d_n11;

        let assign17520_e17116: f64 = if locals.var_fn205_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign17520_e17116;

        let (assign17530_e17124, assign17530_e17124_d_n2, assign17530_e17124_d_n3, assign17530_e17124_d_n4, assign17530_e17124_d_n7, assign17530_e17124_d_n10, assign17530_e17124_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard213 != 0.0)) {
        let assign17530_e17122: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas);
        (assign17530_e17122, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17530_e17124;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17530_e17124_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17530_e17124_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17530_e17124_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17530_e17124_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17530_e17124_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17530_e17124_d_n11;

        let assign17540_e17127: f64 = (-50.0);
        let assign17540_e17128: f64 = if locals.var_fn205_calc_iq__etas < assign17540_e17127 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign17540_e17128;

        let (assign17550_e17140, assign17550_e17140_d_n2, assign17550_e17140_d_n3, assign17550_e17140_d_n4, assign17550_e17140_d_n7, assign17550_e17140_d_n10, assign17550_e17140_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) {
        let assign17550_e17137: f64 = (locals.var_fn205_calc_iq__etas).exp();
        let assign17550_e17138: f64 = (locals.var_fn205_calc_iq__qref * assign17550_e17137);
        (assign17550_e17138, (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn2)), (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn4))), (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17550_e17140;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17550_e17140_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17550_e17140_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17550_e17140_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17550_e17140_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17550_e17140_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17550_e17140_d_n11;

        let (assign17560_e17156, assign17560_e17156_d_n2, assign17560_e17156_d_n3, assign17560_e17156_d_n4, assign17560_e17156_d_n7, assign17560_e17156_d_n10, assign17560_e17156_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) {
        let assign17560_e17151: f64 = (locals.var_fn205_calc_iq__etas).exp();
        let assign17560_e17152: f64 = (1.0 + assign17560_e17151);
        let assign17560_e17153: f64 = (assign17560_e17152).ln();
        let assign17560_e17154: f64 = (locals.var_fn205_calc_iq__qref * assign17560_e17153);
        (assign17560_e17154, (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn2) / assign17560_e17152)), (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn3) / assign17560_e17152)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn4) / assign17560_e17152))), (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn7) / assign17560_e17152)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn10) / assign17560_e17152))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn11) / assign17560_e17152))),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17560_e17156;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17560_e17156_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17560_e17156_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17560_e17156_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17560_e17156_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17560_e17156_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17560_e17156_d_n11;

        let (assign17570_e17164, assign17570_e17164_d_n2, assign17570_e17164_d_n3, assign17570_e17164_d_n4, assign17570_e17164_d_n7, assign17570_e17164_d_n10, assign17570_e17164_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17570_e17160: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__myarg);
        let assign17570_e17162: f64 = (assign17570_e17160 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17570_e17162, ((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17570_e17160 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17570_e17164;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17570_e17164_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17570_e17164_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17570_e17164_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17570_e17164_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17570_e17164_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17570_e17164_d_n11;

        let assign17580_e17167: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard215 = assign17580_e17167;

        let (assign17590_e17173, assign17590_e17173_d_n2, assign17590_e17173_d_n3, assign17590_e17173_d_n4, assign17590_e17173_d_n7, assign17590_e17173_d_n10, assign17590_e17173_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard215 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17590_e17173;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17590_e17173_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17590_e17173_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17590_e17173_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17590_e17173_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17590_e17173_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17590_e17173_d_n11;

        let assign17600_e17176: f64 = (-50.0);
        let assign17600_e17177: f64 = if locals.var_fn205_calc_iq__exparg < assign17600_e17176 { 1.0 } else { 0.0 };
        locals.var_guard216 = assign17600_e17177;

        let (assign17610_e17186, assign17610_e17186_d_n2, assign17610_e17186_d_n3, assign17610_e17186_d_n4, assign17610_e17186_d_n7, assign17610_e17186_d_n10, assign17610_e17186_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard215 == 0.0)) && (locals.var_guard216 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17610_e17186;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17610_e17186_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17610_e17186_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17610_e17186_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17610_e17186_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17610_e17186_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17610_e17186_d_n11;

        let (assign17620_e17201, assign17620_e17201_d_n2, assign17620_e17201_d_n3, assign17620_e17201_d_n4, assign17620_e17201_d_n7, assign17620_e17201_d_n10, assign17620_e17201_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard215 == 0.0)) && (locals.var_guard216 == 0.0)) {
        let assign17620_e17197: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17620_e17198: f64 = (1.0 + assign17620_e17197);
        let assign17620_e17199: f64 = (1.0 / assign17620_e17198);
        (assign17620_e17199, (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17620_e17198 * assign17620_e17198))),)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17620_e17201;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17620_e17201_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17620_e17201_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17620_e17201_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17620_e17201_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17620_e17201_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17620_e17201_d_n11;

        let (assign17630_e17219, assign17630_e17219_d_n2, assign17630_e17219_d_n3, assign17630_e17219_d_n4, assign17630_e17219_d_n7, assign17630_e17219_d_n10, assign17630_e17219_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17630_e17205: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdx);
        let assign17630_e17209: f64 = (p.p51 * 0.1);
        let assign17630_e17211: f64 = (assign17630_e17209 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17630_e17213: f64 = (assign17630_e17211 * locals.var_fn205_calc_iq__ffd);
        let assign17630_e17214: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17630_e17213);
        let assign17630_e17215: f64 = (assign17630_e17205 - assign17630_e17214);
        let assign17630_e17217: f64 = (assign17630_e17215 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17630_e17217, (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vdx_dn2) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn2))) / locals.var_fn205_calc_iq__two_n_phit), (((-locals.var_fn205_calc_iq__vdx_dn3) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn3))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vdx_dn4) - (locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17630_e17209 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffd) + (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vdx_dn7) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn7))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vdx_dn10) - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdx_dn11) - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__etad, locals.var_fn205_calc_iq__etad_dn2, locals.var_fn205_calc_iq__etad_dn3, locals.var_fn205_calc_iq__etad_dn4, locals.var_fn205_calc_iq__etad_dn7, locals.var_fn205_calc_iq__etad_dn10, locals.var_fn205_calc_iq__etad_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad = assign17630_e17219;
        locals.var_fn205_calc_iq__etad_dn2 = assign17630_e17219_d_n2;
        locals.var_fn205_calc_iq__etad_dn3 = assign17630_e17219_d_n3;
        locals.var_fn205_calc_iq__etad_dn4 = assign17630_e17219_d_n4;
        locals.var_fn205_calc_iq__etad_dn7 = assign17630_e17219_d_n7;
        locals.var_fn205_calc_iq__etad_dn10 = assign17630_e17219_d_n10;
        locals.var_fn205_calc_iq__etad_dn11 = assign17630_e17219_d_n11;

        let assign17640_e17222: f64 = if locals.var_fn205_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign17640_e17222;

        let (assign17650_e17230, assign17650_e17230_d_n2, assign17650_e17230_d_n3, assign17650_e17230_d_n4, assign17650_e17230_d_n7, assign17650_e17230_d_n10, assign17650_e17230_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign17650_e17228: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad);
        (assign17650_e17228, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17650_e17230;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17650_e17230_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17650_e17230_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17650_e17230_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17650_e17230_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17650_e17230_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17650_e17230_d_n11;

        let assign17660_e17233: f64 = (-50.0);
        let assign17660_e17234: f64 = if locals.var_fn205_calc_iq__etad < assign17660_e17233 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign17660_e17234;

        let (assign17670_e17246, assign17670_e17246_d_n2, assign17670_e17246_d_n3, assign17670_e17246_d_n4, assign17670_e17246_d_n7, assign17670_e17246_d_n10, assign17670_e17246_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard218 != 0.0)) {
        let assign17670_e17243: f64 = (locals.var_fn205_calc_iq__etad).exp();
        let assign17670_e17244: f64 = (locals.var_fn205_calc_iq__qref * assign17670_e17243);
        (assign17670_e17244, (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn2)), (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn4))), (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17670_e17246;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17670_e17246_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17670_e17246_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17670_e17246_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17670_e17246_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17670_e17246_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17670_e17246_d_n11;

    }
}
