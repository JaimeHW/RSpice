#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_201(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57180_e89045, assign57180_e89045_d_n0, assign57180_e89045_d_n2, assign57180_e89045_d_n4, assign57180_e89045_d_n5, assign57180_e89045_d_n6, assign57180_e89045_d_n7, assign57180_e89045_d_n8, assign57180_e89045_d_n9, assign57180_e89045_d_n10, assign57180_e89045_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57180_e89033: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign57180_e89036: f64 = (1.0 / 2.0);
        let assign57180_e89037: f64 = (assign57180_e89033).powf(assign57180_e89036);
        let assign57180_e89041: f64 = (1.0 / 2.0);
        let assign57180_e89042: f64 = (locals.var_tmf2).powf(assign57180_e89041);
        let assign57180_e89043: f64 = (assign57180_e89037 - assign57180_e89042);
        (assign57180_e89043, (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn0)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn0 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn2)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn2 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn4)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn4 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn5)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn5 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn6)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn6 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn7)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn7 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn8)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn8 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn9)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn9 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn10)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn10 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57180_e89036) as f64).is_finite() && ((assign57180_e89036) as f64).fract() == 0.0 { if assign57180_e89036 == 0.0 { 0.0 } else { (assign57180_e89036 * ((assign57180_e89033).powf(assign57180_e89036 - 1.0) * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))) } } else { (assign57180_e89037 * (assign57180_e89036 * ((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) / assign57180_e89033))) } - if 0.0 == 0.0 && ((assign57180_e89041) as f64).is_finite() && ((assign57180_e89041) as f64).fract() == 0.0 { if assign57180_e89041 == 0.0 { 0.0 } else { (assign57180_e89041 * ((locals.var_tmf2).powf(assign57180_e89041 - 1.0) * locals.var_tmf2_dn13)) } } else { (assign57180_e89042 * (assign57180_e89041 * (locals.var_tmf2_dn13 / locals.var_tmf2))) }),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57180_e89045;
        locals.var_t0_dn0 = assign57180_e89045_d_n0;
        locals.var_t0_dn2 = assign57180_e89045_d_n2;
        locals.var_t0_dn4 = assign57180_e89045_d_n4;
        locals.var_t0_dn5 = assign57180_e89045_d_n5;
        locals.var_t0_dn6 = assign57180_e89045_d_n6;
        locals.var_t0_dn7 = assign57180_e89045_d_n7;
        locals.var_t0_dn8 = assign57180_e89045_d_n8;
        locals.var_t0_dn9 = assign57180_e89045_d_n9;
        locals.var_t0_dn10 = assign57180_e89045_d_n10;
        locals.var_t0_dn13 = assign57180_e89045_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57190_e89060, assign57190_e89060_d_n0, assign57190_e89060_d_n2, assign57190_e89060_d_n4, assign57190_e89060_d_n5, assign57190_e89060_d_n6, assign57190_e89060_d_n7, assign57190_e89060_d_n8, assign57190_e89060_d_n9, assign57190_e89060_d_n10, assign57190_e89060_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57190_e89057: f64 = (locals.var_leff - p.p402);
        let assign57190_e89058: f64 = (locals.var_t0 / assign57190_e89057);
        (assign57190_e89058, (locals.var_t0_dn0 / assign57190_e89057), (locals.var_t0_dn2 / assign57190_e89057), (locals.var_t0_dn4 / assign57190_e89057), (locals.var_t0_dn5 / assign57190_e89057), (locals.var_t0_dn6 / assign57190_e89057), (locals.var_t0_dn7 / assign57190_e89057), (locals.var_t0_dn8 / assign57190_e89057), (locals.var_t0_dn9 / assign57190_e89057), (locals.var_t0_dn10 / assign57190_e89057), (locals.var_t0_dn13 / assign57190_e89057),)
    } else {
        (locals.var_edri2, locals.var_edri2_dn0, locals.var_edri2_dn2, locals.var_edri2_dn4, locals.var_edri2_dn5, locals.var_edri2_dn6, locals.var_edri2_dn7, locals.var_edri2_dn8, locals.var_edri2_dn9, locals.var_edri2_dn10, locals.var_edri2_dn13,)
    }
};
        locals.var_edri2 = assign57190_e89060;
        locals.var_edri2_dn0 = assign57190_e89060_d_n0;
        locals.var_edri2_dn2 = assign57190_e89060_d_n2;
        locals.var_edri2_dn4 = assign57190_e89060_d_n4;
        locals.var_edri2_dn5 = assign57190_e89060_d_n5;
        locals.var_edri2_dn6 = assign57190_e89060_d_n6;
        locals.var_edri2_dn7 = assign57190_e89060_d_n7;
        locals.var_edri2_dn8 = assign57190_e89060_d_n8;
        locals.var_edri2_dn9 = assign57190_e89060_d_n9;
        locals.var_edri2_dn10 = assign57190_e89060_d_n10;
        locals.var_edri2_dn13 = assign57190_e89060_d_n13;
        locals.var_edri2_rv = 0.0;

        let (assign57200_e89075, assign57200_e89075_d_n0, assign57200_e89075_d_n2, assign57200_e89075_d_n4, assign57200_e89075_d_n5, assign57200_e89075_d_n6, assign57200_e89075_d_n7, assign57200_e89075_d_n8, assign57200_e89075_d_n9, assign57200_e89075_d_n10, assign57200_e89075_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57200_e89071: f64 = (locals.var_muun * locals.var_edri2);
        let assign57200_e89073: f64 = (assign57200_e89071 / locals.var_uc_depvmax);
        (assign57200_e89073, (((((locals.var_muun_dn0 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn0)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn2)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn4)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn5)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn6)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn7)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn8)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn9)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn10)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn13 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn13)) * locals.var_uc_depvmax) - (assign57200_e89071 * locals.var_uc_depvmax_dn13)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57200_e89075;
        locals.var_t1_dn0 = assign57200_e89075_d_n0;
        locals.var_t1_dn2 = assign57200_e89075_d_n2;
        locals.var_t1_dn4 = assign57200_e89075_d_n4;
        locals.var_t1_dn5 = assign57200_e89075_d_n5;
        locals.var_t1_dn6 = assign57200_e89075_d_n6;
        locals.var_t1_dn7 = assign57200_e89075_d_n7;
        locals.var_t1_dn8 = assign57200_e89075_d_n8;
        locals.var_t1_dn9 = assign57200_e89075_d_n9;
        locals.var_t1_dn10 = assign57200_e89075_d_n10;
        locals.var_t1_dn13 = assign57200_e89075_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57210_e89093, assign57210_e89093_d_n0, assign57210_e89093_d_n2, assign57210_e89093_d_n4, assign57210_e89093_d_n5, assign57210_e89093_d_n6, assign57210_e89093_d_n7, assign57210_e89093_d_n8, assign57210_e89093_d_n9, assign57210_e89093_d_n10, assign57210_e89093_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign57210_e89091, assign57210_e89091_d_n0, assign57210_e89091_d_n2, assign57210_e89091_d_n4, assign57210_e89091_d_n5, assign57210_e89091_d_n6, assign57210_e89091_d_n7, assign57210_e89091_d_n8, assign57210_e89091_d_n9, assign57210_e89091_d_n10, assign57210_e89091_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57210_e89090: f64 = (locals.var_t1).powf(p.p378);
                (assign57210_e89090, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn13)) } } else { (assign57210_e89090 * (p.p378 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign57210_e89091, assign57210_e89091_d_n0, assign57210_e89091_d_n2, assign57210_e89091_d_n4, assign57210_e89091_d_n5, assign57210_e89091_d_n6, assign57210_e89091_d_n7, assign57210_e89091_d_n8, assign57210_e89091_d_n9, assign57210_e89091_d_n10, assign57210_e89091_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57210_e89093;
        locals.var_t2_dn0 = assign57210_e89093_d_n0;
        locals.var_t2_dn2 = assign57210_e89093_d_n2;
        locals.var_t2_dn4 = assign57210_e89093_d_n4;
        locals.var_t2_dn5 = assign57210_e89093_d_n5;
        locals.var_t2_dn6 = assign57210_e89093_d_n6;
        locals.var_t2_dn7 = assign57210_e89093_d_n7;
        locals.var_t2_dn8 = assign57210_e89093_d_n8;
        locals.var_t2_dn9 = assign57210_e89093_d_n9;
        locals.var_t2_dn10 = assign57210_e89093_d_n10;
        locals.var_t2_dn13 = assign57210_e89093_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57220_e89106, assign57220_e89106_d_n0, assign57220_e89106_d_n2, assign57220_e89106_d_n4, assign57220_e89106_d_n5, assign57220_e89106_d_n6, assign57220_e89106_d_n7, assign57220_e89106_d_n8, assign57220_e89106_d_n9, assign57220_e89106_d_n10, assign57220_e89106_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57220_e89104: f64 = (1.0 + locals.var_t2);
        (assign57220_e89104, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign57220_e89106;
        locals.var_t3_dn0 = assign57220_e89106_d_n0;
        locals.var_t3_dn2 = assign57220_e89106_d_n2;
        locals.var_t3_dn4 = assign57220_e89106_d_n4;
        locals.var_t3_dn5 = assign57220_e89106_d_n5;
        locals.var_t3_dn6 = assign57220_e89106_d_n6;
        locals.var_t3_dn7 = assign57220_e89106_d_n7;
        locals.var_t3_dn8 = assign57220_e89106_d_n8;
        locals.var_t3_dn9 = assign57220_e89106_d_n9;
        locals.var_t3_dn10 = assign57220_e89106_d_n10;
        locals.var_t3_dn13 = assign57220_e89106_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign57230_e89126, assign57230_e89126_d_n0, assign57230_e89126_d_n2, assign57230_e89126_d_n4, assign57230_e89126_d_n5, assign57230_e89126_d_n6, assign57230_e89126_d_n7, assign57230_e89126_d_n8, assign57230_e89126_d_n9, assign57230_e89126_d_n10, assign57230_e89126_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign57230_e89124, assign57230_e89124_d_n0, assign57230_e89124_d_n2, assign57230_e89124_d_n4, assign57230_e89124_d_n5, assign57230_e89124_d_n6, assign57230_e89124_d_n7, assign57230_e89124_d_n8, assign57230_e89124_d_n9, assign57230_e89124_d_n10, assign57230_e89124_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57230_e89122: f64 = (1.0 / p.p378);
                let assign57230_e89123: f64 = (locals.var_t3).powf(assign57230_e89122);
                (assign57230_e89123, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn0)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn2)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn4)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn5)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn6)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn7)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn8)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn9)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn10)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57230_e89122) as f64).is_finite() && ((assign57230_e89122) as f64).fract() == 0.0 { if assign57230_e89122 == 0.0 { 0.0 } else { (assign57230_e89122 * ((locals.var_t3).powf(assign57230_e89122 - 1.0) * locals.var_t3_dn13)) } } else { (assign57230_e89123 * (assign57230_e89122 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign57230_e89124, assign57230_e89124_d_n0, assign57230_e89124_d_n2, assign57230_e89124_d_n4, assign57230_e89124_d_n5, assign57230_e89124_d_n6, assign57230_e89124_d_n7, assign57230_e89124_d_n8, assign57230_e89124_d_n9, assign57230_e89124_d_n10, assign57230_e89124_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign57230_e89126;
        locals.var_t4_dn0 = assign57230_e89126_d_n0;
        locals.var_t4_dn2 = assign57230_e89126_d_n2;
        locals.var_t4_dn4 = assign57230_e89126_d_n4;
        locals.var_t4_dn5 = assign57230_e89126_d_n5;
        locals.var_t4_dn6 = assign57230_e89126_d_n6;
        locals.var_t4_dn7 = assign57230_e89126_d_n7;
        locals.var_t4_dn8 = assign57230_e89126_d_n8;
        locals.var_t4_dn9 = assign57230_e89126_d_n9;
        locals.var_t4_dn10 = assign57230_e89126_d_n10;
        locals.var_t4_dn13 = assign57230_e89126_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign57240_e89139, assign57240_e89139_d_n0, assign57240_e89139_d_n2, assign57240_e89139_d_n4, assign57240_e89139_d_n5, assign57240_e89139_d_n6, assign57240_e89139_d_n7, assign57240_e89139_d_n8, assign57240_e89139_d_n9, assign57240_e89139_d_n10, assign57240_e89139_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57240_e89137: f64 = (locals.var_muun / locals.var_t4);
        (assign57240_e89137, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn13 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res, locals.var_mu_res_dn0, locals.var_mu_res_dn2, locals.var_mu_res_dn4, locals.var_mu_res_dn5, locals.var_mu_res_dn6, locals.var_mu_res_dn7, locals.var_mu_res_dn8, locals.var_mu_res_dn9, locals.var_mu_res_dn10, locals.var_mu_res_dn13,)
    }
};
        locals.var_mu_res = assign57240_e89139;
        locals.var_mu_res_dn0 = assign57240_e89139_d_n0;
        locals.var_mu_res_dn2 = assign57240_e89139_d_n2;
        locals.var_mu_res_dn4 = assign57240_e89139_d_n4;
        locals.var_mu_res_dn5 = assign57240_e89139_d_n5;
        locals.var_mu_res_dn6 = assign57240_e89139_d_n6;
        locals.var_mu_res_dn7 = assign57240_e89139_d_n7;
        locals.var_mu_res_dn8 = assign57240_e89139_d_n8;
        locals.var_mu_res_dn9 = assign57240_e89139_d_n9;
        locals.var_mu_res_dn10 = assign57240_e89139_d_n10;
        locals.var_mu_res_dn13 = assign57240_e89139_d_n13;
        locals.var_mu_res_rv = 0.0;

        let (assign57250_e89168, assign57250_e89168_d_n0, assign57250_e89168_d_n2, assign57250_e89168_d_n4, assign57250_e89168_d_n5, assign57250_e89168_d_n6, assign57250_e89168_d_n7, assign57250_e89168_d_n8, assign57250_e89168_d_n9, assign57250_e89168_d_n10, assign57250_e89168_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57250_e89152: f64 = (p.p400 * locals.var_edri__blk1115);
        let assign57250_e89158: f64 = (locals.var_muun * locals.var_edri__blk1115);
        let assign57250_e89160: f64 = (assign57250_e89158 / locals.var_uc_depvmax);
        let assign57250_e89161: f64 = (1.0 + assign57250_e89160);
        let assign57250_e89162: f64 = (1.0 / assign57250_e89161);
        let assign57250_e89163: f64 = (1.0 - assign57250_e89162);
        let assign57250_e89164: f64 = (assign57250_e89152 * assign57250_e89163);
        let assign57250_e89165: f64 = (1.0 + assign57250_e89164);
        let assign57250_e89166: f64 = (locals.var_uc_ndepm * assign57250_e89165);
        (assign57250_e89166, ((locals.var_uc_ndepm_dn0 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn0) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn0 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn0)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn2 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn2) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn2 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn2)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn4 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn4) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn4 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn4)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn5 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn5) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn5 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn5)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn6 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn6) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn6 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn6)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn7 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn7) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn7 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn7)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn8 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn8) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn8 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn8)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn9 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn9) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn9 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn9)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn10 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn10) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn10 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn10)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))), ((locals.var_uc_ndepm_dn13 * assign57250_e89165) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1115_dn13) * assign57250_e89163) + (assign57250_e89152 * (-(-((((((locals.var_muun_dn13 * locals.var_edri__blk1115) + (locals.var_muun * locals.var_edri__blk1115_dn13)) * locals.var_uc_depvmax) - (assign57250_e89158 * locals.var_uc_depvmax_dn13)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57250_e89161 * assign57250_e89161)))))))),)
    } else {
        (locals.var_n_res, locals.var_n_res_dn0, locals.var_n_res_dn2, locals.var_n_res_dn4, locals.var_n_res_dn5, locals.var_n_res_dn6, locals.var_n_res_dn7, locals.var_n_res_dn8, locals.var_n_res_dn9, locals.var_n_res_dn10, locals.var_n_res_dn13,)
    }
};
        locals.var_n_res = assign57250_e89168;
        locals.var_n_res_dn0 = assign57250_e89168_d_n0;
        locals.var_n_res_dn2 = assign57250_e89168_d_n2;
        locals.var_n_res_dn4 = assign57250_e89168_d_n4;
        locals.var_n_res_dn5 = assign57250_e89168_d_n5;
        locals.var_n_res_dn6 = assign57250_e89168_d_n6;
        locals.var_n_res_dn7 = assign57250_e89168_d_n7;
        locals.var_n_res_dn8 = assign57250_e89168_d_n8;
        locals.var_n_res_dn9 = assign57250_e89168_d_n9;
        locals.var_n_res_dn10 = assign57250_e89168_d_n10;
        locals.var_n_res_dn13 = assign57250_e89168_d_n13;
        locals.var_n_res_rv = 0.0;

        let (assign57260_e89183, assign57260_e89183_d_n0, assign57260_e89183_d_n2, assign57260_e89183_d_n4, assign57260_e89183_d_n5, assign57260_e89183_d_n6, assign57260_e89183_d_n7, assign57260_e89183_d_n8, assign57260_e89183_d_n9, assign57260_e89183_d_n10, assign57260_e89183_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57260_e89179: f64 = (locals.var_w_res * 1.6021918e-19);
        let assign57260_e89181: f64 = (assign57260_e89179 * locals.var_n_res);
        (assign57260_e89181, (((locals.var_w_res_dn0 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn0)), (((locals.var_w_res_dn2 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn2)), (((locals.var_w_res_dn4 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn4)), (((locals.var_w_res_dn5 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn5)), (((locals.var_w_res_dn6 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn6)), (((locals.var_w_res_dn7 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn7)), (((locals.var_w_res_dn8 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn8)), (((locals.var_w_res_dn9 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn9)), (((locals.var_w_res_dn10 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn10)), (((locals.var_w_res_dn13 * 1.6021918e-19) * locals.var_n_res) + (assign57260_e89179 * locals.var_n_res_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57260_e89183;
        locals.var_t1_dn0 = assign57260_e89183_d_n0;
        locals.var_t1_dn2 = assign57260_e89183_d_n2;
        locals.var_t1_dn4 = assign57260_e89183_d_n4;
        locals.var_t1_dn5 = assign57260_e89183_d_n5;
        locals.var_t1_dn6 = assign57260_e89183_d_n6;
        locals.var_t1_dn7 = assign57260_e89183_d_n7;
        locals.var_t1_dn8 = assign57260_e89183_d_n8;
        locals.var_t1_dn9 = assign57260_e89183_d_n9;
        locals.var_t1_dn10 = assign57260_e89183_d_n10;
        locals.var_t1_dn13 = assign57260_e89183_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57270_e89200, assign57270_e89200_d_n0, assign57270_e89200_d_n2, assign57270_e89200_d_n4, assign57270_e89200_d_n5, assign57270_e89200_d_n6, assign57270_e89200_d_n7, assign57270_e89200_d_n8, assign57270_e89200_d_n9, assign57270_e89200_d_n10, assign57270_e89200_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57270_e89194: f64 = (locals.var_weff / locals.var_leff);
        let assign57270_e89196: f64 = (assign57270_e89194).powf(locals.var_uc_depwlp);
        let assign57270_e89198: f64 = (assign57270_e89196 * p.p7);
        (assign57270_e89198, (if locals.var_uc_depwlp_dn0 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn0 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn2 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn2 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn4 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn4 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn5 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn5 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn6 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn6 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn7 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn7 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn8 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn8 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn9 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn9 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn10 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn10 * (assign57270_e89194).ln())) } * p.p7), (if locals.var_uc_depwlp_dn13 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57270_e89196 * (locals.var_uc_depwlp_dn13 * (assign57270_e89194).ln())) } * p.p7),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57270_e89200;
        locals.var_t2_dn0 = assign57270_e89200_d_n0;
        locals.var_t2_dn2 = assign57270_e89200_d_n2;
        locals.var_t2_dn4 = assign57270_e89200_d_n4;
        locals.var_t2_dn5 = assign57270_e89200_d_n5;
        locals.var_t2_dn6 = assign57270_e89200_d_n6;
        locals.var_t2_dn7 = assign57270_e89200_d_n7;
        locals.var_t2_dn8 = assign57270_e89200_d_n8;
        locals.var_t2_dn9 = assign57270_e89200_d_n9;
        locals.var_t2_dn10 = assign57270_e89200_d_n10;
        locals.var_t2_dn13 = assign57270_e89200_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57280_e89217, assign57280_e89217_d_n0, assign57280_e89217_d_n2, assign57280_e89217_d_n4, assign57280_e89217_d_n5, assign57280_e89217_d_n6, assign57280_e89217_d_n7, assign57280_e89217_d_n8, assign57280_e89217_d_n9, assign57280_e89217_d_n10, assign57280_e89217_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57280_e89211: f64 = (locals.var_weff_nf * locals.var_t1);
        let assign57280_e89213: f64 = (assign57280_e89211 * locals.var_mu_res);
        let assign57280_e89215: f64 = (assign57280_e89213 * locals.var_edri__blk1115);
        (assign57280_e89215, (((((locals.var_weff_nf * locals.var_t1_dn0) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn0)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn0)), (((((locals.var_weff_nf * locals.var_t1_dn2) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn2)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn2)), (((((locals.var_weff_nf * locals.var_t1_dn4) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn4)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn4)), (((((locals.var_weff_nf * locals.var_t1_dn5) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn5)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn5)), (((((locals.var_weff_nf * locals.var_t1_dn6) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn6)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn6)), (((((locals.var_weff_nf * locals.var_t1_dn7) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn7)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn7)), (((((locals.var_weff_nf * locals.var_t1_dn8) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn8)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn8)), (((((locals.var_weff_nf * locals.var_t1_dn9) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn9)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn9)), (((((locals.var_weff_nf * locals.var_t1_dn10) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn10)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn10)), (((((locals.var_weff_nf * locals.var_t1_dn13) * locals.var_mu_res) + (assign57280_e89211 * locals.var_mu_res_dn13)) * locals.var_edri__blk1115) + (assign57280_e89213 * locals.var_edri__blk1115_dn13)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn13,)
    }
};
        locals.var_ids_res = assign57280_e89217;
        locals.var_ids_res_dn0 = assign57280_e89217_d_n0;
        locals.var_ids_res_dn2 = assign57280_e89217_d_n2;
        locals.var_ids_res_dn4 = assign57280_e89217_d_n4;
        locals.var_ids_res_dn5 = assign57280_e89217_d_n5;
        locals.var_ids_res_dn6 = assign57280_e89217_d_n6;
        locals.var_ids_res_dn7 = assign57280_e89217_d_n7;
        locals.var_ids_res_dn8 = assign57280_e89217_d_n8;
        locals.var_ids_res_dn9 = assign57280_e89217_d_n9;
        locals.var_ids_res_dn10 = assign57280_e89217_d_n10;
        locals.var_ids_res_dn13 = assign57280_e89217_d_n13;
        locals.var_ids_res_rv = 0.0;

        let (assign57290_e89234, assign57290_e89234_d_n0, assign57290_e89234_d_n2, assign57290_e89234_d_n4, assign57290_e89234_d_n5, assign57290_e89234_d_n6, assign57290_e89234_d_n7, assign57290_e89234_d_n8, assign57290_e89234_d_n9, assign57290_e89234_d_n10, assign57290_e89234_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57290_e89228: f64 = (locals.var_t2 * locals.var_w_res_leak);
        let assign57290_e89230: f64 = (assign57290_e89228 * p.p363);
        let assign57290_e89232: f64 = (assign57290_e89230 * locals.var_vds_res0_sym);
        (assign57290_e89232, (((((locals.var_t2_dn0 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn0)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn0)), (((((locals.var_t2_dn2 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn2)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn2)), (((((locals.var_t2_dn4 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn4)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn4)), (((((locals.var_t2_dn5 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn5)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn5)), (((((locals.var_t2_dn6 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn6)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn6)), (((((locals.var_t2_dn7 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn7)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn7)), (((((locals.var_t2_dn8 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn8)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn8)), (((((locals.var_t2_dn9 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn9)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn9)), (((((locals.var_t2_dn10 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn10)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn10)), (((((locals.var_t2_dn13 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn13)) * p.p363) * locals.var_vds_res0_sym) + (assign57290_e89230 * locals.var_vds_res0_sym_dn13)),)
    } else {
        (locals.var_ires_leak, locals.var_ires_leak_dn0, locals.var_ires_leak_dn2, locals.var_ires_leak_dn4, locals.var_ires_leak_dn5, locals.var_ires_leak_dn6, locals.var_ires_leak_dn7, locals.var_ires_leak_dn8, locals.var_ires_leak_dn9, locals.var_ires_leak_dn10, locals.var_ires_leak_dn13,)
    }
};
        locals.var_ires_leak = assign57290_e89234;
        locals.var_ires_leak_dn0 = assign57290_e89234_d_n0;
        locals.var_ires_leak_dn2 = assign57290_e89234_d_n2;
        locals.var_ires_leak_dn4 = assign57290_e89234_d_n4;
        locals.var_ires_leak_dn5 = assign57290_e89234_d_n5;
        locals.var_ires_leak_dn6 = assign57290_e89234_d_n6;
        locals.var_ires_leak_dn7 = assign57290_e89234_d_n7;
        locals.var_ires_leak_dn8 = assign57290_e89234_d_n8;
        locals.var_ires_leak_dn9 = assign57290_e89234_d_n9;
        locals.var_ires_leak_dn10 = assign57290_e89234_d_n10;
        locals.var_ires_leak_dn13 = assign57290_e89234_d_n13;
        locals.var_ires_leak_rv = 0.0;

        let (assign57300_e89249, assign57300_e89249_d_n0, assign57300_e89249_d_n2, assign57300_e89249_d_n4, assign57300_e89249_d_n5, assign57300_e89249_d_n6, assign57300_e89249_d_n7, assign57300_e89249_d_n8, assign57300_e89249_d_n9, assign57300_e89249_d_n10, assign57300_e89249_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57300_e89245: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign57300_e89247: f64 = (assign57300_e89245 / locals.var_lch);
        (assign57300_e89247, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn13) * locals.var_lch) - (assign57300_e89245 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn13,)
    }
};
        locals.var_betawl = assign57300_e89249;
        locals.var_betawl_dn0 = assign57300_e89249_d_n0;
        locals.var_betawl_dn2 = assign57300_e89249_d_n2;
        locals.var_betawl_dn4 = assign57300_e89249_d_n4;
        locals.var_betawl_dn5 = assign57300_e89249_d_n5;
        locals.var_betawl_dn6 = assign57300_e89249_d_n6;
        locals.var_betawl_dn7 = assign57300_e89249_d_n7;
        locals.var_betawl_dn8 = assign57300_e89249_d_n8;
        locals.var_betawl_dn9 = assign57300_e89249_d_n9;
        locals.var_betawl_dn10 = assign57300_e89249_d_n10;
        locals.var_betawl_dn13 = assign57300_e89249_d_n13;
        locals.var_betawl_rv = 0.0;

        let (assign57310_e89264, assign57310_e89264_d_n0, assign57310_e89264_d_n2, assign57310_e89264_d_n4, assign57310_e89264_d_n5, assign57310_e89264_d_n6, assign57310_e89264_d_n7, assign57310_e89264_d_n8, assign57310_e89264_d_n9, assign57310_e89264_d_n10, assign57310_e89264_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57310_e89260: f64 = (locals.var_betawl * locals.var_idd);
        let assign57310_e89262: f64 = (assign57310_e89260 * locals.var_mu_acc);
        (assign57310_e89262, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn10)), ((((locals.var_betawl_dn13 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn13)) * locals.var_mu_acc) + (assign57310_e89260 * locals.var_mu_acc_dn13)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn13,)
    }
};
        locals.var_ids_acc = assign57310_e89264;
        locals.var_ids_acc_dn0 = assign57310_e89264_d_n0;
        locals.var_ids_acc_dn2 = assign57310_e89264_d_n2;
        locals.var_ids_acc_dn4 = assign57310_e89264_d_n4;
        locals.var_ids_acc_dn5 = assign57310_e89264_d_n5;
        locals.var_ids_acc_dn6 = assign57310_e89264_d_n6;
        locals.var_ids_acc_dn7 = assign57310_e89264_d_n7;
        locals.var_ids_acc_dn8 = assign57310_e89264_d_n8;
        locals.var_ids_acc_dn9 = assign57310_e89264_d_n9;
        locals.var_ids_acc_dn10 = assign57310_e89264_d_n10;
        locals.var_ids_acc_dn13 = assign57310_e89264_d_n13;
        locals.var_ids_acc_rv = 0.0;

        let (assign57320_e89285, assign57320_e89285_d_n0, assign57320_e89285_d_n2, assign57320_e89285_d_n4, assign57320_e89285_d_n5, assign57320_e89285_d_n6, assign57320_e89285_d_n7, assign57320_e89285_d_n8, assign57320_e89285_d_n9, assign57320_e89285_d_n10, assign57320_e89285_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57320_e89275: f64 = locals.var_ids_acc;
        let assign57320_e89278: f64 = locals.var_ids_res;
        let assign57320_e89279: f64 = (assign57320_e89275 + assign57320_e89278);
        let assign57320_e89282: f64 = locals.var_ires_leak;
        let assign57320_e89283: f64 = (assign57320_e89279 + assign57320_e89282);
        (assign57320_e89283, ((locals.var_ids_acc_dn0 + locals.var_ids_res_dn0) + locals.var_ires_leak_dn0), ((locals.var_ids_acc_dn2 + locals.var_ids_res_dn2) + locals.var_ires_leak_dn2), ((locals.var_ids_acc_dn4 + locals.var_ids_res_dn4) + locals.var_ires_leak_dn4), ((locals.var_ids_acc_dn5 + locals.var_ids_res_dn5) + locals.var_ires_leak_dn5), ((locals.var_ids_acc_dn6 + locals.var_ids_res_dn6) + locals.var_ires_leak_dn6), ((locals.var_ids_acc_dn7 + locals.var_ids_res_dn7) + locals.var_ires_leak_dn7), ((locals.var_ids_acc_dn8 + locals.var_ids_res_dn8) + locals.var_ires_leak_dn8), ((locals.var_ids_acc_dn9 + locals.var_ids_res_dn9) + locals.var_ires_leak_dn9), ((locals.var_ids_acc_dn10 + locals.var_ids_res_dn10) + locals.var_ires_leak_dn10), ((locals.var_ids_acc_dn13 + locals.var_ids_res_dn13) + locals.var_ires_leak_dn13),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign57320_e89285;
        locals.var_ids0_dn0 = assign57320_e89285_d_n0;
        locals.var_ids0_dn2 = assign57320_e89285_d_n2;
        locals.var_ids0_dn4 = assign57320_e89285_d_n4;
        locals.var_ids0_dn5 = assign57320_e89285_d_n5;
        locals.var_ids0_dn6 = assign57320_e89285_d_n6;
        locals.var_ids0_dn7 = assign57320_e89285_d_n7;
        locals.var_ids0_dn8 = assign57320_e89285_d_n8;
        locals.var_ids0_dn9 = assign57320_e89285_d_n9;
        locals.var_ids0_dn10 = assign57320_e89285_d_n10;
        locals.var_ids0_dn13 = assign57320_e89285_d_n13;
        locals.var_ids0_rv = 0.0;

        let (assign57330_e89296, assign57330_e89296_d_n0, assign57330_e89296_d_n2, assign57330_e89296_d_n4, assign57330_e89296_d_n5, assign57330_e89296_d_n6, assign57330_e89296_d_n7, assign57330_e89296_d_n8, assign57330_e89296_d_n9, assign57330_e89296_d_n10, assign57330_e89296_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign57330_e89296;
        locals.var_vds_dn0 = assign57330_e89296_d_n0;
        locals.var_vds_dn2 = assign57330_e89296_d_n2;
        locals.var_vds_dn4 = assign57330_e89296_d_n4;
        locals.var_vds_dn5 = assign57330_e89296_d_n5;
        locals.var_vds_dn6 = assign57330_e89296_d_n6;
        locals.var_vds_dn7 = assign57330_e89296_d_n7;
        locals.var_vds_dn8 = assign57330_e89296_d_n8;
        locals.var_vds_dn9 = assign57330_e89296_d_n9;
        locals.var_vds_dn10 = assign57330_e89296_d_n10;
        locals.var_vds_dn13 = assign57330_e89296_d_n13;
        locals.var_vds_rv = 0.0;

        let assign57340_e89299: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1423 = assign57340_e89299;
        locals.var_guard1423_rv = 0.0;

        let (assign57350_e89316, assign57350_e89316_d_n0, assign57350_e89316_d_n2, assign57350_e89316_d_n4, assign57350_e89316_d_n5, assign57350_e89316_d_n6, assign57350_e89316_d_n7, assign57350_e89316_d_n8, assign57350_e89316_d_n9, assign57350_e89316_d_n10, assign57350_e89316_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57350_e89313: f64 = (locals.var_vds - locals.var_pds);
        let assign57350_e89314: f64 = (0.5 * assign57350_e89313);
        (assign57350_e89314, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57350_e89316;
        locals.var_t1_dn0 = assign57350_e89316_d_n0;
        locals.var_t1_dn2 = assign57350_e89316_d_n2;
        locals.var_t1_dn4 = assign57350_e89316_d_n4;
        locals.var_t1_dn5 = assign57350_e89316_d_n5;
        locals.var_t1_dn6 = assign57350_e89316_d_n6;
        locals.var_t1_dn7 = assign57350_e89316_d_n7;
        locals.var_t1_dn8 = assign57350_e89316_d_n8;
        locals.var_t1_dn9 = assign57350_e89316_d_n9;
        locals.var_t1_dn10 = assign57350_e89316_d_n10;
        locals.var_t1_dn13 = assign57350_e89316_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57360_e89333, assign57360_e89333_d_n0, assign57360_e89333_d_n2, assign57360_e89333_d_n4, assign57360_e89333_d_n5, assign57360_e89333_d_n6, assign57360_e89333_d_n7, assign57360_e89333_d_n8, assign57360_e89333_d_n9, assign57360_e89333_d_n10, assign57360_e89333_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57360_e89329: f64 = (2.0 * locals.var_t1);
        let assign57360_e89331: f64 = (assign57360_e89329 / 0.01);
        (assign57360_e89331, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn13) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign57360_e89333;
        locals.var_tmf1_dn0 = assign57360_e89333_d_n0;
        locals.var_tmf1_dn2 = assign57360_e89333_d_n2;
        locals.var_tmf1_dn4 = assign57360_e89333_d_n4;
        locals.var_tmf1_dn5 = assign57360_e89333_d_n5;
        locals.var_tmf1_dn6 = assign57360_e89333_d_n6;
        locals.var_tmf1_dn7 = assign57360_e89333_d_n7;
        locals.var_tmf1_dn8 = assign57360_e89333_d_n8;
        locals.var_tmf1_dn9 = assign57360_e89333_d_n9;
        locals.var_tmf1_dn10 = assign57360_e89333_d_n10;
        locals.var_tmf1_dn13 = assign57360_e89333_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign57370_e89382, assign57370_e89382_d_n0, assign57370_e89382_d_n2, assign57370_e89382_d_n4, assign57370_e89382_d_n5, assign57370_e89382_d_n6, assign57370_e89382_d_n7, assign57370_e89382_d_n8, assign57370_e89382_d_n9, assign57370_e89382_d_n10, assign57370_e89382_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57370_e89348: f64 = (1.0 / 2.0);
        let assign57370_e89352: f64 = (1.0 / 6.0);
        let assign57370_e89356: f64 = (1.0 / 24.0);
        let assign57370_e89360: f64 = (1.0 / 120.0);
        let assign57370_e89364: f64 = (1.0 / 720.0);
        let assign57370_e89368: f64 = (1.0 / 5040.0);
        let assign57370_e89369: f64 = (locals.var_tmf1 * assign57370_e89368);
        let assign57370_e89370: f64 = (assign57370_e89364 + assign57370_e89369);
        let assign57370_e89371: f64 = (locals.var_tmf1 * assign57370_e89370);
        let assign57370_e89372: f64 = (assign57370_e89360 + assign57370_e89371);
        let assign57370_e89373: f64 = (locals.var_tmf1 * assign57370_e89372);
        let assign57370_e89374: f64 = (assign57370_e89356 + assign57370_e89373);
        let assign57370_e89375: f64 = (locals.var_tmf1 * assign57370_e89374);
        let assign57370_e89376: f64 = (assign57370_e89352 + assign57370_e89375);
        let assign57370_e89377: f64 = (locals.var_tmf1 * assign57370_e89376);
        let assign57370_e89378: f64 = (assign57370_e89348 + assign57370_e89377);
        let assign57370_e89379: f64 = (locals.var_tmf1 * assign57370_e89378);
        let assign57370_e89380: f64 = (1.0 + assign57370_e89379);
        (assign57370_e89380, ((locals.var_tmf1_dn0 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn2 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn4 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn5 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn6 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn7 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn8 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn9 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn10 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57370_e89368))))))))))), ((locals.var_tmf1_dn13 * assign57370_e89378) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57370_e89376) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57370_e89374) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57370_e89372) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57370_e89370) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign57370_e89368))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign57370_e89382;
        locals.var_tmf2_dn0 = assign57370_e89382_d_n0;
        locals.var_tmf2_dn2 = assign57370_e89382_d_n2;
        locals.var_tmf2_dn4 = assign57370_e89382_d_n4;
        locals.var_tmf2_dn5 = assign57370_e89382_d_n5;
        locals.var_tmf2_dn6 = assign57370_e89382_d_n6;
        locals.var_tmf2_dn7 = assign57370_e89382_d_n7;
        locals.var_tmf2_dn8 = assign57370_e89382_d_n8;
        locals.var_tmf2_dn9 = assign57370_e89382_d_n9;
        locals.var_tmf2_dn10 = assign57370_e89382_d_n10;
        locals.var_tmf2_dn13 = assign57370_e89382_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign57380_e89427, assign57380_e89427_d_n0, assign57380_e89427_d_n2, assign57380_e89427_d_n4, assign57380_e89427_d_n5, assign57380_e89427_d_n6, assign57380_e89427_d_n7, assign57380_e89427_d_n8, assign57380_e89427_d_n9, assign57380_e89427_d_n10, assign57380_e89427_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57380_e89395: f64 = (1.0 / 2.0);
        let assign57380_e89399: f64 = (1.0 / 3.0);
        let assign57380_e89403: f64 = (1.0 / 8.0);
        let assign57380_e89407: f64 = (1.0 / 30.0);
        let assign57380_e89411: f64 = (1.0 / 144.0);
        let assign57380_e89415: f64 = (1.0 / 840.0);
        let assign57380_e89416: f64 = (locals.var_tmf1 * assign57380_e89415);
        let assign57380_e89417: f64 = (assign57380_e89411 + assign57380_e89416);
        let assign57380_e89418: f64 = (locals.var_tmf1 * assign57380_e89417);
        let assign57380_e89419: f64 = (assign57380_e89407 + assign57380_e89418);
        let assign57380_e89420: f64 = (locals.var_tmf1 * assign57380_e89419);
        let assign57380_e89421: f64 = (assign57380_e89403 + assign57380_e89420);
        let assign57380_e89422: f64 = (locals.var_tmf1 * assign57380_e89421);
        let assign57380_e89423: f64 = (assign57380_e89399 + assign57380_e89422);
        let assign57380_e89424: f64 = (locals.var_tmf1 * assign57380_e89423);
        let assign57380_e89425: f64 = (assign57380_e89395 + assign57380_e89424);
        (assign57380_e89425, ((locals.var_tmf1_dn0 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57380_e89415))))))))), ((locals.var_tmf1_dn2 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57380_e89415))))))))), ((locals.var_tmf1_dn4 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57380_e89415))))))))), ((locals.var_tmf1_dn5 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57380_e89415))))))))), ((locals.var_tmf1_dn6 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57380_e89415))))))))), ((locals.var_tmf1_dn7 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57380_e89415))))))))), ((locals.var_tmf1_dn8 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57380_e89415))))))))), ((locals.var_tmf1_dn9 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57380_e89415))))))))), ((locals.var_tmf1_dn10 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57380_e89415))))))))), ((locals.var_tmf1_dn13 * assign57380_e89423) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57380_e89421) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57380_e89419) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign57380_e89417) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign57380_e89415))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign57380_e89427;
        locals.var_tmf3_dn0 = assign57380_e89427_d_n0;
        locals.var_tmf3_dn2 = assign57380_e89427_d_n2;
        locals.var_tmf3_dn4 = assign57380_e89427_d_n4;
        locals.var_tmf3_dn5 = assign57380_e89427_d_n5;
        locals.var_tmf3_dn6 = assign57380_e89427_d_n6;
        locals.var_tmf3_dn7 = assign57380_e89427_d_n7;
        locals.var_tmf3_dn8 = assign57380_e89427_d_n8;
        locals.var_tmf3_dn9 = assign57380_e89427_d_n9;
        locals.var_tmf3_dn10 = assign57380_e89427_d_n10;
        locals.var_tmf3_dn13 = assign57380_e89427_d_n13;
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_202(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57390_e89442, assign57390_e89442_d_n0, assign57390_e89442_d_n2, assign57390_e89442_d_n4, assign57390_e89442_d_n5, assign57390_e89442_d_n6, assign57390_e89442_d_n7, assign57390_e89442_d_n8, assign57390_e89442_d_n9, assign57390_e89442_d_n10, assign57390_e89442_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57390_e89440: f64 = (0.01 / locals.var_tmf2);
        (assign57390_e89440, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign57390_e89442;
        locals.var_t6_dn0 = assign57390_e89442_d_n0;
        locals.var_t6_dn2 = assign57390_e89442_d_n2;
        locals.var_t6_dn4 = assign57390_e89442_d_n4;
        locals.var_t6_dn5 = assign57390_e89442_d_n5;
        locals.var_t6_dn6 = assign57390_e89442_d_n6;
        locals.var_t6_dn7 = assign57390_e89442_d_n7;
        locals.var_t6_dn8 = assign57390_e89442_d_n8;
        locals.var_t6_dn9 = assign57390_e89442_d_n9;
        locals.var_t6_dn10 = assign57390_e89442_d_n10;
        locals.var_t6_dn13 = assign57390_e89442_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign57400_e89462, assign57400_e89462_d_n0, assign57400_e89462_d_n2, assign57400_e89462_d_n4, assign57400_e89462_d_n5, assign57400_e89462_d_n6, assign57400_e89462_d_n7, assign57400_e89462_d_n8, assign57400_e89462_d_n9, assign57400_e89462_d_n10, assign57400_e89462_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57400_e89454: f64 = (-2.0);
        let assign57400_e89456: f64 = (assign57400_e89454 * locals.var_tmf3);
        let assign57400_e89459: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign57400_e89460: f64 = (assign57400_e89456 / assign57400_e89459);
        (assign57400_e89460, ((((assign57400_e89454 * locals.var_tmf3_dn0) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn2) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn4) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn5) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn6) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn7) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn8) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn9) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn10) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign57400_e89459 * assign57400_e89459)), ((((assign57400_e89454 * locals.var_tmf3_dn13) * assign57400_e89459) - (assign57400_e89456 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign57400_e89459 * assign57400_e89459)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57400_e89462;
        locals.var_t2_dn0 = assign57400_e89462_d_n0;
        locals.var_t2_dn2 = assign57400_e89462_d_n2;
        locals.var_t2_dn4 = assign57400_e89462_d_n4;
        locals.var_t2_dn5 = assign57400_e89462_d_n5;
        locals.var_t2_dn6 = assign57400_e89462_d_n6;
        locals.var_t2_dn7 = assign57400_e89462_d_n7;
        locals.var_t2_dn8 = assign57400_e89462_d_n8;
        locals.var_t2_dn9 = assign57400_e89462_d_n9;
        locals.var_t2_dn10 = assign57400_e89462_d_n10;
        locals.var_t2_dn13 = assign57400_e89462_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57410_e89479, assign57410_e89479_d_n0, assign57410_e89479_d_n2, assign57410_e89479_d_n4, assign57410_e89479_d_n5, assign57410_e89479_d_n6, assign57410_e89479_d_n7, assign57410_e89479_d_n8, assign57410_e89479_d_n9, assign57410_e89479_d_n10, assign57410_e89479_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57410_e89476: f64 = (locals.var_phi_s0_dep__blk1089 + locals.var_t6);
        let assign57410_e89477: f64 = (1.1 - assign57410_e89476);
        (assign57410_e89477, (-(locals.var_phi_s0_dep__blk1089_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep__blk1089_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep__blk1089_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep__blk1089_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep__blk1089_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep__blk1089_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep__blk1089_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep__blk1089_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep__blk1089_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep__blk1089_dn13 + locals.var_t6_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57410_e89479;
        locals.var_t1_dn0 = assign57410_e89479_d_n0;
        locals.var_t1_dn2 = assign57410_e89479_d_n2;
        locals.var_t1_dn4 = assign57410_e89479_d_n4;
        locals.var_t1_dn5 = assign57410_e89479_d_n5;
        locals.var_t1_dn6 = assign57410_e89479_d_n6;
        locals.var_t1_dn7 = assign57410_e89479_d_n7;
        locals.var_t1_dn8 = assign57410_e89479_d_n8;
        locals.var_t1_dn9 = assign57410_e89479_d_n9;
        locals.var_t1_dn10 = assign57410_e89479_d_n10;
        locals.var_t1_dn13 = assign57410_e89479_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57420_e89501, assign57420_e89501_d_n0, assign57420_e89501_d_n2, assign57420_e89501_d_n4, assign57420_e89501_d_n5, assign57420_e89501_d_n6, assign57420_e89501_d_n7, assign57420_e89501_d_n8, assign57420_e89501_d_n9, assign57420_e89501_d_n10, assign57420_e89501_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57420_e89492: f64 = (locals.var_t1 * locals.var_t1);
        let assign57420_e89495: f64 = (4.0 * 0.05);
        let assign57420_e89497: f64 = (assign57420_e89495 * 0.05);
        let assign57420_e89498: f64 = (assign57420_e89492 + assign57420_e89497);
        let assign57420_e89499: f64 = (assign57420_e89498).sqrt();
        (assign57420_e89499, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign57420_e89499)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign57420_e89499)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign57420_e89501;
        locals.var_tmf2_dn0 = assign57420_e89501_d_n0;
        locals.var_tmf2_dn2 = assign57420_e89501_d_n2;
        locals.var_tmf2_dn4 = assign57420_e89501_d_n4;
        locals.var_tmf2_dn5 = assign57420_e89501_d_n5;
        locals.var_tmf2_dn6 = assign57420_e89501_d_n6;
        locals.var_tmf2_dn7 = assign57420_e89501_d_n7;
        locals.var_tmf2_dn8 = assign57420_e89501_d_n8;
        locals.var_tmf2_dn9 = assign57420_e89501_d_n9;
        locals.var_tmf2_dn10 = assign57420_e89501_d_n10;
        locals.var_tmf2_dn13 = assign57420_e89501_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign57430_e89520, assign57430_e89520_d_n0, assign57430_e89520_d_n2, assign57430_e89520_d_n4, assign57430_e89520_d_n5, assign57430_e89520_d_n6, assign57430_e89520_d_n7, assign57430_e89520_d_n8, assign57430_e89520_d_n9, assign57430_e89520_d_n10, assign57430_e89520_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57430_e89516: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign57430_e89517: f64 = (1.0 + assign57430_e89516);
        let assign57430_e89518: f64 = (0.5 * assign57430_e89517);
        (assign57430_e89518, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57430_e89520;
        locals.var_t0_dn0 = assign57430_e89520_d_n0;
        locals.var_t0_dn2 = assign57430_e89520_d_n2;
        locals.var_t0_dn4 = assign57430_e89520_d_n4;
        locals.var_t0_dn5 = assign57430_e89520_d_n5;
        locals.var_t0_dn6 = assign57430_e89520_d_n6;
        locals.var_t0_dn7 = assign57430_e89520_d_n7;
        locals.var_t0_dn8 = assign57430_e89520_d_n8;
        locals.var_t0_dn9 = assign57430_e89520_d_n9;
        locals.var_t0_dn10 = assign57430_e89520_d_n10;
        locals.var_t0_dn13 = assign57430_e89520_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57440_e89537, assign57440_e89537_d_n0, assign57440_e89537_d_n2, assign57440_e89537_d_n4, assign57440_e89537_d_n5, assign57440_e89537_d_n6, assign57440_e89537_d_n7, assign57440_e89537_d_n8, assign57440_e89537_d_n9, assign57440_e89537_d_n10, assign57440_e89537_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57440_e89534: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign57440_e89535: f64 = (0.5 * assign57440_e89534);
        (assign57440_e89535, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57440_e89537;
        locals.var_t2_dn0 = assign57440_e89537_d_n0;
        locals.var_t2_dn2 = assign57440_e89537_d_n2;
        locals.var_t2_dn4 = assign57440_e89537_d_n4;
        locals.var_t2_dn5 = assign57440_e89537_d_n5;
        locals.var_t2_dn6 = assign57440_e89537_d_n6;
        locals.var_t2_dn7 = assign57440_e89537_d_n7;
        locals.var_t2_dn8 = assign57440_e89537_d_n8;
        locals.var_t2_dn9 = assign57440_e89537_d_n9;
        locals.var_t2_dn10 = assign57440_e89537_d_n10;
        locals.var_t2_dn13 = assign57440_e89537_d_n13;
        locals.var_t2_rv = 0.0;

        let assign57450_e89540: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1424 = assign57450_e89540;
        locals.var_guard1424_rv = 0.0;

        let (assign57460_e89555, assign57460_e89555_d_n0, assign57460_e89555_d_n2, assign57460_e89555_d_n4, assign57460_e89555_d_n5, assign57460_e89555_d_n6, assign57460_e89555_d_n7, assign57460_e89555_d_n8, assign57460_e89555_d_n9, assign57460_e89555_d_n10, assign57460_e89555_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) && (locals.var_guard1424 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57460_e89555;
        locals.var_t2_dn0 = assign57460_e89555_d_n0;
        locals.var_t2_dn2 = assign57460_e89555_d_n2;
        locals.var_t2_dn4 = assign57460_e89555_d_n4;
        locals.var_t2_dn5 = assign57460_e89555_d_n5;
        locals.var_t2_dn6 = assign57460_e89555_d_n6;
        locals.var_t2_dn7 = assign57460_e89555_d_n7;
        locals.var_t2_dn8 = assign57460_e89555_d_n8;
        locals.var_t2_dn9 = assign57460_e89555_d_n9;
        locals.var_t2_dn10 = assign57460_e89555_d_n10;
        locals.var_t2_dn13 = assign57460_e89555_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57470_e89570, assign57470_e89570_d_n0, assign57470_e89570_d_n2, assign57470_e89570_d_n4, assign57470_e89570_d_n5, assign57470_e89570_d_n6, assign57470_e89570_d_n7, assign57470_e89570_d_n8, assign57470_e89570_d_n9, assign57470_e89570_d_n10, assign57470_e89570_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) && (locals.var_guard1424 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57470_e89570;
        locals.var_t0_dn0 = assign57470_e89570_d_n0;
        locals.var_t0_dn2 = assign57470_e89570_d_n2;
        locals.var_t0_dn4 = assign57470_e89570_d_n4;
        locals.var_t0_dn5 = assign57470_e89570_d_n5;
        locals.var_t0_dn6 = assign57470_e89570_d_n6;
        locals.var_t0_dn7 = assign57470_e89570_d_n7;
        locals.var_t0_dn8 = assign57470_e89570_d_n8;
        locals.var_t0_dn9 = assign57470_e89570_d_n9;
        locals.var_t0_dn10 = assign57470_e89570_d_n10;
        locals.var_t0_dn13 = assign57470_e89570_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57480_e89585, assign57480_e89585_d_n0, assign57480_e89585_d_n2, assign57480_e89585_d_n4, assign57480_e89585_d_n5, assign57480_e89585_d_n6, assign57480_e89585_d_n7, assign57480_e89585_d_n8, assign57480_e89585_d_n9, assign57480_e89585_d_n10, assign57480_e89585_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57480_e89583: f64 = (locals.var_t2 + 1e-25);
        (assign57480_e89583, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57480_e89585;
        locals.var_t2_dn0 = assign57480_e89585_d_n0;
        locals.var_t2_dn2 = assign57480_e89585_d_n2;
        locals.var_t2_dn4 = assign57480_e89585_d_n4;
        locals.var_t2_dn5 = assign57480_e89585_d_n5;
        locals.var_t2_dn6 = assign57480_e89585_d_n6;
        locals.var_t2_dn7 = assign57480_e89585_d_n7;
        locals.var_t2_dn8 = assign57480_e89585_d_n8;
        locals.var_t2_dn9 = assign57480_e89585_d_n9;
        locals.var_t2_dn10 = assign57480_e89585_d_n10;
        locals.var_t2_dn13 = assign57480_e89585_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57490_e89600, assign57490_e89600_d_n0, assign57490_e89600_d_n2, assign57490_e89600_d_n4, assign57490_e89600_d_n5, assign57490_e89600_d_n6, assign57490_e89600_d_n7, assign57490_e89600_d_n8, assign57490_e89600_d_n9, assign57490_e89600_d_n10, assign57490_e89600_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57490_e89598: f64 = (locals.var_beta * locals.var_ptl0);
        (assign57490_e89598, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn13 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57490_e89600;
        locals.var_t0_dn0 = assign57490_e89600_d_n0;
        locals.var_t0_dn2 = assign57490_e89600_d_n2;
        locals.var_t0_dn4 = assign57490_e89600_d_n4;
        locals.var_t0_dn5 = assign57490_e89600_d_n5;
        locals.var_t0_dn6 = assign57490_e89600_d_n6;
        locals.var_t0_dn7 = assign57490_e89600_d_n7;
        locals.var_t0_dn8 = assign57490_e89600_d_n8;
        locals.var_t0_dn9 = assign57490_e89600_d_n9;
        locals.var_t0_dn10 = assign57490_e89600_d_n10;
        locals.var_t0_dn13 = assign57490_e89600_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57500_e89615, assign57500_e89615_d_n0, assign57500_e89615_d_n2, assign57500_e89615_d_n4, assign57500_e89615_d_n5, assign57500_e89615_d_n6, assign57500_e89615_d_n7, assign57500_e89615_d_n8, assign57500_e89615_d_n9, assign57500_e89615_d_n10, assign57500_e89615_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57500_e89613: f64 = (locals.var_cox * locals.var_t0);
        (assign57500_e89613, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn13 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign57500_e89615;
        locals.var_t3_dn0 = assign57500_e89615_d_n0;
        locals.var_t3_dn2 = assign57500_e89615_d_n2;
        locals.var_t3_dn4 = assign57500_e89615_d_n4;
        locals.var_t3_dn5 = assign57500_e89615_d_n5;
        locals.var_t3_dn6 = assign57500_e89615_d_n6;
        locals.var_t3_dn7 = assign57500_e89615_d_n7;
        locals.var_t3_dn8 = assign57500_e89615_d_n8;
        locals.var_t3_dn9 = assign57500_e89615_d_n9;
        locals.var_t3_dn10 = assign57500_e89615_d_n10;
        locals.var_t3_dn13 = assign57500_e89615_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign57510_e89630, assign57510_e89630_d_n0, assign57510_e89630_d_n2, assign57510_e89630_d_n4, assign57510_e89630_d_n5, assign57510_e89630_d_n6, assign57510_e89630_d_n7, assign57510_e89630_d_n8, assign57510_e89630_d_n9, assign57510_e89630_d_n10, assign57510_e89630_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57510_e89628: f64 = (locals.var_t2).powf(p.p284);
        (assign57510_e89628, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn13)) } } else { (assign57510_e89628 * (p.p284 * (locals.var_t2_dn13 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57510_e89630;
        locals.var_t0_dn0 = assign57510_e89630_d_n0;
        locals.var_t0_dn2 = assign57510_e89630_d_n2;
        locals.var_t0_dn4 = assign57510_e89630_d_n4;
        locals.var_t0_dn5 = assign57510_e89630_d_n5;
        locals.var_t0_dn6 = assign57510_e89630_d_n6;
        locals.var_t0_dn7 = assign57510_e89630_d_n7;
        locals.var_t0_dn8 = assign57510_e89630_d_n8;
        locals.var_t0_dn9 = assign57510_e89630_d_n9;
        locals.var_t0_dn10 = assign57510_e89630_d_n10;
        locals.var_t0_dn13 = assign57510_e89630_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57520_e89645, assign57520_e89645_d_n0, assign57520_e89645_d_n2, assign57520_e89645_d_n4, assign57520_e89645_d_n5, assign57520_e89645_d_n6, assign57520_e89645_d_n7, assign57520_e89645_d_n8, assign57520_e89645_d_n9, assign57520_e89645_d_n10, assign57520_e89645_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57520_e89643: f64 = (locals.var_t3 * locals.var_t0);
        (assign57520_e89643, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn13 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign57520_e89645;
        locals.var_t9_dn0 = assign57520_e89645_d_n0;
        locals.var_t9_dn2 = assign57520_e89645_d_n2;
        locals.var_t9_dn4 = assign57520_e89645_d_n4;
        locals.var_t9_dn5 = assign57520_e89645_d_n5;
        locals.var_t9_dn6 = assign57520_e89645_d_n6;
        locals.var_t9_dn7 = assign57520_e89645_d_n7;
        locals.var_t9_dn8 = assign57520_e89645_d_n8;
        locals.var_t9_dn9 = assign57520_e89645_d_n9;
        locals.var_t9_dn10 = assign57520_e89645_d_n10;
        locals.var_t9_dn13 = assign57520_e89645_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign57530_e89662, assign57530_e89662_d_n0, assign57530_e89662_d_n2, assign57530_e89662_d_n4, assign57530_e89662_d_n5, assign57530_e89662_d_n6, assign57530_e89662_d_n7, assign57530_e89662_d_n8, assign57530_e89662_d_n9, assign57530_e89662_d_n10, assign57530_e89662_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57530_e89659: f64 = (locals.var_vdsz__blk439 * p.p285);
        let assign57530_e89660: f64 = (1.0 + assign57530_e89659);
        (assign57530_e89660, (locals.var_vdsz__blk439_dn0 * p.p285), (locals.var_vdsz__blk439_dn2 * p.p285), (locals.var_vdsz__blk439_dn4 * p.p285), (locals.var_vdsz__blk439_dn5 * p.p285), (locals.var_vdsz__blk439_dn6 * p.p285), (locals.var_vdsz__blk439_dn7 * p.p285), (locals.var_vdsz__blk439_dn8 * p.p285), (locals.var_vdsz__blk439_dn9 * p.p285), (locals.var_vdsz__blk439_dn10 * p.p285), (locals.var_vdsz__blk439_dn13 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign57530_e89662;
        locals.var_t4_dn0 = assign57530_e89662_d_n0;
        locals.var_t4_dn2 = assign57530_e89662_d_n2;
        locals.var_t4_dn4 = assign57530_e89662_d_n4;
        locals.var_t4_dn5 = assign57530_e89662_d_n5;
        locals.var_t4_dn6 = assign57530_e89662_d_n6;
        locals.var_t4_dn7 = assign57530_e89662_d_n7;
        locals.var_t4_dn8 = assign57530_e89662_d_n8;
        locals.var_t4_dn9 = assign57530_e89662_d_n9;
        locals.var_t4_dn10 = assign57530_e89662_d_n10;
        locals.var_t4_dn13 = assign57530_e89662_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign57540_e89675, assign57540_e89675_d_n0, assign57540_e89675_d_n2, assign57540_e89675_d_n4, assign57540_e89675_d_n5, assign57540_e89675_d_n6, assign57540_e89675_d_n7, assign57540_e89675_d_n8, assign57540_e89675_d_n9, assign57540_e89675_d_n10, assign57540_e89675_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign57540_e89675;
        locals.var_t0_dn0 = assign57540_e89675_d_n0;
        locals.var_t0_dn2 = assign57540_e89675_d_n2;
        locals.var_t0_dn4 = assign57540_e89675_d_n4;
        locals.var_t0_dn5 = assign57540_e89675_d_n5;
        locals.var_t0_dn6 = assign57540_e89675_d_n6;
        locals.var_t0_dn7 = assign57540_e89675_d_n7;
        locals.var_t0_dn8 = assign57540_e89675_d_n8;
        locals.var_t0_dn9 = assign57540_e89675_d_n9;
        locals.var_t0_dn10 = assign57540_e89675_d_n10;
        locals.var_t0_dn13 = assign57540_e89675_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign57550_e89692, assign57550_e89692_d_n0, assign57550_e89692_d_n2, assign57550_e89692_d_n4, assign57550_e89692_d_n5, assign57550_e89692_d_n6, assign57550_e89692_d_n7, assign57550_e89692_d_n8, assign57550_e89692_d_n9, assign57550_e89692_d_n10, assign57550_e89692_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57550_e89688: f64 = (locals.var_phi_s0_dep__blk1089 + locals.var_t6);
        let assign57550_e89690: f64 = (assign57550_e89688 - locals.var_vbsz__blk438);
        (assign57550_e89690, ((locals.var_phi_s0_dep__blk1089_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_phi_s0_dep__blk1089_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_phi_s0_dep__blk1089_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_phi_s0_dep__blk1089_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_phi_s0_dep__blk1089_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_phi_s0_dep__blk1089_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_phi_s0_dep__blk1089_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_phi_s0_dep__blk1089_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_phi_s0_dep__blk1089_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_phi_s0_dep__blk1089_dn13 + locals.var_t6_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign57550_e89692;
        locals.var_t5_dn0 = assign57550_e89692_d_n0;
        locals.var_t5_dn2 = assign57550_e89692_d_n2;
        locals.var_t5_dn4 = assign57550_e89692_d_n4;
        locals.var_t5_dn5 = assign57550_e89692_d_n5;
        locals.var_t5_dn6 = assign57550_e89692_d_n6;
        locals.var_t5_dn7 = assign57550_e89692_d_n7;
        locals.var_t5_dn8 = assign57550_e89692_d_n8;
        locals.var_t5_dn9 = assign57550_e89692_d_n9;
        locals.var_t5_dn10 = assign57550_e89692_d_n10;
        locals.var_t5_dn13 = assign57550_e89692_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign57560_e89711, assign57560_e89711_d_n0, assign57560_e89711_d_n2, assign57560_e89711_d_n4, assign57560_e89711_d_n5, assign57560_e89711_d_n6, assign57560_e89711_d_n7, assign57560_e89711_d_n8, assign57560_e89711_d_n9, assign57560_e89711_d_n10, assign57560_e89711_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57560_e89706: f64 = (locals.var_vdsz__blk439 * locals.var_t0);
        let assign57560_e89708: f64 = (assign57560_e89706 * locals.var_t5);
        let assign57560_e89709: f64 = (locals.var_t4 + assign57560_e89708);
        (assign57560_e89709, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk439_dn0 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn0)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk439_dn2 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn2)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk439_dn4 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn4)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk439_dn5 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn5)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk439_dn6 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn6)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk439_dn7 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn7)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk439_dn8 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn8)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk439_dn9 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn9)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk439_dn10 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn10)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn10))), (locals.var_t4_dn13 + ((((locals.var_vdsz__blk439_dn13 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn13)) * locals.var_t5) + (assign57560_e89706 * locals.var_t5_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign57560_e89711;
        locals.var_t4_dn0 = assign57560_e89711_d_n0;
        locals.var_t4_dn2 = assign57560_e89711_d_n2;
        locals.var_t4_dn4 = assign57560_e89711_d_n4;
        locals.var_t4_dn5 = assign57560_e89711_d_n5;
        locals.var_t4_dn6 = assign57560_e89711_d_n6;
        locals.var_t4_dn7 = assign57560_e89711_d_n7;
        locals.var_t4_dn8 = assign57560_e89711_d_n8;
        locals.var_t4_dn9 = assign57560_e89711_d_n9;
        locals.var_t4_dn10 = assign57560_e89711_d_n10;
        locals.var_t4_dn13 = assign57560_e89711_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign57570_e89726, assign57570_e89726_d_n0, assign57570_e89726_d_n2, assign57570_e89726_d_n4, assign57570_e89726_d_n5, assign57570_e89726_d_n6, assign57570_e89726_d_n7, assign57570_e89726_d_n8, assign57570_e89726_d_n9, assign57570_e89726_d_n10, assign57570_e89726_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        let assign57570_e89724: f64 = (locals.var_t9 * locals.var_t4);
        (assign57570_e89724, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn13 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign57570_e89726;
        locals.var_t6_dn0 = assign57570_e89726_d_n0;
        locals.var_t6_dn2 = assign57570_e89726_d_n2;
        locals.var_t6_dn4 = assign57570_e89726_d_n4;
        locals.var_t6_dn5 = assign57570_e89726_d_n5;
        locals.var_t6_dn6 = assign57570_e89726_d_n6;
        locals.var_t6_dn7 = assign57570_e89726_d_n7;
        locals.var_t6_dn8 = assign57570_e89726_d_n8;
        locals.var_t6_dn9 = assign57570_e89726_d_n9;
        locals.var_t6_dn10 = assign57570_e89726_d_n10;
        locals.var_t6_dn13 = assign57570_e89726_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign57580_e89739, assign57580_e89739_d_n0, assign57580_e89739_d_n2, assign57580_e89739_d_n4, assign57580_e89739_d_n5, assign57580_e89739_d_n6, assign57580_e89739_d_n7, assign57580_e89739_d_n8, assign57580_e89739_d_n9, assign57580_e89739_d_n10, assign57580_e89739_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign57580_e89739;
        locals.var_t9_dn0 = assign57580_e89739_d_n0;
        locals.var_t9_dn2 = assign57580_e89739_d_n2;
        locals.var_t9_dn4 = assign57580_e89739_d_n4;
        locals.var_t9_dn5 = assign57580_e89739_d_n5;
        locals.var_t9_dn6 = assign57580_e89739_d_n6;
        locals.var_t9_dn7 = assign57580_e89739_d_n7;
        locals.var_t9_dn8 = assign57580_e89739_d_n8;
        locals.var_t9_dn9 = assign57580_e89739_d_n9;
        locals.var_t9_dn10 = assign57580_e89739_d_n10;
        locals.var_t9_dn13 = assign57580_e89739_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign57590_e89753, assign57590_e89753_d_n0, assign57590_e89753_d_n2, assign57590_e89753_d_n4, assign57590_e89753_d_n5, assign57590_e89753_d_n6, assign57590_e89753_d_n7, assign57590_e89753_d_n8, assign57590_e89753_d_n9, assign57590_e89753_d_n10, assign57590_e89753_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1423 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign57590_e89753;
        locals.var_t9_dn0 = assign57590_e89753_d_n0;
        locals.var_t9_dn2 = assign57590_e89753_d_n2;
        locals.var_t9_dn4 = assign57590_e89753_d_n4;
        locals.var_t9_dn5 = assign57590_e89753_d_n5;
        locals.var_t9_dn6 = assign57590_e89753_d_n6;
        locals.var_t9_dn7 = assign57590_e89753_d_n7;
        locals.var_t9_dn8 = assign57590_e89753_d_n8;
        locals.var_t9_dn9 = assign57590_e89753_d_n9;
        locals.var_t9_dn10 = assign57590_e89753_d_n10;
        locals.var_t9_dn13 = assign57590_e89753_d_n13;
        locals.var_t9_rv = 0.0;

        let assign57600_e89756: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1425 = assign57600_e89756;
        locals.var_guard1425_rv = 0.0;

        let (assign57610_e89771, assign57610_e89771_d_n0, assign57610_e89771_d_n2, assign57610_e89771_d_n4, assign57610_e89771_d_n5, assign57610_e89771_d_n6, assign57610_e89771_d_n7, assign57610_e89771_d_n8, assign57610_e89771_d_n9, assign57610_e89771_d_n10, assign57610_e89771_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57610_e89769: f64 = (locals.var_beta * locals.var_gdl0);
        (assign57610_e89769, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn13 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57610_e89771;
        locals.var_t1_dn0 = assign57610_e89771_d_n0;
        locals.var_t1_dn2 = assign57610_e89771_d_n2;
        locals.var_t1_dn4 = assign57610_e89771_d_n4;
        locals.var_t1_dn5 = assign57610_e89771_d_n5;
        locals.var_t1_dn6 = assign57610_e89771_d_n6;
        locals.var_t1_dn7 = assign57610_e89771_d_n7;
        locals.var_t1_dn8 = assign57610_e89771_d_n8;
        locals.var_t1_dn9 = assign57610_e89771_d_n9;
        locals.var_t1_dn10 = assign57610_e89771_d_n10;
        locals.var_t1_dn13 = assign57610_e89771_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57620_e89786, assign57620_e89786_d_n0, assign57620_e89786_d_n2, assign57620_e89786_d_n4, assign57620_e89786_d_n5, assign57620_e89786_d_n6, assign57620_e89786_d_n7, assign57620_e89786_d_n8, assign57620_e89786_d_n9, assign57620_e89786_d_n10, assign57620_e89786_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57620_e89784: f64 = (locals.var_cox * locals.var_t1);
        (assign57620_e89784, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn13 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57620_e89786;
        locals.var_t2_dn0 = assign57620_e89786_d_n0;
        locals.var_t2_dn2 = assign57620_e89786_d_n2;
        locals.var_t2_dn4 = assign57620_e89786_d_n4;
        locals.var_t2_dn5 = assign57620_e89786_d_n5;
        locals.var_t2_dn6 = assign57620_e89786_d_n6;
        locals.var_t2_dn7 = assign57620_e89786_d_n7;
        locals.var_t2_dn8 = assign57620_e89786_d_n8;
        locals.var_t2_dn9 = assign57620_e89786_d_n9;
        locals.var_t2_dn10 = assign57620_e89786_d_n10;
        locals.var_t2_dn13 = assign57620_e89786_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign57630_e89801, assign57630_e89801_d_n0, assign57630_e89801_d_n2, assign57630_e89801_d_n4, assign57630_e89801_d_n5, assign57630_e89801_d_n6, assign57630_e89801_d_n7, assign57630_e89801_d_n8, assign57630_e89801_d_n9, assign57630_e89801_d_n10, assign57630_e89801_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57630_e89799: f64 = (locals.var_t2 * locals.var_vdsz__blk439);
        (assign57630_e89799, ((locals.var_t2_dn0 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn10)), ((locals.var_t2_dn13 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign57630_e89801;
        locals.var_t8_dn0 = assign57630_e89801_d_n0;
        locals.var_t8_dn2 = assign57630_e89801_d_n2;
        locals.var_t8_dn4 = assign57630_e89801_d_n4;
        locals.var_t8_dn5 = assign57630_e89801_d_n5;
        locals.var_t8_dn6 = assign57630_e89801_d_n6;
        locals.var_t8_dn7 = assign57630_e89801_d_n7;
        locals.var_t8_dn8 = assign57630_e89801_d_n8;
        locals.var_t8_dn9 = assign57630_e89801_d_n9;
        locals.var_t8_dn10 = assign57630_e89801_d_n10;
        locals.var_t8_dn13 = assign57630_e89801_d_n13;
        locals.var_t8_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_203(
        locals: &mut StampLocals,
    ) {
        let (assign57640_e89815, assign57640_e89815_d_n0, assign57640_e89815_d_n2, assign57640_e89815_d_n4, assign57640_e89815_d_n5, assign57640_e89815_d_n6, assign57640_e89815_d_n7, assign57640_e89815_d_n8, assign57640_e89815_d_n9, assign57640_e89815_d_n10, assign57640_e89815_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1425 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign57640_e89815;
        locals.var_t8_dn0 = assign57640_e89815_d_n0;
        locals.var_t8_dn2 = assign57640_e89815_d_n2;
        locals.var_t8_dn4 = assign57640_e89815_d_n4;
        locals.var_t8_dn5 = assign57640_e89815_d_n5;
        locals.var_t8_dn6 = assign57640_e89815_d_n6;
        locals.var_t8_dn7 = assign57640_e89815_d_n7;
        locals.var_t8_dn8 = assign57640_e89815_d_n8;
        locals.var_t8_dn9 = assign57640_e89815_d_n9;
        locals.var_t8_dn10 = assign57640_e89815_d_n10;
        locals.var_t8_dn13 = assign57640_e89815_d_n13;
        locals.var_t8_rv = 0.0;

        let assign57650_e89818: f64 = (locals.var_t9 + locals.var_t8);
        let assign57650_e89820: f64 = if assign57650_e89818 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1426 = assign57650_e89820;
        locals.var_guard1426_rv = 0.0;

        let (assign57660_e89837, assign57660_e89837_d_n0, assign57660_e89837_d_n2, assign57660_e89837_d_n4, assign57660_e89837_d_n5, assign57660_e89837_d_n6, assign57660_e89837_d_n7, assign57660_e89837_d_n8, assign57660_e89837_d_n9, assign57660_e89837_d_n10, assign57660_e89837_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1426 != 0.0)) {
        let assign57660_e89834: f64 = (locals.var_t9 + locals.var_t8);
        let assign57660_e89835: f64 = (locals.var_pds * assign57660_e89834);
        (assign57660_e89835, ((locals.var_pds_dn0 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn13 * assign57660_e89834) + (locals.var_pds * (locals.var_t9_dn13 + locals.var_t8_dn13))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn13,)
    }
};
        locals.var_idd1 = assign57660_e89837;
        locals.var_idd1_dn0 = assign57660_e89837_d_n0;
        locals.var_idd1_dn2 = assign57660_e89837_d_n2;
        locals.var_idd1_dn4 = assign57660_e89837_d_n4;
        locals.var_idd1_dn5 = assign57660_e89837_d_n5;
        locals.var_idd1_dn6 = assign57660_e89837_d_n6;
        locals.var_idd1_dn7 = assign57660_e89837_d_n7;
        locals.var_idd1_dn8 = assign57660_e89837_d_n8;
        locals.var_idd1_dn9 = assign57660_e89837_d_n9;
        locals.var_idd1_dn10 = assign57660_e89837_d_n10;
        locals.var_idd1_dn13 = assign57660_e89837_d_n13;
        locals.var_idd1_rv = 0.0;

        let (assign57670_e89856, assign57670_e89856_d_n0, assign57670_e89856_d_n2, assign57670_e89856_d_n4, assign57670_e89856_d_n5, assign57670_e89856_d_n6, assign57670_e89856_d_n7, assign57670_e89856_d_n8, assign57670_e89856_d_n9, assign57670_e89856_d_n10, assign57670_e89856_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1426 != 0.0)) {
        let assign57670_e89851: f64 = (locals.var_betawl * locals.var_idd1);
        let assign57670_e89853: f64 = (assign57670_e89851 * locals.var_mu);
        let assign57670_e89854: f64 = (locals.var_ids0 + assign57670_e89853);
        (assign57670_e89854, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn10))), (locals.var_ids0_dn13 + ((((locals.var_betawl_dn13 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn13)) * locals.var_mu) + (assign57670_e89851 * locals.var_mu_dn13))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign57670_e89856;
        locals.var_ids0_dn0 = assign57670_e89856_d_n0;
        locals.var_ids0_dn2 = assign57670_e89856_d_n2;
        locals.var_ids0_dn4 = assign57670_e89856_d_n4;
        locals.var_ids0_dn5 = assign57670_e89856_d_n5;
        locals.var_ids0_dn6 = assign57670_e89856_d_n6;
        locals.var_ids0_dn7 = assign57670_e89856_d_n7;
        locals.var_ids0_dn8 = assign57670_e89856_d_n8;
        locals.var_ids0_dn9 = assign57670_e89856_d_n9;
        locals.var_ids0_dn10 = assign57670_e89856_d_n10;
        locals.var_ids0_dn13 = assign57670_e89856_d_n13;
        locals.var_ids0_rv = 0.0;

        let (assign57680_e89867, assign57680_e89867_d_n0, assign57680_e89867_d_n2, assign57680_e89867_d_n4, assign57680_e89867_d_n5, assign57680_e89867_d_n6, assign57680_e89867_d_n7, assign57680_e89867_d_n8, assign57680_e89867_d_n9, assign57680_e89867_d_n10, assign57680_e89867_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign57680_e89867;
        locals.var_ids_dn0 = assign57680_e89867_d_n0;
        locals.var_ids_dn2 = assign57680_e89867_d_n2;
        locals.var_ids_dn4 = assign57680_e89867_d_n4;
        locals.var_ids_dn5 = assign57680_e89867_d_n5;
        locals.var_ids_dn6 = assign57680_e89867_d_n6;
        locals.var_ids_dn7 = assign57680_e89867_d_n7;
        locals.var_ids_dn8 = assign57680_e89867_d_n8;
        locals.var_ids_dn9 = assign57680_e89867_d_n9;
        locals.var_ids_dn10 = assign57680_e89867_d_n10;
        locals.var_ids_dn13 = assign57680_e89867_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign57690_e89887, assign57690_e89887_d_n0, assign57690_e89887_d_n2, assign57690_e89887_d_n4, assign57690_e89887_d_n5, assign57690_e89887_d_n6, assign57690_e89887_d_n7, assign57690_e89887_d_n8, assign57690_e89887_d_n9, assign57690_e89887_d_n10, assign57690_e89887_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57690_e89877: f64 = (-0.5);
        let assign57690_e89880: f64 = (locals.var_q_s0__blk1098 - locals.var_q_n0__blk1122);
        let assign57690_e89882: f64 = (assign57690_e89880 + locals.var_q_sl__blk1099);
        let assign57690_e89884: f64 = (assign57690_e89882 - locals.var_q_nl__blk1123);
        let assign57690_e89885: f64 = (assign57690_e89877 * assign57690_e89884);
        (assign57690_e89885, (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn0 - locals.var_q_n0__blk1122_dn0) + locals.var_q_sl__blk1099_dn0) - locals.var_q_nl__blk1123_dn0)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn2 - locals.var_q_n0__blk1122_dn2) + locals.var_q_sl__blk1099_dn2) - locals.var_q_nl__blk1123_dn2)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn4 - locals.var_q_n0__blk1122_dn4) + locals.var_q_sl__blk1099_dn4) - locals.var_q_nl__blk1123_dn4)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn5 - locals.var_q_n0__blk1122_dn5) + locals.var_q_sl__blk1099_dn5) - locals.var_q_nl__blk1123_dn5)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn6 - locals.var_q_n0__blk1122_dn6) + locals.var_q_sl__blk1099_dn6) - locals.var_q_nl__blk1123_dn6)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn7 - locals.var_q_n0__blk1122_dn7) + locals.var_q_sl__blk1099_dn7) - locals.var_q_nl__blk1123_dn7)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn8 - locals.var_q_n0__blk1122_dn8) + locals.var_q_sl__blk1099_dn8) - locals.var_q_nl__blk1123_dn8)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn9 - locals.var_q_n0__blk1122_dn9) + locals.var_q_sl__blk1099_dn9) - locals.var_q_nl__blk1123_dn9)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn10 - locals.var_q_n0__blk1122_dn10) + locals.var_q_sl__blk1099_dn10) - locals.var_q_nl__blk1123_dn10)), (assign57690_e89877 * (((locals.var_q_s0__blk1098_dn13 - locals.var_q_n0__blk1122_dn13) + locals.var_q_sl__blk1099_dn13) - locals.var_q_nl__blk1123_dn13)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign57690_e89887;
        locals.var_qbu_dn0 = assign57690_e89887_d_n0;
        locals.var_qbu_dn2 = assign57690_e89887_d_n2;
        locals.var_qbu_dn4 = assign57690_e89887_d_n4;
        locals.var_qbu_dn5 = assign57690_e89887_d_n5;
        locals.var_qbu_dn6 = assign57690_e89887_d_n6;
        locals.var_qbu_dn7 = assign57690_e89887_d_n7;
        locals.var_qbu_dn8 = assign57690_e89887_d_n8;
        locals.var_qbu_dn9 = assign57690_e89887_d_n9;
        locals.var_qbu_dn10 = assign57690_e89887_d_n10;
        locals.var_qbu_dn13 = assign57690_e89887_d_n13;
        locals.var_qbu_rv = 0.0;

        let (assign57700_e89903, assign57700_e89903_d_n0, assign57700_e89903_d_n2, assign57700_e89903_d_n4, assign57700_e89903_d_n5, assign57700_e89903_d_n6, assign57700_e89903_d_n7, assign57700_e89903_d_n8, assign57700_e89903_d_n9, assign57700_e89903_d_n10, assign57700_e89903_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57700_e89897: f64 = (-0.5);
        let assign57700_e89900: f64 = (locals.var_q_n0__blk1122 + locals.var_q_nl__blk1123);
        let assign57700_e89901: f64 = (assign57700_e89897 * assign57700_e89900);
        (assign57700_e89901, (assign57700_e89897 * (locals.var_q_n0__blk1122_dn0 + locals.var_q_nl__blk1123_dn0)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn2 + locals.var_q_nl__blk1123_dn2)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn4 + locals.var_q_nl__blk1123_dn4)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn5 + locals.var_q_nl__blk1123_dn5)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn6 + locals.var_q_nl__blk1123_dn6)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn7 + locals.var_q_nl__blk1123_dn7)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn8 + locals.var_q_nl__blk1123_dn8)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn9 + locals.var_q_nl__blk1123_dn9)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn10 + locals.var_q_nl__blk1123_dn10)), (assign57700_e89897 * (locals.var_q_n0__blk1122_dn13 + locals.var_q_nl__blk1123_dn13)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign57700_e89903;
        locals.var_qiu_dn0 = assign57700_e89903_d_n0;
        locals.var_qiu_dn2 = assign57700_e89903_d_n2;
        locals.var_qiu_dn4 = assign57700_e89903_d_n4;
        locals.var_qiu_dn5 = assign57700_e89903_d_n5;
        locals.var_qiu_dn6 = assign57700_e89903_d_n6;
        locals.var_qiu_dn7 = assign57700_e89903_d_n7;
        locals.var_qiu_dn8 = assign57700_e89903_d_n8;
        locals.var_qiu_dn9 = assign57700_e89903_d_n9;
        locals.var_qiu_dn10 = assign57700_e89903_d_n10;
        locals.var_qiu_dn13 = assign57700_e89903_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign57710_e89914, assign57710_e89914_d_n0, assign57710_e89914_d_n2, assign57710_e89914_d_n4, assign57710_e89914_d_n5, assign57710_e89914_d_n6, assign57710_e89914_d_n7, assign57710_e89914_d_n8, assign57710_e89914_d_n9, assign57710_e89914_d_n10, assign57710_e89914_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign57710_e89914;
        locals.var_qdrat_dn0 = assign57710_e89914_d_n0;
        locals.var_qdrat_dn2 = assign57710_e89914_d_n2;
        locals.var_qdrat_dn4 = assign57710_e89914_d_n4;
        locals.var_qdrat_dn5 = assign57710_e89914_d_n5;
        locals.var_qdrat_dn6 = assign57710_e89914_d_n6;
        locals.var_qdrat_dn7 = assign57710_e89914_d_n7;
        locals.var_qdrat_dn8 = assign57710_e89914_d_n8;
        locals.var_qdrat_dn9 = assign57710_e89914_d_n9;
        locals.var_qdrat_dn10 = assign57710_e89914_d_n10;
        locals.var_qdrat_dn13 = assign57710_e89914_d_n13;
        locals.var_qdrat_rv = 0.0;

        let (assign57720_e89930, assign57720_e89930_d_n0, assign57720_e89930_d_n2, assign57720_e89930_d_n4, assign57720_e89930_d_n5, assign57720_e89930_d_n6, assign57720_e89930_d_n7, assign57720_e89930_d_n8, assign57720_e89930_d_n9, assign57720_e89930_d_n10, assign57720_e89930_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57720_e89924: f64 = (-0.5);
        let assign57720_e89927: f64 = (locals.var_q_n0__blk1122 + locals.var_q_nl__blk1123);
        let assign57720_e89928: f64 = (assign57720_e89924 * assign57720_e89927);
        (assign57720_e89928, (assign57720_e89924 * (locals.var_q_n0__blk1122_dn0 + locals.var_q_nl__blk1123_dn0)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn2 + locals.var_q_nl__blk1123_dn2)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn4 + locals.var_q_nl__blk1123_dn4)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn5 + locals.var_q_nl__blk1123_dn5)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn6 + locals.var_q_nl__blk1123_dn6)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn7 + locals.var_q_nl__blk1123_dn7)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn8 + locals.var_q_nl__blk1123_dn8)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn9 + locals.var_q_nl__blk1123_dn9)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn10 + locals.var_q_nl__blk1123_dn10)), (assign57720_e89924 * (locals.var_q_n0__blk1122_dn13 + locals.var_q_nl__blk1123_dn13)),)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn13,)
    }
};
        locals.var_qiu_noi = assign57720_e89930;
        locals.var_qiu_noi_dn0 = assign57720_e89930_d_n0;
        locals.var_qiu_noi_dn2 = assign57720_e89930_d_n2;
        locals.var_qiu_noi_dn4 = assign57720_e89930_d_n4;
        locals.var_qiu_noi_dn5 = assign57720_e89930_d_n5;
        locals.var_qiu_noi_dn6 = assign57720_e89930_d_n6;
        locals.var_qiu_noi_dn7 = assign57720_e89930_d_n7;
        locals.var_qiu_noi_dn8 = assign57720_e89930_d_n8;
        locals.var_qiu_noi_dn9 = assign57720_e89930_d_n9;
        locals.var_qiu_noi_dn10 = assign57720_e89930_d_n10;
        locals.var_qiu_noi_dn13 = assign57720_e89930_d_n13;
        locals.var_qiu_noi_rv = 0.0;

        let (assign57730_e89942, assign57730_e89942_d_n0, assign57730_e89942_d_n2, assign57730_e89942_d_n4, assign57730_e89942_d_n5, assign57730_e89942_d_n6, assign57730_e89942_d_n7, assign57730_e89942_d_n8, assign57730_e89942_d_n9, assign57730_e89942_d_n10, assign57730_e89942_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign57730_e89940: f64 = (-locals.var_q_n0__blk1122);
        (assign57730_e89940, (-locals.var_q_n0__blk1122_dn0), (-locals.var_q_n0__blk1122_dn2), (-locals.var_q_n0__blk1122_dn4), (-locals.var_q_n0__blk1122_dn5), (-locals.var_q_n0__blk1122_dn6), (-locals.var_q_n0__blk1122_dn7), (-locals.var_q_n0__blk1122_dn8), (-locals.var_q_n0__blk1122_dn9), (-locals.var_q_n0__blk1122_dn10), (-locals.var_q_n0__blk1122_dn13),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign57730_e89942;
        locals.var_qn0_dn0 = assign57730_e89942_d_n0;
        locals.var_qn0_dn2 = assign57730_e89942_d_n2;
        locals.var_qn0_dn4 = assign57730_e89942_d_n4;
        locals.var_qn0_dn5 = assign57730_e89942_d_n5;
        locals.var_qn0_dn6 = assign57730_e89942_d_n6;
        locals.var_qn0_dn7 = assign57730_e89942_d_n7;
        locals.var_qn0_dn8 = assign57730_e89942_d_n8;
        locals.var_qn0_dn9 = assign57730_e89942_d_n9;
        locals.var_qn0_dn10 = assign57730_e89942_d_n10;
        locals.var_qn0_dn13 = assign57730_e89942_d_n13;
        locals.var_qn0_rv = 0.0;

        let (assign57740_e89953, assign57740_e89953_d_n0, assign57740_e89953_d_n2, assign57740_e89953_d_n4, assign57740_e89953_d_n5, assign57740_e89953_d_n6, assign57740_e89953_d_n7, assign57740_e89953_d_n8, assign57740_e89953_d_n9, assign57740_e89953_d_n10, assign57740_e89953_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_ey_acc__blk1116, locals.var_ey_acc__blk1116_dn0, locals.var_ey_acc__blk1116_dn2, locals.var_ey_acc__blk1116_dn4, locals.var_ey_acc__blk1116_dn5, locals.var_ey_acc__blk1116_dn6, locals.var_ey_acc__blk1116_dn7, locals.var_ey_acc__blk1116_dn8, locals.var_ey_acc__blk1116_dn9, locals.var_ey_acc__blk1116_dn10, locals.var_ey_acc__blk1116_dn13,)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign57740_e89953;
        locals.var_ey_dn0 = assign57740_e89953_d_n0;
        locals.var_ey_dn2 = assign57740_e89953_d_n2;
        locals.var_ey_dn4 = assign57740_e89953_d_n4;
        locals.var_ey_dn5 = assign57740_e89953_d_n5;
        locals.var_ey_dn6 = assign57740_e89953_d_n6;
        locals.var_ey_dn7 = assign57740_e89953_d_n7;
        locals.var_ey_dn8 = assign57740_e89953_d_n8;
        locals.var_ey_dn9 = assign57740_e89953_d_n9;
        locals.var_ey_dn10 = assign57740_e89953_d_n10;
        locals.var_ey_dn13 = assign57740_e89953_d_n13;
        locals.var_ey_rv = 0.0;

        let assign57750_e89960: f64 = if ((locals.var_qn0 < 1e-25) || (locals.var_qiu < 1e-25)) { 1.0 } else { 0.0 };
        locals.var_guard1427 = assign57750_e89960;
        locals.var_guard1427_rv = 0.0;

        let (assign57760_e89973,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign57760_e89973;
        locals.var_flg_noqi_rv = 0.0;

        let assign57770_e89976: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1428 = assign57770_e89976;
        locals.var_guard1428_rv = 0.0;

        let (assign57780_e89984,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57780_e89982: f64 = (-1.0);
        (assign57780_e89982,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign57780_e89984;
        locals.var_flg_zone_rv = 0.0;

        let (assign57790_e89999, assign57790_e89999_d_n0, assign57790_e89999_d_n2, assign57790_e89999_d_n4, assign57790_e89999_d_n5, assign57790_e89999_d_n6, assign57790_e89999_d_n7, assign57790_e89999_d_n8, assign57790_e89999_d_n9, assign57790_e89999_d_n10, assign57790_e89999_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57790_e89991: f64 = (2.0 * locals.var_beta_inv);
        let assign57790_e89993: f64 = (-locals.var_vgs_min);
        let assign57790_e89995: f64 = (assign57790_e89993 / locals.var_fac1);
        let assign57790_e89996: f64 = (assign57790_e89995).ln();
        let assign57790_e89997: f64 = (assign57790_e89991 * assign57790_e89996);
        (assign57790_e89997, (((2.0 * locals.var_beta_inv_dn0) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn2) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn4) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn5) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn6) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn7) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn8) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn9) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn10) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))), (((2.0 * locals.var_beta_inv_dn13) * assign57790_e89996) + (assign57790_e89991 * ((-((assign57790_e89993 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign57790_e89995))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign57790_e89999;
        locals.var_ps0_min_dn0 = assign57790_e89999_d_n0;
        locals.var_ps0_min_dn2 = assign57790_e89999_d_n2;
        locals.var_ps0_min_dn4 = assign57790_e89999_d_n4;
        locals.var_ps0_min_dn5 = assign57790_e89999_d_n5;
        locals.var_ps0_min_dn6 = assign57790_e89999_d_n6;
        locals.var_ps0_min_dn7 = assign57790_e89999_d_n7;
        locals.var_ps0_min_dn8 = assign57790_e89999_d_n8;
        locals.var_ps0_min_dn9 = assign57790_e89999_d_n9;
        locals.var_ps0_min_dn10 = assign57790_e89999_d_n10;
        locals.var_ps0_min_dn13 = assign57790_e89999_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign57800_e90010, assign57800_e90010_d_n0, assign57800_e90010_d_n2, assign57800_e90010_d_n4, assign57800_e90010_d_n5, assign57800_e90010_d_n6, assign57800_e90010_d_n7, assign57800_e90010_d_n8, assign57800_e90010_d_n9, assign57800_e90010_d_n10, assign57800_e90010_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57800_e90007: f64 = (locals.var_vgp - locals.var_vbscl__blk435);
        let assign57800_e90008: f64 = (locals.var_beta * assign57800_e90007);
        (assign57800_e90008, ((locals.var_beta_dn0 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk435_dn0))), ((locals.var_beta_dn2 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk435_dn2))), ((locals.var_beta_dn4 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk435_dn4))), ((locals.var_beta_dn5 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk435_dn5))), ((locals.var_beta_dn6 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk435_dn6))), ((locals.var_beta_dn7 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk435_dn7))), ((locals.var_beta_dn8 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk435_dn8))), ((locals.var_beta_dn9 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk435_dn9))), ((locals.var_beta_dn10 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk435_dn10))), ((locals.var_beta_dn13 * assign57800_e90007) + (locals.var_beta * (locals.var_vgp_dn13 - locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign57800_e90010;
        locals.var_tx_dn0 = assign57800_e90010_d_n0;
        locals.var_tx_dn2 = assign57800_e90010_d_n2;
        locals.var_tx_dn4 = assign57800_e90010_d_n4;
        locals.var_tx_dn5 = assign57800_e90010_d_n5;
        locals.var_tx_dn6 = assign57800_e90010_d_n6;
        locals.var_tx_dn7 = assign57800_e90010_d_n7;
        locals.var_tx_dn8 = assign57800_e90010_d_n8;
        locals.var_tx_dn9 = assign57800_e90010_d_n9;
        locals.var_tx_dn10 = assign57800_e90010_d_n10;
        locals.var_tx_dn13 = assign57800_e90010_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign57810_e90021, assign57810_e90021_d_n0, assign57810_e90021_d_n2, assign57810_e90021_d_n4, assign57810_e90021_d_n5, assign57810_e90021_d_n6, assign57810_e90021_d_n7, assign57810_e90021_d_n8, assign57810_e90021_d_n9, assign57810_e90021_d_n10, assign57810_e90021_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57810_e90018: f64 = (locals.var_beta * locals.var_cnst0);
        let assign57810_e90019: f64 = (1.0 / assign57810_e90018);
        (assign57810_e90019, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign57810_e90018 * assign57810_e90018))), (-(((locals.var_beta_dn13 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn13)) / (assign57810_e90018 * assign57810_e90018))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57810_e90021;
        locals.var_t1_dn0 = assign57810_e90021_d_n0;
        locals.var_t1_dn2 = assign57810_e90021_d_n2;
        locals.var_t1_dn4 = assign57810_e90021_d_n4;
        locals.var_t1_dn5 = assign57810_e90021_d_n5;
        locals.var_t1_dn6 = assign57810_e90021_d_n6;
        locals.var_t1_dn7 = assign57810_e90021_d_n7;
        locals.var_t1_dn8 = assign57810_e90021_d_n8;
        locals.var_t1_dn9 = assign57810_e90021_d_n9;
        locals.var_t1_dn10 = assign57810_e90021_d_n10;
        locals.var_t1_dn13 = assign57810_e90021_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57820_e90030, assign57820_e90030_d_n0, assign57820_e90030_d_n2, assign57820_e90030_d_n4, assign57820_e90030_d_n5, assign57820_e90030_d_n6, assign57820_e90030_d_n7, assign57820_e90030_d_n8, assign57820_e90030_d_n9, assign57820_e90030_d_n10, assign57820_e90030_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57820_e90028: f64 = (locals.var_t1 * locals.var_cox);
        (assign57820_e90028, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn13 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign57820_e90030;
        locals.var_ty_dn0 = assign57820_e90030_d_n0;
        locals.var_ty_dn2 = assign57820_e90030_d_n2;
        locals.var_ty_dn4 = assign57820_e90030_d_n4;
        locals.var_ty_dn5 = assign57820_e90030_d_n5;
        locals.var_ty_dn6 = assign57820_e90030_d_n6;
        locals.var_ty_dn7 = assign57820_e90030_d_n7;
        locals.var_ty_dn8 = assign57820_e90030_d_n8;
        locals.var_ty_dn9 = assign57820_e90030_d_n9;
        locals.var_ty_dn10 = assign57820_e90030_d_n10;
        locals.var_ty_dn13 = assign57820_e90030_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign57830_e90043, assign57830_e90043_d_n0, assign57830_e90043_d_n2, assign57830_e90043_d_n4, assign57830_e90043_d_n5, assign57830_e90043_d_n6, assign57830_e90043_d_n7, assign57830_e90043_d_n8, assign57830_e90043_d_n9, assign57830_e90043_d_n10, assign57830_e90043_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57830_e90038: f64 = (3.0 * 1.414213562373095);
        let assign57830_e90040: f64 = (assign57830_e90038 * locals.var_ty);
        let assign57830_e90041: f64 = (2.0 + assign57830_e90040);
        (assign57830_e90041, (assign57830_e90038 * locals.var_ty_dn0), (assign57830_e90038 * locals.var_ty_dn2), (assign57830_e90038 * locals.var_ty_dn4), (assign57830_e90038 * locals.var_ty_dn5), (assign57830_e90038 * locals.var_ty_dn6), (assign57830_e90038 * locals.var_ty_dn7), (assign57830_e90038 * locals.var_ty_dn8), (assign57830_e90038 * locals.var_ty_dn9), (assign57830_e90038 * locals.var_ty_dn10), (assign57830_e90038 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign57830_e90043;
        locals.var_ac41_dn0 = assign57830_e90043_d_n0;
        locals.var_ac41_dn2 = assign57830_e90043_d_n2;
        locals.var_ac41_dn4 = assign57830_e90043_d_n4;
        locals.var_ac41_dn5 = assign57830_e90043_d_n5;
        locals.var_ac41_dn6 = assign57830_e90043_d_n6;
        locals.var_ac41_dn7 = assign57830_e90043_d_n7;
        locals.var_ac41_dn8 = assign57830_e90043_d_n8;
        locals.var_ac41_dn9 = assign57830_e90043_d_n9;
        locals.var_ac41_dn10 = assign57830_e90043_d_n10;
        locals.var_ac41_dn13 = assign57830_e90043_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign57840_e90056, assign57840_e90056_d_n0, assign57840_e90056_d_n2, assign57840_e90056_d_n4, assign57840_e90056_d_n5, assign57840_e90056_d_n6, assign57840_e90056_d_n7, assign57840_e90056_d_n8, assign57840_e90056_d_n9, assign57840_e90056_d_n10, assign57840_e90056_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57840_e90050: f64 = (8.0 * locals.var_ac41);
        let assign57840_e90052: f64 = (assign57840_e90050 * locals.var_ac41);
        let assign57840_e90054: f64 = (assign57840_e90052 * locals.var_ac41);
        (assign57840_e90054, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign57840_e90050 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign57840_e90052 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign57840_e90056;
        locals.var_ac4_dn0 = assign57840_e90056_d_n0;
        locals.var_ac4_dn2 = assign57840_e90056_d_n2;
        locals.var_ac4_dn4 = assign57840_e90056_d_n4;
        locals.var_ac4_dn5 = assign57840_e90056_d_n5;
        locals.var_ac4_dn6 = assign57840_e90056_d_n6;
        locals.var_ac4_dn7 = assign57840_e90056_d_n7;
        locals.var_ac4_dn8 = assign57840_e90056_d_n8;
        locals.var_ac4_dn9 = assign57840_e90056_d_n9;
        locals.var_ac4_dn10 = assign57840_e90056_d_n10;
        locals.var_ac4_dn13 = assign57840_e90056_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign57850_e90065, assign57850_e90065_d_n0, assign57850_e90065_d_n2, assign57850_e90065_d_n4, assign57850_e90065_d_n5, assign57850_e90065_d_n6, assign57850_e90065_d_n7, assign57850_e90065_d_n8, assign57850_e90065_d_n9, assign57850_e90065_d_n10, assign57850_e90065_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57850_e90063: f64 = (locals.var_tx - 2.0);
        (assign57850_e90063, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign57850_e90065;
        locals.var_t4_dn0 = assign57850_e90065_d_n0;
        locals.var_t4_dn2 = assign57850_e90065_d_n2;
        locals.var_t4_dn4 = assign57850_e90065_d_n4;
        locals.var_t4_dn5 = assign57850_e90065_d_n5;
        locals.var_t4_dn6 = assign57850_e90065_d_n6;
        locals.var_t4_dn7 = assign57850_e90065_d_n7;
        locals.var_t4_dn8 = assign57850_e90065_d_n8;
        locals.var_t4_dn9 = assign57850_e90065_d_n9;
        locals.var_t4_dn10 = assign57850_e90065_d_n10;
        locals.var_t4_dn13 = assign57850_e90065_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign57860_e90076, assign57860_e90076_d_n0, assign57860_e90076_d_n2, assign57860_e90076_d_n4, assign57860_e90076_d_n5, assign57860_e90076_d_n6, assign57860_e90076_d_n7, assign57860_e90076_d_n8, assign57860_e90076_d_n9, assign57860_e90076_d_n10, assign57860_e90076_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57860_e90072: f64 = (9.0 * locals.var_ty);
        let assign57860_e90074: f64 = (assign57860_e90072 * locals.var_t4);
        (assign57860_e90074, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn13) * locals.var_t4) + (assign57860_e90072 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign57860_e90076;
        locals.var_t5_dn0 = assign57860_e90076_d_n0;
        locals.var_t5_dn2 = assign57860_e90076_d_n2;
        locals.var_t5_dn4 = assign57860_e90076_d_n4;
        locals.var_t5_dn5 = assign57860_e90076_d_n5;
        locals.var_t5_dn6 = assign57860_e90076_d_n6;
        locals.var_t5_dn7 = assign57860_e90076_d_n7;
        locals.var_t5_dn8 = assign57860_e90076_d_n8;
        locals.var_t5_dn9 = assign57860_e90076_d_n9;
        locals.var_t5_dn10 = assign57860_e90076_d_n10;
        locals.var_t5_dn13 = assign57860_e90076_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign57870_e90087, assign57870_e90087_d_n0, assign57870_e90087_d_n2, assign57870_e90087_d_n4, assign57870_e90087_d_n5, assign57870_e90087_d_n6, assign57870_e90087_d_n7, assign57870_e90087_d_n8, assign57870_e90087_d_n9, assign57870_e90087_d_n10, assign57870_e90087_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57870_e90083: f64 = (7.0 * 1.414213562373095);
        let assign57870_e90085: f64 = (assign57870_e90083 - locals.var_t5);
        (assign57870_e90085, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn13),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign57870_e90087;
        locals.var_ac31_dn0 = assign57870_e90087_d_n0;
        locals.var_ac31_dn2 = assign57870_e90087_d_n2;
        locals.var_ac31_dn4 = assign57870_e90087_d_n4;
        locals.var_ac31_dn5 = assign57870_e90087_d_n5;
        locals.var_ac31_dn6 = assign57870_e90087_d_n6;
        locals.var_ac31_dn7 = assign57870_e90087_d_n7;
        locals.var_ac31_dn8 = assign57870_e90087_d_n8;
        locals.var_ac31_dn9 = assign57870_e90087_d_n9;
        locals.var_ac31_dn10 = assign57870_e90087_d_n10;
        locals.var_ac31_dn13 = assign57870_e90087_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign57880_e90096, assign57880_e90096_d_n0, assign57880_e90096_d_n2, assign57880_e90096_d_n4, assign57880_e90096_d_n5, assign57880_e90096_d_n6, assign57880_e90096_d_n7, assign57880_e90096_d_n8, assign57880_e90096_d_n9, assign57880_e90096_d_n10, assign57880_e90096_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57880_e90094: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign57880_e90094, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign57880_e90096;
        locals.var_ac3_dn0 = assign57880_e90096_d_n0;
        locals.var_ac3_dn2 = assign57880_e90096_d_n2;
        locals.var_ac3_dn4 = assign57880_e90096_d_n4;
        locals.var_ac3_dn5 = assign57880_e90096_d_n5;
        locals.var_ac3_dn6 = assign57880_e90096_d_n6;
        locals.var_ac3_dn7 = assign57880_e90096_d_n7;
        locals.var_ac3_dn8 = assign57880_e90096_d_n8;
        locals.var_ac3_dn9 = assign57880_e90096_d_n9;
        locals.var_ac3_dn10 = assign57880_e90096_d_n10;
        locals.var_ac3_dn13 = assign57880_e90096_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign57890_e90100: f64 = (locals.var_ac3 * 1e-8);
        let assign57890_e90101: f64 = if locals.var_ac4 < assign57890_e90100 { 1.0 } else { 0.0 };
        locals.var_guard1429 = assign57890_e90101;
        locals.var_guard1429_rv = 0.0;

        let (assign57900_e90114, assign57900_e90114_d_n0, assign57900_e90114_d_n2, assign57900_e90114_d_n4, assign57900_e90114_d_n5, assign57900_e90114_d_n6, assign57900_e90114_d_n7, assign57900_e90114_d_n8, assign57900_e90114_d_n9, assign57900_e90114_d_n10, assign57900_e90114_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) && (locals.var_guard1429 != 0.0)) {
        let assign57900_e90110: f64 = (0.5 * locals.var_ac4);
        let assign57900_e90112: f64 = (assign57900_e90110 / locals.var_ac31);
        (assign57900_e90112, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign57900_e90110 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign57900_e90114;
        locals.var_ac1_dn0 = assign57900_e90114_d_n0;
        locals.var_ac1_dn2 = assign57900_e90114_d_n2;
        locals.var_ac1_dn4 = assign57900_e90114_d_n4;
        locals.var_ac1_dn5 = assign57900_e90114_d_n5;
        locals.var_ac1_dn6 = assign57900_e90114_d_n6;
        locals.var_ac1_dn7 = assign57900_e90114_d_n7;
        locals.var_ac1_dn8 = assign57900_e90114_d_n8;
        locals.var_ac1_dn9 = assign57900_e90114_d_n9;
        locals.var_ac1_dn10 = assign57900_e90114_d_n10;
        locals.var_ac1_dn13 = assign57900_e90114_d_n13;
        locals.var_ac1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_204(
        locals: &mut StampLocals,
    ) {
        let (assign57910_e90127, assign57910_e90127_d_n0, assign57910_e90127_d_n2, assign57910_e90127_d_n4, assign57910_e90127_d_n5, assign57910_e90127_d_n6, assign57910_e90127_d_n7, assign57910_e90127_d_n8, assign57910_e90127_d_n9, assign57910_e90127_d_n10, assign57910_e90127_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) && (locals.var_guard1429 == 0.0)) {
        let assign57910_e90124: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign57910_e90125: f64 = (assign57910_e90124).sqrt();
        (assign57910_e90125, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign57910_e90125)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign57910_e90125)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign57910_e90127;
        locals.var_ac2_dn0 = assign57910_e90127_d_n0;
        locals.var_ac2_dn2 = assign57910_e90127_d_n2;
        locals.var_ac2_dn4 = assign57910_e90127_d_n4;
        locals.var_ac2_dn5 = assign57910_e90127_d_n5;
        locals.var_ac2_dn6 = assign57910_e90127_d_n6;
        locals.var_ac2_dn7 = assign57910_e90127_d_n7;
        locals.var_ac2_dn8 = assign57910_e90127_d_n8;
        locals.var_ac2_dn9 = assign57910_e90127_d_n9;
        locals.var_ac2_dn10 = assign57910_e90127_d_n10;
        locals.var_ac2_dn13 = assign57910_e90127_d_n13;
        locals.var_ac2_rv = 0.0;

        let (assign57920_e90140, assign57920_e90140_d_n0, assign57920_e90140_d_n2, assign57920_e90140_d_n4, assign57920_e90140_d_n5, assign57920_e90140_d_n6, assign57920_e90140_d_n7, assign57920_e90140_d_n8, assign57920_e90140_d_n9, assign57920_e90140_d_n10, assign57920_e90140_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) && (locals.var_guard1429 == 0.0)) {
        let assign57920_e90136: f64 = (-locals.var_ac31);
        let assign57920_e90138: f64 = (assign57920_e90136 + locals.var_ac2);
        (assign57920_e90138, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign57920_e90140;
        locals.var_ac1_dn0 = assign57920_e90140_d_n0;
        locals.var_ac1_dn2 = assign57920_e90140_d_n2;
        locals.var_ac1_dn4 = assign57920_e90140_d_n4;
        locals.var_ac1_dn5 = assign57920_e90140_d_n5;
        locals.var_ac1_dn6 = assign57920_e90140_d_n6;
        locals.var_ac1_dn7 = assign57920_e90140_d_n7;
        locals.var_ac1_dn8 = assign57920_e90140_d_n8;
        locals.var_ac1_dn9 = assign57920_e90140_d_n9;
        locals.var_ac1_dn10 = assign57920_e90140_d_n10;
        locals.var_ac1_dn13 = assign57920_e90140_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign57930_e90154, assign57930_e90154_d_n0, assign57930_e90154_d_n2, assign57930_e90154_d_n4, assign57930_e90154_d_n5, assign57930_e90154_d_n6, assign57930_e90154_d_n7, assign57930_e90154_d_n8, assign57930_e90154_d_n9, assign57930_e90154_d_n10, assign57930_e90154_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let (assign57930_e90152, assign57930_e90152_d_n0, assign57930_e90152_d_n2, assign57930_e90152_d_n4, assign57930_e90152_d_n5, assign57930_e90152_d_n6, assign57930_e90152_d_n7, assign57930_e90152_d_n8, assign57930_e90152_d_n9, assign57930_e90152_d_n10, assign57930_e90152_d_n13,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57930_e90151: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign57930_e90151, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign57930_e90151 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
            }
        };
        (assign57930_e90152, assign57930_e90152_d_n0, assign57930_e90152_d_n2, assign57930_e90152_d_n4, assign57930_e90152_d_n5, assign57930_e90152_d_n6, assign57930_e90152_d_n7, assign57930_e90152_d_n8, assign57930_e90152_d_n9, assign57930_e90152_d_n10, assign57930_e90152_d_n13,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign57930_e90154;
        locals.var_acd_dn0 = assign57930_e90154_d_n0;
        locals.var_acd_dn2 = assign57930_e90154_d_n2;
        locals.var_acd_dn4 = assign57930_e90154_d_n4;
        locals.var_acd_dn5 = assign57930_e90154_d_n5;
        locals.var_acd_dn6 = assign57930_e90154_d_n6;
        locals.var_acd_dn7 = assign57930_e90154_d_n7;
        locals.var_acd_dn8 = assign57930_e90154_d_n8;
        locals.var_acd_dn9 = assign57930_e90154_d_n9;
        locals.var_acd_dn10 = assign57930_e90154_d_n10;
        locals.var_acd_dn13 = assign57930_e90154_d_n13;
        locals.var_acd_rv = 0.0;

        let (assign57940_e90178, assign57940_e90178_d_n0, assign57940_e90178_d_n2, assign57940_e90178_d_n4, assign57940_e90178_d_n5, assign57940_e90178_d_n6, assign57940_e90178_d_n7, assign57940_e90178_d_n8, assign57940_e90178_d_n9, assign57940_e90178_d_n10, assign57940_e90178_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57940_e90160: f64 = (-4.0);
        let assign57940_e90162: f64 = (assign57940_e90160 * 1.414213562373095);
        let assign57940_e90165: f64 = (12.0 * locals.var_ty);
        let assign57940_e90166: f64 = (assign57940_e90162 - assign57940_e90165);
        let assign57940_e90169: f64 = (2.0 * locals.var_acd);
        let assign57940_e90170: f64 = (assign57940_e90166 + assign57940_e90169);
        let assign57940_e90173: f64 = (1.414213562373095 * locals.var_acd);
        let assign57940_e90175: f64 = (assign57940_e90173 * locals.var_acd);
        let assign57940_e90176: f64 = (assign57940_e90170 + assign57940_e90175);
        (assign57940_e90176, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign57940_e90173 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign57940_e90178;
        locals.var_acn_dn0 = assign57940_e90178_d_n0;
        locals.var_acn_dn2 = assign57940_e90178_d_n2;
        locals.var_acn_dn4 = assign57940_e90178_d_n4;
        locals.var_acn_dn5 = assign57940_e90178_d_n5;
        locals.var_acn_dn6 = assign57940_e90178_d_n6;
        locals.var_acn_dn7 = assign57940_e90178_d_n7;
        locals.var_acn_dn8 = assign57940_e90178_d_n8;
        locals.var_acn_dn9 = assign57940_e90178_d_n9;
        locals.var_acn_dn10 = assign57940_e90178_d_n10;
        locals.var_acn_dn13 = assign57940_e90178_d_n13;
        locals.var_acn_rv = 0.0;

        let (assign57950_e90187, assign57950_e90187_d_n0, assign57950_e90187_d_n2, assign57950_e90187_d_n4, assign57950_e90187_d_n5, assign57950_e90187_d_n6, assign57950_e90187_d_n7, assign57950_e90187_d_n8, assign57950_e90187_d_n9, assign57950_e90187_d_n10, assign57950_e90187_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57950_e90185: f64 = (1.0 / locals.var_acd);
        (assign57950_e90185, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn13 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57950_e90187;
        locals.var_t1_dn0 = assign57950_e90187_d_n0;
        locals.var_t1_dn2 = assign57950_e90187_d_n2;
        locals.var_t1_dn4 = assign57950_e90187_d_n4;
        locals.var_t1_dn5 = assign57950_e90187_d_n5;
        locals.var_t1_dn6 = assign57950_e90187_d_n6;
        locals.var_t1_dn7 = assign57950_e90187_d_n7;
        locals.var_t1_dn8 = assign57950_e90187_d_n8;
        locals.var_t1_dn9 = assign57950_e90187_d_n9;
        locals.var_t1_dn10 = assign57950_e90187_d_n10;
        locals.var_t1_dn13 = assign57950_e90187_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57960_e90196, assign57960_e90196_d_n0, assign57960_e90196_d_n2, assign57960_e90196_d_n4, assign57960_e90196_d_n5, assign57960_e90196_d_n6, assign57960_e90196_d_n7, assign57960_e90196_d_n8, assign57960_e90196_d_n9, assign57960_e90196_d_n10, assign57960_e90196_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57960_e90194: f64 = (locals.var_acn * locals.var_t1);
        (assign57960_e90194, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn13 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign57960_e90196;
        locals.var_chi_dn0 = assign57960_e90196_d_n0;
        locals.var_chi_dn2 = assign57960_e90196_d_n2;
        locals.var_chi_dn4 = assign57960_e90196_d_n4;
        locals.var_chi_dn5 = assign57960_e90196_d_n5;
        locals.var_chi_dn6 = assign57960_e90196_d_n6;
        locals.var_chi_dn7 = assign57960_e90196_d_n7;
        locals.var_chi_dn8 = assign57960_e90196_d_n8;
        locals.var_chi_dn9 = assign57960_e90196_d_n9;
        locals.var_chi_dn10 = assign57960_e90196_d_n10;
        locals.var_chi_dn13 = assign57960_e90196_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign57970_e90207, assign57970_e90207_d_n0, assign57970_e90207_d_n2, assign57970_e90207_d_n4, assign57970_e90207_d_n5, assign57970_e90207_d_n6, assign57970_e90207_d_n7, assign57970_e90207_d_n8, assign57970_e90207_d_n9, assign57970_e90207_d_n10, assign57970_e90207_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57970_e90203: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign57970_e90205: f64 = (assign57970_e90203 + locals.var_vbscl__blk435);
        (assign57970_e90205, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk435_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk435_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk435_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk435_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk435_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk435_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk435_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk435_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk435_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk435_dn13),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn13,)
    }
};
        locals.var_psa = assign57970_e90207;
        locals.var_psa_dn0 = assign57970_e90207_d_n0;
        locals.var_psa_dn2 = assign57970_e90207_d_n2;
        locals.var_psa_dn4 = assign57970_e90207_d_n4;
        locals.var_psa_dn5 = assign57970_e90207_d_n5;
        locals.var_psa_dn6 = assign57970_e90207_d_n6;
        locals.var_psa_dn7 = assign57970_e90207_d_n7;
        locals.var_psa_dn8 = assign57970_e90207_d_n8;
        locals.var_psa_dn9 = assign57970_e90207_d_n9;
        locals.var_psa_dn10 = assign57970_e90207_d_n10;
        locals.var_psa_dn13 = assign57970_e90207_d_n13;
        locals.var_psa_rv = 0.0;

        let (assign57980_e90216, assign57980_e90216_d_n0, assign57980_e90216_d_n2, assign57980_e90216_d_n4, assign57980_e90216_d_n5, assign57980_e90216_d_n6, assign57980_e90216_d_n7, assign57980_e90216_d_n8, assign57980_e90216_d_n9, assign57980_e90216_d_n10, assign57980_e90216_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57980_e90214: f64 = (locals.var_psa - locals.var_vbscl__blk435);
        (assign57980_e90214, (locals.var_psa_dn0 - locals.var_vbscl__blk435_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk435_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk435_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk435_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk435_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk435_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk435_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk435_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk435_dn10), (locals.var_psa_dn13 - locals.var_vbscl__blk435_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign57980_e90216;
        locals.var_t1_dn0 = assign57980_e90216_d_n0;
        locals.var_t1_dn2 = assign57980_e90216_d_n2;
        locals.var_t1_dn4 = assign57980_e90216_d_n4;
        locals.var_t1_dn5 = assign57980_e90216_d_n5;
        locals.var_t1_dn6 = assign57980_e90216_d_n6;
        locals.var_t1_dn7 = assign57980_e90216_d_n7;
        locals.var_t1_dn8 = assign57980_e90216_d_n8;
        locals.var_t1_dn9 = assign57980_e90216_d_n9;
        locals.var_t1_dn10 = assign57980_e90216_d_n10;
        locals.var_t1_dn13 = assign57980_e90216_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign57990_e90225, assign57990_e90225_d_n0, assign57990_e90225_d_n2, assign57990_e90225_d_n4, assign57990_e90225_d_n5, assign57990_e90225_d_n6, assign57990_e90225_d_n7, assign57990_e90225_d_n8, assign57990_e90225_d_n9, assign57990_e90225_d_n10, assign57990_e90225_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign57990_e90223: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign57990_e90223, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign57990_e90225;
        locals.var_t2_dn0 = assign57990_e90225_d_n0;
        locals.var_t2_dn2 = assign57990_e90225_d_n2;
        locals.var_t2_dn4 = assign57990_e90225_d_n4;
        locals.var_t2_dn5 = assign57990_e90225_d_n5;
        locals.var_t2_dn6 = assign57990_e90225_d_n6;
        locals.var_t2_dn7 = assign57990_e90225_d_n7;
        locals.var_t2_dn8 = assign57990_e90225_d_n8;
        locals.var_t2_dn9 = assign57990_e90225_d_n9;
        locals.var_t2_dn10 = assign57990_e90225_d_n10;
        locals.var_t2_dn13 = assign57990_e90225_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign58000_e90237, assign58000_e90237_d_n0, assign58000_e90237_d_n2, assign58000_e90237_d_n4, assign58000_e90237_d_n5, assign58000_e90237_d_n6, assign58000_e90237_d_n7, assign58000_e90237_d_n8, assign58000_e90237_d_n9, assign58000_e90237_d_n10, assign58000_e90237_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign58000_e90233: f64 = (locals.var_t2 * locals.var_t2);
        let assign58000_e90234: f64 = (1.0 + assign58000_e90233);
        let assign58000_e90235: f64 = (assign58000_e90234).sqrt();
        (assign58000_e90235, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign58000_e90235)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign58000_e90235)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign58000_e90237;
        locals.var_t3_dn0 = assign58000_e90237_d_n0;
        locals.var_t3_dn2 = assign58000_e90237_d_n2;
        locals.var_t3_dn4 = assign58000_e90237_d_n4;
        locals.var_t3_dn5 = assign58000_e90237_d_n5;
        locals.var_t3_dn6 = assign58000_e90237_d_n6;
        locals.var_t3_dn7 = assign58000_e90237_d_n7;
        locals.var_t3_dn8 = assign58000_e90237_d_n8;
        locals.var_t3_dn9 = assign58000_e90237_d_n9;
        locals.var_t3_dn10 = assign58000_e90237_d_n10;
        locals.var_t3_dn13 = assign58000_e90237_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign58010_e90248, assign58010_e90248_d_n0, assign58010_e90248_d_n2, assign58010_e90248_d_n4, assign58010_e90248_d_n5, assign58010_e90248_d_n6, assign58010_e90248_d_n7, assign58010_e90248_d_n8, assign58010_e90248_d_n9, assign58010_e90248_d_n10, assign58010_e90248_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign58010_e90244: f64 = (locals.var_t1 / locals.var_t3);
        let assign58010_e90246: f64 = (assign58010_e90244 + locals.var_vbscl__blk435);
        (assign58010_e90246, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk435_dn13),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    }
};
        locals.var_ps0 = assign58010_e90248;
        locals.var_ps0_dn0 = assign58010_e90248_d_n0;
        locals.var_ps0_dn2 = assign58010_e90248_d_n2;
        locals.var_ps0_dn4 = assign58010_e90248_d_n4;
        locals.var_ps0_dn5 = assign58010_e90248_d_n5;
        locals.var_ps0_dn6 = assign58010_e90248_d_n6;
        locals.var_ps0_dn7 = assign58010_e90248_d_n7;
        locals.var_ps0_dn8 = assign58010_e90248_d_n8;
        locals.var_ps0_dn9 = assign58010_e90248_d_n9;
        locals.var_ps0_dn10 = assign58010_e90248_d_n10;
        locals.var_ps0_dn13 = assign58010_e90248_d_n13;
        locals.var_ps0_rv = 0.0;

        let (assign58020_e90255, assign58020_e90255_d_n0, assign58020_e90255_d_n2, assign58020_e90255_d_n4, assign58020_e90255_d_n5, assign58020_e90255_d_n6, assign58020_e90255_d_n7, assign58020_e90255_d_n8, assign58020_e90255_d_n9, assign58020_e90255_d_n10, assign58020_e90255_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
        locals.var_psl = assign58020_e90255;
        locals.var_psl_dn0 = assign58020_e90255_d_n0;
        locals.var_psl_dn2 = assign58020_e90255_d_n2;
        locals.var_psl_dn4 = assign58020_e90255_d_n4;
        locals.var_psl_dn5 = assign58020_e90255_d_n5;
        locals.var_psl_dn6 = assign58020_e90255_d_n6;
        locals.var_psl_dn7 = assign58020_e90255_d_n7;
        locals.var_psl_dn8 = assign58020_e90255_d_n8;
        locals.var_psl_dn9 = assign58020_e90255_d_n9;
        locals.var_psl_dn10 = assign58020_e90255_d_n10;
        locals.var_psl_dn13 = assign58020_e90255_d_n13;
        locals.var_psl_rv = 0.0;

        let (assign58030_e90262, assign58030_e90262_d_n0, assign58030_e90262_d_n2, assign58030_e90262_d_n4, assign58030_e90262_d_n5, assign58030_e90262_d_n6, assign58030_e90262_d_n7, assign58030_e90262_d_n8, assign58030_e90262_d_n9, assign58030_e90262_d_n10, assign58030_e90262_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign58030_e90262;
        locals.var_pds_dn0 = assign58030_e90262_d_n0;
        locals.var_pds_dn2 = assign58030_e90262_d_n2;
        locals.var_pds_dn4 = assign58030_e90262_d_n4;
        locals.var_pds_dn5 = assign58030_e90262_d_n5;
        locals.var_pds_dn6 = assign58030_e90262_d_n6;
        locals.var_pds_dn7 = assign58030_e90262_d_n7;
        locals.var_pds_dn8 = assign58030_e90262_d_n8;
        locals.var_pds_dn9 = assign58030_e90262_d_n9;
        locals.var_pds_dn10 = assign58030_e90262_d_n10;
        locals.var_pds_dn13 = assign58030_e90262_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign58040_e90271, assign58040_e90271_d_n0, assign58040_e90271_d_n2, assign58040_e90271_d_n4, assign58040_e90271_d_n5, assign58040_e90271_d_n6, assign58040_e90271_d_n7, assign58040_e90271_d_n8, assign58040_e90271_d_n9, assign58040_e90271_d_n10, assign58040_e90271_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign58040_e90269: f64 = (locals.var_vgp - locals.var_ps0);
        (assign58040_e90269, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn4 - locals.var_ps0_dn4), (locals.var_vgp_dn5 - locals.var_ps0_dn5), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn8 - locals.var_ps0_dn8), (locals.var_vgp_dn9 - locals.var_ps0_dn9), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn13 - locals.var_ps0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign58040_e90271;
        locals.var_t2_dn0 = assign58040_e90271_d_n0;
        locals.var_t2_dn2 = assign58040_e90271_d_n2;
        locals.var_t2_dn4 = assign58040_e90271_d_n4;
        locals.var_t2_dn5 = assign58040_e90271_d_n5;
        locals.var_t2_dn6 = assign58040_e90271_d_n6;
        locals.var_t2_dn7 = assign58040_e90271_d_n7;
        locals.var_t2_dn8 = assign58040_e90271_d_n8;
        locals.var_t2_dn9 = assign58040_e90271_d_n9;
        locals.var_t2_dn10 = assign58040_e90271_d_n10;
        locals.var_t2_dn13 = assign58040_e90271_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign58050_e90280, assign58050_e90280_d_n0, assign58050_e90280_d_n2, assign58050_e90280_d_n4, assign58050_e90280_d_n5, assign58050_e90280_d_n6, assign58050_e90280_d_n7, assign58050_e90280_d_n8, assign58050_e90280_d_n9, assign58050_e90280_d_n10, assign58050_e90280_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        let assign58050_e90278: f64 = (locals.var_cox * locals.var_t2);
        (assign58050_e90278, ((locals.var_cox_dn0 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn0)), ((locals.var_cox_dn2 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn2)), ((locals.var_cox_dn4 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn4)), ((locals.var_cox_dn5 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn5)), ((locals.var_cox_dn6 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn6)), ((locals.var_cox_dn7 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn7)), ((locals.var_cox_dn8 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn8)), ((locals.var_cox_dn9 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn9)), ((locals.var_cox_dn10 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn10)), ((locals.var_cox_dn13 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign58050_e90280;
        locals.var_qbu_dn0 = assign58050_e90280_d_n0;
        locals.var_qbu_dn2 = assign58050_e90280_d_n2;
        locals.var_qbu_dn4 = assign58050_e90280_d_n4;
        locals.var_qbu_dn5 = assign58050_e90280_d_n5;
        locals.var_qbu_dn6 = assign58050_e90280_d_n6;
        locals.var_qbu_dn7 = assign58050_e90280_d_n7;
        locals.var_qbu_dn8 = assign58050_e90280_d_n8;
        locals.var_qbu_dn9 = assign58050_e90280_d_n9;
        locals.var_qbu_dn10 = assign58050_e90280_d_n10;
        locals.var_qbu_dn13 = assign58050_e90280_d_n13;
        locals.var_qbu_rv = 0.0;

        let (assign58060_e90287, assign58060_e90287_d_n0, assign58060_e90287_d_n2, assign58060_e90287_d_n4, assign58060_e90287_d_n5, assign58060_e90287_d_n6, assign58060_e90287_d_n7, assign58060_e90287_d_n8, assign58060_e90287_d_n9, assign58060_e90287_d_n10, assign58060_e90287_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign58060_e90287;
        locals.var_qiu_dn0 = assign58060_e90287_d_n0;
        locals.var_qiu_dn2 = assign58060_e90287_d_n2;
        locals.var_qiu_dn4 = assign58060_e90287_d_n4;
        locals.var_qiu_dn5 = assign58060_e90287_d_n5;
        locals.var_qiu_dn6 = assign58060_e90287_d_n6;
        locals.var_qiu_dn7 = assign58060_e90287_d_n7;
        locals.var_qiu_dn8 = assign58060_e90287_d_n8;
        locals.var_qiu_dn9 = assign58060_e90287_d_n9;
        locals.var_qiu_dn10 = assign58060_e90287_d_n10;
        locals.var_qiu_dn13 = assign58060_e90287_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign58070_e90294, assign58070_e90294_d_n0, assign58070_e90294_d_n2, assign58070_e90294_d_n4, assign58070_e90294_d_n5, assign58070_e90294_d_n6, assign58070_e90294_d_n7, assign58070_e90294_d_n8, assign58070_e90294_d_n9, assign58070_e90294_d_n10, assign58070_e90294_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign58070_e90294;
        locals.var_qdrat_dn0 = assign58070_e90294_d_n0;
        locals.var_qdrat_dn2 = assign58070_e90294_d_n2;
        locals.var_qdrat_dn4 = assign58070_e90294_d_n4;
        locals.var_qdrat_dn5 = assign58070_e90294_d_n5;
        locals.var_qdrat_dn6 = assign58070_e90294_d_n6;
        locals.var_qdrat_dn7 = assign58070_e90294_d_n7;
        locals.var_qdrat_dn8 = assign58070_e90294_d_n8;
        locals.var_qdrat_dn9 = assign58070_e90294_d_n9;
        locals.var_qdrat_dn10 = assign58070_e90294_d_n10;
        locals.var_qdrat_dn13 = assign58070_e90294_d_n13;
        locals.var_qdrat_rv = 0.0;

        let (assign58080_e90301, assign58080_e90301_d_n0, assign58080_e90301_d_n2, assign58080_e90301_d_n4, assign58080_e90301_d_n5, assign58080_e90301_d_n6, assign58080_e90301_d_n7, assign58080_e90301_d_n8, assign58080_e90301_d_n9, assign58080_e90301_d_n10, assign58080_e90301_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign58080_e90301;
        locals.var_lred_dn0 = assign58080_e90301_d_n0;
        locals.var_lred_dn2 = assign58080_e90301_d_n2;
        locals.var_lred_dn4 = assign58080_e90301_d_n4;
        locals.var_lred_dn5 = assign58080_e90301_d_n5;
        locals.var_lred_dn6 = assign58080_e90301_d_n6;
        locals.var_lred_dn7 = assign58080_e90301_d_n7;
        locals.var_lred_dn8 = assign58080_e90301_d_n8;
        locals.var_lred_dn9 = assign58080_e90301_d_n9;
        locals.var_lred_dn10 = assign58080_e90301_d_n10;
        locals.var_lred_dn13 = assign58080_e90301_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign58090_e90308, assign58090_e90308_d_n0, assign58090_e90308_d_n2, assign58090_e90308_d_n4, assign58090_e90308_d_n5, assign58090_e90308_d_n6, assign58090_e90308_d_n7, assign58090_e90308_d_n8, assign58090_e90308_d_n9, assign58090_e90308_d_n10, assign58090_e90308_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign58090_e90308;
        locals.var_ids_dn0 = assign58090_e90308_d_n0;
        locals.var_ids_dn2 = assign58090_e90308_d_n2;
        locals.var_ids_dn4 = assign58090_e90308_d_n4;
        locals.var_ids_dn5 = assign58090_e90308_d_n5;
        locals.var_ids_dn6 = assign58090_e90308_d_n6;
        locals.var_ids_dn7 = assign58090_e90308_d_n7;
        locals.var_ids_dn8 = assign58090_e90308_d_n8;
        locals.var_ids_dn9 = assign58090_e90308_d_n9;
        locals.var_ids_dn10 = assign58090_e90308_d_n10;
        locals.var_ids_dn13 = assign58090_e90308_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign58100_e90315, assign58100_e90315_d_n0, assign58100_e90315_d_n2, assign58100_e90315_d_n4, assign58100_e90315_d_n5, assign58100_e90315_d_n6, assign58100_e90315_d_n7, assign58100_e90315_d_n8, assign58100_e90315_d_n9, assign58100_e90315_d_n10, assign58100_e90315_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn4, locals.var_vgvt_dn5, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn8, locals.var_vgvt_dn9, locals.var_vgvt_dn10, locals.var_vgvt_dn13,)
    }
};
        locals.var_vgvt = assign58100_e90315;
        locals.var_vgvt_dn0 = assign58100_e90315_d_n0;
        locals.var_vgvt_dn2 = assign58100_e90315_d_n2;
        locals.var_vgvt_dn4 = assign58100_e90315_d_n4;
        locals.var_vgvt_dn5 = assign58100_e90315_d_n5;
        locals.var_vgvt_dn6 = assign58100_e90315_d_n6;
        locals.var_vgvt_dn7 = assign58100_e90315_d_n7;
        locals.var_vgvt_dn8 = assign58100_e90315_d_n8;
        locals.var_vgvt_dn9 = assign58100_e90315_d_n9;
        locals.var_vgvt_dn10 = assign58100_e90315_d_n10;
        locals.var_vgvt_dn13 = assign58100_e90315_d_n13;
        locals.var_vgvt_rv = 0.0;

        let (assign58110_e90322,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58110_e90322;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58120_e90329,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1428 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign58120_e90329;
        locals.var_end_of_part_1_rv = 0.0;

        let assign58130_e90332: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1430 = assign58130_e90332;
        locals.var_guard1430_rv = 0.0;

        let (assign58140_e90353, assign58140_e90353_d_n0, assign58140_e90353_d_n2, assign58140_e90353_d_n4, assign58140_e90353_d_n5, assign58140_e90353_d_n6, assign58140_e90353_d_n7, assign58140_e90353_d_n8, assign58140_e90353_d_n9, assign58140_e90353_d_n10, assign58140_e90353_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58140_e90342: f64 = (locals.var_vgp - locals.var_vbscl__blk435);
        let assign58140_e90343: f64 = (locals.var_beta * assign58140_e90342);
        let assign58140_e90345: f64 = (assign58140_e90343 - 1.0);
        let assign58140_e90346: f64 = (4.0 * assign58140_e90345);
        let assign58140_e90349: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign58140_e90350: f64 = (assign58140_e90346 / assign58140_e90349);
        let assign58140_e90351: f64 = (1.0 + assign58140_e90350);
        (assign58140_e90351, ((((4.0 * ((locals.var_beta_dn0 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk435_dn0)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn2 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk435_dn2)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn4 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk435_dn4)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn5 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk435_dn5)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn6 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk435_dn6)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn7 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk435_dn7)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn8 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk435_dn8)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn9 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk435_dn9)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn10 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk435_dn10)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign58140_e90349 * assign58140_e90349)), ((((4.0 * ((locals.var_beta_dn13 * assign58140_e90342) + (locals.var_beta * (locals.var_vgp_dn13 - locals.var_vbscl__blk435_dn13)))) * assign58140_e90349) - (assign58140_e90346 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign58140_e90349 * assign58140_e90349)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58140_e90353;
        locals.var_tx_dn0 = assign58140_e90353_d_n0;
        locals.var_tx_dn2 = assign58140_e90353_d_n2;
        locals.var_tx_dn4 = assign58140_e90353_d_n4;
        locals.var_tx_dn5 = assign58140_e90353_d_n5;
        locals.var_tx_dn6 = assign58140_e90353_d_n6;
        locals.var_tx_dn7 = assign58140_e90353_d_n7;
        locals.var_tx_dn8 = assign58140_e90353_d_n8;
        locals.var_tx_dn9 = assign58140_e90353_d_n9;
        locals.var_tx_dn10 = assign58140_e90353_d_n10;
        locals.var_tx_dn13 = assign58140_e90353_d_n13;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_205(
        locals: &mut StampLocals,
    ) {
        let (assign58150_e90369, assign58150_e90369_d_n0, assign58150_e90369_d_n2, assign58150_e90369_d_n4, assign58150_e90369_d_n5, assign58150_e90369_d_n6, assign58150_e90369_d_n7, assign58150_e90369_d_n8, assign58150_e90369_d_n9, assign58150_e90369_d_n10, assign58150_e90369_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58150_e90361: f64 = (10.0 * 2.220446049250313e-16);
        let (assign58150_e90367, assign58150_e90367_d_n0, assign58150_e90367_d_n2, assign58150_e90367_d_n4, assign58150_e90367_d_n5, assign58150_e90367_d_n6, assign58150_e90367_d_n7, assign58150_e90367_d_n8, assign58150_e90367_d_n9, assign58150_e90367_d_n10, assign58150_e90367_d_n13,) = {
            if (locals.var_tx >= assign58150_e90361) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
            } else {
                let assign58150_e90366: f64 = (10.0 * 2.220446049250313e-16);
                (assign58150_e90366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign58150_e90367, assign58150_e90367_d_n0, assign58150_e90367_d_n2, assign58150_e90367_d_n4, assign58150_e90367_d_n5, assign58150_e90367_d_n6, assign58150_e90367_d_n7, assign58150_e90367_d_n8, assign58150_e90367_d_n9, assign58150_e90367_d_n10, assign58150_e90367_d_n13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58150_e90369;
        locals.var_tx_dn0 = assign58150_e90369_d_n0;
        locals.var_tx_dn2 = assign58150_e90369_d_n2;
        locals.var_tx_dn4 = assign58150_e90369_d_n4;
        locals.var_tx_dn5 = assign58150_e90369_d_n5;
        locals.var_tx_dn6 = assign58150_e90369_d_n6;
        locals.var_tx_dn7 = assign58150_e90369_d_n7;
        locals.var_tx_dn8 = assign58150_e90369_d_n8;
        locals.var_tx_dn9 = assign58150_e90369_d_n9;
        locals.var_tx_dn10 = assign58150_e90369_d_n10;
        locals.var_tx_dn13 = assign58150_e90369_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign58160_e90387, assign58160_e90387_d_n0, assign58160_e90387_d_n2, assign58160_e90387_d_n4, assign58160_e90387_d_n5, assign58160_e90387_d_n6, assign58160_e90387_d_n7, assign58160_e90387_d_n8, assign58160_e90387_d_n9, assign58160_e90387_d_n10, assign58160_e90387_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58160_e90377: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign58160_e90379: f64 = (assign58160_e90377 * 0.5);
        let assign58160_e90382: f64 = (locals.var_tx).sqrt();
        let assign58160_e90383: f64 = (1.0 - assign58160_e90382);
        let assign58160_e90384: f64 = (assign58160_e90379 * assign58160_e90383);
        let assign58160_e90385: f64 = (locals.var_vgp + assign58160_e90384);
        (assign58160_e90385, (locals.var_vgp_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn0 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn2 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn4 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn5 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn6 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn7 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn8 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn9 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn10 / (2.0 * assign58160_e90382)))))), (locals.var_vgp_dn13 + (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) * 0.5) * assign58160_e90383) + (assign58160_e90379 * (-(locals.var_tx_dn13 / (2.0 * assign58160_e90382)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign58160_e90387;
        locals.var_ps0_inia_dn0 = assign58160_e90387_d_n0;
        locals.var_ps0_inia_dn2 = assign58160_e90387_d_n2;
        locals.var_ps0_inia_dn4 = assign58160_e90387_d_n4;
        locals.var_ps0_inia_dn5 = assign58160_e90387_d_n5;
        locals.var_ps0_inia_dn6 = assign58160_e90387_d_n6;
        locals.var_ps0_inia_dn7 = assign58160_e90387_d_n7;
        locals.var_ps0_inia_dn8 = assign58160_e90387_d_n8;
        locals.var_ps0_inia_dn9 = assign58160_e90387_d_n9;
        locals.var_ps0_inia_dn10 = assign58160_e90387_d_n10;
        locals.var_ps0_inia_dn13 = assign58160_e90387_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let assign58170_e90390: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1431 = assign58170_e90390;
        locals.var_guard1431_rv = 0.0;

        let (assign58180_e90403, assign58180_e90403_d_n0, assign58180_e90403_d_n2, assign58180_e90403_d_n4, assign58180_e90403_d_n5, assign58180_e90403_d_n6, assign58180_e90403_d_n7, assign58180_e90403_d_n8, assign58180_e90403_d_n9, assign58180_e90403_d_n10, assign58180_e90403_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) {
        let assign58180_e90400: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk435);
        let assign58180_e90401: f64 = (locals.var_beta * assign58180_e90400);
        (assign58180_e90401, ((locals.var_beta_dn0 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk435_dn0))), ((locals.var_beta_dn2 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk435_dn2))), ((locals.var_beta_dn4 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk435_dn4))), ((locals.var_beta_dn5 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk435_dn5))), ((locals.var_beta_dn6 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk435_dn6))), ((locals.var_beta_dn7 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk435_dn7))), ((locals.var_beta_dn8 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk435_dn8))), ((locals.var_beta_dn9 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk435_dn9))), ((locals.var_beta_dn10 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk435_dn10))), ((locals.var_beta_dn13 * assign58180_e90400) + (locals.var_beta * (locals.var_ps0_inia_dn13 - locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign58180_e90403;
        locals.var_chi_dn0 = assign58180_e90403_d_n0;
        locals.var_chi_dn2 = assign58180_e90403_d_n2;
        locals.var_chi_dn4 = assign58180_e90403_d_n4;
        locals.var_chi_dn5 = assign58180_e90403_d_n5;
        locals.var_chi_dn6 = assign58180_e90403_d_n6;
        locals.var_chi_dn7 = assign58180_e90403_d_n7;
        locals.var_chi_dn8 = assign58180_e90403_d_n8;
        locals.var_chi_dn9 = assign58180_e90403_d_n9;
        locals.var_chi_dn10 = assign58180_e90403_d_n10;
        locals.var_chi_dn13 = assign58180_e90403_d_n13;
        locals.var_chi_rv = 0.0;

        let assign58190_e90406: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1432 = assign58190_e90406;
        locals.var_guard1432_rv = 0.0;

        let (assign58200_e90421, assign58200_e90421_d_n0, assign58200_e90421_d_n2, assign58200_e90421_d_n4, assign58200_e90421_d_n5, assign58200_e90421_d_n6, assign58200_e90421_d_n7, assign58200_e90421_d_n8, assign58200_e90421_d_n9, assign58200_e90421_d_n10, assign58200_e90421_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58200_e90418: f64 = (locals.var_vgp - locals.var_vbscl__blk435);
        let assign58200_e90419: f64 = (locals.var_beta * assign58200_e90418);
        (assign58200_e90419, ((locals.var_beta_dn0 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk435_dn0))), ((locals.var_beta_dn2 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk435_dn2))), ((locals.var_beta_dn4 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk435_dn4))), ((locals.var_beta_dn5 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk435_dn5))), ((locals.var_beta_dn6 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk435_dn6))), ((locals.var_beta_dn7 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk435_dn7))), ((locals.var_beta_dn8 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk435_dn8))), ((locals.var_beta_dn9 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk435_dn9))), ((locals.var_beta_dn10 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk435_dn10))), ((locals.var_beta_dn13 * assign58200_e90418) + (locals.var_beta * (locals.var_vgp_dn13 - locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign58200_e90421;
        locals.var_ty_dn0 = assign58200_e90421_d_n0;
        locals.var_ty_dn2 = assign58200_e90421_d_n2;
        locals.var_ty_dn4 = assign58200_e90421_d_n4;
        locals.var_ty_dn5 = assign58200_e90421_d_n5;
        locals.var_ty_dn6 = assign58200_e90421_d_n6;
        locals.var_ty_dn7 = assign58200_e90421_d_n7;
        locals.var_ty_dn8 = assign58200_e90421_d_n8;
        locals.var_ty_dn9 = assign58200_e90421_d_n9;
        locals.var_ty_dn10 = assign58200_e90421_d_n10;
        locals.var_ty_dn13 = assign58200_e90421_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign58210_e90440, assign58210_e90440_d_n0, assign58210_e90440_d_n2, assign58210_e90440_d_n4, assign58210_e90440_d_n5, assign58210_e90440_d_n6, assign58210_e90440_d_n7, assign58210_e90440_d_n8, assign58210_e90440_d_n9, assign58210_e90440_d_n10, assign58210_e90440_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58210_e90433: f64 = (1.414213562373095 / 108.0);
        let assign58210_e90435: f64 = (assign58210_e90433 * locals.var_beta);
        let assign58210_e90437: f64 = (assign58210_e90435 * locals.var_fac1);
        let assign58210_e90438: f64 = (1.0 / assign58210_e90437);
        (assign58210_e90438, (-((((assign58210_e90433 * locals.var_beta_dn0) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn0)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn2) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn2)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn4) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn4)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn5) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn5)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn6) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn6)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn7) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn7)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn8) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn8)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn9) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn9)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn10) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn10)) / (assign58210_e90437 * assign58210_e90437))), (-((((assign58210_e90433 * locals.var_beta_dn13) * locals.var_fac1) + (assign58210_e90435 * locals.var_fac1_dn13)) / (assign58210_e90437 * assign58210_e90437))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58210_e90440;
        locals.var_t1_dn0 = assign58210_e90440_d_n0;
        locals.var_t1_dn2 = assign58210_e90440_d_n2;
        locals.var_t1_dn4 = assign58210_e90440_d_n4;
        locals.var_t1_dn5 = assign58210_e90440_d_n5;
        locals.var_t1_dn6 = assign58210_e90440_d_n6;
        locals.var_t1_dn7 = assign58210_e90440_d_n7;
        locals.var_t1_dn8 = assign58210_e90440_d_n8;
        locals.var_t1_dn9 = assign58210_e90440_d_n9;
        locals.var_t1_dn10 = assign58210_e90440_d_n10;
        locals.var_t1_dn13 = assign58210_e90440_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58220_e90455, assign58220_e90455_d_n0, assign58220_e90455_d_n2, assign58220_e90455_d_n4, assign58220_e90455_d_n5, assign58220_e90455_d_n6, assign58220_e90455_d_n7, assign58220_e90455_d_n8, assign58220_e90455_d_n9, assign58220_e90455_d_n10, assign58220_e90455_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58220_e90452: f64 = (3.0 * locals.var_t1);
        let assign58220_e90453: f64 = (81.0 + assign58220_e90452);
        (assign58220_e90453, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign58220_e90455;
        locals.var_t2_dn0 = assign58220_e90455_d_n0;
        locals.var_t2_dn2 = assign58220_e90455_d_n2;
        locals.var_t2_dn4 = assign58220_e90455_d_n4;
        locals.var_t2_dn5 = assign58220_e90455_d_n5;
        locals.var_t2_dn6 = assign58220_e90455_d_n6;
        locals.var_t2_dn7 = assign58220_e90455_d_n7;
        locals.var_t2_dn8 = assign58220_e90455_d_n8;
        locals.var_t2_dn9 = assign58220_e90455_d_n9;
        locals.var_t2_dn10 = assign58220_e90455_d_n10;
        locals.var_t2_dn13 = assign58220_e90455_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign58230_e90477, assign58230_e90477_d_n0, assign58230_e90477_d_n2, assign58230_e90477_d_n4, assign58230_e90477_d_n5, assign58230_e90477_d_n6, assign58230_e90477_d_n7, assign58230_e90477_d_n8, assign58230_e90477_d_n9, assign58230_e90477_d_n10, assign58230_e90477_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58230_e90465: f64 = (-2916.0);
        let assign58230_e90468: f64 = (81.0 * locals.var_t1);
        let assign58230_e90469: f64 = (assign58230_e90465 - assign58230_e90468);
        let assign58230_e90472: f64 = (27.0 * locals.var_t1);
        let assign58230_e90474: f64 = (assign58230_e90472 * locals.var_ty);
        let assign58230_e90475: f64 = (assign58230_e90469 + assign58230_e90474);
        (assign58230_e90475, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign58230_e90472 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign58230_e90477;
        locals.var_t3_dn0 = assign58230_e90477_d_n0;
        locals.var_t3_dn2 = assign58230_e90477_d_n2;
        locals.var_t3_dn4 = assign58230_e90477_d_n4;
        locals.var_t3_dn5 = assign58230_e90477_d_n5;
        locals.var_t3_dn6 = assign58230_e90477_d_n6;
        locals.var_t3_dn7 = assign58230_e90477_d_n7;
        locals.var_t3_dn8 = assign58230_e90477_d_n8;
        locals.var_t3_dn9 = assign58230_e90477_d_n9;
        locals.var_t3_dn10 = assign58230_e90477_d_n10;
        locals.var_t3_dn13 = assign58230_e90477_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign58240_e90500, assign58240_e90500_d_n0, assign58240_e90500_d_n2, assign58240_e90500_d_n4, assign58240_e90500_d_n5, assign58240_e90500_d_n6, assign58240_e90500_d_n7, assign58240_e90500_d_n8, assign58240_e90500_d_n9, assign58240_e90500_d_n10, assign58240_e90500_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58240_e90490: f64 = (54.0 + locals.var_t1);
        let assign58240_e90491: f64 = (81.0 * assign58240_e90490);
        let assign58240_e90492: f64 = (1458.0 - assign58240_e90491);
        let assign58240_e90495: f64 = (27.0 * locals.var_t1);
        let assign58240_e90497: f64 = (assign58240_e90495 * locals.var_ty);
        let assign58240_e90498: f64 = (assign58240_e90492 + assign58240_e90497);
        (assign58240_e90498, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn13)) + (((27.0 * locals.var_t1_dn13) * locals.var_ty) + (assign58240_e90495 * locals.var_ty_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign58240_e90500;
        locals.var_t4_dn0 = assign58240_e90500_d_n0;
        locals.var_t4_dn2 = assign58240_e90500_d_n2;
        locals.var_t4_dn4 = assign58240_e90500_d_n4;
        locals.var_t4_dn5 = assign58240_e90500_d_n5;
        locals.var_t4_dn6 = assign58240_e90500_d_n6;
        locals.var_t4_dn7 = assign58240_e90500_d_n7;
        locals.var_t4_dn8 = assign58240_e90500_d_n8;
        locals.var_t4_dn9 = assign58240_e90500_d_n9;
        locals.var_t4_dn10 = assign58240_e90500_d_n10;
        locals.var_t4_dn13 = assign58240_e90500_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign58250_e90513, assign58250_e90513_d_n0, assign58250_e90513_d_n2, assign58250_e90513_d_n4, assign58250_e90513_d_n5, assign58250_e90513_d_n6, assign58250_e90513_d_n7, assign58250_e90513_d_n8, assign58250_e90513_d_n9, assign58250_e90513_d_n10, assign58250_e90513_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58250_e90511: f64 = (locals.var_t4 * locals.var_t4);
        (assign58250_e90511, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign58250_e90513;
        locals.var_t4_dn0 = assign58250_e90513_d_n0;
        locals.var_t4_dn2 = assign58250_e90513_d_n2;
        locals.var_t4_dn4 = assign58250_e90513_d_n4;
        locals.var_t4_dn5 = assign58250_e90513_d_n5;
        locals.var_t4_dn6 = assign58250_e90513_d_n6;
        locals.var_t4_dn7 = assign58250_e90513_d_n7;
        locals.var_t4_dn8 = assign58250_e90513_d_n8;
        locals.var_t4_dn9 = assign58250_e90513_d_n9;
        locals.var_t4_dn10 = assign58250_e90513_d_n10;
        locals.var_t4_dn13 = assign58250_e90513_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign58260_e90553, assign58260_e90553_d_n0, assign58260_e90553_d_n2, assign58260_e90553_d_n4, assign58260_e90553_d_n5, assign58260_e90553_d_n6, assign58260_e90553_d_n7, assign58260_e90553_d_n8, assign58260_e90553_d_n9, assign58260_e90553_d_n10, assign58260_e90553_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58260_e90525: f64 = (4.0 * locals.var_t2);
        let assign58260_e90527: f64 = (assign58260_e90525 * locals.var_t2);
        let assign58260_e90529: f64 = (assign58260_e90527 * locals.var_t2);
        let assign58260_e90531: f64 = (assign58260_e90529 + locals.var_t4);
        let assign58260_e90532: f64 = (assign58260_e90531).sqrt();
        let assign58260_e90533: f64 = (locals.var_t3 + assign58260_e90532);
        let (assign58260_e90551, assign58260_e90551_d_n0, assign58260_e90551_d_n2, assign58260_e90551_d_n4, assign58260_e90551_d_n5, assign58260_e90551_d_n6, assign58260_e90551_d_n7, assign58260_e90551_d_n8, assign58260_e90551_d_n9, assign58260_e90551_d_n10, assign58260_e90551_d_n13,) = {
            if (assign58260_e90533 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58260_e90540: f64 = (4.0 * locals.var_t2);
                let assign58260_e90542: f64 = (assign58260_e90540 * locals.var_t2);
                let assign58260_e90544: f64 = (assign58260_e90542 * locals.var_t2);
                let assign58260_e90546: f64 = (assign58260_e90544 + locals.var_t4);
                let assign58260_e90547: f64 = (assign58260_e90546).sqrt();
                let assign58260_e90548: f64 = (locals.var_t3 + assign58260_e90547);
                let assign58260_e90550: f64 = (assign58260_e90548).powf(0.3333333333333333);
                (assign58260_e90550, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn0)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn0)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn2)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn2)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn4)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn4)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn5)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn5)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn6)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn6)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn7)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn7)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn8)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn8)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn9)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn9)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn10)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn10)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58260_e90547))) / assign58260_e90548))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58260_e90548).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn13)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign58260_e90547))))) } } else { (assign58260_e90550 * (0.3333333333333333 * ((locals.var_t3_dn13 + (((((((4.0 * locals.var_t2_dn13) * locals.var_t2) + (assign58260_e90540 * locals.var_t2_dn13)) * locals.var_t2) + (assign58260_e90542 * locals.var_t2_dn13)) + locals.var_t4_dn13) / (2.0 * assign58260_e90547))) / assign58260_e90548))) },)
            }
        };
        (assign58260_e90551, assign58260_e90551_d_n0, assign58260_e90551_d_n2, assign58260_e90551_d_n4, assign58260_e90551_d_n5, assign58260_e90551_d_n6, assign58260_e90551_d_n7, assign58260_e90551_d_n8, assign58260_e90551_d_n9, assign58260_e90551_d_n10, assign58260_e90551_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign58260_e90553;
        locals.var_t5_dn0 = assign58260_e90553_d_n0;
        locals.var_t5_dn2 = assign58260_e90553_d_n2;
        locals.var_t5_dn4 = assign58260_e90553_d_n4;
        locals.var_t5_dn5 = assign58260_e90553_d_n5;
        locals.var_t5_dn6 = assign58260_e90553_d_n6;
        locals.var_t5_dn7 = assign58260_e90553_d_n7;
        locals.var_t5_dn8 = assign58260_e90553_d_n8;
        locals.var_t5_dn9 = assign58260_e90553_d_n9;
        locals.var_t5_dn10 = assign58260_e90553_d_n10;
        locals.var_t5_dn13 = assign58260_e90553_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign58270_e90580, assign58270_e90580_d_n0, assign58270_e90580_d_n2, assign58270_e90580_d_n4, assign58270_e90580_d_n5, assign58270_e90580_d_n6, assign58270_e90580_d_n7, assign58270_e90580_d_n8, assign58270_e90580_d_n9, assign58270_e90580_d_n10, assign58270_e90580_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58270_e90565: f64 = (1.259921049894873 * locals.var_t2);
        let assign58270_e90568: f64 = (3.0 * locals.var_t5);
        let assign58270_e90569: f64 = (assign58270_e90565 / assign58270_e90568);
        let assign58270_e90570: f64 = (3.0 - assign58270_e90569);
        let assign58270_e90574: f64 = (3.0 * 1.259921049894873);
        let assign58270_e90575: f64 = (1.0 / assign58270_e90574);
        let assign58270_e90577: f64 = (assign58270_e90575 * locals.var_t5);
        let assign58270_e90578: f64 = (assign58270_e90570 + assign58270_e90577);
        (assign58270_e90578, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn0))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn2))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn4))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn5))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn6))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn7))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn8))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn9))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn10))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn13) * assign58270_e90568) - (assign58270_e90565 * (3.0 * locals.var_t5_dn13))) / (assign58270_e90568 * assign58270_e90568))) + (assign58270_e90575 * locals.var_t5_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58270_e90580;
        locals.var_tx_dn0 = assign58270_e90580_d_n0;
        locals.var_tx_dn2 = assign58270_e90580_d_n2;
        locals.var_tx_dn4 = assign58270_e90580_d_n4;
        locals.var_tx_dn5 = assign58270_e90580_d_n5;
        locals.var_tx_dn6 = assign58270_e90580_d_n6;
        locals.var_tx_dn7 = assign58270_e90580_d_n7;
        locals.var_tx_dn8 = assign58270_e90580_d_n8;
        locals.var_tx_dn9 = assign58270_e90580_d_n9;
        locals.var_tx_dn10 = assign58270_e90580_d_n10;
        locals.var_tx_dn13 = assign58270_e90580_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign58280_e90595, assign58280_e90595_d_n0, assign58280_e90595_d_n2, assign58280_e90595_d_n4, assign58280_e90595_d_n5, assign58280_e90595_d_n6, assign58280_e90595_d_n7, assign58280_e90595_d_n8, assign58280_e90595_d_n9, assign58280_e90595_d_n10, assign58280_e90595_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        let assign58280_e90591: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign58280_e90593: f64 = (assign58280_e90591 + locals.var_vbscl__blk435);
        (assign58280_e90593, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk435_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk435_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk435_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk435_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk435_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk435_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk435_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk435_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk435_dn10), (((locals.var_tx_dn13 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn13)) + locals.var_vbscl__blk435_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign58280_e90595;
        locals.var_ps0_inia_dn0 = assign58280_e90595_d_n0;
        locals.var_ps0_inia_dn2 = assign58280_e90595_d_n2;
        locals.var_ps0_inia_dn4 = assign58280_e90595_d_n4;
        locals.var_ps0_inia_dn5 = assign58280_e90595_d_n5;
        locals.var_ps0_inia_dn6 = assign58280_e90595_d_n6;
        locals.var_ps0_inia_dn7 = assign58280_e90595_d_n7;
        locals.var_ps0_inia_dn8 = assign58280_e90595_d_n8;
        locals.var_ps0_inia_dn9 = assign58280_e90595_d_n9;
        locals.var_ps0_inia_dn10 = assign58280_e90595_d_n10;
        locals.var_ps0_inia_dn13 = assign58280_e90595_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign58290_e90606, assign58290_e90606_d_n0, assign58290_e90606_d_n2, assign58290_e90606_d_n4, assign58290_e90606_d_n5, assign58290_e90606_d_n6, assign58290_e90606_d_n7, assign58290_e90606_d_n8, assign58290_e90606_d_n9, assign58290_e90606_d_n10, assign58290_e90606_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign58290_e90606;
        locals.var_ps0_ini_dn0 = assign58290_e90606_d_n0;
        locals.var_ps0_ini_dn2 = assign58290_e90606_d_n2;
        locals.var_ps0_ini_dn4 = assign58290_e90606_d_n4;
        locals.var_ps0_ini_dn5 = assign58290_e90606_d_n5;
        locals.var_ps0_ini_dn6 = assign58290_e90606_d_n6;
        locals.var_ps0_ini_dn7 = assign58290_e90606_d_n7;
        locals.var_ps0_ini_dn8 = assign58290_e90606_d_n8;
        locals.var_ps0_ini_dn9 = assign58290_e90606_d_n9;
        locals.var_ps0_ini_dn10 = assign58290_e90606_d_n10;
        locals.var_ps0_ini_dn13 = assign58290_e90606_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let assign58300_e90609: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard1433 = assign58300_e90609;
        locals.var_guard1433_rv = 0.0;

        let (assign58310_e90623, assign58310_e90623_d_n0, assign58310_e90623_d_n2, assign58310_e90623_d_n4, assign58310_e90623_d_n5, assign58310_e90623_d_n6, assign58310_e90623_d_n7, assign58310_e90623_d_n8, assign58310_e90623_d_n9, assign58310_e90623_d_n10, assign58310_e90623_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign58310_e90623;
        locals.var_ps0_ini_dn0 = assign58310_e90623_d_n0;
        locals.var_ps0_ini_dn2 = assign58310_e90623_d_n2;
        locals.var_ps0_ini_dn4 = assign58310_e90623_d_n4;
        locals.var_ps0_ini_dn5 = assign58310_e90623_d_n5;
        locals.var_ps0_ini_dn6 = assign58310_e90623_d_n6;
        locals.var_ps0_ini_dn7 = assign58310_e90623_d_n7;
        locals.var_ps0_ini_dn8 = assign58310_e90623_d_n8;
        locals.var_ps0_ini_dn9 = assign58310_e90623_d_n9;
        locals.var_ps0_ini_dn10 = assign58310_e90623_d_n10;
        locals.var_ps0_ini_dn13 = assign58310_e90623_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58320_e90642, assign58320_e90642_d_n0, assign58320_e90642_d_n2, assign58320_e90642_d_n4, assign58320_e90642_d_n5, assign58320_e90642_d_n6, assign58320_e90642_d_n7, assign58320_e90642_d_n8, assign58320_e90642_d_n9, assign58320_e90642_d_n10, assign58320_e90642_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58320_e90638: f64 = (1.0 / locals.var_cnst1);
        let assign58320_e90640: f64 = (assign58320_e90638 / locals.var_cnstcoxi);
        (assign58320_e90640, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn13 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58320_e90638 * locals.var_cnstcoxi_dn13)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58320_e90642;
        locals.var_t1_dn0 = assign58320_e90642_d_n0;
        locals.var_t1_dn2 = assign58320_e90642_d_n2;
        locals.var_t1_dn4 = assign58320_e90642_d_n4;
        locals.var_t1_dn5 = assign58320_e90642_d_n5;
        locals.var_t1_dn6 = assign58320_e90642_d_n6;
        locals.var_t1_dn7 = assign58320_e90642_d_n7;
        locals.var_t1_dn8 = assign58320_e90642_d_n8;
        locals.var_t1_dn9 = assign58320_e90642_d_n9;
        locals.var_t1_dn10 = assign58320_e90642_d_n10;
        locals.var_t1_dn13 = assign58320_e90642_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58330_e90661, assign58330_e90661_d_n0, assign58330_e90661_d_n2, assign58330_e90661_d_n4, assign58330_e90661_d_n5, assign58330_e90661_d_n6, assign58330_e90661_d_n7, assign58330_e90661_d_n8, assign58330_e90661_d_n9, assign58330_e90661_d_n10, assign58330_e90661_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58330_e90657: f64 = (locals.var_t1 * locals.var_vgp);
        let assign58330_e90659: f64 = (assign58330_e90657 * locals.var_vgp);
        (assign58330_e90659, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn4)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn5)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn8)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn9)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn10)), ((((locals.var_t1_dn13 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn13)) * locals.var_vgp) + (assign58330_e90657 * locals.var_vgp_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign58330_e90661;
        locals.var_t2_dn0 = assign58330_e90661_d_n0;
        locals.var_t2_dn2 = assign58330_e90661_d_n2;
        locals.var_t2_dn4 = assign58330_e90661_d_n4;
        locals.var_t2_dn5 = assign58330_e90661_d_n5;
        locals.var_t2_dn6 = assign58330_e90661_d_n6;
        locals.var_t2_dn7 = assign58330_e90661_d_n7;
        locals.var_t2_dn8 = assign58330_e90661_d_n8;
        locals.var_t2_dn9 = assign58330_e90661_d_n9;
        locals.var_t2_dn10 = assign58330_e90661_d_n10;
        locals.var_t2_dn13 = assign58330_e90661_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign58340_e90680, assign58340_e90680_d_n0, assign58340_e90680_d_n2, assign58340_e90680_d_n4, assign58340_e90680_d_n5, assign58340_e90680_d_n6, assign58340_e90680_d_n7, assign58340_e90680_d_n8, assign58340_e90680_d_n9, assign58340_e90680_d_n10, assign58340_e90680_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58340_e90677: f64 = (2.0 / locals.var_vgp);
        let assign58340_e90678: f64 = (locals.var_beta + assign58340_e90677);
        (assign58340_e90678, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn13 + (-((2.0 * locals.var_vgp_dn13) / (locals.var_vgp * locals.var_vgp)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign58340_e90680;
        locals.var_t3_dn0 = assign58340_e90680_d_n0;
        locals.var_t3_dn2 = assign58340_e90680_d_n2;
        locals.var_t3_dn4 = assign58340_e90680_d_n4;
        locals.var_t3_dn5 = assign58340_e90680_d_n5;
        locals.var_t3_dn6 = assign58340_e90680_d_n6;
        locals.var_t3_dn7 = assign58340_e90680_d_n7;
        locals.var_t3_dn8 = assign58340_e90680_d_n8;
        locals.var_t3_dn9 = assign58340_e90680_d_n9;
        locals.var_t3_dn10 = assign58340_e90680_d_n10;
        locals.var_t3_dn13 = assign58340_e90680_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign58350_e90698, assign58350_e90698_d_n0, assign58350_e90698_d_n2, assign58350_e90698_d_n4, assign58350_e90698_d_n5, assign58350_e90698_d_n6, assign58350_e90698_d_n7, assign58350_e90698_d_n8, assign58350_e90698_d_n9, assign58350_e90698_d_n10, assign58350_e90698_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58350_e90694: f64 = (locals.var_t2).ln();
        let assign58350_e90696: f64 = (assign58350_e90694 / locals.var_t3);
        (assign58350_e90696, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn13 / locals.var_t2) * locals.var_t3) - (assign58350_e90694 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn13,)
    }
};
        locals.var_ps0_inib = assign58350_e90698;
        locals.var_ps0_inib_dn0 = assign58350_e90698_d_n0;
        locals.var_ps0_inib_dn2 = assign58350_e90698_d_n2;
        locals.var_ps0_inib_dn4 = assign58350_e90698_d_n4;
        locals.var_ps0_inib_dn5 = assign58350_e90698_d_n5;
        locals.var_ps0_inib_dn6 = assign58350_e90698_d_n6;
        locals.var_ps0_inib_dn7 = assign58350_e90698_d_n7;
        locals.var_ps0_inib_dn8 = assign58350_e90698_d_n8;
        locals.var_ps0_inib_dn9 = assign58350_e90698_d_n9;
        locals.var_ps0_inib_dn10 = assign58350_e90698_d_n10;
        locals.var_ps0_inib_dn13 = assign58350_e90698_d_n13;
        locals.var_ps0_inib_rv = 0.0;

        let (assign58360_e90717, assign58360_e90717_d_n0, assign58360_e90717_d_n2, assign58360_e90717_d_n4, assign58360_e90717_d_n5, assign58360_e90717_d_n6, assign58360_e90717_d_n7, assign58360_e90717_d_n8, assign58360_e90717_d_n9, assign58360_e90717_d_n10, assign58360_e90717_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58360_e90713: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign58360_e90715: f64 = (assign58360_e90713 - 0.0008);
        (assign58360_e90715, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn13 - locals.var_ps0_inia_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign58360_e90717;
        locals.var_tmf1_dn0 = assign58360_e90717_d_n0;
        locals.var_tmf1_dn2 = assign58360_e90717_d_n2;
        locals.var_tmf1_dn4 = assign58360_e90717_d_n4;
        locals.var_tmf1_dn5 = assign58360_e90717_d_n5;
        locals.var_tmf1_dn6 = assign58360_e90717_d_n6;
        locals.var_tmf1_dn7 = assign58360_e90717_d_n7;
        locals.var_tmf1_dn8 = assign58360_e90717_d_n8;
        locals.var_tmf1_dn9 = assign58360_e90717_d_n9;
        locals.var_tmf1_dn10 = assign58360_e90717_d_n10;
        locals.var_tmf1_dn13 = assign58360_e90717_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign58370_e90736, assign58370_e90736_d_n0, assign58370_e90736_d_n2, assign58370_e90736_d_n4, assign58370_e90736_d_n5, assign58370_e90736_d_n6, assign58370_e90736_d_n7, assign58370_e90736_d_n8, assign58370_e90736_d_n9, assign58370_e90736_d_n10, assign58370_e90736_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58370_e90732: f64 = (4.0 * locals.var_ps0_inib);
        let assign58370_e90734: f64 = (assign58370_e90732 * 0.0008);
        (assign58370_e90734, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn13) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign58370_e90736;
        locals.var_tmf2_dn0 = assign58370_e90736_d_n0;
        locals.var_tmf2_dn2 = assign58370_e90736_d_n2;
        locals.var_tmf2_dn4 = assign58370_e90736_d_n4;
        locals.var_tmf2_dn5 = assign58370_e90736_d_n5;
        locals.var_tmf2_dn6 = assign58370_e90736_d_n6;
        locals.var_tmf2_dn7 = assign58370_e90736_d_n7;
        locals.var_tmf2_dn8 = assign58370_e90736_d_n8;
        locals.var_tmf2_dn9 = assign58370_e90736_d_n9;
        locals.var_tmf2_dn10 = assign58370_e90736_d_n10;
        locals.var_tmf2_dn13 = assign58370_e90736_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_206(
        locals: &mut StampLocals,
    ) {
        let (assign58380_e90757, assign58380_e90757_d_n0, assign58380_e90757_d_n2, assign58380_e90757_d_n4, assign58380_e90757_d_n5, assign58380_e90757_d_n6, assign58380_e90757_d_n7, assign58380_e90757_d_n8, assign58380_e90757_d_n9, assign58380_e90757_d_n10, assign58380_e90757_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let (assign58380_e90755, assign58380_e90755_d_n0, assign58380_e90755_d_n2, assign58380_e90755_d_n4, assign58380_e90755_d_n5, assign58380_e90755_d_n6, assign58380_e90755_d_n7, assign58380_e90755_d_n8, assign58380_e90755_d_n9, assign58380_e90755_d_n10, assign58380_e90755_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign58380_e90754: f64 = (-locals.var_tmf2);
                (assign58380_e90754, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign58380_e90755, assign58380_e90755_d_n0, assign58380_e90755_d_n2, assign58380_e90755_d_n4, assign58380_e90755_d_n5, assign58380_e90755_d_n6, assign58380_e90755_d_n7, assign58380_e90755_d_n8, assign58380_e90755_d_n9, assign58380_e90755_d_n10, assign58380_e90755_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign58380_e90757;
        locals.var_tmf2_dn0 = assign58380_e90757_d_n0;
        locals.var_tmf2_dn2 = assign58380_e90757_d_n2;
        locals.var_tmf2_dn4 = assign58380_e90757_d_n4;
        locals.var_tmf2_dn5 = assign58380_e90757_d_n5;
        locals.var_tmf2_dn6 = assign58380_e90757_d_n6;
        locals.var_tmf2_dn7 = assign58380_e90757_d_n7;
        locals.var_tmf2_dn8 = assign58380_e90757_d_n8;
        locals.var_tmf2_dn9 = assign58380_e90757_d_n9;
        locals.var_tmf2_dn10 = assign58380_e90757_d_n10;
        locals.var_tmf2_dn13 = assign58380_e90757_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign58390_e90777, assign58390_e90777_d_n0, assign58390_e90777_d_n2, assign58390_e90777_d_n4, assign58390_e90777_d_n5, assign58390_e90777_d_n6, assign58390_e90777_d_n7, assign58390_e90777_d_n8, assign58390_e90777_d_n9, assign58390_e90777_d_n10, assign58390_e90777_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58390_e90772: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign58390_e90774: f64 = (assign58390_e90772 + locals.var_tmf2);
        let assign58390_e90775: f64 = (assign58390_e90774).sqrt();
        (assign58390_e90775, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign58390_e90775)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign58390_e90775)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign58390_e90777;
        locals.var_tmf2_dn0 = assign58390_e90777_d_n0;
        locals.var_tmf2_dn2 = assign58390_e90777_d_n2;
        locals.var_tmf2_dn4 = assign58390_e90777_d_n4;
        locals.var_tmf2_dn5 = assign58390_e90777_d_n5;
        locals.var_tmf2_dn6 = assign58390_e90777_d_n6;
        locals.var_tmf2_dn7 = assign58390_e90777_d_n7;
        locals.var_tmf2_dn8 = assign58390_e90777_d_n8;
        locals.var_tmf2_dn9 = assign58390_e90777_d_n9;
        locals.var_tmf2_dn10 = assign58390_e90777_d_n10;
        locals.var_tmf2_dn13 = assign58390_e90777_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign58400_e90798, assign58400_e90798_d_n0, assign58400_e90798_d_n2, assign58400_e90798_d_n4, assign58400_e90798_d_n5, assign58400_e90798_d_n6, assign58400_e90798_d_n7, assign58400_e90798_d_n8, assign58400_e90798_d_n9, assign58400_e90798_d_n10, assign58400_e90798_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58400_e90794: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign58400_e90795: f64 = (1.0 + assign58400_e90794);
        let assign58400_e90796: f64 = (0.5 * assign58400_e90795);
        (assign58400_e90796, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58400_e90798;
        locals.var_t1_dn0 = assign58400_e90798_d_n0;
        locals.var_t1_dn2 = assign58400_e90798_d_n2;
        locals.var_t1_dn4 = assign58400_e90798_d_n4;
        locals.var_t1_dn5 = assign58400_e90798_d_n5;
        locals.var_t1_dn6 = assign58400_e90798_d_n6;
        locals.var_t1_dn7 = assign58400_e90798_d_n7;
        locals.var_t1_dn8 = assign58400_e90798_d_n8;
        locals.var_t1_dn9 = assign58400_e90798_d_n9;
        locals.var_t1_dn10 = assign58400_e90798_d_n10;
        locals.var_t1_dn13 = assign58400_e90798_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58410_e90819, assign58410_e90819_d_n0, assign58410_e90819_d_n2, assign58410_e90819_d_n4, assign58410_e90819_d_n5, assign58410_e90819_d_n6, assign58410_e90819_d_n7, assign58410_e90819_d_n8, assign58410_e90819_d_n9, assign58410_e90819_d_n10, assign58410_e90819_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1433 == 0.0)) {
        let assign58410_e90815: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign58410_e90816: f64 = (0.5 * assign58410_e90815);
        let assign58410_e90817: f64 = (locals.var_ps0_inib - assign58410_e90816);
        (assign58410_e90817, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign58410_e90819;
        locals.var_ps0_ini_dn0 = assign58410_e90819_d_n0;
        locals.var_ps0_ini_dn2 = assign58410_e90819_d_n2;
        locals.var_ps0_ini_dn4 = assign58410_e90819_d_n4;
        locals.var_ps0_ini_dn5 = assign58410_e90819_d_n5;
        locals.var_ps0_ini_dn6 = assign58410_e90819_d_n6;
        locals.var_ps0_ini_dn7 = assign58410_e90819_d_n7;
        locals.var_ps0_ini_dn8 = assign58410_e90819_d_n8;
        locals.var_ps0_ini_dn9 = assign58410_e90819_d_n9;
        locals.var_ps0_ini_dn10 = assign58410_e90819_d_n10;
        locals.var_ps0_ini_dn13 = assign58410_e90819_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58420_e90830, assign58420_e90830_d_n0, assign58420_e90830_d_n2, assign58420_e90830_d_n4, assign58420_e90830_d_n5, assign58420_e90830_d_n6, assign58420_e90830_d_n7, assign58420_e90830_d_n8, assign58420_e90830_d_n9, assign58420_e90830_d_n10, assign58420_e90830_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58420_e90827: f64 = (1e-12 / 2.0);
        let assign58420_e90828: f64 = (locals.var_vbscl__blk435 + assign58420_e90827);
        (assign58420_e90828, locals.var_vbscl__blk435_dn0, locals.var_vbscl__blk435_dn2, locals.var_vbscl__blk435_dn4, locals.var_vbscl__blk435_dn5, locals.var_vbscl__blk435_dn6, locals.var_vbscl__blk435_dn7, locals.var_vbscl__blk435_dn8, locals.var_vbscl__blk435_dn9, locals.var_vbscl__blk435_dn10, locals.var_vbscl__blk435_dn13,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58420_e90830;
        locals.var_tx_dn0 = assign58420_e90830_d_n0;
        locals.var_tx_dn2 = assign58420_e90830_d_n2;
        locals.var_tx_dn4 = assign58420_e90830_d_n4;
        locals.var_tx_dn5 = assign58420_e90830_d_n5;
        locals.var_tx_dn6 = assign58420_e90830_d_n6;
        locals.var_tx_dn7 = assign58420_e90830_d_n7;
        locals.var_tx_dn8 = assign58420_e90830_d_n8;
        locals.var_tx_dn9 = assign58420_e90830_d_n9;
        locals.var_tx_dn10 = assign58420_e90830_d_n10;
        locals.var_tx_dn13 = assign58420_e90830_d_n13;
        locals.var_tx_rv = 0.0;

        let assign58430_e90833: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1434 = assign58430_e90833;
        locals.var_guard1434_rv = 0.0;

        let (assign58440_e90842, assign58440_e90842_d_n0, assign58440_e90842_d_n2, assign58440_e90842_d_n4, assign58440_e90842_d_n5, assign58440_e90842_d_n6, assign58440_e90842_d_n7, assign58440_e90842_d_n8, assign58440_e90842_d_n9, assign58440_e90842_d_n10, assign58440_e90842_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    }
};
        locals.var_ps0_ini = assign58440_e90842;
        locals.var_ps0_ini_dn0 = assign58440_e90842_d_n0;
        locals.var_ps0_ini_dn2 = assign58440_e90842_d_n2;
        locals.var_ps0_ini_dn4 = assign58440_e90842_d_n4;
        locals.var_ps0_ini_dn5 = assign58440_e90842_d_n5;
        locals.var_ps0_ini_dn6 = assign58440_e90842_d_n6;
        locals.var_ps0_ini_dn7 = assign58440_e90842_d_n7;
        locals.var_ps0_ini_dn8 = assign58440_e90842_d_n8;
        locals.var_ps0_ini_dn9 = assign58440_e90842_d_n9;
        locals.var_ps0_ini_dn10 = assign58440_e90842_d_n10;
        locals.var_ps0_ini_dn13 = assign58440_e90842_d_n13;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58450_e90849, assign58450_e90849_d_n0, assign58450_e90849_d_n2, assign58450_e90849_d_n4, assign58450_e90849_d_n5, assign58450_e90849_d_n6, assign58450_e90849_d_n7, assign58450_e90849_d_n8, assign58450_e90849_d_n9, assign58450_e90849_d_n10, assign58450_e90849_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn13,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    }
};
        locals.var_ps0 = assign58450_e90849;
        locals.var_ps0_dn0 = assign58450_e90849_d_n0;
        locals.var_ps0_dn2 = assign58450_e90849_d_n2;
        locals.var_ps0_dn4 = assign58450_e90849_d_n4;
        locals.var_ps0_dn5 = assign58450_e90849_d_n5;
        locals.var_ps0_dn6 = assign58450_e90849_d_n6;
        locals.var_ps0_dn7 = assign58450_e90849_d_n7;
        locals.var_ps0_dn8 = assign58450_e90849_d_n8;
        locals.var_ps0_dn9 = assign58450_e90849_d_n9;
        locals.var_ps0_dn10 = assign58450_e90849_d_n10;
        locals.var_ps0_dn13 = assign58450_e90849_d_n13;
        locals.var_ps0_rv = 0.0;

        let (assign58460_e90856, assign58460_e90856_d_n0, assign58460_e90856_d_n2, assign58460_e90856_d_n4, assign58460_e90856_d_n5, assign58460_e90856_d_n6, assign58460_e90856_d_n7, assign58460_e90856_d_n8, assign58460_e90856_d_n9, assign58460_e90856_d_n10, assign58460_e90856_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn4, locals.var_psl_lim_dn5, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn8, locals.var_psl_lim_dn9, locals.var_psl_lim_dn10, locals.var_psl_lim_dn13,)
    }
};
        locals.var_psl_lim = assign58460_e90856;
        locals.var_psl_lim_dn0 = assign58460_e90856_d_n0;
        locals.var_psl_lim_dn2 = assign58460_e90856_d_n2;
        locals.var_psl_lim_dn4 = assign58460_e90856_d_n4;
        locals.var_psl_lim_dn5 = assign58460_e90856_d_n5;
        locals.var_psl_lim_dn6 = assign58460_e90856_d_n6;
        locals.var_psl_lim_dn7 = assign58460_e90856_d_n7;
        locals.var_psl_lim_dn8 = assign58460_e90856_d_n8;
        locals.var_psl_lim_dn9 = assign58460_e90856_d_n9;
        locals.var_psl_lim_dn10 = assign58460_e90856_d_n10;
        locals.var_psl_lim_dn13 = assign58460_e90856_d_n13;
        locals.var_psl_lim_rv = 0.0;

        let (assign58470_e90866, assign58470_e90866_d_n0, assign58470_e90866_d_n2, assign58470_e90866_d_n4, assign58470_e90866_d_n5, assign58470_e90866_d_n6, assign58470_e90866_d_n7, assign58470_e90866_d_n8, assign58470_e90866_d_n9, assign58470_e90866_d_n10, assign58470_e90866_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58470_e90863: f64 = (locals.var_beta * locals.var_vbscl__blk435);
        let assign58470_e90864: f64 = (assign58470_e90863).exp();
        (assign58470_e90864, (assign58470_e90864 * ((locals.var_beta_dn0 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn0))), (assign58470_e90864 * ((locals.var_beta_dn2 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn2))), (assign58470_e90864 * ((locals.var_beta_dn4 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn4))), (assign58470_e90864 * ((locals.var_beta_dn5 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn5))), (assign58470_e90864 * ((locals.var_beta_dn6 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn6))), (assign58470_e90864 * ((locals.var_beta_dn7 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn7))), (assign58470_e90864 * ((locals.var_beta_dn8 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn8))), (assign58470_e90864 * ((locals.var_beta_dn9 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn9))), (assign58470_e90864 * ((locals.var_beta_dn10 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn10))), (assign58470_e90864 * ((locals.var_beta_dn13 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign58470_e90866;
        locals.var_exp_bvbs_dn0 = assign58470_e90866_d_n0;
        locals.var_exp_bvbs_dn2 = assign58470_e90866_d_n2;
        locals.var_exp_bvbs_dn4 = assign58470_e90866_d_n4;
        locals.var_exp_bvbs_dn5 = assign58470_e90866_d_n5;
        locals.var_exp_bvbs_dn6 = assign58470_e90866_d_n6;
        locals.var_exp_bvbs_dn7 = assign58470_e90866_d_n7;
        locals.var_exp_bvbs_dn8 = assign58470_e90866_d_n8;
        locals.var_exp_bvbs_dn9 = assign58470_e90866_d_n9;
        locals.var_exp_bvbs_dn10 = assign58470_e90866_d_n10;
        locals.var_exp_bvbs_dn13 = assign58470_e90866_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign58480_e90875, assign58480_e90875_d_n0, assign58480_e90875_d_n2, assign58480_e90875_d_n4, assign58480_e90875_d_n5, assign58480_e90875_d_n6, assign58480_e90875_d_n7, assign58480_e90875_d_n8, assign58480_e90875_d_n9, assign58480_e90875_d_n10, assign58480_e90875_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58480_e90873: f64 = (locals.var_cnst1 * locals.var_exp_bvbs);
        (assign58480_e90873, ((locals.var_cnst1_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign58480_e90875;
        locals.var_cfs1_dn0 = assign58480_e90875_d_n0;
        locals.var_cfs1_dn2 = assign58480_e90875_d_n2;
        locals.var_cfs1_dn4 = assign58480_e90875_d_n4;
        locals.var_cfs1_dn5 = assign58480_e90875_d_n5;
        locals.var_cfs1_dn6 = assign58480_e90875_d_n6;
        locals.var_cfs1_dn7 = assign58480_e90875_d_n7;
        locals.var_cfs1_dn8 = assign58480_e90875_d_n8;
        locals.var_cfs1_dn9 = assign58480_e90875_d_n9;
        locals.var_cfs1_dn10 = assign58480_e90875_d_n10;
        locals.var_cfs1_dn13 = assign58480_e90875_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign58490_e90882,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign58490_e90882;
        locals.var_flg_conv_rv = 0.0;

        let (assign58500_e90889,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign58500_e90889;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_207(
        locals: &mut StampLocals,
    ) {
        let mut assign58510_loop_guard: usize = 0;
        while {
            let assign58510_cond_e90897: f64 = (locals.var_lp_s0_max + 1.0);
            let assign58510_cond_e90899: f64 = if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_lp_s0 <= assign58510_cond_e90897)) { 1.0 } else { 0.0 };
            assign58510_cond_e90899 != 0.0
        } {
            assign58510_loop_guard += 1;
            assert!(assign58510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign58510_body1_e90919, assign58510_body1_e90919_d_n0, assign58510_body1_e90919_d_n2, assign58510_body1_e90919_d_n4, assign58510_body1_e90919_d_n5, assign58510_body1_e90919_d_n6, assign58510_body1_e90919_d_n7, assign58510_body1_e90919_d_n8, assign58510_body1_e90919_d_n9, assign58510_body1_e90919_d_n10, assign58510_body1_e90919_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58510_body1_e90916: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign58510_body1_e90917: f64 = (locals.var_beta * assign58510_body1_e90916);
        (assign58510_body1_e90917, ((locals.var_beta_dn0 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0))), ((locals.var_beta_dn2 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2))), ((locals.var_beta_dn4 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4))), ((locals.var_beta_dn5 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5))), ((locals.var_beta_dn6 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6))), ((locals.var_beta_dn7 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7))), ((locals.var_beta_dn8 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8))), ((locals.var_beta_dn9 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9))), ((locals.var_beta_dn10 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10))), ((locals.var_beta_dn13 * assign58510_body1_e90916) + (locals.var_beta * (locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign58510_body1_e90919;
            locals.var_chi_dn0 = assign58510_body1_e90919_d_n0;
            locals.var_chi_dn2 = assign58510_body1_e90919_d_n2;
            locals.var_chi_dn4 = assign58510_body1_e90919_d_n4;
            locals.var_chi_dn5 = assign58510_body1_e90919_d_n5;
            locals.var_chi_dn6 = assign58510_body1_e90919_d_n6;
            locals.var_chi_dn7 = assign58510_body1_e90919_d_n7;
            locals.var_chi_dn8 = assign58510_body1_e90919_d_n8;
            locals.var_chi_dn9 = assign58510_body1_e90919_d_n9;
            locals.var_chi_dn10 = assign58510_body1_e90919_d_n10;
            locals.var_chi_dn13 = assign58510_body1_e90919_d_n13;
            locals.var_chi_rv = 0.0;
            let assign58510_body2_e90922: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1435 = assign58510_body2_e90922;
            locals.var_guard1435_rv = 0.0;
            let (assign58510_body3_e90946, assign58510_body3_e90946_d_n0, assign58510_body3_e90946_d_n2, assign58510_body3_e90946_d_n4, assign58510_body3_e90946_d_n5, assign58510_body3_e90946_d_n6, assign58510_body3_e90946_d_n7, assign58510_body3_e90946_d_n8, assign58510_body3_e90946_d_n9, assign58510_body3_e90946_d_n10, assign58510_body3_e90946_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body3_e90931: f64 = (locals.var_chi * locals.var_chi);
        let assign58510_body3_e90933: f64 = (assign58510_body3_e90931 * locals.var_chi);
        let assign58510_body3_e90937: f64 = (-0.07053654284009761);
        let assign58510_body3_e90940: f64 = (locals.var_chi * 0.006115288895133179);
        let assign58510_body3_e90941: f64 = (assign58510_body3_e90937 + assign58510_body3_e90940);
        let assign58510_body3_e90942: f64 = (locals.var_chi * assign58510_body3_e90941);
        let assign58510_body3_e90943: f64 = (0.29693154855771 + assign58510_body3_e90942);
        let assign58510_body3_e90944: f64 = (assign58510_body3_e90933 * assign58510_body3_e90943);
        (assign58510_body3_e90944, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn0)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn0 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn2)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn2 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn4)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn4 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn5)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn5 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn6)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn6 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn7)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn7 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn8)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn8 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn9)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn9 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn10)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn10 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) * locals.var_chi) + (assign58510_body3_e90931 * locals.var_chi_dn13)) * assign58510_body3_e90943) + (assign58510_body3_e90933 * ((locals.var_chi_dn13 * assign58510_body3_e90941) + (locals.var_chi * (locals.var_chi_dn13 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn13,)
    }
};
            locals.var_fi = assign58510_body3_e90946;
            locals.var_fi_dn0 = assign58510_body3_e90946_d_n0;
            locals.var_fi_dn2 = assign58510_body3_e90946_d_n2;
            locals.var_fi_dn4 = assign58510_body3_e90946_d_n4;
            locals.var_fi_dn5 = assign58510_body3_e90946_d_n5;
            locals.var_fi_dn6 = assign58510_body3_e90946_d_n6;
            locals.var_fi_dn7 = assign58510_body3_e90946_d_n7;
            locals.var_fi_dn8 = assign58510_body3_e90946_d_n8;
            locals.var_fi_dn9 = assign58510_body3_e90946_d_n9;
            locals.var_fi_dn10 = assign58510_body3_e90946_d_n10;
            locals.var_fi_dn13 = assign58510_body3_e90946_d_n13;
            locals.var_fi_rv = 0.0;
            let (assign58510_body4_e90974, assign58510_body4_e90974_d_n0, assign58510_body4_e90974_d_n2, assign58510_body4_e90974_d_n4, assign58510_body4_e90974_d_n5, assign58510_body4_e90974_d_n6, assign58510_body4_e90974_d_n7, assign58510_body4_e90974_d_n8, assign58510_body4_e90974_d_n9, assign58510_body4_e90974_d_n10, assign58510_body4_e90974_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body4_e90955: f64 = (locals.var_chi * locals.var_chi);
        let assign58510_body4_e90958: f64 = (3.0 * 0.29693154855771);
        let assign58510_body4_e90962: f64 = (-0.07053654284009761);
        let assign58510_body4_e90963: f64 = (4.0 * assign58510_body4_e90962);
        let assign58510_body4_e90966: f64 = (locals.var_chi * 5.0);
        let assign58510_body4_e90968: f64 = (assign58510_body4_e90966 * 0.006115288895133179);
        let assign58510_body4_e90969: f64 = (assign58510_body4_e90963 + assign58510_body4_e90968);
        let assign58510_body4_e90970: f64 = (locals.var_chi * assign58510_body4_e90969);
        let assign58510_body4_e90971: f64 = (assign58510_body4_e90958 + assign58510_body4_e90970);
        let assign58510_body4_e90972: f64 = (assign58510_body4_e90955 * assign58510_body4_e90971);
        (assign58510_body4_e90972, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn0 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn2 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn4 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn5 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn6 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn7 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn8 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn9 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn10 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) * assign58510_body4_e90971) + (assign58510_body4_e90955 * ((locals.var_chi_dn13 * assign58510_body4_e90969) + (locals.var_chi * ((locals.var_chi_dn13 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn13,)
    }
};
            locals.var_fi_dchi = assign58510_body4_e90974;
            locals.var_fi_dchi_dn0 = assign58510_body4_e90974_d_n0;
            locals.var_fi_dchi_dn2 = assign58510_body4_e90974_d_n2;
            locals.var_fi_dchi_dn4 = assign58510_body4_e90974_d_n4;
            locals.var_fi_dchi_dn5 = assign58510_body4_e90974_d_n5;
            locals.var_fi_dchi_dn6 = assign58510_body4_e90974_d_n6;
            locals.var_fi_dchi_dn7 = assign58510_body4_e90974_d_n7;
            locals.var_fi_dchi_dn8 = assign58510_body4_e90974_d_n8;
            locals.var_fi_dchi_dn9 = assign58510_body4_e90974_d_n9;
            locals.var_fi_dchi_dn10 = assign58510_body4_e90974_d_n10;
            locals.var_fi_dchi_dn13 = assign58510_body4_e90974_d_n13;
            locals.var_fi_dchi_rv = 0.0;
            let (assign58510_body5_e90987, assign58510_body5_e90987_d_n0, assign58510_body5_e90987_d_n2, assign58510_body5_e90987_d_n4, assign58510_body5_e90987_d_n5, assign58510_body5_e90987_d_n6, assign58510_body5_e90987_d_n7, assign58510_body5_e90987_d_n8, assign58510_body5_e90987_d_n9, assign58510_body5_e90987_d_n10, assign58510_body5_e90987_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body5_e90983: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign58510_body5_e90985: f64 = (assign58510_body5_e90983 * locals.var_fi);
        (assign58510_body5_e90985, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn13 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn13)) * locals.var_fi) + (assign58510_body5_e90983 * locals.var_fi_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign58510_body5_e90987;
            locals.var_fs01_dn0 = assign58510_body5_e90987_d_n0;
            locals.var_fs01_dn2 = assign58510_body5_e90987_d_n2;
            locals.var_fs01_dn4 = assign58510_body5_e90987_d_n4;
            locals.var_fs01_dn5 = assign58510_body5_e90987_d_n5;
            locals.var_fs01_dn6 = assign58510_body5_e90987_d_n6;
            locals.var_fs01_dn7 = assign58510_body5_e90987_d_n7;
            locals.var_fs01_dn8 = assign58510_body5_e90987_d_n8;
            locals.var_fs01_dn9 = assign58510_body5_e90987_d_n9;
            locals.var_fs01_dn10 = assign58510_body5_e90987_d_n10;
            locals.var_fs01_dn13 = assign58510_body5_e90987_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign58510_body6_e91004, assign58510_body6_e91004_d_n0, assign58510_body6_e91004_d_n2, assign58510_body6_e91004_d_n4, assign58510_body6_e91004_d_n5, assign58510_body6_e91004_d_n6, assign58510_body6_e91004_d_n7, assign58510_body6_e91004_d_n8, assign58510_body6_e91004_d_n9, assign58510_body6_e91004_d_n10, assign58510_body6_e91004_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body6_e90996: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58510_body6_e90998: f64 = (assign58510_body6_e90996 * 2.0);
        let assign58510_body6_e91000: f64 = (assign58510_body6_e90998 * locals.var_fi);
        let assign58510_body6_e91002: f64 = (assign58510_body6_e91000 * locals.var_fi_dchi);
        (assign58510_body6_e91002, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * 2.0) * locals.var_fi) + (assign58510_body6_e90998 * locals.var_fi_dn13)) * locals.var_fi_dchi) + (assign58510_body6_e91000 * locals.var_fi_dchi_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign58510_body6_e91004;
            locals.var_fs01_dps0_dn0 = assign58510_body6_e91004_d_n0;
            locals.var_fs01_dps0_dn2 = assign58510_body6_e91004_d_n2;
            locals.var_fs01_dps0_dn4 = assign58510_body6_e91004_d_n4;
            locals.var_fs01_dps0_dn5 = assign58510_body6_e91004_d_n5;
            locals.var_fs01_dps0_dn6 = assign58510_body6_e91004_d_n6;
            locals.var_fs01_dps0_dn7 = assign58510_body6_e91004_d_n7;
            locals.var_fs01_dps0_dn8 = assign58510_body6_e91004_d_n8;
            locals.var_fs01_dps0_dn9 = assign58510_body6_e91004_d_n9;
            locals.var_fs01_dps0_dn10 = assign58510_body6_e91004_d_n10;
            locals.var_fs01_dps0_dn13 = assign58510_body6_e91004_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58510_body7_e91033, assign58510_body7_e91033_d_n0, assign58510_body7_e91033_d_n2, assign58510_body7_e91033_d_n4, assign58510_body7_e91033_d_n5, assign58510_body7_e91033_d_n6, assign58510_body7_e91033_d_n7, assign58510_body7_e91033_d_n8, assign58510_body7_e91033_d_n9, assign58510_body7_e91033_d_n10, assign58510_body7_e91033_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body7_e91015: f64 = (-0.117851130197758);
        let assign58510_body7_e91020: f64 = (-0.00163730162779191);
        let assign58510_body7_e91023: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign58510_body7_e91024: f64 = (assign58510_body7_e91020 + assign58510_body7_e91023);
        let assign58510_body7_e91025: f64 = (locals.var_chi * assign58510_body7_e91024);
        let assign58510_body7_e91026: f64 = (0.0178800506338833 + assign58510_body7_e91025);
        let assign58510_body7_e91027: f64 = (locals.var_chi * assign58510_body7_e91026);
        let assign58510_body7_e91028: f64 = (assign58510_body7_e91015 + assign58510_body7_e91027);
        let assign58510_body7_e91029: f64 = (locals.var_chi * assign58510_body7_e91028);
        let assign58510_body7_e91030: f64 = (0.707106781186548 + assign58510_body7_e91029);
        let assign58510_body7_e91031: f64 = (locals.var_chi * assign58510_body7_e91030);
        (assign58510_body7_e91031, ((locals.var_chi_dn0 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn0 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn0 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn0 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn2 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn2 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn2 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn4 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn4 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn4 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn5 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn5 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn5 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn6 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn6 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn6 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn7 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn7 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn7 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn8 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn8 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn8 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn9 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn9 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn9 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn10 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn10 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn10 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn13 * assign58510_body7_e91030) + (locals.var_chi * ((locals.var_chi_dn13 * assign58510_body7_e91028) + (locals.var_chi * ((locals.var_chi_dn13 * assign58510_body7_e91026) + (locals.var_chi * ((locals.var_chi_dn13 * assign58510_body7_e91024) + (locals.var_chi * (locals.var_chi_dn13 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign58510_body7_e91033;
            locals.var_fb_dn0 = assign58510_body7_e91033_d_n0;
            locals.var_fb_dn2 = assign58510_body7_e91033_d_n2;
            locals.var_fb_dn4 = assign58510_body7_e91033_d_n4;
            locals.var_fb_dn5 = assign58510_body7_e91033_d_n5;
            locals.var_fb_dn6 = assign58510_body7_e91033_d_n6;
            locals.var_fb_dn7 = assign58510_body7_e91033_d_n7;
            locals.var_fb_dn8 = assign58510_body7_e91033_d_n8;
            locals.var_fb_dn9 = assign58510_body7_e91033_d_n9;
            locals.var_fb_dn10 = assign58510_body7_e91033_d_n10;
            locals.var_fb_dn13 = assign58510_body7_e91033_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign58510_body8_e91068, assign58510_body8_e91068_d_n0, assign58510_body8_e91068_d_n2, assign58510_body8_e91068_d_n4, assign58510_body8_e91068_d_n5, assign58510_body8_e91068_d_n6, assign58510_body8_e91068_d_n7, assign58510_body8_e91068_d_n8, assign58510_body8_e91068_d_n9, assign58510_body8_e91068_d_n10, assign58510_body8_e91068_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body8_e91044: f64 = (-0.117851130197758);
        let assign58510_body8_e91045: f64 = (2.0 * assign58510_body8_e91044);
        let assign58510_body8_e91049: f64 = (3.0 * 0.0178800506338833);
        let assign58510_body8_e91053: f64 = (-0.00163730162779191);
        let assign58510_body8_e91054: f64 = (4.0 * assign58510_body8_e91053);
        let assign58510_body8_e91057: f64 = (locals.var_chi * 5.0);
        let assign58510_body8_e91059: f64 = (assign58510_body8_e91057 * 6.36964918866352e-5);
        let assign58510_body8_e91060: f64 = (assign58510_body8_e91054 + assign58510_body8_e91059);
        let assign58510_body8_e91061: f64 = (locals.var_chi * assign58510_body8_e91060);
        let assign58510_body8_e91062: f64 = (assign58510_body8_e91049 + assign58510_body8_e91061);
        let assign58510_body8_e91063: f64 = (locals.var_chi * assign58510_body8_e91062);
        let assign58510_body8_e91064: f64 = (assign58510_body8_e91045 + assign58510_body8_e91063);
        let assign58510_body8_e91065: f64 = (locals.var_chi * assign58510_body8_e91064);
        let assign58510_body8_e91066: f64 = (0.707106781186548 + assign58510_body8_e91065);
        (assign58510_body8_e91066, ((locals.var_chi_dn0 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn0 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn0 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn2 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn2 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn4 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn4 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn5 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn5 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn6 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn6 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn7 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn7 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn8 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn8 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn9 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn9 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn10 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn10 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn13 * assign58510_body8_e91064) + (locals.var_chi * ((locals.var_chi_dn13 * assign58510_body8_e91062) + (locals.var_chi * ((locals.var_chi_dn13 * assign58510_body8_e91060) + (locals.var_chi * ((locals.var_chi_dn13 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn13,)
    }
};
            locals.var_fb_dchi = assign58510_body8_e91068;
            locals.var_fb_dchi_dn0 = assign58510_body8_e91068_d_n0;
            locals.var_fb_dchi_dn2 = assign58510_body8_e91068_d_n2;
            locals.var_fb_dchi_dn4 = assign58510_body8_e91068_d_n4;
            locals.var_fb_dchi_dn5 = assign58510_body8_e91068_d_n5;
            locals.var_fb_dchi_dn6 = assign58510_body8_e91068_d_n6;
            locals.var_fb_dchi_dn7 = assign58510_body8_e91068_d_n7;
            locals.var_fb_dchi_dn8 = assign58510_body8_e91068_d_n8;
            locals.var_fb_dchi_dn9 = assign58510_body8_e91068_d_n9;
            locals.var_fb_dchi_dn10 = assign58510_body8_e91068_d_n10;
            locals.var_fb_dchi_dn13 = assign58510_body8_e91068_d_n13;
            locals.var_fb_dchi_rv = 0.0;
            let (assign58510_body9_e91082, assign58510_body9_e91082_d_n0, assign58510_body9_e91082_d_n2, assign58510_body9_e91082_d_n4, assign58510_body9_e91082_d_n5, assign58510_body9_e91082_d_n6, assign58510_body9_e91082_d_n7, assign58510_body9_e91082_d_n8, assign58510_body9_e91082_d_n9, assign58510_body9_e91082_d_n10, assign58510_body9_e91082_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body9_e91077: f64 = (locals.var_fb * locals.var_fb);
        let assign58510_body9_e91079: f64 = (assign58510_body9_e91077 + locals.var_fs01);
        let assign58510_body9_e91080: f64 = (assign58510_body9_e91079).sqrt();
        (assign58510_body9_e91080, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign58510_body9_e91080)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign58510_body9_e91080)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign58510_body9_e91082;
            locals.var_fs02_dn0 = assign58510_body9_e91082_d_n0;
            locals.var_fs02_dn2 = assign58510_body9_e91082_d_n2;
            locals.var_fs02_dn4 = assign58510_body9_e91082_d_n4;
            locals.var_fs02_dn5 = assign58510_body9_e91082_d_n5;
            locals.var_fs02_dn6 = assign58510_body9_e91082_d_n6;
            locals.var_fs02_dn7 = assign58510_body9_e91082_d_n7;
            locals.var_fs02_dn8 = assign58510_body9_e91082_d_n8;
            locals.var_fs02_dn9 = assign58510_body9_e91082_d_n9;
            locals.var_fs02_dn10 = assign58510_body9_e91082_d_n10;
            locals.var_fs02_dn13 = assign58510_body9_e91082_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign58510_body10_e91103, assign58510_body10_e91103_d_n0, assign58510_body10_e91103_d_n2, assign58510_body10_e91103_d_n4, assign58510_body10_e91103_d_n5, assign58510_body10_e91103_d_n6, assign58510_body10_e91103_d_n7, assign58510_body10_e91103_d_n8, assign58510_body10_e91103_d_n9, assign58510_body10_e91103_d_n10, assign58510_body10_e91103_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign58510_body10_e91091: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign58510_body10_e91093: f64 = (assign58510_body10_e91091 * 2.0);
        let assign58510_body10_e91095: f64 = (assign58510_body10_e91093 * locals.var_fb);
        let assign58510_body10_e91097: f64 = (assign58510_body10_e91095 + locals.var_fs01_dps0);
        let assign58510_body10_e91100: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58510_body10_e91101: f64 = (assign58510_body10_e91097 / assign58510_body10_e91100);
        (assign58510_body10_e91101, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58510_body10_e91100 * assign58510_body10_e91100)), (((((((((locals.var_beta_dn13 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn13)) * 2.0) * locals.var_fb) + (assign58510_body10_e91093 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13) * assign58510_body10_e91100) - (assign58510_body10_e91097 * (locals.var_fs02_dn13 + locals.var_fs02_dn13))) / (assign58510_body10_e91100 * assign58510_body10_e91100)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign58510_body10_e91103;
            locals.var_fs02_dps0_dn0 = assign58510_body10_e91103_d_n0;
            locals.var_fs02_dps0_dn2 = assign58510_body10_e91103_d_n2;
            locals.var_fs02_dps0_dn4 = assign58510_body10_e91103_d_n4;
            locals.var_fs02_dps0_dn5 = assign58510_body10_e91103_d_n5;
            locals.var_fs02_dps0_dn6 = assign58510_body10_e91103_d_n6;
            locals.var_fs02_dps0_dn7 = assign58510_body10_e91103_d_n7;
            locals.var_fs02_dps0_dn8 = assign58510_body10_e91103_d_n8;
            locals.var_fs02_dps0_dn9 = assign58510_body10_e91103_d_n9;
            locals.var_fs02_dps0_dn10 = assign58510_body10_e91103_d_n10;
            locals.var_fs02_dps0_dn13 = assign58510_body10_e91103_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign58510_body11_e91106: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1436 = assign58510_body11_e91106;
            locals.var_guard1436_rv = 0.0;
            let (assign58510_body12_e91119, assign58510_body12_e91119_d_n0, assign58510_body12_e91119_d_n2, assign58510_body12_e91119_d_n4, assign58510_body12_e91119_d_n5, assign58510_body12_e91119_d_n6, assign58510_body12_e91119_d_n7, assign58510_body12_e91119_d_n8, assign58510_body12_e91119_d_n9, assign58510_body12_e91119_d_n10, assign58510_body12_e91119_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58510_body12_e91117: f64 = (locals.var_chi).exp();
        (assign58510_body12_e91117, (assign58510_body12_e91117 * locals.var_chi_dn0), (assign58510_body12_e91117 * locals.var_chi_dn2), (assign58510_body12_e91117 * locals.var_chi_dn4), (assign58510_body12_e91117 * locals.var_chi_dn5), (assign58510_body12_e91117 * locals.var_chi_dn6), (assign58510_body12_e91117 * locals.var_chi_dn7), (assign58510_body12_e91117 * locals.var_chi_dn8), (assign58510_body12_e91117 * locals.var_chi_dn9), (assign58510_body12_e91117 * locals.var_chi_dn10), (assign58510_body12_e91117 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign58510_body12_e91119;
            locals.var_exp_chi_dn0 = assign58510_body12_e91119_d_n0;
            locals.var_exp_chi_dn2 = assign58510_body12_e91119_d_n2;
            locals.var_exp_chi_dn4 = assign58510_body12_e91119_d_n4;
            locals.var_exp_chi_dn5 = assign58510_body12_e91119_d_n5;
            locals.var_exp_chi_dn6 = assign58510_body12_e91119_d_n6;
            locals.var_exp_chi_dn7 = assign58510_body12_e91119_d_n7;
            locals.var_exp_chi_dn8 = assign58510_body12_e91119_d_n8;
            locals.var_exp_chi_dn9 = assign58510_body12_e91119_d_n9;
            locals.var_exp_chi_dn10 = assign58510_body12_e91119_d_n10;
            locals.var_exp_chi_dn13 = assign58510_body12_e91119_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign58510_body13_e91135, assign58510_body13_e91135_d_n0, assign58510_body13_e91135_d_n2, assign58510_body13_e91135_d_n4, assign58510_body13_e91135_d_n5, assign58510_body13_e91135_d_n6, assign58510_body13_e91135_d_n7, assign58510_body13_e91135_d_n8, assign58510_body13_e91135_d_n9, assign58510_body13_e91135_d_n10, assign58510_body13_e91135_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58510_body13_e91132: f64 = (locals.var_exp_chi - 1.0);
        let assign58510_body13_e91133: f64 = (locals.var_cfs1 * assign58510_body13_e91132);
        (assign58510_body13_e91133, ((locals.var_cfs1_dn0 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn7 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn7)), ((locals.var_cfs1_dn8 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn9 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn9)), ((locals.var_cfs1_dn10 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn13 * assign58510_body13_e91132) + (locals.var_cfs1 * locals.var_exp_chi_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign58510_body13_e91135;
            locals.var_fs01_dn0 = assign58510_body13_e91135_d_n0;
            locals.var_fs01_dn2 = assign58510_body13_e91135_d_n2;
            locals.var_fs01_dn4 = assign58510_body13_e91135_d_n4;
            locals.var_fs01_dn5 = assign58510_body13_e91135_d_n5;
            locals.var_fs01_dn6 = assign58510_body13_e91135_d_n6;
            locals.var_fs01_dn7 = assign58510_body13_e91135_d_n7;
            locals.var_fs01_dn8 = assign58510_body13_e91135_d_n8;
            locals.var_fs01_dn9 = assign58510_body13_e91135_d_n9;
            locals.var_fs01_dn10 = assign58510_body13_e91135_d_n10;
            locals.var_fs01_dn13 = assign58510_body13_e91135_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign58510_body14_e91151, assign58510_body14_e91151_d_n0, assign58510_body14_e91151_d_n2, assign58510_body14_e91151_d_n4, assign58510_body14_e91151_d_n5, assign58510_body14_e91151_d_n6, assign58510_body14_e91151_d_n7, assign58510_body14_e91151_d_n8, assign58510_body14_e91151_d_n9, assign58510_body14_e91151_d_n10, assign58510_body14_e91151_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign58510_body14_e91147: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58510_body14_e91149: f64 = (assign58510_body14_e91147 * locals.var_exp_chi);
        (assign58510_body14_e91149, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_exp_chi) + (assign58510_body14_e91147 * locals.var_exp_chi_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign58510_body14_e91151;
            locals.var_fs01_dps0_dn0 = assign58510_body14_e91151_d_n0;
            locals.var_fs01_dps0_dn2 = assign58510_body14_e91151_d_n2;
            locals.var_fs01_dps0_dn4 = assign58510_body14_e91151_d_n4;
            locals.var_fs01_dps0_dn5 = assign58510_body14_e91151_d_n5;
            locals.var_fs01_dps0_dn6 = assign58510_body14_e91151_d_n6;
            locals.var_fs01_dps0_dn7 = assign58510_body14_e91151_d_n7;
            locals.var_fs01_dps0_dn8 = assign58510_body14_e91151_d_n8;
            locals.var_fs01_dps0_dn9 = assign58510_body14_e91151_d_n9;
            locals.var_fs01_dps0_dn10 = assign58510_body14_e91151_d_n10;
            locals.var_fs01_dps0_dn13 = assign58510_body14_e91151_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58510_body15_e91167, assign58510_body15_e91167_d_n0, assign58510_body15_e91167_d_n2, assign58510_body15_e91167_d_n4, assign58510_body15_e91167_d_n5, assign58510_body15_e91167_d_n6, assign58510_body15_e91167_d_n7, assign58510_body15_e91167_d_n8, assign58510_body15_e91167_d_n9, assign58510_body15_e91167_d_n10, assign58510_body15_e91167_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 == 0.0)) {
        let assign58510_body15_e91164: f64 = (locals.var_beta * locals.var_ps0);
        let assign58510_body15_e91165: f64 = (assign58510_body15_e91164).exp();
        (assign58510_body15_e91165, (assign58510_body15_e91165 * ((locals.var_beta_dn0 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn0))), (assign58510_body15_e91165 * ((locals.var_beta_dn2 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn2))), (assign58510_body15_e91165 * ((locals.var_beta_dn4 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn4))), (assign58510_body15_e91165 * ((locals.var_beta_dn5 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn5))), (assign58510_body15_e91165 * ((locals.var_beta_dn6 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn6))), (assign58510_body15_e91165 * ((locals.var_beta_dn7 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn7))), (assign58510_body15_e91165 * ((locals.var_beta_dn8 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn8))), (assign58510_body15_e91165 * ((locals.var_beta_dn9 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn9))), (assign58510_body15_e91165 * ((locals.var_beta_dn10 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn10))), (assign58510_body15_e91165 * ((locals.var_beta_dn13 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign58510_body15_e91167;
            locals.var_exp_bps0_dn0 = assign58510_body15_e91167_d_n0;
            locals.var_exp_bps0_dn2 = assign58510_body15_e91167_d_n2;
            locals.var_exp_bps0_dn4 = assign58510_body15_e91167_d_n4;
            locals.var_exp_bps0_dn5 = assign58510_body15_e91167_d_n5;
            locals.var_exp_bps0_dn6 = assign58510_body15_e91167_d_n6;
            locals.var_exp_bps0_dn7 = assign58510_body15_e91167_d_n7;
            locals.var_exp_bps0_dn8 = assign58510_body15_e91167_d_n8;
            locals.var_exp_bps0_dn9 = assign58510_body15_e91167_d_n9;
            locals.var_exp_bps0_dn10 = assign58510_body15_e91167_d_n10;
            locals.var_exp_bps0_dn13 = assign58510_body15_e91167_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign58510_body16_e91184, assign58510_body16_e91184_d_n0, assign58510_body16_e91184_d_n2, assign58510_body16_e91184_d_n4, assign58510_body16_e91184_d_n5, assign58510_body16_e91184_d_n6, assign58510_body16_e91184_d_n7, assign58510_body16_e91184_d_n8, assign58510_body16_e91184_d_n9, assign58510_body16_e91184_d_n10, assign58510_body16_e91184_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 == 0.0)) {
        let assign58510_body16_e91181: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign58510_body16_e91182: f64 = (locals.var_cnst1 * assign58510_body16_e91181);
        (assign58510_body16_e91182, ((locals.var_cnst1_dn0 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1_dn2 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1_dn4 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1_dn5 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1_dn6 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1_dn7 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((locals.var_cnst1_dn8 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1_dn9 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((locals.var_cnst1_dn10 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1_dn13 * assign58510_body16_e91181) + (locals.var_cnst1 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign58510_body16_e91184;
            locals.var_fs01_dn0 = assign58510_body16_e91184_d_n0;
            locals.var_fs01_dn2 = assign58510_body16_e91184_d_n2;
            locals.var_fs01_dn4 = assign58510_body16_e91184_d_n4;
            locals.var_fs01_dn5 = assign58510_body16_e91184_d_n5;
            locals.var_fs01_dn6 = assign58510_body16_e91184_d_n6;
            locals.var_fs01_dn7 = assign58510_body16_e91184_d_n7;
            locals.var_fs01_dn8 = assign58510_body16_e91184_d_n8;
            locals.var_fs01_dn9 = assign58510_body16_e91184_d_n9;
            locals.var_fs01_dn10 = assign58510_body16_e91184_d_n10;
            locals.var_fs01_dn13 = assign58510_body16_e91184_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign58510_body17_e91201, assign58510_body17_e91201_d_n0, assign58510_body17_e91201_d_n2, assign58510_body17_e91201_d_n4, assign58510_body17_e91201_d_n5, assign58510_body17_e91201_d_n6, assign58510_body17_e91201_d_n7, assign58510_body17_e91201_d_n8, assign58510_body17_e91201_d_n9, assign58510_body17_e91201_d_n10, assign58510_body17_e91201_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) && (locals.var_guard1436 == 0.0)) {
        let assign58510_body17_e91197: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign58510_body17_e91199: f64 = (assign58510_body17_e91197 * locals.var_exp_bps0);
        (assign58510_body17_e91199, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn10)), ((((locals.var_cnst1_dn13 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn13)) * locals.var_exp_bps0) + (assign58510_body17_e91197 * locals.var_exp_bps0_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign58510_body17_e91201;
            locals.var_fs01_dps0_dn0 = assign58510_body17_e91201_d_n0;
            locals.var_fs01_dps0_dn2 = assign58510_body17_e91201_d_n2;
            locals.var_fs01_dps0_dn4 = assign58510_body17_e91201_d_n4;
            locals.var_fs01_dps0_dn5 = assign58510_body17_e91201_d_n5;
            locals.var_fs01_dps0_dn6 = assign58510_body17_e91201_d_n6;
            locals.var_fs01_dps0_dn7 = assign58510_body17_e91201_d_n7;
            locals.var_fs01_dps0_dn8 = assign58510_body17_e91201_d_n8;
            locals.var_fs01_dps0_dn9 = assign58510_body17_e91201_d_n9;
            locals.var_fs01_dps0_dn10 = assign58510_body17_e91201_d_n10;
            locals.var_fs01_dps0_dn13 = assign58510_body17_e91201_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58510_body18_e91216, assign58510_body18_e91216_d_n0, assign58510_body18_e91216_d_n2, assign58510_body18_e91216_d_n4, assign58510_body18_e91216_d_n5, assign58510_body18_e91216_d_n6, assign58510_body18_e91216_d_n7, assign58510_body18_e91216_d_n8, assign58510_body18_e91216_d_n9, assign58510_body18_e91216_d_n10, assign58510_body18_e91216_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58510_body18_e91211: f64 = (locals.var_chi - 1.0);
        let assign58510_body18_e91213: f64 = (assign58510_body18_e91211 + locals.var_fs01);
        let assign58510_body18_e91214: f64 = (assign58510_body18_e91213).sqrt();
        (assign58510_body18_e91214, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn7 + locals.var_fs01_dn7) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn9 + locals.var_fs01_dn9) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign58510_body18_e91214)), ((locals.var_chi_dn13 + locals.var_fs01_dn13) / (2.0 * assign58510_body18_e91214)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign58510_body18_e91216;
            locals.var_fs02_dn0 = assign58510_body18_e91216_d_n0;
            locals.var_fs02_dn2 = assign58510_body18_e91216_d_n2;
            locals.var_fs02_dn4 = assign58510_body18_e91216_d_n4;
            locals.var_fs02_dn5 = assign58510_body18_e91216_d_n5;
            locals.var_fs02_dn6 = assign58510_body18_e91216_d_n6;
            locals.var_fs02_dn7 = assign58510_body18_e91216_d_n7;
            locals.var_fs02_dn8 = assign58510_body18_e91216_d_n8;
            locals.var_fs02_dn9 = assign58510_body18_e91216_d_n9;
            locals.var_fs02_dn10 = assign58510_body18_e91216_d_n10;
            locals.var_fs02_dn13 = assign58510_body18_e91216_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign58510_body19_e91232, assign58510_body19_e91232_d_n0, assign58510_body19_e91232_d_n2, assign58510_body19_e91232_d_n4, assign58510_body19_e91232_d_n5, assign58510_body19_e91232_d_n6, assign58510_body19_e91232_d_n7, assign58510_body19_e91232_d_n8, assign58510_body19_e91232_d_n9, assign58510_body19_e91232_d_n10, assign58510_body19_e91232_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58510_body19_e91226: f64 = (locals.var_beta + locals.var_fs01_dps0);
        let assign58510_body19_e91229: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58510_body19_e91230: f64 = (assign58510_body19_e91226 / assign58510_body19_e91229);
        (assign58510_body19_e91230, ((((locals.var_beta_dn0 + locals.var_fs01_dps0_dn0) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn2 + locals.var_fs01_dps0_dn2) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn5 + locals.var_fs01_dps0_dn5) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn6 + locals.var_fs01_dps0_dn6) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn7 + locals.var_fs01_dps0_dn7) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn8 + locals.var_fs01_dps0_dn8) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn9 + locals.var_fs01_dps0_dn9) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn10 + locals.var_fs01_dps0_dn10) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58510_body19_e91229 * assign58510_body19_e91229)), ((((locals.var_beta_dn13 + locals.var_fs01_dps0_dn13) * assign58510_body19_e91229) - (assign58510_body19_e91226 * (locals.var_fs02_dn13 + locals.var_fs02_dn13))) / (assign58510_body19_e91229 * assign58510_body19_e91229)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign58510_body19_e91232;
            locals.var_fs02_dps0_dn0 = assign58510_body19_e91232_d_n0;
            locals.var_fs02_dps0_dn2 = assign58510_body19_e91232_d_n2;
            locals.var_fs02_dps0_dn4 = assign58510_body19_e91232_d_n4;
            locals.var_fs02_dps0_dn5 = assign58510_body19_e91232_d_n5;
            locals.var_fs02_dps0_dn6 = assign58510_body19_e91232_d_n6;
            locals.var_fs02_dps0_dn7 = assign58510_body19_e91232_d_n7;
            locals.var_fs02_dps0_dn8 = assign58510_body19_e91232_d_n8;
            locals.var_fs02_dps0_dn9 = assign58510_body19_e91232_d_n9;
            locals.var_fs02_dps0_dn10 = assign58510_body19_e91232_d_n10;
            locals.var_fs02_dps0_dn13 = assign58510_body19_e91232_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign58510_body20_e91245, assign58510_body20_e91245_d_n0, assign58510_body20_e91245_d_n2, assign58510_body20_e91245_d_n4, assign58510_body20_e91245_d_n5, assign58510_body20_e91245_d_n6, assign58510_body20_e91245_d_n7, assign58510_body20_e91245_d_n8, assign58510_body20_e91245_d_n9, assign58510_body20_e91245_d_n10, assign58510_body20_e91245_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58510_body20_e91239: f64 = (locals.var_vgp - locals.var_ps0);
        let assign58510_body20_e91242: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign58510_body20_e91243: f64 = (assign58510_body20_e91239 - assign58510_body20_e91242);
        (assign58510_body20_e91243, ((locals.var_vgp_dn0 - locals.var_ps0_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgp_dn2 - locals.var_ps0_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((locals.var_vgp_dn4 - locals.var_ps0_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgp_dn5 - locals.var_ps0_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((locals.var_vgp_dn6 - locals.var_ps0_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((locals.var_vgp_dn7 - locals.var_ps0_dn7) - ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), ((locals.var_vgp_dn8 - locals.var_ps0_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((locals.var_vgp_dn9 - locals.var_ps0_dn9) - ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), ((locals.var_vgp_dn10 - locals.var_ps0_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((locals.var_vgp_dn13 - locals.var_ps0_dn13) - ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign58510_body20_e91245;
            locals.var_fs0_dn0 = assign58510_body20_e91245_d_n0;
            locals.var_fs0_dn2 = assign58510_body20_e91245_d_n2;
            locals.var_fs0_dn4 = assign58510_body20_e91245_d_n4;
            locals.var_fs0_dn5 = assign58510_body20_e91245_d_n5;
            locals.var_fs0_dn6 = assign58510_body20_e91245_d_n6;
            locals.var_fs0_dn7 = assign58510_body20_e91245_d_n7;
            locals.var_fs0_dn8 = assign58510_body20_e91245_d_n8;
            locals.var_fs0_dn9 = assign58510_body20_e91245_d_n9;
            locals.var_fs0_dn10 = assign58510_body20_e91245_d_n10;
            locals.var_fs0_dn13 = assign58510_body20_e91245_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign58510_body21_e91257, assign58510_body21_e91257_d_n0, assign58510_body21_e91257_d_n2, assign58510_body21_e91257_d_n4, assign58510_body21_e91257_d_n5, assign58510_body21_e91257_d_n6, assign58510_body21_e91257_d_n7, assign58510_body21_e91257_d_n8, assign58510_body21_e91257_d_n9, assign58510_body21_e91257_d_n10, assign58510_body21_e91257_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58510_body21_e91251: f64 = (-1.0);
        let assign58510_body21_e91254: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign58510_body21_e91255: f64 = (assign58510_body21_e91251 - assign58510_body21_e91254);
        (assign58510_body21_e91255, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13))),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign58510_body21_e91257;
            locals.var_fs0_dps0_dn0 = assign58510_body21_e91257_d_n0;
            locals.var_fs0_dps0_dn2 = assign58510_body21_e91257_d_n2;
            locals.var_fs0_dps0_dn4 = assign58510_body21_e91257_d_n4;
            locals.var_fs0_dps0_dn5 = assign58510_body21_e91257_d_n5;
            locals.var_fs0_dps0_dn6 = assign58510_body21_e91257_d_n6;
            locals.var_fs0_dps0_dn7 = assign58510_body21_e91257_d_n7;
            locals.var_fs0_dps0_dn8 = assign58510_body21_e91257_d_n8;
            locals.var_fs0_dps0_dn9 = assign58510_body21_e91257_d_n9;
            locals.var_fs0_dps0_dn10 = assign58510_body21_e91257_d_n10;
            locals.var_fs0_dps0_dn13 = assign58510_body21_e91257_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign58510_body22_e91260: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1437 = assign58510_body22_e91260;
            locals.var_guard1437_rv = 0.0;
            let (assign58510_body23_e91269,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58510_body23_e91269;
            locals.var_flg_brk1_rv = 0.0;
            let assign58510_body24_e91272: f64 = if locals.var_flg_brk1 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1438 = assign58510_body24_e91272;
            locals.var_guard1438_rv = 0.0;
            let (assign58510_body25_e91284, assign58510_body25_e91284_d_n0, assign58510_body25_e91284_d_n2, assign58510_body25_e91284_d_n4, assign58510_body25_e91284_d_n5, assign58510_body25_e91284_d_n6, assign58510_body25_e91284_d_n7, assign58510_body25_e91284_d_n8, assign58510_body25_e91284_d_n9, assign58510_body25_e91284_d_n10, assign58510_body25_e91284_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58510_body25_e91280: f64 = (-locals.var_fs0);
        let assign58510_body25_e91282: f64 = (assign58510_body25_e91280 / locals.var_fs0_dps0);
        (assign58510_body25_e91282, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign58510_body25_e91280 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign58510_body25_e91284;
            locals.var_dps0_dn0 = assign58510_body25_e91284_d_n0;
            locals.var_dps0_dn2 = assign58510_body25_e91284_d_n2;
            locals.var_dps0_dn4 = assign58510_body25_e91284_d_n4;
            locals.var_dps0_dn5 = assign58510_body25_e91284_d_n5;
            locals.var_dps0_dn6 = assign58510_body25_e91284_d_n6;
            locals.var_dps0_dn7 = assign58510_body25_e91284_d_n7;
            locals.var_dps0_dn8 = assign58510_body25_e91284_d_n8;
            locals.var_dps0_dn9 = assign58510_body25_e91284_d_n9;
            locals.var_dps0_dn10 = assign58510_body25_e91284_d_n10;
            locals.var_dps0_dn13 = assign58510_body25_e91284_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign58510_body26_e91306, assign58510_body26_e91306_d_n0, assign58510_body26_e91306_d_n2, assign58510_body26_e91306_d_n4, assign58510_body26_e91306_d_n5, assign58510_body26_e91306_d_n6, assign58510_body26_e91306_d_n7, assign58510_body26_e91306_d_n8, assign58510_body26_e91306_d_n9, assign58510_body26_e91306_d_n10, assign58510_body26_e91306_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58510_body26_e91293: f64 = (0.5 * 0.1);
        let assign58510_body26_e91297: f64 = (locals.var_ps0).abs();
        let (assign58510_body26_e91302, assign58510_body26_e91302_d_n0, assign58510_body26_e91302_d_n2, assign58510_body26_e91302_d_n4, assign58510_body26_e91302_d_n5, assign58510_body26_e91302_d_n6, assign58510_body26_e91302_d_n7, assign58510_body26_e91302_d_n8, assign58510_body26_e91302_d_n9, assign58510_body26_e91302_d_n10, assign58510_body26_e91302_d_n13,) = {
            if (1.0 >= assign58510_body26_e91297) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58510_body26_e91301: f64 = (locals.var_ps0).abs();
                (assign58510_body26_e91301, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn0 } else { (-locals.var_ps0_dn0) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn2 } else { (-locals.var_ps0_dn2) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn4 } else { (-locals.var_ps0_dn4) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn5 } else { (-locals.var_ps0_dn5) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn6 } else { (-locals.var_ps0_dn6) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn7 } else { (-locals.var_ps0_dn7) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn8 } else { (-locals.var_ps0_dn8) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn9 } else { (-locals.var_ps0_dn9) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn10 } else { (-locals.var_ps0_dn10) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn13 } else { (-locals.var_ps0_dn13) },)
            }
        };
        let assign58510_body26_e91303: f64 = (1.0 + assign58510_body26_e91302);
        let assign58510_body26_e91304: f64 = (assign58510_body26_e91293 * assign58510_body26_e91303);
        (assign58510_body26_e91304, (assign58510_body26_e91293 * assign58510_body26_e91302_d_n0), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n2), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n4), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n5), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n6), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n7), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n8), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n9), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n10), (assign58510_body26_e91293 * assign58510_body26_e91302_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign58510_body26_e91306;
            locals.var_dplim_dn0 = assign58510_body26_e91306_d_n0;
            locals.var_dplim_dn2 = assign58510_body26_e91306_d_n2;
            locals.var_dplim_dn4 = assign58510_body26_e91306_d_n4;
            locals.var_dplim_dn5 = assign58510_body26_e91306_d_n5;
            locals.var_dplim_dn6 = assign58510_body26_e91306_d_n6;
            locals.var_dplim_dn7 = assign58510_body26_e91306_d_n7;
            locals.var_dplim_dn8 = assign58510_body26_e91306_d_n8;
            locals.var_dplim_dn9 = assign58510_body26_e91306_d_n9;
            locals.var_dplim_dn10 = assign58510_body26_e91306_d_n10;
            locals.var_dplim_dn13 = assign58510_body26_e91306_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign58510_body27_e91308: f64 = (locals.var_dps0).abs();
            let assign58510_body27_e91310: f64 = if assign58510_body27_e91308 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1439 = assign58510_body27_e91310;
            locals.var_guard1439_rv = 0.0;
            let (assign58510_body28_e91329, assign58510_body28_e91329_d_n0, assign58510_body28_e91329_d_n2, assign58510_body28_e91329_d_n4, assign58510_body28_e91329_d_n5, assign58510_body28_e91329_d_n6, assign58510_body28_e91329_d_n7, assign58510_body28_e91329_d_n8, assign58510_body28_e91329_d_n9, assign58510_body28_e91329_d_n10, assign58510_body28_e91329_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1438 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        let (assign58510_body28_e91326,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign58510_body28_e91325: f64 = (-1.0);
                (assign58510_body28_e91325,)
            }
        };
        let assign58510_body28_e91327: f64 = (locals.var_dplim * assign58510_body28_e91326);
        (assign58510_body28_e91327, (locals.var_dplim_dn0 * assign58510_body28_e91326), (locals.var_dplim_dn2 * assign58510_body28_e91326), (locals.var_dplim_dn4 * assign58510_body28_e91326), (locals.var_dplim_dn5 * assign58510_body28_e91326), (locals.var_dplim_dn6 * assign58510_body28_e91326), (locals.var_dplim_dn7 * assign58510_body28_e91326), (locals.var_dplim_dn8 * assign58510_body28_e91326), (locals.var_dplim_dn9 * assign58510_body28_e91326), (locals.var_dplim_dn10 * assign58510_body28_e91326), (locals.var_dplim_dn13 * assign58510_body28_e91326),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign58510_body28_e91329;
            locals.var_dps0_dn0 = assign58510_body28_e91329_d_n0;
            locals.var_dps0_dn2 = assign58510_body28_e91329_d_n2;
            locals.var_dps0_dn4 = assign58510_body28_e91329_d_n4;
            locals.var_dps0_dn5 = assign58510_body28_e91329_d_n5;
            locals.var_dps0_dn6 = assign58510_body28_e91329_d_n6;
            locals.var_dps0_dn7 = assign58510_body28_e91329_d_n7;
            locals.var_dps0_dn8 = assign58510_body28_e91329_d_n8;
            locals.var_dps0_dn9 = assign58510_body28_e91329_d_n9;
            locals.var_dps0_dn10 = assign58510_body28_e91329_d_n10;
            locals.var_dps0_dn13 = assign58510_body28_e91329_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign58510_body29_e91340, assign58510_body29_e91340_d_n0, assign58510_body29_e91340_d_n2, assign58510_body29_e91340_d_n4, assign58510_body29_e91340_d_n5, assign58510_body29_e91340_d_n6, assign58510_body29_e91340_d_n7, assign58510_body29_e91340_d_n8, assign58510_body29_e91340_d_n9, assign58510_body29_e91340_d_n10, assign58510_body29_e91340_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58510_body29_e91338: f64 = (locals.var_ps0 + locals.var_dps0);
        (assign58510_body29_e91338, (locals.var_ps0_dn0 + locals.var_dps0_dn0), (locals.var_ps0_dn2 + locals.var_dps0_dn2), (locals.var_ps0_dn4 + locals.var_dps0_dn4), (locals.var_ps0_dn5 + locals.var_dps0_dn5), (locals.var_ps0_dn6 + locals.var_dps0_dn6), (locals.var_ps0_dn7 + locals.var_dps0_dn7), (locals.var_ps0_dn8 + locals.var_dps0_dn8), (locals.var_ps0_dn9 + locals.var_dps0_dn9), (locals.var_ps0_dn10 + locals.var_dps0_dn10), (locals.var_ps0_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    }
};
            locals.var_ps0 = assign58510_body29_e91340;
            locals.var_ps0_dn0 = assign58510_body29_e91340_d_n0;
            locals.var_ps0_dn2 = assign58510_body29_e91340_d_n2;
            locals.var_ps0_dn4 = assign58510_body29_e91340_d_n4;
            locals.var_ps0_dn5 = assign58510_body29_e91340_d_n5;
            locals.var_ps0_dn6 = assign58510_body29_e91340_d_n6;
            locals.var_ps0_dn7 = assign58510_body29_e91340_d_n7;
            locals.var_ps0_dn8 = assign58510_body29_e91340_d_n8;
            locals.var_ps0_dn9 = assign58510_body29_e91340_d_n9;
            locals.var_ps0_dn10 = assign58510_body29_e91340_d_n10;
            locals.var_ps0_dn13 = assign58510_body29_e91340_d_n13;
            locals.var_ps0_rv = 0.0;
            let assign58510_body30_e91342: f64 = (locals.var_dps0).abs();
            let assign58510_body30_e91346: f64 = (locals.var_fs0).abs();
            let assign58510_body30_e91349: f64 = if ((assign58510_body30_e91342 <= 1e-12) && (assign58510_body30_e91346 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1440 = assign58510_body30_e91349;
            locals.var_guard1440_rv = 0.0;
            let (assign58510_body31_e91360,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1438 != 0.0)) && (locals.var_guard1440 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign58510_body31_e91360;
            locals.var_flg_conv_rv = 0.0;
            let (assign58510_body32_e91371,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_flg_brk1 != 0.0)) {
        let assign58510_body32_e91369: f64 = (locals.var_lp_s0_max + 1.0);
        (assign58510_body32_e91369,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58510_body32_e91371;
            locals.var_lp_s0_rv = 0.0;
            let (assign58510_body33_e91378,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58510_body33_e91378;
            locals.var_flg_brk1_rv = 0.0;
            let (assign58510_body34_e91387,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58510_body34_e91385: f64 = (locals.var_lp_s0 + 1.0);
        (assign58510_body34_e91385,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58510_body34_e91387;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_208(
        locals: &mut StampLocals,
    ) {
        let (assign58520_e91396,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58520_e91394: f64 = (locals.var_lp_s0 - 1.0);
        (assign58520_e91394,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign58520_e91396;
        locals.var_lp_s0_rv = 0.0;

        let assign58540_e91402: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1442 = assign58540_e91402;
        locals.var_guard1442_rv = 0.0;

        let (assign58550_e91417, assign58550_e91417_d_n0, assign58550_e91417_d_n2, assign58550_e91417_d_n4, assign58550_e91417_d_n5, assign58550_e91417_d_n6, assign58550_e91417_d_n7, assign58550_e91417_d_n8, assign58550_e91417_d_n9, assign58550_e91417_d_n10, assign58550_e91417_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58550_e91411: f64 = (locals.var_fb * locals.var_fb);
        let assign58550_e91414: f64 = (10.0 * 2.220446049250313e-16);
        let assign58550_e91415: f64 = (assign58550_e91411 + assign58550_e91414);
        (assign58550_e91415, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn8, locals.var_xi0_dn9, locals.var_xi0_dn10, locals.var_xi0_dn13,)
    }
};
        locals.var_xi0 = assign58550_e91417;
        locals.var_xi0_dn0 = assign58550_e91417_d_n0;
        locals.var_xi0_dn2 = assign58550_e91417_d_n2;
        locals.var_xi0_dn4 = assign58550_e91417_d_n4;
        locals.var_xi0_dn5 = assign58550_e91417_d_n5;
        locals.var_xi0_dn6 = assign58550_e91417_d_n6;
        locals.var_xi0_dn7 = assign58550_e91417_d_n7;
        locals.var_xi0_dn8 = assign58550_e91417_d_n8;
        locals.var_xi0_dn9 = assign58550_e91417_d_n9;
        locals.var_xi0_dn10 = assign58550_e91417_d_n10;
        locals.var_xi0_dn13 = assign58550_e91417_d_n13;
        locals.var_xi0_rv = 0.0;

        let (assign58560_e91430, assign58560_e91430_d_n0, assign58560_e91430_d_n2, assign58560_e91430_d_n4, assign58560_e91430_d_n5, assign58560_e91430_d_n6, assign58560_e91430_d_n7, assign58560_e91430_d_n8, assign58560_e91430_d_n9, assign58560_e91430_d_n10, assign58560_e91430_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58560_e91427: f64 = (10.0 * 2.220446049250313e-16);
        let assign58560_e91428: f64 = (locals.var_fb + assign58560_e91427);
        (assign58560_e91428, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign58560_e91430;
        locals.var_xi0p12_dn0 = assign58560_e91430_d_n0;
        locals.var_xi0p12_dn2 = assign58560_e91430_d_n2;
        locals.var_xi0p12_dn4 = assign58560_e91430_d_n4;
        locals.var_xi0p12_dn5 = assign58560_e91430_d_n5;
        locals.var_xi0p12_dn6 = assign58560_e91430_d_n6;
        locals.var_xi0p12_dn7 = assign58560_e91430_d_n7;
        locals.var_xi0p12_dn8 = assign58560_e91430_d_n8;
        locals.var_xi0p12_dn9 = assign58560_e91430_d_n9;
        locals.var_xi0p12_dn10 = assign58560_e91430_d_n10;
        locals.var_xi0p12_dn13 = assign58560_e91430_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign58570_e91447, assign58570_e91447_d_n0, assign58570_e91447_d_n2, assign58570_e91447_d_n4, assign58570_e91447_d_n5, assign58570_e91447_d_n6, assign58570_e91447_d_n7, assign58570_e91447_d_n8, assign58570_e91447_d_n9, assign58570_e91447_d_n10, assign58570_e91447_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        let assign58570_e91439: f64 = (locals.var_fb * locals.var_fb);
        let assign58570_e91441: f64 = (assign58570_e91439 * locals.var_fb);
        let assign58570_e91444: f64 = (10.0 * 2.220446049250313e-16);
        let assign58570_e91445: f64 = (assign58570_e91441 + assign58570_e91444);
        (assign58570_e91445, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn0)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn2)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn4)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn5)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn6)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn7)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn8)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn9)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn10)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) * locals.var_fb) + (assign58570_e91439 * locals.var_fb_dn13)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn4, locals.var_xi0p32_dn5, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn8, locals.var_xi0p32_dn9, locals.var_xi0p32_dn10, locals.var_xi0p32_dn13,)
    }
};
        locals.var_xi0p32 = assign58570_e91447;
        locals.var_xi0p32_dn0 = assign58570_e91447_d_n0;
        locals.var_xi0p32_dn2 = assign58570_e91447_d_n2;
        locals.var_xi0p32_dn4 = assign58570_e91447_d_n4;
        locals.var_xi0p32_dn5 = assign58570_e91447_d_n5;
        locals.var_xi0p32_dn6 = assign58570_e91447_d_n6;
        locals.var_xi0p32_dn7 = assign58570_e91447_d_n7;
        locals.var_xi0p32_dn8 = assign58570_e91447_d_n8;
        locals.var_xi0p32_dn9 = assign58570_e91447_d_n9;
        locals.var_xi0p32_dn10 = assign58570_e91447_d_n10;
        locals.var_xi0p32_dn13 = assign58570_e91447_d_n13;
        locals.var_xi0p32_rv = 0.0;

        let (assign58580_e91457,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 == 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58580_e91457;
        locals.var_flg_zone_rv = 0.0;

        let (assign58590_e91467,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58590_e91467;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58600_e91479, assign58600_e91479_d_n0, assign58600_e91479_d_n2, assign58600_e91479_d_n4, assign58600_e91479_d_n5, assign58600_e91479_d_n6, assign58600_e91479_d_n7, assign58600_e91479_d_n8, assign58600_e91479_d_n9, assign58600_e91479_d_n10, assign58600_e91479_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 == 0.0)) {
        let assign58600_e91477: f64 = (locals.var_chi - 1.0);
        (assign58600_e91477, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn8, locals.var_xi0_dn9, locals.var_xi0_dn10, locals.var_xi0_dn13,)
    }
};
        locals.var_xi0 = assign58600_e91479;
        locals.var_xi0_dn0 = assign58600_e91479_d_n0;
        locals.var_xi0_dn2 = assign58600_e91479_d_n2;
        locals.var_xi0_dn4 = assign58600_e91479_d_n4;
        locals.var_xi0_dn5 = assign58600_e91479_d_n5;
        locals.var_xi0_dn6 = assign58600_e91479_d_n6;
        locals.var_xi0_dn7 = assign58600_e91479_d_n7;
        locals.var_xi0_dn8 = assign58600_e91479_d_n8;
        locals.var_xi0_dn9 = assign58600_e91479_d_n9;
        locals.var_xi0_dn10 = assign58600_e91479_d_n10;
        locals.var_xi0_dn13 = assign58600_e91479_d_n13;
        locals.var_xi0_rv = 0.0;

        let (assign58610_e91490, assign58610_e91490_d_n0, assign58610_e91490_d_n2, assign58610_e91490_d_n4, assign58610_e91490_d_n5, assign58610_e91490_d_n6, assign58610_e91490_d_n7, assign58610_e91490_d_n8, assign58610_e91490_d_n9, assign58610_e91490_d_n10, assign58610_e91490_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 == 0.0)) {
        let assign58610_e91488: f64 = (locals.var_xi0).sqrt();
        (assign58610_e91488, (locals.var_xi0_dn0 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn2 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn4 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn5 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn6 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn7 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn8 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn9 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn10 / (2.0 * assign58610_e91488)), (locals.var_xi0_dn13 / (2.0 * assign58610_e91488)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign58610_e91490;
        locals.var_xi0p12_dn0 = assign58610_e91490_d_n0;
        locals.var_xi0p12_dn2 = assign58610_e91490_d_n2;
        locals.var_xi0p12_dn4 = assign58610_e91490_d_n4;
        locals.var_xi0p12_dn5 = assign58610_e91490_d_n5;
        locals.var_xi0p12_dn6 = assign58610_e91490_d_n6;
        locals.var_xi0p12_dn7 = assign58610_e91490_d_n7;
        locals.var_xi0p12_dn8 = assign58610_e91490_d_n8;
        locals.var_xi0p12_dn9 = assign58610_e91490_d_n9;
        locals.var_xi0p12_dn10 = assign58610_e91490_d_n10;
        locals.var_xi0p12_dn13 = assign58610_e91490_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign58620_e91502, assign58620_e91502_d_n0, assign58620_e91502_d_n2, assign58620_e91502_d_n4, assign58620_e91502_d_n5, assign58620_e91502_d_n6, assign58620_e91502_d_n7, assign58620_e91502_d_n8, assign58620_e91502_d_n9, assign58620_e91502_d_n10, assign58620_e91502_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1442 == 0.0)) {
        let assign58620_e91500: f64 = (locals.var_xi0 * locals.var_xi0p12);
        (assign58620_e91500, ((locals.var_xi0_dn0 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn0)), ((locals.var_xi0_dn2 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn2)), ((locals.var_xi0_dn4 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn4)), ((locals.var_xi0_dn5 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn5)), ((locals.var_xi0_dn6 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn6)), ((locals.var_xi0_dn7 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn7)), ((locals.var_xi0_dn8 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn8)), ((locals.var_xi0_dn9 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn9)), ((locals.var_xi0_dn10 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn10)), ((locals.var_xi0_dn13 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn4, locals.var_xi0p32_dn5, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn8, locals.var_xi0p32_dn9, locals.var_xi0p32_dn10, locals.var_xi0p32_dn13,)
    }
};
        locals.var_xi0p32 = assign58620_e91502;
        locals.var_xi0p32_dn0 = assign58620_e91502_d_n0;
        locals.var_xi0p32_dn2 = assign58620_e91502_d_n2;
        locals.var_xi0p32_dn4 = assign58620_e91502_d_n4;
        locals.var_xi0p32_dn5 = assign58620_e91502_d_n5;
        locals.var_xi0p32_dn6 = assign58620_e91502_d_n6;
        locals.var_xi0p32_dn7 = assign58620_e91502_d_n7;
        locals.var_xi0p32_dn8 = assign58620_e91502_d_n8;
        locals.var_xi0p32_dn9 = assign58620_e91502_d_n9;
        locals.var_xi0p32_dn10 = assign58620_e91502_d_n10;
        locals.var_xi0p32_dn13 = assign58620_e91502_d_n13;
        locals.var_xi0p32_rv = 0.0;

        let (assign58630_e91511, assign58630_e91511_d_n0, assign58630_e91511_d_n2, assign58630_e91511_d_n4, assign58630_e91511_d_n5, assign58630_e91511_d_n6, assign58630_e91511_d_n7, assign58630_e91511_d_n8, assign58630_e91511_d_n9, assign58630_e91511_d_n10, assign58630_e91511_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58630_e91509: f64 = (locals.var_cnst0 * locals.var_xi0p12);
        (assign58630_e91509, ((locals.var_cnst0_dn0 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn0)), ((locals.var_cnst0_dn2 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn2)), ((locals.var_cnst0_dn4 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn4)), ((locals.var_cnst0_dn5 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn5)), ((locals.var_cnst0_dn6 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn6)), ((locals.var_cnst0_dn7 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn7)), ((locals.var_cnst0_dn8 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn8)), ((locals.var_cnst0_dn9 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn9)), ((locals.var_cnst0_dn10 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn10)), ((locals.var_cnst0_dn13 * locals.var_xi0p12) + (locals.var_cnst0 * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn4, locals.var_qb0_dn5, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn8, locals.var_qb0_dn9, locals.var_qb0_dn10, locals.var_qb0_dn13,)
    }
};
        locals.var_qb0 = assign58630_e91511;
        locals.var_qb0_dn0 = assign58630_e91511_d_n0;
        locals.var_qb0_dn2 = assign58630_e91511_d_n2;
        locals.var_qb0_dn4 = assign58630_e91511_d_n4;
        locals.var_qb0_dn5 = assign58630_e91511_d_n5;
        locals.var_qb0_dn6 = assign58630_e91511_d_n6;
        locals.var_qb0_dn7 = assign58630_e91511_d_n7;
        locals.var_qb0_dn8 = assign58630_e91511_d_n8;
        locals.var_qb0_dn9 = assign58630_e91511_d_n9;
        locals.var_qb0_dn10 = assign58630_e91511_d_n10;
        locals.var_qb0_dn13 = assign58630_e91511_d_n13;
        locals.var_qb0_rv = 0.0;

        let (assign58640_e91522, assign58640_e91522_d_n0, assign58640_e91522_d_n2, assign58640_e91522_d_n4, assign58640_e91522_d_n5, assign58640_e91522_d_n6, assign58640_e91522_d_n7, assign58640_e91522_d_n8, assign58640_e91522_d_n9, assign58640_e91522_d_n10, assign58640_e91522_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58640_e91519: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign58640_e91520: f64 = (1.0 / assign58640_e91519);
        (assign58640_e91520, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign58640_e91519 * assign58640_e91519))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign58640_e91519 * assign58640_e91519))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58640_e91522;
        locals.var_t1_dn0 = assign58640_e91522_d_n0;
        locals.var_t1_dn2 = assign58640_e91522_d_n2;
        locals.var_t1_dn4 = assign58640_e91522_d_n4;
        locals.var_t1_dn5 = assign58640_e91522_d_n5;
        locals.var_t1_dn6 = assign58640_e91522_d_n6;
        locals.var_t1_dn7 = assign58640_e91522_d_n7;
        locals.var_t1_dn8 = assign58640_e91522_d_n8;
        locals.var_t1_dn9 = assign58640_e91522_d_n9;
        locals.var_t1_dn10 = assign58640_e91522_d_n10;
        locals.var_t1_dn13 = assign58640_e91522_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58650_e91535, assign58650_e91535_d_n0, assign58650_e91535_d_n2, assign58650_e91535_d_n4, assign58650_e91535_d_n5, assign58650_e91535_d_n6, assign58650_e91535_d_n7, assign58650_e91535_d_n8, assign58650_e91535_d_n9, assign58650_e91535_d_n10, assign58650_e91535_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58650_e91529: f64 = (locals.var_cnst0 * locals.var_fs01);
        let assign58650_e91531: f64 = (assign58650_e91529 * locals.var_t1);
        let assign58650_e91533: f64 = (assign58650_e91531 + 1e-25);
        (assign58650_e91533, ((((locals.var_cnst0_dn0 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn0)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn0)), ((((locals.var_cnst0_dn2 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn2)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn2)), ((((locals.var_cnst0_dn4 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn4)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn4)), ((((locals.var_cnst0_dn5 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn5)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn5)), ((((locals.var_cnst0_dn6 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn6)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn6)), ((((locals.var_cnst0_dn7 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn7)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn7)), ((((locals.var_cnst0_dn8 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn8)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn8)), ((((locals.var_cnst0_dn9 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn9)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn9)), ((((locals.var_cnst0_dn10 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn10)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn10)), ((((locals.var_cnst0_dn13 * locals.var_fs01) + (locals.var_cnst0 * locals.var_fs01_dn13)) * locals.var_t1) + (assign58650_e91529 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign58650_e91535;
        locals.var_qn0_dn0 = assign58650_e91535_d_n0;
        locals.var_qn0_dn2 = assign58650_e91535_d_n2;
        locals.var_qn0_dn4 = assign58650_e91535_d_n4;
        locals.var_qn0_dn5 = assign58650_e91535_d_n5;
        locals.var_qn0_dn6 = assign58650_e91535_d_n6;
        locals.var_qn0_dn7 = assign58650_e91535_d_n7;
        locals.var_qn0_dn8 = assign58650_e91535_d_n8;
        locals.var_qn0_dn9 = assign58650_e91535_d_n9;
        locals.var_qn0_dn10 = assign58650_e91535_d_n10;
        locals.var_qn0_dn13 = assign58650_e91535_d_n13;
        locals.var_qn0_rv = 0.0;

        let assign58660_e91538: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1443 = assign58660_e91538;
        locals.var_guard1443_rv = 0.0;

        let assign58670_e91541: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1444 = assign58670_e91541;
        locals.var_guard1444_rv = 0.0;

        let (assign58680_e91552,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58680_e91552;
        locals.var_flg_zone_rv = 0.0;

        let (assign58690_e91563,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58690_e91563;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58700_e91574, assign58700_e91574_d_n0, assign58700_e91574_d_n2, assign58700_e91574_d_n4, assign58700_e91574_d_n5, assign58700_e91574_d_n6, assign58700_e91574_d_n7, assign58700_e91574_d_n8, assign58700_e91574_d_n9, assign58700_e91574_d_n10, assign58700_e91574_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign58700_e91574;
        locals.var_qiu_dn0 = assign58700_e91574_d_n0;
        locals.var_qiu_dn2 = assign58700_e91574_d_n2;
        locals.var_qiu_dn4 = assign58700_e91574_d_n4;
        locals.var_qiu_dn5 = assign58700_e91574_d_n5;
        locals.var_qiu_dn6 = assign58700_e91574_d_n6;
        locals.var_qiu_dn7 = assign58700_e91574_d_n7;
        locals.var_qiu_dn8 = assign58700_e91574_d_n8;
        locals.var_qiu_dn9 = assign58700_e91574_d_n9;
        locals.var_qiu_dn10 = assign58700_e91574_d_n10;
        locals.var_qiu_dn13 = assign58700_e91574_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign58710_e91585, assign58710_e91585_d_n0, assign58710_e91585_d_n2, assign58710_e91585_d_n4, assign58710_e91585_d_n5, assign58710_e91585_d_n6, assign58710_e91585_d_n7, assign58710_e91585_d_n8, assign58710_e91585_d_n9, assign58710_e91585_d_n10, assign58710_e91585_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn4, locals.var_qb0_dn5, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn8, locals.var_qb0_dn9, locals.var_qb0_dn10, locals.var_qb0_dn13,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign58710_e91585;
        locals.var_qbu_dn0 = assign58710_e91585_d_n0;
        locals.var_qbu_dn2 = assign58710_e91585_d_n2;
        locals.var_qbu_dn4 = assign58710_e91585_d_n4;
        locals.var_qbu_dn5 = assign58710_e91585_d_n5;
        locals.var_qbu_dn6 = assign58710_e91585_d_n6;
        locals.var_qbu_dn7 = assign58710_e91585_d_n7;
        locals.var_qbu_dn8 = assign58710_e91585_d_n8;
        locals.var_qbu_dn9 = assign58710_e91585_d_n9;
        locals.var_qbu_dn10 = assign58710_e91585_d_n10;
        locals.var_qbu_dn13 = assign58710_e91585_d_n13;
        locals.var_qbu_rv = 0.0;

        let (assign58720_e91596, assign58720_e91596_d_n0, assign58720_e91596_d_n2, assign58720_e91596_d_n4, assign58720_e91596_d_n5, assign58720_e91596_d_n6, assign58720_e91596_d_n7, assign58720_e91596_d_n8, assign58720_e91596_d_n9, assign58720_e91596_d_n10, assign58720_e91596_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign58720_e91596;
        locals.var_qdrat_dn0 = assign58720_e91596_d_n0;
        locals.var_qdrat_dn2 = assign58720_e91596_d_n2;
        locals.var_qdrat_dn4 = assign58720_e91596_d_n4;
        locals.var_qdrat_dn5 = assign58720_e91596_d_n5;
        locals.var_qdrat_dn6 = assign58720_e91596_d_n6;
        locals.var_qdrat_dn7 = assign58720_e91596_d_n7;
        locals.var_qdrat_dn8 = assign58720_e91596_d_n8;
        locals.var_qdrat_dn9 = assign58720_e91596_d_n9;
        locals.var_qdrat_dn10 = assign58720_e91596_d_n10;
        locals.var_qdrat_dn13 = assign58720_e91596_d_n13;
        locals.var_qdrat_rv = 0.0;

        let (assign58730_e91607, assign58730_e91607_d_n0, assign58730_e91607_d_n2, assign58730_e91607_d_n4, assign58730_e91607_d_n5, assign58730_e91607_d_n6, assign58730_e91607_d_n7, assign58730_e91607_d_n8, assign58730_e91607_d_n9, assign58730_e91607_d_n10, assign58730_e91607_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign58730_e91607;
        locals.var_lred_dn0 = assign58730_e91607_d_n0;
        locals.var_lred_dn2 = assign58730_e91607_d_n2;
        locals.var_lred_dn4 = assign58730_e91607_d_n4;
        locals.var_lred_dn5 = assign58730_e91607_d_n5;
        locals.var_lred_dn6 = assign58730_e91607_d_n6;
        locals.var_lred_dn7 = assign58730_e91607_d_n7;
        locals.var_lred_dn8 = assign58730_e91607_d_n8;
        locals.var_lred_dn9 = assign58730_e91607_d_n9;
        locals.var_lred_dn10 = assign58730_e91607_d_n10;
        locals.var_lred_dn13 = assign58730_e91607_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign58740_e91619,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign58740_e91619;
        locals.var_flg_zone_rv = 0.0;

        let (assign58750_e91631,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58750_e91631;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58760_e91647, assign58760_e91647_d_n0, assign58760_e91647_d_n2, assign58760_e91647_d_n4, assign58760_e91647_d_n5, assign58760_e91647_d_n6, assign58760_e91647_d_n7, assign58760_e91647_d_n8, assign58760_e91647_d_n9, assign58760_e91647_d_n10, assign58760_e91647_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 == 0.0)) {
        let assign58760_e91644: f64 = (5.0 - 3.0);
        let assign58760_e91645: f64 = (1.0 / assign58760_e91644);
        (assign58760_e91645, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58760_e91647;
        locals.var_t1_dn0 = assign58760_e91647_d_n0;
        locals.var_t1_dn2 = assign58760_e91647_d_n2;
        locals.var_t1_dn4 = assign58760_e91647_d_n4;
        locals.var_t1_dn5 = assign58760_e91647_d_n5;
        locals.var_t1_dn6 = assign58760_e91647_d_n6;
        locals.var_t1_dn7 = assign58760_e91647_d_n7;
        locals.var_t1_dn8 = assign58760_e91647_d_n8;
        locals.var_t1_dn9 = assign58760_e91647_d_n9;
        locals.var_t1_dn10 = assign58760_e91647_d_n10;
        locals.var_t1_dn13 = assign58760_e91647_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58770_e91663, assign58770_e91663_d_n0, assign58770_e91663_d_n2, assign58770_e91663_d_n4, assign58770_e91663_d_n5, assign58770_e91663_d_n6, assign58770_e91663_d_n7, assign58770_e91663_d_n8, assign58770_e91663_d_n9, assign58770_e91663_d_n10, assign58770_e91663_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 == 0.0)) {
        let assign58770_e91660: f64 = (locals.var_chi - 3.0);
        let assign58770_e91661: f64 = (locals.var_t1 * assign58770_e91660);
        (assign58770_e91661, ((locals.var_t1_dn0 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn0)), ((locals.var_t1_dn2 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn2)), ((locals.var_t1_dn4 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn4)), ((locals.var_t1_dn5 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn5)), ((locals.var_t1_dn6 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn6)), ((locals.var_t1_dn7 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn7)), ((locals.var_t1_dn8 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn8)), ((locals.var_t1_dn9 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn9)), ((locals.var_t1_dn10 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn10)), ((locals.var_t1_dn13 * assign58770_e91660) + (locals.var_t1 * locals.var_chi_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58770_e91663;
        locals.var_tx_dn0 = assign58770_e91663_d_n0;
        locals.var_tx_dn2 = assign58770_e91663_d_n2;
        locals.var_tx_dn4 = assign58770_e91663_d_n4;
        locals.var_tx_dn5 = assign58770_e91663_d_n5;
        locals.var_tx_dn6 = assign58770_e91663_d_n6;
        locals.var_tx_dn7 = assign58770_e91663_d_n7;
        locals.var_tx_dn8 = assign58770_e91663_d_n8;
        locals.var_tx_dn9 = assign58770_e91663_d_n9;
        locals.var_tx_dn10 = assign58770_e91663_d_n10;
        locals.var_tx_dn13 = assign58770_e91663_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign58780_e91690, assign58780_e91690_d_n0, assign58780_e91690_d_n2, assign58780_e91690_d_n4, assign58780_e91690_d_n5, assign58780_e91690_d_n6, assign58780_e91690_d_n7, assign58780_e91690_d_n8, assign58780_e91690_d_n9, assign58780_e91690_d_n10, assign58780_e91690_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1443 != 0.0)) && (locals.var_guard1444 == 0.0)) {
        let assign58780_e91675: f64 = (locals.var_tx * locals.var_tx);
        let assign58780_e91677: f64 = (assign58780_e91675 * locals.var_tx);
        let assign58780_e91681: f64 = (-15.0);
        let assign58780_e91684: f64 = (locals.var_tx * 6.0);
        let assign58780_e91685: f64 = (assign58780_e91681 + assign58780_e91684);
        let assign58780_e91686: f64 = (locals.var_tx * assign58780_e91685);
        let assign58780_e91687: f64 = (10.0 + assign58780_e91686);
        let assign58780_e91688: f64 = (assign58780_e91677 * assign58780_e91687);
        (assign58780_e91688, ((((((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn0)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn0 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn0 * 6.0))))), ((((((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn2)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn2 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn2 * 6.0))))), ((((((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn4)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn4 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn4 * 6.0))))), ((((((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn5)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn5 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn5 * 6.0))))), ((((((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn6)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn6 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn6 * 6.0))))), ((((((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn7)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn7 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn7 * 6.0))))), ((((((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn8)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn8 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn8 * 6.0))))), ((((((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn9)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn9 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn9 * 6.0))))), ((((((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn10)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn10 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn10 * 6.0))))), ((((((locals.var_tx_dn13 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn13)) * locals.var_tx) + (assign58780_e91675 * locals.var_tx_dn13)) * assign58780_e91687) + (assign58780_e91677 * ((locals.var_tx_dn13 * assign58780_e91685) + (locals.var_tx * (locals.var_tx_dn13 * 6.0))))),)
    } else {
        (locals.var_fd2, locals.var_fd2_dn0, locals.var_fd2_dn2, locals.var_fd2_dn4, locals.var_fd2_dn5, locals.var_fd2_dn6, locals.var_fd2_dn7, locals.var_fd2_dn8, locals.var_fd2_dn9, locals.var_fd2_dn10, locals.var_fd2_dn13,)
    }
};
        locals.var_fd2 = assign58780_e91690;
        locals.var_fd2_dn0 = assign58780_e91690_d_n0;
        locals.var_fd2_dn2 = assign58780_e91690_d_n2;
        locals.var_fd2_dn4 = assign58780_e91690_d_n4;
        locals.var_fd2_dn5 = assign58780_e91690_d_n5;
        locals.var_fd2_dn6 = assign58780_e91690_d_n6;
        locals.var_fd2_dn7 = assign58780_e91690_d_n7;
        locals.var_fd2_dn8 = assign58780_e91690_d_n8;
        locals.var_fd2_dn9 = assign58780_e91690_d_n9;
        locals.var_fd2_dn10 = assign58780_e91690_d_n10;
        locals.var_fd2_dn13 = assign58780_e91690_d_n13;
        locals.var_fd2_rv = 0.0;

        let (assign58790_e91699, assign58790_e91699_d_n0, assign58790_e91699_d_n2, assign58790_e91699_d_n4, assign58790_e91699_d_n5, assign58790_e91699_d_n6, assign58790_e91699_d_n7, assign58790_e91699_d_n8, assign58790_e91699_d_n9, assign58790_e91699_d_n10, assign58790_e91699_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58790_e91697: f64 = (locals.var_qn0 * locals.var_cox_inv);
        (assign58790_e91697, ((locals.var_qn0_dn0 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn2)), ((locals.var_qn0_dn4 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn4)), ((locals.var_qn0_dn5 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn5)), ((locals.var_qn0_dn6 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn7)), ((locals.var_qn0_dn8 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn8)), ((locals.var_qn0_dn9 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn9)), ((locals.var_qn0_dn10 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn10)), ((locals.var_qn0_dn13 * locals.var_cox_inv) + (locals.var_qn0 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn4, locals.var_vgvt_dn5, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn8, locals.var_vgvt_dn9, locals.var_vgvt_dn10, locals.var_vgvt_dn13,)
    }
};
        locals.var_vgvt = assign58790_e91699;
        locals.var_vgvt_dn0 = assign58790_e91699_d_n0;
        locals.var_vgvt_dn2 = assign58790_e91699_d_n2;
        locals.var_vgvt_dn4 = assign58790_e91699_d_n4;
        locals.var_vgvt_dn5 = assign58790_e91699_d_n5;
        locals.var_vgvt_dn6 = assign58790_e91699_d_n6;
        locals.var_vgvt_dn7 = assign58790_e91699_d_n7;
        locals.var_vgvt_dn8 = assign58790_e91699_d_n8;
        locals.var_vgvt_dn9 = assign58790_e91699_d_n9;
        locals.var_vgvt_dn10 = assign58790_e91699_d_n10;
        locals.var_vgvt_dn13 = assign58790_e91699_d_n13;
        locals.var_vgvt_rv = 0.0;

        let (assign58800_e91706, assign58800_e91706_d_n0, assign58800_e91706_d_n2, assign58800_e91706_d_n4, assign58800_e91706_d_n5, assign58800_e91706_d_n6, assign58800_e91706_d_n7, assign58800_e91706_d_n8, assign58800_e91706_d_n9, assign58800_e91706_d_n10, assign58800_e91706_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    }
};
        locals.var_vdsorg = assign58800_e91706;
        locals.var_vdsorg_dn0 = assign58800_e91706_d_n0;
        locals.var_vdsorg_dn2 = assign58800_e91706_d_n2;
        locals.var_vdsorg_dn4 = assign58800_e91706_d_n4;
        locals.var_vdsorg_dn5 = assign58800_e91706_d_n5;
        locals.var_vdsorg_dn6 = assign58800_e91706_d_n6;
        locals.var_vdsorg_dn7 = assign58800_e91706_d_n7;
        locals.var_vdsorg_dn8 = assign58800_e91706_d_n8;
        locals.var_vdsorg_dn9 = assign58800_e91706_d_n9;
        locals.var_vdsorg_dn10 = assign58800_e91706_d_n10;
        locals.var_vdsorg_dn13 = assign58800_e91706_d_n13;
        locals.var_vdsorg_rv = 0.0;

        let (assign58810_e91717, assign58810_e91717_d_n0, assign58810_e91717_d_n2, assign58810_e91717_d_n4, assign58810_e91717_d_n5, assign58810_e91717_d_n6, assign58810_e91717_d_n7, assign58810_e91717_d_n8, assign58810_e91717_d_n9, assign58810_e91717_d_n10, assign58810_e91717_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58810_e91714: f64 = (locals.var_cox * locals.var_cox);
        let assign58810_e91715: f64 = (locals.var_qnsub_esi / assign58810_e91714);
        (assign58810_e91715, (((locals.var_qnsub_esi_dn0 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn2 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn4 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn5 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn6 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn7 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn8 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn9 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn10 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign58810_e91714 * assign58810_e91714)), (((locals.var_qnsub_esi_dn13 * assign58810_e91714) - (locals.var_qnsub_esi * ((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)))) / (assign58810_e91714 * assign58810_e91714)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign58810_e91717;
        locals.var_t2_dn0 = assign58810_e91717_d_n0;
        locals.var_t2_dn2 = assign58810_e91717_d_n2;
        locals.var_t2_dn4 = assign58810_e91717_d_n4;
        locals.var_t2_dn5 = assign58810_e91717_d_n5;
        locals.var_t2_dn6 = assign58810_e91717_d_n6;
        locals.var_t2_dn7 = assign58810_e91717_d_n7;
        locals.var_t2_dn8 = assign58810_e91717_d_n8;
        locals.var_t2_dn9 = assign58810_e91717_d_n9;
        locals.var_t2_dn10 = assign58810_e91717_d_n10;
        locals.var_t2_dn13 = assign58810_e91717_d_n13;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_209(
        locals: &mut StampLocals,
    ) {
        let (assign58820_e91728, assign58820_e91728_d_n0, assign58820_e91728_d_n2, assign58820_e91728_d_n4, assign58820_e91728_d_n5, assign58820_e91728_d_n6, assign58820_e91728_d_n7, assign58820_e91728_d_n8, assign58820_e91728_d_n9, assign58820_e91728_d_n10, assign58820_e91728_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58820_e91724: f64 = (locals.var_vgp - locals.var_beta_inv);
        let assign58820_e91726: f64 = (assign58820_e91724 - locals.var_vbsz__blk438);
        (assign58820_e91726, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_vgp_dn13 - locals.var_beta_inv_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign58820_e91728;
        locals.var_t0_dn0 = assign58820_e91728_d_n0;
        locals.var_t0_dn2 = assign58820_e91728_d_n2;
        locals.var_t0_dn4 = assign58820_e91728_d_n4;
        locals.var_t0_dn5 = assign58820_e91728_d_n5;
        locals.var_t0_dn6 = assign58820_e91728_d_n6;
        locals.var_t0_dn7 = assign58820_e91728_d_n7;
        locals.var_t0_dn8 = assign58820_e91728_d_n8;
        locals.var_t0_dn9 = assign58820_e91728_d_n9;
        locals.var_t0_dn10 = assign58820_e91728_d_n10;
        locals.var_t0_dn13 = assign58820_e91728_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign58830_e91741, assign58830_e91741_d_n0, assign58830_e91741_d_n2, assign58830_e91741_d_n4, assign58830_e91741_d_n5, assign58830_e91741_d_n6, assign58830_e91741_d_n7, assign58830_e91741_d_n8, assign58830_e91741_d_n9, assign58830_e91741_d_n10, assign58830_e91741_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58830_e91736: f64 = (2.0 / locals.var_t2);
        let assign58830_e91738: f64 = (assign58830_e91736 * locals.var_t0);
        let assign58830_e91739: f64 = (1.0 + assign58830_e91738);
        (assign58830_e91739, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58830_e91736 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign58830_e91741;
        locals.var_t1_dn0 = assign58830_e91741_d_n0;
        locals.var_t1_dn2 = assign58830_e91741_d_n2;
        locals.var_t1_dn4 = assign58830_e91741_d_n4;
        locals.var_t1_dn5 = assign58830_e91741_d_n5;
        locals.var_t1_dn6 = assign58830_e91741_d_n6;
        locals.var_t1_dn7 = assign58830_e91741_d_n7;
        locals.var_t1_dn8 = assign58830_e91741_d_n8;
        locals.var_t1_dn9 = assign58830_e91741_d_n9;
        locals.var_t1_dn10 = assign58830_e91741_d_n10;
        locals.var_t1_dn13 = assign58830_e91741_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign58840_e91757, assign58840_e91757_d_n0, assign58840_e91757_d_n2, assign58840_e91757_d_n4, assign58840_e91757_d_n5, assign58840_e91757_d_n6, assign58840_e91757_d_n7, assign58840_e91757_d_n8, assign58840_e91757_d_n9, assign58840_e91757_d_n10, assign58840_e91757_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58840_e91748: f64 = (locals.var_t1 * locals.var_t1);
        let assign58840_e91751: f64 = (4.0 * 0.001);
        let assign58840_e91753: f64 = (assign58840_e91751 * 0.001);
        let assign58840_e91754: f64 = (assign58840_e91748 + assign58840_e91753);
        let assign58840_e91755: f64 = (assign58840_e91754).sqrt();
        (assign58840_e91755, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign58840_e91755)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign58840_e91755)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign58840_e91757;
        locals.var_tmf2_dn0 = assign58840_e91757_d_n0;
        locals.var_tmf2_dn2 = assign58840_e91757_d_n2;
        locals.var_tmf2_dn4 = assign58840_e91757_d_n4;
        locals.var_tmf2_dn5 = assign58840_e91757_d_n5;
        locals.var_tmf2_dn6 = assign58840_e91757_d_n6;
        locals.var_tmf2_dn7 = assign58840_e91757_d_n7;
        locals.var_tmf2_dn8 = assign58840_e91757_d_n8;
        locals.var_tmf2_dn9 = assign58840_e91757_d_n9;
        locals.var_tmf2_dn10 = assign58840_e91757_d_n10;
        locals.var_tmf2_dn13 = assign58840_e91757_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign58850_e91770, assign58850_e91770_d_n0, assign58850_e91770_d_n2, assign58850_e91770_d_n4, assign58850_e91770_d_n5, assign58850_e91770_d_n6, assign58850_e91770_d_n7, assign58850_e91770_d_n8, assign58850_e91770_d_n9, assign58850_e91770_d_n10, assign58850_e91770_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58850_e91766: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign58850_e91767: f64 = (1.0 + assign58850_e91766);
        let assign58850_e91768: f64 = (0.5 * assign58850_e91767);
        (assign58850_e91768, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58850_e91770;
        locals.var_tx_dn0 = assign58850_e91770_d_n0;
        locals.var_tx_dn2 = assign58850_e91770_d_n2;
        locals.var_tx_dn4 = assign58850_e91770_d_n4;
        locals.var_tx_dn5 = assign58850_e91770_d_n5;
        locals.var_tx_dn6 = assign58850_e91770_d_n6;
        locals.var_tx_dn7 = assign58850_e91770_d_n7;
        locals.var_tx_dn8 = assign58850_e91770_d_n8;
        locals.var_tx_dn9 = assign58850_e91770_d_n9;
        locals.var_tx_dn10 = assign58850_e91770_d_n10;
        locals.var_tx_dn13 = assign58850_e91770_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign58860_e91781, assign58860_e91781_d_n0, assign58860_e91781_d_n2, assign58860_e91781_d_n4, assign58860_e91781_d_n5, assign58860_e91781_d_n6, assign58860_e91781_d_n7, assign58860_e91781_d_n8, assign58860_e91781_d_n9, assign58860_e91781_d_n10, assign58860_e91781_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58860_e91778: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign58860_e91779: f64 = (0.5 * assign58860_e91778);
        (assign58860_e91779, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign58860_e91781;
        locals.var_t9_dn0 = assign58860_e91781_d_n0;
        locals.var_t9_dn2 = assign58860_e91781_d_n2;
        locals.var_t9_dn4 = assign58860_e91781_d_n4;
        locals.var_t9_dn5 = assign58860_e91781_d_n5;
        locals.var_t9_dn6 = assign58860_e91781_d_n6;
        locals.var_t9_dn7 = assign58860_e91781_d_n7;
        locals.var_t9_dn8 = assign58860_e91781_d_n8;
        locals.var_t9_dn9 = assign58860_e91781_d_n9;
        locals.var_t9_dn10 = assign58860_e91781_d_n10;
        locals.var_t9_dn13 = assign58860_e91781_d_n13;
        locals.var_t9_rv = 0.0;

        let assign58870_e91784: f64 = if locals.var_t9 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1445 = assign58870_e91784;
        locals.var_guard1445_rv = 0.0;

        let (assign58880_e91793, assign58880_e91793_d_n0, assign58880_e91793_d_n2, assign58880_e91793_d_n4, assign58880_e91793_d_n5, assign58880_e91793_d_n6, assign58880_e91793_d_n7, assign58880_e91793_d_n8, assign58880_e91793_d_n9, assign58880_e91793_d_n10, assign58880_e91793_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1445 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign58880_e91793;
        locals.var_t9_dn0 = assign58880_e91793_d_n0;
        locals.var_t9_dn2 = assign58880_e91793_d_n2;
        locals.var_t9_dn4 = assign58880_e91793_d_n4;
        locals.var_t9_dn5 = assign58880_e91793_d_n5;
        locals.var_t9_dn6 = assign58880_e91793_d_n6;
        locals.var_t9_dn7 = assign58880_e91793_d_n7;
        locals.var_t9_dn8 = assign58880_e91793_d_n8;
        locals.var_t9_dn9 = assign58880_e91793_d_n9;
        locals.var_t9_dn10 = assign58880_e91793_d_n10;
        locals.var_t9_dn13 = assign58880_e91793_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign58890_e91802, assign58890_e91802_d_n0, assign58890_e91802_d_n2, assign58890_e91802_d_n4, assign58890_e91802_d_n5, assign58890_e91802_d_n6, assign58890_e91802_d_n7, assign58890_e91802_d_n8, assign58890_e91802_d_n9, assign58890_e91802_d_n10, assign58890_e91802_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1445 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign58890_e91802;
        locals.var_tx_dn0 = assign58890_e91802_d_n0;
        locals.var_tx_dn2 = assign58890_e91802_d_n2;
        locals.var_tx_dn4 = assign58890_e91802_d_n4;
        locals.var_tx_dn5 = assign58890_e91802_d_n5;
        locals.var_tx_dn6 = assign58890_e91802_d_n6;
        locals.var_tx_dn7 = assign58890_e91802_d_n7;
        locals.var_tx_dn8 = assign58890_e91802_d_n8;
        locals.var_tx_dn9 = assign58890_e91802_d_n9;
        locals.var_tx_dn10 = assign58890_e91802_d_n10;
        locals.var_tx_dn13 = assign58890_e91802_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign58900_e91811, assign58900_e91811_d_n0, assign58900_e91811_d_n2, assign58900_e91811_d_n4, assign58900_e91811_d_n5, assign58900_e91811_d_n6, assign58900_e91811_d_n7, assign58900_e91811_d_n8, assign58900_e91811_d_n9, assign58900_e91811_d_n10, assign58900_e91811_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58900_e91809: f64 = (locals.var_t9 + 1e-25);
        (assign58900_e91809, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign58900_e91811;
        locals.var_t9_dn0 = assign58900_e91811_d_n0;
        locals.var_t9_dn2 = assign58900_e91811_d_n2;
        locals.var_t9_dn4 = assign58900_e91811_d_n4;
        locals.var_t9_dn5 = assign58900_e91811_d_n5;
        locals.var_t9_dn6 = assign58900_e91811_d_n6;
        locals.var_t9_dn7 = assign58900_e91811_d_n7;
        locals.var_t9_dn8 = assign58900_e91811_d_n8;
        locals.var_t9_dn9 = assign58900_e91811_d_n9;
        locals.var_t9_dn10 = assign58900_e91811_d_n10;
        locals.var_t9_dn13 = assign58900_e91811_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign58910_e91819, assign58910_e91819_d_n0, assign58910_e91819_d_n2, assign58910_e91819_d_n4, assign58910_e91819_d_n5, assign58910_e91819_d_n6, assign58910_e91819_d_n7, assign58910_e91819_d_n8, assign58910_e91819_d_n9, assign58910_e91819_d_n10, assign58910_e91819_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58910_e91817: f64 = (locals.var_t9).sqrt();
        (assign58910_e91817, (locals.var_t9_dn0 / (2.0 * assign58910_e91817)), (locals.var_t9_dn2 / (2.0 * assign58910_e91817)), (locals.var_t9_dn4 / (2.0 * assign58910_e91817)), (locals.var_t9_dn5 / (2.0 * assign58910_e91817)), (locals.var_t9_dn6 / (2.0 * assign58910_e91817)), (locals.var_t9_dn7 / (2.0 * assign58910_e91817)), (locals.var_t9_dn8 / (2.0 * assign58910_e91817)), (locals.var_t9_dn9 / (2.0 * assign58910_e91817)), (locals.var_t9_dn10 / (2.0 * assign58910_e91817)), (locals.var_t9_dn13 / (2.0 * assign58910_e91817)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign58910_e91819;
        locals.var_t3_dn0 = assign58910_e91819_d_n0;
        locals.var_t3_dn2 = assign58910_e91819_d_n2;
        locals.var_t3_dn4 = assign58910_e91819_d_n4;
        locals.var_t3_dn5 = assign58910_e91819_d_n5;
        locals.var_t3_dn6 = assign58910_e91819_d_n6;
        locals.var_t3_dn7 = assign58910_e91819_d_n7;
        locals.var_t3_dn8 = assign58910_e91819_d_n8;
        locals.var_t3_dn9 = assign58910_e91819_d_n9;
        locals.var_t3_dn10 = assign58910_e91819_d_n10;
        locals.var_t3_dn13 = assign58910_e91819_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign58920_e91832, assign58920_e91832_d_n0, assign58920_e91832_d_n2, assign58920_e91832_d_n4, assign58920_e91832_d_n5, assign58920_e91832_d_n6, assign58920_e91832_d_n7, assign58920_e91832_d_n8, assign58920_e91832_d_n9, assign58920_e91832_d_n10, assign58920_e91832_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58920_e91828: f64 = (1.0 - locals.var_t3);
        let assign58920_e91829: f64 = (locals.var_t2 * assign58920_e91828);
        let assign58920_e91830: f64 = (locals.var_vgp + assign58920_e91829);
        (assign58920_e91830, (locals.var_vgp_dn0 + ((locals.var_t2_dn0 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2_dn2 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_t2_dn4 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_t2_dn5 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_t2_dn6 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2_dn7 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_t2_dn8 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_t2_dn9 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_t2_dn10 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn13 + ((locals.var_t2_dn13 * assign58920_e91828) + (locals.var_t2 * (-locals.var_t3_dn13)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign58920_e91832;
        locals.var_t10_dn0 = assign58920_e91832_d_n0;
        locals.var_t10_dn2 = assign58920_e91832_d_n2;
        locals.var_t10_dn4 = assign58920_e91832_d_n4;
        locals.var_t10_dn5 = assign58920_e91832_d_n5;
        locals.var_t10_dn6 = assign58920_e91832_d_n6;
        locals.var_t10_dn7 = assign58920_e91832_d_n7;
        locals.var_t10_dn8 = assign58920_e91832_d_n8;
        locals.var_t10_dn9 = assign58920_e91832_d_n9;
        locals.var_t10_dn10 = assign58920_e91832_d_n10;
        locals.var_t10_dn13 = assign58920_e91832_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign58930_e91848, assign58930_e91848_d_n0, assign58930_e91848_d_n2, assign58930_e91848_d_n4, assign58930_e91848_d_n5, assign58930_e91848_d_n6, assign58930_e91848_d_n7, assign58930_e91848_d_n8, assign58930_e91848_d_n9, assign58930_e91848_d_n10, assign58930_e91848_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58930_e91839: f64 = (locals.var_t10 * locals.var_t10);
        let assign58930_e91842: f64 = (4.0 * 0.01);
        let assign58930_e91844: f64 = (assign58930_e91842 * 0.01);
        let assign58930_e91845: f64 = (assign58930_e91839 + assign58930_e91844);
        let assign58930_e91846: f64 = (assign58930_e91845).sqrt();
        (assign58930_e91846, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign58930_e91846)), (((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)) / (2.0 * assign58930_e91846)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign58930_e91848;
        locals.var_tmf2_dn0 = assign58930_e91848_d_n0;
        locals.var_tmf2_dn2 = assign58930_e91848_d_n2;
        locals.var_tmf2_dn4 = assign58930_e91848_d_n4;
        locals.var_tmf2_dn5 = assign58930_e91848_d_n5;
        locals.var_tmf2_dn6 = assign58930_e91848_d_n6;
        locals.var_tmf2_dn7 = assign58930_e91848_d_n7;
        locals.var_tmf2_dn8 = assign58930_e91848_d_n8;
        locals.var_tmf2_dn9 = assign58930_e91848_d_n9;
        locals.var_tmf2_dn10 = assign58930_e91848_d_n10;
        locals.var_tmf2_dn13 = assign58930_e91848_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign58940_e91861, assign58940_e91861_d_n0, assign58940_e91861_d_n2, assign58940_e91861_d_n4, assign58940_e91861_d_n5, assign58940_e91861_d_n6, assign58940_e91861_d_n7, assign58940_e91861_d_n8, assign58940_e91861_d_n9, assign58940_e91861_d_n10, assign58940_e91861_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58940_e91857: f64 = (locals.var_t10 / locals.var_tmf2);
        let assign58940_e91858: f64 = (1.0 + assign58940_e91857);
        let assign58940_e91859: f64 = (0.5 * assign58940_e91858);
        (assign58940_e91859, (0.5 * (((locals.var_t10_dn0 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn2 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn4 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn5 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn6 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn7 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn8 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn9 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn10 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn13 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign58940_e91861;
        locals.var_t0_dn0 = assign58940_e91861_d_n0;
        locals.var_t0_dn2 = assign58940_e91861_d_n2;
        locals.var_t0_dn4 = assign58940_e91861_d_n4;
        locals.var_t0_dn5 = assign58940_e91861_d_n5;
        locals.var_t0_dn6 = assign58940_e91861_d_n6;
        locals.var_t0_dn7 = assign58940_e91861_d_n7;
        locals.var_t0_dn8 = assign58940_e91861_d_n8;
        locals.var_t0_dn9 = assign58940_e91861_d_n9;
        locals.var_t0_dn10 = assign58940_e91861_d_n10;
        locals.var_t0_dn13 = assign58940_e91861_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign58950_e91872, assign58950_e91872_d_n0, assign58950_e91872_d_n2, assign58950_e91872_d_n4, assign58950_e91872_d_n5, assign58950_e91872_d_n6, assign58950_e91872_d_n7, assign58950_e91872_d_n8, assign58950_e91872_d_n9, assign58950_e91872_d_n10, assign58950_e91872_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58950_e91869: f64 = (locals.var_t10 + locals.var_tmf2);
        let assign58950_e91870: f64 = (0.5 * assign58950_e91869);
        (assign58950_e91870, (0.5 * (locals.var_t10_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t10_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t10_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t10_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t10_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t10_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign58950_e91872;
        locals.var_t10_dn0 = assign58950_e91872_d_n0;
        locals.var_t10_dn2 = assign58950_e91872_d_n2;
        locals.var_t10_dn4 = assign58950_e91872_d_n4;
        locals.var_t10_dn5 = assign58950_e91872_d_n5;
        locals.var_t10_dn6 = assign58950_e91872_d_n6;
        locals.var_t10_dn7 = assign58950_e91872_d_n7;
        locals.var_t10_dn8 = assign58950_e91872_d_n8;
        locals.var_t10_dn9 = assign58950_e91872_d_n9;
        locals.var_t10_dn10 = assign58950_e91872_d_n10;
        locals.var_t10_dn13 = assign58950_e91872_d_n13;
        locals.var_t10_rv = 0.0;

        let assign58960_e91875: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1446 = assign58960_e91875;
        locals.var_guard1446_rv = 0.0;

        let (assign58970_e91884, assign58970_e91884_d_n0, assign58970_e91884_d_n2, assign58970_e91884_d_n4, assign58970_e91884_d_n5, assign58970_e91884_d_n6, assign58970_e91884_d_n7, assign58970_e91884_d_n8, assign58970_e91884_d_n9, assign58970_e91884_d_n10, assign58970_e91884_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign58970_e91884;
        locals.var_t10_dn0 = assign58970_e91884_d_n0;
        locals.var_t10_dn2 = assign58970_e91884_d_n2;
        locals.var_t10_dn4 = assign58970_e91884_d_n4;
        locals.var_t10_dn5 = assign58970_e91884_d_n5;
        locals.var_t10_dn6 = assign58970_e91884_d_n6;
        locals.var_t10_dn7 = assign58970_e91884_d_n7;
        locals.var_t10_dn8 = assign58970_e91884_d_n8;
        locals.var_t10_dn9 = assign58970_e91884_d_n9;
        locals.var_t10_dn10 = assign58970_e91884_d_n10;
        locals.var_t10_dn13 = assign58970_e91884_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign58980_e91893, assign58980_e91893_d_n0, assign58980_e91893_d_n2, assign58980_e91893_d_n4, assign58980_e91893_d_n5, assign58980_e91893_d_n6, assign58980_e91893_d_n7, assign58980_e91893_d_n8, assign58980_e91893_d_n9, assign58980_e91893_d_n10, assign58980_e91893_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign58980_e91893;
        locals.var_t0_dn0 = assign58980_e91893_d_n0;
        locals.var_t0_dn2 = assign58980_e91893_d_n2;
        locals.var_t0_dn4 = assign58980_e91893_d_n4;
        locals.var_t0_dn5 = assign58980_e91893_d_n5;
        locals.var_t0_dn6 = assign58980_e91893_d_n6;
        locals.var_t0_dn7 = assign58980_e91893_d_n7;
        locals.var_t0_dn8 = assign58980_e91893_d_n8;
        locals.var_t0_dn9 = assign58980_e91893_d_n9;
        locals.var_t0_dn10 = assign58980_e91893_d_n10;
        locals.var_t0_dn13 = assign58980_e91893_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign58990_e91904, assign58990_e91904_d_n0, assign58990_e91904_d_n2, assign58990_e91904_d_n4, assign58990_e91904_d_n5, assign58990_e91904_d_n6, assign58990_e91904_d_n7, assign58990_e91904_d_n8, assign58990_e91904_d_n9, assign58990_e91904_d_n10, assign58990_e91904_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58990_e91901: f64 = (10.0 * 2.220446049250313e-16);
        let assign58990_e91902: f64 = (locals.var_t10 + assign58990_e91901);
        (assign58990_e91902, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign58990_e91904;
        locals.var_t10_dn0 = assign58990_e91904_d_n0;
        locals.var_t10_dn2 = assign58990_e91904_d_n2;
        locals.var_t10_dn4 = assign58990_e91904_d_n4;
        locals.var_t10_dn5 = assign58990_e91904_d_n5;
        locals.var_t10_dn6 = assign58990_e91904_d_n6;
        locals.var_t10_dn7 = assign58990_e91904_d_n7;
        locals.var_t10_dn8 = assign58990_e91904_d_n8;
        locals.var_t10_dn9 = assign58990_e91904_d_n9;
        locals.var_t10_dn10 = assign58990_e91904_d_n10;
        locals.var_t10_dn13 = assign58990_e91904_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign59000_e91913, assign59000_e91913_d_n0, assign59000_e91913_d_n2, assign59000_e91913_d_n4, assign59000_e91913_d_n5, assign59000_e91913_d_n6, assign59000_e91913_d_n7, assign59000_e91913_d_n8, assign59000_e91913_d_n9, assign59000_e91913_d_n10, assign59000_e91913_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59000_e91911: f64 = (locals.var_vds / locals.var_t10);
        (assign59000_e91911, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn4 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn5 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn8 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn9 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn13 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59000_e91913;
        locals.var_t1_dn0 = assign59000_e91913_d_n0;
        locals.var_t1_dn2 = assign59000_e91913_d_n2;
        locals.var_t1_dn4 = assign59000_e91913_d_n4;
        locals.var_t1_dn5 = assign59000_e91913_d_n5;
        locals.var_t1_dn6 = assign59000_e91913_d_n6;
        locals.var_t1_dn7 = assign59000_e91913_d_n7;
        locals.var_t1_dn8 = assign59000_e91913_d_n8;
        locals.var_t1_dn9 = assign59000_e91913_d_n9;
        locals.var_t1_dn10 = assign59000_e91913_d_n10;
        locals.var_t1_dn13 = assign59000_e91913_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59010_e91929, assign59010_e91929_d_n0, assign59010_e91929_d_n2, assign59010_e91929_d_n4, assign59010_e91929_d_n5, assign59010_e91929_d_n6, assign59010_e91929_d_n7, assign59010_e91929_d_n8, assign59010_e91929_d_n9, assign59010_e91929_d_n10, assign59010_e91929_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let (assign59010_e91927, assign59010_e91927_d_n0, assign59010_e91927_d_n2, assign59010_e91927_d_n4, assign59010_e91927_d_n5, assign59010_e91927_d_n6, assign59010_e91927_d_n7, assign59010_e91927_d_n8, assign59010_e91927_d_n9, assign59010_e91927_d_n10, assign59010_e91927_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59010_e91925: f64 = (locals.var_ddlte - 1.0);
                let assign59010_e91926: f64 = (locals.var_t1).powf(assign59010_e91925);
                (assign59010_e91926, if locals.var_ddlte_dn0 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn0)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn2)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn4)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn5)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn6)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn7)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn8)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn9)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn10)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn13 == 0.0 && ((assign59010_e91925) as f64).is_finite() && ((assign59010_e91925) as f64).fract() == 0.0 { if assign59010_e91925 == 0.0 { 0.0 } else { (assign59010_e91925 * ((locals.var_t1).powf(assign59010_e91925 - 1.0) * locals.var_t1_dn13)) } } else { (assign59010_e91926 * ((locals.var_ddlte_dn13 * (locals.var_t1).ln()) + (assign59010_e91925 * (locals.var_t1_dn13 / locals.var_t1)))) },)
            }
        };
        (assign59010_e91927, assign59010_e91927_d_n0, assign59010_e91927_d_n2, assign59010_e91927_d_n4, assign59010_e91927_d_n5, assign59010_e91927_d_n6, assign59010_e91927_d_n7, assign59010_e91927_d_n8, assign59010_e91927_d_n9, assign59010_e91927_d_n10, assign59010_e91927_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign59010_e91929;
        locals.var_t2_dn0 = assign59010_e91929_d_n0;
        locals.var_t2_dn2 = assign59010_e91929_d_n2;
        locals.var_t2_dn4 = assign59010_e91929_d_n4;
        locals.var_t2_dn5 = assign59010_e91929_d_n5;
        locals.var_t2_dn6 = assign59010_e91929_d_n6;
        locals.var_t2_dn7 = assign59010_e91929_d_n7;
        locals.var_t2_dn8 = assign59010_e91929_d_n8;
        locals.var_t2_dn9 = assign59010_e91929_d_n9;
        locals.var_t2_dn10 = assign59010_e91929_d_n10;
        locals.var_t2_dn13 = assign59010_e91929_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign59020_e91940, assign59020_e91940_d_n0, assign59020_e91940_d_n2, assign59020_e91940_d_n4, assign59020_e91940_d_n5, assign59020_e91940_d_n6, assign59020_e91940_d_n7, assign59020_e91940_d_n8, assign59020_e91940_d_n9, assign59020_e91940_d_n10, assign59020_e91940_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59020_e91937: f64 = (locals.var_t2 * locals.var_t1);
        let assign59020_e91938: f64 = (1.0 + assign59020_e91937);
        (assign59020_e91938, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn13 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign59020_e91940;
        locals.var_t3_dn0 = assign59020_e91940_d_n0;
        locals.var_t3_dn2 = assign59020_e91940_d_n2;
        locals.var_t3_dn4 = assign59020_e91940_d_n4;
        locals.var_t3_dn5 = assign59020_e91940_d_n5;
        locals.var_t3_dn6 = assign59020_e91940_d_n6;
        locals.var_t3_dn7 = assign59020_e91940_d_n7;
        locals.var_t3_dn8 = assign59020_e91940_d_n8;
        locals.var_t3_dn9 = assign59020_e91940_d_n9;
        locals.var_t3_dn10 = assign59020_e91940_d_n10;
        locals.var_t3_dn13 = assign59020_e91940_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign59030_e91958, assign59030_e91958_d_n0, assign59030_e91958_d_n2, assign59030_e91958_d_n4, assign59030_e91958_d_n5, assign59030_e91958_d_n6, assign59030_e91958_d_n7, assign59030_e91958_d_n8, assign59030_e91958_d_n9, assign59030_e91958_d_n10, assign59030_e91958_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let (assign59030_e91956, assign59030_e91956_d_n0, assign59030_e91956_d_n2, assign59030_e91956_d_n4, assign59030_e91956_d_n5, assign59030_e91956_d_n6, assign59030_e91956_d_n7, assign59030_e91956_d_n8, assign59030_e91956_d_n9, assign59030_e91956_d_n10, assign59030_e91956_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59030_e91952: f64 = (1.0 / locals.var_ddlte);
                let assign59030_e91954: f64 = (assign59030_e91952 - 1.0);
                let assign59030_e91955: f64 = (locals.var_t3).powf(assign59030_e91954);
                (assign59030_e91955, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn0)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn2)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn4)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn5)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn6)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn7)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn8)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn9)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn10)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn13 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59030_e91954) as f64).is_finite() && ((assign59030_e91954) as f64).fract() == 0.0 { if assign59030_e91954 == 0.0 { 0.0 } else { (assign59030_e91954 * ((locals.var_t3).powf(assign59030_e91954 - 1.0) * locals.var_t3_dn13)) } } else { (assign59030_e91955 * (((-(locals.var_ddlte_dn13 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59030_e91954 * (locals.var_t3_dn13 / locals.var_t3)))) },)
            }
        };
        (assign59030_e91956, assign59030_e91956_d_n0, assign59030_e91956_d_n2, assign59030_e91956_d_n4, assign59030_e91956_d_n5, assign59030_e91956_d_n6, assign59030_e91956_d_n7, assign59030_e91956_d_n8, assign59030_e91956_d_n9, assign59030_e91956_d_n10, assign59030_e91956_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign59030_e91958;
        locals.var_t4_dn0 = assign59030_e91958_d_n0;
        locals.var_t4_dn2 = assign59030_e91958_d_n2;
        locals.var_t4_dn4 = assign59030_e91958_d_n4;
        locals.var_t4_dn5 = assign59030_e91958_d_n5;
        locals.var_t4_dn6 = assign59030_e91958_d_n6;
        locals.var_t4_dn7 = assign59030_e91958_d_n7;
        locals.var_t4_dn8 = assign59030_e91958_d_n8;
        locals.var_t4_dn9 = assign59030_e91958_d_n9;
        locals.var_t4_dn10 = assign59030_e91958_d_n10;
        locals.var_t4_dn13 = assign59030_e91958_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign59040_e91967, assign59040_e91967_d_n0, assign59040_e91967_d_n2, assign59040_e91967_d_n4, assign59040_e91967_d_n5, assign59040_e91967_d_n6, assign59040_e91967_d_n7, assign59040_e91967_d_n8, assign59040_e91967_d_n9, assign59040_e91967_d_n10, assign59040_e91967_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59040_e91965: f64 = (locals.var_t4 * locals.var_t3);
        (assign59040_e91965, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn13 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign59040_e91967;
        locals.var_t6_dn0 = assign59040_e91967_d_n0;
        locals.var_t6_dn2 = assign59040_e91967_d_n2;
        locals.var_t6_dn4 = assign59040_e91967_d_n4;
        locals.var_t6_dn5 = assign59040_e91967_d_n5;
        locals.var_t6_dn6 = assign59040_e91967_d_n6;
        locals.var_t6_dn7 = assign59040_e91967_d_n7;
        locals.var_t6_dn8 = assign59040_e91967_d_n8;
        locals.var_t6_dn9 = assign59040_e91967_d_n9;
        locals.var_t6_dn10 = assign59040_e91967_d_n10;
        locals.var_t6_dn13 = assign59040_e91967_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign59050_e91976, assign59050_e91976_d_n0, assign59050_e91976_d_n2, assign59050_e91976_d_n4, assign59050_e91976_d_n5, assign59050_e91976_d_n6, assign59050_e91976_d_n7, assign59050_e91976_d_n8, assign59050_e91976_d_n9, assign59050_e91976_d_n10, assign59050_e91976_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59050_e91974: f64 = (locals.var_vds / locals.var_t6);
        (assign59050_e91974, (((locals.var_vds_dn0 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn2 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn4 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn5 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn6 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn7 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn8 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn9 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn10 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn13 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn13,)
    }
};
        locals.var_vdseff = assign59050_e91976;
        locals.var_vdseff_dn0 = assign59050_e91976_d_n0;
        locals.var_vdseff_dn2 = assign59050_e91976_d_n2;
        locals.var_vdseff_dn4 = assign59050_e91976_d_n4;
        locals.var_vdseff_dn5 = assign59050_e91976_d_n5;
        locals.var_vdseff_dn6 = assign59050_e91976_d_n6;
        locals.var_vdseff_dn7 = assign59050_e91976_d_n7;
        locals.var_vdseff_dn8 = assign59050_e91976_d_n8;
        locals.var_vdseff_dn9 = assign59050_e91976_d_n9;
        locals.var_vdseff_dn10 = assign59050_e91976_d_n10;
        locals.var_vdseff_dn13 = assign59050_e91976_d_n13;
        locals.var_vdseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_210(
        locals: &mut StampLocals,
    ) {
        let (assign59060_e91983, assign59060_e91983_d_n0, assign59060_e91983_d_n2, assign59060_e91983_d_n4, assign59060_e91983_d_n5, assign59060_e91983_d_n6, assign59060_e91983_d_n7, assign59060_e91983_d_n8, assign59060_e91983_d_n9, assign59060_e91983_d_n10, assign59060_e91983_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign59060_e91983;
        locals.var_vds_dn0 = assign59060_e91983_d_n0;
        locals.var_vds_dn2 = assign59060_e91983_d_n2;
        locals.var_vds_dn4 = assign59060_e91983_d_n4;
        locals.var_vds_dn5 = assign59060_e91983_d_n5;
        locals.var_vds_dn6 = assign59060_e91983_d_n6;
        locals.var_vds_dn7 = assign59060_e91983_d_n7;
        locals.var_vds_dn8 = assign59060_e91983_d_n8;
        locals.var_vds_dn9 = assign59060_e91983_d_n9;
        locals.var_vds_dn10 = assign59060_e91983_d_n10;
        locals.var_vds_dn13 = assign59060_e91983_d_n13;
        locals.var_vds_rv = 0.0;

        let (assign59080_e92002, assign59080_e92002_d_n0, assign59080_e92002_d_n2, assign59080_e92002_d_n4, assign59080_e92002_d_n5, assign59080_e92002_d_n6, assign59080_e92002_d_n7, assign59080_e92002_d_n8, assign59080_e92002_d_n9, assign59080_e92002_d_n10, assign59080_e92002_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59080_e91998: f64 = (locals.var_vbscl__blk435 - locals.var_vds);
        let assign59080_e91999: f64 = (locals.var_beta * assign59080_e91998);
        let assign59080_e92000: f64 = (assign59080_e91999).exp();
        (assign59080_e92000, (assign59080_e92000 * ((locals.var_beta_dn0 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn0 - locals.var_vds_dn0)))), (assign59080_e92000 * ((locals.var_beta_dn2 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn2 - locals.var_vds_dn2)))), (assign59080_e92000 * ((locals.var_beta_dn4 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn4 - locals.var_vds_dn4)))), (assign59080_e92000 * ((locals.var_beta_dn5 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn5 - locals.var_vds_dn5)))), (assign59080_e92000 * ((locals.var_beta_dn6 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn6 - locals.var_vds_dn6)))), (assign59080_e92000 * ((locals.var_beta_dn7 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn7 - locals.var_vds_dn7)))), (assign59080_e92000 * ((locals.var_beta_dn8 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn8 - locals.var_vds_dn8)))), (assign59080_e92000 * ((locals.var_beta_dn9 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn9 - locals.var_vds_dn9)))), (assign59080_e92000 * ((locals.var_beta_dn10 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn10 - locals.var_vds_dn10)))), (assign59080_e92000 * ((locals.var_beta_dn13 * assign59080_e91998) + (locals.var_beta * (locals.var_vbscl__blk435_dn13 - locals.var_vds_dn13)))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn13,)
    }
};
        locals.var_exp_bvbsvds = assign59080_e92002;
        locals.var_exp_bvbsvds_dn0 = assign59080_e92002_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign59080_e92002_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign59080_e92002_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign59080_e92002_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign59080_e92002_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign59080_e92002_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign59080_e92002_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign59080_e92002_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign59080_e92002_d_n10;
        locals.var_exp_bvbsvds_dn13 = assign59080_e92002_d_n13;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign59090_e92005: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1447 = assign59090_e92005;
        locals.var_guard1447_rv = 0.0;

        let (assign59100_e92014, assign59100_e92014_d_n0, assign59100_e92014_d_n2, assign59100_e92014_d_n4, assign59100_e92014_d_n5, assign59100_e92014_d_n6, assign59100_e92014_d_n7, assign59100_e92014_d_n8, assign59100_e92014_d_n9, assign59100_e92014_d_n10, assign59100_e92014_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1447 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign59100_e92014;
        locals.var_pds_dn0 = assign59100_e92014_d_n0;
        locals.var_pds_dn2 = assign59100_e92014_d_n2;
        locals.var_pds_dn4 = assign59100_e92014_d_n4;
        locals.var_pds_dn5 = assign59100_e92014_d_n5;
        locals.var_pds_dn6 = assign59100_e92014_d_n6;
        locals.var_pds_dn7 = assign59100_e92014_d_n7;
        locals.var_pds_dn8 = assign59100_e92014_d_n8;
        locals.var_pds_dn9 = assign59100_e92014_d_n9;
        locals.var_pds_dn10 = assign59100_e92014_d_n10;
        locals.var_pds_dn13 = assign59100_e92014_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign59110_e92023, assign59110_e92023_d_n0, assign59110_e92023_d_n2, assign59110_e92023_d_n4, assign59110_e92023_d_n5, assign59110_e92023_d_n6, assign59110_e92023_d_n7, assign59110_e92023_d_n8, assign59110_e92023_d_n9, assign59110_e92023_d_n10, assign59110_e92023_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1447 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
        locals.var_psl = assign59110_e92023;
        locals.var_psl_dn0 = assign59110_e92023_d_n0;
        locals.var_psl_dn2 = assign59110_e92023_d_n2;
        locals.var_psl_dn4 = assign59110_e92023_d_n4;
        locals.var_psl_dn5 = assign59110_e92023_d_n5;
        locals.var_psl_dn6 = assign59110_e92023_d_n6;
        locals.var_psl_dn7 = assign59110_e92023_d_n7;
        locals.var_psl_dn8 = assign59110_e92023_d_n8;
        locals.var_psl_dn9 = assign59110_e92023_d_n9;
        locals.var_psl_dn10 = assign59110_e92023_d_n10;
        locals.var_psl_dn13 = assign59110_e92023_d_n13;
        locals.var_psl_rv = 0.0;

        let (assign59120_e92032,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1447 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59120_e92032;
        locals.var_start_of_loopl_rv = 0.0;

        let assign59130_e92035: f64 = if locals.var_start_of_loopl == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1448 = assign59130_e92035;
        locals.var_guard1448_rv = 0.0;

        let assign59140_e92038: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1449 = assign59140_e92038;
        locals.var_guard1449_rv = 0.0;

        let (assign59150_e92058, assign59150_e92058_d_n0, assign59150_e92058_d_n2, assign59150_e92058_d_n4, assign59150_e92058_d_n5, assign59150_e92058_d_n6, assign59150_e92058_d_n7, assign59150_e92058_d_n8, assign59150_e92058_d_n9, assign59150_e92058_d_n10, assign59150_e92058_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        let assign59150_e92049: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign59150_e92056, assign59150_e92056_d_n0, assign59150_e92056_d_n2, assign59150_e92056_d_n4, assign59150_e92056_d_n5, assign59150_e92056_d_n6, assign59150_e92056_d_n7, assign59150_e92056_d_n8, assign59150_e92056_d_n9, assign59150_e92056_d_n10, assign59150_e92056_d_n13,) = {
            if (assign59150_e92049 >= 0.0) {
                let assign59150_e92054: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign59150_e92054, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn4 - locals.var_ps0_dn4), (locals.var_psl_lim_dn5 - locals.var_ps0_dn5), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn8 - locals.var_ps0_dn8), (locals.var_psl_lim_dn9 - locals.var_ps0_dn9), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn13 - locals.var_ps0_dn13),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign59150_e92056, assign59150_e92056_d_n0, assign59150_e92056_d_n2, assign59150_e92056_d_n4, assign59150_e92056_d_n5, assign59150_e92056_d_n6, assign59150_e92056_d_n7, assign59150_e92056_d_n8, assign59150_e92056_d_n9, assign59150_e92056_d_n10, assign59150_e92056_d_n13,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn13,)
    }
};
        locals.var_pds_max = assign59150_e92058;
        locals.var_pds_max_dn0 = assign59150_e92058_d_n0;
        locals.var_pds_max_dn2 = assign59150_e92058_d_n2;
        locals.var_pds_max_dn4 = assign59150_e92058_d_n4;
        locals.var_pds_max_dn5 = assign59150_e92058_d_n5;
        locals.var_pds_max_dn6 = assign59150_e92058_d_n6;
        locals.var_pds_max_dn7 = assign59150_e92058_d_n7;
        locals.var_pds_max_dn8 = assign59150_e92058_d_n8;
        locals.var_pds_max_dn9 = assign59150_e92058_d_n9;
        locals.var_pds_max_dn10 = assign59150_e92058_d_n10;
        locals.var_pds_max_dn13 = assign59150_e92058_d_n13;
        locals.var_pds_max_rv = 0.0;

        let assign59160_e92061: f64 = (1.0 + 0.3);
        let assign59160_e92063: f64 = (assign59160_e92061 * locals.var_pds_max);
        let assign59160_e92065: f64 = if assign59160_e92063 > 0.03 { 1.0 } else { 0.0 };
        locals.var_guard1450 = assign59160_e92065;
        locals.var_guard1450_rv = 0.0;

        let (assign59170_e92086, assign59170_e92086_d_n0, assign59170_e92086_d_n2, assign59170_e92086_d_n4, assign59170_e92086_d_n5, assign59170_e92086_d_n6, assign59170_e92086_d_n7, assign59170_e92086_d_n8, assign59170_e92086_d_n9, assign59170_e92086_d_n10, assign59170_e92086_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let assign59170_e92078: f64 = (1.0 + 0.3);
        let assign59170_e92080: f64 = (assign59170_e92078 * locals.var_pds_max);
        let assign59170_e92082: f64 = (assign59170_e92080 - locals.var_vds);
        let assign59170_e92084: f64 = (assign59170_e92082 - 0.03);
        (assign59170_e92084, ((assign59170_e92078 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign59170_e92078 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign59170_e92078 * locals.var_pds_max_dn4) - locals.var_vds_dn4), ((assign59170_e92078 * locals.var_pds_max_dn5) - locals.var_vds_dn5), ((assign59170_e92078 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign59170_e92078 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign59170_e92078 * locals.var_pds_max_dn8) - locals.var_vds_dn8), ((assign59170_e92078 * locals.var_pds_max_dn9) - locals.var_vds_dn9), ((assign59170_e92078 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign59170_e92078 * locals.var_pds_max_dn13) - locals.var_vds_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign59170_e92086;
        locals.var_tmf1_dn0 = assign59170_e92086_d_n0;
        locals.var_tmf1_dn2 = assign59170_e92086_d_n2;
        locals.var_tmf1_dn4 = assign59170_e92086_d_n4;
        locals.var_tmf1_dn5 = assign59170_e92086_d_n5;
        locals.var_tmf1_dn6 = assign59170_e92086_d_n6;
        locals.var_tmf1_dn7 = assign59170_e92086_d_n7;
        locals.var_tmf1_dn8 = assign59170_e92086_d_n8;
        locals.var_tmf1_dn9 = assign59170_e92086_d_n9;
        locals.var_tmf1_dn10 = assign59170_e92086_d_n10;
        locals.var_tmf1_dn13 = assign59170_e92086_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign59180_e92107, assign59180_e92107_d_n0, assign59180_e92107_d_n2, assign59180_e92107_d_n4, assign59180_e92107_d_n5, assign59180_e92107_d_n6, assign59180_e92107_d_n7, assign59180_e92107_d_n8, assign59180_e92107_d_n9, assign59180_e92107_d_n10, assign59180_e92107_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let assign59180_e92100: f64 = (1.0 + 0.3);
        let assign59180_e92102: f64 = (assign59180_e92100 * locals.var_pds_max);
        let assign59180_e92103: f64 = (4.0 * assign59180_e92102);
        let assign59180_e92105: f64 = (assign59180_e92103 * 0.03);
        (assign59180_e92105, ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn4)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn5)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn8)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn9)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign59180_e92100 * locals.var_pds_max_dn13)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign59180_e92107;
        locals.var_tmf2_dn0 = assign59180_e92107_d_n0;
        locals.var_tmf2_dn2 = assign59180_e92107_d_n2;
        locals.var_tmf2_dn4 = assign59180_e92107_d_n4;
        locals.var_tmf2_dn5 = assign59180_e92107_d_n5;
        locals.var_tmf2_dn6 = assign59180_e92107_d_n6;
        locals.var_tmf2_dn7 = assign59180_e92107_d_n7;
        locals.var_tmf2_dn8 = assign59180_e92107_d_n8;
        locals.var_tmf2_dn9 = assign59180_e92107_d_n9;
        locals.var_tmf2_dn10 = assign59180_e92107_d_n10;
        locals.var_tmf2_dn13 = assign59180_e92107_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign59190_e92126, assign59190_e92126_d_n0, assign59190_e92126_d_n2, assign59190_e92126_d_n4, assign59190_e92126_d_n5, assign59190_e92126_d_n6, assign59190_e92126_d_n7, assign59190_e92126_d_n8, assign59190_e92126_d_n9, assign59190_e92126_d_n10, assign59190_e92126_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let (assign59190_e92124, assign59190_e92124_d_n0, assign59190_e92124_d_n2, assign59190_e92124_d_n4, assign59190_e92124_d_n5, assign59190_e92124_d_n6, assign59190_e92124_d_n7, assign59190_e92124_d_n8, assign59190_e92124_d_n9, assign59190_e92124_d_n10, assign59190_e92124_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign59190_e92123: f64 = (-locals.var_tmf2);
                (assign59190_e92123, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign59190_e92124, assign59190_e92124_d_n0, assign59190_e92124_d_n2, assign59190_e92124_d_n4, assign59190_e92124_d_n5, assign59190_e92124_d_n6, assign59190_e92124_d_n7, assign59190_e92124_d_n8, assign59190_e92124_d_n9, assign59190_e92124_d_n10, assign59190_e92124_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign59190_e92126;
        locals.var_tmf2_dn0 = assign59190_e92126_d_n0;
        locals.var_tmf2_dn2 = assign59190_e92126_d_n2;
        locals.var_tmf2_dn4 = assign59190_e92126_d_n4;
        locals.var_tmf2_dn5 = assign59190_e92126_d_n5;
        locals.var_tmf2_dn6 = assign59190_e92126_d_n6;
        locals.var_tmf2_dn7 = assign59190_e92126_d_n7;
        locals.var_tmf2_dn8 = assign59190_e92126_d_n8;
        locals.var_tmf2_dn9 = assign59190_e92126_d_n9;
        locals.var_tmf2_dn10 = assign59190_e92126_d_n10;
        locals.var_tmf2_dn13 = assign59190_e92126_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign59200_e92144, assign59200_e92144_d_n0, assign59200_e92144_d_n2, assign59200_e92144_d_n4, assign59200_e92144_d_n5, assign59200_e92144_d_n6, assign59200_e92144_d_n7, assign59200_e92144_d_n8, assign59200_e92144_d_n9, assign59200_e92144_d_n10, assign59200_e92144_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let assign59200_e92139: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign59200_e92141: f64 = (assign59200_e92139 + locals.var_tmf2);
        let assign59200_e92142: f64 = (assign59200_e92141).sqrt();
        (assign59200_e92142, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign59200_e92142)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign59200_e92142)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign59200_e92144;
        locals.var_tmf2_dn0 = assign59200_e92144_d_n0;
        locals.var_tmf2_dn2 = assign59200_e92144_d_n2;
        locals.var_tmf2_dn4 = assign59200_e92144_d_n4;
        locals.var_tmf2_dn5 = assign59200_e92144_d_n5;
        locals.var_tmf2_dn6 = assign59200_e92144_d_n6;
        locals.var_tmf2_dn7 = assign59200_e92144_d_n7;
        locals.var_tmf2_dn8 = assign59200_e92144_d_n8;
        locals.var_tmf2_dn9 = assign59200_e92144_d_n9;
        locals.var_tmf2_dn10 = assign59200_e92144_d_n10;
        locals.var_tmf2_dn13 = assign59200_e92144_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign59210_e92163, assign59210_e92163_d_n0, assign59210_e92163_d_n2, assign59210_e92163_d_n4, assign59210_e92163_d_n5, assign59210_e92163_d_n6, assign59210_e92163_d_n7, assign59210_e92163_d_n8, assign59210_e92163_d_n9, assign59210_e92163_d_n10, assign59210_e92163_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let assign59210_e92159: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign59210_e92160: f64 = (1.0 + assign59210_e92159);
        let assign59210_e92161: f64 = (0.5 * assign59210_e92160);
        (assign59210_e92161, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59210_e92163;
        locals.var_t1_dn0 = assign59210_e92163_d_n0;
        locals.var_t1_dn2 = assign59210_e92163_d_n2;
        locals.var_t1_dn4 = assign59210_e92163_d_n4;
        locals.var_t1_dn5 = assign59210_e92163_d_n5;
        locals.var_t1_dn6 = assign59210_e92163_d_n6;
        locals.var_t1_dn7 = assign59210_e92163_d_n7;
        locals.var_t1_dn8 = assign59210_e92163_d_n8;
        locals.var_t1_dn9 = assign59210_e92163_d_n9;
        locals.var_t1_dn10 = assign59210_e92163_d_n10;
        locals.var_t1_dn13 = assign59210_e92163_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59220_e92186, assign59220_e92186_d_n0, assign59220_e92186_d_n2, assign59220_e92186_d_n4, assign59220_e92186_d_n5, assign59220_e92186_d_n6, assign59220_e92186_d_n7, assign59220_e92186_d_n8, assign59220_e92186_d_n9, assign59220_e92186_d_n10, assign59220_e92186_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        let assign59220_e92176: f64 = (1.0 + 0.3);
        let assign59220_e92178: f64 = (assign59220_e92176 * locals.var_pds_max);
        let assign59220_e92182: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign59220_e92183: f64 = (0.5 * assign59220_e92182);
        let assign59220_e92184: f64 = (assign59220_e92178 - assign59220_e92183);
        (assign59220_e92184, ((assign59220_e92176 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign59220_e92176 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign59220_e92176 * locals.var_pds_max_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((assign59220_e92176 * locals.var_pds_max_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((assign59220_e92176 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign59220_e92176 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign59220_e92176 * locals.var_pds_max_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((assign59220_e92176 * locals.var_pds_max_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((assign59220_e92176 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign59220_e92176 * locals.var_pds_max_dn13) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    }
};
        locals.var_pds_ini = assign59220_e92186;
        locals.var_pds_ini_dn0 = assign59220_e92186_d_n0;
        locals.var_pds_ini_dn2 = assign59220_e92186_d_n2;
        locals.var_pds_ini_dn4 = assign59220_e92186_d_n4;
        locals.var_pds_ini_dn5 = assign59220_e92186_d_n5;
        locals.var_pds_ini_dn6 = assign59220_e92186_d_n6;
        locals.var_pds_ini_dn7 = assign59220_e92186_d_n7;
        locals.var_pds_ini_dn8 = assign59220_e92186_d_n8;
        locals.var_pds_ini_dn9 = assign59220_e92186_d_n9;
        locals.var_pds_ini_dn10 = assign59220_e92186_d_n10;
        locals.var_pds_ini_dn13 = assign59220_e92186_d_n13;
        locals.var_pds_ini_rv = 0.0;

        let (assign59230_e92204, assign59230_e92204_d_n0, assign59230_e92204_d_n2, assign59230_e92204_d_n4, assign59230_e92204_d_n5, assign59230_e92204_d_n6, assign59230_e92204_d_n7, assign59230_e92204_d_n8, assign59230_e92204_d_n9, assign59230_e92204_d_n10, assign59230_e92204_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) && (locals.var_guard1450 == 0.0)) {
        let assign59230_e92200: f64 = (1.0 + 0.3);
        let assign59230_e92202: f64 = (assign59230_e92200 * locals.var_pds_max);
        (assign59230_e92202, (assign59230_e92200 * locals.var_pds_max_dn0), (assign59230_e92200 * locals.var_pds_max_dn2), (assign59230_e92200 * locals.var_pds_max_dn4), (assign59230_e92200 * locals.var_pds_max_dn5), (assign59230_e92200 * locals.var_pds_max_dn6), (assign59230_e92200 * locals.var_pds_max_dn7), (assign59230_e92200 * locals.var_pds_max_dn8), (assign59230_e92200 * locals.var_pds_max_dn9), (assign59230_e92200 * locals.var_pds_max_dn10), (assign59230_e92200 * locals.var_pds_max_dn13),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    }
};
        locals.var_pds_ini = assign59230_e92204;
        locals.var_pds_ini_dn0 = assign59230_e92204_d_n0;
        locals.var_pds_ini_dn2 = assign59230_e92204_d_n2;
        locals.var_pds_ini_dn4 = assign59230_e92204_d_n4;
        locals.var_pds_ini_dn5 = assign59230_e92204_d_n5;
        locals.var_pds_ini_dn6 = assign59230_e92204_d_n6;
        locals.var_pds_ini_dn7 = assign59230_e92204_d_n7;
        locals.var_pds_ini_dn8 = assign59230_e92204_d_n8;
        locals.var_pds_ini_dn9 = assign59230_e92204_d_n9;
        locals.var_pds_ini_dn10 = assign59230_e92204_d_n10;
        locals.var_pds_ini_dn13 = assign59230_e92204_d_n13;
        locals.var_pds_ini_rv = 0.0;

        let (assign59240_e92220, assign59240_e92220_d_n0, assign59240_e92220_d_n2, assign59240_e92220_d_n4, assign59240_e92220_d_n5, assign59240_e92220_d_n6, assign59240_e92220_d_n7, assign59240_e92220_d_n8, assign59240_e92220_d_n9, assign59240_e92220_d_n10, assign59240_e92220_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        let (assign59240_e92218, assign59240_e92218_d_n0, assign59240_e92218_d_n2, assign59240_e92218_d_n4, assign59240_e92218_d_n5, assign59240_e92218_d_n6, assign59240_e92218_d_n7, assign59240_e92218_d_n8, assign59240_e92218_d_n9, assign59240_e92218_d_n10, assign59240_e92218_d_n13,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn13,)
            }
        };
        (assign59240_e92218, assign59240_e92218_d_n0, assign59240_e92218_d_n2, assign59240_e92218_d_n4, assign59240_e92218_d_n5, assign59240_e92218_d_n6, assign59240_e92218_d_n7, assign59240_e92218_d_n8, assign59240_e92218_d_n9, assign59240_e92218_d_n10, assign59240_e92218_d_n13,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    }
};
        locals.var_pds_ini = assign59240_e92220;
        locals.var_pds_ini_dn0 = assign59240_e92220_d_n0;
        locals.var_pds_ini_dn2 = assign59240_e92220_d_n2;
        locals.var_pds_ini_dn4 = assign59240_e92220_d_n4;
        locals.var_pds_ini_dn5 = assign59240_e92220_d_n5;
        locals.var_pds_ini_dn6 = assign59240_e92220_d_n6;
        locals.var_pds_ini_dn7 = assign59240_e92220_d_n7;
        locals.var_pds_ini_dn8 = assign59240_e92220_d_n8;
        locals.var_pds_ini_dn9 = assign59240_e92220_d_n9;
        locals.var_pds_ini_dn10 = assign59240_e92220_d_n10;
        locals.var_pds_ini_dn13 = assign59240_e92220_d_n13;
        locals.var_pds_ini_rv = 0.0;

        let assign59250_e92223: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1451 = assign59250_e92223;
        locals.var_guard1451_rv = 0.0;

        let (assign59260_e92234, assign59260_e92234_d_n0, assign59260_e92234_d_n2, assign59260_e92234_d_n4, assign59260_e92234_d_n5, assign59260_e92234_d_n6, assign59260_e92234_d_n7, assign59260_e92234_d_n8, assign59260_e92234_d_n9, assign59260_e92234_d_n10, assign59260_e92234_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    }
};
        locals.var_pds_ini = assign59260_e92234;
        locals.var_pds_ini_dn0 = assign59260_e92234_d_n0;
        locals.var_pds_ini_dn2 = assign59260_e92234_d_n2;
        locals.var_pds_ini_dn4 = assign59260_e92234_d_n4;
        locals.var_pds_ini_dn5 = assign59260_e92234_d_n5;
        locals.var_pds_ini_dn6 = assign59260_e92234_d_n6;
        locals.var_pds_ini_dn7 = assign59260_e92234_d_n7;
        locals.var_pds_ini_dn8 = assign59260_e92234_d_n8;
        locals.var_pds_ini_dn9 = assign59260_e92234_d_n9;
        locals.var_pds_ini_dn10 = assign59260_e92234_d_n10;
        locals.var_pds_ini_dn13 = assign59260_e92234_d_n13;
        locals.var_pds_ini_rv = 0.0;

        let assign59270_e92237: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard1452 = assign59270_e92237;
        locals.var_guard1452_rv = 0.0;

        let (assign59280_e92251, assign59280_e92251_d_n0, assign59280_e92251_d_n2, assign59280_e92251_d_n4, assign59280_e92251_d_n5, assign59280_e92251_d_n6, assign59280_e92251_d_n7, assign59280_e92251_d_n8, assign59280_e92251_d_n9, assign59280_e92251_d_n10, assign59280_e92251_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) && (locals.var_guard1451 == 0.0)) && (locals.var_guard1452 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    }
};
        locals.var_pds_ini = assign59280_e92251;
        locals.var_pds_ini_dn0 = assign59280_e92251_d_n0;
        locals.var_pds_ini_dn2 = assign59280_e92251_d_n2;
        locals.var_pds_ini_dn4 = assign59280_e92251_d_n4;
        locals.var_pds_ini_dn5 = assign59280_e92251_d_n5;
        locals.var_pds_ini_dn6 = assign59280_e92251_d_n6;
        locals.var_pds_ini_dn7 = assign59280_e92251_d_n7;
        locals.var_pds_ini_dn8 = assign59280_e92251_d_n8;
        locals.var_pds_ini_dn9 = assign59280_e92251_d_n9;
        locals.var_pds_ini_dn10 = assign59280_e92251_d_n10;
        locals.var_pds_ini_dn13 = assign59280_e92251_d_n13;
        locals.var_pds_ini_rv = 0.0;

        let (assign59300_e92269, assign59300_e92269_d_n0, assign59300_e92269_d_n2, assign59300_e92269_d_n4, assign59300_e92269_d_n5, assign59300_e92269_d_n6, assign59300_e92269_d_n7, assign59300_e92269_d_n8, assign59300_e92269_d_n9, assign59300_e92269_d_n10, assign59300_e92269_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn13,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign59300_e92269;
        locals.var_pds_dn0 = assign59300_e92269_d_n0;
        locals.var_pds_dn2 = assign59300_e92269_d_n2;
        locals.var_pds_dn4 = assign59300_e92269_d_n4;
        locals.var_pds_dn5 = assign59300_e92269_d_n5;
        locals.var_pds_dn6 = assign59300_e92269_d_n6;
        locals.var_pds_dn7 = assign59300_e92269_d_n7;
        locals.var_pds_dn8 = assign59300_e92269_d_n8;
        locals.var_pds_dn9 = assign59300_e92269_d_n9;
        locals.var_pds_dn10 = assign59300_e92269_d_n10;
        locals.var_pds_dn13 = assign59300_e92269_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign59310_e92280, assign59310_e92280_d_n0, assign59310_e92280_d_n2, assign59310_e92280_d_n4, assign59310_e92280_d_n5, assign59310_e92280_d_n6, assign59310_e92280_d_n7, assign59310_e92280_d_n8, assign59310_e92280_d_n9, assign59310_e92280_d_n10, assign59310_e92280_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        let assign59310_e92278: f64 = (locals.var_ps0 + locals.var_pds);
        (assign59310_e92278, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn4 + locals.var_pds_dn4), (locals.var_ps0_dn5 + locals.var_pds_dn5), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn8 + locals.var_pds_dn8), (locals.var_ps0_dn9 + locals.var_pds_dn9), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn13 + locals.var_pds_dn13),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
        locals.var_psl = assign59310_e92280;
        locals.var_psl_dn0 = assign59310_e92280_d_n0;
        locals.var_psl_dn2 = assign59310_e92280_d_n2;
        locals.var_psl_dn4 = assign59310_e92280_d_n4;
        locals.var_psl_dn5 = assign59310_e92280_d_n5;
        locals.var_psl_dn6 = assign59310_e92280_d_n6;
        locals.var_psl_dn7 = assign59310_e92280_d_n7;
        locals.var_psl_dn8 = assign59310_e92280_d_n8;
        locals.var_psl_dn9 = assign59310_e92280_d_n9;
        locals.var_psl_dn10 = assign59310_e92280_d_n10;
        locals.var_psl_dn13 = assign59310_e92280_d_n13;
        locals.var_psl_rv = 0.0;

        let (assign59320_e92289,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1448 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign59320_e92289;
        locals.var_flg_conv_rv = 0.0;

        let (assign59330_e92298,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_start_of_loopl != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59330_e92298;
        locals.var_start_of_loopl_rv = 0.0;

        let (assign59340_e92305,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign59340_e92305;
        locals.var_lp_sl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_211(
        locals: &mut StampLocals,
    ) {
        let mut assign59350_loop_guard: usize = 0;
        while {
            let assign59350_cond_e92313: f64 = (40.0 + 1.0);
            let assign59350_cond_e92315: f64 = if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_lp_sl <= assign59350_cond_e92313)) { 1.0 } else { 0.0 };
            assign59350_cond_e92315 != 0.0
        } {
            assign59350_loop_guard += 1;
            assert!(assign59350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign59350_body1_e92335, assign59350_body1_e92335_d_n0, assign59350_body1_e92335_d_n2, assign59350_body1_e92335_d_n4, assign59350_body1_e92335_d_n5, assign59350_body1_e92335_d_n6, assign59350_body1_e92335_d_n7, assign59350_body1_e92335_d_n8, assign59350_body1_e92335_d_n9, assign59350_body1_e92335_d_n10, assign59350_body1_e92335_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59350_body1_e92332: f64 = (locals.var_psl - locals.var_vbscl__blk435);
        let assign59350_body1_e92333: f64 = (locals.var_beta * assign59350_body1_e92332);
        (assign59350_body1_e92333, ((locals.var_beta_dn0 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vbscl__blk435_dn0))), ((locals.var_beta_dn2 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vbscl__blk435_dn2))), ((locals.var_beta_dn4 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vbscl__blk435_dn4))), ((locals.var_beta_dn5 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vbscl__blk435_dn5))), ((locals.var_beta_dn6 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vbscl__blk435_dn6))), ((locals.var_beta_dn7 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vbscl__blk435_dn7))), ((locals.var_beta_dn8 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vbscl__blk435_dn8))), ((locals.var_beta_dn9 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vbscl__blk435_dn9))), ((locals.var_beta_dn10 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vbscl__blk435_dn10))), ((locals.var_beta_dn13 * assign59350_body1_e92332) + (locals.var_beta * (locals.var_psl_dn13 - locals.var_vbscl__blk435_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign59350_body1_e92335;
            locals.var_chi_dn0 = assign59350_body1_e92335_d_n0;
            locals.var_chi_dn2 = assign59350_body1_e92335_d_n2;
            locals.var_chi_dn4 = assign59350_body1_e92335_d_n4;
            locals.var_chi_dn5 = assign59350_body1_e92335_d_n5;
            locals.var_chi_dn6 = assign59350_body1_e92335_d_n6;
            locals.var_chi_dn7 = assign59350_body1_e92335_d_n7;
            locals.var_chi_dn8 = assign59350_body1_e92335_d_n8;
            locals.var_chi_dn9 = assign59350_body1_e92335_d_n9;
            locals.var_chi_dn10 = assign59350_body1_e92335_d_n10;
            locals.var_chi_dn13 = assign59350_body1_e92335_d_n13;
            locals.var_chi_rv = 0.0;
            let assign59350_body2_e92338: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1453 = assign59350_body2_e92338;
            locals.var_guard1453_rv = 0.0;
            let (assign59350_body3_e92362, assign59350_body3_e92362_d_n0, assign59350_body3_e92362_d_n2, assign59350_body3_e92362_d_n4, assign59350_body3_e92362_d_n5, assign59350_body3_e92362_d_n6, assign59350_body3_e92362_d_n7, assign59350_body3_e92362_d_n8, assign59350_body3_e92362_d_n9, assign59350_body3_e92362_d_n10, assign59350_body3_e92362_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body3_e92347: f64 = (locals.var_chi * locals.var_chi);
        let assign59350_body3_e92349: f64 = (assign59350_body3_e92347 * locals.var_chi);
        let assign59350_body3_e92353: f64 = (-0.07053654284009761);
        let assign59350_body3_e92356: f64 = (locals.var_chi * 0.006115288895133179);
        let assign59350_body3_e92357: f64 = (assign59350_body3_e92353 + assign59350_body3_e92356);
        let assign59350_body3_e92358: f64 = (locals.var_chi * assign59350_body3_e92357);
        let assign59350_body3_e92359: f64 = (0.29693154855771 + assign59350_body3_e92358);
        let assign59350_body3_e92360: f64 = (assign59350_body3_e92349 * assign59350_body3_e92359);
        (assign59350_body3_e92360, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn0)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn0 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn2)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn2 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn4)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn4 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn5)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn5 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn6)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn6 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn7)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn7 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn8)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn8 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn9)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn9 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn10)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn10 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) * locals.var_chi) + (assign59350_body3_e92347 * locals.var_chi_dn13)) * assign59350_body3_e92359) + (assign59350_body3_e92349 * ((locals.var_chi_dn13 * assign59350_body3_e92357) + (locals.var_chi * (locals.var_chi_dn13 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn13,)
    }
};
            locals.var_fi = assign59350_body3_e92362;
            locals.var_fi_dn0 = assign59350_body3_e92362_d_n0;
            locals.var_fi_dn2 = assign59350_body3_e92362_d_n2;
            locals.var_fi_dn4 = assign59350_body3_e92362_d_n4;
            locals.var_fi_dn5 = assign59350_body3_e92362_d_n5;
            locals.var_fi_dn6 = assign59350_body3_e92362_d_n6;
            locals.var_fi_dn7 = assign59350_body3_e92362_d_n7;
            locals.var_fi_dn8 = assign59350_body3_e92362_d_n8;
            locals.var_fi_dn9 = assign59350_body3_e92362_d_n9;
            locals.var_fi_dn10 = assign59350_body3_e92362_d_n10;
            locals.var_fi_dn13 = assign59350_body3_e92362_d_n13;
            locals.var_fi_rv = 0.0;
            let (assign59350_body4_e92390, assign59350_body4_e92390_d_n0, assign59350_body4_e92390_d_n2, assign59350_body4_e92390_d_n4, assign59350_body4_e92390_d_n5, assign59350_body4_e92390_d_n6, assign59350_body4_e92390_d_n7, assign59350_body4_e92390_d_n8, assign59350_body4_e92390_d_n9, assign59350_body4_e92390_d_n10, assign59350_body4_e92390_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body4_e92371: f64 = (locals.var_chi * locals.var_chi);
        let assign59350_body4_e92374: f64 = (3.0 * 0.29693154855771);
        let assign59350_body4_e92378: f64 = (-0.07053654284009761);
        let assign59350_body4_e92379: f64 = (4.0 * assign59350_body4_e92378);
        let assign59350_body4_e92382: f64 = (locals.var_chi * 5.0);
        let assign59350_body4_e92384: f64 = (assign59350_body4_e92382 * 0.006115288895133179);
        let assign59350_body4_e92385: f64 = (assign59350_body4_e92379 + assign59350_body4_e92384);
        let assign59350_body4_e92386: f64 = (locals.var_chi * assign59350_body4_e92385);
        let assign59350_body4_e92387: f64 = (assign59350_body4_e92374 + assign59350_body4_e92386);
        let assign59350_body4_e92388: f64 = (assign59350_body4_e92371 * assign59350_body4_e92387);
        (assign59350_body4_e92388, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn0 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn2 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn4 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn5 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn6 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn7 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn8 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn9 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn10 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) * assign59350_body4_e92387) + (assign59350_body4_e92371 * ((locals.var_chi_dn13 * assign59350_body4_e92385) + (locals.var_chi * ((locals.var_chi_dn13 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn13,)
    }
};
            locals.var_fi_dchi = assign59350_body4_e92390;
            locals.var_fi_dchi_dn0 = assign59350_body4_e92390_d_n0;
            locals.var_fi_dchi_dn2 = assign59350_body4_e92390_d_n2;
            locals.var_fi_dchi_dn4 = assign59350_body4_e92390_d_n4;
            locals.var_fi_dchi_dn5 = assign59350_body4_e92390_d_n5;
            locals.var_fi_dchi_dn6 = assign59350_body4_e92390_d_n6;
            locals.var_fi_dchi_dn7 = assign59350_body4_e92390_d_n7;
            locals.var_fi_dchi_dn8 = assign59350_body4_e92390_d_n8;
            locals.var_fi_dchi_dn9 = assign59350_body4_e92390_d_n9;
            locals.var_fi_dchi_dn10 = assign59350_body4_e92390_d_n10;
            locals.var_fi_dchi_dn13 = assign59350_body4_e92390_d_n13;
            locals.var_fi_dchi_rv = 0.0;
            let (assign59350_body5_e92401, assign59350_body5_e92401_d_n0, assign59350_body5_e92401_d_n2, assign59350_body5_e92401_d_n4, assign59350_body5_e92401_d_n5, assign59350_body5_e92401_d_n6, assign59350_body5_e92401_d_n7, assign59350_body5_e92401_d_n8, assign59350_body5_e92401_d_n9, assign59350_body5_e92401_d_n10, assign59350_body5_e92401_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body5_e92399: f64 = (locals.var_cnst1 * locals.var_exp_bvbsvds);
        (assign59350_body5_e92399, ((locals.var_cnst1_dn0 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn10)), ((locals.var_cnst1_dn13 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
            locals.var_cfs1 = assign59350_body5_e92401;
            locals.var_cfs1_dn0 = assign59350_body5_e92401_d_n0;
            locals.var_cfs1_dn2 = assign59350_body5_e92401_d_n2;
            locals.var_cfs1_dn4 = assign59350_body5_e92401_d_n4;
            locals.var_cfs1_dn5 = assign59350_body5_e92401_d_n5;
            locals.var_cfs1_dn6 = assign59350_body5_e92401_d_n6;
            locals.var_cfs1_dn7 = assign59350_body5_e92401_d_n7;
            locals.var_cfs1_dn8 = assign59350_body5_e92401_d_n8;
            locals.var_cfs1_dn9 = assign59350_body5_e92401_d_n9;
            locals.var_cfs1_dn10 = assign59350_body5_e92401_d_n10;
            locals.var_cfs1_dn13 = assign59350_body5_e92401_d_n13;
            locals.var_cfs1_rv = 0.0;
            let (assign59350_body6_e92414, assign59350_body6_e92414_d_n0, assign59350_body6_e92414_d_n2, assign59350_body6_e92414_d_n4, assign59350_body6_e92414_d_n5, assign59350_body6_e92414_d_n6, assign59350_body6_e92414_d_n7, assign59350_body6_e92414_d_n8, assign59350_body6_e92414_d_n9, assign59350_body6_e92414_d_n10, assign59350_body6_e92414_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body6_e92410: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign59350_body6_e92412: f64 = (assign59350_body6_e92410 * locals.var_fi);
        (assign59350_body6_e92412, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn13 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn13)) * locals.var_fi) + (assign59350_body6_e92410 * locals.var_fi_dn13)),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn13,)
    }
};
            locals.var_fsl1 = assign59350_body6_e92414;
            locals.var_fsl1_dn0 = assign59350_body6_e92414_d_n0;
            locals.var_fsl1_dn2 = assign59350_body6_e92414_d_n2;
            locals.var_fsl1_dn4 = assign59350_body6_e92414_d_n4;
            locals.var_fsl1_dn5 = assign59350_body6_e92414_d_n5;
            locals.var_fsl1_dn6 = assign59350_body6_e92414_d_n6;
            locals.var_fsl1_dn7 = assign59350_body6_e92414_d_n7;
            locals.var_fsl1_dn8 = assign59350_body6_e92414_d_n8;
            locals.var_fsl1_dn9 = assign59350_body6_e92414_d_n9;
            locals.var_fsl1_dn10 = assign59350_body6_e92414_d_n10;
            locals.var_fsl1_dn13 = assign59350_body6_e92414_d_n13;
            locals.var_fsl1_rv = 0.0;
            let (assign59350_body7_e92431, assign59350_body7_e92431_d_n0, assign59350_body7_e92431_d_n2, assign59350_body7_e92431_d_n4, assign59350_body7_e92431_d_n5, assign59350_body7_e92431_d_n6, assign59350_body7_e92431_d_n7, assign59350_body7_e92431_d_n8, assign59350_body7_e92431_d_n9, assign59350_body7_e92431_d_n10, assign59350_body7_e92431_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body7_e92423: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign59350_body7_e92425: f64 = (assign59350_body7_e92423 * 2.0);
        let assign59350_body7_e92427: f64 = (assign59350_body7_e92425 * locals.var_fi);
        let assign59350_body7_e92429: f64 = (assign59350_body7_e92427 * locals.var_fi_dchi);
        (assign59350_body7_e92429, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * 2.0) * locals.var_fi) + (assign59350_body7_e92425 * locals.var_fi_dn13)) * locals.var_fi_dchi) + (assign59350_body7_e92427 * locals.var_fi_dchi_dn13)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn13,)
    }
};
            locals.var_fsl1_dpsl = assign59350_body7_e92431;
            locals.var_fsl1_dpsl_dn0 = assign59350_body7_e92431_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59350_body7_e92431_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59350_body7_e92431_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59350_body7_e92431_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59350_body7_e92431_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59350_body7_e92431_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59350_body7_e92431_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59350_body7_e92431_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59350_body7_e92431_d_n10;
            locals.var_fsl1_dpsl_dn13 = assign59350_body7_e92431_d_n13;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign59350_body8_e92460, assign59350_body8_e92460_d_n0, assign59350_body8_e92460_d_n2, assign59350_body8_e92460_d_n4, assign59350_body8_e92460_d_n5, assign59350_body8_e92460_d_n6, assign59350_body8_e92460_d_n7, assign59350_body8_e92460_d_n8, assign59350_body8_e92460_d_n9, assign59350_body8_e92460_d_n10, assign59350_body8_e92460_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body8_e92442: f64 = (-0.117851130197758);
        let assign59350_body8_e92447: f64 = (-0.00163730162779191);
        let assign59350_body8_e92450: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign59350_body8_e92451: f64 = (assign59350_body8_e92447 + assign59350_body8_e92450);
        let assign59350_body8_e92452: f64 = (locals.var_chi * assign59350_body8_e92451);
        let assign59350_body8_e92453: f64 = (0.0178800506338833 + assign59350_body8_e92452);
        let assign59350_body8_e92454: f64 = (locals.var_chi * assign59350_body8_e92453);
        let assign59350_body8_e92455: f64 = (assign59350_body8_e92442 + assign59350_body8_e92454);
        let assign59350_body8_e92456: f64 = (locals.var_chi * assign59350_body8_e92455);
        let assign59350_body8_e92457: f64 = (0.707106781186548 + assign59350_body8_e92456);
        let assign59350_body8_e92458: f64 = (locals.var_chi * assign59350_body8_e92457);
        (assign59350_body8_e92458, ((locals.var_chi_dn0 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn0 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn0 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn0 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn2 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn2 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn2 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn4 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn4 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn4 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn5 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn5 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn5 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn6 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn6 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn6 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn7 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn7 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn7 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn8 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn8 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn8 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn9 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn9 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn9 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn10 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn10 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn10 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn13 * assign59350_body8_e92457) + (locals.var_chi * ((locals.var_chi_dn13 * assign59350_body8_e92455) + (locals.var_chi * ((locals.var_chi_dn13 * assign59350_body8_e92453) + (locals.var_chi * ((locals.var_chi_dn13 * assign59350_body8_e92451) + (locals.var_chi * (locals.var_chi_dn13 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign59350_body8_e92460;
            locals.var_fb_dn0 = assign59350_body8_e92460_d_n0;
            locals.var_fb_dn2 = assign59350_body8_e92460_d_n2;
            locals.var_fb_dn4 = assign59350_body8_e92460_d_n4;
            locals.var_fb_dn5 = assign59350_body8_e92460_d_n5;
            locals.var_fb_dn6 = assign59350_body8_e92460_d_n6;
            locals.var_fb_dn7 = assign59350_body8_e92460_d_n7;
            locals.var_fb_dn8 = assign59350_body8_e92460_d_n8;
            locals.var_fb_dn9 = assign59350_body8_e92460_d_n9;
            locals.var_fb_dn10 = assign59350_body8_e92460_d_n10;
            locals.var_fb_dn13 = assign59350_body8_e92460_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign59350_body9_e92495, assign59350_body9_e92495_d_n0, assign59350_body9_e92495_d_n2, assign59350_body9_e92495_d_n4, assign59350_body9_e92495_d_n5, assign59350_body9_e92495_d_n6, assign59350_body9_e92495_d_n7, assign59350_body9_e92495_d_n8, assign59350_body9_e92495_d_n9, assign59350_body9_e92495_d_n10, assign59350_body9_e92495_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body9_e92471: f64 = (-0.117851130197758);
        let assign59350_body9_e92472: f64 = (2.0 * assign59350_body9_e92471);
        let assign59350_body9_e92476: f64 = (3.0 * 0.0178800506338833);
        let assign59350_body9_e92480: f64 = (-0.00163730162779191);
        let assign59350_body9_e92481: f64 = (4.0 * assign59350_body9_e92480);
        let assign59350_body9_e92484: f64 = (locals.var_chi * 5.0);
        let assign59350_body9_e92486: f64 = (assign59350_body9_e92484 * 6.36964918866352e-5);
        let assign59350_body9_e92487: f64 = (assign59350_body9_e92481 + assign59350_body9_e92486);
        let assign59350_body9_e92488: f64 = (locals.var_chi * assign59350_body9_e92487);
        let assign59350_body9_e92489: f64 = (assign59350_body9_e92476 + assign59350_body9_e92488);
        let assign59350_body9_e92490: f64 = (locals.var_chi * assign59350_body9_e92489);
        let assign59350_body9_e92491: f64 = (assign59350_body9_e92472 + assign59350_body9_e92490);
        let assign59350_body9_e92492: f64 = (locals.var_chi * assign59350_body9_e92491);
        let assign59350_body9_e92493: f64 = (0.707106781186548 + assign59350_body9_e92492);
        (assign59350_body9_e92493, ((locals.var_chi_dn0 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn0 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn0 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn2 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn2 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn4 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn4 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn5 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn5 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn6 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn6 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn7 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn7 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn8 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn8 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn9 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn9 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn10 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn10 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn13 * assign59350_body9_e92491) + (locals.var_chi * ((locals.var_chi_dn13 * assign59350_body9_e92489) + (locals.var_chi * ((locals.var_chi_dn13 * assign59350_body9_e92487) + (locals.var_chi * ((locals.var_chi_dn13 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn13,)
    }
};
            locals.var_fb_dchi = assign59350_body9_e92495;
            locals.var_fb_dchi_dn0 = assign59350_body9_e92495_d_n0;
            locals.var_fb_dchi_dn2 = assign59350_body9_e92495_d_n2;
            locals.var_fb_dchi_dn4 = assign59350_body9_e92495_d_n4;
            locals.var_fb_dchi_dn5 = assign59350_body9_e92495_d_n5;
            locals.var_fb_dchi_dn6 = assign59350_body9_e92495_d_n6;
            locals.var_fb_dchi_dn7 = assign59350_body9_e92495_d_n7;
            locals.var_fb_dchi_dn8 = assign59350_body9_e92495_d_n8;
            locals.var_fb_dchi_dn9 = assign59350_body9_e92495_d_n9;
            locals.var_fb_dchi_dn10 = assign59350_body9_e92495_d_n10;
            locals.var_fb_dchi_dn13 = assign59350_body9_e92495_d_n13;
            locals.var_fb_dchi_rv = 0.0;
            let (assign59350_body10_e92509, assign59350_body10_e92509_d_n0, assign59350_body10_e92509_d_n2, assign59350_body10_e92509_d_n4, assign59350_body10_e92509_d_n5, assign59350_body10_e92509_d_n6, assign59350_body10_e92509_d_n7, assign59350_body10_e92509_d_n8, assign59350_body10_e92509_d_n9, assign59350_body10_e92509_d_n10, assign59350_body10_e92509_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body10_e92504: f64 = (locals.var_fb * locals.var_fb);
        let assign59350_body10_e92506: f64 = (assign59350_body10_e92504 + locals.var_fsl1);
        let assign59350_body10_e92507: f64 = (assign59350_body10_e92506).sqrt();
        (assign59350_body10_e92507, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fsl1_dn4) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fsl1_dn5) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fsl1_dn8) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fsl1_dn9) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign59350_body10_e92507)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fsl1_dn13) / (2.0 * assign59350_body10_e92507)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn13,)
    }
};
            locals.var_fsl2 = assign59350_body10_e92509;
            locals.var_fsl2_dn0 = assign59350_body10_e92509_d_n0;
            locals.var_fsl2_dn2 = assign59350_body10_e92509_d_n2;
            locals.var_fsl2_dn4 = assign59350_body10_e92509_d_n4;
            locals.var_fsl2_dn5 = assign59350_body10_e92509_d_n5;
            locals.var_fsl2_dn6 = assign59350_body10_e92509_d_n6;
            locals.var_fsl2_dn7 = assign59350_body10_e92509_d_n7;
            locals.var_fsl2_dn8 = assign59350_body10_e92509_d_n8;
            locals.var_fsl2_dn9 = assign59350_body10_e92509_d_n9;
            locals.var_fsl2_dn10 = assign59350_body10_e92509_d_n10;
            locals.var_fsl2_dn13 = assign59350_body10_e92509_d_n13;
            locals.var_fsl2_rv = 0.0;
            let (assign59350_body11_e92530, assign59350_body11_e92530_d_n0, assign59350_body11_e92530_d_n2, assign59350_body11_e92530_d_n4, assign59350_body11_e92530_d_n5, assign59350_body11_e92530_d_n6, assign59350_body11_e92530_d_n7, assign59350_body11_e92530_d_n8, assign59350_body11_e92530_d_n9, assign59350_body11_e92530_d_n10, assign59350_body11_e92530_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59350_body11_e92518: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign59350_body11_e92520: f64 = (assign59350_body11_e92518 * 2.0);
        let assign59350_body11_e92522: f64 = (assign59350_body11_e92520 * locals.var_fb);
        let assign59350_body11_e92524: f64 = (assign59350_body11_e92522 + locals.var_fsl1_dpsl);
        let assign59350_body11_e92527: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59350_body11_e92528: f64 = (assign59350_body11_e92524 / assign59350_body11_e92527);
        (assign59350_body11_e92528, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn4)) + locals.var_fsl1_dpsl_dn4) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn5)) + locals.var_fsl1_dpsl_dn5) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn8)) + locals.var_fsl1_dpsl_dn8) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn9)) + locals.var_fsl1_dpsl_dn9) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59350_body11_e92527 * assign59350_body11_e92527)), (((((((((locals.var_beta_dn13 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn13)) * 2.0) * locals.var_fb) + (assign59350_body11_e92520 * locals.var_fb_dn13)) + locals.var_fsl1_dpsl_dn13) * assign59350_body11_e92527) - (assign59350_body11_e92524 * (locals.var_fsl2_dn13 + locals.var_fsl2_dn13))) / (assign59350_body11_e92527 * assign59350_body11_e92527)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn13,)
    }
};
            locals.var_fsl2_dpsl = assign59350_body11_e92530;
            locals.var_fsl2_dpsl_dn0 = assign59350_body11_e92530_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59350_body11_e92530_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59350_body11_e92530_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59350_body11_e92530_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59350_body11_e92530_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59350_body11_e92530_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59350_body11_e92530_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59350_body11_e92530_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59350_body11_e92530_d_n10;
            locals.var_fsl2_dpsl_dn13 = assign59350_body11_e92530_d_n13;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign59350_body12_e92544, assign59350_body12_e92544_d_n0, assign59350_body12_e92544_d_n2, assign59350_body12_e92544_d_n4, assign59350_body12_e92544_d_n5, assign59350_body12_e92544_d_n6, assign59350_body12_e92544_d_n7, assign59350_body12_e92544_d_n8, assign59350_body12_e92544_d_n9, assign59350_body12_e92544_d_n10, assign59350_body12_e92544_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body12_e92541: f64 = (locals.var_psl - locals.var_vds);
        let assign59350_body12_e92542: f64 = (locals.var_beta * assign59350_body12_e92541);
        (assign59350_body12_e92542, ((locals.var_beta_dn0 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vds_dn0))), ((locals.var_beta_dn2 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vds_dn2))), ((locals.var_beta_dn4 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vds_dn4))), ((locals.var_beta_dn5 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vds_dn5))), ((locals.var_beta_dn6 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vds_dn6))), ((locals.var_beta_dn7 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vds_dn7))), ((locals.var_beta_dn8 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vds_dn8))), ((locals.var_beta_dn9 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vds_dn9))), ((locals.var_beta_dn10 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vds_dn10))), ((locals.var_beta_dn13 * assign59350_body12_e92541) + (locals.var_beta * (locals.var_psl_dn13 - locals.var_vds_dn13))),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn13,)
    }
};
            locals.var_rho = assign59350_body12_e92544;
            locals.var_rho_dn0 = assign59350_body12_e92544_d_n0;
            locals.var_rho_dn2 = assign59350_body12_e92544_d_n2;
            locals.var_rho_dn4 = assign59350_body12_e92544_d_n4;
            locals.var_rho_dn5 = assign59350_body12_e92544_d_n5;
            locals.var_rho_dn6 = assign59350_body12_e92544_d_n6;
            locals.var_rho_dn7 = assign59350_body12_e92544_d_n7;
            locals.var_rho_dn8 = assign59350_body12_e92544_d_n8;
            locals.var_rho_dn9 = assign59350_body12_e92544_d_n9;
            locals.var_rho_dn10 = assign59350_body12_e92544_d_n10;
            locals.var_rho_dn13 = assign59350_body12_e92544_d_n13;
            locals.var_rho_rv = 0.0;
            let (assign59350_body13_e92555, assign59350_body13_e92555_d_n0, assign59350_body13_e92555_d_n2, assign59350_body13_e92555_d_n4, assign59350_body13_e92555_d_n5, assign59350_body13_e92555_d_n6, assign59350_body13_e92555_d_n7, assign59350_body13_e92555_d_n8, assign59350_body13_e92555_d_n9, assign59350_body13_e92555_d_n10, assign59350_body13_e92555_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body13_e92553: f64 = (locals.var_rho).exp();
        (assign59350_body13_e92553, (assign59350_body13_e92553 * locals.var_rho_dn0), (assign59350_body13_e92553 * locals.var_rho_dn2), (assign59350_body13_e92553 * locals.var_rho_dn4), (assign59350_body13_e92553 * locals.var_rho_dn5), (assign59350_body13_e92553 * locals.var_rho_dn6), (assign59350_body13_e92553 * locals.var_rho_dn7), (assign59350_body13_e92553 * locals.var_rho_dn8), (assign59350_body13_e92553 * locals.var_rho_dn9), (assign59350_body13_e92553 * locals.var_rho_dn10), (assign59350_body13_e92553 * locals.var_rho_dn13),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn13,)
    }
};
            locals.var_exp_rho = assign59350_body13_e92555;
            locals.var_exp_rho_dn0 = assign59350_body13_e92555_d_n0;
            locals.var_exp_rho_dn2 = assign59350_body13_e92555_d_n2;
            locals.var_exp_rho_dn4 = assign59350_body13_e92555_d_n4;
            locals.var_exp_rho_dn5 = assign59350_body13_e92555_d_n5;
            locals.var_exp_rho_dn6 = assign59350_body13_e92555_d_n6;
            locals.var_exp_rho_dn7 = assign59350_body13_e92555_d_n7;
            locals.var_exp_rho_dn8 = assign59350_body13_e92555_d_n8;
            locals.var_exp_rho_dn9 = assign59350_body13_e92555_d_n9;
            locals.var_exp_rho_dn10 = assign59350_body13_e92555_d_n10;
            locals.var_exp_rho_dn13 = assign59350_body13_e92555_d_n13;
            locals.var_exp_rho_rv = 0.0;
            let (assign59350_body14_e92569, assign59350_body14_e92569_d_n0, assign59350_body14_e92569_d_n2, assign59350_body14_e92569_d_n4, assign59350_body14_e92569_d_n5, assign59350_body14_e92569_d_n6, assign59350_body14_e92569_d_n7, assign59350_body14_e92569_d_n8, assign59350_body14_e92569_d_n9, assign59350_body14_e92569_d_n10, assign59350_body14_e92569_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body14_e92566: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign59350_body14_e92567: f64 = (locals.var_cnst1 * assign59350_body14_e92566);
        (assign59350_body14_e92567, ((locals.var_cnst1_dn0 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((locals.var_cnst1_dn2 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((locals.var_cnst1_dn4 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((locals.var_cnst1_dn5 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((locals.var_cnst1_dn6 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((locals.var_cnst1_dn7 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((locals.var_cnst1_dn8 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((locals.var_cnst1_dn9 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((locals.var_cnst1_dn10 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((locals.var_cnst1_dn13 * assign59350_body14_e92566) + (locals.var_cnst1 * (locals.var_exp_rho_dn13 - locals.var_exp_bvbsvds_dn13))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn13,)
    }
};
            locals.var_fsl1 = assign59350_body14_e92569;
            locals.var_fsl1_dn0 = assign59350_body14_e92569_d_n0;
            locals.var_fsl1_dn2 = assign59350_body14_e92569_d_n2;
            locals.var_fsl1_dn4 = assign59350_body14_e92569_d_n4;
            locals.var_fsl1_dn5 = assign59350_body14_e92569_d_n5;
            locals.var_fsl1_dn6 = assign59350_body14_e92569_d_n6;
            locals.var_fsl1_dn7 = assign59350_body14_e92569_d_n7;
            locals.var_fsl1_dn8 = assign59350_body14_e92569_d_n8;
            locals.var_fsl1_dn9 = assign59350_body14_e92569_d_n9;
            locals.var_fsl1_dn10 = assign59350_body14_e92569_d_n10;
            locals.var_fsl1_dn13 = assign59350_body14_e92569_d_n13;
            locals.var_fsl1_rv = 0.0;
            let (assign59350_body15_e92583, assign59350_body15_e92583_d_n0, assign59350_body15_e92583_d_n2, assign59350_body15_e92583_d_n4, assign59350_body15_e92583_d_n5, assign59350_body15_e92583_d_n6, assign59350_body15_e92583_d_n7, assign59350_body15_e92583_d_n8, assign59350_body15_e92583_d_n9, assign59350_body15_e92583_d_n10, assign59350_body15_e92583_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body15_e92579: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign59350_body15_e92581: f64 = (assign59350_body15_e92579 * locals.var_exp_rho);
        (assign59350_body15_e92581, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn10)), ((((locals.var_cnst1_dn13 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn13)) * locals.var_exp_rho) + (assign59350_body15_e92579 * locals.var_exp_rho_dn13)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn13,)
    }
};
            locals.var_fsl1_dpsl = assign59350_body15_e92583;
            locals.var_fsl1_dpsl_dn0 = assign59350_body15_e92583_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59350_body15_e92583_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59350_body15_e92583_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59350_body15_e92583_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59350_body15_e92583_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59350_body15_e92583_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59350_body15_e92583_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59350_body15_e92583_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59350_body15_e92583_d_n10;
            locals.var_fsl1_dpsl_dn13 = assign59350_body15_e92583_d_n13;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign59350_body16_e92595, assign59350_body16_e92595_d_n0, assign59350_body16_e92595_d_n2, assign59350_body16_e92595_d_n4, assign59350_body16_e92595_d_n5, assign59350_body16_e92595_d_n6, assign59350_body16_e92595_d_n7, assign59350_body16_e92595_d_n8, assign59350_body16_e92595_d_n9, assign59350_body16_e92595_d_n10, assign59350_body16_e92595_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body16_e92593: f64 = (locals.var_chi - 1.0);
        (assign59350_body16_e92593, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn13,)
    }
};
            locals.var_xil = assign59350_body16_e92595;
            locals.var_xil_dn0 = assign59350_body16_e92595_d_n0;
            locals.var_xil_dn2 = assign59350_body16_e92595_d_n2;
            locals.var_xil_dn4 = assign59350_body16_e92595_d_n4;
            locals.var_xil_dn5 = assign59350_body16_e92595_d_n5;
            locals.var_xil_dn6 = assign59350_body16_e92595_d_n6;
            locals.var_xil_dn7 = assign59350_body16_e92595_d_n7;
            locals.var_xil_dn8 = assign59350_body16_e92595_d_n8;
            locals.var_xil_dn9 = assign59350_body16_e92595_d_n9;
            locals.var_xil_dn10 = assign59350_body16_e92595_d_n10;
            locals.var_xil_dn13 = assign59350_body16_e92595_d_n13;
            locals.var_xil_rv = 0.0;
            let (assign59350_body17_e92608, assign59350_body17_e92608_d_n0, assign59350_body17_e92608_d_n2, assign59350_body17_e92608_d_n4, assign59350_body17_e92608_d_n5, assign59350_body17_e92608_d_n6, assign59350_body17_e92608_d_n7, assign59350_body17_e92608_d_n8, assign59350_body17_e92608_d_n9, assign59350_body17_e92608_d_n10, assign59350_body17_e92608_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body17_e92605: f64 = (locals.var_xil + locals.var_fsl1);
        let assign59350_body17_e92606: f64 = (assign59350_body17_e92605).sqrt();
        (assign59350_body17_e92606, ((locals.var_xil_dn0 + locals.var_fsl1_dn0) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn2 + locals.var_fsl1_dn2) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn4 + locals.var_fsl1_dn4) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn5 + locals.var_fsl1_dn5) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn6 + locals.var_fsl1_dn6) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn7 + locals.var_fsl1_dn7) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn8 + locals.var_fsl1_dn8) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn9 + locals.var_fsl1_dn9) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn10 + locals.var_fsl1_dn10) / (2.0 * assign59350_body17_e92606)), ((locals.var_xil_dn13 + locals.var_fsl1_dn13) / (2.0 * assign59350_body17_e92606)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn13,)
    }
};
            locals.var_fsl2 = assign59350_body17_e92608;
            locals.var_fsl2_dn0 = assign59350_body17_e92608_d_n0;
            locals.var_fsl2_dn2 = assign59350_body17_e92608_d_n2;
            locals.var_fsl2_dn4 = assign59350_body17_e92608_d_n4;
            locals.var_fsl2_dn5 = assign59350_body17_e92608_d_n5;
            locals.var_fsl2_dn6 = assign59350_body17_e92608_d_n6;
            locals.var_fsl2_dn7 = assign59350_body17_e92608_d_n7;
            locals.var_fsl2_dn8 = assign59350_body17_e92608_d_n8;
            locals.var_fsl2_dn9 = assign59350_body17_e92608_d_n9;
            locals.var_fsl2_dn10 = assign59350_body17_e92608_d_n10;
            locals.var_fsl2_dn13 = assign59350_body17_e92608_d_n13;
            locals.var_fsl2_rv = 0.0;
            let (assign59350_body18_e92624, assign59350_body18_e92624_d_n0, assign59350_body18_e92624_d_n2, assign59350_body18_e92624_d_n4, assign59350_body18_e92624_d_n5, assign59350_body18_e92624_d_n6, assign59350_body18_e92624_d_n7, assign59350_body18_e92624_d_n8, assign59350_body18_e92624_d_n9, assign59350_body18_e92624_d_n10, assign59350_body18_e92624_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1453 == 0.0)) {
        let assign59350_body18_e92618: f64 = (locals.var_beta + locals.var_fsl1_dpsl);
        let assign59350_body18_e92621: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59350_body18_e92622: f64 = (assign59350_body18_e92618 / assign59350_body18_e92621);
        (assign59350_body18_e92622, ((((locals.var_beta_dn0 + locals.var_fsl1_dpsl_dn0) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn2 + locals.var_fsl1_dpsl_dn2) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn4 + locals.var_fsl1_dpsl_dn4) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn5 + locals.var_fsl1_dpsl_dn5) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn6 + locals.var_fsl1_dpsl_dn6) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn7 + locals.var_fsl1_dpsl_dn7) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn8 + locals.var_fsl1_dpsl_dn8) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn9 + locals.var_fsl1_dpsl_dn9) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn10 + locals.var_fsl1_dpsl_dn10) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59350_body18_e92621 * assign59350_body18_e92621)), ((((locals.var_beta_dn13 + locals.var_fsl1_dpsl_dn13) * assign59350_body18_e92621) - (assign59350_body18_e92618 * (locals.var_fsl2_dn13 + locals.var_fsl2_dn13))) / (assign59350_body18_e92621 * assign59350_body18_e92621)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn13,)
    }
};
            locals.var_fsl2_dpsl = assign59350_body18_e92624;
            locals.var_fsl2_dpsl_dn0 = assign59350_body18_e92624_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59350_body18_e92624_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59350_body18_e92624_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59350_body18_e92624_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59350_body18_e92624_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59350_body18_e92624_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59350_body18_e92624_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59350_body18_e92624_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59350_body18_e92624_d_n10;
            locals.var_fsl2_dpsl_dn13 = assign59350_body18_e92624_d_n13;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign59350_body19_e92637, assign59350_body19_e92637_d_n0, assign59350_body19_e92637_d_n2, assign59350_body19_e92637_d_n4, assign59350_body19_e92637_d_n5, assign59350_body19_e92637_d_n6, assign59350_body19_e92637_d_n7, assign59350_body19_e92637_d_n8, assign59350_body19_e92637_d_n9, assign59350_body19_e92637_d_n10, assign59350_body19_e92637_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59350_body19_e92631: f64 = (locals.var_vgp - locals.var_psl);
        let assign59350_body19_e92634: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign59350_body19_e92635: f64 = (assign59350_body19_e92631 - assign59350_body19_e92634);
        (assign59350_body19_e92635, ((locals.var_vgp_dn0 - locals.var_psl_dn0) - ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))), ((locals.var_vgp_dn2 - locals.var_psl_dn2) - ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))), ((locals.var_vgp_dn4 - locals.var_psl_dn4) - ((locals.var_fac1_dn4 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn4))), ((locals.var_vgp_dn5 - locals.var_psl_dn5) - ((locals.var_fac1_dn5 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn5))), ((locals.var_vgp_dn6 - locals.var_psl_dn6) - ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))), ((locals.var_vgp_dn7 - locals.var_psl_dn7) - ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))), ((locals.var_vgp_dn8 - locals.var_psl_dn8) - ((locals.var_fac1_dn8 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn8))), ((locals.var_vgp_dn9 - locals.var_psl_dn9) - ((locals.var_fac1_dn9 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn9))), ((locals.var_vgp_dn10 - locals.var_psl_dn10) - ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))), ((locals.var_vgp_dn13 - locals.var_psl_dn13) - ((locals.var_fac1_dn13 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn13))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn4, locals.var_fsl_dn5, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn8, locals.var_fsl_dn9, locals.var_fsl_dn10, locals.var_fsl_dn13,)
    }
};
            locals.var_fsl = assign59350_body19_e92637;
            locals.var_fsl_dn0 = assign59350_body19_e92637_d_n0;
            locals.var_fsl_dn2 = assign59350_body19_e92637_d_n2;
            locals.var_fsl_dn4 = assign59350_body19_e92637_d_n4;
            locals.var_fsl_dn5 = assign59350_body19_e92637_d_n5;
            locals.var_fsl_dn6 = assign59350_body19_e92637_d_n6;
            locals.var_fsl_dn7 = assign59350_body19_e92637_d_n7;
            locals.var_fsl_dn8 = assign59350_body19_e92637_d_n8;
            locals.var_fsl_dn9 = assign59350_body19_e92637_d_n9;
            locals.var_fsl_dn10 = assign59350_body19_e92637_d_n10;
            locals.var_fsl_dn13 = assign59350_body19_e92637_d_n13;
            locals.var_fsl_rv = 0.0;
            let (assign59350_body20_e92649, assign59350_body20_e92649_d_n0, assign59350_body20_e92649_d_n2, assign59350_body20_e92649_d_n4, assign59350_body20_e92649_d_n5, assign59350_body20_e92649_d_n6, assign59350_body20_e92649_d_n7, assign59350_body20_e92649_d_n8, assign59350_body20_e92649_d_n9, assign59350_body20_e92649_d_n10, assign59350_body20_e92649_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59350_body20_e92643: f64 = (-1.0);
        let assign59350_body20_e92646: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign59350_body20_e92647: f64 = (assign59350_body20_e92643 - assign59350_body20_e92646);
        (assign59350_body20_e92647, (-((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0))), (-((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2))), (-((locals.var_fac1_dn4 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn4))), (-((locals.var_fac1_dn5 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn5))), (-((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6))), (-((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7))), (-((locals.var_fac1_dn8 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn8))), (-((locals.var_fac1_dn9 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn9))), (-((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10))), (-((locals.var_fac1_dn13 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn13))),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn4, locals.var_fsl_dpsl_dn5, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn8, locals.var_fsl_dpsl_dn9, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn13,)
    }
};
            locals.var_fsl_dpsl = assign59350_body20_e92649;
            locals.var_fsl_dpsl_dn0 = assign59350_body20_e92649_d_n0;
            locals.var_fsl_dpsl_dn2 = assign59350_body20_e92649_d_n2;
            locals.var_fsl_dpsl_dn4 = assign59350_body20_e92649_d_n4;
            locals.var_fsl_dpsl_dn5 = assign59350_body20_e92649_d_n5;
            locals.var_fsl_dpsl_dn6 = assign59350_body20_e92649_d_n6;
            locals.var_fsl_dpsl_dn7 = assign59350_body20_e92649_d_n7;
            locals.var_fsl_dpsl_dn8 = assign59350_body20_e92649_d_n8;
            locals.var_fsl_dpsl_dn9 = assign59350_body20_e92649_d_n9;
            locals.var_fsl_dpsl_dn10 = assign59350_body20_e92649_d_n10;
            locals.var_fsl_dpsl_dn13 = assign59350_body20_e92649_d_n13;
            locals.var_fsl_dpsl_rv = 0.0;
            let assign59350_body21_e92652: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1454 = assign59350_body21_e92652;
            locals.var_guard1454_rv = 0.0;
            let (assign59350_body22_e92661,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59350_body22_e92661;
            locals.var_flg_brk2_rv = 0.0;
            let assign59350_body23_e92664: f64 = if locals.var_flg_brk2 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1455 = assign59350_body23_e92664;
            locals.var_guard1455_rv = 0.0;
            let (assign59350_body24_e92676, assign59350_body24_e92676_d_n0, assign59350_body24_e92676_d_n2, assign59350_body24_e92676_d_n4, assign59350_body24_e92676_d_n5, assign59350_body24_e92676_d_n6, assign59350_body24_e92676_d_n7, assign59350_body24_e92676_d_n8, assign59350_body24_e92676_d_n9, assign59350_body24_e92676_d_n10, assign59350_body24_e92676_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59350_body24_e92672: f64 = (-locals.var_fsl);
        let assign59350_body24_e92674: f64 = (assign59350_body24_e92672 / locals.var_fsl_dpsl);
        (assign59350_body24_e92674, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn4) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn4)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn5) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn5)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn8) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn8)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn9) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn9)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn13) * locals.var_fsl_dpsl) - (assign59350_body24_e92672 * locals.var_fsl_dpsl_dn13)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn13,)
    }
};
            locals.var_dpsl = assign59350_body24_e92676;
            locals.var_dpsl_dn0 = assign59350_body24_e92676_d_n0;
            locals.var_dpsl_dn2 = assign59350_body24_e92676_d_n2;
            locals.var_dpsl_dn4 = assign59350_body24_e92676_d_n4;
            locals.var_dpsl_dn5 = assign59350_body24_e92676_d_n5;
            locals.var_dpsl_dn6 = assign59350_body24_e92676_d_n6;
            locals.var_dpsl_dn7 = assign59350_body24_e92676_d_n7;
            locals.var_dpsl_dn8 = assign59350_body24_e92676_d_n8;
            locals.var_dpsl_dn9 = assign59350_body24_e92676_d_n9;
            locals.var_dpsl_dn10 = assign59350_body24_e92676_d_n10;
            locals.var_dpsl_dn13 = assign59350_body24_e92676_d_n13;
            locals.var_dpsl_rv = 0.0;
            let (assign59350_body25_e92698, assign59350_body25_e92698_d_n0, assign59350_body25_e92698_d_n2, assign59350_body25_e92698_d_n4, assign59350_body25_e92698_d_n5, assign59350_body25_e92698_d_n6, assign59350_body25_e92698_d_n7, assign59350_body25_e92698_d_n8, assign59350_body25_e92698_d_n9, assign59350_body25_e92698_d_n10, assign59350_body25_e92698_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59350_body25_e92685: f64 = (0.5 * 0.1);
        let assign59350_body25_e92689: f64 = (locals.var_psl).abs();
        let (assign59350_body25_e92694, assign59350_body25_e92694_d_n0, assign59350_body25_e92694_d_n2, assign59350_body25_e92694_d_n4, assign59350_body25_e92694_d_n5, assign59350_body25_e92694_d_n6, assign59350_body25_e92694_d_n7, assign59350_body25_e92694_d_n8, assign59350_body25_e92694_d_n9, assign59350_body25_e92694_d_n10, assign59350_body25_e92694_d_n13,) = {
            if (1.0 >= assign59350_body25_e92689) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59350_body25_e92693: f64 = (locals.var_psl).abs();
                (assign59350_body25_e92693, if locals.var_psl >= 0.0 { locals.var_psl_dn0 } else { (-locals.var_psl_dn0) }, if locals.var_psl >= 0.0 { locals.var_psl_dn2 } else { (-locals.var_psl_dn2) }, if locals.var_psl >= 0.0 { locals.var_psl_dn4 } else { (-locals.var_psl_dn4) }, if locals.var_psl >= 0.0 { locals.var_psl_dn5 } else { (-locals.var_psl_dn5) }, if locals.var_psl >= 0.0 { locals.var_psl_dn6 } else { (-locals.var_psl_dn6) }, if locals.var_psl >= 0.0 { locals.var_psl_dn7 } else { (-locals.var_psl_dn7) }, if locals.var_psl >= 0.0 { locals.var_psl_dn8 } else { (-locals.var_psl_dn8) }, if locals.var_psl >= 0.0 { locals.var_psl_dn9 } else { (-locals.var_psl_dn9) }, if locals.var_psl >= 0.0 { locals.var_psl_dn10 } else { (-locals.var_psl_dn10) }, if locals.var_psl >= 0.0 { locals.var_psl_dn13 } else { (-locals.var_psl_dn13) },)
            }
        };
        let assign59350_body25_e92695: f64 = (1.0 + assign59350_body25_e92694);
        let assign59350_body25_e92696: f64 = (assign59350_body25_e92685 * assign59350_body25_e92695);
        (assign59350_body25_e92696, (assign59350_body25_e92685 * assign59350_body25_e92694_d_n0), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n2), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n4), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n5), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n6), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n7), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n8), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n9), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n10), (assign59350_body25_e92685 * assign59350_body25_e92694_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign59350_body25_e92698;
            locals.var_dplim_dn0 = assign59350_body25_e92698_d_n0;
            locals.var_dplim_dn2 = assign59350_body25_e92698_d_n2;
            locals.var_dplim_dn4 = assign59350_body25_e92698_d_n4;
            locals.var_dplim_dn5 = assign59350_body25_e92698_d_n5;
            locals.var_dplim_dn6 = assign59350_body25_e92698_d_n6;
            locals.var_dplim_dn7 = assign59350_body25_e92698_d_n7;
            locals.var_dplim_dn8 = assign59350_body25_e92698_d_n8;
            locals.var_dplim_dn9 = assign59350_body25_e92698_d_n9;
            locals.var_dplim_dn10 = assign59350_body25_e92698_d_n10;
            locals.var_dplim_dn13 = assign59350_body25_e92698_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign59350_body26_e92700: f64 = (locals.var_dpsl).abs();
            let assign59350_body26_e92702: f64 = if assign59350_body26_e92700 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1456 = assign59350_body26_e92702;
            locals.var_guard1456_rv = 0.0;
            let (assign59350_body27_e92721, assign59350_body27_e92721_d_n0, assign59350_body27_e92721_d_n2, assign59350_body27_e92721_d_n4, assign59350_body27_e92721_d_n5, assign59350_body27_e92721_d_n6, assign59350_body27_e92721_d_n7, assign59350_body27_e92721_d_n8, assign59350_body27_e92721_d_n9, assign59350_body27_e92721_d_n10, assign59350_body27_e92721_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1455 != 0.0)) && (locals.var_guard1456 != 0.0)) {
        let (assign59350_body27_e92718,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign59350_body27_e92717: f64 = (-1.0);
                (assign59350_body27_e92717,)
            }
        };
        let assign59350_body27_e92719: f64 = (locals.var_dplim * assign59350_body27_e92718);
        (assign59350_body27_e92719, (locals.var_dplim_dn0 * assign59350_body27_e92718), (locals.var_dplim_dn2 * assign59350_body27_e92718), (locals.var_dplim_dn4 * assign59350_body27_e92718), (locals.var_dplim_dn5 * assign59350_body27_e92718), (locals.var_dplim_dn6 * assign59350_body27_e92718), (locals.var_dplim_dn7 * assign59350_body27_e92718), (locals.var_dplim_dn8 * assign59350_body27_e92718), (locals.var_dplim_dn9 * assign59350_body27_e92718), (locals.var_dplim_dn10 * assign59350_body27_e92718), (locals.var_dplim_dn13 * assign59350_body27_e92718),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn13,)
    }
};
            locals.var_dpsl = assign59350_body27_e92721;
            locals.var_dpsl_dn0 = assign59350_body27_e92721_d_n0;
            locals.var_dpsl_dn2 = assign59350_body27_e92721_d_n2;
            locals.var_dpsl_dn4 = assign59350_body27_e92721_d_n4;
            locals.var_dpsl_dn5 = assign59350_body27_e92721_d_n5;
            locals.var_dpsl_dn6 = assign59350_body27_e92721_d_n6;
            locals.var_dpsl_dn7 = assign59350_body27_e92721_d_n7;
            locals.var_dpsl_dn8 = assign59350_body27_e92721_d_n8;
            locals.var_dpsl_dn9 = assign59350_body27_e92721_d_n9;
            locals.var_dpsl_dn10 = assign59350_body27_e92721_d_n10;
            locals.var_dpsl_dn13 = assign59350_body27_e92721_d_n13;
            locals.var_dpsl_rv = 0.0;
            let (assign59350_body28_e92732, assign59350_body28_e92732_d_n0, assign59350_body28_e92732_d_n2, assign59350_body28_e92732_d_n4, assign59350_body28_e92732_d_n5, assign59350_body28_e92732_d_n6, assign59350_body28_e92732_d_n7, assign59350_body28_e92732_d_n8, assign59350_body28_e92732_d_n9, assign59350_body28_e92732_d_n10, assign59350_body28_e92732_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59350_body28_e92730: f64 = (locals.var_psl + locals.var_dpsl);
        (assign59350_body28_e92730, (locals.var_psl_dn0 + locals.var_dpsl_dn0), (locals.var_psl_dn2 + locals.var_dpsl_dn2), (locals.var_psl_dn4 + locals.var_dpsl_dn4), (locals.var_psl_dn5 + locals.var_dpsl_dn5), (locals.var_psl_dn6 + locals.var_dpsl_dn6), (locals.var_psl_dn7 + locals.var_dpsl_dn7), (locals.var_psl_dn8 + locals.var_dpsl_dn8), (locals.var_psl_dn9 + locals.var_dpsl_dn9), (locals.var_psl_dn10 + locals.var_dpsl_dn10), (locals.var_psl_dn13 + locals.var_dpsl_dn13),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
            locals.var_psl = assign59350_body28_e92732;
            locals.var_psl_dn0 = assign59350_body28_e92732_d_n0;
            locals.var_psl_dn2 = assign59350_body28_e92732_d_n2;
            locals.var_psl_dn4 = assign59350_body28_e92732_d_n4;
            locals.var_psl_dn5 = assign59350_body28_e92732_d_n5;
            locals.var_psl_dn6 = assign59350_body28_e92732_d_n6;
            locals.var_psl_dn7 = assign59350_body28_e92732_d_n7;
            locals.var_psl_dn8 = assign59350_body28_e92732_d_n8;
            locals.var_psl_dn9 = assign59350_body28_e92732_d_n9;
            locals.var_psl_dn10 = assign59350_body28_e92732_d_n10;
            locals.var_psl_dn13 = assign59350_body28_e92732_d_n13;
            locals.var_psl_rv = 0.0;
            let assign59350_body29_e92734: f64 = (locals.var_dpsl).abs();
            let assign59350_body29_e92738: f64 = (locals.var_fsl).abs();
            let assign59350_body29_e92741: f64 = if ((assign59350_body29_e92734 <= 1e-12) && (assign59350_body29_e92738 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1457 = assign59350_body29_e92741;
            locals.var_guard1457_rv = 0.0;
            let (assign59350_body30_e92752,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1455 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign59350_body30_e92752;
            locals.var_flg_conv_rv = 0.0;
            let (assign59350_body31_e92763,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_flg_brk2 != 0.0)) {
        let assign59350_body31_e92761: f64 = (40.0 + 1.0);
        (assign59350_body31_e92761,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59350_body31_e92763;
            locals.var_lp_sl_rv = 0.0;
            let (assign59350_body32_e92770,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59350_body32_e92770;
            locals.var_flg_brk2_rv = 0.0;
            let (assign59350_body33_e92779,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59350_body33_e92777: f64 = (locals.var_lp_sl + 1.0);
        (assign59350_body33_e92777,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59350_body33_e92779;
            locals.var_lp_sl_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_212(
        locals: &mut StampLocals,
    ) {
        let (assign59360_e92788,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59360_e92786: f64 = (locals.var_lp_sl - 1.0);
        (assign59360_e92786,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign59360_e92788;
        locals.var_lp_sl_rv = 0.0;

        let assign59380_e92794: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1459 = assign59380_e92794;
        locals.var_guard1459_rv = 0.0;

        let (assign59390_e92809, assign59390_e92809_d_n0, assign59390_e92809_d_n2, assign59390_e92809_d_n4, assign59390_e92809_d_n5, assign59390_e92809_d_n6, assign59390_e92809_d_n7, assign59390_e92809_d_n8, assign59390_e92809_d_n9, assign59390_e92809_d_n10, assign59390_e92809_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59390_e92803: f64 = (locals.var_fb * locals.var_fb);
        let assign59390_e92806: f64 = (10.0 * 2.220446049250313e-16);
        let assign59390_e92807: f64 = (assign59390_e92803 + assign59390_e92806);
        (assign59390_e92807, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn13,)
    }
};
        locals.var_xil = assign59390_e92809;
        locals.var_xil_dn0 = assign59390_e92809_d_n0;
        locals.var_xil_dn2 = assign59390_e92809_d_n2;
        locals.var_xil_dn4 = assign59390_e92809_d_n4;
        locals.var_xil_dn5 = assign59390_e92809_d_n5;
        locals.var_xil_dn6 = assign59390_e92809_d_n6;
        locals.var_xil_dn7 = assign59390_e92809_d_n7;
        locals.var_xil_dn8 = assign59390_e92809_d_n8;
        locals.var_xil_dn9 = assign59390_e92809_d_n9;
        locals.var_xil_dn10 = assign59390_e92809_d_n10;
        locals.var_xil_dn13 = assign59390_e92809_d_n13;
        locals.var_xil_rv = 0.0;

        let (assign59400_e92822, assign59400_e92822_d_n0, assign59400_e92822_d_n2, assign59400_e92822_d_n4, assign59400_e92822_d_n5, assign59400_e92822_d_n6, assign59400_e92822_d_n7, assign59400_e92822_d_n8, assign59400_e92822_d_n9, assign59400_e92822_d_n10, assign59400_e92822_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59400_e92819: f64 = (10.0 * 2.220446049250313e-16);
        let assign59400_e92820: f64 = (locals.var_fb + assign59400_e92819);
        (assign59400_e92820, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn13,)
    }
};
        locals.var_xilp12 = assign59400_e92822;
        locals.var_xilp12_dn0 = assign59400_e92822_d_n0;
        locals.var_xilp12_dn2 = assign59400_e92822_d_n2;
        locals.var_xilp12_dn4 = assign59400_e92822_d_n4;
        locals.var_xilp12_dn5 = assign59400_e92822_d_n5;
        locals.var_xilp12_dn6 = assign59400_e92822_d_n6;
        locals.var_xilp12_dn7 = assign59400_e92822_d_n7;
        locals.var_xilp12_dn8 = assign59400_e92822_d_n8;
        locals.var_xilp12_dn9 = assign59400_e92822_d_n9;
        locals.var_xilp12_dn10 = assign59400_e92822_d_n10;
        locals.var_xilp12_dn13 = assign59400_e92822_d_n13;
        locals.var_xilp12_rv = 0.0;

        let (assign59410_e92839, assign59410_e92839_d_n0, assign59410_e92839_d_n2, assign59410_e92839_d_n4, assign59410_e92839_d_n5, assign59410_e92839_d_n6, assign59410_e92839_d_n7, assign59410_e92839_d_n8, assign59410_e92839_d_n9, assign59410_e92839_d_n10, assign59410_e92839_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59410_e92831: f64 = (locals.var_fb * locals.var_fb);
        let assign59410_e92833: f64 = (assign59410_e92831 * locals.var_fb);
        let assign59410_e92836: f64 = (10.0 * 2.220446049250313e-16);
        let assign59410_e92837: f64 = (assign59410_e92833 + assign59410_e92836);
        (assign59410_e92837, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn0)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn2)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn4)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn5)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn6)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn7)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn8)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn9)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn10)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) * locals.var_fb) + (assign59410_e92831 * locals.var_fb_dn13)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn13,)
    }
};
        locals.var_xilp32 = assign59410_e92839;
        locals.var_xilp32_dn0 = assign59410_e92839_d_n0;
        locals.var_xilp32_dn2 = assign59410_e92839_d_n2;
        locals.var_xilp32_dn4 = assign59410_e92839_d_n4;
        locals.var_xilp32_dn5 = assign59410_e92839_d_n5;
        locals.var_xilp32_dn6 = assign59410_e92839_d_n6;
        locals.var_xilp32_dn7 = assign59410_e92839_d_n7;
        locals.var_xilp32_dn8 = assign59410_e92839_d_n8;
        locals.var_xilp32_dn9 = assign59410_e92839_d_n9;
        locals.var_xilp32_dn10 = assign59410_e92839_d_n10;
        locals.var_xilp32_dn13 = assign59410_e92839_d_n13;
        locals.var_xilp32_rv = 0.0;

        let (assign59420_e92851, assign59420_e92851_d_n0, assign59420_e92851_d_n2, assign59420_e92851_d_n4, assign59420_e92851_d_n5, assign59420_e92851_d_n6, assign59420_e92851_d_n7, assign59420_e92851_d_n8, assign59420_e92851_d_n9, assign59420_e92851_d_n10, assign59420_e92851_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 == 0.0)) {
        let assign59420_e92849: f64 = (locals.var_chi - 1.0);
        (assign59420_e92849, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn13,)
    }
};
        locals.var_xil = assign59420_e92851;
        locals.var_xil_dn0 = assign59420_e92851_d_n0;
        locals.var_xil_dn2 = assign59420_e92851_d_n2;
        locals.var_xil_dn4 = assign59420_e92851_d_n4;
        locals.var_xil_dn5 = assign59420_e92851_d_n5;
        locals.var_xil_dn6 = assign59420_e92851_d_n6;
        locals.var_xil_dn7 = assign59420_e92851_d_n7;
        locals.var_xil_dn8 = assign59420_e92851_d_n8;
        locals.var_xil_dn9 = assign59420_e92851_d_n9;
        locals.var_xil_dn10 = assign59420_e92851_d_n10;
        locals.var_xil_dn13 = assign59420_e92851_d_n13;
        locals.var_xil_rv = 0.0;

        let (assign59430_e92862, assign59430_e92862_d_n0, assign59430_e92862_d_n2, assign59430_e92862_d_n4, assign59430_e92862_d_n5, assign59430_e92862_d_n6, assign59430_e92862_d_n7, assign59430_e92862_d_n8, assign59430_e92862_d_n9, assign59430_e92862_d_n10, assign59430_e92862_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 == 0.0)) {
        let assign59430_e92860: f64 = (locals.var_xil).sqrt();
        (assign59430_e92860, (locals.var_xil_dn0 / (2.0 * assign59430_e92860)), (locals.var_xil_dn2 / (2.0 * assign59430_e92860)), (locals.var_xil_dn4 / (2.0 * assign59430_e92860)), (locals.var_xil_dn5 / (2.0 * assign59430_e92860)), (locals.var_xil_dn6 / (2.0 * assign59430_e92860)), (locals.var_xil_dn7 / (2.0 * assign59430_e92860)), (locals.var_xil_dn8 / (2.0 * assign59430_e92860)), (locals.var_xil_dn9 / (2.0 * assign59430_e92860)), (locals.var_xil_dn10 / (2.0 * assign59430_e92860)), (locals.var_xil_dn13 / (2.0 * assign59430_e92860)),)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn13,)
    }
};
        locals.var_xilp12 = assign59430_e92862;
        locals.var_xilp12_dn0 = assign59430_e92862_d_n0;
        locals.var_xilp12_dn2 = assign59430_e92862_d_n2;
        locals.var_xilp12_dn4 = assign59430_e92862_d_n4;
        locals.var_xilp12_dn5 = assign59430_e92862_d_n5;
        locals.var_xilp12_dn6 = assign59430_e92862_d_n6;
        locals.var_xilp12_dn7 = assign59430_e92862_d_n7;
        locals.var_xilp12_dn8 = assign59430_e92862_d_n8;
        locals.var_xilp12_dn9 = assign59430_e92862_d_n9;
        locals.var_xilp12_dn10 = assign59430_e92862_d_n10;
        locals.var_xilp12_dn13 = assign59430_e92862_d_n13;
        locals.var_xilp12_rv = 0.0;

        let (assign59440_e92874, assign59440_e92874_d_n0, assign59440_e92874_d_n2, assign59440_e92874_d_n4, assign59440_e92874_d_n5, assign59440_e92874_d_n6, assign59440_e92874_d_n7, assign59440_e92874_d_n8, assign59440_e92874_d_n9, assign59440_e92874_d_n10, assign59440_e92874_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1459 == 0.0)) {
        let assign59440_e92872: f64 = (locals.var_xil * locals.var_xilp12);
        (assign59440_e92872, ((locals.var_xil_dn0 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn2)), ((locals.var_xil_dn4 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn4)), ((locals.var_xil_dn5 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn5)), ((locals.var_xil_dn6 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn6)), ((locals.var_xil_dn7 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn7)), ((locals.var_xil_dn8 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn8)), ((locals.var_xil_dn9 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn9)), ((locals.var_xil_dn10 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn10)), ((locals.var_xil_dn13 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn13)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn13,)
    }
};
        locals.var_xilp32 = assign59440_e92874;
        locals.var_xilp32_dn0 = assign59440_e92874_d_n0;
        locals.var_xilp32_dn2 = assign59440_e92874_d_n2;
        locals.var_xilp32_dn4 = assign59440_e92874_d_n4;
        locals.var_xilp32_dn5 = assign59440_e92874_d_n5;
        locals.var_xilp32_dn6 = assign59440_e92874_d_n6;
        locals.var_xilp32_dn7 = assign59440_e92874_d_n7;
        locals.var_xilp32_dn8 = assign59440_e92874_d_n8;
        locals.var_xilp32_dn9 = assign59440_e92874_d_n9;
        locals.var_xilp32_dn10 = assign59440_e92874_d_n10;
        locals.var_xilp32_dn13 = assign59440_e92874_d_n13;
        locals.var_xilp32_rv = 0.0;

        let (assign59450_e92883, assign59450_e92883_d_n0, assign59450_e92883_d_n2, assign59450_e92883_d_n4, assign59450_e92883_d_n5, assign59450_e92883_d_n6, assign59450_e92883_d_n7, assign59450_e92883_d_n8, assign59450_e92883_d_n9, assign59450_e92883_d_n10, assign59450_e92883_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59450_e92881: f64 = (locals.var_psl - locals.var_ps0);
        (assign59450_e92881, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn4 - locals.var_ps0_dn4), (locals.var_psl_dn5 - locals.var_ps0_dn5), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn8 - locals.var_ps0_dn8), (locals.var_psl_dn9 - locals.var_ps0_dn9), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn13 - locals.var_ps0_dn13),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign59450_e92883;
        locals.var_pds_dn0 = assign59450_e92883_d_n0;
        locals.var_pds_dn2 = assign59450_e92883_d_n2;
        locals.var_pds_dn4 = assign59450_e92883_d_n4;
        locals.var_pds_dn5 = assign59450_e92883_d_n5;
        locals.var_pds_dn6 = assign59450_e92883_d_n6;
        locals.var_pds_dn7 = assign59450_e92883_d_n7;
        locals.var_pds_dn8 = assign59450_e92883_d_n8;
        locals.var_pds_dn9 = assign59450_e92883_d_n9;
        locals.var_pds_dn10 = assign59450_e92883_d_n10;
        locals.var_pds_dn13 = assign59450_e92883_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign59460_e92890, assign59460_e92890_d_n0, assign59460_e92890_d_n2, assign59460_e92890_d_n4, assign59460_e92890_d_n5, assign59460_e92890_d_n6, assign59460_e92890_d_n7, assign59460_e92890_d_n8, assign59460_e92890_d_n9, assign59460_e92890_d_n10, assign59460_e92890_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign59460_e92890;
        locals.var_vds_dn0 = assign59460_e92890_d_n0;
        locals.var_vds_dn2 = assign59460_e92890_d_n2;
        locals.var_vds_dn4 = assign59460_e92890_d_n4;
        locals.var_vds_dn5 = assign59460_e92890_d_n5;
        locals.var_vds_dn6 = assign59460_e92890_d_n6;
        locals.var_vds_dn7 = assign59460_e92890_d_n7;
        locals.var_vds_dn8 = assign59460_e92890_d_n8;
        locals.var_vds_dn9 = assign59460_e92890_d_n9;
        locals.var_vds_dn10 = assign59460_e92890_d_n10;
        locals.var_vds_dn13 = assign59460_e92890_d_n13;
        locals.var_vds_rv = 0.0;

        let (assign59470_e92899, assign59470_e92899_d_n0, assign59470_e92899_d_n2, assign59470_e92899_d_n4, assign59470_e92899_d_n5, assign59470_e92899_d_n6, assign59470_e92899_d_n7, assign59470_e92899_d_n8, assign59470_e92899_d_n9, assign59470_e92899_d_n10, assign59470_e92899_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59470_e92897: f64 = (locals.var_beta / locals.var_xi0);
        (assign59470_e92897, (((locals.var_beta_dn0 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn0)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn2 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn2)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn4 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn4)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn5 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn5)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn6 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn6)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn7 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn7)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn8 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn8)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn9 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn9)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn13 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn13)) / (locals.var_xi0 * locals.var_xi0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59470_e92899;
        locals.var_t1_dn0 = assign59470_e92899_d_n0;
        locals.var_t1_dn2 = assign59470_e92899_d_n2;
        locals.var_t1_dn4 = assign59470_e92899_d_n4;
        locals.var_t1_dn5 = assign59470_e92899_d_n5;
        locals.var_t1_dn6 = assign59470_e92899_d_n6;
        locals.var_t1_dn7 = assign59470_e92899_d_n7;
        locals.var_t1_dn8 = assign59470_e92899_d_n8;
        locals.var_t1_dn9 = assign59470_e92899_d_n9;
        locals.var_t1_dn10 = assign59470_e92899_d_n10;
        locals.var_t1_dn13 = assign59470_e92899_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59480_e92908, assign59480_e92908_d_n0, assign59480_e92908_d_n2, assign59480_e92908_d_n4, assign59480_e92908_d_n5, assign59480_e92908_d_n6, assign59480_e92908_d_n7, assign59480_e92908_d_n8, assign59480_e92908_d_n9, assign59480_e92908_d_n10, assign59480_e92908_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59480_e92906: f64 = (locals.var_t1 * locals.var_pds);
        (assign59480_e92906, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn4 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn4)), ((locals.var_t1_dn5 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn5)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn8 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn8)), ((locals.var_t1_dn9 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn9)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn13 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn13)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn13,)
    }
};
        locals.var_eta = assign59480_e92908;
        locals.var_eta_dn0 = assign59480_e92908_d_n0;
        locals.var_eta_dn2 = assign59480_e92908_d_n2;
        locals.var_eta_dn4 = assign59480_e92908_d_n4;
        locals.var_eta_dn5 = assign59480_e92908_d_n5;
        locals.var_eta_dn6 = assign59480_e92908_d_n6;
        locals.var_eta_dn7 = assign59480_e92908_d_n7;
        locals.var_eta_dn8 = assign59480_e92908_d_n8;
        locals.var_eta_dn9 = assign59480_e92908_d_n9;
        locals.var_eta_dn10 = assign59480_e92908_d_n10;
        locals.var_eta_dn13 = assign59480_e92908_d_n13;
        locals.var_eta_rv = 0.0;

        let (assign59490_e92917, assign59490_e92917_d_n0, assign59490_e92917_d_n2, assign59490_e92917_d_n4, assign59490_e92917_d_n5, assign59490_e92917_d_n6, assign59490_e92917_d_n7, assign59490_e92917_d_n8, assign59490_e92917_d_n9, assign59490_e92917_d_n10, assign59490_e92917_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59490_e92915: f64 = (locals.var_eta + 1.0);
        (assign59490_e92915, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn13,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn4, locals.var_eta1_dn5, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn8, locals.var_eta1_dn9, locals.var_eta1_dn10, locals.var_eta1_dn13,)
    }
};
        locals.var_eta1 = assign59490_e92917;
        locals.var_eta1_dn0 = assign59490_e92917_d_n0;
        locals.var_eta1_dn2 = assign59490_e92917_d_n2;
        locals.var_eta1_dn4 = assign59490_e92917_d_n4;
        locals.var_eta1_dn5 = assign59490_e92917_d_n5;
        locals.var_eta1_dn6 = assign59490_e92917_d_n6;
        locals.var_eta1_dn7 = assign59490_e92917_d_n7;
        locals.var_eta1_dn8 = assign59490_e92917_d_n8;
        locals.var_eta1_dn9 = assign59490_e92917_d_n9;
        locals.var_eta1_dn10 = assign59490_e92917_d_n10;
        locals.var_eta1_dn13 = assign59490_e92917_d_n13;
        locals.var_eta1_rv = 0.0;

        let (assign59500_e92925, assign59500_e92925_d_n0, assign59500_e92925_d_n2, assign59500_e92925_d_n4, assign59500_e92925_d_n5, assign59500_e92925_d_n6, assign59500_e92925_d_n7, assign59500_e92925_d_n8, assign59500_e92925_d_n9, assign59500_e92925_d_n10, assign59500_e92925_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59500_e92923: f64 = (locals.var_eta1).sqrt();
        (assign59500_e92923, (locals.var_eta1_dn0 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn2 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn4 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn5 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn6 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn7 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn8 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn9 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn10 / (2.0 * assign59500_e92923)), (locals.var_eta1_dn13 / (2.0 * assign59500_e92923)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn4, locals.var_eta1p12_dn5, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn8, locals.var_eta1p12_dn9, locals.var_eta1p12_dn10, locals.var_eta1p12_dn13,)
    }
};
        locals.var_eta1p12 = assign59500_e92925;
        locals.var_eta1p12_dn0 = assign59500_e92925_d_n0;
        locals.var_eta1p12_dn2 = assign59500_e92925_d_n2;
        locals.var_eta1p12_dn4 = assign59500_e92925_d_n4;
        locals.var_eta1p12_dn5 = assign59500_e92925_d_n5;
        locals.var_eta1p12_dn6 = assign59500_e92925_d_n6;
        locals.var_eta1p12_dn7 = assign59500_e92925_d_n7;
        locals.var_eta1p12_dn8 = assign59500_e92925_d_n8;
        locals.var_eta1p12_dn9 = assign59500_e92925_d_n9;
        locals.var_eta1p12_dn10 = assign59500_e92925_d_n10;
        locals.var_eta1p12_dn13 = assign59500_e92925_d_n13;
        locals.var_eta1p12_rv = 0.0;

        let (assign59510_e92934, assign59510_e92934_d_n0, assign59510_e92934_d_n2, assign59510_e92934_d_n4, assign59510_e92934_d_n5, assign59510_e92934_d_n6, assign59510_e92934_d_n7, assign59510_e92934_d_n8, assign59510_e92934_d_n9, assign59510_e92934_d_n10, assign59510_e92934_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59510_e92932: f64 = (locals.var_eta1p12 * locals.var_eta1);
        (assign59510_e92932, ((locals.var_eta1p12_dn0 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn0)), ((locals.var_eta1p12_dn2 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn2)), ((locals.var_eta1p12_dn4 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn4)), ((locals.var_eta1p12_dn5 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn5)), ((locals.var_eta1p12_dn6 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn6)), ((locals.var_eta1p12_dn7 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn7)), ((locals.var_eta1p12_dn8 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn8)), ((locals.var_eta1p12_dn9 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn9)), ((locals.var_eta1p12_dn10 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn10)), ((locals.var_eta1p12_dn13 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn13)),)
    } else {
        (locals.var_eta1p32, locals.var_eta1p32_dn0, locals.var_eta1p32_dn2, locals.var_eta1p32_dn4, locals.var_eta1p32_dn5, locals.var_eta1p32_dn6, locals.var_eta1p32_dn7, locals.var_eta1p32_dn8, locals.var_eta1p32_dn9, locals.var_eta1p32_dn10, locals.var_eta1p32_dn13,)
    }
};
        locals.var_eta1p32 = assign59510_e92934;
        locals.var_eta1p32_dn0 = assign59510_e92934_d_n0;
        locals.var_eta1p32_dn2 = assign59510_e92934_d_n2;
        locals.var_eta1p32_dn4 = assign59510_e92934_d_n4;
        locals.var_eta1p32_dn5 = assign59510_e92934_d_n5;
        locals.var_eta1p32_dn6 = assign59510_e92934_d_n6;
        locals.var_eta1p32_dn7 = assign59510_e92934_d_n7;
        locals.var_eta1p32_dn8 = assign59510_e92934_d_n8;
        locals.var_eta1p32_dn9 = assign59510_e92934_d_n9;
        locals.var_eta1p32_dn10 = assign59510_e92934_d_n10;
        locals.var_eta1p32_dn13 = assign59510_e92934_d_n13;
        locals.var_eta1p32_rv = 0.0;

        let (assign59520_e92943, assign59520_e92943_d_n0, assign59520_e92943_d_n2, assign59520_e92943_d_n4, assign59520_e92943_d_n5, assign59520_e92943_d_n6, assign59520_e92943_d_n7, assign59520_e92943_d_n8, assign59520_e92943_d_n9, assign59520_e92943_d_n10, assign59520_e92943_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59520_e92941: f64 = (locals.var_eta1p32 * locals.var_eta1);
        (assign59520_e92941, ((locals.var_eta1p32_dn0 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn0)), ((locals.var_eta1p32_dn2 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn2)), ((locals.var_eta1p32_dn4 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn4)), ((locals.var_eta1p32_dn5 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn5)), ((locals.var_eta1p32_dn6 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn6)), ((locals.var_eta1p32_dn7 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn7)), ((locals.var_eta1p32_dn8 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn8)), ((locals.var_eta1p32_dn9 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn9)), ((locals.var_eta1p32_dn10 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn10)), ((locals.var_eta1p32_dn13 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn13)),)
    } else {
        (locals.var_eta1p52, locals.var_eta1p52_dn0, locals.var_eta1p52_dn2, locals.var_eta1p52_dn4, locals.var_eta1p52_dn5, locals.var_eta1p52_dn6, locals.var_eta1p52_dn7, locals.var_eta1p52_dn8, locals.var_eta1p52_dn9, locals.var_eta1p52_dn10, locals.var_eta1p52_dn13,)
    }
};
        locals.var_eta1p52 = assign59520_e92943;
        locals.var_eta1p52_dn0 = assign59520_e92943_d_n0;
        locals.var_eta1p52_dn2 = assign59520_e92943_d_n2;
        locals.var_eta1p52_dn4 = assign59520_e92943_d_n4;
        locals.var_eta1p52_dn5 = assign59520_e92943_d_n5;
        locals.var_eta1p52_dn6 = assign59520_e92943_d_n6;
        locals.var_eta1p52_dn7 = assign59520_e92943_d_n7;
        locals.var_eta1p52_dn8 = assign59520_e92943_d_n8;
        locals.var_eta1p52_dn9 = assign59520_e92943_d_n9;
        locals.var_eta1p52_dn10 = assign59520_e92943_d_n10;
        locals.var_eta1p52_dn13 = assign59520_e92943_d_n13;
        locals.var_eta1p52_rv = 0.0;

        let (assign59530_e92954, assign59530_e92954_d_n0, assign59530_e92954_d_n2, assign59530_e92954_d_n4, assign59530_e92954_d_n5, assign59530_e92954_d_n6, assign59530_e92954_d_n7, assign59530_e92954_d_n8, assign59530_e92954_d_n9, assign59530_e92954_d_n10, assign59530_e92954_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59530_e92951: f64 = (locals.var_eta1p12 + 1.0);
        let assign59530_e92952: f64 = (1.0 / assign59530_e92951);
        (assign59530_e92952, (-(locals.var_eta1p12_dn0 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn2 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn4 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn5 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn6 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn7 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn8 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn9 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn10 / (assign59530_e92951 * assign59530_e92951))), (-(locals.var_eta1p12_dn13 / (assign59530_e92951 * assign59530_e92951))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn4, locals.var_zeta12_dn5, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn8, locals.var_zeta12_dn9, locals.var_zeta12_dn10, locals.var_zeta12_dn13,)
    }
};
        locals.var_zeta12 = assign59530_e92954;
        locals.var_zeta12_dn0 = assign59530_e92954_d_n0;
        locals.var_zeta12_dn2 = assign59530_e92954_d_n2;
        locals.var_zeta12_dn4 = assign59530_e92954_d_n4;
        locals.var_zeta12_dn5 = assign59530_e92954_d_n5;
        locals.var_zeta12_dn6 = assign59530_e92954_d_n6;
        locals.var_zeta12_dn7 = assign59530_e92954_d_n7;
        locals.var_zeta12_dn8 = assign59530_e92954_d_n8;
        locals.var_zeta12_dn9 = assign59530_e92954_d_n9;
        locals.var_zeta12_dn10 = assign59530_e92954_d_n10;
        locals.var_zeta12_dn13 = assign59530_e92954_d_n13;
        locals.var_zeta12_rv = 0.0;

        let (assign59540_e92965, assign59540_e92965_d_n0, assign59540_e92965_d_n2, assign59540_e92965_d_n4, assign59540_e92965_d_n5, assign59540_e92965_d_n6, assign59540_e92965_d_n7, assign59540_e92965_d_n8, assign59540_e92965_d_n9, assign59540_e92965_d_n10, assign59540_e92965_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59540_e92962: f64 = (locals.var_eta1p32 + 1.0);
        let assign59540_e92963: f64 = (1.0 / assign59540_e92962);
        (assign59540_e92963, (-(locals.var_eta1p32_dn0 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn2 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn4 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn5 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn6 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn7 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn8 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn9 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn10 / (assign59540_e92962 * assign59540_e92962))), (-(locals.var_eta1p32_dn13 / (assign59540_e92962 * assign59540_e92962))),)
    } else {
        (locals.var_zeta32, locals.var_zeta32_dn0, locals.var_zeta32_dn2, locals.var_zeta32_dn4, locals.var_zeta32_dn5, locals.var_zeta32_dn6, locals.var_zeta32_dn7, locals.var_zeta32_dn8, locals.var_zeta32_dn9, locals.var_zeta32_dn10, locals.var_zeta32_dn13,)
    }
};
        locals.var_zeta32 = assign59540_e92965;
        locals.var_zeta32_dn0 = assign59540_e92965_d_n0;
        locals.var_zeta32_dn2 = assign59540_e92965_d_n2;
        locals.var_zeta32_dn4 = assign59540_e92965_d_n4;
        locals.var_zeta32_dn5 = assign59540_e92965_d_n5;
        locals.var_zeta32_dn6 = assign59540_e92965_d_n6;
        locals.var_zeta32_dn7 = assign59540_e92965_d_n7;
        locals.var_zeta32_dn8 = assign59540_e92965_d_n8;
        locals.var_zeta32_dn9 = assign59540_e92965_d_n9;
        locals.var_zeta32_dn10 = assign59540_e92965_d_n10;
        locals.var_zeta32_dn13 = assign59540_e92965_d_n13;
        locals.var_zeta32_rv = 0.0;

        let (assign59550_e92976, assign59550_e92976_d_n0, assign59550_e92976_d_n2, assign59550_e92976_d_n4, assign59550_e92976_d_n5, assign59550_e92976_d_n6, assign59550_e92976_d_n7, assign59550_e92976_d_n8, assign59550_e92976_d_n9, assign59550_e92976_d_n10, assign59550_e92976_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59550_e92973: f64 = (locals.var_eta1p52 + 1.0);
        let assign59550_e92974: f64 = (1.0 / assign59550_e92973);
        (assign59550_e92974, (-(locals.var_eta1p52_dn0 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn2 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn4 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn5 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn6 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn7 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn8 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn9 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn10 / (assign59550_e92973 * assign59550_e92973))), (-(locals.var_eta1p52_dn13 / (assign59550_e92973 * assign59550_e92973))),)
    } else {
        (locals.var_zeta52, locals.var_zeta52_dn0, locals.var_zeta52_dn2, locals.var_zeta52_dn4, locals.var_zeta52_dn5, locals.var_zeta52_dn6, locals.var_zeta52_dn7, locals.var_zeta52_dn8, locals.var_zeta52_dn9, locals.var_zeta52_dn10, locals.var_zeta52_dn13,)
    }
};
        locals.var_zeta52 = assign59550_e92976;
        locals.var_zeta52_dn0 = assign59550_e92976_d_n0;
        locals.var_zeta52_dn2 = assign59550_e92976_d_n2;
        locals.var_zeta52_dn4 = assign59550_e92976_d_n4;
        locals.var_zeta52_dn5 = assign59550_e92976_d_n5;
        locals.var_zeta52_dn6 = assign59550_e92976_d_n6;
        locals.var_zeta52_dn7 = assign59550_e92976_d_n7;
        locals.var_zeta52_dn8 = assign59550_e92976_d_n8;
        locals.var_zeta52_dn9 = assign59550_e92976_d_n9;
        locals.var_zeta52_dn10 = assign59550_e92976_d_n10;
        locals.var_zeta52_dn13 = assign59550_e92976_d_n13;
        locals.var_zeta52_rv = 0.0;

        let (assign59560_e92985, assign59560_e92985_d_n0, assign59560_e92985_d_n2, assign59560_e92985_d_n4, assign59560_e92985_d_n5, assign59560_e92985_d_n6, assign59560_e92985_d_n7, assign59560_e92985_d_n8, assign59560_e92985_d_n9, assign59560_e92985_d_n10, assign59560_e92985_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59560_e92983: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign59560_e92983, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn4 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn4)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn5 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn5)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn8 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn8)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn9 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn9)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn13 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn13)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn4, locals.var_f00_dn5, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn8, locals.var_f00_dn9, locals.var_f00_dn10, locals.var_f00_dn13,)
    }
};
        locals.var_f00 = assign59560_e92985;
        locals.var_f00_dn0 = assign59560_e92985_d_n0;
        locals.var_f00_dn2 = assign59560_e92985_d_n2;
        locals.var_f00_dn4 = assign59560_e92985_d_n4;
        locals.var_f00_dn5 = assign59560_e92985_d_n5;
        locals.var_f00_dn6 = assign59560_e92985_d_n6;
        locals.var_f00_dn7 = assign59560_e92985_d_n7;
        locals.var_f00_dn8 = assign59560_e92985_d_n8;
        locals.var_f00_dn9 = assign59560_e92985_d_n9;
        locals.var_f00_dn10 = assign59560_e92985_d_n10;
        locals.var_f00_dn13 = assign59560_e92985_d_n13;
        locals.var_f00_rv = 0.0;

        let (assign59570_e92998, assign59570_e92998_d_n0, assign59570_e92998_d_n2, assign59570_e92998_d_n4, assign59570_e92998_d_n5, assign59570_e92998_d_n6, assign59570_e92998_d_n7, assign59570_e92998_d_n8, assign59570_e92998_d_n9, assign59570_e92998_d_n10, assign59570_e92998_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59570_e92994: f64 = (3.0 + locals.var_eta);
        let assign59570_e92995: f64 = (locals.var_eta * assign59570_e92994);
        let assign59570_e92996: f64 = (3.0 + assign59570_e92995);
        (assign59570_e92996, ((locals.var_eta_dn0 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn0)), ((locals.var_eta_dn2 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn2)), ((locals.var_eta_dn4 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn4)), ((locals.var_eta_dn5 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn5)), ((locals.var_eta_dn6 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn6)), ((locals.var_eta_dn7 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn7)), ((locals.var_eta_dn8 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn8)), ((locals.var_eta_dn9 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn9)), ((locals.var_eta_dn10 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn10)), ((locals.var_eta_dn13 * assign59570_e92994) + (locals.var_eta * locals.var_eta_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59570_e92998;
        locals.var_t1_dn0 = assign59570_e92998_d_n0;
        locals.var_t1_dn2 = assign59570_e92998_d_n2;
        locals.var_t1_dn4 = assign59570_e92998_d_n4;
        locals.var_t1_dn5 = assign59570_e92998_d_n5;
        locals.var_t1_dn6 = assign59570_e92998_d_n6;
        locals.var_t1_dn7 = assign59570_e92998_d_n7;
        locals.var_t1_dn8 = assign59570_e92998_d_n8;
        locals.var_t1_dn9 = assign59570_e92998_d_n9;
        locals.var_t1_dn10 = assign59570_e92998_d_n10;
        locals.var_t1_dn13 = assign59570_e92998_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59580_e93011, assign59580_e93011_d_n0, assign59580_e93011_d_n2, assign59580_e93011_d_n4, assign59580_e93011_d_n5, assign59580_e93011_d_n6, assign59580_e93011_d_n7, assign59580_e93011_d_n8, assign59580_e93011_d_n9, assign59580_e93011_d_n10, assign59580_e93011_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59580_e93005: f64 = (0.6666666666666667 * locals.var_xi0p12);
        let assign59580_e93007: f64 = (assign59580_e93005 * locals.var_zeta32);
        let assign59580_e93009: f64 = (assign59580_e93007 * locals.var_t1);
        (assign59580_e93009, (((((0.6666666666666667 * locals.var_xi0p12_dn0) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn0)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn0)), (((((0.6666666666666667 * locals.var_xi0p12_dn2) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn2)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn2)), (((((0.6666666666666667 * locals.var_xi0p12_dn4) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn4)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn4)), (((((0.6666666666666667 * locals.var_xi0p12_dn5) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn5)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn5)), (((((0.6666666666666667 * locals.var_xi0p12_dn6) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn6)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn6)), (((((0.6666666666666667 * locals.var_xi0p12_dn7) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn7)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn7)), (((((0.6666666666666667 * locals.var_xi0p12_dn8) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn8)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn8)), (((((0.6666666666666667 * locals.var_xi0p12_dn9) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn9)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn9)), (((((0.6666666666666667 * locals.var_xi0p12_dn10) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn10)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn10)), (((((0.6666666666666667 * locals.var_xi0p12_dn13) * locals.var_zeta32) + (assign59580_e93005 * locals.var_zeta32_dn13)) * locals.var_t1) + (assign59580_e93007 * locals.var_t1_dn13)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn4, locals.var_f10_dn5, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn8, locals.var_f10_dn9, locals.var_f10_dn10, locals.var_f10_dn13,)
    }
};
        locals.var_f10 = assign59580_e93011;
        locals.var_f10_dn0 = assign59580_e93011_d_n0;
        locals.var_f10_dn2 = assign59580_e93011_d_n2;
        locals.var_f10_dn4 = assign59580_e93011_d_n4;
        locals.var_f10_dn5 = assign59580_e93011_d_n5;
        locals.var_f10_dn6 = assign59580_e93011_d_n6;
        locals.var_f10_dn7 = assign59580_e93011_d_n7;
        locals.var_f10_dn8 = assign59580_e93011_d_n8;
        locals.var_f10_dn9 = assign59580_e93011_d_n9;
        locals.var_f10_dn10 = assign59580_e93011_d_n10;
        locals.var_f10_dn13 = assign59580_e93011_d_n13;
        locals.var_f10_rv = 0.0;

        let (assign59590_e93032, assign59590_e93032_d_n0, assign59590_e93032_d_n2, assign59590_e93032_d_n4, assign59590_e93032_d_n5, assign59590_e93032_d_n6, assign59590_e93032_d_n7, assign59590_e93032_d_n8, assign59590_e93032_d_n9, assign59590_e93032_d_n10, assign59590_e93032_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59590_e93024: f64 = (5.0 + locals.var_eta);
        let assign59590_e93025: f64 = (locals.var_eta * assign59590_e93024);
        let assign59590_e93026: f64 = (10.0 + assign59590_e93025);
        let assign59590_e93027: f64 = (locals.var_eta * assign59590_e93026);
        let assign59590_e93028: f64 = (10.0 + assign59590_e93027);
        let assign59590_e93029: f64 = (locals.var_eta * assign59590_e93028);
        let assign59590_e93030: f64 = (5.0 + assign59590_e93029);
        (assign59590_e93030, ((locals.var_eta_dn0 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn0 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn0 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn0)))))), ((locals.var_eta_dn2 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn2 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn2 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn2)))))), ((locals.var_eta_dn4 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn4 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn4 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn4)))))), ((locals.var_eta_dn5 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn5 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn5 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn5)))))), ((locals.var_eta_dn6 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn6 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn6 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn6)))))), ((locals.var_eta_dn7 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn7 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn7 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn7)))))), ((locals.var_eta_dn8 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn8 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn8 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn8)))))), ((locals.var_eta_dn9 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn9 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn9 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn9)))))), ((locals.var_eta_dn10 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn10 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn10 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn10)))))), ((locals.var_eta_dn13 * assign59590_e93028) + (locals.var_eta * ((locals.var_eta_dn13 * assign59590_e93026) + (locals.var_eta * ((locals.var_eta_dn13 * assign59590_e93024) + (locals.var_eta * locals.var_eta_dn13)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59590_e93032;
        locals.var_t1_dn0 = assign59590_e93032_d_n0;
        locals.var_t1_dn2 = assign59590_e93032_d_n2;
        locals.var_t1_dn4 = assign59590_e93032_d_n4;
        locals.var_t1_dn5 = assign59590_e93032_d_n5;
        locals.var_t1_dn6 = assign59590_e93032_d_n6;
        locals.var_t1_dn7 = assign59590_e93032_d_n7;
        locals.var_t1_dn8 = assign59590_e93032_d_n8;
        locals.var_t1_dn9 = assign59590_e93032_d_n9;
        locals.var_t1_dn10 = assign59590_e93032_d_n10;
        locals.var_t1_dn13 = assign59590_e93032_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59600_e93049, assign59600_e93049_d_n0, assign59600_e93049_d_n2, assign59600_e93049_d_n4, assign59600_e93049_d_n5, assign59600_e93049_d_n6, assign59600_e93049_d_n7, assign59600_e93049_d_n8, assign59600_e93049_d_n9, assign59600_e93049_d_n10, assign59600_e93049_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59600_e93040: f64 = (15.0 * locals.var_beta);
        let assign59600_e93041: f64 = (4.0 / assign59600_e93040);
        let assign59600_e93043: f64 = (assign59600_e93041 * locals.var_xi0p32);
        let assign59600_e93045: f64 = (assign59600_e93043 * locals.var_zeta52);
        let assign59600_e93047: f64 = (assign59600_e93045 * locals.var_t1);
        (assign59600_e93047, (((((((-((4.0 * (15.0 * locals.var_beta_dn0)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn0)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn0)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn0)), (((((((-((4.0 * (15.0 * locals.var_beta_dn2)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn2)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn2)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn2)), (((((((-((4.0 * (15.0 * locals.var_beta_dn4)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn4)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn4)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn4)), (((((((-((4.0 * (15.0 * locals.var_beta_dn5)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn5)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn5)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn5)), (((((((-((4.0 * (15.0 * locals.var_beta_dn6)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn6)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn6)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn6)), (((((((-((4.0 * (15.0 * locals.var_beta_dn7)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn7)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn7)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn7)), (((((((-((4.0 * (15.0 * locals.var_beta_dn8)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn8)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn8)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn8)), (((((((-((4.0 * (15.0 * locals.var_beta_dn9)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn9)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn9)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn9)), (((((((-((4.0 * (15.0 * locals.var_beta_dn10)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn10)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn10)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn10)), (((((((-((4.0 * (15.0 * locals.var_beta_dn13)) / (assign59600_e93040 * assign59600_e93040))) * locals.var_xi0p32) + (assign59600_e93041 * locals.var_xi0p32_dn13)) * locals.var_zeta52) + (assign59600_e93043 * locals.var_zeta52_dn13)) * locals.var_t1) + (assign59600_e93045 * locals.var_t1_dn13)),)
    } else {
        (locals.var_f30, locals.var_f30_dn0, locals.var_f30_dn2, locals.var_f30_dn4, locals.var_f30_dn5, locals.var_f30_dn6, locals.var_f30_dn7, locals.var_f30_dn8, locals.var_f30_dn9, locals.var_f30_dn10, locals.var_f30_dn13,)
    }
};
        locals.var_f30 = assign59600_e93049;
        locals.var_f30_dn0 = assign59600_e93049_d_n0;
        locals.var_f30_dn2 = assign59600_e93049_d_n2;
        locals.var_f30_dn4 = assign59600_e93049_d_n4;
        locals.var_f30_dn5 = assign59600_e93049_d_n5;
        locals.var_f30_dn6 = assign59600_e93049_d_n6;
        locals.var_f30_dn7 = assign59600_e93049_d_n7;
        locals.var_f30_dn8 = assign59600_e93049_d_n8;
        locals.var_f30_dn9 = assign59600_e93049_d_n9;
        locals.var_f30_dn10 = assign59600_e93049_d_n10;
        locals.var_f30_dn13 = assign59600_e93049_d_n13;
        locals.var_f30_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_213(
        locals: &mut StampLocals,
    ) {
        let (assign59610_e93066, assign59610_e93066_d_n0, assign59610_e93066_d_n2, assign59610_e93066_d_n4, assign59610_e93066_d_n5, assign59610_e93066_d_n6, assign59610_e93066_d_n7, assign59610_e93066_d_n8, assign59610_e93066_d_n9, assign59610_e93066_d_n10, assign59610_e93066_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59610_e93056: f64 = (locals.var_ps0 * locals.var_f10);
        let assign59610_e93059: f64 = (0.6666666666666667 * locals.var_beta_inv);
        let assign59610_e93061: f64 = (assign59610_e93059 * locals.var_xilp32);
        let assign59610_e93062: f64 = (assign59610_e93056 + assign59610_e93061);
        let assign59610_e93064: f64 = (assign59610_e93062 - locals.var_f30);
        (assign59610_e93064, ((((locals.var_ps0_dn0 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn0)) + (((0.6666666666666667 * locals.var_beta_inv_dn0) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn0))) - locals.var_f30_dn0), ((((locals.var_ps0_dn2 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn2)) + (((0.6666666666666667 * locals.var_beta_inv_dn2) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn2))) - locals.var_f30_dn2), ((((locals.var_ps0_dn4 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn4)) + (((0.6666666666666667 * locals.var_beta_inv_dn4) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn4))) - locals.var_f30_dn4), ((((locals.var_ps0_dn5 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn5)) + (((0.6666666666666667 * locals.var_beta_inv_dn5) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn5))) - locals.var_f30_dn5), ((((locals.var_ps0_dn6 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn6)) + (((0.6666666666666667 * locals.var_beta_inv_dn6) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn6))) - locals.var_f30_dn6), ((((locals.var_ps0_dn7 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn7)) + (((0.6666666666666667 * locals.var_beta_inv_dn7) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn7))) - locals.var_f30_dn7), ((((locals.var_ps0_dn8 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn8)) + (((0.6666666666666667 * locals.var_beta_inv_dn8) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn8))) - locals.var_f30_dn8), ((((locals.var_ps0_dn9 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn9)) + (((0.6666666666666667 * locals.var_beta_inv_dn9) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn9))) - locals.var_f30_dn9), ((((locals.var_ps0_dn10 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn10)) + (((0.6666666666666667 * locals.var_beta_inv_dn10) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn10))) - locals.var_f30_dn10), ((((locals.var_ps0_dn13 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn13)) + (((0.6666666666666667 * locals.var_beta_inv_dn13) * locals.var_xilp32) + (assign59610_e93059 * locals.var_xilp32_dn13))) - locals.var_f30_dn13),)
    } else {
        (locals.var_f11, locals.var_f11_dn0, locals.var_f11_dn2, locals.var_f11_dn4, locals.var_f11_dn5, locals.var_f11_dn6, locals.var_f11_dn7, locals.var_f11_dn8, locals.var_f11_dn9, locals.var_f11_dn10, locals.var_f11_dn13,)
    }
};
        locals.var_f11 = assign59610_e93066;
        locals.var_f11_dn0 = assign59610_e93066_d_n0;
        locals.var_f11_dn2 = assign59610_e93066_d_n2;
        locals.var_f11_dn4 = assign59610_e93066_d_n4;
        locals.var_f11_dn5 = assign59610_e93066_d_n5;
        locals.var_f11_dn6 = assign59610_e93066_d_n6;
        locals.var_f11_dn7 = assign59610_e93066_d_n7;
        locals.var_f11_dn8 = assign59610_e93066_d_n8;
        locals.var_f11_dn9 = assign59610_e93066_d_n9;
        locals.var_f11_dn10 = assign59610_e93066_d_n10;
        locals.var_f11_dn13 = assign59610_e93066_d_n13;
        locals.var_f11_rv = 0.0;

        let (assign59620_e93083, assign59620_e93083_d_n0, assign59620_e93083_d_n2, assign59620_e93083_d_n4, assign59620_e93083_d_n5, assign59620_e93083_d_n6, assign59620_e93083_d_n7, assign59620_e93083_d_n8, assign59620_e93083_d_n9, assign59620_e93083_d_n10, assign59620_e93083_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59620_e93073: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign59620_e93077: f64 = (2.0 * locals.var_ps0);
        let assign59620_e93079: f64 = (assign59620_e93077 + locals.var_pds);
        let assign59620_e93080: f64 = (0.5 * assign59620_e93079);
        let assign59620_e93081: f64 = (assign59620_e93073 - assign59620_e93080);
        (assign59620_e93081, ((locals.var_vgp_dn0 + locals.var_beta_inv_dn0) - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), ((locals.var_vgp_dn2 + locals.var_beta_inv_dn2) - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), ((locals.var_vgp_dn4 + locals.var_beta_inv_dn4) - (0.5 * ((2.0 * locals.var_ps0_dn4) + locals.var_pds_dn4))), ((locals.var_vgp_dn5 + locals.var_beta_inv_dn5) - (0.5 * ((2.0 * locals.var_ps0_dn5) + locals.var_pds_dn5))), ((locals.var_vgp_dn6 + locals.var_beta_inv_dn6) - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), ((locals.var_vgp_dn7 + locals.var_beta_inv_dn7) - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn8 + locals.var_beta_inv_dn8) - (0.5 * ((2.0 * locals.var_ps0_dn8) + locals.var_pds_dn8))), ((locals.var_vgp_dn9 + locals.var_beta_inv_dn9) - (0.5 * ((2.0 * locals.var_ps0_dn9) + locals.var_pds_dn9))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), ((locals.var_vgp_dn13 + locals.var_beta_inv_dn13) - (0.5 * ((2.0 * locals.var_ps0_dn13) + locals.var_pds_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign59620_e93083;
        locals.var_t1_dn0 = assign59620_e93083_d_n0;
        locals.var_t1_dn2 = assign59620_e93083_d_n2;
        locals.var_t1_dn4 = assign59620_e93083_d_n4;
        locals.var_t1_dn5 = assign59620_e93083_d_n5;
        locals.var_t1_dn6 = assign59620_e93083_d_n6;
        locals.var_t1_dn7 = assign59620_e93083_d_n7;
        locals.var_t1_dn8 = assign59620_e93083_d_n8;
        locals.var_t1_dn9 = assign59620_e93083_d_n9;
        locals.var_t1_dn10 = assign59620_e93083_d_n10;
        locals.var_t1_dn13 = assign59620_e93083_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign59630_e93093, assign59630_e93093_d_n0, assign59630_e93093_d_n2, assign59630_e93093_d_n4, assign59630_e93093_d_n5, assign59630_e93093_d_n6, assign59630_e93093_d_n7, assign59630_e93093_d_n8, assign59630_e93093_d_n9, assign59630_e93093_d_n10, assign59630_e93093_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59630_e93089: f64 = (-locals.var_f10);
        let assign59630_e93091: f64 = (assign59630_e93089 + locals.var_f00);
        (assign59630_e93091, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn4) + locals.var_f00_dn4), ((-locals.var_f10_dn5) + locals.var_f00_dn5), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn8) + locals.var_f00_dn8), ((-locals.var_f10_dn9) + locals.var_f00_dn9), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn13) + locals.var_f00_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign59630_e93093;
        locals.var_t2_dn0 = assign59630_e93093_d_n0;
        locals.var_t2_dn2 = assign59630_e93093_d_n2;
        locals.var_t2_dn4 = assign59630_e93093_d_n4;
        locals.var_t2_dn5 = assign59630_e93093_d_n5;
        locals.var_t2_dn6 = assign59630_e93093_d_n6;
        locals.var_t2_dn7 = assign59630_e93093_d_n7;
        locals.var_t2_dn8 = assign59630_e93093_d_n8;
        locals.var_t2_dn9 = assign59630_e93093_d_n9;
        locals.var_t2_dn10 = assign59630_e93093_d_n10;
        locals.var_t2_dn13 = assign59630_e93093_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign59640_e93102, assign59640_e93102_d_n0, assign59640_e93102_d_n2, assign59640_e93102_d_n4, assign59640_e93102_d_n5, assign59640_e93102_d_n6, assign59640_e93102_d_n7, assign59640_e93102_d_n8, assign59640_e93102_d_n9, assign59640_e93102_d_n10, assign59640_e93102_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59640_e93100: f64 = (locals.var_beta * locals.var_cox);
        (assign59640_e93100, ((locals.var_beta_dn0 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn0)), ((locals.var_beta_dn2 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn2)), ((locals.var_beta_dn4 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn4)), ((locals.var_beta_dn5 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn5)), ((locals.var_beta_dn6 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn6)), ((locals.var_beta_dn7 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn7)), ((locals.var_beta_dn8 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn8)), ((locals.var_beta_dn9 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn9)), ((locals.var_beta_dn10 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn10)), ((locals.var_beta_dn13 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign59640_e93102;
        locals.var_t3_dn0 = assign59640_e93102_d_n0;
        locals.var_t3_dn2 = assign59640_e93102_d_n2;
        locals.var_t3_dn4 = assign59640_e93102_d_n4;
        locals.var_t3_dn5 = assign59640_e93102_d_n5;
        locals.var_t3_dn6 = assign59640_e93102_d_n6;
        locals.var_t3_dn7 = assign59640_e93102_d_n7;
        locals.var_t3_dn8 = assign59640_e93102_d_n8;
        locals.var_t3_dn9 = assign59640_e93102_d_n9;
        locals.var_t3_dn10 = assign59640_e93102_d_n10;
        locals.var_t3_dn13 = assign59640_e93102_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign59650_e93111, assign59650_e93111_d_n0, assign59650_e93111_d_n2, assign59650_e93111_d_n4, assign59650_e93111_d_n5, assign59650_e93111_d_n6, assign59650_e93111_d_n7, assign59650_e93111_d_n8, assign59650_e93111_d_n9, assign59650_e93111_d_n10, assign59650_e93111_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59650_e93109: f64 = (locals.var_beta * locals.var_cnst0);
        (assign59650_e93109, ((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)), ((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)), ((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)), ((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)), ((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)), ((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)), ((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)), ((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)), ((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)), ((locals.var_beta_dn13 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign59650_e93111;
        locals.var_t4_dn0 = assign59650_e93111_d_n0;
        locals.var_t4_dn2 = assign59650_e93111_d_n2;
        locals.var_t4_dn4 = assign59650_e93111_d_n4;
        locals.var_t4_dn5 = assign59650_e93111_d_n5;
        locals.var_t4_dn6 = assign59650_e93111_d_n6;
        locals.var_t4_dn7 = assign59650_e93111_d_n7;
        locals.var_t4_dn8 = assign59650_e93111_d_n8;
        locals.var_t4_dn9 = assign59650_e93111_d_n9;
        locals.var_t4_dn10 = assign59650_e93111_d_n10;
        locals.var_t4_dn13 = assign59650_e93111_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign59660_e93124, assign59660_e93124_d_n0, assign59660_e93124_d_n2, assign59660_e93124_d_n4, assign59660_e93124_d_n5, assign59660_e93124_d_n6, assign59660_e93124_d_n7, assign59660_e93124_d_n8, assign59660_e93124_d_n9, assign59660_e93124_d_n10, assign59660_e93124_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59660_e93118: f64 = (locals.var_t3 * locals.var_t1);
        let assign59660_e93121: f64 = (locals.var_t4 * locals.var_t2);
        let assign59660_e93122: f64 = (assign59660_e93118 + assign59660_e93121);
        (assign59660_e93122, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn4 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4))), (((locals.var_t3_dn5 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn8 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8))), (((locals.var_t3_dn9 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn13 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn13))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn4, locals.var_fdd_dn5, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn8, locals.var_fdd_dn9, locals.var_fdd_dn10, locals.var_fdd_dn13,)
    }
};
        locals.var_fdd = assign59660_e93124;
        locals.var_fdd_dn0 = assign59660_e93124_d_n0;
        locals.var_fdd_dn2 = assign59660_e93124_d_n2;
        locals.var_fdd_dn4 = assign59660_e93124_d_n4;
        locals.var_fdd_dn5 = assign59660_e93124_d_n5;
        locals.var_fdd_dn6 = assign59660_e93124_d_n6;
        locals.var_fdd_dn7 = assign59660_e93124_d_n7;
        locals.var_fdd_dn8 = assign59660_e93124_d_n8;
        locals.var_fdd_dn9 = assign59660_e93124_d_n9;
        locals.var_fdd_dn10 = assign59660_e93124_d_n10;
        locals.var_fdd_dn13 = assign59660_e93124_d_n13;
        locals.var_fdd_rv = 0.0;

        let (assign59670_e93133, assign59670_e93133_d_n0, assign59670_e93133_d_n2, assign59670_e93133_d_n4, assign59670_e93133_d_n5, assign59670_e93133_d_n6, assign59670_e93133_d_n7, assign59670_e93133_d_n8, assign59670_e93133_d_n9, assign59670_e93133_d_n10, assign59670_e93133_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign59670_e93131: f64 = (locals.var_pds * locals.var_fdd);
        (assign59670_e93131, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn4 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn4)), ((locals.var_pds_dn5 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn5)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn8 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn8)), ((locals.var_pds_dn9 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn9)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn13 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn13)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign59670_e93133;
        locals.var_idd_dn0 = assign59670_e93133_d_n0;
        locals.var_idd_dn2 = assign59670_e93133_d_n2;
        locals.var_idd_dn4 = assign59670_e93133_d_n4;
        locals.var_idd_dn5 = assign59670_e93133_d_n5;
        locals.var_idd_dn6 = assign59670_e93133_d_n6;
        locals.var_idd_dn7 = assign59670_e93133_d_n7;
        locals.var_idd_dn8 = assign59670_e93133_d_n8;
        locals.var_idd_dn9 = assign59670_e93133_d_n9;
        locals.var_idd_dn10 = assign59670_e93133_d_n10;
        locals.var_idd_dn13 = assign59670_e93133_d_n13;
        locals.var_idd_rv = 0.0;

        let assign59680_e93136: f64 = if locals.var_flg_zone == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1460 = assign59680_e93136;
        locals.var_guard1460_rv = 0.0;

        let (assign59690_e93145,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign59690_e93145;
        locals.var_start_of_mobility_rv = 0.0;

        let assign59700_e93148: f64 = if locals.var_start_of_mobility == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign59700_e93148;
        locals.var_guard1461_rv = 0.0;

        let assign59710_e93152: f64 = (10.0 * 2.220446049250313e-16);
        let assign59710_e93157: f64 = (10.0 * 2.220446049250313e-16);
        let assign59710_e93159: f64 = if ((locals.var_uc_clm2 < assign59710_e93152) && (locals.var_uc_clm3 < assign59710_e93157)) { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign59710_e93159;
        locals.var_guard1462_rv = 0.0;

        let (assign59720_e93170, assign59720_e93170_d_n0, assign59720_e93170_d_n2, assign59720_e93170_d_n4, assign59720_e93170_d_n5, assign59720_e93170_d_n6, assign59720_e93170_d_n7, assign59720_e93170_d_n8, assign59720_e93170_d_n9, assign59720_e93170_d_n10, assign59720_e93170_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign59720_e93170;
        locals.var_lred_dn0 = assign59720_e93170_d_n0;
        locals.var_lred_dn2 = assign59720_e93170_d_n2;
        locals.var_lred_dn4 = assign59720_e93170_d_n4;
        locals.var_lred_dn5 = assign59720_e93170_d_n5;
        locals.var_lred_dn6 = assign59720_e93170_d_n6;
        locals.var_lred_dn7 = assign59720_e93170_d_n7;
        locals.var_lred_dn8 = assign59720_e93170_d_n8;
        locals.var_lred_dn9 = assign59720_e93170_d_n9;
        locals.var_lred_dn10 = assign59720_e93170_d_n10;
        locals.var_lred_dn13 = assign59720_e93170_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign59730_e93181, assign59730_e93181_d_n0, assign59730_e93181_d_n2, assign59730_e93181_d_n4, assign59730_e93181_d_n5, assign59730_e93181_d_n6, assign59730_e93181_d_n7, assign59730_e93181_d_n8, assign59730_e93181_d_n9, assign59730_e93181_d_n10, assign59730_e93181_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign59730_e93181;
        locals.var_psdl_dn0 = assign59730_e93181_d_n0;
        locals.var_psdl_dn2 = assign59730_e93181_d_n2;
        locals.var_psdl_dn4 = assign59730_e93181_d_n4;
        locals.var_psdl_dn5 = assign59730_e93181_d_n5;
        locals.var_psdl_dn6 = assign59730_e93181_d_n6;
        locals.var_psdl_dn7 = assign59730_e93181_d_n7;
        locals.var_psdl_dn8 = assign59730_e93181_d_n8;
        locals.var_psdl_dn9 = assign59730_e93181_d_n9;
        locals.var_psdl_dn10 = assign59730_e93181_d_n10;
        locals.var_psdl_dn13 = assign59730_e93181_d_n13;
        locals.var_psdl_rv = 0.0;

        let assign59740_e93185: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59740_e93188: f64 = (10.0 * 2.220446049250313e-16);
        let assign59740_e93189: f64 = (assign59740_e93185 - assign59740_e93188);
        let assign59740_e93192: f64 = (10.0 * 2.220446049250313e-16);
        let assign59740_e93193: f64 = (assign59740_e93189 - assign59740_e93192);
        let assign59740_e93197: f64 = (10.0 * 2.220446049250313e-16);
        let assign59740_e93200: f64 = if ((locals.var_psdl > assign59740_e93193) && (assign59740_e93197 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign59740_e93200;
        locals.var_guard1463_rv = 0.0;

        let (assign59750_e93225, assign59750_e93225_d_n0, assign59750_e93225_d_n2, assign59750_e93225_d_n4, assign59750_e93225_d_n5, assign59750_e93225_d_n6, assign59750_e93225_d_n7, assign59750_e93225_d_n8, assign59750_e93225_d_n9, assign59750_e93225_d_n10, assign59750_e93225_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59750_e93214: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59750_e93217: f64 = (10.0 * 2.220446049250313e-16);
        let assign59750_e93218: f64 = (assign59750_e93214 - assign59750_e93217);
        let assign59750_e93219: f64 = (locals.var_psdl - assign59750_e93218);
        let assign59750_e93222: f64 = (10.0 * 2.220446049250313e-16);
        let assign59750_e93223: f64 = (assign59750_e93219 + assign59750_e93222);
        (assign59750_e93223, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign59750_e93225;
        locals.var_tmf1_dn0 = assign59750_e93225_d_n0;
        locals.var_tmf1_dn2 = assign59750_e93225_d_n2;
        locals.var_tmf1_dn4 = assign59750_e93225_d_n4;
        locals.var_tmf1_dn5 = assign59750_e93225_d_n5;
        locals.var_tmf1_dn6 = assign59750_e93225_d_n6;
        locals.var_tmf1_dn7 = assign59750_e93225_d_n7;
        locals.var_tmf1_dn8 = assign59750_e93225_d_n8;
        locals.var_tmf1_dn9 = assign59750_e93225_d_n9;
        locals.var_tmf1_dn10 = assign59750_e93225_d_n10;
        locals.var_tmf1_dn13 = assign59750_e93225_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign59760_e93240, assign59760_e93240_d_n0, assign59760_e93240_d_n2, assign59760_e93240_d_n4, assign59760_e93240_d_n5, assign59760_e93240_d_n6, assign59760_e93240_d_n7, assign59760_e93240_d_n8, assign59760_e93240_d_n9, assign59760_e93240_d_n10, assign59760_e93240_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59760_e93238: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign59760_e93238, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign59760_e93240;
        locals.var_x2_dn0 = assign59760_e93240_d_n0;
        locals.var_x2_dn2 = assign59760_e93240_d_n2;
        locals.var_x2_dn4 = assign59760_e93240_d_n4;
        locals.var_x2_dn5 = assign59760_e93240_d_n5;
        locals.var_x2_dn6 = assign59760_e93240_d_n6;
        locals.var_x2_dn7 = assign59760_e93240_d_n7;
        locals.var_x2_dn8 = assign59760_e93240_d_n8;
        locals.var_x2_dn9 = assign59760_e93240_d_n9;
        locals.var_x2_dn10 = assign59760_e93240_d_n10;
        locals.var_x2_dn13 = assign59760_e93240_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign59770_e93259, assign59770_e93259_d_n0, assign59770_e93259_d_n2, assign59770_e93259_d_n4, assign59770_e93259_d_n5, assign59770_e93259_d_n6, assign59770_e93259_d_n7, assign59770_e93259_d_n8, assign59770_e93259_d_n9, assign59770_e93259_d_n10, assign59770_e93259_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59770_e93253: f64 = (10.0 * 2.220446049250313e-16);
        let assign59770_e93256: f64 = (10.0 * 2.220446049250313e-16);
        let assign59770_e93257: f64 = (assign59770_e93253 * assign59770_e93256);
        (assign59770_e93257, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign59770_e93259;
        locals.var_xmax2_dn0 = assign59770_e93259_d_n0;
        locals.var_xmax2_dn2 = assign59770_e93259_d_n2;
        locals.var_xmax2_dn4 = assign59770_e93259_d_n4;
        locals.var_xmax2_dn5 = assign59770_e93259_d_n5;
        locals.var_xmax2_dn6 = assign59770_e93259_d_n6;
        locals.var_xmax2_dn7 = assign59770_e93259_d_n7;
        locals.var_xmax2_dn8 = assign59770_e93259_d_n8;
        locals.var_xmax2_dn9 = assign59770_e93259_d_n9;
        locals.var_xmax2_dn10 = assign59770_e93259_d_n10;
        locals.var_xmax2_dn13 = assign59770_e93259_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign59780_e93272, assign59780_e93272_d_n0, assign59780_e93272_d_n2, assign59780_e93272_d_n4, assign59780_e93272_d_n5, assign59780_e93272_d_n6, assign59780_e93272_d_n7, assign59780_e93272_d_n8, assign59780_e93272_d_n9, assign59780_e93272_d_n10, assign59780_e93272_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign59780_e93272;
        locals.var_xp_dn0 = assign59780_e93272_d_n0;
        locals.var_xp_dn2 = assign59780_e93272_d_n2;
        locals.var_xp_dn4 = assign59780_e93272_d_n4;
        locals.var_xp_dn5 = assign59780_e93272_d_n5;
        locals.var_xp_dn6 = assign59780_e93272_d_n6;
        locals.var_xp_dn7 = assign59780_e93272_d_n7;
        locals.var_xp_dn8 = assign59780_e93272_d_n8;
        locals.var_xp_dn9 = assign59780_e93272_d_n9;
        locals.var_xp_dn10 = assign59780_e93272_d_n10;
        locals.var_xp_dn13 = assign59780_e93272_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign59790_e93285, assign59790_e93285_d_n0, assign59790_e93285_d_n2, assign59790_e93285_d_n4, assign59790_e93285_d_n5, assign59790_e93285_d_n6, assign59790_e93285_d_n7, assign59790_e93285_d_n8, assign59790_e93285_d_n9, assign59790_e93285_d_n10, assign59790_e93285_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign59790_e93285;
        locals.var_xmp_dn0 = assign59790_e93285_d_n0;
        locals.var_xmp_dn2 = assign59790_e93285_d_n2;
        locals.var_xmp_dn4 = assign59790_e93285_d_n4;
        locals.var_xmp_dn5 = assign59790_e93285_d_n5;
        locals.var_xmp_dn6 = assign59790_e93285_d_n6;
        locals.var_xmp_dn7 = assign59790_e93285_d_n7;
        locals.var_xmp_dn8 = assign59790_e93285_d_n8;
        locals.var_xmp_dn9 = assign59790_e93285_d_n9;
        locals.var_xmp_dn10 = assign59790_e93285_d_n10;
        locals.var_xmp_dn13 = assign59790_e93285_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign59800_e93298,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign59800_e93298;
        locals.var_m0_rv = 0.0;

        let (assign59810_e93311,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59810_e93311;
        locals.var_mm_rv = 0.0;

        let (assign59820_e93324, assign59820_e93324_d_n0, assign59820_e93324_d_n2, assign59820_e93324_d_n4, assign59820_e93324_d_n5, assign59820_e93324_d_n6, assign59820_e93324_d_n7, assign59820_e93324_d_n8, assign59820_e93324_d_n9, assign59820_e93324_d_n10, assign59820_e93324_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign59820_e93324;
        locals.var_arg_dn0 = assign59820_e93324_d_n0;
        locals.var_arg_dn2 = assign59820_e93324_d_n2;
        locals.var_arg_dn4 = assign59820_e93324_d_n4;
        locals.var_arg_dn5 = assign59820_e93324_d_n5;
        locals.var_arg_dn6 = assign59820_e93324_d_n6;
        locals.var_arg_dn7 = assign59820_e93324_d_n7;
        locals.var_arg_dn8 = assign59820_e93324_d_n8;
        locals.var_arg_dn9 = assign59820_e93324_d_n9;
        locals.var_arg_dn10 = assign59820_e93324_d_n10;
        locals.var_arg_dn13 = assign59820_e93324_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign59830_e93337, assign59830_e93337_d_n0, assign59830_e93337_d_n2, assign59830_e93337_d_n4, assign59830_e93337_d_n5, assign59830_e93337_d_n6, assign59830_e93337_d_n7, assign59830_e93337_d_n8, assign59830_e93337_d_n9, assign59830_e93337_d_n10, assign59830_e93337_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign59830_e93337;
        locals.var_dnm_dn0 = assign59830_e93337_d_n0;
        locals.var_dnm_dn2 = assign59830_e93337_d_n2;
        locals.var_dnm_dn4 = assign59830_e93337_d_n4;
        locals.var_dnm_dn5 = assign59830_e93337_d_n5;
        locals.var_dnm_dn6 = assign59830_e93337_d_n6;
        locals.var_dnm_dn7 = assign59830_e93337_d_n7;
        locals.var_dnm_dn8 = assign59830_e93337_d_n8;
        locals.var_dnm_dn9 = assign59830_e93337_d_n9;
        locals.var_dnm_dn10 = assign59830_e93337_d_n10;
        locals.var_dnm_dn13 = assign59830_e93337_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign59840_e93352, assign59840_e93352_d_n0, assign59840_e93352_d_n2, assign59840_e93352_d_n4, assign59840_e93352_d_n5, assign59840_e93352_d_n6, assign59840_e93352_d_n7, assign59840_e93352_d_n8, assign59840_e93352_d_n9, assign59840_e93352_d_n10, assign59840_e93352_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59840_e93350: f64 = (locals.var_xp * locals.var_x2);
        (assign59840_e93350, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign59840_e93352;
        locals.var_xp_dn0 = assign59840_e93352_d_n0;
        locals.var_xp_dn2 = assign59840_e93352_d_n2;
        locals.var_xp_dn4 = assign59840_e93352_d_n4;
        locals.var_xp_dn5 = assign59840_e93352_d_n5;
        locals.var_xp_dn6 = assign59840_e93352_d_n6;
        locals.var_xp_dn7 = assign59840_e93352_d_n7;
        locals.var_xp_dn8 = assign59840_e93352_d_n8;
        locals.var_xp_dn9 = assign59840_e93352_d_n9;
        locals.var_xp_dn10 = assign59840_e93352_d_n10;
        locals.var_xp_dn13 = assign59840_e93352_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign59850_e93367, assign59850_e93367_d_n0, assign59850_e93367_d_n2, assign59850_e93367_d_n4, assign59850_e93367_d_n5, assign59850_e93367_d_n6, assign59850_e93367_d_n7, assign59850_e93367_d_n8, assign59850_e93367_d_n9, assign59850_e93367_d_n10, assign59850_e93367_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59850_e93365: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59850_e93365, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign59850_e93367;
        locals.var_xmp_dn0 = assign59850_e93367_d_n0;
        locals.var_xmp_dn2 = assign59850_e93367_d_n2;
        locals.var_xmp_dn4 = assign59850_e93367_d_n4;
        locals.var_xmp_dn5 = assign59850_e93367_d_n5;
        locals.var_xmp_dn6 = assign59850_e93367_d_n6;
        locals.var_xmp_dn7 = assign59850_e93367_d_n7;
        locals.var_xmp_dn8 = assign59850_e93367_d_n8;
        locals.var_xmp_dn9 = assign59850_e93367_d_n9;
        locals.var_xmp_dn10 = assign59850_e93367_d_n10;
        locals.var_xmp_dn13 = assign59850_e93367_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign59860_e93382, assign59860_e93382_d_n0, assign59860_e93382_d_n2, assign59860_e93382_d_n4, assign59860_e93382_d_n5, assign59860_e93382_d_n6, assign59860_e93382_d_n7, assign59860_e93382_d_n8, assign59860_e93382_d_n9, assign59860_e93382_d_n10, assign59860_e93382_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59860_e93380: f64 = (locals.var_xp * locals.var_x2);
        (assign59860_e93380, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign59860_e93382;
        locals.var_xp_dn0 = assign59860_e93382_d_n0;
        locals.var_xp_dn2 = assign59860_e93382_d_n2;
        locals.var_xp_dn4 = assign59860_e93382_d_n4;
        locals.var_xp_dn5 = assign59860_e93382_d_n5;
        locals.var_xp_dn6 = assign59860_e93382_d_n6;
        locals.var_xp_dn7 = assign59860_e93382_d_n7;
        locals.var_xp_dn8 = assign59860_e93382_d_n8;
        locals.var_xp_dn9 = assign59860_e93382_d_n9;
        locals.var_xp_dn10 = assign59860_e93382_d_n10;
        locals.var_xp_dn13 = assign59860_e93382_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign59870_e93397, assign59870_e93397_d_n0, assign59870_e93397_d_n2, assign59870_e93397_d_n4, assign59870_e93397_d_n5, assign59870_e93397_d_n6, assign59870_e93397_d_n7, assign59870_e93397_d_n8, assign59870_e93397_d_n9, assign59870_e93397_d_n10, assign59870_e93397_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59870_e93395: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59870_e93395, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign59870_e93397;
        locals.var_xmp_dn0 = assign59870_e93397_d_n0;
        locals.var_xmp_dn2 = assign59870_e93397_d_n2;
        locals.var_xmp_dn4 = assign59870_e93397_d_n4;
        locals.var_xmp_dn5 = assign59870_e93397_d_n5;
        locals.var_xmp_dn6 = assign59870_e93397_d_n6;
        locals.var_xmp_dn7 = assign59870_e93397_d_n7;
        locals.var_xmp_dn8 = assign59870_e93397_d_n8;
        locals.var_xmp_dn9 = assign59870_e93397_d_n9;
        locals.var_xmp_dn10 = assign59870_e93397_d_n10;
        locals.var_xmp_dn13 = assign59870_e93397_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign59880_e93412, assign59880_e93412_d_n0, assign59880_e93412_d_n2, assign59880_e93412_d_n4, assign59880_e93412_d_n5, assign59880_e93412_d_n6, assign59880_e93412_d_n7, assign59880_e93412_d_n8, assign59880_e93412_d_n9, assign59880_e93412_d_n10, assign59880_e93412_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59880_e93410: f64 = (locals.var_xp + locals.var_xmp);
        (assign59880_e93410, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign59880_e93412;
        locals.var_arg_dn0 = assign59880_e93412_d_n0;
        locals.var_arg_dn2 = assign59880_e93412_d_n2;
        locals.var_arg_dn4 = assign59880_e93412_d_n4;
        locals.var_arg_dn5 = assign59880_e93412_d_n5;
        locals.var_arg_dn6 = assign59880_e93412_d_n6;
        locals.var_arg_dn7 = assign59880_e93412_d_n7;
        locals.var_arg_dn8 = assign59880_e93412_d_n8;
        locals.var_arg_dn9 = assign59880_e93412_d_n9;
        locals.var_arg_dn10 = assign59880_e93412_d_n10;
        locals.var_arg_dn13 = assign59880_e93412_d_n13;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_214(
        locals: &mut StampLocals,
    ) {
        let (assign59890_e93425, assign59890_e93425_d_n0, assign59890_e93425_d_n2, assign59890_e93425_d_n4, assign59890_e93425_d_n5, assign59890_e93425_d_n6, assign59890_e93425_d_n7, assign59890_e93425_d_n8, assign59890_e93425_d_n9, assign59890_e93425_d_n10, assign59890_e93425_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign59890_e93425;
        locals.var_dnm_dn0 = assign59890_e93425_d_n0;
        locals.var_dnm_dn2 = assign59890_e93425_d_n2;
        locals.var_dnm_dn4 = assign59890_e93425_d_n4;
        locals.var_dnm_dn5 = assign59890_e93425_d_n5;
        locals.var_dnm_dn6 = assign59890_e93425_d_n6;
        locals.var_dnm_dn7 = assign59890_e93425_d_n7;
        locals.var_dnm_dn8 = assign59890_e93425_d_n8;
        locals.var_dnm_dn9 = assign59890_e93425_d_n9;
        locals.var_dnm_dn10 = assign59890_e93425_d_n10;
        locals.var_dnm_dn13 = assign59890_e93425_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign59900_e93440: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign59900_e93440;
        locals.var_guard1464_rv = 0.0;

        let assign59910_e93443: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign59910_e93443;
        locals.var_guard1465_rv = 0.0;

        let (assign59920_e93460,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59920_e93460;
        locals.var_mm_rv = 0.0;

        let assign59930_e93463: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign59930_e93463;
        locals.var_guard1466_rv = 0.0;

        let (assign59940_e93483,) = {
    if ((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) && (locals.var_guard1466 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59940_e93483;
        locals.var_mm_rv = 0.0;

        let assign59950_e93486: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign59950_e93486;
        locals.var_guard1467_rv = 0.0;

        let (assign59960_e93509,) = {
    if (((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1467 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59960_e93509;
        locals.var_mm_rv = 0.0;

        let assign59970_e93512: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign59970_e93512;
        locals.var_guard1468_rv = 0.0;

        let (assign59980_e93538,) = {
    if ((((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1467 == 0.0)) && (locals.var_guard1468 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59980_e93538;
        locals.var_mm_rv = 0.0;

        let (assign59990_e93553,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign59990_e93553;
        locals.var_m0_rv = 0.0;

        let mut assign60000_loop_guard: usize = 0;
        while {
            let assign60000_cond_e93569: f64 = if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60000_cond_e93569 != 0.0
        } {
            assign60000_loop_guard += 1;
            assert!(assign60000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60000_body0_e93585, assign60000_body0_e93585_d_n0, assign60000_body0_e93585_d_n2, assign60000_body0_e93585_d_n4, assign60000_body0_e93585_d_n5, assign60000_body0_e93585_d_n6, assign60000_body0_e93585_d_n7, assign60000_body0_e93585_d_n8, assign60000_body0_e93585_d_n9, assign60000_body0_e93585_d_n10, assign60000_body0_e93585_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign60000_body0_e93583: f64 = (locals.var_dnm).sqrt();
        (assign60000_body0_e93583, (locals.var_dnm_dn0 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn2 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn4 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn5 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn6 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn7 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn8 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn9 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn10 / (2.0 * assign60000_body0_e93583)), (locals.var_dnm_dn13 / (2.0 * assign60000_body0_e93583)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign60000_body0_e93585;
            locals.var_dnm_dn0 = assign60000_body0_e93585_d_n0;
            locals.var_dnm_dn2 = assign60000_body0_e93585_d_n2;
            locals.var_dnm_dn4 = assign60000_body0_e93585_d_n4;
            locals.var_dnm_dn5 = assign60000_body0_e93585_d_n5;
            locals.var_dnm_dn6 = assign60000_body0_e93585_d_n6;
            locals.var_dnm_dn7 = assign60000_body0_e93585_d_n7;
            locals.var_dnm_dn8 = assign60000_body0_e93585_d_n8;
            locals.var_dnm_dn9 = assign60000_body0_e93585_d_n9;
            locals.var_dnm_dn10 = assign60000_body0_e93585_d_n10;
            locals.var_dnm_dn13 = assign60000_body0_e93585_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign60000_body1_e93602,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign60000_body1_e93600: f64 = (locals.var_m0 + 1.0);
        (assign60000_body1_e93600,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60000_body1_e93602;
            locals.var_m0_rv = 0.0;
        }

        let (assign60010_e93629, assign60010_e93629_d_n0, assign60010_e93629_d_n2, assign60010_e93629_d_n4, assign60010_e93629_d_n5, assign60010_e93629_d_n6, assign60010_e93629_d_n7, assign60010_e93629_d_n8, assign60010_e93629_d_n9, assign60010_e93629_d_n10, assign60010_e93629_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let (assign60010_e93627, assign60010_e93627_d_n0, assign60010_e93627_d_n2, assign60010_e93627_d_n4, assign60010_e93627_d_n5, assign60010_e93627_d_n6, assign60010_e93627_d_n7, assign60010_e93627_d_n8, assign60010_e93627_d_n9, assign60010_e93627_d_n10, assign60010_e93627_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60010_e93624: f64 = (2.0 * 2.0);
                let assign60010_e93625: f64 = (1.0 / assign60010_e93624);
                let assign60010_e93626: f64 = (locals.var_dnm).powf(assign60010_e93625);
                (assign60010_e93626, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((locals.var_dnm).powf(assign60010_e93625 - 1.0) * locals.var_dnm_dn13)) } } else { (assign60010_e93626 * (assign60010_e93625 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign60010_e93627, assign60010_e93627_d_n0, assign60010_e93627_d_n2, assign60010_e93627_d_n4, assign60010_e93627_d_n5, assign60010_e93627_d_n6, assign60010_e93627_d_n7, assign60010_e93627_d_n8, assign60010_e93627_d_n9, assign60010_e93627_d_n10, assign60010_e93627_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60010_e93629;
        locals.var_dnm_dn0 = assign60010_e93629_d_n0;
        locals.var_dnm_dn2 = assign60010_e93629_d_n2;
        locals.var_dnm_dn4 = assign60010_e93629_d_n4;
        locals.var_dnm_dn5 = assign60010_e93629_d_n5;
        locals.var_dnm_dn6 = assign60010_e93629_d_n6;
        locals.var_dnm_dn7 = assign60010_e93629_d_n7;
        locals.var_dnm_dn8 = assign60010_e93629_d_n8;
        locals.var_dnm_dn9 = assign60010_e93629_d_n9;
        locals.var_dnm_dn10 = assign60010_e93629_d_n10;
        locals.var_dnm_dn13 = assign60010_e93629_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign60020_e93644, assign60020_e93644_d_n0, assign60020_e93644_d_n2, assign60020_e93644_d_n4, assign60020_e93644_d_n5, assign60020_e93644_d_n6, assign60020_e93644_d_n7, assign60020_e93644_d_n8, assign60020_e93644_d_n9, assign60020_e93644_d_n10, assign60020_e93644_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60020_e93642: f64 = (1.0 / locals.var_dnm);
        (assign60020_e93642, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60020_e93644;
        locals.var_dnm_dn0 = assign60020_e93644_d_n0;
        locals.var_dnm_dn2 = assign60020_e93644_d_n2;
        locals.var_dnm_dn4 = assign60020_e93644_d_n4;
        locals.var_dnm_dn5 = assign60020_e93644_d_n5;
        locals.var_dnm_dn6 = assign60020_e93644_d_n6;
        locals.var_dnm_dn7 = assign60020_e93644_d_n7;
        locals.var_dnm_dn8 = assign60020_e93644_d_n8;
        locals.var_dnm_dn9 = assign60020_e93644_d_n9;
        locals.var_dnm_dn10 = assign60020_e93644_d_n10;
        locals.var_dnm_dn13 = assign60020_e93644_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign60030_e93663, assign60030_e93663_d_n0, assign60030_e93663_d_n2, assign60030_e93663_d_n4, assign60030_e93663_d_n5, assign60030_e93663_d_n6, assign60030_e93663_d_n7, assign60030_e93663_d_n8, assign60030_e93663_d_n9, assign60030_e93663_d_n10, assign60030_e93663_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60030_e93658: f64 = (10.0 * 2.220446049250313e-16);
        let assign60030_e93659: f64 = (locals.var_tmf1 * assign60030_e93658);
        let assign60030_e93661: f64 = (assign60030_e93659 * locals.var_dnm);
        (assign60030_e93661, (((locals.var_tmf1_dn0 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign60030_e93658) * locals.var_dnm) + (assign60030_e93659 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign60030_e93663;
        locals.var_tmf0_dn0 = assign60030_e93663_d_n0;
        locals.var_tmf0_dn2 = assign60030_e93663_d_n2;
        locals.var_tmf0_dn4 = assign60030_e93663_d_n4;
        locals.var_tmf0_dn5 = assign60030_e93663_d_n5;
        locals.var_tmf0_dn6 = assign60030_e93663_d_n6;
        locals.var_tmf0_dn7 = assign60030_e93663_d_n7;
        locals.var_tmf0_dn8 = assign60030_e93663_d_n8;
        locals.var_tmf0_dn9 = assign60030_e93663_d_n9;
        locals.var_tmf0_dn10 = assign60030_e93663_d_n10;
        locals.var_tmf0_dn13 = assign60030_e93663_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign60040_e93684, assign60040_e93684_d_n0, assign60040_e93684_d_n2, assign60040_e93684_d_n4, assign60040_e93684_d_n5, assign60040_e93684_d_n6, assign60040_e93684_d_n7, assign60040_e93684_d_n8, assign60040_e93684_d_n9, assign60040_e93684_d_n10, assign60040_e93684_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60040_e93676: f64 = (10.0 * 2.220446049250313e-16);
        let assign60040_e93678: f64 = (assign60040_e93676 * locals.var_xmp);
        let assign60040_e93680: f64 = (assign60040_e93678 * locals.var_dnm);
        let assign60040_e93682: f64 = (assign60040_e93680 / locals.var_arg);
        (assign60040_e93682, ((((((assign60040_e93676 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60040_e93676 * locals.var_xmp_dn13) * locals.var_dnm) + (assign60040_e93678 * locals.var_dnm_dn13)) * locals.var_arg) - (assign60040_e93680 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60040_e93684;
        locals.var_t0_dn0 = assign60040_e93684_d_n0;
        locals.var_t0_dn2 = assign60040_e93684_d_n2;
        locals.var_t0_dn4 = assign60040_e93684_d_n4;
        locals.var_t0_dn5 = assign60040_e93684_d_n5;
        locals.var_t0_dn6 = assign60040_e93684_d_n6;
        locals.var_t0_dn7 = assign60040_e93684_d_n7;
        locals.var_t0_dn8 = assign60040_e93684_d_n8;
        locals.var_t0_dn9 = assign60040_e93684_d_n9;
        locals.var_t0_dn10 = assign60040_e93684_d_n10;
        locals.var_t0_dn13 = assign60040_e93684_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60050_e93709, assign60050_e93709_d_n0, assign60050_e93709_d_n2, assign60050_e93709_d_n4, assign60050_e93709_d_n5, assign60050_e93709_d_n6, assign60050_e93709_d_n7, assign60050_e93709_d_n8, assign60050_e93709_d_n9, assign60050_e93709_d_n10, assign60050_e93709_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60050_e93697: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60050_e93700: f64 = (10.0 * 2.220446049250313e-16);
        let assign60050_e93701: f64 = (assign60050_e93697 - assign60050_e93700);
        let assign60050_e93704: f64 = (10.0 * 2.220446049250313e-16);
        let assign60050_e93705: f64 = (assign60050_e93701 - assign60050_e93704);
        let assign60050_e93707: f64 = (assign60050_e93705 + locals.var_tmf0);
        (assign60050_e93707, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign60050_e93709;
        locals.var_psdl_dn0 = assign60050_e93709_d_n0;
        locals.var_psdl_dn2 = assign60050_e93709_d_n2;
        locals.var_psdl_dn4 = assign60050_e93709_d_n4;
        locals.var_psdl_dn5 = assign60050_e93709_d_n5;
        locals.var_psdl_dn6 = assign60050_e93709_d_n6;
        locals.var_psdl_dn7 = assign60050_e93709_d_n7;
        locals.var_psdl_dn8 = assign60050_e93709_d_n8;
        locals.var_psdl_dn9 = assign60050_e93709_d_n9;
        locals.var_psdl_dn10 = assign60050_e93709_d_n10;
        locals.var_psdl_dn13 = assign60050_e93709_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign60060_e93722, assign60060_e93722_d_n0, assign60060_e93722_d_n2, assign60060_e93722_d_n4, assign60060_e93722_d_n5, assign60060_e93722_d_n6, assign60060_e93722_d_n7, assign60060_e93722_d_n8, assign60060_e93722_d_n9, assign60060_e93722_d_n10, assign60060_e93722_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60060_e93722;
        locals.var_t0_dn0 = assign60060_e93722_d_n0;
        locals.var_t0_dn2 = assign60060_e93722_d_n2;
        locals.var_t0_dn4 = assign60060_e93722_d_n4;
        locals.var_t0_dn5 = assign60060_e93722_d_n5;
        locals.var_t0_dn6 = assign60060_e93722_d_n6;
        locals.var_t0_dn7 = assign60060_e93722_d_n7;
        locals.var_t0_dn8 = assign60060_e93722_d_n8;
        locals.var_t0_dn9 = assign60060_e93722_d_n9;
        locals.var_t0_dn10 = assign60060_e93722_d_n10;
        locals.var_t0_dn13 = assign60060_e93722_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60070_e93736, assign60070_e93736_d_n0, assign60070_e93736_d_n2, assign60070_e93736_d_n4, assign60070_e93736_d_n5, assign60070_e93736_d_n6, assign60070_e93736_d_n7, assign60070_e93736_d_n8, assign60070_e93736_d_n9, assign60070_e93736_d_n10, assign60070_e93736_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign60070_e93736;
        locals.var_psdl_dn0 = assign60070_e93736_d_n0;
        locals.var_psdl_dn2 = assign60070_e93736_d_n2;
        locals.var_psdl_dn4 = assign60070_e93736_d_n4;
        locals.var_psdl_dn5 = assign60070_e93736_d_n5;
        locals.var_psdl_dn6 = assign60070_e93736_d_n6;
        locals.var_psdl_dn7 = assign60070_e93736_d_n7;
        locals.var_psdl_dn8 = assign60070_e93736_d_n8;
        locals.var_psdl_dn9 = assign60070_e93736_d_n9;
        locals.var_psdl_dn10 = assign60070_e93736_d_n10;
        locals.var_psdl_dn13 = assign60070_e93736_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign60080_e93750, assign60080_e93750_d_n0, assign60080_e93750_d_n2, assign60080_e93750_d_n4, assign60080_e93750_d_n5, assign60080_e93750_d_n6, assign60080_e93750_d_n7, assign60080_e93750_d_n8, assign60080_e93750_d_n9, assign60080_e93750_d_n10, assign60080_e93750_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) && (locals.var_guard1463 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60080_e93750;
        locals.var_t0_dn0 = assign60080_e93750_d_n0;
        locals.var_t0_dn2 = assign60080_e93750_d_n2;
        locals.var_t0_dn4 = assign60080_e93750_d_n4;
        locals.var_t0_dn5 = assign60080_e93750_d_n5;
        locals.var_t0_dn6 = assign60080_e93750_d_n6;
        locals.var_t0_dn7 = assign60080_e93750_d_n7;
        locals.var_t0_dn8 = assign60080_e93750_d_n8;
        locals.var_t0_dn9 = assign60080_e93750_d_n9;
        locals.var_t0_dn10 = assign60080_e93750_d_n10;
        locals.var_t0_dn13 = assign60080_e93750_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60090_e93762, assign60090_e93762_d_n0, assign60090_e93762_d_n2, assign60090_e93762_d_n4, assign60090_e93762_d_n5, assign60090_e93762_d_n6, assign60090_e93762_d_n7, assign60090_e93762_d_n8, assign60090_e93762_d_n9, assign60090_e93762_d_n10, assign60090_e93762_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60090_e93762;
        locals.var_t1_dn0 = assign60090_e93762_d_n0;
        locals.var_t1_dn2 = assign60090_e93762_d_n2;
        locals.var_t1_dn4 = assign60090_e93762_d_n4;
        locals.var_t1_dn5 = assign60090_e93762_d_n5;
        locals.var_t1_dn6 = assign60090_e93762_d_n6;
        locals.var_t1_dn7 = assign60090_e93762_d_n7;
        locals.var_t1_dn8 = assign60090_e93762_d_n8;
        locals.var_t1_dn9 = assign60090_e93762_d_n9;
        locals.var_t1_dn10 = assign60090_e93762_d_n10;
        locals.var_t1_dn13 = assign60090_e93762_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60100_e93777, assign60100_e93777_d_n0, assign60100_e93777_d_n2, assign60100_e93777_d_n4, assign60100_e93777_d_n5, assign60100_e93777_d_n6, assign60100_e93777_d_n7, assign60100_e93777_d_n8, assign60100_e93777_d_n9, assign60100_e93777_d_n10, assign60100_e93777_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60100_e93774: f64 = (locals.var_psl - locals.var_vbscl__blk435);
        let assign60100_e93775: f64 = (assign60100_e93774).sqrt();
        (assign60100_e93775, ((locals.var_psl_dn0 - locals.var_vbscl__blk435_dn0) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn2 - locals.var_vbscl__blk435_dn2) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn4 - locals.var_vbscl__blk435_dn4) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn5 - locals.var_vbscl__blk435_dn5) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn6 - locals.var_vbscl__blk435_dn6) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn7 - locals.var_vbscl__blk435_dn7) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn8 - locals.var_vbscl__blk435_dn8) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn9 - locals.var_vbscl__blk435_dn9) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn10 - locals.var_vbscl__blk435_dn10) / (2.0 * assign60100_e93775)), ((locals.var_psl_dn13 - locals.var_vbscl__blk435_dn13) / (2.0 * assign60100_e93775)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign60100_e93777;
        locals.var_t8_dn0 = assign60100_e93777_d_n0;
        locals.var_t8_dn2 = assign60100_e93777_d_n2;
        locals.var_t8_dn4 = assign60100_e93777_d_n4;
        locals.var_t8_dn5 = assign60100_e93777_d_n5;
        locals.var_t8_dn6 = assign60100_e93777_d_n6;
        locals.var_t8_dn7 = assign60100_e93777_d_n7;
        locals.var_t8_dn8 = assign60100_e93777_d_n8;
        locals.var_t8_dn9 = assign60100_e93777_d_n9;
        locals.var_t8_dn10 = assign60100_e93777_d_n10;
        locals.var_t8_dn13 = assign60100_e93777_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign60110_e93791, assign60110_e93791_d_n0, assign60110_e93791_d_n2, assign60110_e93791_d_n4, assign60110_e93791_d_n5, assign60110_e93791_d_n6, assign60110_e93791_d_n7, assign60110_e93791_d_n8, assign60110_e93791_d_n9, assign60110_e93791_d_n10, assign60110_e93791_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60110_e93789: f64 = (locals.var_t1 * locals.var_t8);
        (assign60110_e93789, ((locals.var_t1_dn0 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn0)), ((locals.var_t1_dn2 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn2)), ((locals.var_t1_dn4 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn4)), ((locals.var_t1_dn5 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn5)), ((locals.var_t1_dn6 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn6)), ((locals.var_t1_dn7 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn7)), ((locals.var_t1_dn8 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn8)), ((locals.var_t1_dn9 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn9)), ((locals.var_t1_dn10 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn10)), ((locals.var_t1_dn13 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn13)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn2, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn13,)
    }
};
        locals.var_wd = assign60110_e93791;
        locals.var_wd_dn0 = assign60110_e93791_d_n0;
        locals.var_wd_dn2 = assign60110_e93791_d_n2;
        locals.var_wd_dn4 = assign60110_e93791_d_n4;
        locals.var_wd_dn5 = assign60110_e93791_d_n5;
        locals.var_wd_dn6 = assign60110_e93791_d_n6;
        locals.var_wd_dn7 = assign60110_e93791_d_n7;
        locals.var_wd_dn8 = assign60110_e93791_d_n8;
        locals.var_wd_dn9 = assign60110_e93791_d_n9;
        locals.var_wd_dn10 = assign60110_e93791_d_n10;
        locals.var_wd_dn13 = assign60110_e93791_d_n13;
        locals.var_wd_rv = 0.0;

        let (assign60120_e93807, assign60120_e93807_d_n0, assign60120_e93807_d_n2, assign60120_e93807_d_n4, assign60120_e93807_d_n5, assign60120_e93807_d_n6, assign60120_e93807_d_n7, assign60120_e93807_d_n8, assign60120_e93807_d_n9, assign60120_e93807_d_n10, assign60120_e93807_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60120_e93803: f64 = (0.5 * locals.var_t1);
        let assign60120_e93805: f64 = (assign60120_e93803 / locals.var_t8);
        (assign60120_e93805, ((((0.5 * locals.var_t1_dn0) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn0)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn2) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn2)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn4) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn5) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn6) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn7) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn8) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn9) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn10) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn13) * locals.var_t8) - (assign60120_e93803 * locals.var_t8_dn13)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign60120_e93807;
        locals.var_t9_dn0 = assign60120_e93807_d_n0;
        locals.var_t9_dn2 = assign60120_e93807_d_n2;
        locals.var_t9_dn4 = assign60120_e93807_d_n4;
        locals.var_t9_dn5 = assign60120_e93807_d_n5;
        locals.var_t9_dn6 = assign60120_e93807_d_n6;
        locals.var_t9_dn7 = assign60120_e93807_d_n7;
        locals.var_t9_dn8 = assign60120_e93807_d_n8;
        locals.var_t9_dn9 = assign60120_e93807_d_n9;
        locals.var_t9_dn10 = assign60120_e93807_d_n10;
        locals.var_t9_dn13 = assign60120_e93807_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign60130_e93821, assign60130_e93821_d_n0, assign60130_e93821_d_n2, assign60130_e93821_d_n4, assign60130_e93821_d_n5, assign60130_e93821_d_n6, assign60130_e93821_d_n7, assign60130_e93821_d_n8, assign60130_e93821_d_n9, assign60130_e93821_d_n10, assign60130_e93821_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60130_e93819: f64 = (1.0 / locals.var_wd);
        (assign60130_e93819, (-(locals.var_wd_dn0 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn2 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn4 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn5 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn6 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn7 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn8 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn9 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn10 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn13 / (locals.var_wd * locals.var_wd))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60130_e93821;
        locals.var_t0_dn0 = assign60130_e93821_d_n0;
        locals.var_t0_dn2 = assign60130_e93821_d_n2;
        locals.var_t0_dn4 = assign60130_e93821_d_n4;
        locals.var_t0_dn5 = assign60130_e93821_d_n5;
        locals.var_t0_dn6 = assign60130_e93821_d_n6;
        locals.var_t0_dn7 = assign60130_e93821_d_n7;
        locals.var_t0_dn8 = assign60130_e93821_d_n8;
        locals.var_t0_dn9 = assign60130_e93821_d_n9;
        locals.var_t0_dn10 = assign60130_e93821_d_n10;
        locals.var_t0_dn13 = assign60130_e93821_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60140_e93835, assign60140_e93835_d_n0, assign60140_e93835_d_n2, assign60140_e93835_d_n4, assign60140_e93835_d_n5, assign60140_e93835_d_n6, assign60140_e93835_d_n7, assign60140_e93835_d_n8, assign60140_e93835_d_n9, assign60140_e93835_d_n10, assign60140_e93835_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60140_e93833: f64 = (locals.var_qn0 * locals.var_t0);
        (assign60140_e93833, ((locals.var_qn0_dn0 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn0)), ((locals.var_qn0_dn2 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn2)), ((locals.var_qn0_dn4 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn4)), ((locals.var_qn0_dn5 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn5)), ((locals.var_qn0_dn6 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn6)), ((locals.var_qn0_dn7 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn7)), ((locals.var_qn0_dn8 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn8)), ((locals.var_qn0_dn9 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn9)), ((locals.var_qn0_dn10 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn10)), ((locals.var_qn0_dn13 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60140_e93835;
        locals.var_t1_dn0 = assign60140_e93835_d_n0;
        locals.var_t1_dn2 = assign60140_e93835_d_n2;
        locals.var_t1_dn4 = assign60140_e93835_d_n4;
        locals.var_t1_dn5 = assign60140_e93835_d_n5;
        locals.var_t1_dn6 = assign60140_e93835_d_n6;
        locals.var_t1_dn7 = assign60140_e93835_d_n7;
        locals.var_t1_dn8 = assign60140_e93835_d_n8;
        locals.var_t1_dn9 = assign60140_e93835_d_n9;
        locals.var_t1_dn10 = assign60140_e93835_d_n10;
        locals.var_t1_dn13 = assign60140_e93835_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60150_e93849, assign60150_e93849_d_n0, assign60150_e93849_d_n2, assign60150_e93849_d_n4, assign60150_e93849_d_n5, assign60150_e93849_d_n6, assign60150_e93849_d_n7, assign60150_e93849_d_n8, assign60150_e93849_d_n9, assign60150_e93849_d_n10, assign60150_e93849_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60150_e93847: f64 = (locals.var_uc_clm3 * locals.var_t1);
        (assign60150_e93847, (locals.var_uc_clm3 * locals.var_t1_dn0), (locals.var_uc_clm3 * locals.var_t1_dn2), (locals.var_uc_clm3 * locals.var_t1_dn4), (locals.var_uc_clm3 * locals.var_t1_dn5), (locals.var_uc_clm3 * locals.var_t1_dn6), (locals.var_uc_clm3 * locals.var_t1_dn7), (locals.var_uc_clm3 * locals.var_t1_dn8), (locals.var_uc_clm3 * locals.var_t1_dn9), (locals.var_uc_clm3 * locals.var_t1_dn10), (locals.var_uc_clm3 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign60150_e93849;
        locals.var_t2_dn0 = assign60150_e93849_d_n0;
        locals.var_t2_dn2 = assign60150_e93849_d_n2;
        locals.var_t2_dn4 = assign60150_e93849_d_n4;
        locals.var_t2_dn5 = assign60150_e93849_d_n5;
        locals.var_t2_dn6 = assign60150_e93849_d_n6;
        locals.var_t2_dn7 = assign60150_e93849_d_n7;
        locals.var_t2_dn8 = assign60150_e93849_d_n8;
        locals.var_t2_dn9 = assign60150_e93849_d_n9;
        locals.var_t2_dn10 = assign60150_e93849_d_n10;
        locals.var_t2_dn13 = assign60150_e93849_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign60160_e93863, assign60160_e93863_d_n0, assign60160_e93863_d_n2, assign60160_e93863_d_n4, assign60160_e93863_d_n5, assign60160_e93863_d_n6, assign60160_e93863_d_n7, assign60160_e93863_d_n8, assign60160_e93863_d_n9, assign60160_e93863_d_n10, assign60160_e93863_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60160_e93861: f64 = (locals.var_uc_clm3 * locals.var_t0);
        (assign60160_e93861, (locals.var_uc_clm3 * locals.var_t0_dn0), (locals.var_uc_clm3 * locals.var_t0_dn2), (locals.var_uc_clm3 * locals.var_t0_dn4), (locals.var_uc_clm3 * locals.var_t0_dn5), (locals.var_uc_clm3 * locals.var_t0_dn6), (locals.var_uc_clm3 * locals.var_t0_dn7), (locals.var_uc_clm3 * locals.var_t0_dn8), (locals.var_uc_clm3 * locals.var_t0_dn9), (locals.var_uc_clm3 * locals.var_t0_dn10), (locals.var_uc_clm3 * locals.var_t0_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign60160_e93863;
        locals.var_t3_dn0 = assign60160_e93863_d_n0;
        locals.var_t3_dn2 = assign60160_e93863_d_n2;
        locals.var_t3_dn4 = assign60160_e93863_d_n4;
        locals.var_t3_dn5 = assign60160_e93863_d_n5;
        locals.var_t3_dn6 = assign60160_e93863_d_n6;
        locals.var_t3_dn7 = assign60160_e93863_d_n7;
        locals.var_t3_dn8 = assign60160_e93863_d_n8;
        locals.var_t3_dn9 = assign60160_e93863_d_n9;
        locals.var_t3_dn10 = assign60160_e93863_d_n10;
        locals.var_t3_dn13 = assign60160_e93863_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign60170_e93879, assign60170_e93879_d_n0, assign60170_e93879_d_n2, assign60170_e93879_d_n4, assign60170_e93879_d_n5, assign60170_e93879_d_n6, assign60170_e93879_d_n7, assign60170_e93879_d_n8, assign60170_e93879_d_n9, assign60170_e93879_d_n10, assign60170_e93879_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60170_e93875: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign60170_e93877: f64 = (assign60170_e93875 + locals.var_t2);
        (assign60170_e93877, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2_dn2), (((locals.var_uc_clm2_dn4 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn4)) + locals.var_t2_dn4), (((locals.var_uc_clm2_dn5 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn5)) + locals.var_t2_dn5), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2_dn7), (((locals.var_uc_clm2_dn8 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn8)) + locals.var_t2_dn8), (((locals.var_uc_clm2_dn9 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn9)) + locals.var_t2_dn9), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2_dn10), (((locals.var_uc_clm2_dn13 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn13)) + locals.var_t2_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign60170_e93879;
        locals.var_t5_dn0 = assign60170_e93879_d_n0;
        locals.var_t5_dn2 = assign60170_e93879_d_n2;
        locals.var_t5_dn4 = assign60170_e93879_d_n4;
        locals.var_t5_dn5 = assign60170_e93879_d_n5;
        locals.var_t5_dn6 = assign60170_e93879_d_n6;
        locals.var_t5_dn7 = assign60170_e93879_d_n7;
        locals.var_t5_dn8 = assign60170_e93879_d_n8;
        locals.var_t5_dn9 = assign60170_e93879_d_n9;
        locals.var_t5_dn10 = assign60170_e93879_d_n10;
        locals.var_t5_dn13 = assign60170_e93879_d_n13;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_215(
        locals: &mut StampLocals,
    ) {
        let (assign60180_e93893, assign60180_e93893_d_n0, assign60180_e93893_d_n2, assign60180_e93893_d_n4, assign60180_e93893_d_n5, assign60180_e93893_d_n6, assign60180_e93893_d_n7, assign60180_e93893_d_n8, assign60180_e93893_d_n9, assign60180_e93893_d_n10, assign60180_e93893_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60180_e93891: f64 = (1.0 / locals.var_t5);
        (assign60180_e93891, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn13 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60180_e93893;
        locals.var_t1_dn0 = assign60180_e93893_d_n0;
        locals.var_t1_dn2 = assign60180_e93893_d_n2;
        locals.var_t1_dn4 = assign60180_e93893_d_n4;
        locals.var_t1_dn5 = assign60180_e93893_d_n5;
        locals.var_t1_dn6 = assign60180_e93893_d_n6;
        locals.var_t1_dn7 = assign60180_e93893_d_n7;
        locals.var_t1_dn8 = assign60180_e93893_d_n8;
        locals.var_t1_dn9 = assign60180_e93893_d_n9;
        locals.var_t1_dn10 = assign60180_e93893_d_n10;
        locals.var_t1_dn13 = assign60180_e93893_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60190_e93907, assign60190_e93907_d_n0, assign60190_e93907_d_n2, assign60190_e93907_d_n4, assign60190_e93907_d_n5, assign60190_e93907_d_n6, assign60190_e93907_d_n7, assign60190_e93907_d_n8, assign60190_e93907_d_n9, assign60190_e93907_d_n10, assign60190_e93907_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60190_e93905: f64 = (1.034943e-10 * locals.var_t1);
        (assign60190_e93905, (1.034943e-10 * locals.var_t1_dn0), (1.034943e-10 * locals.var_t1_dn2), (1.034943e-10 * locals.var_t1_dn4), (1.034943e-10 * locals.var_t1_dn5), (1.034943e-10 * locals.var_t1_dn6), (1.034943e-10 * locals.var_t1_dn7), (1.034943e-10 * locals.var_t1_dn8), (1.034943e-10 * locals.var_t1_dn9), (1.034943e-10 * locals.var_t1_dn10), (1.034943e-10 * locals.var_t1_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign60190_e93907;
        locals.var_t4_dn0 = assign60190_e93907_d_n0;
        locals.var_t4_dn2 = assign60190_e93907_d_n2;
        locals.var_t4_dn4 = assign60190_e93907_d_n4;
        locals.var_t4_dn5 = assign60190_e93907_d_n5;
        locals.var_t4_dn6 = assign60190_e93907_d_n6;
        locals.var_t4_dn7 = assign60190_e93907_d_n7;
        locals.var_t4_dn8 = assign60190_e93907_d_n8;
        locals.var_t4_dn9 = assign60190_e93907_d_n9;
        locals.var_t4_dn10 = assign60190_e93907_d_n10;
        locals.var_t4_dn13 = assign60190_e93907_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign60200_e93921, assign60200_e93921_d_n0, assign60200_e93921_d_n2, assign60200_e93921_d_n4, assign60200_e93921_d_n5, assign60200_e93921_d_n6, assign60200_e93921_d_n7, assign60200_e93921_d_n8, assign60200_e93921_d_n9, assign60200_e93921_d_n10, assign60200_e93921_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60200_e93919: f64 = (1.0 - locals.var_uc_clm1);
        (assign60200_e93919, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60200_e93921;
        locals.var_t1_dn0 = assign60200_e93921_d_n0;
        locals.var_t1_dn2 = assign60200_e93921_d_n2;
        locals.var_t1_dn4 = assign60200_e93921_d_n4;
        locals.var_t1_dn5 = assign60200_e93921_d_n5;
        locals.var_t1_dn6 = assign60200_e93921_d_n6;
        locals.var_t1_dn7 = assign60200_e93921_d_n7;
        locals.var_t1_dn8 = assign60200_e93921_d_n8;
        locals.var_t1_dn9 = assign60200_e93921_d_n9;
        locals.var_t1_dn10 = assign60200_e93921_d_n10;
        locals.var_t1_dn13 = assign60200_e93921_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60210_e93941, assign60210_e93941_d_n0, assign60210_e93941_d_n2, assign60210_e93941_d_n4, assign60210_e93941_d_n5, assign60210_e93941_d_n6, assign60210_e93941_d_n7, assign60210_e93941_d_n8, assign60210_e93941_d_n9, assign60210_e93941_d_n10, assign60210_e93941_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60210_e93934: f64 = (locals.var_vds + locals.var_ps0);
        let assign60210_e93935: f64 = (locals.var_uc_clm1 * assign60210_e93934);
        let assign60210_e93938: f64 = (locals.var_t1 * locals.var_psl);
        let assign60210_e93939: f64 = (assign60210_e93935 + assign60210_e93938);
        (assign60210_e93939, ((locals.var_uc_clm1 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1_dn0 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn0))), ((locals.var_uc_clm1 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1_dn2 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn2))), ((locals.var_uc_clm1 * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + ((locals.var_t1_dn4 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn4))), ((locals.var_uc_clm1 * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + ((locals.var_t1_dn5 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn5))), ((locals.var_uc_clm1 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1_dn6 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn6))), ((locals.var_uc_clm1 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1_dn7 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn7))), ((locals.var_uc_clm1 * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + ((locals.var_t1_dn8 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn8))), ((locals.var_uc_clm1 * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + ((locals.var_t1_dn9 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn9))), ((locals.var_uc_clm1 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1_dn10 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn10))), ((locals.var_uc_clm1 * (locals.var_vds_dn13 + locals.var_ps0_dn13)) + ((locals.var_t1_dn13 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn13))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign60210_e93941;
        locals.var_psdl_dn0 = assign60210_e93941_d_n0;
        locals.var_psdl_dn2 = assign60210_e93941_d_n2;
        locals.var_psdl_dn4 = assign60210_e93941_d_n4;
        locals.var_psdl_dn5 = assign60210_e93941_d_n5;
        locals.var_psdl_dn6 = assign60210_e93941_d_n6;
        locals.var_psdl_dn7 = assign60210_e93941_d_n7;
        locals.var_psdl_dn8 = assign60210_e93941_d_n8;
        locals.var_psdl_dn9 = assign60210_e93941_d_n9;
        locals.var_psdl_dn10 = assign60210_e93941_d_n10;
        locals.var_psdl_dn13 = assign60210_e93941_d_n13;
        locals.var_psdl_rv = 0.0;

        let assign60220_e93945: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60220_e93948: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93949: f64 = (assign60220_e93945 - assign60220_e93948);
        let assign60220_e93952: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93953: f64 = (assign60220_e93949 - assign60220_e93952);
        let assign60220_e93957: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93960: f64 = if ((locals.var_psdl > assign60220_e93953) && (assign60220_e93957 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign60220_e93960;
        locals.var_guard1469_rv = 0.0;

        let (assign60230_e93986, assign60230_e93986_d_n0, assign60230_e93986_d_n2, assign60230_e93986_d_n4, assign60230_e93986_d_n5, assign60230_e93986_d_n6, assign60230_e93986_d_n7, assign60230_e93986_d_n8, assign60230_e93986_d_n9, assign60230_e93986_d_n10, assign60230_e93986_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60230_e93975: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60230_e93978: f64 = (10.0 * 2.220446049250313e-16);
        let assign60230_e93979: f64 = (assign60230_e93975 - assign60230_e93978);
        let assign60230_e93980: f64 = (locals.var_psdl - assign60230_e93979);
        let assign60230_e93983: f64 = (10.0 * 2.220446049250313e-16);
        let assign60230_e93984: f64 = (assign60230_e93980 + assign60230_e93983);
        (assign60230_e93984, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign60230_e93986;
        locals.var_tmf1_dn0 = assign60230_e93986_d_n0;
        locals.var_tmf1_dn2 = assign60230_e93986_d_n2;
        locals.var_tmf1_dn4 = assign60230_e93986_d_n4;
        locals.var_tmf1_dn5 = assign60230_e93986_d_n5;
        locals.var_tmf1_dn6 = assign60230_e93986_d_n6;
        locals.var_tmf1_dn7 = assign60230_e93986_d_n7;
        locals.var_tmf1_dn8 = assign60230_e93986_d_n8;
        locals.var_tmf1_dn9 = assign60230_e93986_d_n9;
        locals.var_tmf1_dn10 = assign60230_e93986_d_n10;
        locals.var_tmf1_dn13 = assign60230_e93986_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign60240_e94002, assign60240_e94002_d_n0, assign60240_e94002_d_n2, assign60240_e94002_d_n4, assign60240_e94002_d_n5, assign60240_e94002_d_n6, assign60240_e94002_d_n7, assign60240_e94002_d_n8, assign60240_e94002_d_n9, assign60240_e94002_d_n10, assign60240_e94002_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60240_e94000: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign60240_e94000, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign60240_e94002;
        locals.var_x2_dn0 = assign60240_e94002_d_n0;
        locals.var_x2_dn2 = assign60240_e94002_d_n2;
        locals.var_x2_dn4 = assign60240_e94002_d_n4;
        locals.var_x2_dn5 = assign60240_e94002_d_n5;
        locals.var_x2_dn6 = assign60240_e94002_d_n6;
        locals.var_x2_dn7 = assign60240_e94002_d_n7;
        locals.var_x2_dn8 = assign60240_e94002_d_n8;
        locals.var_x2_dn9 = assign60240_e94002_d_n9;
        locals.var_x2_dn10 = assign60240_e94002_d_n10;
        locals.var_x2_dn13 = assign60240_e94002_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign60250_e94022, assign60250_e94022_d_n0, assign60250_e94022_d_n2, assign60250_e94022_d_n4, assign60250_e94022_d_n5, assign60250_e94022_d_n6, assign60250_e94022_d_n7, assign60250_e94022_d_n8, assign60250_e94022_d_n9, assign60250_e94022_d_n10, assign60250_e94022_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60250_e94016: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e94019: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e94020: f64 = (assign60250_e94016 * assign60250_e94019);
        (assign60250_e94020, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign60250_e94022;
        locals.var_xmax2_dn0 = assign60250_e94022_d_n0;
        locals.var_xmax2_dn2 = assign60250_e94022_d_n2;
        locals.var_xmax2_dn4 = assign60250_e94022_d_n4;
        locals.var_xmax2_dn5 = assign60250_e94022_d_n5;
        locals.var_xmax2_dn6 = assign60250_e94022_d_n6;
        locals.var_xmax2_dn7 = assign60250_e94022_d_n7;
        locals.var_xmax2_dn8 = assign60250_e94022_d_n8;
        locals.var_xmax2_dn9 = assign60250_e94022_d_n9;
        locals.var_xmax2_dn10 = assign60250_e94022_d_n10;
        locals.var_xmax2_dn13 = assign60250_e94022_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign60260_e94036, assign60260_e94036_d_n0, assign60260_e94036_d_n2, assign60260_e94036_d_n4, assign60260_e94036_d_n5, assign60260_e94036_d_n6, assign60260_e94036_d_n7, assign60260_e94036_d_n8, assign60260_e94036_d_n9, assign60260_e94036_d_n10, assign60260_e94036_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign60260_e94036;
        locals.var_xp_dn0 = assign60260_e94036_d_n0;
        locals.var_xp_dn2 = assign60260_e94036_d_n2;
        locals.var_xp_dn4 = assign60260_e94036_d_n4;
        locals.var_xp_dn5 = assign60260_e94036_d_n5;
        locals.var_xp_dn6 = assign60260_e94036_d_n6;
        locals.var_xp_dn7 = assign60260_e94036_d_n7;
        locals.var_xp_dn8 = assign60260_e94036_d_n8;
        locals.var_xp_dn9 = assign60260_e94036_d_n9;
        locals.var_xp_dn10 = assign60260_e94036_d_n10;
        locals.var_xp_dn13 = assign60260_e94036_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign60270_e94050, assign60270_e94050_d_n0, assign60270_e94050_d_n2, assign60270_e94050_d_n4, assign60270_e94050_d_n5, assign60270_e94050_d_n6, assign60270_e94050_d_n7, assign60270_e94050_d_n8, assign60270_e94050_d_n9, assign60270_e94050_d_n10, assign60270_e94050_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign60270_e94050;
        locals.var_xmp_dn0 = assign60270_e94050_d_n0;
        locals.var_xmp_dn2 = assign60270_e94050_d_n2;
        locals.var_xmp_dn4 = assign60270_e94050_d_n4;
        locals.var_xmp_dn5 = assign60270_e94050_d_n5;
        locals.var_xmp_dn6 = assign60270_e94050_d_n6;
        locals.var_xmp_dn7 = assign60270_e94050_d_n7;
        locals.var_xmp_dn8 = assign60270_e94050_d_n8;
        locals.var_xmp_dn9 = assign60270_e94050_d_n9;
        locals.var_xmp_dn10 = assign60270_e94050_d_n10;
        locals.var_xmp_dn13 = assign60270_e94050_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign60280_e94064,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60280_e94064;
        locals.var_m0_rv = 0.0;

        let (assign60290_e94078,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60290_e94078;
        locals.var_mm_rv = 0.0;

        let (assign60300_e94092, assign60300_e94092_d_n0, assign60300_e94092_d_n2, assign60300_e94092_d_n4, assign60300_e94092_d_n5, assign60300_e94092_d_n6, assign60300_e94092_d_n7, assign60300_e94092_d_n8, assign60300_e94092_d_n9, assign60300_e94092_d_n10, assign60300_e94092_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign60300_e94092;
        locals.var_arg_dn0 = assign60300_e94092_d_n0;
        locals.var_arg_dn2 = assign60300_e94092_d_n2;
        locals.var_arg_dn4 = assign60300_e94092_d_n4;
        locals.var_arg_dn5 = assign60300_e94092_d_n5;
        locals.var_arg_dn6 = assign60300_e94092_d_n6;
        locals.var_arg_dn7 = assign60300_e94092_d_n7;
        locals.var_arg_dn8 = assign60300_e94092_d_n8;
        locals.var_arg_dn9 = assign60300_e94092_d_n9;
        locals.var_arg_dn10 = assign60300_e94092_d_n10;
        locals.var_arg_dn13 = assign60300_e94092_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign60310_e94106, assign60310_e94106_d_n0, assign60310_e94106_d_n2, assign60310_e94106_d_n4, assign60310_e94106_d_n5, assign60310_e94106_d_n6, assign60310_e94106_d_n7, assign60310_e94106_d_n8, assign60310_e94106_d_n9, assign60310_e94106_d_n10, assign60310_e94106_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60310_e94106;
        locals.var_dnm_dn0 = assign60310_e94106_d_n0;
        locals.var_dnm_dn2 = assign60310_e94106_d_n2;
        locals.var_dnm_dn4 = assign60310_e94106_d_n4;
        locals.var_dnm_dn5 = assign60310_e94106_d_n5;
        locals.var_dnm_dn6 = assign60310_e94106_d_n6;
        locals.var_dnm_dn7 = assign60310_e94106_d_n7;
        locals.var_dnm_dn8 = assign60310_e94106_d_n8;
        locals.var_dnm_dn9 = assign60310_e94106_d_n9;
        locals.var_dnm_dn10 = assign60310_e94106_d_n10;
        locals.var_dnm_dn13 = assign60310_e94106_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign60320_e94122, assign60320_e94122_d_n0, assign60320_e94122_d_n2, assign60320_e94122_d_n4, assign60320_e94122_d_n5, assign60320_e94122_d_n6, assign60320_e94122_d_n7, assign60320_e94122_d_n8, assign60320_e94122_d_n9, assign60320_e94122_d_n10, assign60320_e94122_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60320_e94120: f64 = (locals.var_xp * locals.var_x2);
        (assign60320_e94120, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign60320_e94122;
        locals.var_xp_dn0 = assign60320_e94122_d_n0;
        locals.var_xp_dn2 = assign60320_e94122_d_n2;
        locals.var_xp_dn4 = assign60320_e94122_d_n4;
        locals.var_xp_dn5 = assign60320_e94122_d_n5;
        locals.var_xp_dn6 = assign60320_e94122_d_n6;
        locals.var_xp_dn7 = assign60320_e94122_d_n7;
        locals.var_xp_dn8 = assign60320_e94122_d_n8;
        locals.var_xp_dn9 = assign60320_e94122_d_n9;
        locals.var_xp_dn10 = assign60320_e94122_d_n10;
        locals.var_xp_dn13 = assign60320_e94122_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign60330_e94138, assign60330_e94138_d_n0, assign60330_e94138_d_n2, assign60330_e94138_d_n4, assign60330_e94138_d_n5, assign60330_e94138_d_n6, assign60330_e94138_d_n7, assign60330_e94138_d_n8, assign60330_e94138_d_n9, assign60330_e94138_d_n10, assign60330_e94138_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60330_e94136: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60330_e94136, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign60330_e94138;
        locals.var_xmp_dn0 = assign60330_e94138_d_n0;
        locals.var_xmp_dn2 = assign60330_e94138_d_n2;
        locals.var_xmp_dn4 = assign60330_e94138_d_n4;
        locals.var_xmp_dn5 = assign60330_e94138_d_n5;
        locals.var_xmp_dn6 = assign60330_e94138_d_n6;
        locals.var_xmp_dn7 = assign60330_e94138_d_n7;
        locals.var_xmp_dn8 = assign60330_e94138_d_n8;
        locals.var_xmp_dn9 = assign60330_e94138_d_n9;
        locals.var_xmp_dn10 = assign60330_e94138_d_n10;
        locals.var_xmp_dn13 = assign60330_e94138_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign60340_e94154, assign60340_e94154_d_n0, assign60340_e94154_d_n2, assign60340_e94154_d_n4, assign60340_e94154_d_n5, assign60340_e94154_d_n6, assign60340_e94154_d_n7, assign60340_e94154_d_n8, assign60340_e94154_d_n9, assign60340_e94154_d_n10, assign60340_e94154_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60340_e94152: f64 = (locals.var_xp * locals.var_x2);
        (assign60340_e94152, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign60340_e94154;
        locals.var_xp_dn0 = assign60340_e94154_d_n0;
        locals.var_xp_dn2 = assign60340_e94154_d_n2;
        locals.var_xp_dn4 = assign60340_e94154_d_n4;
        locals.var_xp_dn5 = assign60340_e94154_d_n5;
        locals.var_xp_dn6 = assign60340_e94154_d_n6;
        locals.var_xp_dn7 = assign60340_e94154_d_n7;
        locals.var_xp_dn8 = assign60340_e94154_d_n8;
        locals.var_xp_dn9 = assign60340_e94154_d_n9;
        locals.var_xp_dn10 = assign60340_e94154_d_n10;
        locals.var_xp_dn13 = assign60340_e94154_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign60350_e94170, assign60350_e94170_d_n0, assign60350_e94170_d_n2, assign60350_e94170_d_n4, assign60350_e94170_d_n5, assign60350_e94170_d_n6, assign60350_e94170_d_n7, assign60350_e94170_d_n8, assign60350_e94170_d_n9, assign60350_e94170_d_n10, assign60350_e94170_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60350_e94168: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60350_e94168, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign60350_e94170;
        locals.var_xmp_dn0 = assign60350_e94170_d_n0;
        locals.var_xmp_dn2 = assign60350_e94170_d_n2;
        locals.var_xmp_dn4 = assign60350_e94170_d_n4;
        locals.var_xmp_dn5 = assign60350_e94170_d_n5;
        locals.var_xmp_dn6 = assign60350_e94170_d_n6;
        locals.var_xmp_dn7 = assign60350_e94170_d_n7;
        locals.var_xmp_dn8 = assign60350_e94170_d_n8;
        locals.var_xmp_dn9 = assign60350_e94170_d_n9;
        locals.var_xmp_dn10 = assign60350_e94170_d_n10;
        locals.var_xmp_dn13 = assign60350_e94170_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign60360_e94186, assign60360_e94186_d_n0, assign60360_e94186_d_n2, assign60360_e94186_d_n4, assign60360_e94186_d_n5, assign60360_e94186_d_n6, assign60360_e94186_d_n7, assign60360_e94186_d_n8, assign60360_e94186_d_n9, assign60360_e94186_d_n10, assign60360_e94186_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60360_e94184: f64 = (locals.var_xp + locals.var_xmp);
        (assign60360_e94184, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign60360_e94186;
        locals.var_arg_dn0 = assign60360_e94186_d_n0;
        locals.var_arg_dn2 = assign60360_e94186_d_n2;
        locals.var_arg_dn4 = assign60360_e94186_d_n4;
        locals.var_arg_dn5 = assign60360_e94186_d_n5;
        locals.var_arg_dn6 = assign60360_e94186_d_n6;
        locals.var_arg_dn7 = assign60360_e94186_d_n7;
        locals.var_arg_dn8 = assign60360_e94186_d_n8;
        locals.var_arg_dn9 = assign60360_e94186_d_n9;
        locals.var_arg_dn10 = assign60360_e94186_d_n10;
        locals.var_arg_dn13 = assign60360_e94186_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign60370_e94200, assign60370_e94200_d_n0, assign60370_e94200_d_n2, assign60370_e94200_d_n4, assign60370_e94200_d_n5, assign60370_e94200_d_n6, assign60370_e94200_d_n7, assign60370_e94200_d_n8, assign60370_e94200_d_n9, assign60370_e94200_d_n10, assign60370_e94200_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60370_e94200;
        locals.var_dnm_dn0 = assign60370_e94200_d_n0;
        locals.var_dnm_dn2 = assign60370_e94200_d_n2;
        locals.var_dnm_dn4 = assign60370_e94200_d_n4;
        locals.var_dnm_dn5 = assign60370_e94200_d_n5;
        locals.var_dnm_dn6 = assign60370_e94200_d_n6;
        locals.var_dnm_dn7 = assign60370_e94200_d_n7;
        locals.var_dnm_dn8 = assign60370_e94200_d_n8;
        locals.var_dnm_dn9 = assign60370_e94200_d_n9;
        locals.var_dnm_dn10 = assign60370_e94200_d_n10;
        locals.var_dnm_dn13 = assign60370_e94200_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign60380_e94215: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign60380_e94215;
        locals.var_guard1470_rv = 0.0;

        let assign60390_e94218: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign60390_e94218;
        locals.var_guard1471_rv = 0.0;

        let (assign60400_e94236,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) && (locals.var_guard1471 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60400_e94236;
        locals.var_mm_rv = 0.0;

        let assign60410_e94239: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign60410_e94239;
        locals.var_guard1472_rv = 0.0;

        let (assign60420_e94260,) = {
    if ((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) && (locals.var_guard1471 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60420_e94260;
        locals.var_mm_rv = 0.0;

        let assign60430_e94263: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign60430_e94263;
        locals.var_guard1473_rv = 0.0;

        let (assign60440_e94287,) = {
    if (((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) && (locals.var_guard1471 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60440_e94287;
        locals.var_mm_rv = 0.0;

        let assign60450_e94290: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign60450_e94290;
        locals.var_guard1474_rv = 0.0;

        let (assign60460_e94317,) = {
    if ((((((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) && (locals.var_guard1471 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60460_e94317;
        locals.var_mm_rv = 0.0;

        let (assign60470_e94333,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60470_e94333;
        locals.var_m0_rv = 0.0;

        let mut assign60480_loop_guard: usize = 0;
        while {
            let assign60480_cond_e94350: f64 = if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60480_cond_e94350 != 0.0
        } {
            assign60480_loop_guard += 1;
            assert!(assign60480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60480_body0_e94367, assign60480_body0_e94367_d_n0, assign60480_body0_e94367_d_n2, assign60480_body0_e94367_d_n4, assign60480_body0_e94367_d_n5, assign60480_body0_e94367_d_n6, assign60480_body0_e94367_d_n7, assign60480_body0_e94367_d_n8, assign60480_body0_e94367_d_n9, assign60480_body0_e94367_d_n10, assign60480_body0_e94367_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
        let assign60480_body0_e94365: f64 = (locals.var_dnm).sqrt();
        (assign60480_body0_e94365, (locals.var_dnm_dn0 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn2 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn4 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn5 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn6 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn7 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn8 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn9 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn10 / (2.0 * assign60480_body0_e94365)), (locals.var_dnm_dn13 / (2.0 * assign60480_body0_e94365)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign60480_body0_e94367;
            locals.var_dnm_dn0 = assign60480_body0_e94367_d_n0;
            locals.var_dnm_dn2 = assign60480_body0_e94367_d_n2;
            locals.var_dnm_dn4 = assign60480_body0_e94367_d_n4;
            locals.var_dnm_dn5 = assign60480_body0_e94367_d_n5;
            locals.var_dnm_dn6 = assign60480_body0_e94367_d_n6;
            locals.var_dnm_dn7 = assign60480_body0_e94367_d_n7;
            locals.var_dnm_dn8 = assign60480_body0_e94367_d_n8;
            locals.var_dnm_dn9 = assign60480_body0_e94367_d_n9;
            locals.var_dnm_dn10 = assign60480_body0_e94367_d_n10;
            locals.var_dnm_dn13 = assign60480_body0_e94367_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign60480_body1_e94385,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
        let assign60480_body1_e94383: f64 = (locals.var_m0 + 1.0);
        (assign60480_body1_e94383,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60480_body1_e94385;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_216(
        locals: &mut StampLocals,
    ) {
        let (assign60490_e94413, assign60490_e94413_d_n0, assign60490_e94413_d_n2, assign60490_e94413_d_n4, assign60490_e94413_d_n5, assign60490_e94413_d_n6, assign60490_e94413_d_n7, assign60490_e94413_d_n8, assign60490_e94413_d_n9, assign60490_e94413_d_n10, assign60490_e94413_d_n13,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 == 0.0)) {
        let (assign60490_e94411, assign60490_e94411_d_n0, assign60490_e94411_d_n2, assign60490_e94411_d_n4, assign60490_e94411_d_n5, assign60490_e94411_d_n6, assign60490_e94411_d_n7, assign60490_e94411_d_n8, assign60490_e94411_d_n9, assign60490_e94411_d_n10, assign60490_e94411_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60490_e94408: f64 = (2.0 * 2.0);
                let assign60490_e94409: f64 = (1.0 / assign60490_e94408);
                let assign60490_e94410: f64 = (locals.var_dnm).powf(assign60490_e94409);
                (assign60490_e94410, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((locals.var_dnm).powf(assign60490_e94409 - 1.0) * locals.var_dnm_dn13)) } } else { (assign60490_e94410 * (assign60490_e94409 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign60490_e94411, assign60490_e94411_d_n0, assign60490_e94411_d_n2, assign60490_e94411_d_n4, assign60490_e94411_d_n5, assign60490_e94411_d_n6, assign60490_e94411_d_n7, assign60490_e94411_d_n8, assign60490_e94411_d_n9, assign60490_e94411_d_n10, assign60490_e94411_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60490_e94413;
        locals.var_dnm_dn0 = assign60490_e94413_d_n0;
        locals.var_dnm_dn2 = assign60490_e94413_d_n2;
        locals.var_dnm_dn4 = assign60490_e94413_d_n4;
        locals.var_dnm_dn5 = assign60490_e94413_d_n5;
        locals.var_dnm_dn6 = assign60490_e94413_d_n6;
        locals.var_dnm_dn7 = assign60490_e94413_d_n7;
        locals.var_dnm_dn8 = assign60490_e94413_d_n8;
        locals.var_dnm_dn9 = assign60490_e94413_d_n9;
        locals.var_dnm_dn10 = assign60490_e94413_d_n10;
        locals.var_dnm_dn13 = assign60490_e94413_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign60500_e94429, assign60500_e94429_d_n0, assign60500_e94429_d_n2, assign60500_e94429_d_n4, assign60500_e94429_d_n5, assign60500_e94429_d_n6, assign60500_e94429_d_n7, assign60500_e94429_d_n8, assign60500_e94429_d_n9, assign60500_e94429_d_n10, assign60500_e94429_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60500_e94427: f64 = (1.0 / locals.var_dnm);
        (assign60500_e94427, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign60500_e94429;
        locals.var_dnm_dn0 = assign60500_e94429_d_n0;
        locals.var_dnm_dn2 = assign60500_e94429_d_n2;
        locals.var_dnm_dn4 = assign60500_e94429_d_n4;
        locals.var_dnm_dn5 = assign60500_e94429_d_n5;
        locals.var_dnm_dn6 = assign60500_e94429_d_n6;
        locals.var_dnm_dn7 = assign60500_e94429_d_n7;
        locals.var_dnm_dn8 = assign60500_e94429_d_n8;
        locals.var_dnm_dn9 = assign60500_e94429_d_n9;
        locals.var_dnm_dn10 = assign60500_e94429_d_n10;
        locals.var_dnm_dn13 = assign60500_e94429_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign60510_e94449, assign60510_e94449_d_n0, assign60510_e94449_d_n2, assign60510_e94449_d_n4, assign60510_e94449_d_n5, assign60510_e94449_d_n6, assign60510_e94449_d_n7, assign60510_e94449_d_n8, assign60510_e94449_d_n9, assign60510_e94449_d_n10, assign60510_e94449_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60510_e94444: f64 = (10.0 * 2.220446049250313e-16);
        let assign60510_e94445: f64 = (locals.var_tmf1 * assign60510_e94444);
        let assign60510_e94447: f64 = (assign60510_e94445 * locals.var_dnm);
        (assign60510_e94447, (((locals.var_tmf1_dn0 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign60510_e94444) * locals.var_dnm) + (assign60510_e94445 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign60510_e94449;
        locals.var_tmf0_dn0 = assign60510_e94449_d_n0;
        locals.var_tmf0_dn2 = assign60510_e94449_d_n2;
        locals.var_tmf0_dn4 = assign60510_e94449_d_n4;
        locals.var_tmf0_dn5 = assign60510_e94449_d_n5;
        locals.var_tmf0_dn6 = assign60510_e94449_d_n6;
        locals.var_tmf0_dn7 = assign60510_e94449_d_n7;
        locals.var_tmf0_dn8 = assign60510_e94449_d_n8;
        locals.var_tmf0_dn9 = assign60510_e94449_d_n9;
        locals.var_tmf0_dn10 = assign60510_e94449_d_n10;
        locals.var_tmf0_dn13 = assign60510_e94449_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign60520_e94471, assign60520_e94471_d_n0, assign60520_e94471_d_n2, assign60520_e94471_d_n4, assign60520_e94471_d_n5, assign60520_e94471_d_n6, assign60520_e94471_d_n7, assign60520_e94471_d_n8, assign60520_e94471_d_n9, assign60520_e94471_d_n10, assign60520_e94471_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60520_e94463: f64 = (10.0 * 2.220446049250313e-16);
        let assign60520_e94465: f64 = (assign60520_e94463 * locals.var_xmp);
        let assign60520_e94467: f64 = (assign60520_e94465 * locals.var_dnm);
        let assign60520_e94469: f64 = (assign60520_e94467 / locals.var_arg);
        (assign60520_e94469, ((((((assign60520_e94463 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60520_e94463 * locals.var_xmp_dn13) * locals.var_dnm) + (assign60520_e94465 * locals.var_dnm_dn13)) * locals.var_arg) - (assign60520_e94467 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60520_e94471;
        locals.var_t0_dn0 = assign60520_e94471_d_n0;
        locals.var_t0_dn2 = assign60520_e94471_d_n2;
        locals.var_t0_dn4 = assign60520_e94471_d_n4;
        locals.var_t0_dn5 = assign60520_e94471_d_n5;
        locals.var_t0_dn6 = assign60520_e94471_d_n6;
        locals.var_t0_dn7 = assign60520_e94471_d_n7;
        locals.var_t0_dn8 = assign60520_e94471_d_n8;
        locals.var_t0_dn9 = assign60520_e94471_d_n9;
        locals.var_t0_dn10 = assign60520_e94471_d_n10;
        locals.var_t0_dn13 = assign60520_e94471_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60530_e94497, assign60530_e94497_d_n0, assign60530_e94497_d_n2, assign60530_e94497_d_n4, assign60530_e94497_d_n5, assign60530_e94497_d_n6, assign60530_e94497_d_n7, assign60530_e94497_d_n8, assign60530_e94497_d_n9, assign60530_e94497_d_n10, assign60530_e94497_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign60530_e94485: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60530_e94488: f64 = (10.0 * 2.220446049250313e-16);
        let assign60530_e94489: f64 = (assign60530_e94485 - assign60530_e94488);
        let assign60530_e94492: f64 = (10.0 * 2.220446049250313e-16);
        let assign60530_e94493: f64 = (assign60530_e94489 - assign60530_e94492);
        let assign60530_e94495: f64 = (assign60530_e94493 + locals.var_tmf0);
        (assign60530_e94495, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign60530_e94497;
        locals.var_psdl_dn0 = assign60530_e94497_d_n0;
        locals.var_psdl_dn2 = assign60530_e94497_d_n2;
        locals.var_psdl_dn4 = assign60530_e94497_d_n4;
        locals.var_psdl_dn5 = assign60530_e94497_d_n5;
        locals.var_psdl_dn6 = assign60530_e94497_d_n6;
        locals.var_psdl_dn7 = assign60530_e94497_d_n7;
        locals.var_psdl_dn8 = assign60530_e94497_d_n8;
        locals.var_psdl_dn9 = assign60530_e94497_d_n9;
        locals.var_psdl_dn10 = assign60530_e94497_d_n10;
        locals.var_psdl_dn13 = assign60530_e94497_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign60540_e94511, assign60540_e94511_d_n0, assign60540_e94511_d_n2, assign60540_e94511_d_n4, assign60540_e94511_d_n5, assign60540_e94511_d_n6, assign60540_e94511_d_n7, assign60540_e94511_d_n8, assign60540_e94511_d_n9, assign60540_e94511_d_n10, assign60540_e94511_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60540_e94511;
        locals.var_t0_dn0 = assign60540_e94511_d_n0;
        locals.var_t0_dn2 = assign60540_e94511_d_n2;
        locals.var_t0_dn4 = assign60540_e94511_d_n4;
        locals.var_t0_dn5 = assign60540_e94511_d_n5;
        locals.var_t0_dn6 = assign60540_e94511_d_n6;
        locals.var_t0_dn7 = assign60540_e94511_d_n7;
        locals.var_t0_dn8 = assign60540_e94511_d_n8;
        locals.var_t0_dn9 = assign60540_e94511_d_n9;
        locals.var_t0_dn10 = assign60540_e94511_d_n10;
        locals.var_t0_dn13 = assign60540_e94511_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60550_e94526, assign60550_e94526_d_n0, assign60550_e94526_d_n2, assign60550_e94526_d_n4, assign60550_e94526_d_n5, assign60550_e94526_d_n6, assign60550_e94526_d_n7, assign60550_e94526_d_n8, assign60550_e94526_d_n9, assign60550_e94526_d_n10, assign60550_e94526_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign60550_e94526;
        locals.var_psdl_dn0 = assign60550_e94526_d_n0;
        locals.var_psdl_dn2 = assign60550_e94526_d_n2;
        locals.var_psdl_dn4 = assign60550_e94526_d_n4;
        locals.var_psdl_dn5 = assign60550_e94526_d_n5;
        locals.var_psdl_dn6 = assign60550_e94526_d_n6;
        locals.var_psdl_dn7 = assign60550_e94526_d_n7;
        locals.var_psdl_dn8 = assign60550_e94526_d_n8;
        locals.var_psdl_dn9 = assign60550_e94526_d_n9;
        locals.var_psdl_dn10 = assign60550_e94526_d_n10;
        locals.var_psdl_dn13 = assign60550_e94526_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign60560_e94541, assign60560_e94541_d_n0, assign60560_e94541_d_n2, assign60560_e94541_d_n4, assign60560_e94541_d_n5, assign60560_e94541_d_n6, assign60560_e94541_d_n7, assign60560_e94541_d_n8, assign60560_e94541_d_n9, assign60560_e94541_d_n10, assign60560_e94541_d_n13,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign60560_e94541;
        locals.var_t0_dn0 = assign60560_e94541_d_n0;
        locals.var_t0_dn2 = assign60560_e94541_d_n2;
        locals.var_t0_dn4 = assign60560_e94541_d_n4;
        locals.var_t0_dn5 = assign60560_e94541_d_n5;
        locals.var_t0_dn6 = assign60560_e94541_d_n6;
        locals.var_t0_dn7 = assign60560_e94541_d_n7;
        locals.var_t0_dn8 = assign60560_e94541_d_n8;
        locals.var_t0_dn9 = assign60560_e94541_d_n9;
        locals.var_t0_dn10 = assign60560_e94541_d_n10;
        locals.var_t0_dn13 = assign60560_e94541_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign60570_e94555, assign60570_e94555_d_n0, assign60570_e94555_d_n2, assign60570_e94555_d_n4, assign60570_e94555_d_n5, assign60570_e94555_d_n6, assign60570_e94555_d_n7, assign60570_e94555_d_n8, assign60570_e94555_d_n9, assign60570_e94555_d_n10, assign60570_e94555_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60570_e94553: f64 = (locals.var_psdl - locals.var_psl);
        (assign60570_e94553, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn4 - locals.var_psl_dn4), (locals.var_psdl_dn5 - locals.var_psl_dn5), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn8 - locals.var_psl_dn8), (locals.var_psdl_dn9 - locals.var_psl_dn9), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign60570_e94555;
        locals.var_t6_dn0 = assign60570_e94555_d_n0;
        locals.var_t6_dn2 = assign60570_e94555_d_n2;
        locals.var_t6_dn4 = assign60570_e94555_d_n4;
        locals.var_t6_dn5 = assign60570_e94555_d_n5;
        locals.var_t6_dn6 = assign60570_e94555_d_n6;
        locals.var_t6_dn7 = assign60570_e94555_d_n7;
        locals.var_t6_dn8 = assign60570_e94555_d_n8;
        locals.var_t6_dn9 = assign60570_e94555_d_n9;
        locals.var_t6_dn10 = assign60570_e94555_d_n10;
        locals.var_t6_dn13 = assign60570_e94555_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign60580_e94569, assign60580_e94569_d_n0, assign60580_e94569_d_n2, assign60580_e94569_d_n4, assign60580_e94569_d_n5, assign60580_e94569_d_n6, assign60580_e94569_d_n7, assign60580_e94569_d_n8, assign60580_e94569_d_n9, assign60580_e94569_d_n10, assign60580_e94569_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60580_e94567: f64 = (locals.var_beta * locals.var_qn0);
        (assign60580_e94567, ((locals.var_beta_dn0 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn0)), ((locals.var_beta_dn2 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn2)), ((locals.var_beta_dn4 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn4)), ((locals.var_beta_dn5 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn5)), ((locals.var_beta_dn6 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn6)), ((locals.var_beta_dn7 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn7)), ((locals.var_beta_dn8 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn8)), ((locals.var_beta_dn9 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn9)), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), ((locals.var_beta_dn13 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign60580_e94569;
        locals.var_t3_dn0 = assign60580_e94569_d_n0;
        locals.var_t3_dn2 = assign60580_e94569_d_n2;
        locals.var_t3_dn4 = assign60580_e94569_d_n4;
        locals.var_t3_dn5 = assign60580_e94569_d_n5;
        locals.var_t3_dn6 = assign60580_e94569_d_n6;
        locals.var_t3_dn7 = assign60580_e94569_d_n7;
        locals.var_t3_dn8 = assign60580_e94569_d_n8;
        locals.var_t3_dn9 = assign60580_e94569_d_n9;
        locals.var_t3_dn10 = assign60580_e94569_d_n10;
        locals.var_t3_dn13 = assign60580_e94569_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign60590_e94583, assign60590_e94583_d_n0, assign60590_e94583_d_n2, assign60590_e94583_d_n4, assign60590_e94583_d_n5, assign60590_e94583_d_n6, assign60590_e94583_d_n7, assign60590_e94583_d_n8, assign60590_e94583_d_n9, assign60590_e94583_d_n10, assign60590_e94583_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60590_e94581: f64 = (1.0 / locals.var_t3);
        (assign60590_e94581, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60590_e94583;
        locals.var_t1_dn0 = assign60590_e94583_d_n0;
        locals.var_t1_dn2 = assign60590_e94583_d_n2;
        locals.var_t1_dn4 = assign60590_e94583_d_n4;
        locals.var_t1_dn5 = assign60590_e94583_d_n5;
        locals.var_t1_dn6 = assign60590_e94583_d_n6;
        locals.var_t1_dn7 = assign60590_e94583_d_n7;
        locals.var_t1_dn8 = assign60590_e94583_d_n8;
        locals.var_t1_dn9 = assign60590_e94583_d_n9;
        locals.var_t1_dn10 = assign60590_e94583_d_n10;
        locals.var_t1_dn13 = assign60590_e94583_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60600_e94603, assign60600_e94603_d_n0, assign60600_e94603_d_n2, assign60600_e94603_d_n4, assign60600_e94603_d_n5, assign60600_e94603_d_n6, assign60600_e94603_d_n7, assign60600_e94603_d_n8, assign60600_e94603_d_n9, assign60600_e94603_d_n10, assign60600_e94603_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60600_e94596: f64 = (10.0 * 2.220446049250313e-16);
        let assign60600_e94597: f64 = (locals.var_pds + assign60600_e94596);
        let assign60600_e94599: f64 = (assign60600_e94597 * locals.var_fdd);
        let assign60600_e94601: f64 = (assign60600_e94599 * locals.var_t1);
        (assign60600_e94601, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn0)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn2)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn4)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn5)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn6)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn7)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn8)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn9)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn10)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn10)), ((((locals.var_pds_dn13 * locals.var_fdd) + (assign60600_e94597 * locals.var_fdd_dn13)) * locals.var_t1) + (assign60600_e94599 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign60600_e94603;
        locals.var_t5_dn0 = assign60600_e94603_d_n0;
        locals.var_t5_dn2 = assign60600_e94603_d_n2;
        locals.var_t5_dn4 = assign60600_e94603_d_n4;
        locals.var_t5_dn5 = assign60600_e94603_d_n5;
        locals.var_t5_dn6 = assign60600_e94603_d_n6;
        locals.var_t5_dn7 = assign60600_e94603_d_n7;
        locals.var_t5_dn8 = assign60600_e94603_d_n8;
        locals.var_t5_dn9 = assign60600_e94603_d_n9;
        locals.var_t5_dn10 = assign60600_e94603_d_n10;
        locals.var_t5_dn13 = assign60600_e94603_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign60610_e94617, assign60610_e94617_d_n0, assign60610_e94617_d_n2, assign60610_e94617_d_n4, assign60610_e94617_d_n5, assign60610_e94617_d_n6, assign60610_e94617_d_n7, assign60610_e94617_d_n8, assign60610_e94617_d_n9, assign60610_e94617_d_n10, assign60610_e94617_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60610_e94615: f64 = (locals.var_t5 * locals.var_beta);
        (assign60610_e94615, ((locals.var_t5_dn0 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn0)), ((locals.var_t5_dn2 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn2)), ((locals.var_t5_dn4 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn4)), ((locals.var_t5_dn5 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn5)), ((locals.var_t5_dn6 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn6)), ((locals.var_t5_dn7 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn7)), ((locals.var_t5_dn8 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn8)), ((locals.var_t5_dn9 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn9)), ((locals.var_t5_dn10 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn10)), ((locals.var_t5_dn13 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign60610_e94617;
        locals.var_t2_dn0 = assign60610_e94617_d_n0;
        locals.var_t2_dn2 = assign60610_e94617_d_n2;
        locals.var_t2_dn4 = assign60610_e94617_d_n4;
        locals.var_t2_dn5 = assign60610_e94617_d_n5;
        locals.var_t2_dn6 = assign60610_e94617_d_n6;
        locals.var_t2_dn7 = assign60610_e94617_d_n7;
        locals.var_t2_dn8 = assign60610_e94617_d_n8;
        locals.var_t2_dn9 = assign60610_e94617_d_n9;
        locals.var_t2_dn10 = assign60610_e94617_d_n10;
        locals.var_t2_dn13 = assign60610_e94617_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign60620_e94631, assign60620_e94631_d_n0, assign60620_e94631_d_n2, assign60620_e94631_d_n4, assign60620_e94631_d_n5, assign60620_e94631_d_n6, assign60620_e94631_d_n7, assign60620_e94631_d_n8, assign60620_e94631_d_n9, assign60620_e94631_d_n10, assign60620_e94631_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60620_e94629: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign60620_e94629, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn4 / 1.034943e-10), (locals.var_q_nsub_dn5 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn8 / 1.034943e-10), (locals.var_q_nsub_dn9 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn13 / 1.034943e-10),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign60620_e94631;
        locals.var_t10_dn0 = assign60620_e94631_d_n0;
        locals.var_t10_dn2 = assign60620_e94631_d_n2;
        locals.var_t10_dn4 = assign60620_e94631_d_n4;
        locals.var_t10_dn5 = assign60620_e94631_d_n5;
        locals.var_t10_dn6 = assign60620_e94631_d_n6;
        locals.var_t10_dn7 = assign60620_e94631_d_n7;
        locals.var_t10_dn8 = assign60620_e94631_d_n8;
        locals.var_t10_dn9 = assign60620_e94631_d_n9;
        locals.var_t10_dn10 = assign60620_e94631_d_n10;
        locals.var_t10_dn13 = assign60620_e94631_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign60630_e94643, assign60630_e94643_d_n0, assign60630_e94643_d_n2, assign60630_e94643_d_n4, assign60630_e94643_d_n5, assign60630_e94643_d_n6, assign60630_e94643_d_n7, assign60630_e94643_d_n8, assign60630_e94643_d_n9, assign60630_e94643_d_n10, assign60630_e94643_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60630_e94643;
        locals.var_t1_dn0 = assign60630_e94643_d_n0;
        locals.var_t1_dn2 = assign60630_e94643_d_n2;
        locals.var_t1_dn4 = assign60630_e94643_d_n4;
        locals.var_t1_dn5 = assign60630_e94643_d_n5;
        locals.var_t1_dn6 = assign60630_e94643_d_n6;
        locals.var_t1_dn7 = assign60630_e94643_d_n7;
        locals.var_t1_dn8 = assign60630_e94643_d_n8;
        locals.var_t1_dn9 = assign60630_e94643_d_n9;
        locals.var_t1_dn10 = assign60630_e94643_d_n10;
        locals.var_t1_dn13 = assign60630_e94643_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60640_e94657, assign60640_e94657_d_n0, assign60640_e94657_d_n2, assign60640_e94657_d_n4, assign60640_e94657_d_n5, assign60640_e94657_d_n6, assign60640_e94657_d_n7, assign60640_e94657_d_n8, assign60640_e94657_d_n9, assign60640_e94657_d_n10, assign60640_e94657_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60640_e94655: f64 = (1.0 / locals.var_leff);
        (assign60640_e94655, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign60640_e94657;
        locals.var_t2_dn0 = assign60640_e94657_d_n0;
        locals.var_t2_dn2 = assign60640_e94657_d_n2;
        locals.var_t2_dn4 = assign60640_e94657_d_n4;
        locals.var_t2_dn5 = assign60640_e94657_d_n5;
        locals.var_t2_dn6 = assign60640_e94657_d_n6;
        locals.var_t2_dn7 = assign60640_e94657_d_n7;
        locals.var_t2_dn8 = assign60640_e94657_d_n8;
        locals.var_t2_dn9 = assign60640_e94657_d_n9;
        locals.var_t2_dn10 = assign60640_e94657_d_n10;
        locals.var_t2_dn13 = assign60640_e94657_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign60650_e94685, assign60650_e94685_d_n0, assign60650_e94685_d_n2, assign60650_e94685_d_n4, assign60650_e94685_d_n5, assign60650_e94685_d_n6, assign60650_e94685_d_n7, assign60650_e94685_d_n8, assign60650_e94685_d_n9, assign60650_e94685_d_n10, assign60650_e94685_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60650_e94669: f64 = (2.0 * locals.var_t5);
        let assign60650_e94672: f64 = (2.0 * locals.var_t10);
        let assign60650_e94674: f64 = (assign60650_e94672 * locals.var_t6);
        let assign60650_e94676: f64 = (assign60650_e94674 * locals.var_t4);
        let assign60650_e94677: f64 = (assign60650_e94669 + assign60650_e94676);
        let assign60650_e94680: f64 = (locals.var_t1 * locals.var_t4);
        let assign60650_e94681: f64 = (assign60650_e94677 + assign60650_e94680);
        let assign60650_e94683: f64 = (assign60650_e94681 * locals.var_t2);
        (assign60650_e94683, (((((2.0 * locals.var_t5_dn0) + (((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn0)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn0))) + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn0)), (((((2.0 * locals.var_t5_dn2) + (((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn2)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn2))) + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn2)), (((((2.0 * locals.var_t5_dn4) + (((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn4)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn4))) + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn4)), (((((2.0 * locals.var_t5_dn5) + (((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn5)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn5))) + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn5)), (((((2.0 * locals.var_t5_dn6) + (((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn6)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn6))) + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn6)), (((((2.0 * locals.var_t5_dn7) + (((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn7)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn7))) + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn7)), (((((2.0 * locals.var_t5_dn8) + (((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn8)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn8))) + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn8)), (((((2.0 * locals.var_t5_dn9) + (((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn9)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn9))) + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn9)), (((((2.0 * locals.var_t5_dn10) + (((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn10)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn10))) + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn10)), (((((2.0 * locals.var_t5_dn13) + (((((2.0 * locals.var_t10_dn13) * locals.var_t6) + (assign60650_e94672 * locals.var_t6_dn13)) * locals.var_t4) + (assign60650_e94674 * locals.var_t4_dn13))) + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))) * locals.var_t2) + (assign60650_e94681 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign60650_e94685;
        locals.var_t11_dn0 = assign60650_e94685_d_n0;
        locals.var_t11_dn2 = assign60650_e94685_d_n2;
        locals.var_t11_dn4 = assign60650_e94685_d_n4;
        locals.var_t11_dn5 = assign60650_e94685_d_n5;
        locals.var_t11_dn6 = assign60650_e94685_d_n6;
        locals.var_t11_dn7 = assign60650_e94685_d_n7;
        locals.var_t11_dn8 = assign60650_e94685_d_n8;
        locals.var_t11_dn9 = assign60650_e94685_d_n9;
        locals.var_t11_dn10 = assign60650_e94685_d_n10;
        locals.var_t11_dn13 = assign60650_e94685_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign60660_e94699, assign60660_e94699_d_n0, assign60660_e94699_d_n2, assign60660_e94699_d_n4, assign60660_e94699_d_n5, assign60660_e94699_d_n6, assign60660_e94699_d_n7, assign60660_e94699_d_n8, assign60660_e94699_d_n9, assign60660_e94699_d_n10, assign60660_e94699_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60660_e94697: f64 = (locals.var_t2 * locals.var_t4);
        (assign60660_e94697, ((locals.var_t2_dn0 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn0)), ((locals.var_t2_dn2 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn2)), ((locals.var_t2_dn4 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn4)), ((locals.var_t2_dn5 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn5)), ((locals.var_t2_dn6 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn6)), ((locals.var_t2_dn7 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn7)), ((locals.var_t2_dn8 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn8)), ((locals.var_t2_dn9 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn9)), ((locals.var_t2_dn10 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn10)), ((locals.var_t2_dn13 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign60660_e94699;
        locals.var_t3_dn0 = assign60660_e94699_d_n0;
        locals.var_t3_dn2 = assign60660_e94699_d_n2;
        locals.var_t3_dn4 = assign60660_e94699_d_n4;
        locals.var_t3_dn5 = assign60660_e94699_d_n5;
        locals.var_t3_dn6 = assign60660_e94699_d_n6;
        locals.var_t3_dn7 = assign60660_e94699_d_n7;
        locals.var_t3_dn8 = assign60660_e94699_d_n8;
        locals.var_t3_dn9 = assign60660_e94699_d_n9;
        locals.var_t3_dn10 = assign60660_e94699_d_n10;
        locals.var_t3_dn13 = assign60660_e94699_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign60670_e94713, assign60670_e94713_d_n0, assign60670_e94713_d_n2, assign60670_e94713_d_n4, assign60670_e94713_d_n5, assign60670_e94713_d_n6, assign60670_e94713_d_n7, assign60670_e94713_d_n8, assign60670_e94713_d_n9, assign60670_e94713_d_n10, assign60670_e94713_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60670_e94711: f64 = (locals.var_t11 * locals.var_t4);
        (assign60670_e94711, ((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)), ((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)), ((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)), ((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)), ((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)), ((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)), ((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)), ((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)), ((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)), ((locals.var_t11_dn13 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign60670_e94713;
        locals.var_t7_dn0 = assign60670_e94713_d_n0;
        locals.var_t7_dn2 = assign60670_e94713_d_n2;
        locals.var_t7_dn4 = assign60670_e94713_d_n4;
        locals.var_t7_dn5 = assign60670_e94713_d_n5;
        locals.var_t7_dn6 = assign60670_e94713_d_n6;
        locals.var_t7_dn7 = assign60670_e94713_d_n7;
        locals.var_t7_dn8 = assign60670_e94713_d_n8;
        locals.var_t7_dn9 = assign60670_e94713_d_n9;
        locals.var_t7_dn10 = assign60670_e94713_d_n10;
        locals.var_t7_dn13 = assign60670_e94713_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign60680_e94733, assign60680_e94733_d_n0, assign60680_e94733_d_n2, assign60680_e94733_d_n4, assign60680_e94733_d_n5, assign60680_e94733_d_n6, assign60680_e94733_d_n7, assign60680_e94733_d_n8, assign60680_e94733_d_n9, assign60680_e94733_d_n10, assign60680_e94733_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60680_e94726: f64 = (2.0 * locals.var_t10);
        let assign60680_e94728: f64 = (assign60680_e94726 * locals.var_t6);
        let assign60680_e94730: f64 = (assign60680_e94728 + locals.var_t1);
        let assign60680_e94731: f64 = (4.0 * assign60680_e94730);
        (assign60680_e94731, (4.0 * ((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn0)) + locals.var_t1_dn0)), (4.0 * ((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn2)) + locals.var_t1_dn2)), (4.0 * ((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn4)) + locals.var_t1_dn4)), (4.0 * ((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn5)) + locals.var_t1_dn5)), (4.0 * ((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn6)) + locals.var_t1_dn6)), (4.0 * ((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn7)) + locals.var_t1_dn7)), (4.0 * ((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn8)) + locals.var_t1_dn8)), (4.0 * ((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn9)) + locals.var_t1_dn9)), (4.0 * ((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn10)) + locals.var_t1_dn10)), (4.0 * ((((2.0 * locals.var_t10_dn13) * locals.var_t6) + (assign60680_e94726 * locals.var_t6_dn13)) + locals.var_t1_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign60680_e94733;
        locals.var_t11_dn0 = assign60680_e94733_d_n0;
        locals.var_t11_dn2 = assign60680_e94733_d_n2;
        locals.var_t11_dn4 = assign60680_e94733_d_n4;
        locals.var_t11_dn5 = assign60680_e94733_d_n5;
        locals.var_t11_dn6 = assign60680_e94733_d_n6;
        locals.var_t11_dn7 = assign60680_e94733_d_n7;
        locals.var_t11_dn8 = assign60680_e94733_d_n8;
        locals.var_t11_dn9 = assign60680_e94733_d_n9;
        locals.var_t11_dn10 = assign60680_e94733_d_n10;
        locals.var_t11_dn13 = assign60680_e94733_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign60690_e94751, assign60690_e94751_d_n0, assign60690_e94751_d_n2, assign60690_e94751_d_n4, assign60690_e94751_d_n5, assign60690_e94751_d_n6, assign60690_e94751_d_n7, assign60690_e94751_d_n8, assign60690_e94751_d_n9, assign60690_e94751_d_n10, assign60690_e94751_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60690_e94745: f64 = (8.0 * locals.var_t10);
        let assign60690_e94747: f64 = (assign60690_e94745 * locals.var_t4);
        let assign60690_e94749: f64 = (assign60690_e94747 * locals.var_t4);
        (assign60690_e94749, (((((8.0 * locals.var_t10_dn0) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn0)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn0)), (((((8.0 * locals.var_t10_dn2) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn2)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn2)), (((((8.0 * locals.var_t10_dn4) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn4)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn4)), (((((8.0 * locals.var_t10_dn5) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn5)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn5)), (((((8.0 * locals.var_t10_dn6) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn6)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn6)), (((((8.0 * locals.var_t10_dn7) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn7)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn7)), (((((8.0 * locals.var_t10_dn8) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn8)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn8)), (((((8.0 * locals.var_t10_dn9) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn9)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn9)), (((((8.0 * locals.var_t10_dn10) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn10)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn10)), (((((8.0 * locals.var_t10_dn13) * locals.var_t4) + (assign60690_e94745 * locals.var_t4_dn13)) * locals.var_t4) + (assign60690_e94747 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign60690_e94751;
        locals.var_t1_dn0 = assign60690_e94751_d_n0;
        locals.var_t1_dn2 = assign60690_e94751_d_n2;
        locals.var_t1_dn4 = assign60690_e94751_d_n4;
        locals.var_t1_dn5 = assign60690_e94751_d_n5;
        locals.var_t1_dn6 = assign60690_e94751_d_n6;
        locals.var_t1_dn7 = assign60690_e94751_d_n7;
        locals.var_t1_dn8 = assign60690_e94751_d_n8;
        locals.var_t1_dn9 = assign60690_e94751_d_n9;
        locals.var_t1_dn10 = assign60690_e94751_d_n10;
        locals.var_t1_dn13 = assign60690_e94751_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign60700_e94767, assign60700_e94767_d_n0, assign60700_e94767_d_n2, assign60700_e94767_d_n4, assign60700_e94767_d_n5, assign60700_e94767_d_n6, assign60700_e94767_d_n7, assign60700_e94767_d_n8, assign60700_e94767_d_n9, assign60700_e94767_d_n10, assign60700_e94767_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign60700_e94763: f64 = (2.0 * locals.var_t11);
        let assign60700_e94765: f64 = (assign60700_e94763 * locals.var_t4);
        (assign60700_e94765, (((2.0 * locals.var_t11_dn0) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn0)), (((2.0 * locals.var_t11_dn2) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn2)), (((2.0 * locals.var_t11_dn4) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn4)), (((2.0 * locals.var_t11_dn5) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn5)), (((2.0 * locals.var_t11_dn6) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn6)), (((2.0 * locals.var_t11_dn7) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn7)), (((2.0 * locals.var_t11_dn8) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn8)), (((2.0 * locals.var_t11_dn9) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn9)), (((2.0 * locals.var_t11_dn10) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn10)), (((2.0 * locals.var_t11_dn13) * locals.var_t4) + (assign60700_e94763 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign60700_e94767;
        locals.var_t2_dn0 = assign60700_e94767_d_n0;
        locals.var_t2_dn2 = assign60700_e94767_d_n2;
        locals.var_t2_dn4 = assign60700_e94767_d_n4;
        locals.var_t2_dn5 = assign60700_e94767_d_n5;
        locals.var_t2_dn6 = assign60700_e94767_d_n6;
        locals.var_t2_dn7 = assign60700_e94767_d_n7;
        locals.var_t2_dn8 = assign60700_e94767_d_n8;
        locals.var_t2_dn9 = assign60700_e94767_d_n9;
        locals.var_t2_dn10 = assign60700_e94767_d_n10;
        locals.var_t2_dn13 = assign60700_e94767_d_n13;
        locals.var_t2_rv = 0.0;

    }
}
