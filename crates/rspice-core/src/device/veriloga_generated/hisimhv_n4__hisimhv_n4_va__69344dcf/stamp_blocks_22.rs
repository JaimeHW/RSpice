#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_352(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign99110_e151412: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99110_e151412;

        let assign99120_e151415: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99120_e151415;

        let (assign99130_e151427, assign99130_e151427_d_n0, assign99130_e151427_d_n2, assign99130_e151427_d_n4, assign99130_e151427_d_n5, assign99130_e151427_d_n6, assign99130_e151427_d_n7, assign99130_e151427_d_n8, assign99130_e151427_d_n9, assign99130_e151427_d_n10, assign99130_e151427_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99130_e151424: f64 = (locals.var_vbsi_jct / locals.var_pzbsswg);
        let assign99130_e151425: f64 = (1.0 - assign99130_e151424);
        (assign99130_e151425, (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn2) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbsi_jct_dn7 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(((locals.var_vbsi_jct_dn8 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn13) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign99130_e151427;
        locals.var_arg_dn0 = assign99130_e151427_d_n0;
        locals.var_arg_dn2 = assign99130_e151427_d_n2;
        locals.var_arg_dn4 = assign99130_e151427_d_n4;
        locals.var_arg_dn5 = assign99130_e151427_d_n5;
        locals.var_arg_dn6 = assign99130_e151427_d_n6;
        locals.var_arg_dn7 = assign99130_e151427_d_n7;
        locals.var_arg_dn8 = assign99130_e151427_d_n8;
        locals.var_arg_dn9 = assign99130_e151427_d_n9;
        locals.var_arg_dn10 = assign99130_e151427_d_n10;
        locals.var_arg_dn13 = assign99130_e151427_d_n13;

        let assign99140_e151430: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99140_e151430;

        let (assign99150_e151443, assign99150_e151443_d_n0, assign99150_e151443_d_n2, assign99150_e151443_d_n4, assign99150_e151443_d_n5, assign99150_e151443_d_n6, assign99150_e151443_d_n7, assign99150_e151443_d_n8, assign99150_e151443_d_n9, assign99150_e151443_d_n10, assign99150_e151443_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 != 0.0)) {
        let assign99150_e151440: f64 = (locals.var_arg).sqrt();
        let assign99150_e151441: f64 = (1.0 / assign99150_e151440);
        (assign99150_e151441, (-((locals.var_arg_dn0 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn2 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn4 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn5 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn6 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn7 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn8 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn9 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn10 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn13 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99150_e151443;
        locals.var_sarg_dn0 = assign99150_e151443_d_n0;
        locals.var_sarg_dn2 = assign99150_e151443_d_n2;
        locals.var_sarg_dn4 = assign99150_e151443_d_n4;
        locals.var_sarg_dn5 = assign99150_e151443_d_n5;
        locals.var_sarg_dn6 = assign99150_e151443_d_n6;
        locals.var_sarg_dn7 = assign99150_e151443_d_n7;
        locals.var_sarg_dn8 = assign99150_e151443_d_n8;
        locals.var_sarg_dn9 = assign99150_e151443_d_n9;
        locals.var_sarg_dn10 = assign99150_e151443_d_n10;
        locals.var_sarg_dn13 = assign99150_e151443_d_n13;

        let (assign99160_e151462, assign99160_e151462_d_n0, assign99160_e151462_d_n2, assign99160_e151462_d_n4, assign99160_e151462_d_n5, assign99160_e151462_d_n6, assign99160_e151462_d_n7, assign99160_e151462_d_n8, assign99160_e151462_d_n9, assign99160_e151462_d_n10, assign99160_e151462_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 == 0.0)) {
        let (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99160_e151458: f64 = (-p.p528);
                let assign99160_e151459: f64 = (locals.var_arg).powf(assign99160_e151458);
                (assign99160_e151459, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn0)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn2)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn4)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn5)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn6)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn7)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn8)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn9)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn10)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn13)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99160_e151462;
        locals.var_sarg_dn0 = assign99160_e151462_d_n0;
        locals.var_sarg_dn2 = assign99160_e151462_d_n2;
        locals.var_sarg_dn4 = assign99160_e151462_d_n4;
        locals.var_sarg_dn5 = assign99160_e151462_d_n5;
        locals.var_sarg_dn6 = assign99160_e151462_d_n6;
        locals.var_sarg_dn7 = assign99160_e151462_d_n7;
        locals.var_sarg_dn8 = assign99160_e151462_d_n8;
        locals.var_sarg_dn9 = assign99160_e151462_d_n9;
        locals.var_sarg_dn10 = assign99160_e151462_d_n10;
        locals.var_sarg_dn13 = assign99160_e151462_d_n13;

        let (assign99170_e151482, assign99170_e151482_d_n0, assign99170_e151482_d_n2, assign99170_e151482_d_n4, assign99170_e151482_d_n5, assign99170_e151482_d_n6, assign99170_e151482_d_n7, assign99170_e151482_d_n8, assign99170_e151482_d_n9, assign99170_e151482_d_n10, assign99170_e151482_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99170_e151470: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99170_e151474: f64 = (locals.var_arg * locals.var_sarg);
        let assign99170_e151475: f64 = (1.0 - assign99170_e151474);
        let assign99170_e151476: f64 = (assign99170_e151470 * assign99170_e151475);
        let assign99170_e151479: f64 = (1.0 - p.p528);
        let assign99170_e151480: f64 = (assign99170_e151476 / assign99170_e151479);
        (assign99170_e151480, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn13 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn13)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99170_e151479),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99170_e151482;
        locals.var_qbs_swg_dn0 = assign99170_e151482_d_n0;
        locals.var_qbs_swg_dn2 = assign99170_e151482_d_n2;
        locals.var_qbs_swg_dn4 = assign99170_e151482_d_n4;
        locals.var_qbs_swg_dn5 = assign99170_e151482_d_n5;
        locals.var_qbs_swg_dn6 = assign99170_e151482_d_n6;
        locals.var_qbs_swg_dn7 = assign99170_e151482_d_n7;
        locals.var_qbs_swg_dn8 = assign99170_e151482_d_n8;
        locals.var_qbs_swg_dn9 = assign99170_e151482_d_n9;
        locals.var_qbs_swg_dn10 = assign99170_e151482_d_n10;
        locals.var_qbs_swg_dn13 = assign99170_e151482_d_n13;

        let (assign99190_e151501, assign99190_e151501_d_n0, assign99190_e151501_d_n2, assign99190_e151501_d_n4, assign99190_e151501_d_n5, assign99190_e151501_d_n6, assign99190_e151501_d_n7, assign99190_e151501_d_n8, assign99190_e151501_d_n9, assign99190_e151501_d_n10, assign99190_e151501_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99190_e151501;
        locals.var_t1_dn0 = assign99190_e151501_d_n0;
        locals.var_t1_dn2 = assign99190_e151501_d_n2;
        locals.var_t1_dn4 = assign99190_e151501_d_n4;
        locals.var_t1_dn5 = assign99190_e151501_d_n5;
        locals.var_t1_dn6 = assign99190_e151501_d_n6;
        locals.var_t1_dn7 = assign99190_e151501_d_n7;
        locals.var_t1_dn8 = assign99190_e151501_d_n8;
        locals.var_t1_dn9 = assign99190_e151501_d_n9;
        locals.var_t1_dn10 = assign99190_e151501_d_n10;
        locals.var_t1_dn13 = assign99190_e151501_d_n13;

        let (assign99200_e151514, assign99200_e151514_d_n0, assign99200_e151514_d_n2, assign99200_e151514_d_n4, assign99200_e151514_d_n5, assign99200_e151514_d_n6, assign99200_e151514_d_n7, assign99200_e151514_d_n8, assign99200_e151514_d_n9, assign99200_e151514_d_n10, assign99200_e151514_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        let assign99200_e151510: f64 = (locals.var_czbsswg * p.p528);
        let assign99200_e151512: f64 = (assign99200_e151510 / locals.var_pzbsswg);
        (assign99200_e151512, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn13 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn13)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99200_e151514;
        locals.var_t2_dn0 = assign99200_e151514_d_n0;
        locals.var_t2_dn2 = assign99200_e151514_d_n2;
        locals.var_t2_dn4 = assign99200_e151514_d_n4;
        locals.var_t2_dn5 = assign99200_e151514_d_n5;
        locals.var_t2_dn6 = assign99200_e151514_d_n6;
        locals.var_t2_dn7 = assign99200_e151514_d_n7;
        locals.var_t2_dn8 = assign99200_e151514_d_n8;
        locals.var_t2_dn9 = assign99200_e151514_d_n9;
        locals.var_t2_dn10 = assign99200_e151514_d_n10;
        locals.var_t2_dn13 = assign99200_e151514_d_n13;

        let (assign99210_e151531, assign99210_e151531_d_n0, assign99210_e151531_d_n2, assign99210_e151531_d_n4, assign99210_e151531_d_n5, assign99210_e151531_d_n6, assign99210_e151531_d_n7, assign99210_e151531_d_n8, assign99210_e151531_d_n9, assign99210_e151531_d_n10, assign99210_e151531_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        let assign99210_e151525: f64 = (locals.var_vbsi_jct * 0.5);
        let assign99210_e151527: f64 = (assign99210_e151525 * locals.var_t2);
        let assign99210_e151528: f64 = (locals.var_t1 + assign99210_e151527);
        let assign99210_e151529: f64 = (locals.var_vbsi_jct * assign99210_e151528);
        (assign99210_e151529, (locals.var_vbsi_jct * (locals.var_t1_dn0 + (assign99210_e151525 * locals.var_t2_dn0))), (locals.var_vbsi_jct * (locals.var_t1_dn2 + (assign99210_e151525 * locals.var_t2_dn2))), (locals.var_vbsi_jct * (locals.var_t1_dn4 + (assign99210_e151525 * locals.var_t2_dn4))), (locals.var_vbsi_jct * (locals.var_t1_dn5 + (assign99210_e151525 * locals.var_t2_dn5))), (locals.var_vbsi_jct * (locals.var_t1_dn6 + (assign99210_e151525 * locals.var_t2_dn6))), ((locals.var_vbsi_jct_dn7 * assign99210_e151528) + (locals.var_vbsi_jct * (locals.var_t1_dn7 + (((locals.var_vbsi_jct_dn7 * 0.5) * locals.var_t2) + (assign99210_e151525 * locals.var_t2_dn7))))), ((locals.var_vbsi_jct_dn8 * assign99210_e151528) + (locals.var_vbsi_jct * (locals.var_t1_dn8 + (((locals.var_vbsi_jct_dn8 * 0.5) * locals.var_t2) + (assign99210_e151525 * locals.var_t2_dn8))))), (locals.var_vbsi_jct * (locals.var_t1_dn9 + (assign99210_e151525 * locals.var_t2_dn9))), (locals.var_vbsi_jct * (locals.var_t1_dn10 + (assign99210_e151525 * locals.var_t2_dn10))), (locals.var_vbsi_jct * (locals.var_t1_dn13 + (assign99210_e151525 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99210_e151531;
        locals.var_qbs_swg_dn0 = assign99210_e151531_d_n0;
        locals.var_qbs_swg_dn2 = assign99210_e151531_d_n2;
        locals.var_qbs_swg_dn4 = assign99210_e151531_d_n4;
        locals.var_qbs_swg_dn5 = assign99210_e151531_d_n5;
        locals.var_qbs_swg_dn6 = assign99210_e151531_d_n6;
        locals.var_qbs_swg_dn7 = assign99210_e151531_d_n7;
        locals.var_qbs_swg_dn8 = assign99210_e151531_d_n8;
        locals.var_qbs_swg_dn9 = assign99210_e151531_d_n9;
        locals.var_qbs_swg_dn10 = assign99210_e151531_d_n10;
        locals.var_qbs_swg_dn13 = assign99210_e151531_d_n13;

        let (assign99230_e151551, assign99230_e151551_d_n0, assign99230_e151551_d_n2, assign99230_e151551_d_n4, assign99230_e151551_d_n5, assign99230_e151551_d_n6, assign99230_e151551_d_n7, assign99230_e151551_d_n8, assign99230_e151551_d_n9, assign99230_e151551_d_n10, assign99230_e151551_d_n13,) = {
    if ((locals.var_guard2293 != 0.0) && (locals.var_guard2294 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99230_e151551;
        locals.var_qbs_swg_dn0 = assign99230_e151551_d_n0;
        locals.var_qbs_swg_dn2 = assign99230_e151551_d_n2;
        locals.var_qbs_swg_dn4 = assign99230_e151551_d_n4;
        locals.var_qbs_swg_dn5 = assign99230_e151551_d_n5;
        locals.var_qbs_swg_dn6 = assign99230_e151551_d_n6;
        locals.var_qbs_swg_dn7 = assign99230_e151551_d_n7;
        locals.var_qbs_swg_dn8 = assign99230_e151551_d_n8;
        locals.var_qbs_swg_dn9 = assign99230_e151551_d_n9;
        locals.var_qbs_swg_dn10 = assign99230_e151551_d_n10;
        locals.var_qbs_swg_dn13 = assign99230_e151551_d_n13;

        let assign99250_e151561: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99250_e151561;

        let assign99260_e151564: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99260_e151564;

        let (assign99270_e151577, assign99270_e151577_d_n0, assign99270_e151577_d_n2, assign99270_e151577_d_n4, assign99270_e151577_d_n5, assign99270_e151577_d_n6, assign99270_e151577_d_n7, assign99270_e151577_d_n8, assign99270_e151577_d_n9, assign99270_e151577_d_n10, assign99270_e151577_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) {
        let assign99270_e151574: f64 = (locals.var_vbs_jct / locals.var_pzbsswg);
        let assign99270_e151575: f64 = (1.0 - assign99270_e151574);
        (assign99270_e151575, (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn8) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn13) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign99270_e151577;
        locals.var_arg_dn0 = assign99270_e151577_d_n0;
        locals.var_arg_dn2 = assign99270_e151577_d_n2;
        locals.var_arg_dn4 = assign99270_e151577_d_n4;
        locals.var_arg_dn5 = assign99270_e151577_d_n5;
        locals.var_arg_dn6 = assign99270_e151577_d_n6;
        locals.var_arg_dn7 = assign99270_e151577_d_n7;
        locals.var_arg_dn8 = assign99270_e151577_d_n8;
        locals.var_arg_dn9 = assign99270_e151577_d_n9;
        locals.var_arg_dn10 = assign99270_e151577_d_n10;
        locals.var_arg_dn13 = assign99270_e151577_d_n13;

        let assign99280_e151580: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99280_e151580;

        let (assign99290_e151594, assign99290_e151594_d_n0, assign99290_e151594_d_n2, assign99290_e151594_d_n4, assign99290_e151594_d_n5, assign99290_e151594_d_n6, assign99290_e151594_d_n7, assign99290_e151594_d_n8, assign99290_e151594_d_n9, assign99290_e151594_d_n10, assign99290_e151594_d_n13,) = {
    if ((((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) {
        let assign99290_e151591: f64 = (locals.var_arg).sqrt();
        let assign99290_e151592: f64 = (1.0 / assign99290_e151591);
        (assign99290_e151592, (-((locals.var_arg_dn0 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn2 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn4 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn5 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn6 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn7 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn8 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn9 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn10 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn13 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99290_e151594;
        locals.var_sarg_dn0 = assign99290_e151594_d_n0;
        locals.var_sarg_dn2 = assign99290_e151594_d_n2;
        locals.var_sarg_dn4 = assign99290_e151594_d_n4;
        locals.var_sarg_dn5 = assign99290_e151594_d_n5;
        locals.var_sarg_dn6 = assign99290_e151594_d_n6;
        locals.var_sarg_dn7 = assign99290_e151594_d_n7;
        locals.var_sarg_dn8 = assign99290_e151594_d_n8;
        locals.var_sarg_dn9 = assign99290_e151594_d_n9;
        locals.var_sarg_dn10 = assign99290_e151594_d_n10;
        locals.var_sarg_dn13 = assign99290_e151594_d_n13;

        let (assign99300_e151614, assign99300_e151614_d_n0, assign99300_e151614_d_n2, assign99300_e151614_d_n4, assign99300_e151614_d_n5, assign99300_e151614_d_n6, assign99300_e151614_d_n7, assign99300_e151614_d_n8, assign99300_e151614_d_n9, assign99300_e151614_d_n10, assign99300_e151614_d_n13,) = {
    if ((((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 == 0.0)) {
        let (assign99300_e151612, assign99300_e151612_d_n0, assign99300_e151612_d_n2, assign99300_e151612_d_n4, assign99300_e151612_d_n5, assign99300_e151612_d_n6, assign99300_e151612_d_n7, assign99300_e151612_d_n8, assign99300_e151612_d_n9, assign99300_e151612_d_n10, assign99300_e151612_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99300_e151610: f64 = (-p.p528);
                let assign99300_e151611: f64 = (locals.var_arg).powf(assign99300_e151610);
                (assign99300_e151611, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn0)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn2)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn4)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn5)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn6)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn7)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn8)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn9)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn10)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn13)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99300_e151612, assign99300_e151612_d_n0, assign99300_e151612_d_n2, assign99300_e151612_d_n4, assign99300_e151612_d_n5, assign99300_e151612_d_n6, assign99300_e151612_d_n7, assign99300_e151612_d_n8, assign99300_e151612_d_n9, assign99300_e151612_d_n10, assign99300_e151612_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99300_e151614;
        locals.var_sarg_dn0 = assign99300_e151614_d_n0;
        locals.var_sarg_dn2 = assign99300_e151614_d_n2;
        locals.var_sarg_dn4 = assign99300_e151614_d_n4;
        locals.var_sarg_dn5 = assign99300_e151614_d_n5;
        locals.var_sarg_dn6 = assign99300_e151614_d_n6;
        locals.var_sarg_dn7 = assign99300_e151614_d_n7;
        locals.var_sarg_dn8 = assign99300_e151614_d_n8;
        locals.var_sarg_dn9 = assign99300_e151614_d_n9;
        locals.var_sarg_dn10 = assign99300_e151614_d_n10;
        locals.var_sarg_dn13 = assign99300_e151614_d_n13;

        let (assign99310_e151635, assign99310_e151635_d_n0, assign99310_e151635_d_n2, assign99310_e151635_d_n4, assign99310_e151635_d_n5, assign99310_e151635_d_n6, assign99310_e151635_d_n7, assign99310_e151635_d_n8, assign99310_e151635_d_n9, assign99310_e151635_d_n10, assign99310_e151635_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) {
        let assign99310_e151623: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99310_e151627: f64 = (locals.var_arg * locals.var_sarg);
        let assign99310_e151628: f64 = (1.0 - assign99310_e151627);
        let assign99310_e151629: f64 = (assign99310_e151623 * assign99310_e151628);
        let assign99310_e151632: f64 = (1.0 - p.p528);
        let assign99310_e151633: f64 = (assign99310_e151629 / assign99310_e151632);
        (assign99310_e151633, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn13 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn13)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99310_e151632),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99310_e151635;
        locals.var_qbs_swg_dn0 = assign99310_e151635_d_n0;
        locals.var_qbs_swg_dn2 = assign99310_e151635_d_n2;
        locals.var_qbs_swg_dn4 = assign99310_e151635_d_n4;
        locals.var_qbs_swg_dn5 = assign99310_e151635_d_n5;
        locals.var_qbs_swg_dn6 = assign99310_e151635_d_n6;
        locals.var_qbs_swg_dn7 = assign99310_e151635_d_n7;
        locals.var_qbs_swg_dn8 = assign99310_e151635_d_n8;
        locals.var_qbs_swg_dn9 = assign99310_e151635_d_n9;
        locals.var_qbs_swg_dn10 = assign99310_e151635_d_n10;
        locals.var_qbs_swg_dn13 = assign99310_e151635_d_n13;

        let (assign99330_e151656, assign99330_e151656_d_n0, assign99330_e151656_d_n2, assign99330_e151656_d_n4, assign99330_e151656_d_n5, assign99330_e151656_d_n6, assign99330_e151656_d_n7, assign99330_e151656_d_n8, assign99330_e151656_d_n9, assign99330_e151656_d_n10, assign99330_e151656_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99330_e151656;
        locals.var_t1_dn0 = assign99330_e151656_d_n0;
        locals.var_t1_dn2 = assign99330_e151656_d_n2;
        locals.var_t1_dn4 = assign99330_e151656_d_n4;
        locals.var_t1_dn5 = assign99330_e151656_d_n5;
        locals.var_t1_dn6 = assign99330_e151656_d_n6;
        locals.var_t1_dn7 = assign99330_e151656_d_n7;
        locals.var_t1_dn8 = assign99330_e151656_d_n8;
        locals.var_t1_dn9 = assign99330_e151656_d_n9;
        locals.var_t1_dn10 = assign99330_e151656_d_n10;
        locals.var_t1_dn13 = assign99330_e151656_d_n13;

        let (assign99340_e151670, assign99340_e151670_d_n0, assign99340_e151670_d_n2, assign99340_e151670_d_n4, assign99340_e151670_d_n5, assign99340_e151670_d_n6, assign99340_e151670_d_n7, assign99340_e151670_d_n8, assign99340_e151670_d_n9, assign99340_e151670_d_n10, assign99340_e151670_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        let assign99340_e151666: f64 = (locals.var_czbsswg * p.p528);
        let assign99340_e151668: f64 = (assign99340_e151666 / locals.var_pzbsswg);
        (assign99340_e151668, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn13 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn13)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99340_e151670;
        locals.var_t2_dn0 = assign99340_e151670_d_n0;
        locals.var_t2_dn2 = assign99340_e151670_d_n2;
        locals.var_t2_dn4 = assign99340_e151670_d_n4;
        locals.var_t2_dn5 = assign99340_e151670_d_n5;
        locals.var_t2_dn6 = assign99340_e151670_d_n6;
        locals.var_t2_dn7 = assign99340_e151670_d_n7;
        locals.var_t2_dn8 = assign99340_e151670_d_n8;
        locals.var_t2_dn9 = assign99340_e151670_d_n9;
        locals.var_t2_dn10 = assign99340_e151670_d_n10;
        locals.var_t2_dn13 = assign99340_e151670_d_n13;

        let (assign99350_e151688, assign99350_e151688_d_n0, assign99350_e151688_d_n2, assign99350_e151688_d_n4, assign99350_e151688_d_n5, assign99350_e151688_d_n6, assign99350_e151688_d_n7, assign99350_e151688_d_n8, assign99350_e151688_d_n9, assign99350_e151688_d_n10, assign99350_e151688_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        let assign99350_e151682: f64 = (locals.var_vbs_jct * 0.5);
        let assign99350_e151684: f64 = (assign99350_e151682 * locals.var_t2);
        let assign99350_e151685: f64 = (locals.var_t1 + assign99350_e151684);
        let assign99350_e151686: f64 = (locals.var_vbs_jct * assign99350_e151685);
        (assign99350_e151686, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99350_e151682 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99350_e151685) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99350_e151682 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99350_e151682 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99350_e151682 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99350_e151682 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99350_e151682 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99350_e151682 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99350_e151682 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign99350_e151685) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign99350_e151682 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign99350_e151682 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99350_e151688;
        locals.var_qbs_swg_dn0 = assign99350_e151688_d_n0;
        locals.var_qbs_swg_dn2 = assign99350_e151688_d_n2;
        locals.var_qbs_swg_dn4 = assign99350_e151688_d_n4;
        locals.var_qbs_swg_dn5 = assign99350_e151688_d_n5;
        locals.var_qbs_swg_dn6 = assign99350_e151688_d_n6;
        locals.var_qbs_swg_dn7 = assign99350_e151688_d_n7;
        locals.var_qbs_swg_dn8 = assign99350_e151688_d_n8;
        locals.var_qbs_swg_dn9 = assign99350_e151688_d_n9;
        locals.var_qbs_swg_dn10 = assign99350_e151688_d_n10;
        locals.var_qbs_swg_dn13 = assign99350_e151688_d_n13;

        let (assign99370_e151710, assign99370_e151710_d_n0, assign99370_e151710_d_n2, assign99370_e151710_d_n4, assign99370_e151710_d_n5, assign99370_e151710_d_n6, assign99370_e151710_d_n7, assign99370_e151710_d_n8, assign99370_e151710_d_n9, assign99370_e151710_d_n10, assign99370_e151710_d_n13,) = {
    if ((locals.var_guard2293 == 0.0) && (locals.var_guard2297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99370_e151710;
        locals.var_qbs_swg_dn0 = assign99370_e151710_d_n0;
        locals.var_qbs_swg_dn2 = assign99370_e151710_d_n2;
        locals.var_qbs_swg_dn4 = assign99370_e151710_d_n4;
        locals.var_qbs_swg_dn5 = assign99370_e151710_d_n5;
        locals.var_qbs_swg_dn6 = assign99370_e151710_d_n6;
        locals.var_qbs_swg_dn7 = assign99370_e151710_d_n7;
        locals.var_qbs_swg_dn8 = assign99370_e151710_d_n8;
        locals.var_qbs_swg_dn9 = assign99370_e151710_d_n9;
        locals.var_qbs_swg_dn10 = assign99370_e151710_d_n10;
        locals.var_qbs_swg_dn13 = assign99370_e151710_d_n13;

        let assign99390_e151722: f64 = (locals.var_ibs_btm + locals.var_ibs_sws);
        let assign99390_e151723: f64 = (locals.var_mfactor * assign99390_e151722);
        locals.var_ibs = assign99390_e151723;
        locals.var_ibs_dn0 = (locals.var_mfactor * (locals.var_ibs_btm_dn0 + locals.var_ibs_sws_dn0));
        locals.var_ibs_dn2 = (locals.var_mfactor * (locals.var_ibs_btm_dn2 + locals.var_ibs_sws_dn2));
        locals.var_ibs_dn4 = (locals.var_mfactor * (locals.var_ibs_btm_dn4 + locals.var_ibs_sws_dn4));
        locals.var_ibs_dn5 = (locals.var_mfactor * (locals.var_ibs_btm_dn5 + locals.var_ibs_sws_dn5));
        locals.var_ibs_dn6 = (locals.var_mfactor * (locals.var_ibs_btm_dn6 + locals.var_ibs_sws_dn6));
        locals.var_ibs_dn7 = (locals.var_mfactor * (locals.var_ibs_btm_dn7 + locals.var_ibs_sws_dn7));
        locals.var_ibs_dn8 = (locals.var_mfactor * (locals.var_ibs_btm_dn8 + locals.var_ibs_sws_dn8));
        locals.var_ibs_dn9 = (locals.var_mfactor * (locals.var_ibs_btm_dn9 + locals.var_ibs_sws_dn9));
        locals.var_ibs_dn10 = (locals.var_mfactor * (locals.var_ibs_btm_dn10 + locals.var_ibs_sws_dn10));
        locals.var_ibs_dn13 = (locals.var_mfactor * (locals.var_ibs_btm_dn13 + locals.var_ibs_sws_dn13));

        let assign99400_e151727: f64 = (locals.var_ibd_btm + locals.var_ibd_sws);
        let assign99400_e151728: f64 = (locals.var_mfactor * assign99400_e151727);
        locals.var_ibd = assign99400_e151728;
        locals.var_ibd_dn0 = (locals.var_mfactor * (locals.var_ibd_btm_dn0 + locals.var_ibd_sws_dn0));
        locals.var_ibd_dn2 = (locals.var_mfactor * (locals.var_ibd_btm_dn2 + locals.var_ibd_sws_dn2));
        locals.var_ibd_dn4 = (locals.var_mfactor * (locals.var_ibd_btm_dn4 + locals.var_ibd_sws_dn4));
        locals.var_ibd_dn5 = (locals.var_mfactor * (locals.var_ibd_btm_dn5 + locals.var_ibd_sws_dn5));
        locals.var_ibd_dn6 = (locals.var_mfactor * (locals.var_ibd_btm_dn6 + locals.var_ibd_sws_dn6));
        locals.var_ibd_dn7 = (locals.var_mfactor * (locals.var_ibd_btm_dn7 + locals.var_ibd_sws_dn7));
        locals.var_ibd_dn8 = (locals.var_mfactor * (locals.var_ibd_btm_dn8 + locals.var_ibd_sws_dn8));
        locals.var_ibd_dn9 = (locals.var_mfactor * (locals.var_ibd_btm_dn9 + locals.var_ibd_sws_dn9));
        locals.var_ibd_dn10 = (locals.var_mfactor * (locals.var_ibd_btm_dn10 + locals.var_ibd_sws_dn10));
        locals.var_ibd_dn13 = (locals.var_mfactor * (locals.var_ibd_btm_dn13 + locals.var_ibd_sws_dn13));

        let assign99410_e151731: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99410_e151731;

        let (assign99420_e151737, assign99420_e151737_d_n0, assign99420_e151737_d_n2, assign99420_e151737_d_n4, assign99420_e151737_d_n5, assign99420_e151737_d_n6, assign99420_e151737_d_n7, assign99420_e151737_d_n8, assign99420_e151737_d_n9, assign99420_e151737_d_n10, assign99420_e151737_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99420_e151735: f64 = (locals.var_mfactor * locals.var_ibs_swg);
        (assign99420_e151735, (locals.var_mfactor * locals.var_ibs_swg_dn0), (locals.var_mfactor * locals.var_ibs_swg_dn2), (locals.var_mfactor * locals.var_ibs_swg_dn4), (locals.var_mfactor * locals.var_ibs_swg_dn5), (locals.var_mfactor * locals.var_ibs_swg_dn6), (locals.var_mfactor * locals.var_ibs_swg_dn7), (locals.var_mfactor * locals.var_ibs_swg_dn8), (locals.var_mfactor * locals.var_ibs_swg_dn9), (locals.var_mfactor * locals.var_ibs_swg_dn10), (locals.var_mfactor * locals.var_ibs_swg_dn13),)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn13,)
    }
};
        locals.var_ibsi = assign99420_e151737;
        locals.var_ibsi_dn0 = assign99420_e151737_d_n0;
        locals.var_ibsi_dn2 = assign99420_e151737_d_n2;
        locals.var_ibsi_dn4 = assign99420_e151737_d_n4;
        locals.var_ibsi_dn5 = assign99420_e151737_d_n5;
        locals.var_ibsi_dn6 = assign99420_e151737_d_n6;
        locals.var_ibsi_dn7 = assign99420_e151737_d_n7;
        locals.var_ibsi_dn8 = assign99420_e151737_d_n8;
        locals.var_ibsi_dn9 = assign99420_e151737_d_n9;
        locals.var_ibsi_dn10 = assign99420_e151737_d_n10;
        locals.var_ibsi_dn13 = assign99420_e151737_d_n13;

        let (assign99430_e151743, assign99430_e151743_d_n0, assign99430_e151743_d_n2, assign99430_e151743_d_n4, assign99430_e151743_d_n5, assign99430_e151743_d_n6, assign99430_e151743_d_n7, assign99430_e151743_d_n8, assign99430_e151743_d_n9, assign99430_e151743_d_n10, assign99430_e151743_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99430_e151741: f64 = (locals.var_mfactor * locals.var_ibd_swg);
        (assign99430_e151741, (locals.var_mfactor * locals.var_ibd_swg_dn0), (locals.var_mfactor * locals.var_ibd_swg_dn2), (locals.var_mfactor * locals.var_ibd_swg_dn4), (locals.var_mfactor * locals.var_ibd_swg_dn5), (locals.var_mfactor * locals.var_ibd_swg_dn6), (locals.var_mfactor * locals.var_ibd_swg_dn7), (locals.var_mfactor * locals.var_ibd_swg_dn8), (locals.var_mfactor * locals.var_ibd_swg_dn9), (locals.var_mfactor * locals.var_ibd_swg_dn10), (locals.var_mfactor * locals.var_ibd_swg_dn13),)
    } else {
        (locals.var_ibdi, locals.var_ibdi_dn0, locals.var_ibdi_dn2, locals.var_ibdi_dn4, locals.var_ibdi_dn5, locals.var_ibdi_dn6, locals.var_ibdi_dn7, locals.var_ibdi_dn8, locals.var_ibdi_dn9, locals.var_ibdi_dn10, locals.var_ibdi_dn13,)
    }
};
        locals.var_ibdi = assign99430_e151743;
        locals.var_ibdi_dn0 = assign99430_e151743_d_n0;
        locals.var_ibdi_dn2 = assign99430_e151743_d_n2;
        locals.var_ibdi_dn4 = assign99430_e151743_d_n4;
        locals.var_ibdi_dn5 = assign99430_e151743_d_n5;
        locals.var_ibdi_dn6 = assign99430_e151743_d_n6;
        locals.var_ibdi_dn7 = assign99430_e151743_d_n7;
        locals.var_ibdi_dn8 = assign99430_e151743_d_n8;
        locals.var_ibdi_dn9 = assign99430_e151743_d_n9;
        locals.var_ibdi_dn10 = assign99430_e151743_d_n10;
        locals.var_ibdi_dn13 = assign99430_e151743_d_n13;

        let (assign99440_e151751, assign99440_e151751_d_n0, assign99440_e151751_d_n2, assign99440_e151751_d_n4, assign99440_e151751_d_n5, assign99440_e151751_d_n6, assign99440_e151751_d_n7, assign99440_e151751_d_n8, assign99440_e151751_d_n9, assign99440_e151751_d_n10, assign99440_e151751_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99440_e151748: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99440_e151749: f64 = (locals.var_mfactor * assign99440_e151748);
        (assign99440_e151749, (locals.var_mfactor * (locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0)), (locals.var_mfactor * (locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2)), (locals.var_mfactor * (locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4)), (locals.var_mfactor * (locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5)), (locals.var_mfactor * (locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6)), (locals.var_mfactor * (locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7)), (locals.var_mfactor * (locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8)), (locals.var_mfactor * (locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9)), (locals.var_mfactor * (locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10)), (locals.var_mfactor * (locals.var_qbs_btm_dn13 + locals.var_qbs_sws_dn13)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn13,)
    }
};
        locals.var_qbs = assign99440_e151751;
        locals.var_qbs_dn0 = assign99440_e151751_d_n0;
        locals.var_qbs_dn2 = assign99440_e151751_d_n2;
        locals.var_qbs_dn4 = assign99440_e151751_d_n4;
        locals.var_qbs_dn5 = assign99440_e151751_d_n5;
        locals.var_qbs_dn6 = assign99440_e151751_d_n6;
        locals.var_qbs_dn7 = assign99440_e151751_d_n7;
        locals.var_qbs_dn8 = assign99440_e151751_d_n8;
        locals.var_qbs_dn9 = assign99440_e151751_d_n9;
        locals.var_qbs_dn10 = assign99440_e151751_d_n10;
        locals.var_qbs_dn13 = assign99440_e151751_d_n13;

        let (assign99450_e151759, assign99450_e151759_d_n0, assign99450_e151759_d_n2, assign99450_e151759_d_n4, assign99450_e151759_d_n5, assign99450_e151759_d_n6, assign99450_e151759_d_n7, assign99450_e151759_d_n8, assign99450_e151759_d_n9, assign99450_e151759_d_n10, assign99450_e151759_d_n13, assign99450_e151759_d_n15, assign99450_e151759_d_n16, assign99450_e151759_d_n17,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99450_e151756: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99450_e151757: f64 = (locals.var_mfactor * assign99450_e151756);
        (assign99450_e151757, (locals.var_mfactor * (locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0)), (locals.var_mfactor * (locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2)), (locals.var_mfactor * (locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4)), (locals.var_mfactor * (locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5)), (locals.var_mfactor * (locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6)), (locals.var_mfactor * (locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7)), (locals.var_mfactor * (locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8)), (locals.var_mfactor * (locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9)), (locals.var_mfactor * (locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10)), (locals.var_mfactor * (locals.var_qbd_btm_dn13 + locals.var_qbd_sws_dn13)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign99450_e151759;
        locals.var_qbd_dn0 = assign99450_e151759_d_n0;
        locals.var_qbd_dn2 = assign99450_e151759_d_n2;
        locals.var_qbd_dn4 = assign99450_e151759_d_n4;
        locals.var_qbd_dn5 = assign99450_e151759_d_n5;
        locals.var_qbd_dn6 = assign99450_e151759_d_n6;
        locals.var_qbd_dn7 = assign99450_e151759_d_n7;
        locals.var_qbd_dn8 = assign99450_e151759_d_n8;
        locals.var_qbd_dn9 = assign99450_e151759_d_n9;
        locals.var_qbd_dn10 = assign99450_e151759_d_n10;
        locals.var_qbd_dn13 = assign99450_e151759_d_n13;
        locals.var_qbd_dn15 = assign99450_e151759_d_n15;
        locals.var_qbd_dn16 = assign99450_e151759_d_n16;
        locals.var_qbd_dn17 = assign99450_e151759_d_n17;

        let (assign99460_e151765, assign99460_e151765_d_n0, assign99460_e151765_d_n2, assign99460_e151765_d_n4, assign99460_e151765_d_n5, assign99460_e151765_d_n6, assign99460_e151765_d_n7, assign99460_e151765_d_n8, assign99460_e151765_d_n9, assign99460_e151765_d_n10, assign99460_e151765_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99460_e151763: f64 = (locals.var_mfactor * locals.var_qbs_swg);
        (assign99460_e151763, (locals.var_mfactor * locals.var_qbs_swg_dn0), (locals.var_mfactor * locals.var_qbs_swg_dn2), (locals.var_mfactor * locals.var_qbs_swg_dn4), (locals.var_mfactor * locals.var_qbs_swg_dn5), (locals.var_mfactor * locals.var_qbs_swg_dn6), (locals.var_mfactor * locals.var_qbs_swg_dn7), (locals.var_mfactor * locals.var_qbs_swg_dn8), (locals.var_mfactor * locals.var_qbs_swg_dn9), (locals.var_mfactor * locals.var_qbs_swg_dn10), (locals.var_mfactor * locals.var_qbs_swg_dn13),)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn13,)
    }
};
        locals.var_qbsi = assign99460_e151765;
        locals.var_qbsi_dn0 = assign99460_e151765_d_n0;
        locals.var_qbsi_dn2 = assign99460_e151765_d_n2;
        locals.var_qbsi_dn4 = assign99460_e151765_d_n4;
        locals.var_qbsi_dn5 = assign99460_e151765_d_n5;
        locals.var_qbsi_dn6 = assign99460_e151765_d_n6;
        locals.var_qbsi_dn7 = assign99460_e151765_d_n7;
        locals.var_qbsi_dn8 = assign99460_e151765_d_n8;
        locals.var_qbsi_dn9 = assign99460_e151765_d_n9;
        locals.var_qbsi_dn10 = assign99460_e151765_d_n10;
        locals.var_qbsi_dn13 = assign99460_e151765_d_n13;

    }

    pub(super) fn stamp_transient_block_353(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign99470_e151771, assign99470_e151771_d_n0, assign99470_e151771_d_n2, assign99470_e151771_d_n4, assign99470_e151771_d_n5, assign99470_e151771_d_n6, assign99470_e151771_d_n7, assign99470_e151771_d_n8, assign99470_e151771_d_n9, assign99470_e151771_d_n10, assign99470_e151771_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99470_e151769: f64 = (locals.var_mfactor * locals.var_qbd_swg);
        (assign99470_e151769, (locals.var_mfactor * locals.var_qbd_swg_dn0), (locals.var_mfactor * locals.var_qbd_swg_dn2), (locals.var_mfactor * locals.var_qbd_swg_dn4), (locals.var_mfactor * locals.var_qbd_swg_dn5), (locals.var_mfactor * locals.var_qbd_swg_dn6), (locals.var_mfactor * locals.var_qbd_swg_dn7), (locals.var_mfactor * locals.var_qbd_swg_dn8), (locals.var_mfactor * locals.var_qbd_swg_dn9), (locals.var_mfactor * locals.var_qbd_swg_dn10), (locals.var_mfactor * locals.var_qbd_swg_dn13),)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn13,)
    }
};
        locals.var_qbdi = assign99470_e151771;
        locals.var_qbdi_dn0 = assign99470_e151771_d_n0;
        locals.var_qbdi_dn2 = assign99470_e151771_d_n2;
        locals.var_qbdi_dn4 = assign99470_e151771_d_n4;
        locals.var_qbdi_dn5 = assign99470_e151771_d_n5;
        locals.var_qbdi_dn6 = assign99470_e151771_d_n6;
        locals.var_qbdi_dn7 = assign99470_e151771_d_n7;
        locals.var_qbdi_dn8 = assign99470_e151771_d_n8;
        locals.var_qbdi_dn9 = assign99470_e151771_d_n9;
        locals.var_qbdi_dn10 = assign99470_e151771_d_n10;
        locals.var_qbdi_dn13 = assign99470_e151771_d_n13;

        let (assign99520_e151804, assign99520_e151804_d_n0, assign99520_e151804_d_n2, assign99520_e151804_d_n4, assign99520_e151804_d_n5, assign99520_e151804_d_n6, assign99520_e151804_d_n7, assign99520_e151804_d_n8, assign99520_e151804_d_n9, assign99520_e151804_d_n10, assign99520_e151804_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn13,)
    }
};
        locals.var_ibsi = assign99520_e151804;
        locals.var_ibsi_dn0 = assign99520_e151804_d_n0;
        locals.var_ibsi_dn2 = assign99520_e151804_d_n2;
        locals.var_ibsi_dn4 = assign99520_e151804_d_n4;
        locals.var_ibsi_dn5 = assign99520_e151804_d_n5;
        locals.var_ibsi_dn6 = assign99520_e151804_d_n6;
        locals.var_ibsi_dn7 = assign99520_e151804_d_n7;
        locals.var_ibsi_dn8 = assign99520_e151804_d_n8;
        locals.var_ibsi_dn9 = assign99520_e151804_d_n9;
        locals.var_ibsi_dn10 = assign99520_e151804_d_n10;
        locals.var_ibsi_dn13 = assign99520_e151804_d_n13;

        let (assign99530_e151809, assign99530_e151809_d_n0, assign99530_e151809_d_n2, assign99530_e151809_d_n4, assign99530_e151809_d_n5, assign99530_e151809_d_n6, assign99530_e151809_d_n7, assign99530_e151809_d_n8, assign99530_e151809_d_n9, assign99530_e151809_d_n10, assign99530_e151809_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibdi, locals.var_ibdi_dn0, locals.var_ibdi_dn2, locals.var_ibdi_dn4, locals.var_ibdi_dn5, locals.var_ibdi_dn6, locals.var_ibdi_dn7, locals.var_ibdi_dn8, locals.var_ibdi_dn9, locals.var_ibdi_dn10, locals.var_ibdi_dn13,)
    }
};
        locals.var_ibdi = assign99530_e151809;
        locals.var_ibdi_dn0 = assign99530_e151809_d_n0;
        locals.var_ibdi_dn2 = assign99530_e151809_d_n2;
        locals.var_ibdi_dn4 = assign99530_e151809_d_n4;
        locals.var_ibdi_dn5 = assign99530_e151809_d_n5;
        locals.var_ibdi_dn6 = assign99530_e151809_d_n6;
        locals.var_ibdi_dn7 = assign99530_e151809_d_n7;
        locals.var_ibdi_dn8 = assign99530_e151809_d_n8;
        locals.var_ibdi_dn9 = assign99530_e151809_d_n9;
        locals.var_ibdi_dn10 = assign99530_e151809_d_n10;
        locals.var_ibdi_dn13 = assign99530_e151809_d_n13;

        let (assign99540_e151820, assign99540_e151820_d_n0, assign99540_e151820_d_n2, assign99540_e151820_d_n4, assign99540_e151820_d_n5, assign99540_e151820_d_n6, assign99540_e151820_d_n7, assign99540_e151820_d_n8, assign99540_e151820_d_n9, assign99540_e151820_d_n10, assign99540_e151820_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        let assign99540_e151815: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99540_e151817: f64 = (assign99540_e151815 + locals.var_qbs_swg);
        let assign99540_e151818: f64 = (locals.var_mfactor * assign99540_e151817);
        (assign99540_e151818, (locals.var_mfactor * ((locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0) + locals.var_qbs_swg_dn0)), (locals.var_mfactor * ((locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2) + locals.var_qbs_swg_dn2)), (locals.var_mfactor * ((locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4) + locals.var_qbs_swg_dn4)), (locals.var_mfactor * ((locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5) + locals.var_qbs_swg_dn5)), (locals.var_mfactor * ((locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6) + locals.var_qbs_swg_dn6)), (locals.var_mfactor * ((locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7) + locals.var_qbs_swg_dn7)), (locals.var_mfactor * ((locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8) + locals.var_qbs_swg_dn8)), (locals.var_mfactor * ((locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9) + locals.var_qbs_swg_dn9)), (locals.var_mfactor * ((locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10) + locals.var_qbs_swg_dn10)), (locals.var_mfactor * ((locals.var_qbs_btm_dn13 + locals.var_qbs_sws_dn13) + locals.var_qbs_swg_dn13)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn13,)
    }
};
        locals.var_qbs = assign99540_e151820;
        locals.var_qbs_dn0 = assign99540_e151820_d_n0;
        locals.var_qbs_dn2 = assign99540_e151820_d_n2;
        locals.var_qbs_dn4 = assign99540_e151820_d_n4;
        locals.var_qbs_dn5 = assign99540_e151820_d_n5;
        locals.var_qbs_dn6 = assign99540_e151820_d_n6;
        locals.var_qbs_dn7 = assign99540_e151820_d_n7;
        locals.var_qbs_dn8 = assign99540_e151820_d_n8;
        locals.var_qbs_dn9 = assign99540_e151820_d_n9;
        locals.var_qbs_dn10 = assign99540_e151820_d_n10;
        locals.var_qbs_dn13 = assign99540_e151820_d_n13;

        let (assign99550_e151831, assign99550_e151831_d_n0, assign99550_e151831_d_n2, assign99550_e151831_d_n4, assign99550_e151831_d_n5, assign99550_e151831_d_n6, assign99550_e151831_d_n7, assign99550_e151831_d_n8, assign99550_e151831_d_n9, assign99550_e151831_d_n10, assign99550_e151831_d_n13, assign99550_e151831_d_n15, assign99550_e151831_d_n16, assign99550_e151831_d_n17,) = {
    if (locals.var_guard2300 == 0.0) {
        let assign99550_e151826: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99550_e151828: f64 = (assign99550_e151826 + locals.var_qbd_swg);
        let assign99550_e151829: f64 = (locals.var_mfactor * assign99550_e151828);
        (assign99550_e151829, (locals.var_mfactor * ((locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0) + locals.var_qbd_swg_dn0)), (locals.var_mfactor * ((locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2) + locals.var_qbd_swg_dn2)), (locals.var_mfactor * ((locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4) + locals.var_qbd_swg_dn4)), (locals.var_mfactor * ((locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5) + locals.var_qbd_swg_dn5)), (locals.var_mfactor * ((locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6) + locals.var_qbd_swg_dn6)), (locals.var_mfactor * ((locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7) + locals.var_qbd_swg_dn7)), (locals.var_mfactor * ((locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8) + locals.var_qbd_swg_dn8)), (locals.var_mfactor * ((locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9) + locals.var_qbd_swg_dn9)), (locals.var_mfactor * ((locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10) + locals.var_qbd_swg_dn10)), (locals.var_mfactor * ((locals.var_qbd_btm_dn13 + locals.var_qbd_sws_dn13) + locals.var_qbd_swg_dn13)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign99550_e151831;
        locals.var_qbd_dn0 = assign99550_e151831_d_n0;
        locals.var_qbd_dn2 = assign99550_e151831_d_n2;
        locals.var_qbd_dn4 = assign99550_e151831_d_n4;
        locals.var_qbd_dn5 = assign99550_e151831_d_n5;
        locals.var_qbd_dn6 = assign99550_e151831_d_n6;
        locals.var_qbd_dn7 = assign99550_e151831_d_n7;
        locals.var_qbd_dn8 = assign99550_e151831_d_n8;
        locals.var_qbd_dn9 = assign99550_e151831_d_n9;
        locals.var_qbd_dn10 = assign99550_e151831_d_n10;
        locals.var_qbd_dn13 = assign99550_e151831_d_n13;
        locals.var_qbd_dn15 = assign99550_e151831_d_n15;
        locals.var_qbd_dn16 = assign99550_e151831_d_n16;
        locals.var_qbd_dn17 = assign99550_e151831_d_n17;

        let (assign99580_e151858, assign99580_e151858_d_n0, assign99580_e151858_d_n2, assign99580_e151858_d_n4, assign99580_e151858_d_n5, assign99580_e151858_d_n6, assign99580_e151858_d_n7, assign99580_e151858_d_n8, assign99580_e151858_d_n9, assign99580_e151858_d_n10, assign99580_e151858_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn13,)
    }
};
        locals.var_qbsi = assign99580_e151858;
        locals.var_qbsi_dn0 = assign99580_e151858_d_n0;
        locals.var_qbsi_dn2 = assign99580_e151858_d_n2;
        locals.var_qbsi_dn4 = assign99580_e151858_d_n4;
        locals.var_qbsi_dn5 = assign99580_e151858_d_n5;
        locals.var_qbsi_dn6 = assign99580_e151858_d_n6;
        locals.var_qbsi_dn7 = assign99580_e151858_d_n7;
        locals.var_qbsi_dn8 = assign99580_e151858_d_n8;
        locals.var_qbsi_dn9 = assign99580_e151858_d_n9;
        locals.var_qbsi_dn10 = assign99580_e151858_d_n10;
        locals.var_qbsi_dn13 = assign99580_e151858_d_n13;

        let (assign99590_e151863, assign99590_e151863_d_n0, assign99590_e151863_d_n2, assign99590_e151863_d_n4, assign99590_e151863_d_n5, assign99590_e151863_d_n6, assign99590_e151863_d_n7, assign99590_e151863_d_n8, assign99590_e151863_d_n9, assign99590_e151863_d_n10, assign99590_e151863_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn13,)
    }
};
        locals.var_qbdi = assign99590_e151863;
        locals.var_qbdi_dn0 = assign99590_e151863_d_n0;
        locals.var_qbdi_dn2 = assign99590_e151863_d_n2;
        locals.var_qbdi_dn4 = assign99590_e151863_d_n4;
        locals.var_qbdi_dn5 = assign99590_e151863_d_n5;
        locals.var_qbdi_dn6 = assign99590_e151863_d_n6;
        locals.var_qbdi_dn7 = assign99590_e151863_d_n7;
        locals.var_qbdi_dn8 = assign99590_e151863_d_n8;
        locals.var_qbdi_dn9 = assign99590_e151863_d_n9;
        locals.var_qbdi_dn10 = assign99590_e151863_d_n10;
        locals.var_qbdi_dn13 = assign99590_e151863_d_n13;

        let assign99620_e151876: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99620_e151876;

        locals.var_njl = locals.var_uc_njd;

        let assign99640_e151880: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99640_e151880;

        let assign99650_e151883: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99650_e151883;

        locals.var_juncdlt = 0.001;

        let assign99670_e151888: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign99670_e151891: f64 = (locals.var_eg * locals.var_beta);
        let assign99670_e151892: f64 = (assign99670_e151888 - assign99670_e151891);
        let assign99670_e151895: f64 = (p.p499 * locals.var_log_tratio);
        let assign99670_e151896: f64 = (assign99670_e151892 + assign99670_e151895);
        let assign99670_e151898: f64 = (assign99670_e151896 / locals.var_uc_njd);
        let assign99670_e151899: f64 = (assign99670_e151898).exp();
        let assign99670_e151900: f64 = (1.45e16 * assign99670_e151899);
        locals.var_nin_dio = assign99670_e151900;
        locals.var_nin_dio_dn0 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn2 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn4 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn5 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn6 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn7 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn8 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn9 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn10 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn13 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd)));

        let assign99680_e151903: f64 = (locals.var_nin_dio * locals.var_nin_dio);
        let assign99680_e151905: f64 = (assign99680_e151903 / locals.var_ndi_i);
        locals.var_pn0 = assign99680_e151905;
        locals.var_pn0_dn0 = (((locals.var_nin_dio_dn0 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn0)) / locals.var_ndi_i);
        locals.var_pn0_dn2 = (((locals.var_nin_dio_dn2 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn2)) / locals.var_ndi_i);
        locals.var_pn0_dn4 = (((locals.var_nin_dio_dn4 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn4)) / locals.var_ndi_i);
        locals.var_pn0_dn5 = (((locals.var_nin_dio_dn5 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn5)) / locals.var_ndi_i);
        locals.var_pn0_dn6 = (((locals.var_nin_dio_dn6 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn6)) / locals.var_ndi_i);
        locals.var_pn0_dn7 = (((locals.var_nin_dio_dn7 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn7)) / locals.var_ndi_i);
        locals.var_pn0_dn8 = (((locals.var_nin_dio_dn8 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn8)) / locals.var_ndi_i);
        locals.var_pn0_dn9 = (((locals.var_nin_dio_dn9 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn9)) / locals.var_ndi_i);
        locals.var_pn0_dn10 = (((locals.var_nin_dio_dn10 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn10)) / locals.var_ndi_i);
        locals.var_pn0_dn13 = (((locals.var_nin_dio_dn13 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn13)) / locals.var_ndi_i);

        let assign99690_e151908: f64 = (-1.5);
        let assign99690_e151909: f64 = (locals.var_tratio).powf(assign99690_e151908);
        locals.var_t1 = assign99690_e151909;
        locals.var_t1_dn0 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t1_dn2 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t1_dn4 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t1_dn5 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t1_dn6 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t1_dn7 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t1_dn8 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t1_dn9 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t1_dn10 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t1_dn13 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn13)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn13 / locals.var_tratio))) };

        let assign99700_e151912: f64 = (locals.var_muen_i * locals.var_t1);
        let assign99700_e151914: f64 = (assign99700_e151912 * locals.var_beta_inv);
        locals.var_dn = assign99700_e151914;
        locals.var_dn_dn0 = (((locals.var_muen_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn0));
        locals.var_dn_dn2 = (((locals.var_muen_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn2));
        locals.var_dn_dn4 = (((locals.var_muen_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn4));
        locals.var_dn_dn5 = (((locals.var_muen_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn5));
        locals.var_dn_dn6 = (((locals.var_muen_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn6));
        locals.var_dn_dn7 = (((locals.var_muen_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn7));
        locals.var_dn_dn8 = (((locals.var_muen_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn8));
        locals.var_dn_dn9 = (((locals.var_muen_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn9));
        locals.var_dn_dn10 = (((locals.var_muen_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn10));
        locals.var_dn_dn13 = (((locals.var_muen_i * locals.var_t1_dn13) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn13));

        let assign99710_e151917: f64 = (locals.var_muep_i * locals.var_t1);
        let assign99710_e151919: f64 = (assign99710_e151917 * locals.var_beta_inv);
        locals.var_dp = assign99710_e151919;
        locals.var_dp_dn0 = (((locals.var_muep_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn0));
        locals.var_dp_dn2 = (((locals.var_muep_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn2));
        locals.var_dp_dn4 = (((locals.var_muep_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn4));
        locals.var_dp_dn5 = (((locals.var_muep_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn5));
        locals.var_dp_dn6 = (((locals.var_muep_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn6));
        locals.var_dp_dn7 = (((locals.var_muep_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn7));
        locals.var_dp_dn8 = (((locals.var_muep_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn8));
        locals.var_dp_dn9 = (((locals.var_muep_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn9));
        locals.var_dp_dn10 = (((locals.var_muep_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn10));
        locals.var_dp_dn13 = (((locals.var_muep_i * locals.var_t1_dn13) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn13));

        let assign99720_e151922: f64 = (2.0 * locals.var_dn);
        let assign99720_e151924: f64 = (assign99720_e151922 * locals.var_dp);
        let assign99720_e151927: f64 = (locals.var_dn + locals.var_dp);
        let assign99720_e151928: f64 = (assign99720_e151924 / assign99720_e151927);
        locals.var_da = assign99720_e151928;
        locals.var_da_dn0 = ((((((2.0 * locals.var_dn_dn0) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn0)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn0 + locals.var_dp_dn0))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn2 = ((((((2.0 * locals.var_dn_dn2) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn2)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn2 + locals.var_dp_dn2))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn4 = ((((((2.0 * locals.var_dn_dn4) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn4)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn4 + locals.var_dp_dn4))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn5 = ((((((2.0 * locals.var_dn_dn5) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn5)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn5 + locals.var_dp_dn5))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn6 = ((((((2.0 * locals.var_dn_dn6) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn6)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn6 + locals.var_dp_dn6))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn7 = ((((((2.0 * locals.var_dn_dn7) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn7)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn7 + locals.var_dp_dn7))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn8 = ((((((2.0 * locals.var_dn_dn8) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn8)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn8 + locals.var_dp_dn8))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn9 = ((((((2.0 * locals.var_dn_dn9) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn9)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn9 + locals.var_dp_dn9))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn10 = ((((((2.0 * locals.var_dn_dn10) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn10)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn10 + locals.var_dp_dn10))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn13 = ((((((2.0 * locals.var_dn_dn13) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn13)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn13 + locals.var_dp_dn13))) / (assign99720_e151927 * assign99720_e151927));

        let assign99730_e151931: f64 = (locals.var_tratio).powf(p.p547);
        locals.var_t2 = assign99730_e151931;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t2_dn4 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t2_dn13 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn13)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn13 / locals.var_tratio))) };

        let assign99740_e151934: f64 = (p.p544 * locals.var_t2);
        locals.var_tau_hl = assign99740_e151934;
        locals.var_tau_hl_dn0 = (p.p544 * locals.var_t2_dn0);
        locals.var_tau_hl_dn2 = (p.p544 * locals.var_t2_dn2);
        locals.var_tau_hl_dn4 = (p.p544 * locals.var_t2_dn4);
        locals.var_tau_hl_dn5 = (p.p544 * locals.var_t2_dn5);
        locals.var_tau_hl_dn6 = (p.p544 * locals.var_t2_dn6);
        locals.var_tau_hl_dn7 = (p.p544 * locals.var_t2_dn7);
        locals.var_tau_hl_dn8 = (p.p544 * locals.var_t2_dn8);
        locals.var_tau_hl_dn9 = (p.p544 * locals.var_t2_dn9);
        locals.var_tau_hl_dn10 = (p.p544 * locals.var_t2_dn10);
        locals.var_tau_hl_dn13 = (p.p544 * locals.var_t2_dn13);

        let assign99750_e151937: f64 = (locals.var_tau_hl * locals.var_da);
        let assign99750_e151938: f64 = (assign99750_e151937).sqrt();
        locals.var_la = assign99750_e151938;
        locals.var_la_dn0 = (((locals.var_tau_hl_dn0 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn0)) / (2.0 * assign99750_e151938));
        locals.var_la_dn2 = (((locals.var_tau_hl_dn2 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn2)) / (2.0 * assign99750_e151938));
        locals.var_la_dn4 = (((locals.var_tau_hl_dn4 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn4)) / (2.0 * assign99750_e151938));
        locals.var_la_dn5 = (((locals.var_tau_hl_dn5 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn5)) / (2.0 * assign99750_e151938));
        locals.var_la_dn6 = (((locals.var_tau_hl_dn6 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn6)) / (2.0 * assign99750_e151938));
        locals.var_la_dn7 = (((locals.var_tau_hl_dn7 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn7)) / (2.0 * assign99750_e151938));
        locals.var_la_dn8 = (((locals.var_tau_hl_dn8 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn8)) / (2.0 * assign99750_e151938));
        locals.var_la_dn9 = (((locals.var_tau_hl_dn9 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn9)) / (2.0 * assign99750_e151938));
        locals.var_la_dn10 = (((locals.var_tau_hl_dn10 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn10)) / (2.0 * assign99750_e151938));
        locals.var_la_dn13 = (((locals.var_tau_hl_dn13 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn13)) / (2.0 * assign99750_e151938));

        let assign99760_e151941: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99760_e151944: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99760_e151945: f64 = (assign99760_e151944).ln();
        let assign99760_e151946: f64 = (assign99760_e151941 * assign99760_e151945);
        locals.var_v_ha = assign99760_e151946;
        locals.var_v_ha_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn13 = (((locals.var_njl * locals.var_beta_inv_dn13) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn13) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));

        let assign99770_e151949: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99770_e151952: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99770_e151953: f64 = (assign99770_e151952).ln();
        let assign99770_e151956: f64 = (p.p545 / locals.var_la);
        let assign99770_e151957: f64 = (assign99770_e151953 + assign99770_e151956);
        let assign99770_e151958: f64 = (assign99770_e151949 * assign99770_e151957);
        locals.var_v_hk = assign99770_e151958;
        locals.var_v_hk_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn0) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn2) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn4) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn5) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn6) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn7) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn8) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn9) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn10) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn13 = (((locals.var_njl * locals.var_beta_inv_dn13) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn13) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn13) / (locals.var_la * locals.var_la))))));

        let assign99780_e151961: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99780_e151961;

        let (assign99790_e151965,) = {
    if (locals.var_guard2301 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99790_e151965;

        let (assign99800_e151972, assign99800_e151972_d_n0, assign99800_e151972_d_n2, assign99800_e151972_d_n4, assign99800_e151972_d_n5, assign99800_e151972_d_n6, assign99800_e151972_d_n7, assign99800_e151972_d_n8, assign99800_e151972_d_n9, assign99800_e151972_d_n10, assign99800_e151972_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99800_e151969: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        let assign99800_e151970: f64 = (assign99800_e151969).exp();
        (assign99800_e151970, (assign99800_e151970 * ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0))), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8)), (assign99800_e151970 * ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9))), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13)),)
    } else {
        (locals.var_exp_a, locals.var_exp_a_dn0, locals.var_exp_a_dn2, locals.var_exp_a_dn4, locals.var_exp_a_dn5, locals.var_exp_a_dn6, locals.var_exp_a_dn7, locals.var_exp_a_dn8, locals.var_exp_a_dn9, locals.var_exp_a_dn10, locals.var_exp_a_dn13,)
    }
};
        locals.var_exp_a = assign99800_e151972;
        locals.var_exp_a_dn0 = assign99800_e151972_d_n0;
        locals.var_exp_a_dn2 = assign99800_e151972_d_n2;
        locals.var_exp_a_dn4 = assign99800_e151972_d_n4;
        locals.var_exp_a_dn5 = assign99800_e151972_d_n5;
        locals.var_exp_a_dn6 = assign99800_e151972_d_n6;
        locals.var_exp_a_dn7 = assign99800_e151972_d_n7;
        locals.var_exp_a_dn8 = assign99800_e151972_d_n8;
        locals.var_exp_a_dn9 = assign99800_e151972_d_n9;
        locals.var_exp_a_dn10 = assign99800_e151972_d_n10;
        locals.var_exp_a_dn13 = assign99800_e151972_d_n13;

        let assign99810_e151976: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99810_e151977: f64 = (locals.var_vbd_jct - assign99810_e151976);
        let assign99810_e151979: f64 = if assign99810_e151977 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99810_e151979;

        let (assign99820_e151996, assign99820_e151996_d_n0, assign99820_e151996_d_n2, assign99820_e151996_d_n4, assign99820_e151996_d_n5, assign99820_e151996_d_n6, assign99820_e151996_d_n7, assign99820_e151996_d_n8, assign99820_e151996_d_n9, assign99820_e151996_d_n10, assign99820_e151996_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2302 != 0.0)) {
        let assign99820_e151986: f64 = (locals.var_vbd_jct / locals.var_nj_k);
        let assign99820_e151989: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99820_e151991: f64 = (assign99820_e151989 / locals.var_nj_k);
        let assign99820_e151992: f64 = (assign99820_e151986 - assign99820_e151991);
        let assign99820_e151993: f64 = (locals.var_beta * assign99820_e151992);
        let assign99820_e151994: f64 = (assign99820_e151993).exp();
        (assign99820_e151994, (assign99820_e151994 * ((locals.var_beta_dn0 * assign99820_e151992) + (locals.var_beta * ((locals.var_vbd_jct_dn0 / locals.var_nj_k) - ((locals.var_v_hk_dn0 - locals.var_v_ha_dn0) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn2 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn2 - locals.var_v_ha_dn2) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn4 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn4 - locals.var_v_ha_dn4) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn5 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn5 - locals.var_v_ha_dn5) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn6 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn6 - locals.var_v_ha_dn6) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn7 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn7 - locals.var_v_ha_dn7) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn8 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn8 - locals.var_v_ha_dn8) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn9 * assign99820_e151992) + (locals.var_beta * ((locals.var_vbd_jct_dn9 / locals.var_nj_k) - ((locals.var_v_hk_dn9 - locals.var_v_ha_dn9) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn10 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn10 - locals.var_v_ha_dn10) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn13 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn13 - locals.var_v_ha_dn13) / locals.var_nj_k))))),)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn13,)
    }
};
        locals.var_exp_k = assign99820_e151996;
        locals.var_exp_k_dn0 = assign99820_e151996_d_n0;
        locals.var_exp_k_dn2 = assign99820_e151996_d_n2;
        locals.var_exp_k_dn4 = assign99820_e151996_d_n4;
        locals.var_exp_k_dn5 = assign99820_e151996_d_n5;
        locals.var_exp_k_dn6 = assign99820_e151996_d_n6;
        locals.var_exp_k_dn7 = assign99820_e151996_d_n7;
        locals.var_exp_k_dn8 = assign99820_e151996_d_n8;
        locals.var_exp_k_dn9 = assign99820_e151996_d_n9;
        locals.var_exp_k_dn10 = assign99820_e151996_d_n10;
        locals.var_exp_k_dn13 = assign99820_e151996_d_n13;

        let (assign99830_e152003, assign99830_e152003_d_n0, assign99830_e152003_d_n2, assign99830_e152003_d_n4, assign99830_e152003_d_n5, assign99830_e152003_d_n6, assign99830_e152003_d_n7, assign99830_e152003_d_n8, assign99830_e152003_d_n9, assign99830_e152003_d_n10, assign99830_e152003_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2302 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn13,)
    }
};
        locals.var_exp_k = assign99830_e152003;
        locals.var_exp_k_dn0 = assign99830_e152003_d_n0;
        locals.var_exp_k_dn2 = assign99830_e152003_d_n2;
        locals.var_exp_k_dn4 = assign99830_e152003_d_n4;
        locals.var_exp_k_dn5 = assign99830_e152003_d_n5;
        locals.var_exp_k_dn6 = assign99830_e152003_d_n6;
        locals.var_exp_k_dn7 = assign99830_e152003_d_n7;
        locals.var_exp_k_dn8 = assign99830_e152003_d_n8;
        locals.var_exp_k_dn9 = assign99830_e152003_d_n9;
        locals.var_exp_k_dn10 = assign99830_e152003_d_n10;
        locals.var_exp_k_dn13 = assign99830_e152003_d_n13;

        let assign99840_e152010: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99840_e152010;

        let (assign99850_e152018, assign99850_e152018_d_n0, assign99850_e152018_d_n2, assign99850_e152018_d_n4, assign99850_e152018_d_n5, assign99850_e152018_d_n6, assign99850_e152018_d_n7, assign99850_e152018_d_n8, assign99850_e152018_d_n9, assign99850_e152018_d_n10, assign99850_e152018_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2303 != 0.0)) {
        let assign99850_e152016: f64 = (locals.var_exp_a * p.p541);
        (assign99850_e152016, (locals.var_exp_a_dn0 * p.p541), (locals.var_exp_a_dn2 * p.p541), (locals.var_exp_a_dn4 * p.p541), (locals.var_exp_a_dn5 * p.p541), (locals.var_exp_a_dn6 * p.p541), (locals.var_exp_a_dn7 * p.p541), (locals.var_exp_a_dn8 * p.p541), (locals.var_exp_a_dn9 * p.p541), (locals.var_exp_a_dn10 * p.p541), (locals.var_exp_a_dn13 * p.p541),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99850_e152018;
        locals.var_exp_a2_dn0 = assign99850_e152018_d_n0;
        locals.var_exp_a2_dn2 = assign99850_e152018_d_n2;
        locals.var_exp_a2_dn4 = assign99850_e152018_d_n4;
        locals.var_exp_a2_dn5 = assign99850_e152018_d_n5;
        locals.var_exp_a2_dn6 = assign99850_e152018_d_n6;
        locals.var_exp_a2_dn7 = assign99850_e152018_d_n7;
        locals.var_exp_a2_dn8 = assign99850_e152018_d_n8;
        locals.var_exp_a2_dn9 = assign99850_e152018_d_n9;
        locals.var_exp_a2_dn10 = assign99850_e152018_d_n10;
        locals.var_exp_a2_dn13 = assign99850_e152018_d_n13;

        let (assign99860_e152047, assign99860_e152047_d_n0, assign99860_e152047_d_n2, assign99860_e152047_d_n4, assign99860_e152047_d_n5, assign99860_e152047_d_n6, assign99860_e152047_d_n7, assign99860_e152047_d_n8, assign99860_e152047_d_n9, assign99860_e152047_d_n10, assign99860_e152047_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2303 == 0.0)) {
        let assign99860_e152025: f64 = (locals.var_exp_a * p.p541);
        let assign99860_e152027: f64 = (-p.p542);
        let assign99860_e152030: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99860_e152031: f64 = (assign99860_e152027 * assign99860_e152030);
        let assign99860_e152034: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99860_e152035: f64 = (assign99860_e152031 * assign99860_e152034);
        let assign99860_e152039: f64 = (1.0 / locals.var_tratio);
        let assign99860_e152040: f64 = (assign99860_e152039).ln();
        let assign99860_e152041: f64 = (p.p548 * assign99860_e152040);
        let assign99860_e152042: f64 = (assign99860_e152041).exp();
        let assign99860_e152043: f64 = (assign99860_e152035 * assign99860_e152042);
        let assign99860_e152044: f64 = (assign99860_e152043).exp();
        let assign99860_e152045: f64 = (assign99860_e152025 * assign99860_e152044);
        (assign99860_e152045, (((locals.var_exp_a_dn0 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0)) * assign99860_e152034) + (assign99860_e152031 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn2 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn2)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn2))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn4 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn4)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn4))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn5 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn5)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn5))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn6 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn6)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn6))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn7 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn7)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn7))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn8 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn8)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn8))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn9 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (locals.var_vbd_jct_dn9 - locals.var_v_ha_dn9)) * assign99860_e152034) + (assign99860_e152031 * (locals.var_vbd_jct_dn9 - locals.var_v_ha_dn9))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn10 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn10)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn10))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn13 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn13)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn13))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn13 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99860_e152047;
        locals.var_exp_a2_dn0 = assign99860_e152047_d_n0;
        locals.var_exp_a2_dn2 = assign99860_e152047_d_n2;
        locals.var_exp_a2_dn4 = assign99860_e152047_d_n4;
        locals.var_exp_a2_dn5 = assign99860_e152047_d_n5;
        locals.var_exp_a2_dn6 = assign99860_e152047_d_n6;
        locals.var_exp_a2_dn7 = assign99860_e152047_d_n7;
        locals.var_exp_a2_dn8 = assign99860_e152047_d_n8;
        locals.var_exp_a2_dn9 = assign99860_e152047_d_n9;
        locals.var_exp_a2_dn10 = assign99860_e152047_d_n10;
        locals.var_exp_a2_dn13 = assign99860_e152047_d_n13;

        let (assign99870_e152056, assign99870_e152056_d_n0, assign99870_e152056_d_n2, assign99870_e152056_d_n4, assign99870_e152056_d_n5, assign99870_e152056_d_n6, assign99870_e152056_d_n7, assign99870_e152056_d_n8, assign99870_e152056_d_n9, assign99870_e152056_d_n10, assign99870_e152056_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign99870_e152054, assign99870_e152054_d_n0, assign99870_e152054_d_n2, assign99870_e152054_d_n4, assign99870_e152054_d_n5, assign99870_e152054_d_n6, assign99870_e152054_d_n7, assign99870_e152054_d_n8, assign99870_e152054_d_n9, assign99870_e152054_d_n10, assign99870_e152054_d_n13,) = {
            if (locals.var_exp_a2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
            }
        };
        (assign99870_e152054, assign99870_e152054_d_n0, assign99870_e152054_d_n2, assign99870_e152054_d_n4, assign99870_e152054_d_n5, assign99870_e152054_d_n6, assign99870_e152054_d_n7, assign99870_e152054_d_n8, assign99870_e152054_d_n9, assign99870_e152054_d_n10, assign99870_e152054_d_n13,)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99870_e152056;
        locals.var_exp_a2_dn0 = assign99870_e152056_d_n0;
        locals.var_exp_a2_dn2 = assign99870_e152056_d_n2;
        locals.var_exp_a2_dn4 = assign99870_e152056_d_n4;
        locals.var_exp_a2_dn5 = assign99870_e152056_d_n5;
        locals.var_exp_a2_dn6 = assign99870_e152056_d_n6;
        locals.var_exp_a2_dn7 = assign99870_e152056_d_n7;
        locals.var_exp_a2_dn8 = assign99870_e152056_d_n8;
        locals.var_exp_a2_dn9 = assign99870_e152056_d_n9;
        locals.var_exp_a2_dn10 = assign99870_e152056_d_n10;
        locals.var_exp_a2_dn13 = assign99870_e152056_d_n13;

        let (assign99880_e152062, assign99880_e152062_d_n0, assign99880_e152062_d_n2, assign99880_e152062_d_n4, assign99880_e152062_d_n5, assign99880_e152062_d_n6, assign99880_e152062_d_n7, assign99880_e152062_d_n8, assign99880_e152062_d_n9, assign99880_e152062_d_n10, assign99880_e152062_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99880_e152060: f64 = (locals.var_pn0 * locals.var_exp_a2);
        (assign99880_e152060, ((locals.var_pn0_dn0 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn10)), ((locals.var_pn0_dn13 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn13)),)
    } else {
        (locals.var_p_na, locals.var_p_na_dn0, locals.var_p_na_dn2, locals.var_p_na_dn4, locals.var_p_na_dn5, locals.var_p_na_dn6, locals.var_p_na_dn7, locals.var_p_na_dn8, locals.var_p_na_dn9, locals.var_p_na_dn10, locals.var_p_na_dn13,)
    }
};
        locals.var_p_na = assign99880_e152062;
        locals.var_p_na_dn0 = assign99880_e152062_d_n0;
        locals.var_p_na_dn2 = assign99880_e152062_d_n2;
        locals.var_p_na_dn4 = assign99880_e152062_d_n4;
        locals.var_p_na_dn5 = assign99880_e152062_d_n5;
        locals.var_p_na_dn6 = assign99880_e152062_d_n6;
        locals.var_p_na_dn7 = assign99880_e152062_d_n7;
        locals.var_p_na_dn8 = assign99880_e152062_d_n8;
        locals.var_p_na_dn9 = assign99880_e152062_d_n9;
        locals.var_p_na_dn10 = assign99880_e152062_d_n10;
        locals.var_p_na_dn13 = assign99880_e152062_d_n13;

    }

    pub(super) fn stamp_transient_block_354(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (assign99890_e152072, assign99890_e152072_d_n0, assign99890_e152072_d_n2, assign99890_e152072_d_n4, assign99890_e152072_d_n5, assign99890_e152072_d_n6, assign99890_e152072_d_n7, assign99890_e152072_d_n8, assign99890_e152072_d_n9, assign99890_e152072_d_n10, assign99890_e152072_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99890_e152066: f64 = (1.6021918e-19 * p.p13);
        let assign99890_e152069: f64 = (locals.var_p_na - locals.var_pn0);
        let assign99890_e152070: f64 = (assign99890_e152066 * assign99890_e152069);
        (assign99890_e152070, (assign99890_e152066 * (locals.var_p_na_dn0 - locals.var_pn0_dn0)), (assign99890_e152066 * (locals.var_p_na_dn2 - locals.var_pn0_dn2)), (assign99890_e152066 * (locals.var_p_na_dn4 - locals.var_pn0_dn4)), (assign99890_e152066 * (locals.var_p_na_dn5 - locals.var_pn0_dn5)), (assign99890_e152066 * (locals.var_p_na_dn6 - locals.var_pn0_dn6)), (assign99890_e152066 * (locals.var_p_na_dn7 - locals.var_pn0_dn7)), (assign99890_e152066 * (locals.var_p_na_dn8 - locals.var_pn0_dn8)), (assign99890_e152066 * (locals.var_p_na_dn9 - locals.var_pn0_dn9)), (assign99890_e152066 * (locals.var_p_na_dn10 - locals.var_pn0_dn10)), (assign99890_e152066 * (locals.var_p_na_dn13 - locals.var_pn0_dn13)),)
    } else {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn13,)
    }
};
        locals.var_q_pexa = assign99890_e152072;
        locals.var_q_pexa_dn0 = assign99890_e152072_d_n0;
        locals.var_q_pexa_dn2 = assign99890_e152072_d_n2;
        locals.var_q_pexa_dn4 = assign99890_e152072_d_n4;
        locals.var_q_pexa_dn5 = assign99890_e152072_d_n5;
        locals.var_q_pexa_dn6 = assign99890_e152072_d_n6;
        locals.var_q_pexa_dn7 = assign99890_e152072_d_n7;
        locals.var_q_pexa_dn8 = assign99890_e152072_d_n8;
        locals.var_q_pexa_dn9 = assign99890_e152072_d_n9;
        locals.var_q_pexa_dn10 = assign99890_e152072_d_n10;
        locals.var_q_pexa_dn13 = assign99890_e152072_d_n13;

        let assign99900_e152075: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99900_e152075;

        let (assign99910_e152083, assign99910_e152083_d_n0, assign99910_e152083_d_n2, assign99910_e152083_d_n4, assign99910_e152083_d_n5, assign99910_e152083_d_n6, assign99910_e152083_d_n7, assign99910_e152083_d_n8, assign99910_e152083_d_n9, assign99910_e152083_d_n10, assign99910_e152083_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99910_e152081: f64 = (locals.var_q_pexa * p.p543);
        (assign99910_e152081, (locals.var_q_pexa_dn0 * p.p543), (locals.var_q_pexa_dn2 * p.p543), (locals.var_q_pexa_dn4 * p.p543), (locals.var_q_pexa_dn5 * p.p543), (locals.var_q_pexa_dn6 * p.p543), (locals.var_q_pexa_dn7 * p.p543), (locals.var_q_pexa_dn8 * p.p543), (locals.var_q_pexa_dn9 * p.p543), (locals.var_q_pexa_dn10 * p.p543), (locals.var_q_pexa_dn13 * p.p543),)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13,)
    }
};
        locals.var_q_qs_a = assign99910_e152083;
        locals.var_q_qs_a_dn0 = assign99910_e152083_d_n0;
        locals.var_q_qs_a_dn2 = assign99910_e152083_d_n2;
        locals.var_q_qs_a_dn4 = assign99910_e152083_d_n4;
        locals.var_q_qs_a_dn5 = assign99910_e152083_d_n5;
        locals.var_q_qs_a_dn6 = assign99910_e152083_d_n6;
        locals.var_q_qs_a_dn7 = assign99910_e152083_d_n7;
        locals.var_q_qs_a_dn8 = assign99910_e152083_d_n8;
        locals.var_q_qs_a_dn9 = assign99910_e152083_d_n9;
        locals.var_q_qs_a_dn10 = assign99910_e152083_d_n10;
        locals.var_q_qs_a_dn13 = assign99910_e152083_d_n13;

        let (assign99920_e152091, assign99920_e152091_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99920_e152089: f64 = (p.p543 * (nv15 - 0.0));
        (assign99920_e152089, p.p543,)
    } else {
        (locals.var_q_nqs_a, locals.var_q_nqs_a_dn15,)
    }
};
        locals.var_q_nqs_a = assign99920_e152091;
        locals.var_q_nqs_a_dn15 = assign99920_e152091_d_n15;

        let (assign99930_e152101, assign99930_e152101_d_n0, assign99930_e152101_d_n2, assign99930_e152101_d_n4, assign99930_e152101_d_n5, assign99930_e152101_d_n6, assign99930_e152101_d_n7, assign99930_e152101_d_n8, assign99930_e152101_d_n9, assign99930_e152101_d_n10, assign99930_e152101_d_n13, assign99930_e152101_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99930_e152097: f64 = (locals.var_q_nqs_a - locals.var_q_qs_a);
        let assign99930_e152099: f64 = (assign99930_e152097 / p.p543);
        (assign99930_e152099, ((-locals.var_q_qs_a_dn0) / p.p543), ((-locals.var_q_qs_a_dn2) / p.p543), ((-locals.var_q_qs_a_dn4) / p.p543), ((-locals.var_q_qs_a_dn5) / p.p543), ((-locals.var_q_qs_a_dn6) / p.p543), ((-locals.var_q_qs_a_dn7) / p.p543), ((-locals.var_q_qs_a_dn8) / p.p543), ((-locals.var_q_qs_a_dn9) / p.p543), ((-locals.var_q_qs_a_dn10) / p.p543), ((-locals.var_q_qs_a_dn13) / p.p543), (locals.var_q_nqs_a_dn15 / p.p543),)
    } else {
        (locals.var_inqs0_a, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn13, locals.var_inqs0_a_dn15,)
    }
};
        locals.var_inqs0_a = assign99930_e152101;
        locals.var_inqs0_a_dn0 = assign99930_e152101_d_n0;
        locals.var_inqs0_a_dn2 = assign99930_e152101_d_n2;
        locals.var_inqs0_a_dn4 = assign99930_e152101_d_n4;
        locals.var_inqs0_a_dn5 = assign99930_e152101_d_n5;
        locals.var_inqs0_a_dn6 = assign99930_e152101_d_n6;
        locals.var_inqs0_a_dn7 = assign99930_e152101_d_n7;
        locals.var_inqs0_a_dn8 = assign99930_e152101_d_n8;
        locals.var_inqs0_a_dn9 = assign99930_e152101_d_n9;
        locals.var_inqs0_a_dn10 = assign99930_e152101_d_n10;
        locals.var_inqs0_a_dn13 = assign99930_e152101_d_n13;
        locals.var_inqs0_a_dn15 = assign99930_e152101_d_n15;

        let (assign99940_e152109, assign99940_e152109_d_n0, assign99940_e152109_d_n2, assign99940_e152109_d_n4, assign99940_e152109_d_n5, assign99940_e152109_d_n6, assign99940_e152109_d_n7, assign99940_e152109_d_n8, assign99940_e152109_d_n9, assign99940_e152109_d_n10, assign99940_e152109_d_n13, assign99940_e152109_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99940_e152107: f64 = (locals.var_q_nqs_a / p.p543);
        (assign99940_e152107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_a_dn15 / p.p543),)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn13, locals.var_q_pexa_nqs_dn15,)
    }
};
        locals.var_q_pexa_nqs = assign99940_e152109;
        locals.var_q_pexa_nqs_dn0 = assign99940_e152109_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99940_e152109_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99940_e152109_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99940_e152109_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99940_e152109_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99940_e152109_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99940_e152109_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99940_e152109_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99940_e152109_d_n10;
        locals.var_q_pexa_nqs_dn13 = assign99940_e152109_d_n13;
        locals.var_q_pexa_nqs_dn15 = assign99940_e152109_d_n15;

        let (assign99950_e152116, assign99950_e152116_d_n0, assign99950_e152116_d_n2, assign99950_e152116_d_n4, assign99950_e152116_d_n5, assign99950_e152116_d_n6, assign99950_e152116_d_n7, assign99950_e152116_d_n8, assign99950_e152116_d_n9, assign99950_e152116_d_n10, assign99950_e152116_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 == 0.0)) {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn13,)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13,)
    }
};
        locals.var_q_qs_a = assign99950_e152116;
        locals.var_q_qs_a_dn0 = assign99950_e152116_d_n0;
        locals.var_q_qs_a_dn2 = assign99950_e152116_d_n2;
        locals.var_q_qs_a_dn4 = assign99950_e152116_d_n4;
        locals.var_q_qs_a_dn5 = assign99950_e152116_d_n5;
        locals.var_q_qs_a_dn6 = assign99950_e152116_d_n6;
        locals.var_q_qs_a_dn7 = assign99950_e152116_d_n7;
        locals.var_q_qs_a_dn8 = assign99950_e152116_d_n8;
        locals.var_q_qs_a_dn9 = assign99950_e152116_d_n9;
        locals.var_q_qs_a_dn10 = assign99950_e152116_d_n10;
        locals.var_q_qs_a_dn13 = assign99950_e152116_d_n13;

        let (assign99960_e152123, assign99960_e152123_d_n0, assign99960_e152123_d_n2, assign99960_e152123_d_n4, assign99960_e152123_d_n5, assign99960_e152123_d_n6, assign99960_e152123_d_n7, assign99960_e152123_d_n8, assign99960_e152123_d_n9, assign99960_e152123_d_n10, assign99960_e152123_d_n13, assign99960_e152123_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 == 0.0)) {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13, 0.0,)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn13, locals.var_q_pexa_nqs_dn15,)
    }
};
        locals.var_q_pexa_nqs = assign99960_e152123;
        locals.var_q_pexa_nqs_dn0 = assign99960_e152123_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99960_e152123_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99960_e152123_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99960_e152123_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99960_e152123_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99960_e152123_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99960_e152123_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99960_e152123_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99960_e152123_d_n10;
        locals.var_q_pexa_nqs_dn13 = assign99960_e152123_d_n13;
        locals.var_q_pexa_nqs_dn15 = assign99960_e152123_d_n15;

        let assign99970_e152130: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99970_e152130;

        let (assign99980_e152138, assign99980_e152138_d_n0, assign99980_e152138_d_n2, assign99980_e152138_d_n4, assign99980_e152138_d_n5, assign99980_e152138_d_n6, assign99980_e152138_d_n7, assign99980_e152138_d_n8, assign99980_e152138_d_n9, assign99980_e152138_d_n10, assign99980_e152138_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2305 != 0.0)) {
        let assign99980_e152136: f64 = (locals.var_exp_k * p.p541);
        (assign99980_e152136, (locals.var_exp_k_dn0 * p.p541), (locals.var_exp_k_dn2 * p.p541), (locals.var_exp_k_dn4 * p.p541), (locals.var_exp_k_dn5 * p.p541), (locals.var_exp_k_dn6 * p.p541), (locals.var_exp_k_dn7 * p.p541), (locals.var_exp_k_dn8 * p.p541), (locals.var_exp_k_dn9 * p.p541), (locals.var_exp_k_dn10 * p.p541), (locals.var_exp_k_dn13 * p.p541),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign99980_e152138;
        locals.var_exp_k2_dn0 = assign99980_e152138_d_n0;
        locals.var_exp_k2_dn2 = assign99980_e152138_d_n2;
        locals.var_exp_k2_dn4 = assign99980_e152138_d_n4;
        locals.var_exp_k2_dn5 = assign99980_e152138_d_n5;
        locals.var_exp_k2_dn6 = assign99980_e152138_d_n6;
        locals.var_exp_k2_dn7 = assign99980_e152138_d_n7;
        locals.var_exp_k2_dn8 = assign99980_e152138_d_n8;
        locals.var_exp_k2_dn9 = assign99980_e152138_d_n9;
        locals.var_exp_k2_dn10 = assign99980_e152138_d_n10;
        locals.var_exp_k2_dn13 = assign99980_e152138_d_n13;

        let (assign99990_e152167, assign99990_e152167_d_n0, assign99990_e152167_d_n2, assign99990_e152167_d_n4, assign99990_e152167_d_n5, assign99990_e152167_d_n6, assign99990_e152167_d_n7, assign99990_e152167_d_n8, assign99990_e152167_d_n9, assign99990_e152167_d_n10, assign99990_e152167_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2305 == 0.0)) {
        let assign99990_e152145: f64 = (locals.var_exp_k * p.p541);
        let assign99990_e152147: f64 = (-p.p542);
        let assign99990_e152150: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign99990_e152151: f64 = (assign99990_e152147 * assign99990_e152150);
        let assign99990_e152154: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign99990_e152155: f64 = (assign99990_e152151 * assign99990_e152154);
        let assign99990_e152159: f64 = (1.0 / locals.var_tratio);
        let assign99990_e152160: f64 = (assign99990_e152159).ln();
        let assign99990_e152161: f64 = (p.p548 * assign99990_e152160);
        let assign99990_e152162: f64 = (assign99990_e152161).exp();
        let assign99990_e152163: f64 = (assign99990_e152155 * assign99990_e152162);
        let assign99990_e152164: f64 = (assign99990_e152163).exp();
        let assign99990_e152165: f64 = (assign99990_e152145 * assign99990_e152164);
        (assign99990_e152165, (((locals.var_exp_k_dn0 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0)) * assign99990_e152154) + (assign99990_e152151 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn2 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn2)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn2))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn4 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn4)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn4))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn5 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn5)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn5))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn6 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn6)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn6))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn7 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn7)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn7))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn8 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn8)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn8))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn9 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (locals.var_vbd_jct_dn9 - locals.var_v_hk_dn9)) * assign99990_e152154) + (assign99990_e152151 * (locals.var_vbd_jct_dn9 - locals.var_v_hk_dn9))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn10 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn10)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn10))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn13 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn13)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn13))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn13 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign99990_e152167;
        locals.var_exp_k2_dn0 = assign99990_e152167_d_n0;
        locals.var_exp_k2_dn2 = assign99990_e152167_d_n2;
        locals.var_exp_k2_dn4 = assign99990_e152167_d_n4;
        locals.var_exp_k2_dn5 = assign99990_e152167_d_n5;
        locals.var_exp_k2_dn6 = assign99990_e152167_d_n6;
        locals.var_exp_k2_dn7 = assign99990_e152167_d_n7;
        locals.var_exp_k2_dn8 = assign99990_e152167_d_n8;
        locals.var_exp_k2_dn9 = assign99990_e152167_d_n9;
        locals.var_exp_k2_dn10 = assign99990_e152167_d_n10;
        locals.var_exp_k2_dn13 = assign99990_e152167_d_n13;

        let (assign100000_e152176, assign100000_e152176_d_n0, assign100000_e152176_d_n2, assign100000_e152176_d_n4, assign100000_e152176_d_n5, assign100000_e152176_d_n6, assign100000_e152176_d_n7, assign100000_e152176_d_n8, assign100000_e152176_d_n9, assign100000_e152176_d_n10, assign100000_e152176_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign100000_e152174, assign100000_e152174_d_n0, assign100000_e152174_d_n2, assign100000_e152174_d_n4, assign100000_e152174_d_n5, assign100000_e152174_d_n6, assign100000_e152174_d_n7, assign100000_e152174_d_n8, assign100000_e152174_d_n9, assign100000_e152174_d_n10, assign100000_e152174_d_n13,) = {
            if (locals.var_exp_k2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
            }
        };
        (assign100000_e152174, assign100000_e152174_d_n0, assign100000_e152174_d_n2, assign100000_e152174_d_n4, assign100000_e152174_d_n5, assign100000_e152174_d_n6, assign100000_e152174_d_n7, assign100000_e152174_d_n8, assign100000_e152174_d_n9, assign100000_e152174_d_n10, assign100000_e152174_d_n13,)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign100000_e152176;
        locals.var_exp_k2_dn0 = assign100000_e152176_d_n0;
        locals.var_exp_k2_dn2 = assign100000_e152176_d_n2;
        locals.var_exp_k2_dn4 = assign100000_e152176_d_n4;
        locals.var_exp_k2_dn5 = assign100000_e152176_d_n5;
        locals.var_exp_k2_dn6 = assign100000_e152176_d_n6;
        locals.var_exp_k2_dn7 = assign100000_e152176_d_n7;
        locals.var_exp_k2_dn8 = assign100000_e152176_d_n8;
        locals.var_exp_k2_dn9 = assign100000_e152176_d_n9;
        locals.var_exp_k2_dn10 = assign100000_e152176_d_n10;
        locals.var_exp_k2_dn13 = assign100000_e152176_d_n13;

        let (assign100010_e152182, assign100010_e152182_d_n0, assign100010_e152182_d_n2, assign100010_e152182_d_n4, assign100010_e152182_d_n5, assign100010_e152182_d_n6, assign100010_e152182_d_n7, assign100010_e152182_d_n8, assign100010_e152182_d_n9, assign100010_e152182_d_n10, assign100010_e152182_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100010_e152180: f64 = (locals.var_pn0 * locals.var_exp_k2);
        (assign100010_e152180, ((locals.var_pn0_dn0 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn10)), ((locals.var_pn0_dn13 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn13)),)
    } else {
        (locals.var_p_nk, locals.var_p_nk_dn0, locals.var_p_nk_dn2, locals.var_p_nk_dn4, locals.var_p_nk_dn5, locals.var_p_nk_dn6, locals.var_p_nk_dn7, locals.var_p_nk_dn8, locals.var_p_nk_dn9, locals.var_p_nk_dn10, locals.var_p_nk_dn13,)
    }
};
        locals.var_p_nk = assign100010_e152182;
        locals.var_p_nk_dn0 = assign100010_e152182_d_n0;
        locals.var_p_nk_dn2 = assign100010_e152182_d_n2;
        locals.var_p_nk_dn4 = assign100010_e152182_d_n4;
        locals.var_p_nk_dn5 = assign100010_e152182_d_n5;
        locals.var_p_nk_dn6 = assign100010_e152182_d_n6;
        locals.var_p_nk_dn7 = assign100010_e152182_d_n7;
        locals.var_p_nk_dn8 = assign100010_e152182_d_n8;
        locals.var_p_nk_dn9 = assign100010_e152182_d_n9;
        locals.var_p_nk_dn10 = assign100010_e152182_d_n10;
        locals.var_p_nk_dn13 = assign100010_e152182_d_n13;

        let (assign100020_e152192, assign100020_e152192_d_n0, assign100020_e152192_d_n2, assign100020_e152192_d_n4, assign100020_e152192_d_n5, assign100020_e152192_d_n6, assign100020_e152192_d_n7, assign100020_e152192_d_n8, assign100020_e152192_d_n9, assign100020_e152192_d_n10, assign100020_e152192_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100020_e152186: f64 = (1.6021918e-19 * p.p13);
        let assign100020_e152189: f64 = (locals.var_p_nk - locals.var_pn0);
        let assign100020_e152190: f64 = (assign100020_e152186 * assign100020_e152189);
        (assign100020_e152190, (assign100020_e152186 * (locals.var_p_nk_dn0 - locals.var_pn0_dn0)), (assign100020_e152186 * (locals.var_p_nk_dn2 - locals.var_pn0_dn2)), (assign100020_e152186 * (locals.var_p_nk_dn4 - locals.var_pn0_dn4)), (assign100020_e152186 * (locals.var_p_nk_dn5 - locals.var_pn0_dn5)), (assign100020_e152186 * (locals.var_p_nk_dn6 - locals.var_pn0_dn6)), (assign100020_e152186 * (locals.var_p_nk_dn7 - locals.var_pn0_dn7)), (assign100020_e152186 * (locals.var_p_nk_dn8 - locals.var_pn0_dn8)), (assign100020_e152186 * (locals.var_p_nk_dn9 - locals.var_pn0_dn9)), (assign100020_e152186 * (locals.var_p_nk_dn10 - locals.var_pn0_dn10)), (assign100020_e152186 * (locals.var_p_nk_dn13 - locals.var_pn0_dn13)),)
    } else {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn13,)
    }
};
        locals.var_q_pexk = assign100020_e152192;
        locals.var_q_pexk_dn0 = assign100020_e152192_d_n0;
        locals.var_q_pexk_dn2 = assign100020_e152192_d_n2;
        locals.var_q_pexk_dn4 = assign100020_e152192_d_n4;
        locals.var_q_pexk_dn5 = assign100020_e152192_d_n5;
        locals.var_q_pexk_dn6 = assign100020_e152192_d_n6;
        locals.var_q_pexk_dn7 = assign100020_e152192_d_n7;
        locals.var_q_pexk_dn8 = assign100020_e152192_d_n8;
        locals.var_q_pexk_dn9 = assign100020_e152192_d_n9;
        locals.var_q_pexk_dn10 = assign100020_e152192_d_n10;
        locals.var_q_pexk_dn13 = assign100020_e152192_d_n13;

        let assign100030_e152195: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign100030_e152195;

        let (assign100040_e152203, assign100040_e152203_d_n0, assign100040_e152203_d_n2, assign100040_e152203_d_n4, assign100040_e152203_d_n5, assign100040_e152203_d_n6, assign100040_e152203_d_n7, assign100040_e152203_d_n8, assign100040_e152203_d_n9, assign100040_e152203_d_n10, assign100040_e152203_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100040_e152201: f64 = (locals.var_q_pexk * p.p543);
        (assign100040_e152201, (locals.var_q_pexk_dn0 * p.p543), (locals.var_q_pexk_dn2 * p.p543), (locals.var_q_pexk_dn4 * p.p543), (locals.var_q_pexk_dn5 * p.p543), (locals.var_q_pexk_dn6 * p.p543), (locals.var_q_pexk_dn7 * p.p543), (locals.var_q_pexk_dn8 * p.p543), (locals.var_q_pexk_dn9 * p.p543), (locals.var_q_pexk_dn10 * p.p543), (locals.var_q_pexk_dn13 * p.p543),)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13,)
    }
};
        locals.var_q_qs_k = assign100040_e152203;
        locals.var_q_qs_k_dn0 = assign100040_e152203_d_n0;
        locals.var_q_qs_k_dn2 = assign100040_e152203_d_n2;
        locals.var_q_qs_k_dn4 = assign100040_e152203_d_n4;
        locals.var_q_qs_k_dn5 = assign100040_e152203_d_n5;
        locals.var_q_qs_k_dn6 = assign100040_e152203_d_n6;
        locals.var_q_qs_k_dn7 = assign100040_e152203_d_n7;
        locals.var_q_qs_k_dn8 = assign100040_e152203_d_n8;
        locals.var_q_qs_k_dn9 = assign100040_e152203_d_n9;
        locals.var_q_qs_k_dn10 = assign100040_e152203_d_n10;
        locals.var_q_qs_k_dn13 = assign100040_e152203_d_n13;

        let (assign100050_e152211, assign100050_e152211_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100050_e152209: f64 = (p.p543 * (nv16 - 0.0));
        (assign100050_e152209, p.p543,)
    } else {
        (locals.var_q_nqs_k, locals.var_q_nqs_k_dn16,)
    }
};
        locals.var_q_nqs_k = assign100050_e152211;
        locals.var_q_nqs_k_dn16 = assign100050_e152211_d_n16;

        let (assign100060_e152221, assign100060_e152221_d_n0, assign100060_e152221_d_n2, assign100060_e152221_d_n4, assign100060_e152221_d_n5, assign100060_e152221_d_n6, assign100060_e152221_d_n7, assign100060_e152221_d_n8, assign100060_e152221_d_n9, assign100060_e152221_d_n10, assign100060_e152221_d_n13, assign100060_e152221_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100060_e152217: f64 = (locals.var_q_nqs_k - locals.var_q_qs_k);
        let assign100060_e152219: f64 = (assign100060_e152217 / p.p543);
        (assign100060_e152219, ((-locals.var_q_qs_k_dn0) / p.p543), ((-locals.var_q_qs_k_dn2) / p.p543), ((-locals.var_q_qs_k_dn4) / p.p543), ((-locals.var_q_qs_k_dn5) / p.p543), ((-locals.var_q_qs_k_dn6) / p.p543), ((-locals.var_q_qs_k_dn7) / p.p543), ((-locals.var_q_qs_k_dn8) / p.p543), ((-locals.var_q_qs_k_dn9) / p.p543), ((-locals.var_q_qs_k_dn10) / p.p543), ((-locals.var_q_qs_k_dn13) / p.p543), (locals.var_q_nqs_k_dn16 / p.p543),)
    } else {
        (locals.var_inqs0_k, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn13, locals.var_inqs0_k_dn16,)
    }
};
        locals.var_inqs0_k = assign100060_e152221;
        locals.var_inqs0_k_dn0 = assign100060_e152221_d_n0;
        locals.var_inqs0_k_dn2 = assign100060_e152221_d_n2;
        locals.var_inqs0_k_dn4 = assign100060_e152221_d_n4;
        locals.var_inqs0_k_dn5 = assign100060_e152221_d_n5;
        locals.var_inqs0_k_dn6 = assign100060_e152221_d_n6;
        locals.var_inqs0_k_dn7 = assign100060_e152221_d_n7;
        locals.var_inqs0_k_dn8 = assign100060_e152221_d_n8;
        locals.var_inqs0_k_dn9 = assign100060_e152221_d_n9;
        locals.var_inqs0_k_dn10 = assign100060_e152221_d_n10;
        locals.var_inqs0_k_dn13 = assign100060_e152221_d_n13;
        locals.var_inqs0_k_dn16 = assign100060_e152221_d_n16;

        let (assign100070_e152229, assign100070_e152229_d_n0, assign100070_e152229_d_n2, assign100070_e152229_d_n4, assign100070_e152229_d_n5, assign100070_e152229_d_n6, assign100070_e152229_d_n7, assign100070_e152229_d_n8, assign100070_e152229_d_n9, assign100070_e152229_d_n10, assign100070_e152229_d_n13, assign100070_e152229_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100070_e152227: f64 = (locals.var_q_nqs_k / p.p543);
        (assign100070_e152227, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_k_dn16 / p.p543),)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn13, locals.var_q_pexk_nqs_dn16,)
    }
};
        locals.var_q_pexk_nqs = assign100070_e152229;
        locals.var_q_pexk_nqs_dn0 = assign100070_e152229_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100070_e152229_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100070_e152229_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100070_e152229_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100070_e152229_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100070_e152229_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100070_e152229_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100070_e152229_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100070_e152229_d_n10;
        locals.var_q_pexk_nqs_dn13 = assign100070_e152229_d_n13;
        locals.var_q_pexk_nqs_dn16 = assign100070_e152229_d_n16;

        let (assign100080_e152236, assign100080_e152236_d_n0, assign100080_e152236_d_n2, assign100080_e152236_d_n4, assign100080_e152236_d_n5, assign100080_e152236_d_n6, assign100080_e152236_d_n7, assign100080_e152236_d_n8, assign100080_e152236_d_n9, assign100080_e152236_d_n10, assign100080_e152236_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn13,)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13,)
    }
};
        locals.var_q_qs_k = assign100080_e152236;
        locals.var_q_qs_k_dn0 = assign100080_e152236_d_n0;
        locals.var_q_qs_k_dn2 = assign100080_e152236_d_n2;
        locals.var_q_qs_k_dn4 = assign100080_e152236_d_n4;
        locals.var_q_qs_k_dn5 = assign100080_e152236_d_n5;
        locals.var_q_qs_k_dn6 = assign100080_e152236_d_n6;
        locals.var_q_qs_k_dn7 = assign100080_e152236_d_n7;
        locals.var_q_qs_k_dn8 = assign100080_e152236_d_n8;
        locals.var_q_qs_k_dn9 = assign100080_e152236_d_n9;
        locals.var_q_qs_k_dn10 = assign100080_e152236_d_n10;
        locals.var_q_qs_k_dn13 = assign100080_e152236_d_n13;

        let (assign100090_e152243, assign100090_e152243_d_n0, assign100090_e152243_d_n2, assign100090_e152243_d_n4, assign100090_e152243_d_n5, assign100090_e152243_d_n6, assign100090_e152243_d_n7, assign100090_e152243_d_n8, assign100090_e152243_d_n9, assign100090_e152243_d_n10, assign100090_e152243_d_n13, assign100090_e152243_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13, 0.0,)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn13, locals.var_q_pexk_nqs_dn16,)
    }
};
        locals.var_q_pexk_nqs = assign100090_e152243;
        locals.var_q_pexk_nqs_dn0 = assign100090_e152243_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100090_e152243_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100090_e152243_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100090_e152243_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100090_e152243_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100090_e152243_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100090_e152243_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100090_e152243_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100090_e152243_d_n10;
        locals.var_q_pexk_nqs_dn13 = assign100090_e152243_d_n13;
        locals.var_q_pexk_nqs_dn16 = assign100090_e152243_d_n16;

        let (assign100100_e152249, assign100100_e152249_d_n0, assign100100_e152249_d_n2, assign100100_e152249_d_n4, assign100100_e152249_d_n5, assign100100_e152249_d_n6, assign100100_e152249_d_n7, assign100100_e152249_d_n8, assign100100_e152249_d_n9, assign100100_e152249_d_n10, assign100100_e152249_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100100_e152247: f64 = (p.p506 - locals.var_vbd_jct);
        (assign100100_e152247, (-locals.var_vbd_jct_dn0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn9), 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100100_e152249;
        locals.var_vjunc_a_dn0 = assign100100_e152249_d_n0;
        locals.var_vjunc_a_dn2 = assign100100_e152249_d_n2;
        locals.var_vjunc_a_dn4 = assign100100_e152249_d_n4;
        locals.var_vjunc_a_dn5 = assign100100_e152249_d_n5;
        locals.var_vjunc_a_dn6 = assign100100_e152249_d_n6;
        locals.var_vjunc_a_dn7 = assign100100_e152249_d_n7;
        locals.var_vjunc_a_dn8 = assign100100_e152249_d_n8;
        locals.var_vjunc_a_dn9 = assign100100_e152249_d_n9;
        locals.var_vjunc_a_dn10 = assign100100_e152249_d_n10;
        locals.var_vjunc_a_dn13 = assign100100_e152249_d_n13;

        let (assign100110_e152262, assign100110_e152262_d_n0, assign100110_e152262_d_n2, assign100110_e152262_d_n4, assign100110_e152262_d_n5, assign100110_e152262_d_n6, assign100110_e152262_d_n7, assign100110_e152262_d_n8, assign100110_e152262_d_n9, assign100110_e152262_d_n10, assign100110_e152262_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100110_e152253: f64 = (locals.var_vjunc_a * locals.var_vjunc_a);
        let assign100110_e152256: f64 = (4.0 * locals.var_juncdlt);
        let assign100110_e152258: f64 = (assign100110_e152256 * locals.var_juncdlt);
        let assign100110_e152259: f64 = (assign100110_e152253 + assign100110_e152258);
        let assign100110_e152260: f64 = (assign100110_e152259).sqrt();
        (assign100110_e152260, (((locals.var_vjunc_a_dn0 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn0)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn2 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn2)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn4 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn4)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn5 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn5)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn6 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn6)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn7 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn7)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn8 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn8)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn9 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn9)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn10 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn10)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn13 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn13)) / (2.0 * assign100110_e152260)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100110_e152262;
        locals.var_tmf2_dn0 = assign100110_e152262_d_n0;
        locals.var_tmf2_dn2 = assign100110_e152262_d_n2;
        locals.var_tmf2_dn4 = assign100110_e152262_d_n4;
        locals.var_tmf2_dn5 = assign100110_e152262_d_n5;
        locals.var_tmf2_dn6 = assign100110_e152262_d_n6;
        locals.var_tmf2_dn7 = assign100110_e152262_d_n7;
        locals.var_tmf2_dn8 = assign100110_e152262_d_n8;
        locals.var_tmf2_dn9 = assign100110_e152262_d_n9;
        locals.var_tmf2_dn10 = assign100110_e152262_d_n10;
        locals.var_tmf2_dn13 = assign100110_e152262_d_n13;

        let (assign100120_e152272, assign100120_e152272_d_n0, assign100120_e152272_d_n2, assign100120_e152272_d_n4, assign100120_e152272_d_n5, assign100120_e152272_d_n6, assign100120_e152272_d_n7, assign100120_e152272_d_n8, assign100120_e152272_d_n9, assign100120_e152272_d_n10, assign100120_e152272_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100120_e152268: f64 = (locals.var_vjunc_a / locals.var_tmf2);
        let assign100120_e152269: f64 = (1.0 + assign100120_e152268);
        let assign100120_e152270: f64 = (0.5 * assign100120_e152269);
        (assign100120_e152270, (0.5 * (((locals.var_vjunc_a_dn0 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn2 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn4 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn5 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn6 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn7 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn8 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn9 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn10 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn13 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100120_e152272;
        locals.var_t0_dn0 = assign100120_e152272_d_n0;
        locals.var_t0_dn2 = assign100120_e152272_d_n2;
        locals.var_t0_dn4 = assign100120_e152272_d_n4;
        locals.var_t0_dn5 = assign100120_e152272_d_n5;
        locals.var_t0_dn6 = assign100120_e152272_d_n6;
        locals.var_t0_dn7 = assign100120_e152272_d_n7;
        locals.var_t0_dn8 = assign100120_e152272_d_n8;
        locals.var_t0_dn9 = assign100120_e152272_d_n9;
        locals.var_t0_dn10 = assign100120_e152272_d_n10;
        locals.var_t0_dn13 = assign100120_e152272_d_n13;

        let (assign100130_e152280, assign100130_e152280_d_n0, assign100130_e152280_d_n2, assign100130_e152280_d_n4, assign100130_e152280_d_n5, assign100130_e152280_d_n6, assign100130_e152280_d_n7, assign100130_e152280_d_n8, assign100130_e152280_d_n9, assign100130_e152280_d_n10, assign100130_e152280_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100130_e152277: f64 = (locals.var_vjunc_a + locals.var_tmf2);
        let assign100130_e152278: f64 = (0.5 * assign100130_e152277);
        (assign100130_e152278, (0.5 * (locals.var_vjunc_a_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vjunc_a_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vjunc_a_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vjunc_a_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vjunc_a_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vjunc_a_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vjunc_a_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vjunc_a_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vjunc_a_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vjunc_a_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100130_e152280;
        locals.var_vjunc_a_dn0 = assign100130_e152280_d_n0;
        locals.var_vjunc_a_dn2 = assign100130_e152280_d_n2;
        locals.var_vjunc_a_dn4 = assign100130_e152280_d_n4;
        locals.var_vjunc_a_dn5 = assign100130_e152280_d_n5;
        locals.var_vjunc_a_dn6 = assign100130_e152280_d_n6;
        locals.var_vjunc_a_dn7 = assign100130_e152280_d_n7;
        locals.var_vjunc_a_dn8 = assign100130_e152280_d_n8;
        locals.var_vjunc_a_dn9 = assign100130_e152280_d_n9;
        locals.var_vjunc_a_dn10 = assign100130_e152280_d_n10;
        locals.var_vjunc_a_dn13 = assign100130_e152280_d_n13;

        let assign100140_e152283: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign100140_e152283;

        let (assign100150_e152289, assign100150_e152289_d_n0, assign100150_e152289_d_n2, assign100150_e152289_d_n4, assign100150_e152289_d_n5, assign100150_e152289_d_n6, assign100150_e152289_d_n7, assign100150_e152289_d_n8, assign100150_e152289_d_n9, assign100150_e152289_d_n10, assign100150_e152289_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100150_e152289;
        locals.var_vjunc_a_dn0 = assign100150_e152289_d_n0;
        locals.var_vjunc_a_dn2 = assign100150_e152289_d_n2;
        locals.var_vjunc_a_dn4 = assign100150_e152289_d_n4;
        locals.var_vjunc_a_dn5 = assign100150_e152289_d_n5;
        locals.var_vjunc_a_dn6 = assign100150_e152289_d_n6;
        locals.var_vjunc_a_dn7 = assign100150_e152289_d_n7;
        locals.var_vjunc_a_dn8 = assign100150_e152289_d_n8;
        locals.var_vjunc_a_dn9 = assign100150_e152289_d_n9;
        locals.var_vjunc_a_dn10 = assign100150_e152289_d_n10;
        locals.var_vjunc_a_dn13 = assign100150_e152289_d_n13;

        let (assign100160_e152295, assign100160_e152295_d_n0, assign100160_e152295_d_n2, assign100160_e152295_d_n4, assign100160_e152295_d_n5, assign100160_e152295_d_n6, assign100160_e152295_d_n7, assign100160_e152295_d_n8, assign100160_e152295_d_n9, assign100160_e152295_d_n10, assign100160_e152295_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100160_e152295;
        locals.var_t0_dn0 = assign100160_e152295_d_n0;
        locals.var_t0_dn2 = assign100160_e152295_d_n2;
        locals.var_t0_dn4 = assign100160_e152295_d_n4;
        locals.var_t0_dn5 = assign100160_e152295_d_n5;
        locals.var_t0_dn6 = assign100160_e152295_d_n6;
        locals.var_t0_dn7 = assign100160_e152295_d_n7;
        locals.var_t0_dn8 = assign100160_e152295_d_n8;
        locals.var_t0_dn9 = assign100160_e152295_d_n9;
        locals.var_t0_dn10 = assign100160_e152295_d_n10;
        locals.var_t0_dn13 = assign100160_e152295_d_n13;

    }

    pub(super) fn stamp_transient_block_355(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign100170_e152308, assign100170_e152308_d_n0, assign100170_e152308_d_n2, assign100170_e152308_d_n4, assign100170_e152308_d_n5, assign100170_e152308_d_n6, assign100170_e152308_d_n7, assign100170_e152308_d_n8, assign100170_e152308_d_n9, assign100170_e152308_d_n10, assign100170_e152308_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100170_e152299: f64 = (2.0 * 1.034943e-10);
        let assign100170_e152301: f64 = (assign100170_e152299 * locals.var_vjunc_a);
        let assign100170_e152304: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign100170_e152305: f64 = (assign100170_e152301 / assign100170_e152304);
        let assign100170_e152306: f64 = (assign100170_e152305).sqrt();
        (assign100170_e152306, (((assign100170_e152299 * locals.var_vjunc_a_dn0) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn2) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn4) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn5) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn6) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn7) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn8) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn9) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn10) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn13) / assign100170_e152304) / (2.0 * assign100170_e152306)),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    }
};
        locals.var_w_depa = assign100170_e152308;
        locals.var_w_depa_dn0 = assign100170_e152308_d_n0;
        locals.var_w_depa_dn2 = assign100170_e152308_d_n2;
        locals.var_w_depa_dn4 = assign100170_e152308_d_n4;
        locals.var_w_depa_dn5 = assign100170_e152308_d_n5;
        locals.var_w_depa_dn6 = assign100170_e152308_d_n6;
        locals.var_w_depa_dn7 = assign100170_e152308_d_n7;
        locals.var_w_depa_dn8 = assign100170_e152308_d_n8;
        locals.var_w_depa_dn9 = assign100170_e152308_d_n9;
        locals.var_w_depa_dn10 = assign100170_e152308_d_n10;
        locals.var_w_depa_dn13 = assign100170_e152308_d_n13;

        let (assign100180_e152316, assign100180_e152316_d_n0, assign100180_e152316_d_n2, assign100180_e152316_d_n4, assign100180_e152316_d_n5, assign100180_e152316_d_n6, assign100180_e152316_d_n7, assign100180_e152316_d_n8, assign100180_e152316_d_n9, assign100180_e152316_d_n10, assign100180_e152316_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100180_e152312: f64 = (p.p545 - locals.var_w_depa);
        let assign100180_e152314: f64 = (assign100180_e152312 - 1e-7);
        (assign100180_e152314, (-locals.var_w_depa_dn0), (-locals.var_w_depa_dn2), (-locals.var_w_depa_dn4), (-locals.var_w_depa_dn5), (-locals.var_w_depa_dn6), (-locals.var_w_depa_dn7), (-locals.var_w_depa_dn8), (-locals.var_w_depa_dn9), (-locals.var_w_depa_dn10), (-locals.var_w_depa_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign100180_e152316;
        locals.var_tmf1_dn0 = assign100180_e152316_d_n0;
        locals.var_tmf1_dn2 = assign100180_e152316_d_n2;
        locals.var_tmf1_dn4 = assign100180_e152316_d_n4;
        locals.var_tmf1_dn5 = assign100180_e152316_d_n5;
        locals.var_tmf1_dn6 = assign100180_e152316_d_n6;
        locals.var_tmf1_dn7 = assign100180_e152316_d_n7;
        locals.var_tmf1_dn8 = assign100180_e152316_d_n8;
        locals.var_tmf1_dn9 = assign100180_e152316_d_n9;
        locals.var_tmf1_dn10 = assign100180_e152316_d_n10;
        locals.var_tmf1_dn13 = assign100180_e152316_d_n13;

        let (assign100190_e152324, assign100190_e152324_d_n0, assign100190_e152324_d_n2, assign100190_e152324_d_n4, assign100190_e152324_d_n5, assign100190_e152324_d_n6, assign100190_e152324_d_n7, assign100190_e152324_d_n8, assign100190_e152324_d_n9, assign100190_e152324_d_n10, assign100190_e152324_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100190_e152320: f64 = (4.0 * p.p545);
        let assign100190_e152322: f64 = (assign100190_e152320 * 1e-7);
        (assign100190_e152322, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100190_e152324;
        locals.var_tmf2_dn0 = assign100190_e152324_d_n0;
        locals.var_tmf2_dn2 = assign100190_e152324_d_n2;
        locals.var_tmf2_dn4 = assign100190_e152324_d_n4;
        locals.var_tmf2_dn5 = assign100190_e152324_d_n5;
        locals.var_tmf2_dn6 = assign100190_e152324_d_n6;
        locals.var_tmf2_dn7 = assign100190_e152324_d_n7;
        locals.var_tmf2_dn8 = assign100190_e152324_d_n8;
        locals.var_tmf2_dn9 = assign100190_e152324_d_n9;
        locals.var_tmf2_dn10 = assign100190_e152324_d_n10;
        locals.var_tmf2_dn13 = assign100190_e152324_d_n13;

        let (assign100200_e152334, assign100200_e152334_d_n0, assign100200_e152334_d_n2, assign100200_e152334_d_n4, assign100200_e152334_d_n5, assign100200_e152334_d_n6, assign100200_e152334_d_n7, assign100200_e152334_d_n8, assign100200_e152334_d_n9, assign100200_e152334_d_n10, assign100200_e152334_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign100200_e152332, assign100200_e152332_d_n0, assign100200_e152332_d_n2, assign100200_e152332_d_n4, assign100200_e152332_d_n5, assign100200_e152332_d_n6, assign100200_e152332_d_n7, assign100200_e152332_d_n8, assign100200_e152332_d_n9, assign100200_e152332_d_n10, assign100200_e152332_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign100200_e152331: f64 = (-locals.var_tmf2);
                (assign100200_e152331, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign100200_e152332, assign100200_e152332_d_n0, assign100200_e152332_d_n2, assign100200_e152332_d_n4, assign100200_e152332_d_n5, assign100200_e152332_d_n6, assign100200_e152332_d_n7, assign100200_e152332_d_n8, assign100200_e152332_d_n9, assign100200_e152332_d_n10, assign100200_e152332_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100200_e152334;
        locals.var_tmf2_dn0 = assign100200_e152334_d_n0;
        locals.var_tmf2_dn2 = assign100200_e152334_d_n2;
        locals.var_tmf2_dn4 = assign100200_e152334_d_n4;
        locals.var_tmf2_dn5 = assign100200_e152334_d_n5;
        locals.var_tmf2_dn6 = assign100200_e152334_d_n6;
        locals.var_tmf2_dn7 = assign100200_e152334_d_n7;
        locals.var_tmf2_dn8 = assign100200_e152334_d_n8;
        locals.var_tmf2_dn9 = assign100200_e152334_d_n9;
        locals.var_tmf2_dn10 = assign100200_e152334_d_n10;
        locals.var_tmf2_dn13 = assign100200_e152334_d_n13;

        let (assign100210_e152343, assign100210_e152343_d_n0, assign100210_e152343_d_n2, assign100210_e152343_d_n4, assign100210_e152343_d_n5, assign100210_e152343_d_n6, assign100210_e152343_d_n7, assign100210_e152343_d_n8, assign100210_e152343_d_n9, assign100210_e152343_d_n10, assign100210_e152343_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100210_e152338: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign100210_e152340: f64 = (assign100210_e152338 + locals.var_tmf2);
        let assign100210_e152341: f64 = (assign100210_e152340).sqrt();
        (assign100210_e152341, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign100210_e152341)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100210_e152343;
        locals.var_tmf2_dn0 = assign100210_e152343_d_n0;
        locals.var_tmf2_dn2 = assign100210_e152343_d_n2;
        locals.var_tmf2_dn4 = assign100210_e152343_d_n4;
        locals.var_tmf2_dn5 = assign100210_e152343_d_n5;
        locals.var_tmf2_dn6 = assign100210_e152343_d_n6;
        locals.var_tmf2_dn7 = assign100210_e152343_d_n7;
        locals.var_tmf2_dn8 = assign100210_e152343_d_n8;
        locals.var_tmf2_dn9 = assign100210_e152343_d_n9;
        locals.var_tmf2_dn10 = assign100210_e152343_d_n10;
        locals.var_tmf2_dn13 = assign100210_e152343_d_n13;

        let (assign100220_e152353, assign100220_e152353_d_n0, assign100220_e152353_d_n2, assign100220_e152353_d_n4, assign100220_e152353_d_n5, assign100220_e152353_d_n6, assign100220_e152353_d_n7, assign100220_e152353_d_n8, assign100220_e152353_d_n9, assign100220_e152353_d_n10, assign100220_e152353_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100220_e152349: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign100220_e152350: f64 = (1.0 + assign100220_e152349);
        let assign100220_e152351: f64 = (0.5 * assign100220_e152350);
        (assign100220_e152351, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100220_e152353;
        locals.var_t0_dn0 = assign100220_e152353_d_n0;
        locals.var_t0_dn2 = assign100220_e152353_d_n2;
        locals.var_t0_dn4 = assign100220_e152353_d_n4;
        locals.var_t0_dn5 = assign100220_e152353_d_n5;
        locals.var_t0_dn6 = assign100220_e152353_d_n6;
        locals.var_t0_dn7 = assign100220_e152353_d_n7;
        locals.var_t0_dn8 = assign100220_e152353_d_n8;
        locals.var_t0_dn9 = assign100220_e152353_d_n9;
        locals.var_t0_dn10 = assign100220_e152353_d_n10;
        locals.var_t0_dn13 = assign100220_e152353_d_n13;

        let (assign100230_e152363, assign100230_e152363_d_n0, assign100230_e152363_d_n2, assign100230_e152363_d_n4, assign100230_e152363_d_n5, assign100230_e152363_d_n6, assign100230_e152363_d_n7, assign100230_e152363_d_n8, assign100230_e152363_d_n9, assign100230_e152363_d_n10, assign100230_e152363_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100230_e152359: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign100230_e152360: f64 = (0.5 * assign100230_e152359);
        let assign100230_e152361: f64 = (p.p545 - assign100230_e152360);
        (assign100230_e152361, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    }
};
        locals.var_w_depa = assign100230_e152363;
        locals.var_w_depa_dn0 = assign100230_e152363_d_n0;
        locals.var_w_depa_dn2 = assign100230_e152363_d_n2;
        locals.var_w_depa_dn4 = assign100230_e152363_d_n4;
        locals.var_w_depa_dn5 = assign100230_e152363_d_n5;
        locals.var_w_depa_dn6 = assign100230_e152363_d_n6;
        locals.var_w_depa_dn7 = assign100230_e152363_d_n7;
        locals.var_w_depa_dn8 = assign100230_e152363_d_n8;
        locals.var_w_depa_dn9 = assign100230_e152363_d_n9;
        locals.var_w_depa_dn10 = assign100230_e152363_d_n10;
        locals.var_w_depa_dn13 = assign100230_e152363_d_n13;

        let assign100240_e152366: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign100240_e152366;

        let (assign100250_e152374, assign100250_e152374_d_n0, assign100250_e152374_d_n2, assign100250_e152374_d_n4, assign100250_e152374_d_n5, assign100250_e152374_d_n6, assign100250_e152374_d_n7, assign100250_e152374_d_n8, assign100250_e152374_d_n9, assign100250_e152374_d_n10, assign100250_e152374_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100250_e152372: f64 = (locals.var_w_depa * p.p546);
        (assign100250_e152372, (locals.var_w_depa_dn0 * p.p546), (locals.var_w_depa_dn2 * p.p546), (locals.var_w_depa_dn4 * p.p546), (locals.var_w_depa_dn5 * p.p546), (locals.var_w_depa_dn6 * p.p546), (locals.var_w_depa_dn7 * p.p546), (locals.var_w_depa_dn8 * p.p546), (locals.var_w_depa_dn9 * p.p546), (locals.var_w_depa_dn10 * p.p546), (locals.var_w_depa_dn13 * p.p546),)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13,)
    }
};
        locals.var_w_qs_a = assign100250_e152374;
        locals.var_w_qs_a_dn0 = assign100250_e152374_d_n0;
        locals.var_w_qs_a_dn2 = assign100250_e152374_d_n2;
        locals.var_w_qs_a_dn4 = assign100250_e152374_d_n4;
        locals.var_w_qs_a_dn5 = assign100250_e152374_d_n5;
        locals.var_w_qs_a_dn6 = assign100250_e152374_d_n6;
        locals.var_w_qs_a_dn7 = assign100250_e152374_d_n7;
        locals.var_w_qs_a_dn8 = assign100250_e152374_d_n8;
        locals.var_w_qs_a_dn9 = assign100250_e152374_d_n9;
        locals.var_w_qs_a_dn10 = assign100250_e152374_d_n10;
        locals.var_w_qs_a_dn13 = assign100250_e152374_d_n13;

        let (assign100260_e152382, assign100260_e152382_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100260_e152380: f64 = (p.p546 * (nv17 - 0.0));
        (assign100260_e152380, p.p546,)
    } else {
        (locals.var_w_nqs_a, locals.var_w_nqs_a_dn17,)
    }
};
        locals.var_w_nqs_a = assign100260_e152382;
        locals.var_w_nqs_a_dn17 = assign100260_e152382_d_n17;

        let (assign100270_e152392, assign100270_e152392_d_n0, assign100270_e152392_d_n2, assign100270_e152392_d_n4, assign100270_e152392_d_n5, assign100270_e152392_d_n6, assign100270_e152392_d_n7, assign100270_e152392_d_n8, assign100270_e152392_d_n9, assign100270_e152392_d_n10, assign100270_e152392_d_n13, assign100270_e152392_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100270_e152388: f64 = (locals.var_w_nqs_a - locals.var_w_qs_a);
        let assign100270_e152390: f64 = (assign100270_e152388 / p.p546);
        (assign100270_e152390, ((-locals.var_w_qs_a_dn0) / p.p546), ((-locals.var_w_qs_a_dn2) / p.p546), ((-locals.var_w_qs_a_dn4) / p.p546), ((-locals.var_w_qs_a_dn5) / p.p546), ((-locals.var_w_qs_a_dn6) / p.p546), ((-locals.var_w_qs_a_dn7) / p.p546), ((-locals.var_w_qs_a_dn8) / p.p546), ((-locals.var_w_qs_a_dn9) / p.p546), ((-locals.var_w_qs_a_dn10) / p.p546), ((-locals.var_w_qs_a_dn13) / p.p546), (locals.var_w_nqs_a_dn17 / p.p546),)
    } else {
        (locals.var_iwnqs0_a, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn13, locals.var_iwnqs0_a_dn17,)
    }
};
        locals.var_iwnqs0_a = assign100270_e152392;
        locals.var_iwnqs0_a_dn0 = assign100270_e152392_d_n0;
        locals.var_iwnqs0_a_dn2 = assign100270_e152392_d_n2;
        locals.var_iwnqs0_a_dn4 = assign100270_e152392_d_n4;
        locals.var_iwnqs0_a_dn5 = assign100270_e152392_d_n5;
        locals.var_iwnqs0_a_dn6 = assign100270_e152392_d_n6;
        locals.var_iwnqs0_a_dn7 = assign100270_e152392_d_n7;
        locals.var_iwnqs0_a_dn8 = assign100270_e152392_d_n8;
        locals.var_iwnqs0_a_dn9 = assign100270_e152392_d_n9;
        locals.var_iwnqs0_a_dn10 = assign100270_e152392_d_n10;
        locals.var_iwnqs0_a_dn13 = assign100270_e152392_d_n13;
        locals.var_iwnqs0_a_dn17 = assign100270_e152392_d_n17;

        let (assign100280_e152400, assign100280_e152400_d_n0, assign100280_e152400_d_n2, assign100280_e152400_d_n4, assign100280_e152400_d_n5, assign100280_e152400_d_n6, assign100280_e152400_d_n7, assign100280_e152400_d_n8, assign100280_e152400_d_n9, assign100280_e152400_d_n10, assign100280_e152400_d_n13, assign100280_e152400_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100280_e152398: f64 = (locals.var_w_nqs_a / p.p546);
        (assign100280_e152398, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_w_nqs_a_dn17 / p.p546),)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn13, locals.var_w_depa_nqs_dn17,)
    }
};
        locals.var_w_depa_nqs = assign100280_e152400;
        locals.var_w_depa_nqs_dn0 = assign100280_e152400_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100280_e152400_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100280_e152400_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100280_e152400_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100280_e152400_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100280_e152400_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100280_e152400_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100280_e152400_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100280_e152400_d_n10;
        locals.var_w_depa_nqs_dn13 = assign100280_e152400_d_n13;
        locals.var_w_depa_nqs_dn17 = assign100280_e152400_d_n17;

        let (assign100290_e152407, assign100290_e152407_d_n0, assign100290_e152407_d_n2, assign100290_e152407_d_n4, assign100290_e152407_d_n5, assign100290_e152407_d_n6, assign100290_e152407_d_n7, assign100290_e152407_d_n8, assign100290_e152407_d_n9, assign100290_e152407_d_n10, assign100290_e152407_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13,)
    }
};
        locals.var_w_qs_a = assign100290_e152407;
        locals.var_w_qs_a_dn0 = assign100290_e152407_d_n0;
        locals.var_w_qs_a_dn2 = assign100290_e152407_d_n2;
        locals.var_w_qs_a_dn4 = assign100290_e152407_d_n4;
        locals.var_w_qs_a_dn5 = assign100290_e152407_d_n5;
        locals.var_w_qs_a_dn6 = assign100290_e152407_d_n6;
        locals.var_w_qs_a_dn7 = assign100290_e152407_d_n7;
        locals.var_w_qs_a_dn8 = assign100290_e152407_d_n8;
        locals.var_w_qs_a_dn9 = assign100290_e152407_d_n9;
        locals.var_w_qs_a_dn10 = assign100290_e152407_d_n10;
        locals.var_w_qs_a_dn13 = assign100290_e152407_d_n13;

        let (assign100300_e152414, assign100300_e152414_d_n0, assign100300_e152414_d_n2, assign100300_e152414_d_n4, assign100300_e152414_d_n5, assign100300_e152414_d_n6, assign100300_e152414_d_n7, assign100300_e152414_d_n8, assign100300_e152414_d_n9, assign100300_e152414_d_n10, assign100300_e152414_d_n13, assign100300_e152414_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13, 0.0,)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn13, locals.var_w_depa_nqs_dn17,)
    }
};
        locals.var_w_depa_nqs = assign100300_e152414;
        locals.var_w_depa_nqs_dn0 = assign100300_e152414_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100300_e152414_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100300_e152414_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100300_e152414_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100300_e152414_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100300_e152414_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100300_e152414_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100300_e152414_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100300_e152414_d_n10;
        locals.var_w_depa_nqs_dn13 = assign100300_e152414_d_n13;
        locals.var_w_depa_nqs_dn17 = assign100300_e152414_d_n17;

        let (assign100310_e152425,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100310_e152418: f64 = (locals.var_ndi_i * p.p13);
        let assign100310_e152420: f64 = (assign100310_e152418 * 1.6021918e-19);
        let assign100310_e152421: f64 = (-assign100310_e152420);
        let assign100310_e152423: f64 = (assign100310_e152421 * p.p545);
        (assign100310_e152423,)
    } else {
        (locals.var_q_n0,)
    }
};
        locals.var_q_n0 = assign100310_e152425;

        let (assign100320_e152443, assign100320_e152443_d_n0, assign100320_e152443_d_n2, assign100320_e152443_d_n4, assign100320_e152443_d_n5, assign100320_e152443_d_n6, assign100320_e152443_d_n7, assign100320_e152443_d_n8, assign100320_e152443_d_n9, assign100320_e152443_d_n10, assign100320_e152443_d_n13, assign100320_e152443_d_n15, assign100320_e152443_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100320_e152429: f64 = (locals.var_la * locals.var_q_pexa_nqs);
        let assign100320_e152431: f64 = (-p.p545);
        let assign100320_e152433: f64 = (assign100320_e152431 / locals.var_la);
        let assign100320_e152434: f64 = (assign100320_e152433).exp();
        let assign100320_e152436: f64 = (-locals.var_w_depa_nqs);
        let assign100320_e152438: f64 = (assign100320_e152436 / locals.var_la);
        let assign100320_e152439: f64 = (assign100320_e152438).exp();
        let assign100320_e152440: f64 = (assign100320_e152434 - assign100320_e152439);
        let assign100320_e152441: f64 = (assign100320_e152429 * assign100320_e152440);
        (assign100320_e152441, ((((locals.var_la_dn0 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn0)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn0) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn0) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn0)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn2 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn2)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn2) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn2) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn2)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn4 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn4)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn4) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn4) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn4)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn5 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn5)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn5) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn5) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn5)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn6 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn6)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn6) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn6) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn6)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn7 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn7)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn7) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn7) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn7)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn8 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn8)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn8) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn8) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn8)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn9 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn9)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn9) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn9) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn9)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn10 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn10)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn10) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn10) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn10)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn13 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn13)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn13) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn13) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn13)) / (locals.var_la * locals.var_la)))))), ((locals.var_la * locals.var_q_pexa_nqs_dn15) * assign100320_e152440), (assign100320_e152429 * (-(assign100320_e152439 * ((-locals.var_w_depa_nqs_dn17) / locals.var_la)))),)
    } else {
        (locals.var_q_nexa_nqs, locals.var_q_nexa_nqs_dn0, locals.var_q_nexa_nqs_dn2, locals.var_q_nexa_nqs_dn4, locals.var_q_nexa_nqs_dn5, locals.var_q_nexa_nqs_dn6, locals.var_q_nexa_nqs_dn7, locals.var_q_nexa_nqs_dn8, locals.var_q_nexa_nqs_dn9, locals.var_q_nexa_nqs_dn10, locals.var_q_nexa_nqs_dn13, locals.var_q_nexa_nqs_dn15, locals.var_q_nexa_nqs_dn17,)
    }
};
        locals.var_q_nexa_nqs = assign100320_e152443;
        locals.var_q_nexa_nqs_dn0 = assign100320_e152443_d_n0;
        locals.var_q_nexa_nqs_dn2 = assign100320_e152443_d_n2;
        locals.var_q_nexa_nqs_dn4 = assign100320_e152443_d_n4;
        locals.var_q_nexa_nqs_dn5 = assign100320_e152443_d_n5;
        locals.var_q_nexa_nqs_dn6 = assign100320_e152443_d_n6;
        locals.var_q_nexa_nqs_dn7 = assign100320_e152443_d_n7;
        locals.var_q_nexa_nqs_dn8 = assign100320_e152443_d_n8;
        locals.var_q_nexa_nqs_dn9 = assign100320_e152443_d_n9;
        locals.var_q_nexa_nqs_dn10 = assign100320_e152443_d_n10;
        locals.var_q_nexa_nqs_dn13 = assign100320_e152443_d_n13;
        locals.var_q_nexa_nqs_dn15 = assign100320_e152443_d_n15;
        locals.var_q_nexa_nqs_dn17 = assign100320_e152443_d_n17;

        let (assign100330_e152459, assign100330_e152459_d_n0, assign100330_e152459_d_n2, assign100330_e152459_d_n4, assign100330_e152459_d_n5, assign100330_e152459_d_n6, assign100330_e152459_d_n7, assign100330_e152459_d_n8, assign100330_e152459_d_n9, assign100330_e152459_d_n10, assign100330_e152459_d_n13, assign100330_e152459_d_n16, assign100330_e152459_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100330_e152447: f64 = (locals.var_la * locals.var_q_pexk_nqs);
        let assign100330_e152450: f64 = (p.p545 - locals.var_w_depa_nqs);
        let assign100330_e152451: f64 = (-assign100330_e152450);
        let assign100330_e152453: f64 = (assign100330_e152451 / locals.var_la);
        let assign100330_e152454: f64 = (assign100330_e152453).exp();
        let assign100330_e152456: f64 = (assign100330_e152454 - 1.0);
        let assign100330_e152457: f64 = (assign100330_e152447 * assign100330_e152456);
        (assign100330_e152457, ((((locals.var_la_dn0 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn0)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn0)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn0)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn2 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn2)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn2)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn2)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn4 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn4)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn4)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn4)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn5 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn5)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn5)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn5)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn6 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn6)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn6)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn6)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn7 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn7)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn7)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn7)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn8 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn8)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn8)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn8)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn9 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn9)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn9)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn9)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn10 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn10)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn10)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn10)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn13 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn13)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn13)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn13)) / (locals.var_la * locals.var_la))))), ((locals.var_la * locals.var_q_pexk_nqs_dn16) * assign100330_e152456), (assign100330_e152447 * (assign100330_e152454 * ((-(-locals.var_w_depa_nqs_dn17)) / locals.var_la))),)
    } else {
        (locals.var_q_nexk_nqs, locals.var_q_nexk_nqs_dn0, locals.var_q_nexk_nqs_dn2, locals.var_q_nexk_nqs_dn4, locals.var_q_nexk_nqs_dn5, locals.var_q_nexk_nqs_dn6, locals.var_q_nexk_nqs_dn7, locals.var_q_nexk_nqs_dn8, locals.var_q_nexk_nqs_dn9, locals.var_q_nexk_nqs_dn10, locals.var_q_nexk_nqs_dn13, locals.var_q_nexk_nqs_dn16, locals.var_q_nexk_nqs_dn17,)
    }
};
        locals.var_q_nexk_nqs = assign100330_e152459;
        locals.var_q_nexk_nqs_dn0 = assign100330_e152459_d_n0;
        locals.var_q_nexk_nqs_dn2 = assign100330_e152459_d_n2;
        locals.var_q_nexk_nqs_dn4 = assign100330_e152459_d_n4;
        locals.var_q_nexk_nqs_dn5 = assign100330_e152459_d_n5;
        locals.var_q_nexk_nqs_dn6 = assign100330_e152459_d_n6;
        locals.var_q_nexk_nqs_dn7 = assign100330_e152459_d_n7;
        locals.var_q_nexk_nqs_dn8 = assign100330_e152459_d_n8;
        locals.var_q_nexk_nqs_dn9 = assign100330_e152459_d_n9;
        locals.var_q_nexk_nqs_dn10 = assign100330_e152459_d_n10;
        locals.var_q_nexk_nqs_dn13 = assign100330_e152459_d_n13;
        locals.var_q_nexk_nqs_dn16 = assign100330_e152459_d_n16;
        locals.var_q_nexk_nqs_dn17 = assign100330_e152459_d_n17;

        let (assign100340_e152468, assign100340_e152468_d_n0, assign100340_e152468_d_n2, assign100340_e152468_d_n4, assign100340_e152468_d_n5, assign100340_e152468_d_n6, assign100340_e152468_d_n7, assign100340_e152468_d_n8, assign100340_e152468_d_n9, assign100340_e152468_d_n10, assign100340_e152468_d_n13, assign100340_e152468_d_n15, assign100340_e152468_d_n16, assign100340_e152468_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100340_e152463: f64 = (locals.var_q_n0 + locals.var_q_nexa_nqs);
        let assign100340_e152465: f64 = (assign100340_e152463 + locals.var_q_nexk_nqs);
        let assign100340_e152466: f64 = (-assign100340_e152465);
        (assign100340_e152466, (-(locals.var_q_nexa_nqs_dn0 + locals.var_q_nexk_nqs_dn0)), (-(locals.var_q_nexa_nqs_dn2 + locals.var_q_nexk_nqs_dn2)), (-(locals.var_q_nexa_nqs_dn4 + locals.var_q_nexk_nqs_dn4)), (-(locals.var_q_nexa_nqs_dn5 + locals.var_q_nexk_nqs_dn5)), (-(locals.var_q_nexa_nqs_dn6 + locals.var_q_nexk_nqs_dn6)), (-(locals.var_q_nexa_nqs_dn7 + locals.var_q_nexk_nqs_dn7)), (-(locals.var_q_nexa_nqs_dn8 + locals.var_q_nexk_nqs_dn8)), (-(locals.var_q_nexa_nqs_dn9 + locals.var_q_nexk_nqs_dn9)), (-(locals.var_q_nexa_nqs_dn10 + locals.var_q_nexk_nqs_dn10)), (-(locals.var_q_nexa_nqs_dn13 + locals.var_q_nexk_nqs_dn13)), (-locals.var_q_nexa_nqs_dn15), (-locals.var_q_nexk_nqs_dn16), (-(locals.var_q_nexa_nqs_dn17 + locals.var_q_nexk_nqs_dn17)),)
    } else {
        (locals.var_qrr, locals.var_qrr_dn0, locals.var_qrr_dn2, locals.var_qrr_dn4, locals.var_qrr_dn5, locals.var_qrr_dn6, locals.var_qrr_dn7, locals.var_qrr_dn8, locals.var_qrr_dn9, locals.var_qrr_dn10, locals.var_qrr_dn13, locals.var_qrr_dn15, locals.var_qrr_dn16, locals.var_qrr_dn17,)
    }
};
        locals.var_qrr = assign100340_e152468;
        locals.var_qrr_dn0 = assign100340_e152468_d_n0;
        locals.var_qrr_dn2 = assign100340_e152468_d_n2;
        locals.var_qrr_dn4 = assign100340_e152468_d_n4;
        locals.var_qrr_dn5 = assign100340_e152468_d_n5;
        locals.var_qrr_dn6 = assign100340_e152468_d_n6;
        locals.var_qrr_dn7 = assign100340_e152468_d_n7;
        locals.var_qrr_dn8 = assign100340_e152468_d_n8;
        locals.var_qrr_dn9 = assign100340_e152468_d_n9;
        locals.var_qrr_dn10 = assign100340_e152468_d_n10;
        locals.var_qrr_dn13 = assign100340_e152468_d_n13;
        locals.var_qrr_dn15 = assign100340_e152468_d_n15;
        locals.var_qrr_dn16 = assign100340_e152468_d_n16;
        locals.var_qrr_dn17 = assign100340_e152468_d_n17;

        let (assign100350_e152476, assign100350_e152476_d_n0, assign100350_e152476_d_n2, assign100350_e152476_d_n4, assign100350_e152476_d_n5, assign100350_e152476_d_n6, assign100350_e152476_d_n7, assign100350_e152476_d_n8, assign100350_e152476_d_n9, assign100350_e152476_d_n10, assign100350_e152476_d_n13, assign100350_e152476_d_n15, assign100350_e152476_d_n16, assign100350_e152476_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100350_e152473: f64 = (locals.var_mfactor * locals.var_qrr);
        let assign100350_e152474: f64 = (locals.var_qbd + assign100350_e152473);
        (assign100350_e152474, (locals.var_qbd_dn0 + (locals.var_mfactor * locals.var_qrr_dn0)), (locals.var_qbd_dn2 + (locals.var_mfactor * locals.var_qrr_dn2)), (locals.var_qbd_dn4 + (locals.var_mfactor * locals.var_qrr_dn4)), (locals.var_qbd_dn5 + (locals.var_mfactor * locals.var_qrr_dn5)), (locals.var_qbd_dn6 + (locals.var_mfactor * locals.var_qrr_dn6)), (locals.var_qbd_dn7 + (locals.var_mfactor * locals.var_qrr_dn7)), (locals.var_qbd_dn8 + (locals.var_mfactor * locals.var_qrr_dn8)), (locals.var_qbd_dn9 + (locals.var_mfactor * locals.var_qrr_dn9)), (locals.var_qbd_dn10 + (locals.var_mfactor * locals.var_qrr_dn10)), (locals.var_qbd_dn13 + (locals.var_mfactor * locals.var_qrr_dn13)), (locals.var_qbd_dn15 + (locals.var_mfactor * locals.var_qrr_dn15)), (locals.var_qbd_dn16 + (locals.var_mfactor * locals.var_qrr_dn16)), (locals.var_qbd_dn17 + (locals.var_mfactor * locals.var_qrr_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign100350_e152476;
        locals.var_qbd_dn0 = assign100350_e152476_d_n0;
        locals.var_qbd_dn2 = assign100350_e152476_d_n2;
        locals.var_qbd_dn4 = assign100350_e152476_d_n4;
        locals.var_qbd_dn5 = assign100350_e152476_d_n5;
        locals.var_qbd_dn6 = assign100350_e152476_d_n6;
        locals.var_qbd_dn7 = assign100350_e152476_d_n7;
        locals.var_qbd_dn8 = assign100350_e152476_d_n8;
        locals.var_qbd_dn9 = assign100350_e152476_d_n9;
        locals.var_qbd_dn10 = assign100350_e152476_d_n10;
        locals.var_qbd_dn13 = assign100350_e152476_d_n13;
        locals.var_qbd_dn15 = assign100350_e152476_d_n15;
        locals.var_qbd_dn16 = assign100350_e152476_d_n16;
        locals.var_qbd_dn17 = assign100350_e152476_d_n17;

        let assign100360_e152483: f64 = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100360_e152483;

        let assign100370_e152490: f64 = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100370_e152490;

        let assign100380_e152493: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100380_e152493;

        let assign100390_e152500: f64 = if ((locals.var_uc_sub1snp > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100390_e152500;

        let (assign100400_e152508, assign100400_e152508_d_n0, assign100400_e152508_d_n2, assign100400_e152508_d_n4, assign100400_e152508_d_n5, assign100400_e152508_d_n6, assign100400_e152508_d_n7, assign100400_e152508_d_n8, assign100400_e152508_d_n9, assign100400_e152508_d_n10, assign100400_e152508_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100400_e152506: f64 = (locals.var_vg2const_1 * locals.var_vgp);
        (assign100400_e152506, ((locals.var_vg2const_1_dn0 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn0)), ((locals.var_vg2const_1_dn2 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn2)), ((locals.var_vg2const_1_dn4 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn4)), ((locals.var_vg2const_1_dn5 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn5)), ((locals.var_vg2const_1_dn6 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn6)), ((locals.var_vg2const_1_dn7 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn7)), ((locals.var_vg2const_1_dn8 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn8)), ((locals.var_vg2const_1_dn9 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn9)), ((locals.var_vg2const_1_dn10 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn10)), ((locals.var_vg2const_1_dn13 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign100400_e152508;
        locals.var_t1_dn0 = assign100400_e152508_d_n0;
        locals.var_t1_dn2 = assign100400_e152508_d_n2;
        locals.var_t1_dn4 = assign100400_e152508_d_n4;
        locals.var_t1_dn5 = assign100400_e152508_d_n5;
        locals.var_t1_dn6 = assign100400_e152508_d_n6;
        locals.var_t1_dn7 = assign100400_e152508_d_n7;
        locals.var_t1_dn8 = assign100400_e152508_d_n8;
        locals.var_t1_dn9 = assign100400_e152508_d_n9;
        locals.var_t1_dn10 = assign100400_e152508_d_n10;
        locals.var_t1_dn13 = assign100400_e152508_d_n13;

        let (assign100410_e152518, assign100410_e152518_d_n0, assign100410_e152518_d_n2, assign100410_e152518_d_n4, assign100410_e152518_d_n5, assign100410_e152518_d_n6, assign100410_e152518_d_n7, assign100410_e152518_d_n8, assign100410_e152518_d_n9, assign100410_e152518_d_n10, assign100410_e152518_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100410_e152515: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100410_e152516: f64 = (locals.var_qnsub_esi / assign100410_e152515);
        (assign100410_e152516, (locals.var_qnsub_esi_dn0 / assign100410_e152515), (locals.var_qnsub_esi_dn2 / assign100410_e152515), (locals.var_qnsub_esi_dn4 / assign100410_e152515), (locals.var_qnsub_esi_dn5 / assign100410_e152515), (locals.var_qnsub_esi_dn6 / assign100410_e152515), (locals.var_qnsub_esi_dn7 / assign100410_e152515), (locals.var_qnsub_esi_dn8 / assign100410_e152515), (locals.var_qnsub_esi_dn9 / assign100410_e152515), (locals.var_qnsub_esi_dn10 / assign100410_e152515), (locals.var_qnsub_esi_dn13 / assign100410_e152515),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign100410_e152518;
        locals.var_t3_dn0 = assign100410_e152518_d_n0;
        locals.var_t3_dn2 = assign100410_e152518_d_n2;
        locals.var_t3_dn4 = assign100410_e152518_d_n4;
        locals.var_t3_dn5 = assign100410_e152518_d_n5;
        locals.var_t3_dn6 = assign100410_e152518_d_n6;
        locals.var_t3_dn7 = assign100410_e152518_d_n7;
        locals.var_t3_dn8 = assign100410_e152518_d_n8;
        locals.var_t3_dn9 = assign100410_e152518_d_n9;
        locals.var_t3_dn10 = assign100410_e152518_d_n10;
        locals.var_t3_dn13 = assign100410_e152518_d_n13;

        let (assign100420_e152530, assign100420_e152530_d_n0, assign100420_e152530_d_n2, assign100420_e152530_d_n4, assign100420_e152530_d_n5, assign100420_e152530_d_n6, assign100420_e152530_d_n7, assign100420_e152530_d_n8, assign100420_e152530_d_n9, assign100420_e152530_d_n10, assign100420_e152530_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100420_e152524: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100420_e152527: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100420_e152528: f64 = (assign100420_e152524 * assign100420_e152527);
        (assign100420_e152528, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn13) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign100420_e152530;
        locals.var_t4_dn0 = assign100420_e152530_d_n0;
        locals.var_t4_dn2 = assign100420_e152530_d_n2;
        locals.var_t4_dn4 = assign100420_e152530_d_n4;
        locals.var_t4_dn5 = assign100420_e152530_d_n5;
        locals.var_t4_dn6 = assign100420_e152530_d_n6;
        locals.var_t4_dn7 = assign100420_e152530_d_n7;
        locals.var_t4_dn8 = assign100420_e152530_d_n8;
        locals.var_t4_dn9 = assign100420_e152530_d_n9;
        locals.var_t4_dn10 = assign100420_e152530_d_n10;
        locals.var_t4_dn13 = assign100420_e152530_d_n13;

        let (assign100430_e152542, assign100430_e152542_d_n0, assign100430_e152542_d_n2, assign100430_e152542_d_n4, assign100430_e152542_d_n5, assign100430_e152542_d_n6, assign100430_e152542_d_n7, assign100430_e152542_d_n8, assign100430_e152542_d_n9, assign100430_e152542_d_n10, assign100430_e152542_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100430_e152536: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100430_e152539: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk438);
        let assign100430_e152540: f64 = (assign100430_e152536 - assign100430_e152539);
        (assign100430_e152540, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn10)), ((locals.var_t1_dn13 - locals.var_beta_inv_dn13) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign100430_e152542;
        locals.var_t5_dn0 = assign100430_e152542_d_n0;
        locals.var_t5_dn2 = assign100430_e152542_d_n2;
        locals.var_t5_dn4 = assign100430_e152542_d_n4;
        locals.var_t5_dn5 = assign100430_e152542_d_n5;
        locals.var_t5_dn6 = assign100430_e152542_d_n6;
        locals.var_t5_dn7 = assign100430_e152542_d_n7;
        locals.var_t5_dn8 = assign100430_e152542_d_n8;
        locals.var_t5_dn9 = assign100430_e152542_d_n9;
        locals.var_t5_dn10 = assign100430_e152542_d_n10;
        locals.var_t5_dn13 = assign100430_e152542_d_n13;

    }

    pub(super) fn stamp_transient_block_356(
        locals: &mut StampLocals,
    ) {
        let (assign100440_e152552, assign100440_e152552_d_n0, assign100440_e152552_d_n2, assign100440_e152552_d_n4, assign100440_e152552_d_n5, assign100440_e152552_d_n6, assign100440_e152552_d_n7, assign100440_e152552_d_n8, assign100440_e152552_d_n9, assign100440_e152552_d_n10, assign100440_e152552_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100440_e152549: f64 = (locals.var_t4 * locals.var_t5);
        let assign100440_e152550: f64 = (1.0 + assign100440_e152549);
        (assign100440_e152550, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100440_e152552;
        locals.var_t6_dn0 = assign100440_e152552_d_n0;
        locals.var_t6_dn2 = assign100440_e152552_d_n2;
        locals.var_t6_dn4 = assign100440_e152552_d_n4;
        locals.var_t6_dn5 = assign100440_e152552_d_n5;
        locals.var_t6_dn6 = assign100440_e152552_d_n6;
        locals.var_t6_dn7 = assign100440_e152552_d_n7;
        locals.var_t6_dn8 = assign100440_e152552_d_n8;
        locals.var_t6_dn9 = assign100440_e152552_d_n9;
        locals.var_t6_dn10 = assign100440_e152552_d_n10;
        locals.var_t6_dn13 = assign100440_e152552_d_n13;

        let (assign100450_e152562, assign100450_e152562_d_n0, assign100450_e152562_d_n2, assign100450_e152562_d_n4, assign100450_e152562_d_n5, assign100450_e152562_d_n6, assign100450_e152562_d_n7, assign100450_e152562_d_n8, assign100450_e152562_d_n9, assign100450_e152562_d_n10, assign100450_e152562_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100450_e152559: f64 = (1.0 + locals.var_t4);
        let assign100450_e152560: f64 = (2.0 * assign100450_e152559);
        (assign100450_e152560, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign100450_e152562;
        locals.var_t7_dn0 = assign100450_e152562_d_n0;
        locals.var_t7_dn2 = assign100450_e152562_d_n2;
        locals.var_t7_dn4 = assign100450_e152562_d_n4;
        locals.var_t7_dn5 = assign100450_e152562_d_n5;
        locals.var_t7_dn6 = assign100450_e152562_d_n6;
        locals.var_t7_dn7 = assign100450_e152562_d_n7;
        locals.var_t7_dn8 = assign100450_e152562_d_n8;
        locals.var_t7_dn9 = assign100450_e152562_d_n9;
        locals.var_t7_dn10 = assign100450_e152562_d_n10;
        locals.var_t7_dn13 = assign100450_e152562_d_n13;

        let assign100460_e152566: f64 = locals.var_t7;
        let assign100460_e152571: f64 = if ((locals.var_t6 < assign100460_e152566) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2313 = assign100460_e152571;

        let (assign100470_e152583, assign100470_e152583_d_n0, assign100470_e152583_d_n2, assign100470_e152583_d_n4, assign100470_e152583_d_n5, assign100470_e152583_d_n6, assign100470_e152583_d_n7, assign100470_e152583_d_n8, assign100470_e152583_d_n9, assign100470_e152583_d_n10, assign100470_e152583_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100470_e152579: f64 = locals.var_t7;
        let assign100470_e152581: f64 = (assign100470_e152579 - locals.var_t6);
        (assign100470_e152581, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn13 - locals.var_t6_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign100470_e152583;
        locals.var_tmf1_dn0 = assign100470_e152583_d_n0;
        locals.var_tmf1_dn2 = assign100470_e152583_d_n2;
        locals.var_tmf1_dn4 = assign100470_e152583_d_n4;
        locals.var_tmf1_dn5 = assign100470_e152583_d_n5;
        locals.var_tmf1_dn6 = assign100470_e152583_d_n6;
        locals.var_tmf1_dn7 = assign100470_e152583_d_n7;
        locals.var_tmf1_dn8 = assign100470_e152583_d_n8;
        locals.var_tmf1_dn9 = assign100470_e152583_d_n9;
        locals.var_tmf1_dn10 = assign100470_e152583_d_n10;
        locals.var_tmf1_dn13 = assign100470_e152583_d_n13;

        let (assign100480_e152593, assign100480_e152593_d_n0, assign100480_e152593_d_n2, assign100480_e152593_d_n4, assign100480_e152593_d_n5, assign100480_e152593_d_n6, assign100480_e152593_d_n7, assign100480_e152593_d_n8, assign100480_e152593_d_n9, assign100480_e152593_d_n10, assign100480_e152593_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100480_e152591: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100480_e152591, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign100480_e152593;
        locals.var_x2_dn0 = assign100480_e152593_d_n0;
        locals.var_x2_dn2 = assign100480_e152593_d_n2;
        locals.var_x2_dn4 = assign100480_e152593_d_n4;
        locals.var_x2_dn5 = assign100480_e152593_d_n5;
        locals.var_x2_dn6 = assign100480_e152593_d_n6;
        locals.var_x2_dn7 = assign100480_e152593_d_n7;
        locals.var_x2_dn8 = assign100480_e152593_d_n8;
        locals.var_x2_dn9 = assign100480_e152593_d_n9;
        locals.var_x2_dn10 = assign100480_e152593_d_n10;
        locals.var_x2_dn13 = assign100480_e152593_d_n13;

        let (assign100490_e152603, assign100490_e152603_d_n0, assign100490_e152603_d_n2, assign100490_e152603_d_n4, assign100490_e152603_d_n5, assign100490_e152603_d_n6, assign100490_e152603_d_n7, assign100490_e152603_d_n8, assign100490_e152603_d_n9, assign100490_e152603_d_n10, assign100490_e152603_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100490_e152601: f64 = (locals.var_t7 * locals.var_t7);
        (assign100490_e152601, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign100490_e152603;
        locals.var_xmax2_dn0 = assign100490_e152603_d_n0;
        locals.var_xmax2_dn2 = assign100490_e152603_d_n2;
        locals.var_xmax2_dn4 = assign100490_e152603_d_n4;
        locals.var_xmax2_dn5 = assign100490_e152603_d_n5;
        locals.var_xmax2_dn6 = assign100490_e152603_d_n6;
        locals.var_xmax2_dn7 = assign100490_e152603_d_n7;
        locals.var_xmax2_dn8 = assign100490_e152603_d_n8;
        locals.var_xmax2_dn9 = assign100490_e152603_d_n9;
        locals.var_xmax2_dn10 = assign100490_e152603_d_n10;
        locals.var_xmax2_dn13 = assign100490_e152603_d_n13;

        let (assign100500_e152611, assign100500_e152611_d_n0, assign100500_e152611_d_n2, assign100500_e152611_d_n4, assign100500_e152611_d_n5, assign100500_e152611_d_n6, assign100500_e152611_d_n7, assign100500_e152611_d_n8, assign100500_e152611_d_n9, assign100500_e152611_d_n10, assign100500_e152611_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100500_e152611;
        locals.var_xp_dn0 = assign100500_e152611_d_n0;
        locals.var_xp_dn2 = assign100500_e152611_d_n2;
        locals.var_xp_dn4 = assign100500_e152611_d_n4;
        locals.var_xp_dn5 = assign100500_e152611_d_n5;
        locals.var_xp_dn6 = assign100500_e152611_d_n6;
        locals.var_xp_dn7 = assign100500_e152611_d_n7;
        locals.var_xp_dn8 = assign100500_e152611_d_n8;
        locals.var_xp_dn9 = assign100500_e152611_d_n9;
        locals.var_xp_dn10 = assign100500_e152611_d_n10;
        locals.var_xp_dn13 = assign100500_e152611_d_n13;

        let (assign100510_e152619, assign100510_e152619_d_n0, assign100510_e152619_d_n2, assign100510_e152619_d_n4, assign100510_e152619_d_n5, assign100510_e152619_d_n6, assign100510_e152619_d_n7, assign100510_e152619_d_n8, assign100510_e152619_d_n9, assign100510_e152619_d_n10, assign100510_e152619_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100510_e152619;
        locals.var_xmp_dn0 = assign100510_e152619_d_n0;
        locals.var_xmp_dn2 = assign100510_e152619_d_n2;
        locals.var_xmp_dn4 = assign100510_e152619_d_n4;
        locals.var_xmp_dn5 = assign100510_e152619_d_n5;
        locals.var_xmp_dn6 = assign100510_e152619_d_n6;
        locals.var_xmp_dn7 = assign100510_e152619_d_n7;
        locals.var_xmp_dn8 = assign100510_e152619_d_n8;
        locals.var_xmp_dn9 = assign100510_e152619_d_n9;
        locals.var_xmp_dn10 = assign100510_e152619_d_n10;
        locals.var_xmp_dn13 = assign100510_e152619_d_n13;

        let (assign100520_e152627,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100520_e152627;

        let (assign100530_e152635,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100530_e152635;

        let (assign100540_e152643, assign100540_e152643_d_n0, assign100540_e152643_d_n2, assign100540_e152643_d_n4, assign100540_e152643_d_n5, assign100540_e152643_d_n6, assign100540_e152643_d_n7, assign100540_e152643_d_n8, assign100540_e152643_d_n9, assign100540_e152643_d_n10, assign100540_e152643_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign100540_e152643;
        locals.var_arg_dn0 = assign100540_e152643_d_n0;
        locals.var_arg_dn2 = assign100540_e152643_d_n2;
        locals.var_arg_dn4 = assign100540_e152643_d_n4;
        locals.var_arg_dn5 = assign100540_e152643_d_n5;
        locals.var_arg_dn6 = assign100540_e152643_d_n6;
        locals.var_arg_dn7 = assign100540_e152643_d_n7;
        locals.var_arg_dn8 = assign100540_e152643_d_n8;
        locals.var_arg_dn9 = assign100540_e152643_d_n9;
        locals.var_arg_dn10 = assign100540_e152643_d_n10;
        locals.var_arg_dn13 = assign100540_e152643_d_n13;

        let (assign100550_e152651, assign100550_e152651_d_n0, assign100550_e152651_d_n2, assign100550_e152651_d_n4, assign100550_e152651_d_n5, assign100550_e152651_d_n6, assign100550_e152651_d_n7, assign100550_e152651_d_n8, assign100550_e152651_d_n9, assign100550_e152651_d_n10, assign100550_e152651_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100550_e152651;
        locals.var_dnm_dn0 = assign100550_e152651_d_n0;
        locals.var_dnm_dn2 = assign100550_e152651_d_n2;
        locals.var_dnm_dn4 = assign100550_e152651_d_n4;
        locals.var_dnm_dn5 = assign100550_e152651_d_n5;
        locals.var_dnm_dn6 = assign100550_e152651_d_n6;
        locals.var_dnm_dn7 = assign100550_e152651_d_n7;
        locals.var_dnm_dn8 = assign100550_e152651_d_n8;
        locals.var_dnm_dn9 = assign100550_e152651_d_n9;
        locals.var_dnm_dn10 = assign100550_e152651_d_n10;
        locals.var_dnm_dn13 = assign100550_e152651_d_n13;

        let (assign100560_e152661, assign100560_e152661_d_n0, assign100560_e152661_d_n2, assign100560_e152661_d_n4, assign100560_e152661_d_n5, assign100560_e152661_d_n6, assign100560_e152661_d_n7, assign100560_e152661_d_n8, assign100560_e152661_d_n9, assign100560_e152661_d_n10, assign100560_e152661_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100560_e152659: f64 = (locals.var_xp * locals.var_x2);
        (assign100560_e152659, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100560_e152661;
        locals.var_xp_dn0 = assign100560_e152661_d_n0;
        locals.var_xp_dn2 = assign100560_e152661_d_n2;
        locals.var_xp_dn4 = assign100560_e152661_d_n4;
        locals.var_xp_dn5 = assign100560_e152661_d_n5;
        locals.var_xp_dn6 = assign100560_e152661_d_n6;
        locals.var_xp_dn7 = assign100560_e152661_d_n7;
        locals.var_xp_dn8 = assign100560_e152661_d_n8;
        locals.var_xp_dn9 = assign100560_e152661_d_n9;
        locals.var_xp_dn10 = assign100560_e152661_d_n10;
        locals.var_xp_dn13 = assign100560_e152661_d_n13;

        let (assign100570_e152671, assign100570_e152671_d_n0, assign100570_e152671_d_n2, assign100570_e152671_d_n4, assign100570_e152671_d_n5, assign100570_e152671_d_n6, assign100570_e152671_d_n7, assign100570_e152671_d_n8, assign100570_e152671_d_n9, assign100570_e152671_d_n10, assign100570_e152671_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100570_e152669: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100570_e152669, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100570_e152671;
        locals.var_xmp_dn0 = assign100570_e152671_d_n0;
        locals.var_xmp_dn2 = assign100570_e152671_d_n2;
        locals.var_xmp_dn4 = assign100570_e152671_d_n4;
        locals.var_xmp_dn5 = assign100570_e152671_d_n5;
        locals.var_xmp_dn6 = assign100570_e152671_d_n6;
        locals.var_xmp_dn7 = assign100570_e152671_d_n7;
        locals.var_xmp_dn8 = assign100570_e152671_d_n8;
        locals.var_xmp_dn9 = assign100570_e152671_d_n9;
        locals.var_xmp_dn10 = assign100570_e152671_d_n10;
        locals.var_xmp_dn13 = assign100570_e152671_d_n13;

        let (assign100580_e152681, assign100580_e152681_d_n0, assign100580_e152681_d_n2, assign100580_e152681_d_n4, assign100580_e152681_d_n5, assign100580_e152681_d_n6, assign100580_e152681_d_n7, assign100580_e152681_d_n8, assign100580_e152681_d_n9, assign100580_e152681_d_n10, assign100580_e152681_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100580_e152679: f64 = (locals.var_xp * locals.var_x2);
        (assign100580_e152679, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100580_e152681;
        locals.var_xp_dn0 = assign100580_e152681_d_n0;
        locals.var_xp_dn2 = assign100580_e152681_d_n2;
        locals.var_xp_dn4 = assign100580_e152681_d_n4;
        locals.var_xp_dn5 = assign100580_e152681_d_n5;
        locals.var_xp_dn6 = assign100580_e152681_d_n6;
        locals.var_xp_dn7 = assign100580_e152681_d_n7;
        locals.var_xp_dn8 = assign100580_e152681_d_n8;
        locals.var_xp_dn9 = assign100580_e152681_d_n9;
        locals.var_xp_dn10 = assign100580_e152681_d_n10;
        locals.var_xp_dn13 = assign100580_e152681_d_n13;

        let (assign100590_e152691, assign100590_e152691_d_n0, assign100590_e152691_d_n2, assign100590_e152691_d_n4, assign100590_e152691_d_n5, assign100590_e152691_d_n6, assign100590_e152691_d_n7, assign100590_e152691_d_n8, assign100590_e152691_d_n9, assign100590_e152691_d_n10, assign100590_e152691_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100590_e152689: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100590_e152689, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100590_e152691;
        locals.var_xmp_dn0 = assign100590_e152691_d_n0;
        locals.var_xmp_dn2 = assign100590_e152691_d_n2;
        locals.var_xmp_dn4 = assign100590_e152691_d_n4;
        locals.var_xmp_dn5 = assign100590_e152691_d_n5;
        locals.var_xmp_dn6 = assign100590_e152691_d_n6;
        locals.var_xmp_dn7 = assign100590_e152691_d_n7;
        locals.var_xmp_dn8 = assign100590_e152691_d_n8;
        locals.var_xmp_dn9 = assign100590_e152691_d_n9;
        locals.var_xmp_dn10 = assign100590_e152691_d_n10;
        locals.var_xmp_dn13 = assign100590_e152691_d_n13;

        let (assign100600_e152701, assign100600_e152701_d_n0, assign100600_e152701_d_n2, assign100600_e152701_d_n4, assign100600_e152701_d_n5, assign100600_e152701_d_n6, assign100600_e152701_d_n7, assign100600_e152701_d_n8, assign100600_e152701_d_n9, assign100600_e152701_d_n10, assign100600_e152701_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100600_e152699: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100600_e152701;
        locals.var_xp_dn0 = assign100600_e152701_d_n0;
        locals.var_xp_dn2 = assign100600_e152701_d_n2;
        locals.var_xp_dn4 = assign100600_e152701_d_n4;
        locals.var_xp_dn5 = assign100600_e152701_d_n5;
        locals.var_xp_dn6 = assign100600_e152701_d_n6;
        locals.var_xp_dn7 = assign100600_e152701_d_n7;
        locals.var_xp_dn8 = assign100600_e152701_d_n8;
        locals.var_xp_dn9 = assign100600_e152701_d_n9;
        locals.var_xp_dn10 = assign100600_e152701_d_n10;
        locals.var_xp_dn13 = assign100600_e152701_d_n13;

        let (assign100610_e152711, assign100610_e152711_d_n0, assign100610_e152711_d_n2, assign100610_e152711_d_n4, assign100610_e152711_d_n5, assign100610_e152711_d_n6, assign100610_e152711_d_n7, assign100610_e152711_d_n8, assign100610_e152711_d_n9, assign100610_e152711_d_n10, assign100610_e152711_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100610_e152709: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152709, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100610_e152711;
        locals.var_xmp_dn0 = assign100610_e152711_d_n0;
        locals.var_xmp_dn2 = assign100610_e152711_d_n2;
        locals.var_xmp_dn4 = assign100610_e152711_d_n4;
        locals.var_xmp_dn5 = assign100610_e152711_d_n5;
        locals.var_xmp_dn6 = assign100610_e152711_d_n6;
        locals.var_xmp_dn7 = assign100610_e152711_d_n7;
        locals.var_xmp_dn8 = assign100610_e152711_d_n8;
        locals.var_xmp_dn9 = assign100610_e152711_d_n9;
        locals.var_xmp_dn10 = assign100610_e152711_d_n10;
        locals.var_xmp_dn13 = assign100610_e152711_d_n13;

        let (assign100620_e152721, assign100620_e152721_d_n0, assign100620_e152721_d_n2, assign100620_e152721_d_n4, assign100620_e152721_d_n5, assign100620_e152721_d_n6, assign100620_e152721_d_n7, assign100620_e152721_d_n8, assign100620_e152721_d_n9, assign100620_e152721_d_n10, assign100620_e152721_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100620_e152719: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100620_e152721;
        locals.var_xp_dn0 = assign100620_e152721_d_n0;
        locals.var_xp_dn2 = assign100620_e152721_d_n2;
        locals.var_xp_dn4 = assign100620_e152721_d_n4;
        locals.var_xp_dn5 = assign100620_e152721_d_n5;
        locals.var_xp_dn6 = assign100620_e152721_d_n6;
        locals.var_xp_dn7 = assign100620_e152721_d_n7;
        locals.var_xp_dn8 = assign100620_e152721_d_n8;
        locals.var_xp_dn9 = assign100620_e152721_d_n9;
        locals.var_xp_dn10 = assign100620_e152721_d_n10;
        locals.var_xp_dn13 = assign100620_e152721_d_n13;

        let (assign100630_e152731, assign100630_e152731_d_n0, assign100630_e152731_d_n2, assign100630_e152731_d_n4, assign100630_e152731_d_n5, assign100630_e152731_d_n6, assign100630_e152731_d_n7, assign100630_e152731_d_n8, assign100630_e152731_d_n9, assign100630_e152731_d_n10, assign100630_e152731_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100630_e152729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100630_e152731;
        locals.var_xmp_dn0 = assign100630_e152731_d_n0;
        locals.var_xmp_dn2 = assign100630_e152731_d_n2;
        locals.var_xmp_dn4 = assign100630_e152731_d_n4;
        locals.var_xmp_dn5 = assign100630_e152731_d_n5;
        locals.var_xmp_dn6 = assign100630_e152731_d_n6;
        locals.var_xmp_dn7 = assign100630_e152731_d_n7;
        locals.var_xmp_dn8 = assign100630_e152731_d_n8;
        locals.var_xmp_dn9 = assign100630_e152731_d_n9;
        locals.var_xmp_dn10 = assign100630_e152731_d_n10;
        locals.var_xmp_dn13 = assign100630_e152731_d_n13;

        let (assign100640_e152741, assign100640_e152741_d_n0, assign100640_e152741_d_n2, assign100640_e152741_d_n4, assign100640_e152741_d_n5, assign100640_e152741_d_n6, assign100640_e152741_d_n7, assign100640_e152741_d_n8, assign100640_e152741_d_n9, assign100640_e152741_d_n10, assign100640_e152741_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100640_e152739: f64 = (locals.var_xp + locals.var_xmp);
        (assign100640_e152739, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign100640_e152741;
        locals.var_arg_dn0 = assign100640_e152741_d_n0;
        locals.var_arg_dn2 = assign100640_e152741_d_n2;
        locals.var_arg_dn4 = assign100640_e152741_d_n4;
        locals.var_arg_dn5 = assign100640_e152741_d_n5;
        locals.var_arg_dn6 = assign100640_e152741_d_n6;
        locals.var_arg_dn7 = assign100640_e152741_d_n7;
        locals.var_arg_dn8 = assign100640_e152741_d_n8;
        locals.var_arg_dn9 = assign100640_e152741_d_n9;
        locals.var_arg_dn10 = assign100640_e152741_d_n10;
        locals.var_arg_dn13 = assign100640_e152741_d_n13;

        let (assign100650_e152749, assign100650_e152749_d_n0, assign100650_e152749_d_n2, assign100650_e152749_d_n4, assign100650_e152749_d_n5, assign100650_e152749_d_n6, assign100650_e152749_d_n7, assign100650_e152749_d_n8, assign100650_e152749_d_n9, assign100650_e152749_d_n10, assign100650_e152749_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100650_e152749;
        locals.var_dnm_dn0 = assign100650_e152749_d_n0;
        locals.var_dnm_dn2 = assign100650_e152749_d_n2;
        locals.var_dnm_dn4 = assign100650_e152749_d_n4;
        locals.var_dnm_dn5 = assign100650_e152749_d_n5;
        locals.var_dnm_dn6 = assign100650_e152749_d_n6;
        locals.var_dnm_dn7 = assign100650_e152749_d_n7;
        locals.var_dnm_dn8 = assign100650_e152749_d_n8;
        locals.var_dnm_dn9 = assign100650_e152749_d_n9;
        locals.var_dnm_dn10 = assign100650_e152749_d_n10;
        locals.var_dnm_dn13 = assign100650_e152749_d_n13;

        let assign100660_e152764: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2314 = assign100660_e152764;

        let assign100670_e152767: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100670_e152767;

        let (assign100680_e152779,) = {
    if (((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100680_e152779;

        let assign100690_e152782: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100690_e152782;

        let (assign100700_e152797,) = {
    if ((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100700_e152797;

        let assign100710_e152800: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100710_e152800;

        let (assign100720_e152818,) = {
    if (((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 == 0.0)) && (locals.var_guard2317 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152818;

        let assign100730_e152821: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100730_e152821;

        let (assign100740_e152842,) = {
    if ((((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 == 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152842;

        let (assign100750_e152852,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100750_e152852;

        let mut assign100760_loop_guard: usize = 0;
        while {
            let assign100760_cond_e152863: f64 = if (((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100760_cond_e152863 != 0.0
        } {
            assign100760_loop_guard += 1;
            assert!(assign100760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100760_body0_e152874, assign100760_body0_e152874_d_n0, assign100760_body0_e152874_d_n2, assign100760_body0_e152874_d_n4, assign100760_body0_e152874_d_n5, assign100760_body0_e152874_d_n6, assign100760_body0_e152874_d_n7, assign100760_body0_e152874_d_n8, assign100760_body0_e152874_d_n9, assign100760_body0_e152874_d_n10, assign100760_body0_e152874_d_n13,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        let assign100760_body0_e152872: f64 = (locals.var_dnm).sqrt();
        (assign100760_body0_e152872, (locals.var_dnm_dn0 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn2 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn4 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn5 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn6 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn7 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn8 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn9 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn10 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn13 / (2.0 * assign100760_body0_e152872)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign100760_body0_e152874;
            locals.var_dnm_dn0 = assign100760_body0_e152874_d_n0;
            locals.var_dnm_dn2 = assign100760_body0_e152874_d_n2;
            locals.var_dnm_dn4 = assign100760_body0_e152874_d_n4;
            locals.var_dnm_dn5 = assign100760_body0_e152874_d_n5;
            locals.var_dnm_dn6 = assign100760_body0_e152874_d_n6;
            locals.var_dnm_dn7 = assign100760_body0_e152874_d_n7;
            locals.var_dnm_dn8 = assign100760_body0_e152874_d_n8;
            locals.var_dnm_dn9 = assign100760_body0_e152874_d_n9;
            locals.var_dnm_dn10 = assign100760_body0_e152874_d_n10;
            locals.var_dnm_dn13 = assign100760_body0_e152874_d_n13;
            let (assign100760_body1_e152886,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        let assign100760_body1_e152884: f64 = (locals.var_m0 + 1.0);
        (assign100760_body1_e152884,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100760_body1_e152886;
        }

    }

    pub(super) fn stamp_transient_block_357(
        locals: &mut StampLocals,
    ) {
        let (assign100770_e152908, assign100770_e152908_d_n0, assign100770_e152908_d_n2, assign100770_e152908_d_n4, assign100770_e152908_d_n5, assign100770_e152908_d_n6, assign100770_e152908_d_n7, assign100770_e152908_d_n8, assign100770_e152908_d_n9, assign100770_e152908_d_n10, assign100770_e152908_d_n13,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 == 0.0)) {
        let (assign100770_e152906, assign100770_e152906_d_n0, assign100770_e152906_d_n2, assign100770_e152906_d_n4, assign100770_e152906_d_n5, assign100770_e152906_d_n6, assign100770_e152906_d_n7, assign100770_e152906_d_n8, assign100770_e152906_d_n9, assign100770_e152906_d_n10, assign100770_e152906_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100770_e152903: f64 = (2.0 * 4.0);
                let assign100770_e152904: f64 = (1.0 / assign100770_e152903);
                let assign100770_e152905: f64 = (locals.var_dnm).powf(assign100770_e152904);
                (assign100770_e152905, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn13)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign100770_e152906, assign100770_e152906_d_n0, assign100770_e152906_d_n2, assign100770_e152906_d_n4, assign100770_e152906_d_n5, assign100770_e152906_d_n6, assign100770_e152906_d_n7, assign100770_e152906_d_n8, assign100770_e152906_d_n9, assign100770_e152906_d_n10, assign100770_e152906_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100770_e152908;
        locals.var_dnm_dn0 = assign100770_e152908_d_n0;
        locals.var_dnm_dn2 = assign100770_e152908_d_n2;
        locals.var_dnm_dn4 = assign100770_e152908_d_n4;
        locals.var_dnm_dn5 = assign100770_e152908_d_n5;
        locals.var_dnm_dn6 = assign100770_e152908_d_n6;
        locals.var_dnm_dn7 = assign100770_e152908_d_n7;
        locals.var_dnm_dn8 = assign100770_e152908_d_n8;
        locals.var_dnm_dn9 = assign100770_e152908_d_n9;
        locals.var_dnm_dn10 = assign100770_e152908_d_n10;
        locals.var_dnm_dn13 = assign100770_e152908_d_n13;

        let (assign100780_e152918, assign100780_e152918_d_n0, assign100780_e152918_d_n2, assign100780_e152918_d_n4, assign100780_e152918_d_n5, assign100780_e152918_d_n6, assign100780_e152918_d_n7, assign100780_e152918_d_n8, assign100780_e152918_d_n9, assign100780_e152918_d_n10, assign100780_e152918_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100780_e152916: f64 = (1.0 / locals.var_dnm);
        (assign100780_e152916, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100780_e152918;
        locals.var_dnm_dn0 = assign100780_e152918_d_n0;
        locals.var_dnm_dn2 = assign100780_e152918_d_n2;
        locals.var_dnm_dn4 = assign100780_e152918_d_n4;
        locals.var_dnm_dn5 = assign100780_e152918_d_n5;
        locals.var_dnm_dn6 = assign100780_e152918_d_n6;
        locals.var_dnm_dn7 = assign100780_e152918_d_n7;
        locals.var_dnm_dn8 = assign100780_e152918_d_n8;
        locals.var_dnm_dn9 = assign100780_e152918_d_n9;
        locals.var_dnm_dn10 = assign100780_e152918_d_n10;
        locals.var_dnm_dn13 = assign100780_e152918_d_n13;

        let (assign100790_e152930, assign100790_e152930_d_n0, assign100790_e152930_d_n2, assign100790_e152930_d_n4, assign100790_e152930_d_n5, assign100790_e152930_d_n6, assign100790_e152930_d_n7, assign100790_e152930_d_n8, assign100790_e152930_d_n9, assign100790_e152930_d_n10, assign100790_e152930_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100790_e152926: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100790_e152928: f64 = (assign100790_e152926 * locals.var_dnm);
        (assign100790_e152928, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn13)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign100790_e152930;
        locals.var_tmf0_dn0 = assign100790_e152930_d_n0;
        locals.var_tmf0_dn2 = assign100790_e152930_d_n2;
        locals.var_tmf0_dn4 = assign100790_e152930_d_n4;
        locals.var_tmf0_dn5 = assign100790_e152930_d_n5;
        locals.var_tmf0_dn6 = assign100790_e152930_d_n6;
        locals.var_tmf0_dn7 = assign100790_e152930_d_n7;
        locals.var_tmf0_dn8 = assign100790_e152930_d_n8;
        locals.var_tmf0_dn9 = assign100790_e152930_d_n9;
        locals.var_tmf0_dn10 = assign100790_e152930_d_n10;
        locals.var_tmf0_dn13 = assign100790_e152930_d_n13;

        let (assign100800_e152944, assign100800_e152944_d_n0, assign100800_e152944_d_n2, assign100800_e152944_d_n4, assign100800_e152944_d_n5, assign100800_e152944_d_n6, assign100800_e152944_d_n7, assign100800_e152944_d_n8, assign100800_e152944_d_n9, assign100800_e152944_d_n10, assign100800_e152944_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100800_e152938: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100800_e152940: f64 = (assign100800_e152938 * locals.var_dnm);
        let assign100800_e152942: f64 = (assign100800_e152940 / locals.var_arg);
        (assign100800_e152942, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn13 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn13)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100800_e152944;
        locals.var_t0_dn0 = assign100800_e152944_d_n0;
        locals.var_t0_dn2 = assign100800_e152944_d_n2;
        locals.var_t0_dn4 = assign100800_e152944_d_n4;
        locals.var_t0_dn5 = assign100800_e152944_d_n5;
        locals.var_t0_dn6 = assign100800_e152944_d_n6;
        locals.var_t0_dn7 = assign100800_e152944_d_n7;
        locals.var_t0_dn8 = assign100800_e152944_d_n8;
        locals.var_t0_dn9 = assign100800_e152944_d_n9;
        locals.var_t0_dn10 = assign100800_e152944_d_n10;
        locals.var_t0_dn13 = assign100800_e152944_d_n13;

        let (assign100810_e152956, assign100810_e152956_d_n0, assign100810_e152956_d_n2, assign100810_e152956_d_n4, assign100810_e152956_d_n5, assign100810_e152956_d_n6, assign100810_e152956_d_n7, assign100810_e152956_d_n8, assign100810_e152956_d_n9, assign100810_e152956_d_n10, assign100810_e152956_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100810_e152952: f64 = locals.var_t7;
        let assign100810_e152954: f64 = (assign100810_e152952 - locals.var_tmf0);
        (assign100810_e152954, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100810_e152956;
        locals.var_t6_dn0 = assign100810_e152956_d_n0;
        locals.var_t6_dn2 = assign100810_e152956_d_n2;
        locals.var_t6_dn4 = assign100810_e152956_d_n4;
        locals.var_t6_dn5 = assign100810_e152956_d_n5;
        locals.var_t6_dn6 = assign100810_e152956_d_n6;
        locals.var_t6_dn7 = assign100810_e152956_d_n7;
        locals.var_t6_dn8 = assign100810_e152956_d_n8;
        locals.var_t6_dn9 = assign100810_e152956_d_n9;
        locals.var_t6_dn10 = assign100810_e152956_d_n10;
        locals.var_t6_dn13 = assign100810_e152956_d_n13;

        let (assign100820_e152964, assign100820_e152964_d_n0, assign100820_e152964_d_n2, assign100820_e152964_d_n4, assign100820_e152964_d_n5, assign100820_e152964_d_n6, assign100820_e152964_d_n7, assign100820_e152964_d_n8, assign100820_e152964_d_n9, assign100820_e152964_d_n10, assign100820_e152964_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100820_e152964;
        locals.var_t0_dn0 = assign100820_e152964_d_n0;
        locals.var_t0_dn2 = assign100820_e152964_d_n2;
        locals.var_t0_dn4 = assign100820_e152964_d_n4;
        locals.var_t0_dn5 = assign100820_e152964_d_n5;
        locals.var_t0_dn6 = assign100820_e152964_d_n6;
        locals.var_t0_dn7 = assign100820_e152964_d_n7;
        locals.var_t0_dn8 = assign100820_e152964_d_n8;
        locals.var_t0_dn9 = assign100820_e152964_d_n9;
        locals.var_t0_dn10 = assign100820_e152964_d_n10;
        locals.var_t0_dn13 = assign100820_e152964_d_n13;

        let (assign100830_e152973, assign100830_e152973_d_n0, assign100830_e152973_d_n2, assign100830_e152973_d_n4, assign100830_e152973_d_n5, assign100830_e152973_d_n6, assign100830_e152973_d_n7, assign100830_e152973_d_n8, assign100830_e152973_d_n9, assign100830_e152973_d_n10, assign100830_e152973_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100830_e152973;
        locals.var_t6_dn0 = assign100830_e152973_d_n0;
        locals.var_t6_dn2 = assign100830_e152973_d_n2;
        locals.var_t6_dn4 = assign100830_e152973_d_n4;
        locals.var_t6_dn5 = assign100830_e152973_d_n5;
        locals.var_t6_dn6 = assign100830_e152973_d_n6;
        locals.var_t6_dn7 = assign100830_e152973_d_n7;
        locals.var_t6_dn8 = assign100830_e152973_d_n8;
        locals.var_t6_dn9 = assign100830_e152973_d_n9;
        locals.var_t6_dn10 = assign100830_e152973_d_n10;
        locals.var_t6_dn13 = assign100830_e152973_d_n13;

        let (assign100840_e152982, assign100840_e152982_d_n0, assign100840_e152982_d_n2, assign100840_e152982_d_n4, assign100840_e152982_d_n5, assign100840_e152982_d_n6, assign100840_e152982_d_n7, assign100840_e152982_d_n8, assign100840_e152982_d_n9, assign100840_e152982_d_n10, assign100840_e152982_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100840_e152982;
        locals.var_t0_dn0 = assign100840_e152982_d_n0;
        locals.var_t0_dn2 = assign100840_e152982_d_n2;
        locals.var_t0_dn4 = assign100840_e152982_d_n4;
        locals.var_t0_dn5 = assign100840_e152982_d_n5;
        locals.var_t0_dn6 = assign100840_e152982_d_n6;
        locals.var_t0_dn7 = assign100840_e152982_d_n7;
        locals.var_t0_dn8 = assign100840_e152982_d_n8;
        locals.var_t0_dn9 = assign100840_e152982_d_n9;
        locals.var_t0_dn10 = assign100840_e152982_d_n10;
        locals.var_t0_dn13 = assign100840_e152982_d_n13;

        let (assign100850_e152989, assign100850_e152989_d_n0, assign100850_e152989_d_n2, assign100850_e152989_d_n4, assign100850_e152989_d_n5, assign100850_e152989_d_n6, assign100850_e152989_d_n7, assign100850_e152989_d_n8, assign100850_e152989_d_n9, assign100850_e152989_d_n10, assign100850_e152989_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100850_e152987: f64 = (locals.var_t6).sqrt();
        (assign100850_e152987, (locals.var_t6_dn0 / (2.0 * assign100850_e152987)), (locals.var_t6_dn2 / (2.0 * assign100850_e152987)), (locals.var_t6_dn4 / (2.0 * assign100850_e152987)), (locals.var_t6_dn5 / (2.0 * assign100850_e152987)), (locals.var_t6_dn6 / (2.0 * assign100850_e152987)), (locals.var_t6_dn7 / (2.0 * assign100850_e152987)), (locals.var_t6_dn8 / (2.0 * assign100850_e152987)), (locals.var_t6_dn9 / (2.0 * assign100850_e152987)), (locals.var_t6_dn10 / (2.0 * assign100850_e152987)), (locals.var_t6_dn13 / (2.0 * assign100850_e152987)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100850_e152989;
        locals.var_t6_dn0 = assign100850_e152989_d_n0;
        locals.var_t6_dn2 = assign100850_e152989_d_n2;
        locals.var_t6_dn4 = assign100850_e152989_d_n4;
        locals.var_t6_dn5 = assign100850_e152989_d_n5;
        locals.var_t6_dn6 = assign100850_e152989_d_n6;
        locals.var_t6_dn7 = assign100850_e152989_d_n7;
        locals.var_t6_dn8 = assign100850_e152989_d_n8;
        locals.var_t6_dn9 = assign100850_e152989_d_n9;
        locals.var_t6_dn10 = assign100850_e152989_d_n10;
        locals.var_t6_dn13 = assign100850_e152989_d_n13;

        let (assign100860_e153001, assign100860_e153001_d_n0, assign100860_e153001_d_n2, assign100860_e153001_d_n4, assign100860_e153001_d_n5, assign100860_e153001_d_n6, assign100860_e153001_d_n7, assign100860_e153001_d_n8, assign100860_e153001_d_n9, assign100860_e153001_d_n10, assign100860_e153001_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100860_e152997: f64 = (1.0 - locals.var_t6);
        let assign100860_e152998: f64 = (locals.var_t3 * assign100860_e152997);
        let assign100860_e152999: f64 = (locals.var_t1 + assign100860_e152998);
        (assign100860_e152999, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn13 + ((locals.var_t3_dn13 * assign100860_e152997) + (locals.var_t3 * (-locals.var_t6_dn13)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn13,)
    }
};
        locals.var_psislsat = assign100860_e153001;
        locals.var_psislsat_dn0 = assign100860_e153001_d_n0;
        locals.var_psislsat_dn2 = assign100860_e153001_d_n2;
        locals.var_psislsat_dn4 = assign100860_e153001_d_n4;
        locals.var_psislsat_dn5 = assign100860_e153001_d_n5;
        locals.var_psislsat_dn6 = assign100860_e153001_d_n6;
        locals.var_psislsat_dn7 = assign100860_e153001_d_n7;
        locals.var_psislsat_dn8 = assign100860_e153001_d_n8;
        locals.var_psislsat_dn9 = assign100860_e153001_d_n9;
        locals.var_psislsat_dn10 = assign100860_e153001_d_n10;
        locals.var_psislsat_dn13 = assign100860_e153001_d_n13;

        let (assign100870_e153011, assign100870_e153011_d_n0, assign100870_e153011_d_n2, assign100870_e153011_d_n4, assign100870_e153011_d_n5, assign100870_e153011_d_n6, assign100870_e153011_d_n7, assign100870_e153011_d_n8, assign100870_e153011_d_n9, assign100870_e153011_d_n10, assign100870_e153011_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100870_e153008: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100870_e153009: f64 = (locals.var_lgate / assign100870_e153008);
        (assign100870_e153009, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign100870_e153011;
        locals.var_t2_dn0 = assign100870_e153011_d_n0;
        locals.var_t2_dn2 = assign100870_e153011_d_n2;
        locals.var_t2_dn4 = assign100870_e153011_d_n4;
        locals.var_t2_dn5 = assign100870_e153011_d_n5;
        locals.var_t2_dn6 = assign100870_e153011_d_n6;
        locals.var_t2_dn7 = assign100870_e153011_d_n7;
        locals.var_t2_dn8 = assign100870_e153011_d_n8;
        locals.var_t2_dn9 = assign100870_e153011_d_n9;
        locals.var_t2_dn10 = assign100870_e153011_d_n10;
        locals.var_t2_dn13 = assign100870_e153011_d_n13;

        let (assign100880_e153025, assign100880_e153025_d_n0, assign100880_e153025_d_n2, assign100880_e153025_d_n4, assign100880_e153025_d_n5, assign100880_e153025_d_n6, assign100880_e153025_d_n7, assign100880_e153025_d_n8, assign100880_e153025_d_n9, assign100880_e153025_d_n10, assign100880_e153025_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100880_e153017: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk439);
        let assign100880_e153019: f64 = (assign100880_e153017 + locals.var_ps0z);
        let assign100880_e153022: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100880_e153023: f64 = (assign100880_e153019 - assign100880_e153022);
        (assign100880_e153023, (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk439_dn13) + locals.var_ps0z_dn13) - ((locals.var_t2_dn13 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn13))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100880_e153025;
        locals.var_psisubsat_dn0 = assign100880_e153025_d_n0;
        locals.var_psisubsat_dn2 = assign100880_e153025_d_n2;
        locals.var_psisubsat_dn4 = assign100880_e153025_d_n4;
        locals.var_psisubsat_dn5 = assign100880_e153025_d_n5;
        locals.var_psisubsat_dn6 = assign100880_e153025_d_n6;
        locals.var_psisubsat_dn7 = assign100880_e153025_d_n7;
        locals.var_psisubsat_dn8 = assign100880_e153025_d_n8;
        locals.var_psisubsat_dn9 = assign100880_e153025_d_n9;
        locals.var_psisubsat_dn10 = assign100880_e153025_d_n10;
        locals.var_psisubsat_dn13 = assign100880_e153025_d_n13;

        let (assign100890_e153040, assign100890_e153040_d_n0, assign100890_e153040_d_n2, assign100890_e153040_d_n4, assign100890_e153040_d_n5, assign100890_e153040_d_n6, assign100890_e153040_d_n7, assign100890_e153040_d_n8, assign100890_e153040_d_n9, assign100890_e153040_d_n10, assign100890_e153040_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100890_e153031: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100890_e153034: f64 = (4.0 * 0.001);
        let assign100890_e153036: f64 = (assign100890_e153034 * 0.001);
        let assign100890_e153037: f64 = (assign100890_e153031 + assign100890_e153036);
        let assign100890_e153038: f64 = (assign100890_e153037).sqrt();
        (assign100890_e153038, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100890_e153038)), (((locals.var_psisubsat_dn13 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn13)) / (2.0 * assign100890_e153038)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100890_e153040;
        locals.var_tmf2_dn0 = assign100890_e153040_d_n0;
        locals.var_tmf2_dn2 = assign100890_e153040_d_n2;
        locals.var_tmf2_dn4 = assign100890_e153040_d_n4;
        locals.var_tmf2_dn5 = assign100890_e153040_d_n5;
        locals.var_tmf2_dn6 = assign100890_e153040_d_n6;
        locals.var_tmf2_dn7 = assign100890_e153040_d_n7;
        locals.var_tmf2_dn8 = assign100890_e153040_d_n8;
        locals.var_tmf2_dn9 = assign100890_e153040_d_n9;
        locals.var_tmf2_dn10 = assign100890_e153040_d_n10;
        locals.var_tmf2_dn13 = assign100890_e153040_d_n13;

        let (assign100900_e153052, assign100900_e153052_d_n0, assign100900_e153052_d_n2, assign100900_e153052_d_n4, assign100900_e153052_d_n5, assign100900_e153052_d_n6, assign100900_e153052_d_n7, assign100900_e153052_d_n8, assign100900_e153052_d_n9, assign100900_e153052_d_n10, assign100900_e153052_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100900_e153048: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100900_e153049: f64 = (1.0 + assign100900_e153048);
        let assign100900_e153050: f64 = (0.5 * assign100900_e153049);
        (assign100900_e153050, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn13 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign100900_e153052;
        locals.var_t9_dn0 = assign100900_e153052_d_n0;
        locals.var_t9_dn2 = assign100900_e153052_d_n2;
        locals.var_t9_dn4 = assign100900_e153052_d_n4;
        locals.var_t9_dn5 = assign100900_e153052_d_n5;
        locals.var_t9_dn6 = assign100900_e153052_d_n6;
        locals.var_t9_dn7 = assign100900_e153052_d_n7;
        locals.var_t9_dn8 = assign100900_e153052_d_n8;
        locals.var_t9_dn9 = assign100900_e153052_d_n9;
        locals.var_t9_dn10 = assign100900_e153052_d_n10;
        locals.var_t9_dn13 = assign100900_e153052_d_n13;

        let (assign100910_e153062, assign100910_e153062_d_n0, assign100910_e153062_d_n2, assign100910_e153062_d_n4, assign100910_e153062_d_n5, assign100910_e153062_d_n6, assign100910_e153062_d_n7, assign100910_e153062_d_n8, assign100910_e153062_d_n9, assign100910_e153062_d_n10, assign100910_e153062_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100910_e153059: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100910_e153060: f64 = (0.5 * assign100910_e153059);
        (assign100910_e153060, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100910_e153062;
        locals.var_psisubsat_dn0 = assign100910_e153062_d_n0;
        locals.var_psisubsat_dn2 = assign100910_e153062_d_n2;
        locals.var_psisubsat_dn4 = assign100910_e153062_d_n4;
        locals.var_psisubsat_dn5 = assign100910_e153062_d_n5;
        locals.var_psisubsat_dn6 = assign100910_e153062_d_n6;
        locals.var_psisubsat_dn7 = assign100910_e153062_d_n7;
        locals.var_psisubsat_dn8 = assign100910_e153062_d_n8;
        locals.var_psisubsat_dn9 = assign100910_e153062_d_n9;
        locals.var_psisubsat_dn10 = assign100910_e153062_d_n10;
        locals.var_psisubsat_dn13 = assign100910_e153062_d_n13;

        let assign100920_e153065: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100920_e153065;

        let (assign100930_e153073, assign100930_e153073_d_n0, assign100930_e153073_d_n2, assign100930_e153073_d_n4, assign100930_e153073_d_n5, assign100930_e153073_d_n6, assign100930_e153073_d_n7, assign100930_e153073_d_n8, assign100930_e153073_d_n9, assign100930_e153073_d_n10, assign100930_e153073_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100930_e153073;
        locals.var_psisubsat_dn0 = assign100930_e153073_d_n0;
        locals.var_psisubsat_dn2 = assign100930_e153073_d_n2;
        locals.var_psisubsat_dn4 = assign100930_e153073_d_n4;
        locals.var_psisubsat_dn5 = assign100930_e153073_d_n5;
        locals.var_psisubsat_dn6 = assign100930_e153073_d_n6;
        locals.var_psisubsat_dn7 = assign100930_e153073_d_n7;
        locals.var_psisubsat_dn8 = assign100930_e153073_d_n8;
        locals.var_psisubsat_dn9 = assign100930_e153073_d_n9;
        locals.var_psisubsat_dn10 = assign100930_e153073_d_n10;
        locals.var_psisubsat_dn13 = assign100930_e153073_d_n13;

        let (assign100940_e153081, assign100940_e153081_d_n0, assign100940_e153081_d_n2, assign100940_e153081_d_n4, assign100940_e153081_d_n5, assign100940_e153081_d_n6, assign100940_e153081_d_n7, assign100940_e153081_d_n8, assign100940_e153081_d_n9, assign100940_e153081_d_n10, assign100940_e153081_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign100940_e153081;
        locals.var_t9_dn0 = assign100940_e153081_d_n0;
        locals.var_t9_dn2 = assign100940_e153081_d_n2;
        locals.var_t9_dn4 = assign100940_e153081_d_n4;
        locals.var_t9_dn5 = assign100940_e153081_d_n5;
        locals.var_t9_dn6 = assign100940_e153081_d_n6;
        locals.var_t9_dn7 = assign100940_e153081_d_n7;
        locals.var_t9_dn8 = assign100940_e153081_d_n8;
        locals.var_t9_dn9 = assign100940_e153081_d_n9;
        locals.var_t9_dn10 = assign100940_e153081_d_n10;
        locals.var_t9_dn13 = assign100940_e153081_d_n13;

        let (assign100950_e153089, assign100950_e153089_d_n0, assign100950_e153089_d_n2, assign100950_e153089_d_n4, assign100950_e153089_d_n5, assign100950_e153089_d_n6, assign100950_e153089_d_n7, assign100950_e153089_d_n8, assign100950_e153089_d_n9, assign100950_e153089_d_n10, assign100950_e153089_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100950_e153087: f64 = (locals.var_psisubsat + 1e-25);
        (assign100950_e153087, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign100950_e153089;
        locals.var_psisubsat_dn0 = assign100950_e153089_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153089_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153089_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153089_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153089_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153089_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153089_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153089_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153089_d_n10;
        locals.var_psisubsat_dn13 = assign100950_e153089_d_n13;

        let (assign100960_e153101, assign100960_e153101_d_n0, assign100960_e153101_d_n2, assign100960_e153101_d_n4, assign100960_e153101_d_n5, assign100960_e153101_d_n6, assign100960_e153101_d_n7, assign100960_e153101_d_n8, assign100960_e153101_d_n9, assign100960_e153101_d_n10, assign100960_e153101_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100960_e153097: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign100960_e153098: f64 = (locals.var_uc_subtmp * assign100960_e153097);
        let assign100960_e153099: f64 = (1.0 + assign100960_e153098);
        (assign100960_e153099, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn13),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign100960_e153101;
        locals.var_xsubtmp_dn0 = assign100960_e153101_d_n0;
        locals.var_xsubtmp_dn2 = assign100960_e153101_d_n2;
        locals.var_xsubtmp_dn4 = assign100960_e153101_d_n4;
        locals.var_xsubtmp_dn5 = assign100960_e153101_d_n5;
        locals.var_xsubtmp_dn6 = assign100960_e153101_d_n6;
        locals.var_xsubtmp_dn7 = assign100960_e153101_d_n7;
        locals.var_xsubtmp_dn8 = assign100960_e153101_d_n8;
        locals.var_xsubtmp_dn9 = assign100960_e153101_d_n9;
        locals.var_xsubtmp_dn10 = assign100960_e153101_d_n10;
        locals.var_xsubtmp_dn13 = assign100960_e153101_d_n13;

        let (assign100970_e153112, assign100970_e153112_d_n0, assign100970_e153112_d_n2, assign100970_e153112_d_n4, assign100970_e153112_d_n5, assign100970_e153112_d_n6, assign100970_e153112_d_n7, assign100970_e153112_d_n8, assign100970_e153112_d_n9, assign100970_e153112_d_n10, assign100970_e153112_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let (assign100970_e153110, assign100970_e153110_d_n0, assign100970_e153110_d_n2, assign100970_e153110_d_n4, assign100970_e153110_d_n5, assign100970_e153110_d_n6, assign100970_e153110_d_n7, assign100970_e153110_d_n8, assign100970_e153110_d_n9, assign100970_e153110_d_n10, assign100970_e153110_d_n13,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
            }
        };
        (assign100970_e153110, assign100970_e153110_d_n0, assign100970_e153110_d_n2, assign100970_e153110_d_n4, assign100970_e153110_d_n5, assign100970_e153110_d_n6, assign100970_e153110_d_n7, assign100970_e153110_d_n8, assign100970_e153110_d_n9, assign100970_e153110_d_n10, assign100970_e153110_d_n13,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign100970_e153112;
        locals.var_xsubtmp_dn0 = assign100970_e153112_d_n0;
        locals.var_xsubtmp_dn2 = assign100970_e153112_d_n2;
        locals.var_xsubtmp_dn4 = assign100970_e153112_d_n4;
        locals.var_xsubtmp_dn5 = assign100970_e153112_d_n5;
        locals.var_xsubtmp_dn6 = assign100970_e153112_d_n6;
        locals.var_xsubtmp_dn7 = assign100970_e153112_d_n7;
        locals.var_xsubtmp_dn8 = assign100970_e153112_d_n8;
        locals.var_xsubtmp_dn9 = assign100970_e153112_d_n9;
        locals.var_xsubtmp_dn10 = assign100970_e153112_d_n10;
        locals.var_xsubtmp_dn13 = assign100970_e153112_d_n13;

        let (assign100980_e153120, assign100980_e153120_d_n0, assign100980_e153120_d_n2, assign100980_e153120_d_n4, assign100980_e153120_d_n5, assign100980_e153120_d_n6, assign100980_e153120_d_n7, assign100980_e153120_d_n8, assign100980_e153120_d_n9, assign100980_e153120_d_n10, assign100980_e153120_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100980_e153118: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign100980_e153118, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn13) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign100980_e153120;
        locals.var_t5_dn0 = assign100980_e153120_d_n0;
        locals.var_t5_dn2 = assign100980_e153120_d_n2;
        locals.var_t5_dn4 = assign100980_e153120_d_n4;
        locals.var_t5_dn5 = assign100980_e153120_d_n5;
        locals.var_t5_dn6 = assign100980_e153120_d_n6;
        locals.var_t5_dn7 = assign100980_e153120_d_n7;
        locals.var_t5_dn8 = assign100980_e153120_d_n8;
        locals.var_t5_dn9 = assign100980_e153120_d_n9;
        locals.var_t5_dn10 = assign100980_e153120_d_n10;
        locals.var_t5_dn13 = assign100980_e153120_d_n13;

        let (assign100990_e153128, assign100990_e153128_d_n0, assign100990_e153128_d_n2, assign100990_e153128_d_n4, assign100990_e153128_d_n5, assign100990_e153128_d_n6, assign100990_e153128_d_n7, assign100990_e153128_d_n8, assign100990_e153128_d_n9, assign100990_e153128_d_n10, assign100990_e153128_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100990_e153126: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign100990_e153126, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100990_e153128;
        locals.var_t6_dn0 = assign100990_e153128_d_n0;
        locals.var_t6_dn2 = assign100990_e153128_d_n2;
        locals.var_t6_dn4 = assign100990_e153128_d_n4;
        locals.var_t6_dn5 = assign100990_e153128_d_n5;
        locals.var_t6_dn6 = assign100990_e153128_d_n6;
        locals.var_t6_dn7 = assign100990_e153128_d_n7;
        locals.var_t6_dn8 = assign100990_e153128_d_n8;
        locals.var_t6_dn9 = assign100990_e153128_d_n9;
        locals.var_t6_dn10 = assign100990_e153128_d_n10;
        locals.var_t6_dn13 = assign100990_e153128_d_n13;

        let (assign101000_e153138, assign101000_e153138_d_n0, assign101000_e153138_d_n2, assign101000_e153138_d_n4, assign101000_e153138_d_n5, assign101000_e153138_d_n6, assign101000_e153138_d_n7, assign101000_e153138_d_n8, assign101000_e153138_d_n9, assign101000_e153138_d_n10, assign101000_e153138_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign101000_e153133: f64 = (-locals.var_t6);
        let assign101000_e153135: f64 = (assign101000_e153133 / locals.var_psisubsat);
        let assign101000_e153136: f64 = (assign101000_e153135).exp();
        (assign101000_e153136, (assign101000_e153136 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101000_e153136 * ((((-locals.var_t6_dn13) * locals.var_psisubsat) - (assign101000_e153133 * locals.var_psisubsat_dn13)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101000_e153138;
        locals.var_t2_dn0 = assign101000_e153138_d_n0;
        locals.var_t2_dn2 = assign101000_e153138_d_n2;
        locals.var_t2_dn4 = assign101000_e153138_d_n4;
        locals.var_t2_dn5 = assign101000_e153138_d_n5;
        locals.var_t2_dn6 = assign101000_e153138_d_n6;
        locals.var_t2_dn7 = assign101000_e153138_d_n7;
        locals.var_t2_dn8 = assign101000_e153138_d_n8;
        locals.var_t2_dn9 = assign101000_e153138_d_n9;
        locals.var_t2_dn10 = assign101000_e153138_d_n10;
        locals.var_t2_dn13 = assign101000_e153138_d_n13;

    }

    pub(super) fn stamp_transient_block_358(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101010_e153148, assign101010_e153148_d_n0, assign101010_e153148_d_n2, assign101010_e153148_d_n4, assign101010_e153148_d_n5, assign101010_e153148_d_n6, assign101010_e153148_d_n7, assign101010_e153148_d_n8, assign101010_e153148_d_n9, assign101010_e153148_d_n10, assign101010_e153148_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign101010_e153144: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign101010_e153146: f64 = (assign101010_e153144 * locals.var_t2);
        (assign101010_e153146, ((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn0)), ((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn2)), ((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn4)), ((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn5)), ((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn6)), ((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn7)), ((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn8)), ((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn9)), ((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn10)), ((((locals.var_t5_dn13 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn13)) * locals.var_t2) + (assign101010_e153144 * locals.var_t2_dn13)),)
    } else {
        (locals.var_iifac, locals.var_iifac_dn0, locals.var_iifac_dn2, locals.var_iifac_dn4, locals.var_iifac_dn5, locals.var_iifac_dn6, locals.var_iifac_dn7, locals.var_iifac_dn8, locals.var_iifac_dn9, locals.var_iifac_dn10, locals.var_iifac_dn13,)
    }
};
        locals.var_iifac = assign101010_e153148;
        locals.var_iifac_dn0 = assign101010_e153148_d_n0;
        locals.var_iifac_dn2 = assign101010_e153148_d_n2;
        locals.var_iifac_dn4 = assign101010_e153148_d_n4;
        locals.var_iifac_dn5 = assign101010_e153148_d_n5;
        locals.var_iifac_dn6 = assign101010_e153148_d_n6;
        locals.var_iifac_dn7 = assign101010_e153148_d_n7;
        locals.var_iifac_dn8 = assign101010_e153148_d_n8;
        locals.var_iifac_dn9 = assign101010_e153148_d_n9;
        locals.var_iifac_dn10 = assign101010_e153148_d_n10;
        locals.var_iifac_dn13 = assign101010_e153148_d_n13;

        let assign101020_e153151: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2320 = assign101020_e153151;

        let (assign101030_e153163, assign101030_e153163_d_n0, assign101030_e153163_d_n2, assign101030_e153163_d_n4, assign101030_e153163_d_n5, assign101030_e153163_d_n6, assign101030_e153163_d_n7, assign101030_e153163_d_n8, assign101030_e153163_d_n9, assign101030_e153163_d_n10, assign101030_e153163_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2320 != 0.0)) {
        let assign101030_e153159: f64 = (1.0 + locals.var_iifac);
        let assign101030_e153161: f64 = (assign101030_e153159 * locals.var_ibsi);
        (assign101030_e153161, ((locals.var_iifac_dn0 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn0)), ((locals.var_iifac_dn2 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn2)), ((locals.var_iifac_dn4 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn4)), ((locals.var_iifac_dn5 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn5)), ((locals.var_iifac_dn6 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn6)), ((locals.var_iifac_dn7 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn7)), ((locals.var_iifac_dn8 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn8)), ((locals.var_iifac_dn9 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn9)), ((locals.var_iifac_dn10 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn10)), ((locals.var_iifac_dn13 * locals.var_ibsi) + (assign101030_e153159 * locals.var_ibsi_dn13)),)
    } else {
        (locals.var_wibjt, locals.var_wibjt_dn0, locals.var_wibjt_dn2, locals.var_wibjt_dn4, locals.var_wibjt_dn5, locals.var_wibjt_dn6, locals.var_wibjt_dn7, locals.var_wibjt_dn8, locals.var_wibjt_dn9, locals.var_wibjt_dn10, locals.var_wibjt_dn13,)
    }
};
        locals.var_wibjt = assign101030_e153163;
        locals.var_wibjt_dn0 = assign101030_e153163_d_n0;
        locals.var_wibjt_dn2 = assign101030_e153163_d_n2;
        locals.var_wibjt_dn4 = assign101030_e153163_d_n4;
        locals.var_wibjt_dn5 = assign101030_e153163_d_n5;
        locals.var_wibjt_dn6 = assign101030_e153163_d_n6;
        locals.var_wibjt_dn7 = assign101030_e153163_d_n7;
        locals.var_wibjt_dn8 = assign101030_e153163_d_n8;
        locals.var_wibjt_dn9 = assign101030_e153163_d_n9;
        locals.var_wibjt_dn10 = assign101030_e153163_d_n10;
        locals.var_wibjt_dn13 = assign101030_e153163_d_n13;

        let (assign101040_e153176, assign101040_e153176_d_n0, assign101040_e153176_d_n2, assign101040_e153176_d_n4, assign101040_e153176_d_n5, assign101040_e153176_d_n6, assign101040_e153176_d_n7, assign101040_e153176_d_n8, assign101040_e153176_d_n9, assign101040_e153176_d_n10, assign101040_e153176_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2320 == 0.0)) {
        let assign101040_e153172: f64 = (1.0 + locals.var_iifac);
        let assign101040_e153174: f64 = (assign101040_e153172 * locals.var_ibs);
        (assign101040_e153174, ((locals.var_iifac_dn0 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn0)), ((locals.var_iifac_dn2 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn2)), ((locals.var_iifac_dn4 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn4)), ((locals.var_iifac_dn5 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn5)), ((locals.var_iifac_dn6 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn6)), ((locals.var_iifac_dn7 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn7)), ((locals.var_iifac_dn8 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn8)), ((locals.var_iifac_dn9 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn9)), ((locals.var_iifac_dn10 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn10)), ((locals.var_iifac_dn13 * locals.var_ibs) + (assign101040_e153172 * locals.var_ibs_dn13)),)
    } else {
        (locals.var_wibjt, locals.var_wibjt_dn0, locals.var_wibjt_dn2, locals.var_wibjt_dn4, locals.var_wibjt_dn5, locals.var_wibjt_dn6, locals.var_wibjt_dn7, locals.var_wibjt_dn8, locals.var_wibjt_dn9, locals.var_wibjt_dn10, locals.var_wibjt_dn13,)
    }
};
        locals.var_wibjt = assign101040_e153176;
        locals.var_wibjt_dn0 = assign101040_e153176_d_n0;
        locals.var_wibjt_dn2 = assign101040_e153176_d_n2;
        locals.var_wibjt_dn4 = assign101040_e153176_d_n4;
        locals.var_wibjt_dn5 = assign101040_e153176_d_n5;
        locals.var_wibjt_dn6 = assign101040_e153176_d_n6;
        locals.var_wibjt_dn7 = assign101040_e153176_d_n7;
        locals.var_wibjt_dn8 = assign101040_e153176_d_n8;
        locals.var_wibjt_dn9 = assign101040_e153176_d_n9;
        locals.var_wibjt_dn10 = assign101040_e153176_d_n10;
        locals.var_wibjt_dn13 = assign101040_e153176_d_n13;

        let assign101050_e153179: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign101050_e153179;

        let (assign101060_e153185, assign101060_e153185_d_n0, assign101060_e153185_d_n2, assign101060_e153185_d_n4, assign101060_e153185_d_n5, assign101060_e153185_d_n6, assign101060_e153185_d_n7, assign101060_e153185_d_n8, assign101060_e153185_d_n9, assign101060_e153185_d_n10, assign101060_e153185_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign101060_e153185;
        locals.var_t12_dn0 = assign101060_e153185_d_n0;
        locals.var_t12_dn2 = assign101060_e153185_d_n2;
        locals.var_t12_dn4 = assign101060_e153185_d_n4;
        locals.var_t12_dn5 = assign101060_e153185_d_n5;
        locals.var_t12_dn6 = assign101060_e153185_d_n6;
        locals.var_t12_dn7 = assign101060_e153185_d_n7;
        locals.var_t12_dn8 = assign101060_e153185_d_n8;
        locals.var_t12_dn9 = assign101060_e153185_d_n9;
        locals.var_t12_dn10 = assign101060_e153185_d_n10;
        locals.var_t12_dn13 = assign101060_e153185_d_n13;

        let (assign101070_e153191, assign101070_e153191_d_n0, assign101070_e153191_d_n2, assign101070_e153191_d_n4, assign101070_e153191_d_n5, assign101070_e153191_d_n6, assign101070_e153191_d_n7, assign101070_e153191_d_n8, assign101070_e153191_d_n9, assign101070_e153191_d_n10, assign101070_e153191_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101070_e153191;
        locals.var_t10_dn0 = assign101070_e153191_d_n0;
        locals.var_t10_dn2 = assign101070_e153191_d_n2;
        locals.var_t10_dn4 = assign101070_e153191_d_n4;
        locals.var_t10_dn5 = assign101070_e153191_d_n5;
        locals.var_t10_dn6 = assign101070_e153191_d_n6;
        locals.var_t10_dn7 = assign101070_e153191_d_n7;
        locals.var_t10_dn8 = assign101070_e153191_d_n8;
        locals.var_t10_dn9 = assign101070_e153191_d_n9;
        locals.var_t10_dn10 = assign101070_e153191_d_n10;
        locals.var_t10_dn13 = assign101070_e153191_d_n13;

        let (assign101080_e153197, assign101080_e153197_d_n0, assign101080_e153197_d_n2, assign101080_e153197_d_n4, assign101080_e153197_d_n5, assign101080_e153197_d_n6, assign101080_e153197_d_n7, assign101080_e153197_d_n8, assign101080_e153197_d_n9, assign101080_e153197_d_n10, assign101080_e153197_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101080_e153197;
        locals.var_t3_dn0 = assign101080_e153197_d_n0;
        locals.var_t3_dn2 = assign101080_e153197_d_n2;
        locals.var_t3_dn4 = assign101080_e153197_d_n4;
        locals.var_t3_dn5 = assign101080_e153197_d_n5;
        locals.var_t3_dn6 = assign101080_e153197_d_n6;
        locals.var_t3_dn7 = assign101080_e153197_d_n7;
        locals.var_t3_dn8 = assign101080_e153197_d_n8;
        locals.var_t3_dn9 = assign101080_e153197_d_n9;
        locals.var_t3_dn10 = assign101080_e153197_d_n10;
        locals.var_t3_dn13 = assign101080_e153197_d_n13;

        let (assign101090_e153209, assign101090_e153209_d_n0, assign101090_e153209_d_n2, assign101090_e153209_d_n4, assign101090_e153209_d_n5, assign101090_e153209_d_n6, assign101090_e153209_d_n7, assign101090_e153209_d_n8, assign101090_e153209_d_n9, assign101090_e153209_d_n10, assign101090_e153209_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        let assign101090_e153203: f64 = (locals.var_t12 * locals.var_t10);
        let assign101090_e153205: f64 = (assign101090_e153203 * locals.var_t3);
        let assign101090_e153207: f64 = (assign101090_e153205 * locals.var_t3);
        (assign101090_e153207, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn0)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn2)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn4)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn5)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn6)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn7)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn8)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn9)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn10)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn10)), ((((((locals.var_t12_dn13 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn13)) * locals.var_t3) + (assign101090_e153203 * locals.var_t3_dn13)) * locals.var_t3) + (assign101090_e153205 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign101090_e153209;
        locals.var_t1_dn0 = assign101090_e153209_d_n0;
        locals.var_t1_dn2 = assign101090_e153209_d_n2;
        locals.var_t1_dn4 = assign101090_e153209_d_n4;
        locals.var_t1_dn5 = assign101090_e153209_d_n5;
        locals.var_t1_dn6 = assign101090_e153209_d_n6;
        locals.var_t1_dn7 = assign101090_e153209_d_n7;
        locals.var_t1_dn8 = assign101090_e153209_d_n8;
        locals.var_t1_dn9 = assign101090_e153209_d_n9;
        locals.var_t1_dn10 = assign101090_e153209_d_n10;
        locals.var_t1_dn13 = assign101090_e153209_d_n13;

        let (assign101100_e153227, assign101100_e153227_d_n0, assign101100_e153227_d_n2, assign101100_e153227_d_n4, assign101100_e153227_d_n5, assign101100_e153227_d_n6, assign101100_e153227_d_n7, assign101100_e153227_d_n8, assign101100_e153227_d_n9, assign101100_e153227_d_n10, assign101100_e153227_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        let assign101100_e153215: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101100_e153217: f64 = (assign101100_e153215 * locals.var_t12);
        let assign101100_e153220: f64 = (locals.var_t10 * locals.var_t3);
        let assign101100_e153222: f64 = (assign101100_e153220 * locals.var_t3);
        let assign101100_e153223: f64 = (assign101100_e153217 + assign101100_e153222);
        let assign101100_e153225: f64 = (assign101100_e153223 + 1e-25);
        (assign101100_e153225, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn10))), (((((locals.var_mu_dn13 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn13)) * locals.var_t12) + (assign101100_e153215 * locals.var_t12_dn13)) + ((((locals.var_t10_dn13 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn13)) * locals.var_t3) + (assign101100_e153220 * locals.var_t3_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101100_e153227;
        locals.var_t2_dn0 = assign101100_e153227_d_n0;
        locals.var_t2_dn2 = assign101100_e153227_d_n2;
        locals.var_t2_dn4 = assign101100_e153227_d_n4;
        locals.var_t2_dn5 = assign101100_e153227_d_n5;
        locals.var_t2_dn6 = assign101100_e153227_d_n6;
        locals.var_t2_dn7 = assign101100_e153227_d_n7;
        locals.var_t2_dn8 = assign101100_e153227_d_n8;
        locals.var_t2_dn9 = assign101100_e153227_d_n9;
        locals.var_t2_dn10 = assign101100_e153227_d_n10;
        locals.var_t2_dn13 = assign101100_e153227_d_n13;

        let (assign101110_e153235, assign101110_e153235_d_n0, assign101110_e153235_d_n2, assign101110_e153235_d_n4, assign101110_e153235_d_n5, assign101110_e153235_d_n6, assign101110_e153235_d_n7, assign101110_e153235_d_n8, assign101110_e153235_d_n9, assign101110_e153235_d_n10, assign101110_e153235_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 != 0.0)) {
        let assign101110_e153233: f64 = (locals.var_t1 / locals.var_t2);
        (assign101110_e153233, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn7, locals.var_tau_dn8, locals.var_tau_dn9, locals.var_tau_dn10, locals.var_tau_dn13,)
    }
};
        locals.var_tau = assign101110_e153235;
        locals.var_tau_dn0 = assign101110_e153235_d_n0;
        locals.var_tau_dn2 = assign101110_e153235_d_n2;
        locals.var_tau_dn4 = assign101110_e153235_d_n4;
        locals.var_tau_dn5 = assign101110_e153235_d_n5;
        locals.var_tau_dn6 = assign101110_e153235_d_n6;
        locals.var_tau_dn7 = assign101110_e153235_d_n7;
        locals.var_tau_dn8 = assign101110_e153235_d_n8;
        locals.var_tau_dn9 = assign101110_e153235_d_n9;
        locals.var_tau_dn10 = assign101110_e153235_d_n10;
        locals.var_tau_dn13 = assign101110_e153235_d_n13;

        let (assign101120_e153242, assign101120_e153242_d_n0, assign101120_e153242_d_n2, assign101120_e153242_d_n4, assign101120_e153242_d_n5, assign101120_e153242_d_n6, assign101120_e153242_d_n7, assign101120_e153242_d_n8, assign101120_e153242_d_n9, assign101120_e153242_d_n10, assign101120_e153242_d_n13,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2321 == 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn7, locals.var_tau_dn8, locals.var_tau_dn9, locals.var_tau_dn10, locals.var_tau_dn13,)
    }
};
        locals.var_tau = assign101120_e153242;
        locals.var_tau_dn0 = assign101120_e153242_d_n0;
        locals.var_tau_dn2 = assign101120_e153242_d_n2;
        locals.var_tau_dn4 = assign101120_e153242_d_n4;
        locals.var_tau_dn5 = assign101120_e153242_d_n5;
        locals.var_tau_dn6 = assign101120_e153242_d_n6;
        locals.var_tau_dn7 = assign101120_e153242_d_n7;
        locals.var_tau_dn8 = assign101120_e153242_d_n8;
        locals.var_tau_dn9 = assign101120_e153242_d_n9;
        locals.var_tau_dn10 = assign101120_e153242_d_n10;
        locals.var_tau_dn13 = assign101120_e153242_d_n13;

        let (assign101130_e153246, assign101130_e153246_d_n0, assign101130_e153246_d_n2, assign101130_e153246_d_n4, assign101130_e153246_d_n5, assign101130_e153246_d_n6, assign101130_e153246_d_n7, assign101130_e153246_d_n8, assign101130_e153246_d_n9, assign101130_e153246_d_n10, assign101130_e153246_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101130_e153246;
        locals.var_t2_dn0 = assign101130_e153246_d_n0;
        locals.var_t2_dn2 = assign101130_e153246_d_n2;
        locals.var_t2_dn4 = assign101130_e153246_d_n4;
        locals.var_t2_dn5 = assign101130_e153246_d_n5;
        locals.var_t2_dn6 = assign101130_e153246_d_n6;
        locals.var_t2_dn7 = assign101130_e153246_d_n7;
        locals.var_t2_dn8 = assign101130_e153246_d_n8;
        locals.var_t2_dn9 = assign101130_e153246_d_n9;
        locals.var_t2_dn10 = assign101130_e153246_d_n10;
        locals.var_t2_dn13 = assign101130_e153246_d_n13;

        let (assign101140_e153252, assign101140_e153252_d_n0, assign101140_e153252_d_n2, assign101140_e153252_d_n4, assign101140_e153252_d_n5, assign101140_e153252_d_n6, assign101140_e153252_d_n7, assign101140_e153252_d_n8, assign101140_e153252_d_n9, assign101140_e153252_d_n10, assign101140_e153252_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign101140_e153250: f64 = (locals.var_t2 * locals.var_cox);
        (assign101140_e153250, ((locals.var_t2_dn0 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn0)), ((locals.var_t2_dn2 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn2)), ((locals.var_t2_dn4 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn4)), ((locals.var_t2_dn5 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn5)), ((locals.var_t2_dn6 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn6)), ((locals.var_t2_dn7 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn7)), ((locals.var_t2_dn8 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn8)), ((locals.var_t2_dn9 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn9)), ((locals.var_t2_dn10 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn10)), ((locals.var_t2_dn13 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn13)),)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn7, locals.var_taub_dn8, locals.var_taub_dn9, locals.var_taub_dn10, locals.var_taub_dn13,)
    }
};
        locals.var_taub = assign101140_e153252;
        locals.var_taub_dn0 = assign101140_e153252_d_n0;
        locals.var_taub_dn2 = assign101140_e153252_d_n2;
        locals.var_taub_dn4 = assign101140_e153252_d_n4;
        locals.var_taub_dn5 = assign101140_e153252_d_n5;
        locals.var_taub_dn6 = assign101140_e153252_d_n6;
        locals.var_taub_dn7 = assign101140_e153252_d_n7;
        locals.var_taub_dn8 = assign101140_e153252_d_n8;
        locals.var_taub_dn9 = assign101140_e153252_d_n9;
        locals.var_taub_dn10 = assign101140_e153252_d_n10;
        locals.var_taub_dn13 = assign101140_e153252_d_n13;

        let assign101150_e153258: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2322 = assign101150_e153258;

        let (assign101160_e153262,) = {
    if (locals.var_guard2322 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101160_e153262;

        let (assign101180_e153270,) = {
    if (locals.var_guard2322 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101180_e153270;

        let (assign101190_e153276, assign101190_e153276_d_n0, assign101190_e153276_d_n2, assign101190_e153276_d_n4, assign101190_e153276_d_n5, assign101190_e153276_d_n6, assign101190_e153276_d_n7, assign101190_e153276_d_n8, assign101190_e153276_d_n9, assign101190_e153276_d_n10, assign101190_e153276_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101190_e153274: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101190_e153274, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn13 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign101190_e153276;
        locals.var_t1_dn0 = assign101190_e153276_d_n0;
        locals.var_t1_dn2 = assign101190_e153276_d_n2;
        locals.var_t1_dn4 = assign101190_e153276_d_n4;
        locals.var_t1_dn5 = assign101190_e153276_d_n5;
        locals.var_t1_dn6 = assign101190_e153276_d_n6;
        locals.var_t1_dn7 = assign101190_e153276_d_n7;
        locals.var_t1_dn8 = assign101190_e153276_d_n8;
        locals.var_t1_dn9 = assign101190_e153276_d_n9;
        locals.var_t1_dn10 = assign101190_e153276_d_n10;
        locals.var_t1_dn13 = assign101190_e153276_d_n13;

        let (assign101200_e153293, assign101200_e153293_d_n0, assign101200_e153293_d_n2, assign101200_e153293_d_n4, assign101200_e153293_d_n5, assign101200_e153293_d_n6, assign101200_e153293_d_n7, assign101200_e153293_d_n8, assign101200_e153293_d_n9, assign101200_e153293_d_n10, assign101200_e153293_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101200_e153280: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101200_e153283: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101200_e153284: f64 = (assign101200_e153280 * assign101200_e153283);
        let assign101200_e153287: f64 = (4.0 * 0.001);
        let assign101200_e153289: f64 = (assign101200_e153287 * 0.001);
        let assign101200_e153290: f64 = (assign101200_e153284 + assign101200_e153289);
        let assign101200_e153291: f64 = (assign101200_e153290).sqrt();
        (assign101200_e153291, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10))) / (2.0 * assign101200_e153291)), ((((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) * assign101200_e153283) + (assign101200_e153280 * (locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13))) / (2.0 * assign101200_e153291)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign101200_e153293;
        locals.var_tmf2_dn0 = assign101200_e153293_d_n0;
        locals.var_tmf2_dn2 = assign101200_e153293_d_n2;
        locals.var_tmf2_dn4 = assign101200_e153293_d_n4;
        locals.var_tmf2_dn5 = assign101200_e153293_d_n5;
        locals.var_tmf2_dn6 = assign101200_e153293_d_n6;
        locals.var_tmf2_dn7 = assign101200_e153293_d_n7;
        locals.var_tmf2_dn8 = assign101200_e153293_d_n8;
        locals.var_tmf2_dn9 = assign101200_e153293_d_n9;
        locals.var_tmf2_dn10 = assign101200_e153293_d_n10;
        locals.var_tmf2_dn13 = assign101200_e153293_d_n13;

        let (assign101210_e153305, assign101210_e153305_d_n0, assign101210_e153305_d_n2, assign101210_e153305_d_n4, assign101210_e153305_d_n5, assign101210_e153305_d_n6, assign101210_e153305_d_n7, assign101210_e153305_d_n8, assign101210_e153305_d_n9, assign101210_e153305_d_n10, assign101210_e153305_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101210_e153299: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101210_e153301: f64 = (assign101210_e153299 / locals.var_tmf2);
        let assign101210_e153302: f64 = (1.0 + assign101210_e153301);
        let assign101210_e153303: f64 = (0.5 * assign101210_e153302);
        (assign101210_e153303, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) * locals.var_tmf2) - (assign101210_e153299 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101210_e153305;
        locals.var_t0_dn0 = assign101210_e153305_d_n0;
        locals.var_t0_dn2 = assign101210_e153305_d_n2;
        locals.var_t0_dn4 = assign101210_e153305_d_n4;
        locals.var_t0_dn5 = assign101210_e153305_d_n5;
        locals.var_t0_dn6 = assign101210_e153305_d_n6;
        locals.var_t0_dn7 = assign101210_e153305_d_n7;
        locals.var_t0_dn8 = assign101210_e153305_d_n8;
        locals.var_t0_dn9 = assign101210_e153305_d_n9;
        locals.var_t0_dn10 = assign101210_e153305_d_n10;
        locals.var_t0_dn13 = assign101210_e153305_d_n13;

        let (assign101220_e153315, assign101220_e153315_d_n0, assign101220_e153315_d_n2, assign101220_e153315_d_n4, assign101220_e153315_d_n5, assign101220_e153315_d_n6, assign101220_e153315_d_n7, assign101220_e153315_d_n8, assign101220_e153315_d_n9, assign101220_e153315_d_n10, assign101220_e153315_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101220_e153310: f64 = (locals.var_ps0 - locals.var_vbscl__blk435);
        let assign101220_e153312: f64 = (assign101220_e153310 + locals.var_tmf2);
        let assign101220_e153313: f64 = (0.5 * assign101220_e153312);
        (assign101220_e153313, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101220_e153315;
        locals.var_t5_dn0 = assign101220_e153315_d_n0;
        locals.var_t5_dn2 = assign101220_e153315_d_n2;
        locals.var_t5_dn4 = assign101220_e153315_d_n4;
        locals.var_t5_dn5 = assign101220_e153315_d_n5;
        locals.var_t5_dn6 = assign101220_e153315_d_n6;
        locals.var_t5_dn7 = assign101220_e153315_d_n7;
        locals.var_t5_dn8 = assign101220_e153315_d_n8;
        locals.var_t5_dn9 = assign101220_e153315_d_n9;
        locals.var_t5_dn10 = assign101220_e153315_d_n10;
        locals.var_t5_dn13 = assign101220_e153315_d_n13;

        let assign101230_e153318: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign101230_e153318;

        let (assign101240_e153324, assign101240_e153324_d_n0, assign101240_e153324_d_n2, assign101240_e153324_d_n4, assign101240_e153324_d_n5, assign101240_e153324_d_n6, assign101240_e153324_d_n7, assign101240_e153324_d_n8, assign101240_e153324_d_n9, assign101240_e153324_d_n10, assign101240_e153324_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101240_e153324;
        locals.var_t5_dn0 = assign101240_e153324_d_n0;
        locals.var_t5_dn2 = assign101240_e153324_d_n2;
        locals.var_t5_dn4 = assign101240_e153324_d_n4;
        locals.var_t5_dn5 = assign101240_e153324_d_n5;
        locals.var_t5_dn6 = assign101240_e153324_d_n6;
        locals.var_t5_dn7 = assign101240_e153324_d_n7;
        locals.var_t5_dn8 = assign101240_e153324_d_n8;
        locals.var_t5_dn9 = assign101240_e153324_d_n9;
        locals.var_t5_dn10 = assign101240_e153324_d_n10;
        locals.var_t5_dn13 = assign101240_e153324_d_n13;

        let (assign101250_e153330, assign101250_e153330_d_n0, assign101250_e153330_d_n2, assign101250_e153330_d_n4, assign101250_e153330_d_n5, assign101250_e153330_d_n6, assign101250_e153330_d_n7, assign101250_e153330_d_n8, assign101250_e153330_d_n9, assign101250_e153330_d_n10, assign101250_e153330_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101250_e153330;
        locals.var_t0_dn0 = assign101250_e153330_d_n0;
        locals.var_t0_dn2 = assign101250_e153330_d_n2;
        locals.var_t0_dn4 = assign101250_e153330_d_n4;
        locals.var_t0_dn5 = assign101250_e153330_d_n5;
        locals.var_t0_dn6 = assign101250_e153330_d_n6;
        locals.var_t0_dn7 = assign101250_e153330_d_n7;
        locals.var_t0_dn8 = assign101250_e153330_d_n8;
        locals.var_t0_dn9 = assign101250_e153330_d_n9;
        locals.var_t0_dn10 = assign101250_e153330_d_n10;
        locals.var_t0_dn13 = assign101250_e153330_d_n13;

        let (assign101260_e153344, assign101260_e153344_d_n0, assign101260_e153344_d_n2, assign101260_e153344_d_n4, assign101260_e153344_d_n5, assign101260_e153344_d_n6, assign101260_e153344_d_n7, assign101260_e153344_d_n8, assign101260_e153344_d_n9, assign101260_e153344_d_n10, assign101260_e153344_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101260_e153335: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101260_e153336: f64 = (locals.var_cox + assign101260_e153335);
        let assign101260_e153338: f64 = (assign101260_e153336 + locals.var_cite);
        let assign101260_e153340: f64 = (assign101260_e153338 * locals.var_beta_inv);
        let assign101260_e153342: f64 = (assign101260_e153340 / 1.6021918e-19);
        (assign101260_e153342, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn13 + (((locals.var_qn0_dn13 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101260_e153338 * locals.var_beta_inv_dn13)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101260_e153344;
        locals.var_t2_dn0 = assign101260_e153344_d_n0;
        locals.var_t2_dn2 = assign101260_e153344_d_n2;
        locals.var_t2_dn4 = assign101260_e153344_d_n4;
        locals.var_t2_dn5 = assign101260_e153344_d_n5;
        locals.var_t2_dn6 = assign101260_e153344_d_n6;
        locals.var_t2_dn7 = assign101260_e153344_d_n7;
        locals.var_t2_dn8 = assign101260_e153344_d_n8;
        locals.var_t2_dn9 = assign101260_e153344_d_n9;
        locals.var_t2_dn10 = assign101260_e153344_d_n10;
        locals.var_t2_dn13 = assign101260_e153344_d_n13;

        let (assign101270_e153359, assign101270_e153359_d_n0, assign101270_e153359_d_n2, assign101270_e153359_d_n4, assign101270_e153359_d_n5, assign101270_e153359_d_n6, assign101270_e153359_d_n7, assign101270_e153359_d_n8, assign101270_e153359_d_n9, assign101270_e153359_d_n10, assign101270_e153359_d_n13,) = {
    if (locals.var_guard2322 != 0.0) {
        let assign101270_e153347: f64 = (-2.0);
        let assign101270_e153349: f64 = (assign101270_e153347 * locals.var_qi_noi);
        let assign101270_e153351: f64 = (assign101270_e153349 / 1.6021918e-19);
        let assign101270_e153353: f64 = (assign101270_e153351 / locals.var_lch);
        let assign101270_e153355: f64 = (assign101270_e153353 / locals.var_weffcv_nf);
        let assign101270_e153357: f64 = (assign101270_e153355 - locals.var_t1);
        (assign101270_e153357, (((((((assign101270_e153347 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101270_e153347 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101270_e153347 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101270_e153347 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101270_e153347 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101270_e153347 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101270_e153347 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101270_e153347 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101270_e153347 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101270_e153347 * locals.var_qi_noi_dn13) / 1.6021918e-19) * locals.var_lch) - (assign101270_e153351 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101270_e153359;
        locals.var_t3_dn0 = assign101270_e153359_d_n0;
        locals.var_t3_dn2 = assign101270_e153359_d_n2;
        locals.var_t3_dn4 = assign101270_e153359_d_n4;
        locals.var_t3_dn5 = assign101270_e153359_d_n5;
        locals.var_t3_dn6 = assign101270_e153359_d_n6;
        locals.var_t3_dn7 = assign101270_e153359_d_n7;
        locals.var_t3_dn8 = assign101270_e153359_d_n8;
        locals.var_t3_dn9 = assign101270_e153359_d_n9;
        locals.var_t3_dn10 = assign101270_e153359_d_n10;
        locals.var_t3_dn13 = assign101270_e153359_d_n13;

        let assign101280_e153362: f64 = (locals.var_t3 - locals.var_t1);
        let assign101280_e153363: f64 = (assign101280_e153362).abs();
        let assign101280_e153366: f64 = (10.0 * 2.220446049250313e-16);
        let assign101280_e153367: f64 = if assign101280_e153363 > assign101280_e153366 { 1.0 } else { 0.0 };
        locals.var_guard2324 = assign101280_e153367;

        let (assign101290_e153414, assign101290_e153414_d_n0, assign101290_e153414_d_n2, assign101290_e153414_d_n4, assign101290_e153414_d_n5, assign101290_e153414_d_n6, assign101290_e153414_d_n7, assign101290_e153414_d_n8, assign101290_e153414_d_n9, assign101290_e153414_d_n10, assign101290_e153414_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2324 != 0.0)) {
        let assign101290_e153374: f64 = (locals.var_t1 + locals.var_t2);
        let assign101290_e153375: f64 = (1.0 / assign101290_e153374);
        let assign101290_e153378: f64 = (locals.var_t3 + locals.var_t2);
        let assign101290_e153379: f64 = (assign101290_e153375 / assign101290_e153378);
        let assign101290_e153382: f64 = (2.0 * locals.var_nfalpe);
        let assign101290_e153384: f64 = (assign101290_e153382 * locals.var_ey);
        let assign101290_e153386: f64 = (assign101290_e153384 * locals.var_mu);
        let assign101290_e153389: f64 = (locals.var_t3 - locals.var_t1);
        let assign101290_e153390: f64 = (assign101290_e153386 / assign101290_e153389);
        let assign101290_e153393: f64 = (locals.var_t3 + locals.var_t2);
        let assign101290_e153396: f64 = (locals.var_t1 + locals.var_t2);
        let assign101290_e153397: f64 = (assign101290_e153393 / assign101290_e153396);
        let assign101290_e153398: f64 = (assign101290_e153397).ln();
        let assign101290_e153399: f64 = (assign101290_e153390 * assign101290_e153398);
        let assign101290_e153400: f64 = (assign101290_e153379 + assign101290_e153399);
        let assign101290_e153403: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101290_e153405: f64 = (assign101290_e153403 * locals.var_mu);
        let assign101290_e153407: f64 = (assign101290_e153405 * locals.var_nfalpe);
        let assign101290_e153409: f64 = (assign101290_e153407 * locals.var_ey);
        let assign101290_e153411: f64 = (assign101290_e153409 * locals.var_mu);
        let assign101290_e153412: f64 = (assign101290_e153400 + assign101290_e153411);
        (assign101290_e153412, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn0) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn0)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn0)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn2) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn2)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn2)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn4) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn4)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn4)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn5) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn5)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn5)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn6) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn6)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn6)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn7) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn7)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn7)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn8) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn8)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn8)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn9) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn9)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn9)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn10) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn10)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn10)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn13 + locals.var_t2_dn13) / (assign101290_e153374 * assign101290_e153374))) * assign101290_e153378) - (assign101290_e153375 * (locals.var_t3_dn13 + locals.var_t2_dn13))) / (assign101290_e153378 * assign101290_e153378)) + ((((((((assign101290_e153382 * locals.var_ey_dn13) * locals.var_mu) + (assign101290_e153384 * locals.var_mu_dn13)) * assign101290_e153389) - (assign101290_e153386 * (locals.var_t3_dn13 - locals.var_t1_dn13))) / (assign101290_e153389 * assign101290_e153389)) * assign101290_e153398) + (assign101290_e153390 * (((((locals.var_t3_dn13 + locals.var_t2_dn13) * assign101290_e153396) - (assign101290_e153393 * (locals.var_t1_dn13 + locals.var_t2_dn13))) / (assign101290_e153396 * assign101290_e153396)) / assign101290_e153397)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn13) * locals.var_mu) + (assign101290_e153403 * locals.var_mu_dn13)) * locals.var_nfalpe) * locals.var_ey) + (assign101290_e153407 * locals.var_ey_dn13)) * locals.var_mu) + (assign101290_e153409 * locals.var_mu_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101290_e153414;
        locals.var_t4_dn0 = assign101290_e153414_d_n0;
        locals.var_t4_dn2 = assign101290_e153414_d_n2;
        locals.var_t4_dn4 = assign101290_e153414_d_n4;
        locals.var_t4_dn5 = assign101290_e153414_d_n5;
        locals.var_t4_dn6 = assign101290_e153414_d_n6;
        locals.var_t4_dn7 = assign101290_e153414_d_n7;
        locals.var_t4_dn8 = assign101290_e153414_d_n8;
        locals.var_t4_dn9 = assign101290_e153414_d_n9;
        locals.var_t4_dn10 = assign101290_e153414_d_n10;
        locals.var_t4_dn13 = assign101290_e153414_d_n13;

    }

    pub(super) fn stamp_transient_block_359(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101300_e153453, assign101300_e153453_d_n0, assign101300_e153453_d_n2, assign101300_e153453_d_n4, assign101300_e153453_d_n5, assign101300_e153453_d_n6, assign101300_e153453_d_n7, assign101300_e153453_d_n8, assign101300_e153453_d_n9, assign101300_e153453_d_n10, assign101300_e153453_d_n13,) = {
    if ((locals.var_guard2322 != 0.0) && (locals.var_guard2324 == 0.0)) {
        let assign101300_e153422: f64 = (locals.var_t1 + locals.var_t2);
        let assign101300_e153423: f64 = (1.0 / assign101300_e153422);
        let assign101300_e153426: f64 = (locals.var_t3 + locals.var_t2);
        let assign101300_e153427: f64 = (assign101300_e153423 / assign101300_e153426);
        let assign101300_e153430: f64 = (2.0 * locals.var_nfalpe);
        let assign101300_e153432: f64 = (assign101300_e153430 * locals.var_ey);
        let assign101300_e153434: f64 = (assign101300_e153432 * locals.var_mu);
        let assign101300_e153437: f64 = (locals.var_t1 + locals.var_t2);
        let assign101300_e153438: f64 = (assign101300_e153434 / assign101300_e153437);
        let assign101300_e153439: f64 = (assign101300_e153427 + assign101300_e153438);
        let assign101300_e153442: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101300_e153444: f64 = (assign101300_e153442 * locals.var_mu);
        let assign101300_e153446: f64 = (assign101300_e153444 * locals.var_nfalpe);
        let assign101300_e153448: f64 = (assign101300_e153446 * locals.var_ey);
        let assign101300_e153450: f64 = (assign101300_e153448 * locals.var_mu);
        let assign101300_e153451: f64 = (assign101300_e153439 + assign101300_e153450);
        (assign101300_e153451, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn0) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn0)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn0)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn2) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn2)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn2)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn4) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn4)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn4)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn5) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn5)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn5)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn6) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn6)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn6)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn7) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn7)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn7)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn8) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn8)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn8)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn9) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn9)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn9)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn10) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn10)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn10)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn13 + locals.var_t2_dn13) / (assign101300_e153422 * assign101300_e153422))) * assign101300_e153426) - (assign101300_e153423 * (locals.var_t3_dn13 + locals.var_t2_dn13))) / (assign101300_e153426 * assign101300_e153426)) + ((((((assign101300_e153430 * locals.var_ey_dn13) * locals.var_mu) + (assign101300_e153432 * locals.var_mu_dn13)) * assign101300_e153437) - (assign101300_e153434 * (locals.var_t1_dn13 + locals.var_t2_dn13))) / (assign101300_e153437 * assign101300_e153437))) + ((((((((locals.var_nfalpe * locals.var_ey_dn13) * locals.var_mu) + (assign101300_e153442 * locals.var_mu_dn13)) * locals.var_nfalpe) * locals.var_ey) + (assign101300_e153446 * locals.var_ey_dn13)) * locals.var_mu) + (assign101300_e153448 * locals.var_mu_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101300_e153453;
        locals.var_t4_dn0 = assign101300_e153453_d_n0;
        locals.var_t4_dn2 = assign101300_e153453_d_n2;
        locals.var_t4_dn4 = assign101300_e153453_d_n4;
        locals.var_t4_dn5 = assign101300_e153453_d_n5;
        locals.var_t4_dn6 = assign101300_e153453_d_n6;
        locals.var_t4_dn7 = assign101300_e153453_d_n7;
        locals.var_t4_dn8 = assign101300_e153453_d_n8;
        locals.var_t4_dn9 = assign101300_e153453_d_n9;
        locals.var_t4_dn10 = assign101300_e153453_d_n10;
        locals.var_t4_dn13 = assign101300_e153453_d_n13;

        let assign101330_e153484: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101330_e153484;

        let (assign101340_e153496, assign101340_e153496_d_n0, assign101340_e153496_d_n2, assign101340_e153496_d_n4, assign101340_e153496_d_n5, assign101340_e153496_d_n6, assign101340_e153496_d_n7, assign101340_e153496_d_n8, assign101340_e153496_d_n9, assign101340_e153496_d_n10, assign101340_e153496_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101340_e153488: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101340_e153491: f64 = (10.0 * 2.220446049250313e-16);
        let assign101340_e153492: f64 = (assign101340_e153488 + assign101340_e153491);
        let assign101340_e153494: f64 = (assign101340_e153492 / locals.var_lch);
        (assign101340_e153494, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn13 - locals.var_ps0_dn13) * locals.var_lch) - (assign101340_e153492 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
    }
};
        locals.var_eyd = assign101340_e153496;
        locals.var_eyd_dn0 = assign101340_e153496_d_n0;
        locals.var_eyd_dn2 = assign101340_e153496_d_n2;
        locals.var_eyd_dn4 = assign101340_e153496_d_n4;
        locals.var_eyd_dn5 = assign101340_e153496_d_n5;
        locals.var_eyd_dn6 = assign101340_e153496_d_n6;
        locals.var_eyd_dn7 = assign101340_e153496_d_n7;
        locals.var_eyd_dn8 = assign101340_e153496_d_n8;
        locals.var_eyd_dn9 = assign101340_e153496_d_n9;
        locals.var_eyd_dn10 = assign101340_e153496_d_n10;
        locals.var_eyd_dn13 = assign101340_e153496_d_n13;

        let (assign101350_e153505, assign101350_e153505_d_n0, assign101350_e153505_d_n2, assign101350_e153505_d_n4, assign101350_e153505_d_n5, assign101350_e153505_d_n6, assign101350_e153505_d_n7, assign101350_e153505_d_n8, assign101350_e153505_d_n9, assign101350_e153505_d_n10, assign101350_e153505_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let (assign101350_e153503, assign101350_e153503_d_n0, assign101350_e153503_d_n2, assign101350_e153503_d_n4, assign101350_e153503_d_n5, assign101350_e153503_d_n6, assign101350_e153503_d_n7, assign101350_e153503_d_n8, assign101350_e153503_d_n9, assign101350_e153503_d_n10, assign101350_e153503_d_n13,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101350_e153503, assign101350_e153503_d_n0, assign101350_e153503_d_n2, assign101350_e153503_d_n4, assign101350_e153503_d_n5, assign101350_e153503_d_n6, assign101350_e153503_d_n7, assign101350_e153503_d_n8, assign101350_e153503_d_n9, assign101350_e153503_d_n10, assign101350_e153503_d_n13,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn13,)
    }
};
        locals.var_eyd = assign101350_e153505;
        locals.var_eyd_dn0 = assign101350_e153505_d_n0;
        locals.var_eyd_dn2 = assign101350_e153505_d_n2;
        locals.var_eyd_dn4 = assign101350_e153505_d_n4;
        locals.var_eyd_dn5 = assign101350_e153505_d_n5;
        locals.var_eyd_dn6 = assign101350_e153505_d_n6;
        locals.var_eyd_dn7 = assign101350_e153505_d_n7;
        locals.var_eyd_dn8 = assign101350_e153505_d_n8;
        locals.var_eyd_dn9 = assign101350_e153505_d_n9;
        locals.var_eyd_dn10 = assign101350_e153505_d_n10;
        locals.var_eyd_dn13 = assign101350_e153505_d_n13;

        let (assign101360_e153513, assign101360_e153513_d_n0, assign101360_e153513_d_n2, assign101360_e153513_d_n4, assign101360_e153513_d_n5, assign101360_e153513_d_n6, assign101360_e153513_d_n7, assign101360_e153513_d_n8, assign101360_e153513_d_n9, assign101360_e153513_d_n10, assign101360_e153513_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101360_e153509: f64 = (locals.var_muun * locals.var_eyd);
        let assign101360_e153511: f64 = (assign101360_e153509 / 10000000.0);
        (assign101360_e153511, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn13 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn13)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign101360_e153513;
        locals.var_t12_dn0 = assign101360_e153513_d_n0;
        locals.var_t12_dn2 = assign101360_e153513_d_n2;
        locals.var_t12_dn4 = assign101360_e153513_d_n4;
        locals.var_t12_dn5 = assign101360_e153513_d_n5;
        locals.var_t12_dn6 = assign101360_e153513_d_n6;
        locals.var_t12_dn7 = assign101360_e153513_d_n7;
        locals.var_t12_dn8 = assign101360_e153513_d_n8;
        locals.var_t12_dn9 = assign101360_e153513_d_n9;
        locals.var_t12_dn10 = assign101360_e153513_d_n10;
        locals.var_t12_dn13 = assign101360_e153513_d_n13;

        let assign101370_e153517: f64 = (10.0 * 2.220446049250313e-16);
        let assign101370_e153518: f64 = (1.0 - assign101370_e153517);
        let assign101370_e153525: f64 = (10.0 * 2.220446049250313e-16);
        let assign101370_e153526: f64 = (1.0 + assign101370_e153525);
        let assign101370_e153528: f64 = if ((assign101370_e153518 <= p.p178) && (p.p178 <= assign101370_e153526)) { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101370_e153528;

        let (assign101380_e153534, assign101380_e153534_d_n0, assign101380_e153534_d_n2, assign101380_e153534_d_n4, assign101380_e153534_d_n5, assign101380_e153534_d_n6, assign101380_e153534_d_n7, assign101380_e153534_d_n8, assign101380_e153534_d_n9, assign101380_e153534_d_n10, assign101380_e153534_d_n13,) = {
    if ((locals.var_guard2325 != 0.0) && (locals.var_guard2326 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101380_e153534;
        locals.var_t7_dn0 = assign101380_e153534_d_n0;
        locals.var_t7_dn2 = assign101380_e153534_d_n2;
        locals.var_t7_dn4 = assign101380_e153534_d_n4;
        locals.var_t7_dn5 = assign101380_e153534_d_n5;
        locals.var_t7_dn6 = assign101380_e153534_d_n6;
        locals.var_t7_dn7 = assign101380_e153534_d_n7;
        locals.var_t7_dn8 = assign101380_e153534_d_n8;
        locals.var_t7_dn9 = assign101380_e153534_d_n9;
        locals.var_t7_dn10 = assign101380_e153534_d_n10;
        locals.var_t7_dn13 = assign101380_e153534_d_n13;

        let assign101390_e153538: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153539: f64 = (2.0 - assign101390_e153538);
        let assign101390_e153546: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153547: f64 = (2.0 + assign101390_e153546);
        let assign101390_e153549: f64 = if ((assign101390_e153539 <= p.p178) && (p.p178 <= assign101390_e153547)) { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101390_e153549;

        let (assign101400_e153558, assign101400_e153558_d_n0, assign101400_e153558_d_n2, assign101400_e153558_d_n4, assign101400_e153558_d_n5, assign101400_e153558_d_n6, assign101400_e153558_d_n7, assign101400_e153558_d_n8, assign101400_e153558_d_n9, assign101400_e153558_d_n10, assign101400_e153558_d_n13,) = {
    if (((locals.var_guard2325 != 0.0) && (locals.var_guard2326 == 0.0)) && (locals.var_guard2327 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101400_e153558;
        locals.var_t7_dn0 = assign101400_e153558_d_n0;
        locals.var_t7_dn2 = assign101400_e153558_d_n2;
        locals.var_t7_dn4 = assign101400_e153558_d_n4;
        locals.var_t7_dn5 = assign101400_e153558_d_n5;
        locals.var_t7_dn6 = assign101400_e153558_d_n6;
        locals.var_t7_dn7 = assign101400_e153558_d_n7;
        locals.var_t7_dn8 = assign101400_e153558_d_n8;
        locals.var_t7_dn9 = assign101400_e153558_d_n9;
        locals.var_t7_dn10 = assign101400_e153558_d_n10;
        locals.var_t7_dn13 = assign101400_e153558_d_n13;

        let (assign101410_e153577, assign101410_e153577_d_n0, assign101410_e153577_d_n2, assign101410_e153577_d_n4, assign101410_e153577_d_n5, assign101410_e153577_d_n6, assign101410_e153577_d_n7, assign101410_e153577_d_n8, assign101410_e153577_d_n9, assign101410_e153577_d_n10, assign101410_e153577_d_n13,) = {
    if (((locals.var_guard2325 != 0.0) && (locals.var_guard2326 == 0.0)) && (locals.var_guard2327 == 0.0)) {
        let (assign101410_e153575, assign101410_e153575_d_n0, assign101410_e153575_d_n2, assign101410_e153575_d_n4, assign101410_e153575_d_n5, assign101410_e153575_d_n6, assign101410_e153575_d_n7, assign101410_e153575_d_n8, assign101410_e153575_d_n9, assign101410_e153575_d_n10, assign101410_e153575_d_n13,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101410_e153573: f64 = (p.p178 - 1.0);
                let assign101410_e153574: f64 = (locals.var_eyd).powf(assign101410_e153573);
                (assign101410_e153574, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101410_e153573) as f64).is_finite() && ((assign101410_e153573) as f64).fract() == 0.0 { if assign101410_e153573 == 0.0 { 0.0 } else { (assign101410_e153573 * ((locals.var_eyd).powf(assign101410_e153573 - 1.0) * locals.var_eyd_dn13)) } } else { (assign101410_e153574 * (assign101410_e153573 * (locals.var_eyd_dn13 / locals.var_eyd))) },)
            }
        };
        (assign101410_e153575, assign101410_e153575_d_n0, assign101410_e153575_d_n2, assign101410_e153575_d_n4, assign101410_e153575_d_n5, assign101410_e153575_d_n6, assign101410_e153575_d_n7, assign101410_e153575_d_n8, assign101410_e153575_d_n9, assign101410_e153575_d_n10, assign101410_e153575_d_n13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101410_e153577;
        locals.var_t7_dn0 = assign101410_e153577_d_n0;
        locals.var_t7_dn2 = assign101410_e153577_d_n2;
        locals.var_t7_dn4 = assign101410_e153577_d_n4;
        locals.var_t7_dn5 = assign101410_e153577_d_n5;
        locals.var_t7_dn6 = assign101410_e153577_d_n6;
        locals.var_t7_dn7 = assign101410_e153577_d_n7;
        locals.var_t7_dn8 = assign101410_e153577_d_n8;
        locals.var_t7_dn9 = assign101410_e153577_d_n9;
        locals.var_t7_dn10 = assign101410_e153577_d_n10;
        locals.var_t7_dn13 = assign101410_e153577_d_n13;

        let (assign101420_e153583, assign101420_e153583_d_n0, assign101420_e153583_d_n2, assign101420_e153583_d_n4, assign101420_e153583_d_n5, assign101420_e153583_d_n6, assign101420_e153583_d_n7, assign101420_e153583_d_n8, assign101420_e153583_d_n9, assign101420_e153583_d_n10, assign101420_e153583_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101420_e153581: f64 = (locals.var_t12 * locals.var_t7);
        (assign101420_e153581, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn13 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign101420_e153583;
        locals.var_t8_dn0 = assign101420_e153583_d_n0;
        locals.var_t8_dn2 = assign101420_e153583_d_n2;
        locals.var_t8_dn4 = assign101420_e153583_d_n4;
        locals.var_t8_dn5 = assign101420_e153583_d_n5;
        locals.var_t8_dn6 = assign101420_e153583_d_n6;
        locals.var_t8_dn7 = assign101420_e153583_d_n7;
        locals.var_t8_dn8 = assign101420_e153583_d_n8;
        locals.var_t8_dn9 = assign101420_e153583_d_n9;
        locals.var_t8_dn10 = assign101420_e153583_d_n10;
        locals.var_t8_dn13 = assign101420_e153583_d_n13;

        let (assign101430_e153589, assign101430_e153589_d_n0, assign101430_e153589_d_n2, assign101430_e153589_d_n4, assign101430_e153589_d_n5, assign101430_e153589_d_n6, assign101430_e153589_d_n7, assign101430_e153589_d_n8, assign101430_e153589_d_n9, assign101430_e153589_d_n10, assign101430_e153589_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101430_e153587: f64 = (1.0 + locals.var_t8);
        (assign101430_e153587, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign101430_e153589;
        locals.var_t9_dn0 = assign101430_e153589_d_n0;
        locals.var_t9_dn2 = assign101430_e153589_d_n2;
        locals.var_t9_dn4 = assign101430_e153589_d_n4;
        locals.var_t9_dn5 = assign101430_e153589_d_n5;
        locals.var_t9_dn6 = assign101430_e153589_d_n6;
        locals.var_t9_dn7 = assign101430_e153589_d_n7;
        locals.var_t9_dn8 = assign101430_e153589_d_n8;
        locals.var_t9_dn9 = assign101430_e153589_d_n9;
        locals.var_t9_dn10 = assign101430_e153589_d_n10;
        locals.var_t9_dn13 = assign101430_e153589_d_n13;

        let (assign101440_e153605, assign101440_e153605_d_n0, assign101440_e153605_d_n2, assign101440_e153605_d_n4, assign101440_e153605_d_n5, assign101440_e153605_d_n6, assign101440_e153605_d_n7, assign101440_e153605_d_n8, assign101440_e153605_d_n9, assign101440_e153605_d_n10, assign101440_e153605_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let (assign101440_e153603, assign101440_e153603_d_n0, assign101440_e153603_d_n2, assign101440_e153603_d_n4, assign101440_e153603_d_n5, assign101440_e153603_d_n6, assign101440_e153603_d_n7, assign101440_e153603_d_n8, assign101440_e153603_d_n9, assign101440_e153603_d_n10, assign101440_e153603_d_n13,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101440_e153597: f64 = (-1.0);
                let assign101440_e153599: f64 = (assign101440_e153597 / p.p178);
                let assign101440_e153601: f64 = (assign101440_e153599 - 1.0);
                let assign101440_e153602: f64 = (locals.var_t9).powf(assign101440_e153601);
                (assign101440_e153602, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn0)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn2)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn4)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn5)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn6)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn7)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn8)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn9)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn10)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101440_e153601) as f64).is_finite() && ((assign101440_e153601) as f64).fract() == 0.0 { if assign101440_e153601 == 0.0 { 0.0 } else { (assign101440_e153601 * ((locals.var_t9).powf(assign101440_e153601 - 1.0) * locals.var_t9_dn13)) } } else { (assign101440_e153602 * (assign101440_e153601 * (locals.var_t9_dn13 / locals.var_t9))) },)
            }
        };
        (assign101440_e153603, assign101440_e153603_d_n0, assign101440_e153603_d_n2, assign101440_e153603_d_n4, assign101440_e153603_d_n5, assign101440_e153603_d_n6, assign101440_e153603_d_n7, assign101440_e153603_d_n8, assign101440_e153603_d_n9, assign101440_e153603_d_n10, assign101440_e153603_d_n13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101440_e153605;
        locals.var_t10_dn0 = assign101440_e153605_d_n0;
        locals.var_t10_dn2 = assign101440_e153605_d_n2;
        locals.var_t10_dn4 = assign101440_e153605_d_n4;
        locals.var_t10_dn5 = assign101440_e153605_d_n5;
        locals.var_t10_dn6 = assign101440_e153605_d_n6;
        locals.var_t10_dn7 = assign101440_e153605_d_n7;
        locals.var_t10_dn8 = assign101440_e153605_d_n8;
        locals.var_t10_dn9 = assign101440_e153605_d_n9;
        locals.var_t10_dn10 = assign101440_e153605_d_n10;
        locals.var_t10_dn13 = assign101440_e153605_d_n13;

        let (assign101450_e153611, assign101450_e153611_d_n0, assign101450_e153611_d_n2, assign101450_e153611_d_n4, assign101450_e153611_d_n5, assign101450_e153611_d_n6, assign101450_e153611_d_n7, assign101450_e153611_d_n8, assign101450_e153611_d_n9, assign101450_e153611_d_n10, assign101450_e153611_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101450_e153609: f64 = (locals.var_t9 * locals.var_t10);
        (assign101450_e153609, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn13 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign101450_e153611;
        locals.var_t11_dn0 = assign101450_e153611_d_n0;
        locals.var_t11_dn2 = assign101450_e153611_d_n2;
        locals.var_t11_dn4 = assign101450_e153611_d_n4;
        locals.var_t11_dn5 = assign101450_e153611_d_n5;
        locals.var_t11_dn6 = assign101450_e153611_d_n6;
        locals.var_t11_dn7 = assign101450_e153611_d_n7;
        locals.var_t11_dn8 = assign101450_e153611_d_n8;
        locals.var_t11_dn9 = assign101450_e153611_d_n9;
        locals.var_t11_dn10 = assign101450_e153611_d_n10;
        locals.var_t11_dn13 = assign101450_e153611_d_n13;

        let (assign101460_e153617, assign101460_e153617_d_n0, assign101460_e153617_d_n2, assign101460_e153617_d_n4, assign101460_e153617_d_n5, assign101460_e153617_d_n6, assign101460_e153617_d_n7, assign101460_e153617_d_n8, assign101460_e153617_d_n9, assign101460_e153617_d_n10, assign101460_e153617_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101460_e153615: f64 = (locals.var_muun * locals.var_t11);
        (assign101460_e153615, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn13 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn13)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn13,)
    }
};
        locals.var_mud_hoso = assign101460_e153617;
        locals.var_mud_hoso_dn0 = assign101460_e153617_d_n0;
        locals.var_mud_hoso_dn2 = assign101460_e153617_d_n2;
        locals.var_mud_hoso_dn4 = assign101460_e153617_d_n4;
        locals.var_mud_hoso_dn5 = assign101460_e153617_d_n5;
        locals.var_mud_hoso_dn6 = assign101460_e153617_d_n6;
        locals.var_mud_hoso_dn7 = assign101460_e153617_d_n7;
        locals.var_mud_hoso_dn8 = assign101460_e153617_d_n8;
        locals.var_mud_hoso_dn9 = assign101460_e153617_d_n9;
        locals.var_mud_hoso_dn10 = assign101460_e153617_d_n10;
        locals.var_mud_hoso_dn13 = assign101460_e153617_d_n13;

        let (assign101470_e153625, assign101470_e153625_d_n0, assign101470_e153625_d_n2, assign101470_e153625_d_n4, assign101470_e153625_d_n5, assign101470_e153625_d_n6, assign101470_e153625_d_n7, assign101470_e153625_d_n8, assign101470_e153625_d_n9, assign101470_e153625_d_n10, assign101470_e153625_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101470_e153621: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101470_e153623: f64 = (assign101470_e153621 / 2.0);
        (assign101470_e153623, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn13 + locals.var_mud_hoso_dn13) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn13,)
    }
};
        locals.var_mu_ave = assign101470_e153625;
        locals.var_mu_ave_dn0 = assign101470_e153625_d_n0;
        locals.var_mu_ave_dn2 = assign101470_e153625_d_n2;
        locals.var_mu_ave_dn4 = assign101470_e153625_d_n4;
        locals.var_mu_ave_dn5 = assign101470_e153625_d_n5;
        locals.var_mu_ave_dn6 = assign101470_e153625_d_n6;
        locals.var_mu_ave_dn7 = assign101470_e153625_d_n7;
        locals.var_mu_ave_dn8 = assign101470_e153625_d_n8;
        locals.var_mu_ave_dn9 = assign101470_e153625_d_n9;
        locals.var_mu_ave_dn10 = assign101470_e153625_d_n10;
        locals.var_mu_ave_dn13 = assign101470_e153625_d_n13;

        let (assign101480_e153631, assign101480_e153631_d_n0, assign101480_e153631_d_n2, assign101480_e153631_d_n4, assign101480_e153631_d_n5, assign101480_e153631_d_n6, assign101480_e153631_d_n7, assign101480_e153631_d_n8, assign101480_e153631_d_n9, assign101480_e153631_d_n10, assign101480_e153631_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101480_e153629: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101480_e153629, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn13 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101480_e153631;
        locals.var_t0_dn0 = assign101480_e153631_d_n0;
        locals.var_t0_dn2 = assign101480_e153631_d_n2;
        locals.var_t0_dn4 = assign101480_e153631_d_n4;
        locals.var_t0_dn5 = assign101480_e153631_d_n5;
        locals.var_t0_dn6 = assign101480_e153631_d_n6;
        locals.var_t0_dn7 = assign101480_e153631_d_n7;
        locals.var_t0_dn8 = assign101480_e153631_d_n8;
        locals.var_t0_dn9 = assign101480_e153631_d_n9;
        locals.var_t0_dn10 = assign101480_e153631_d_n10;
        locals.var_t0_dn13 = assign101480_e153631_d_n13;

        let (assign101490_e153693, assign101490_e153693_d_n0, assign101490_e153693_d_n2, assign101490_e153693_d_n4, assign101490_e153693_d_n5, assign101490_e153693_d_n6, assign101490_e153693_d_n7, assign101490_e153693_d_n8, assign101490_e153693_d_n9, assign101490_e153693_d_n10, assign101490_e153693_d_n13,) = {
    if (locals.var_guard2325 != 0.0) {
        let assign101490_e153635: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101490_e153637: f64 = (assign101490_e153635 * locals.var_vgvt);
        let assign101490_e153639: f64 = (assign101490_e153637 * locals.var_mu);
        let assign101490_e153643: f64 = (3.0 * locals.var_alpha);
        let assign101490_e153644: f64 = (1.0 + assign101490_e153643);
        let assign101490_e153647: f64 = (6.0 * locals.var_t0);
        let assign101490_e153648: f64 = (assign101490_e153644 + assign101490_e153647);
        let assign101490_e153650: f64 = (assign101490_e153648 * locals.var_mud_hoso);
        let assign101490_e153652: f64 = (assign101490_e153650 * locals.var_mud_hoso);
        let assign101490_e153656: f64 = (4.0 * locals.var_alpha);
        let assign101490_e153657: f64 = (3.0 + assign101490_e153656);
        let assign101490_e153660: f64 = (3.0 * locals.var_t0);
        let assign101490_e153661: f64 = (assign101490_e153657 + assign101490_e153660);
        let assign101490_e153663: f64 = (assign101490_e153661 * locals.var_mud_hoso);
        let assign101490_e153665: f64 = (assign101490_e153663 * locals.var_mu);
        let assign101490_e153666: f64 = (assign101490_e153652 + assign101490_e153665);
        let assign101490_e153670: f64 = (3.0 * locals.var_alpha);
        let assign101490_e153671: f64 = (6.0 + assign101490_e153670);
        let assign101490_e153673: f64 = (assign101490_e153671 + locals.var_t0);
        let assign101490_e153675: f64 = (assign101490_e153673 * locals.var_mu);
        let assign101490_e153677: f64 = (assign101490_e153675 * locals.var_mu);
        let assign101490_e153678: f64 = (assign101490_e153666 + assign101490_e153677);
        let assign101490_e153679: f64 = (assign101490_e153639 * assign101490_e153678);
        let assign101490_e153682: f64 = (15.0 * locals.var_lch);
        let assign101490_e153685: f64 = (1.0 + locals.var_alpha);
        let assign101490_e153686: f64 = (assign101490_e153682 * assign101490_e153685);
        let assign101490_e153688: f64 = (assign101490_e153686 * locals.var_mu_ave);
        let assign101490_e153690: f64 = (assign101490_e153688 * locals.var_mu_ave);
        let assign101490_e153691: f64 = (assign101490_e153679 / assign101490_e153690);
        (assign101490_e153691, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn0)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn0)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn0))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn0) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn0)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn2)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn2)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn2))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn2) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn2)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn4)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn4)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn4))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn4) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn4)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn5)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn5)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn5))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn5) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn5)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn6)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn6)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn6))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn6) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn6)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn7)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn7)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn7))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn7) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn7)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn8)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn8)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn8))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn8) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn8)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn9)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn9)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn9))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn9) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn9)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn10)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn10)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn10))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn10) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn10)))) / (assign101490_e153690 * assign101490_e153690)), ((((((((((locals.var_weff_nf * locals.var_cox_dn13) * locals.var_vgvt) + (assign101490_e153635 * locals.var_vgvt_dn13)) * locals.var_mu) + (assign101490_e153637 * locals.var_mu_dn13)) * assign101490_e153678) + (assign101490_e153639 * ((((((((3.0 * locals.var_alpha_dn13) + (6.0 * locals.var_t0_dn13)) * locals.var_mud_hoso) + (assign101490_e153648 * locals.var_mud_hoso_dn13)) * locals.var_mud_hoso) + (assign101490_e153650 * locals.var_mud_hoso_dn13)) + ((((((4.0 * locals.var_alpha_dn13) + (3.0 * locals.var_t0_dn13)) * locals.var_mud_hoso) + (assign101490_e153661 * locals.var_mud_hoso_dn13)) * locals.var_mu) + (assign101490_e153663 * locals.var_mu_dn13))) + ((((((3.0 * locals.var_alpha_dn13) + locals.var_t0_dn13) * locals.var_mu) + (assign101490_e153673 * locals.var_mu_dn13)) * locals.var_mu) + (assign101490_e153675 * locals.var_mu_dn13))))) * assign101490_e153690) - (assign101490_e153679 * (((((((15.0 * locals.var_lch_dn13) * assign101490_e153685) + (assign101490_e153682 * locals.var_alpha_dn13)) * locals.var_mu_ave) + (assign101490_e153686 * locals.var_mu_ave_dn13)) * locals.var_mu_ave) + (assign101490_e153688 * locals.var_mu_ave_dn13)))) / (assign101490_e153690 * assign101490_e153690)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn13,)
    }
};
        locals.var_nthrml = assign101490_e153693;
        locals.var_nthrml_dn0 = assign101490_e153693_d_n0;
        locals.var_nthrml_dn2 = assign101490_e153693_d_n2;
        locals.var_nthrml_dn4 = assign101490_e153693_d_n4;
        locals.var_nthrml_dn5 = assign101490_e153693_d_n5;
        locals.var_nthrml_dn6 = assign101490_e153693_d_n6;
        locals.var_nthrml_dn7 = assign101490_e153693_d_n7;
        locals.var_nthrml_dn8 = assign101490_e153693_d_n8;
        locals.var_nthrml_dn9 = assign101490_e153693_d_n9;
        locals.var_nthrml_dn10 = assign101490_e153693_d_n10;
        locals.var_nthrml_dn13 = assign101490_e153693_d_n13;

        let (assign101500_e153698, assign101500_e153698_d_n0, assign101500_e153698_d_n2, assign101500_e153698_d_n4, assign101500_e153698_d_n5, assign101500_e153698_d_n6, assign101500_e153698_d_n7, assign101500_e153698_d_n8, assign101500_e153698_d_n9, assign101500_e153698_d_n10, assign101500_e153698_d_n13,) = {
    if (locals.var_guard2325 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn13,)
    }
};
        locals.var_nthrml = assign101500_e153698;
        locals.var_nthrml_dn0 = assign101500_e153698_d_n0;
        locals.var_nthrml_dn2 = assign101500_e153698_d_n2;
        locals.var_nthrml_dn4 = assign101500_e153698_d_n4;
        locals.var_nthrml_dn5 = assign101500_e153698_d_n5;
        locals.var_nthrml_dn6 = assign101500_e153698_d_n6;
        locals.var_nthrml_dn7 = assign101500_e153698_d_n7;
        locals.var_nthrml_dn8 = assign101500_e153698_d_n8;
        locals.var_nthrml_dn9 = assign101500_e153698_d_n9;
        locals.var_nthrml_dn10 = assign101500_e153698_d_n10;
        locals.var_nthrml_dn13 = assign101500_e153698_d_n13;

        let assign101510_e153716: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101510_e153716;

        let (assign101520_e153721, assign101520_e153721_d_n0, assign101520_e153721_d_n2, assign101520_e153721_d_n4, assign101520_e153721_d_n5, assign101520_e153721_d_n6, assign101520_e153721_d_n7, assign101520_e153721_d_n8, assign101520_e153721_d_n9, assign101520_e153721_d_n10, assign101520_e153721_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101520_e153719: f64 = (locals.var_kusail).sqrt();
        (assign101520_e153719, (locals.var_kusail_dn0 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn2 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn4 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn5 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn6 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn7 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn8 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn9 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn10 / (2.0 * assign101520_e153719)), (locals.var_kusail_dn13 / (2.0 * assign101520_e153719)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn13,)
    }
};
        locals.var_sqrtkusail = assign101520_e153721;
        locals.var_sqrtkusail_dn0 = assign101520_e153721_d_n0;
        locals.var_sqrtkusail_dn2 = assign101520_e153721_d_n2;
        locals.var_sqrtkusail_dn4 = assign101520_e153721_d_n4;
        locals.var_sqrtkusail_dn5 = assign101520_e153721_d_n5;
        locals.var_sqrtkusail_dn6 = assign101520_e153721_d_n6;
        locals.var_sqrtkusail_dn7 = assign101520_e153721_d_n7;
        locals.var_sqrtkusail_dn8 = assign101520_e153721_d_n8;
        locals.var_sqrtkusail_dn9 = assign101520_e153721_d_n9;
        locals.var_sqrtkusail_dn10 = assign101520_e153721_d_n10;
        locals.var_sqrtkusail_dn13 = assign101520_e153721_d_n13;

        let (assign101530_e153727, assign101530_e153727_d_n0, assign101530_e153727_d_n2, assign101530_e153727_d_n4, assign101530_e153727_d_n5, assign101530_e153727_d_n6, assign101530_e153727_d_n7, assign101530_e153727_d_n8, assign101530_e153727_d_n9, assign101530_e153727_d_n10, assign101530_e153727_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101530_e153725: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101530_e153725, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn13 + locals.var_sqrtkusail_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign101530_e153727;
        locals.var_t2_dn0 = assign101530_e153727_d_n0;
        locals.var_t2_dn2 = assign101530_e153727_d_n2;
        locals.var_t2_dn4 = assign101530_e153727_d_n4;
        locals.var_t2_dn5 = assign101530_e153727_d_n5;
        locals.var_t2_dn6 = assign101530_e153727_d_n6;
        locals.var_t2_dn7 = assign101530_e153727_d_n7;
        locals.var_t2_dn8 = assign101530_e153727_d_n8;
        locals.var_t2_dn9 = assign101530_e153727_d_n9;
        locals.var_t2_dn10 = assign101530_e153727_d_n10;
        locals.var_t2_dn13 = assign101530_e153727_d_n13;

        let (assign101540_e153733, assign101540_e153733_d_n0, assign101540_e153733_d_n2, assign101540_e153733_d_n4, assign101540_e153733_d_n5, assign101540_e153733_d_n6, assign101540_e153733_d_n7, assign101540_e153733_d_n8, assign101540_e153733_d_n9, assign101540_e153733_d_n10, assign101540_e153733_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101540_e153731: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101540_e153731, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn13 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign101540_e153733;
        locals.var_t3_dn0 = assign101540_e153733_d_n0;
        locals.var_t3_dn2 = assign101540_e153733_d_n2;
        locals.var_t3_dn4 = assign101540_e153733_d_n4;
        locals.var_t3_dn5 = assign101540_e153733_d_n5;
        locals.var_t3_dn6 = assign101540_e153733_d_n6;
        locals.var_t3_dn7 = assign101540_e153733_d_n7;
        locals.var_t3_dn8 = assign101540_e153733_d_n8;
        locals.var_t3_dn9 = assign101540_e153733_d_n9;
        locals.var_t3_dn10 = assign101540_e153733_d_n10;
        locals.var_t3_dn13 = assign101540_e153733_d_n13;

        let (assign101550_e153739, assign101550_e153739_d_n0, assign101550_e153739_d_n2, assign101550_e153739_d_n4, assign101550_e153739_d_n5, assign101550_e153739_d_n6, assign101550_e153739_d_n7, assign101550_e153739_d_n8, assign101550_e153739_d_n9, assign101550_e153739_d_n10, assign101550_e153739_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101550_e153737: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101550_e153737, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn13 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign101550_e153739;
        locals.var_t4_dn0 = assign101550_e153739_d_n0;
        locals.var_t4_dn2 = assign101550_e153739_d_n2;
        locals.var_t4_dn4 = assign101550_e153739_d_n4;
        locals.var_t4_dn5 = assign101550_e153739_d_n5;
        locals.var_t4_dn6 = assign101550_e153739_d_n6;
        locals.var_t4_dn7 = assign101550_e153739_d_n7;
        locals.var_t4_dn8 = assign101550_e153739_d_n8;
        locals.var_t4_dn9 = assign101550_e153739_d_n9;
        locals.var_t4_dn10 = assign101550_e153739_d_n10;
        locals.var_t4_dn13 = assign101550_e153739_d_n13;

        let (assign101560_e153747, assign101560_e153747_d_n0, assign101560_e153747_d_n2, assign101560_e153747_d_n4, assign101560_e153747_d_n5, assign101560_e153747_d_n6, assign101560_e153747_d_n7, assign101560_e153747_d_n8, assign101560_e153747_d_n9, assign101560_e153747_d_n10, assign101560_e153747_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101560_e153743: f64 = (42.0 * locals.var_kusai00);
        let assign101560_e153745: f64 = (assign101560_e153743 * locals.var_kusail);
        (assign101560_e153745, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn13) * locals.var_kusail) + (assign101560_e153743 * locals.var_kusail_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101560_e153747;
        locals.var_t5_dn0 = assign101560_e153747_d_n0;
        locals.var_t5_dn2 = assign101560_e153747_d_n2;
        locals.var_t5_dn4 = assign101560_e153747_d_n4;
        locals.var_t5_dn5 = assign101560_e153747_d_n5;
        locals.var_t5_dn6 = assign101560_e153747_d_n6;
        locals.var_t5_dn7 = assign101560_e153747_d_n7;
        locals.var_t5_dn8 = assign101560_e153747_d_n8;
        locals.var_t5_dn9 = assign101560_e153747_d_n9;
        locals.var_t5_dn10 = assign101560_e153747_d_n10;
        locals.var_t5_dn13 = assign101560_e153747_d_n13;

    }

    pub(super) fn stamp_transient_block_360(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101570_e153757, assign101570_e153757_d_n0, assign101570_e153757_d_n2, assign101570_e153757_d_n4, assign101570_e153757_d_n5, assign101570_e153757_d_n6, assign101570_e153757_d_n7, assign101570_e153757_d_n8, assign101570_e153757_d_n9, assign101570_e153757_d_n10, assign101570_e153757_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101570_e153753: f64 = (locals.var_t3 + locals.var_t4);
        let assign101570_e153754: f64 = (4.0 * assign101570_e153753);
        let assign101570_e153755: f64 = (locals.var_t5 + assign101570_e153754);
        (assign101570_e153755, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn13 + (4.0 * (locals.var_t3_dn13 + locals.var_t4_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101570_e153757;
        locals.var_t5_dn0 = assign101570_e153757_d_n0;
        locals.var_t5_dn2 = assign101570_e153757_d_n2;
        locals.var_t5_dn4 = assign101570_e153757_d_n4;
        locals.var_t5_dn5 = assign101570_e153757_d_n5;
        locals.var_t5_dn6 = assign101570_e153757_d_n6;
        locals.var_t5_dn7 = assign101570_e153757_d_n7;
        locals.var_t5_dn8 = assign101570_e153757_d_n8;
        locals.var_t5_dn9 = assign101570_e153757_d_n9;
        locals.var_t5_dn10 = assign101570_e153757_d_n10;
        locals.var_t5_dn13 = assign101570_e153757_d_n13;

        let (assign101580_e153771, assign101580_e153771_d_n0, assign101580_e153771_d_n2, assign101580_e153771_d_n4, assign101580_e153771_d_n5, assign101580_e153771_d_n6, assign101580_e153771_d_n7, assign101580_e153771_d_n8, assign101580_e153771_d_n9, assign101580_e153771_d_n10, assign101580_e153771_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101580_e153762: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101580_e153764: f64 = (assign101580_e153762 * locals.var_vgvt);
        let assign101580_e153767: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101580_e153768: f64 = (assign101580_e153764 * assign101580_e153767);
        let assign101580_e153769: f64 = (locals.var_t5 + assign101580_e153768);
        (assign101580_e153769, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn0)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn2)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn4)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn5)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn6)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn7)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn8)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn9)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn10)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn13 + (((((20.0 * locals.var_sqrtkusail_dn13) * locals.var_vgvt) + (assign101580_e153762 * locals.var_vgvt_dn13)) * assign101580_e153767) + (assign101580_e153764 * (locals.var_kusai00_dn13 + locals.var_kusail_dn13)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign101580_e153771;
        locals.var_t5_dn0 = assign101580_e153771_d_n0;
        locals.var_t5_dn2 = assign101580_e153771_d_n2;
        locals.var_t5_dn4 = assign101580_e153771_d_n4;
        locals.var_t5_dn5 = assign101580_e153771_d_n5;
        locals.var_t5_dn6 = assign101580_e153771_d_n6;
        locals.var_t5_dn7 = assign101580_e153771_d_n7;
        locals.var_t5_dn8 = assign101580_e153771_d_n8;
        locals.var_t5_dn9 = assign101580_e153771_d_n9;
        locals.var_t5_dn10 = assign101580_e153771_d_n10;
        locals.var_t5_dn13 = assign101580_e153771_d_n13;

        let (assign101590_e153777, assign101590_e153777_d_n0, assign101590_e153777_d_n2, assign101590_e153777_d_n4, assign101590_e153777_d_n5, assign101590_e153777_d_n6, assign101590_e153777_d_n7, assign101590_e153777_d_n8, assign101590_e153777_d_n9, assign101590_e153777_d_n10, assign101590_e153777_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101590_e153775: f64 = (locals.var_t2 * locals.var_t2);
        (assign101590_e153775, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101590_e153777;
        locals.var_t10_dn0 = assign101590_e153777_d_n0;
        locals.var_t10_dn2 = assign101590_e153777_d_n2;
        locals.var_t10_dn4 = assign101590_e153777_d_n4;
        locals.var_t10_dn5 = assign101590_e153777_d_n5;
        locals.var_t10_dn6 = assign101590_e153777_d_n6;
        locals.var_t10_dn7 = assign101590_e153777_d_n7;
        locals.var_t10_dn8 = assign101590_e153777_d_n8;
        locals.var_t10_dn9 = assign101590_e153777_d_n9;
        locals.var_t10_dn10 = assign101590_e153777_d_n10;
        locals.var_t10_dn13 = assign101590_e153777_d_n13;

        let (assign101600_e153783, assign101600_e153783_d_n0, assign101600_e153783_d_n2, assign101600_e153783_d_n4, assign101600_e153783_d_n5, assign101600_e153783_d_n6, assign101600_e153783_d_n7, assign101600_e153783_d_n8, assign101600_e153783_d_n9, assign101600_e153783_d_n10, assign101600_e153783_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101600_e153781: f64 = (locals.var_t10 * locals.var_t10);
        (assign101600_e153781, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign101600_e153783;
        locals.var_t10_dn0 = assign101600_e153783_d_n0;
        locals.var_t10_dn2 = assign101600_e153783_d_n2;
        locals.var_t10_dn4 = assign101600_e153783_d_n4;
        locals.var_t10_dn5 = assign101600_e153783_d_n5;
        locals.var_t10_dn6 = assign101600_e153783_d_n6;
        locals.var_t10_dn7 = assign101600_e153783_d_n7;
        locals.var_t10_dn8 = assign101600_e153783_d_n8;
        locals.var_t10_dn9 = assign101600_e153783_d_n9;
        locals.var_t10_dn10 = assign101600_e153783_d_n10;
        locals.var_t10_dn13 = assign101600_e153783_d_n13;

        let (assign101610_e153791, assign101610_e153791_d_n0, assign101610_e153791_d_n2, assign101610_e153791_d_n4, assign101610_e153791_d_n5, assign101610_e153791_d_n6, assign101610_e153791_d_n7, assign101610_e153791_d_n8, assign101610_e153791_d_n9, assign101610_e153791_d_n10, assign101610_e153791_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101610_e153788: f64 = (locals.var_t10 * locals.var_t2);
        let assign101610_e153789: f64 = (locals.var_t5 / assign101610_e153788);
        (assign101610_e153789, (((locals.var_t5_dn0 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn2 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn4 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn5 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn6 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn7 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn8 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn9 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn10 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101610_e153788 * assign101610_e153788)), (((locals.var_t5_dn13 * assign101610_e153788) - (locals.var_t5 * ((locals.var_t10_dn13 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn13)))) / (assign101610_e153788 * assign101610_e153788)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn13,)
    }
};
        locals.var_kusai_ig = assign101610_e153791;
        locals.var_kusai_ig_dn0 = assign101610_e153791_d_n0;
        locals.var_kusai_ig_dn2 = assign101610_e153791_d_n2;
        locals.var_kusai_ig_dn4 = assign101610_e153791_d_n4;
        locals.var_kusai_ig_dn5 = assign101610_e153791_d_n5;
        locals.var_kusai_ig_dn6 = assign101610_e153791_d_n6;
        locals.var_kusai_ig_dn7 = assign101610_e153791_d_n7;
        locals.var_kusai_ig_dn8 = assign101610_e153791_d_n8;
        locals.var_kusai_ig_dn9 = assign101610_e153791_d_n9;
        locals.var_kusai_ig_dn10 = assign101610_e153791_d_n10;
        locals.var_kusai_ig_dn13 = assign101610_e153791_d_n13;

        let (assign101620_e153801, assign101620_e153801_d_n0, assign101620_e153801_d_n2, assign101620_e153801_d_n4, assign101620_e153801_d_n5, assign101620_e153801_d_n6, assign101620_e153801_d_n7, assign101620_e153801_d_n8, assign101620_e153801_d_n9, assign101620_e153801_d_n10, assign101620_e153801_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101620_e153795: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101620_e153797: f64 = (assign101620_e153795 * locals.var_mu);
        let assign101620_e153799: f64 = (assign101620_e153797 * locals.var_cox);
        (assign101620_e153799, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn0)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn2)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn4)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn5)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn6)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn7)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn8)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn9)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn10)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn13) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101620_e153795 * locals.var_mu_dn13)) * locals.var_cox) + (assign101620_e153797 * locals.var_cox_dn13)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn13,)
    }
};
        locals.var_gds0_ign = assign101620_e153801;
        locals.var_gds0_ign_dn0 = assign101620_e153801_d_n0;
        locals.var_gds0_ign_dn2 = assign101620_e153801_d_n2;
        locals.var_gds0_ign_dn4 = assign101620_e153801_d_n4;
        locals.var_gds0_ign_dn5 = assign101620_e153801_d_n5;
        locals.var_gds0_ign_dn6 = assign101620_e153801_d_n6;
        locals.var_gds0_ign_dn7 = assign101620_e153801_d_n7;
        locals.var_gds0_ign_dn8 = assign101620_e153801_d_n8;
        locals.var_gds0_ign_dn9 = assign101620_e153801_d_n9;
        locals.var_gds0_ign_dn10 = assign101620_e153801_d_n10;
        locals.var_gds0_ign_dn13 = assign101620_e153801_d_n13;

        let (assign101630_e153807, assign101630_e153807_d_n0, assign101630_e153807_d_n2, assign101630_e153807_d_n4, assign101630_e153807_d_n5, assign101630_e153807_d_n6, assign101630_e153807_d_n7, assign101630_e153807_d_n8, assign101630_e153807_d_n9, assign101630_e153807_d_n10, assign101630_e153807_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101630_e153805: f64 = (locals.var_gds0_ign * locals.var_vgvt);
        (assign101630_e153805, ((locals.var_gds0_ign_dn0 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn0)), ((locals.var_gds0_ign_dn2 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn2)), ((locals.var_gds0_ign_dn4 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn4)), ((locals.var_gds0_ign_dn5 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn5)), ((locals.var_gds0_ign_dn6 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn6)), ((locals.var_gds0_ign_dn7 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn7)), ((locals.var_gds0_ign_dn8 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn8)), ((locals.var_gds0_ign_dn9 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn9)), ((locals.var_gds0_ign_dn10 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn10)), ((locals.var_gds0_ign_dn13 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn13)),)
    } else {
        (locals.var_gds0_h2, locals.var_gds0_h2_dn0, locals.var_gds0_h2_dn2, locals.var_gds0_h2_dn4, locals.var_gds0_h2_dn5, locals.var_gds0_h2_dn6, locals.var_gds0_h2_dn7, locals.var_gds0_h2_dn8, locals.var_gds0_h2_dn9, locals.var_gds0_h2_dn10, locals.var_gds0_h2_dn13,)
    }
};
        locals.var_gds0_h2 = assign101630_e153807;
        locals.var_gds0_h2_dn0 = assign101630_e153807_d_n0;
        locals.var_gds0_h2_dn2 = assign101630_e153807_d_n2;
        locals.var_gds0_h2_dn4 = assign101630_e153807_d_n4;
        locals.var_gds0_h2_dn5 = assign101630_e153807_d_n5;
        locals.var_gds0_h2_dn6 = assign101630_e153807_d_n6;
        locals.var_gds0_h2_dn7 = assign101630_e153807_d_n7;
        locals.var_gds0_h2_dn8 = assign101630_e153807_d_n8;
        locals.var_gds0_h2_dn9 = assign101630_e153807_d_n9;
        locals.var_gds0_h2_dn10 = assign101630_e153807_d_n10;
        locals.var_gds0_h2_dn13 = assign101630_e153807_d_n13;

        let (assign101640_e153813, assign101640_e153813_d_n0, assign101640_e153813_d_n2, assign101640_e153813_d_n4, assign101640_e153813_d_n5, assign101640_e153813_d_n6, assign101640_e153813_d_n7, assign101640_e153813_d_n8, assign101640_e153813_d_n9, assign101640_e153813_d_n10, assign101640_e153813_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101640_e153811: f64 = (locals.var_nthrml / locals.var_gds0_h2);
        (assign101640_e153811, (((locals.var_nthrml_dn0 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn0)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn2 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn2)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn4 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn4)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn5 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn5)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn6 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn6)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn7 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn7)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn8 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn8)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn9 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn9)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn10 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn10)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn13 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn13)) / (locals.var_gds0_h2 * locals.var_gds0_h2)),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn13,)
    }
};
        locals.var_gamma = assign101640_e153813;
        locals.var_gamma_dn0 = assign101640_e153813_d_n0;
        locals.var_gamma_dn2 = assign101640_e153813_d_n2;
        locals.var_gamma_dn4 = assign101640_e153813_d_n4;
        locals.var_gamma_dn5 = assign101640_e153813_d_n5;
        locals.var_gamma_dn6 = assign101640_e153813_d_n6;
        locals.var_gamma_dn7 = assign101640_e153813_d_n7;
        locals.var_gamma_dn8 = assign101640_e153813_d_n8;
        locals.var_gamma_dn9 = assign101640_e153813_d_n9;
        locals.var_gamma_dn10 = assign101640_e153813_d_n10;
        locals.var_gamma_dn13 = assign101640_e153813_d_n13;

        let (assign101650_e153825, assign101650_e153825_d_n0, assign101650_e153825_d_n2, assign101650_e153825_d_n4, assign101650_e153825_d_n5, assign101650_e153825_d_n6, assign101650_e153825_d_n7, assign101650_e153825_d_n8, assign101650_e153825_d_n9, assign101650_e153825_d_n10, assign101650_e153825_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101650_e153818: f64 = (4.0 * locals.var_vgvt);
        let assign101650_e153820: f64 = (assign101650_e153818 * locals.var_sqrtkusail);
        let assign101650_e153821: f64 = (locals.var_kusai00 + assign101650_e153820);
        let assign101650_e153823: f64 = (assign101650_e153821 + locals.var_kusail);
        (assign101650_e153823, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn13 + (((4.0 * locals.var_vgvt_dn13) * locals.var_sqrtkusail) + (assign101650_e153818 * locals.var_sqrtkusail_dn13))) + locals.var_kusail_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign101650_e153825;
        locals.var_t7_dn0 = assign101650_e153825_d_n0;
        locals.var_t7_dn2 = assign101650_e153825_d_n2;
        locals.var_t7_dn4 = assign101650_e153825_d_n4;
        locals.var_t7_dn5 = assign101650_e153825_d_n5;
        locals.var_t7_dn6 = assign101650_e153825_d_n6;
        locals.var_t7_dn7 = assign101650_e153825_d_n7;
        locals.var_t7_dn8 = assign101650_e153825_d_n8;
        locals.var_t7_dn9 = assign101650_e153825_d_n9;
        locals.var_t7_dn10 = assign101650_e153825_d_n10;
        locals.var_t7_dn13 = assign101650_e153825_d_n13;

        let (assign101660_e153846, assign101660_e153846_d_n0, assign101660_e153846_d_n2, assign101660_e153846_d_n4, assign101660_e153846_d_n5, assign101660_e153846_d_n6, assign101660_e153846_d_n7, assign101660_e153846_d_n8, assign101660_e153846_d_n9, assign101660_e153846_d_n10, assign101660_e153846_d_n13,) = {
    if (locals.var_guard2328 != 0.0) {
        let assign101660_e153829: f64 = (3.872983346207417 * locals.var_kusai00l);
        let assign101660_e153831: f64 = (assign101660_e153829 * locals.var_t7);
        let assign101660_e153834: f64 = (6.0 * locals.var_t2);
        let assign101660_e153837: f64 = (locals.var_gamma * locals.var_t2);
        let assign101660_e153839: f64 = (assign101660_e153837 * locals.var_vgvt);
        let assign101660_e153841: f64 = (assign101660_e153839 * locals.var_t5);
        let assign101660_e153842: f64 = (assign101660_e153841).sqrt();
        let assign101660_e153843: f64 = (assign101660_e153834 * assign101660_e153842);
        let assign101660_e153844: f64 = (assign101660_e153831 / assign101660_e153843);
        (assign101660_e153844, ((((((3.872983346207417 * locals.var_kusai00l_dn0) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn0)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn0) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn0 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn0)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn0)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn0)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn2) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn2)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn2) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn2 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn2)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn2)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn2)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn4) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn4)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn4) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn4 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn4)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn4)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn4)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn5) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn5)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn5) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn5 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn5)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn5)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn5)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn6) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn6)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn6) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn6 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn6)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn6)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn6)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn7) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn7)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn7) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn7 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn7)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn7)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn7)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn8) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn8)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn8) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn8 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn8)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn8)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn8)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn9) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn9)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn9) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn9 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn9)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn9)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn9)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn10) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn10)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn10) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn10 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn10)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn10)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn10)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)), ((((((3.872983346207417 * locals.var_kusai00l_dn13) * locals.var_t7) + (assign101660_e153829 * locals.var_t7_dn13)) * assign101660_e153843) - (assign101660_e153831 * (((6.0 * locals.var_t2_dn13) * assign101660_e153842) + (assign101660_e153834 * (((((((locals.var_gamma_dn13 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn13)) * locals.var_vgvt) + (assign101660_e153837 * locals.var_vgvt_dn13)) * locals.var_t5) + (assign101660_e153839 * locals.var_t5_dn13)) / (2.0 * assign101660_e153842)))))) / (assign101660_e153843 * assign101660_e153843)),)
    } else {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn13,)
    }
};
        locals.var_crl_f = assign101660_e153846;
        locals.var_crl_f_dn0 = assign101660_e153846_d_n0;
        locals.var_crl_f_dn2 = assign101660_e153846_d_n2;
        locals.var_crl_f_dn4 = assign101660_e153846_d_n4;
        locals.var_crl_f_dn5 = assign101660_e153846_d_n5;
        locals.var_crl_f_dn6 = assign101660_e153846_d_n6;
        locals.var_crl_f_dn7 = assign101660_e153846_d_n7;
        locals.var_crl_f_dn8 = assign101660_e153846_d_n8;
        locals.var_crl_f_dn9 = assign101660_e153846_d_n9;
        locals.var_crl_f_dn10 = assign101660_e153846_d_n10;
        locals.var_crl_f_dn13 = assign101660_e153846_d_n13;

        let assign101670_e153849: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101670_e153849;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn9 = (locals.var_mfactor * locals.var_ids_dn9);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn13 = (locals.var_mfactor * locals.var_ids_dn13);

        let assign101710_e153861: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101710_e153861;
        locals.var_idsibpce_dn0 = (locals.var_mfactor * locals.var_idsibpc_dn0);
        locals.var_idsibpce_dn2 = (locals.var_mfactor * locals.var_idsibpc_dn2);
        locals.var_idsibpce_dn4 = (locals.var_mfactor * locals.var_idsibpc_dn4);
        locals.var_idsibpce_dn5 = (locals.var_mfactor * locals.var_idsibpc_dn5);
        locals.var_idsibpce_dn6 = (locals.var_mfactor * locals.var_idsibpc_dn6);
        locals.var_idsibpce_dn7 = (locals.var_mfactor * locals.var_idsibpc_dn7);
        locals.var_idsibpce_dn8 = (locals.var_mfactor * locals.var_idsibpc_dn8);
        locals.var_idsibpce_dn9 = (locals.var_mfactor * locals.var_idsibpc_dn9);
        locals.var_idsibpce_dn10 = (locals.var_mfactor * locals.var_idsibpc_dn10);
        locals.var_idsibpce_dn13 = (locals.var_mfactor * locals.var_idsibpc_dn13);

        locals.var_ibjte = locals.var_wibjt;
        locals.var_ibjte_dn0 = locals.var_wibjt_dn0;
        locals.var_ibjte_dn2 = locals.var_wibjt_dn2;
        locals.var_ibjte_dn4 = locals.var_wibjt_dn4;
        locals.var_ibjte_dn5 = locals.var_wibjt_dn5;
        locals.var_ibjte_dn6 = locals.var_wibjt_dn6;
        locals.var_ibjte_dn7 = locals.var_wibjt_dn7;
        locals.var_ibjte_dn8 = locals.var_wibjt_dn8;
        locals.var_ibjte_dn9 = locals.var_wibjt_dn9;
        locals.var_ibjte_dn10 = locals.var_wibjt_dn10;
        locals.var_ibjte_dn13 = locals.var_wibjt_dn13;

        locals.var_qgexte = 0.0;
        locals.var_qgexte_dn0 = 0.0;
        locals.var_qgexte_dn2 = 0.0;
        locals.var_qgexte_dn4 = 0.0;
        locals.var_qgexte_dn5 = 0.0;
        locals.var_qgexte_dn6 = 0.0;
        locals.var_qgexte_dn7 = 0.0;
        locals.var_qgexte_dn8 = 0.0;
        locals.var_qgexte_dn9 = 0.0;
        locals.var_qgexte_dn10 = 0.0;
        locals.var_qgexte_dn13 = 0.0;

        locals.var_qdexte = 0.0;
        locals.var_qdexte_dn0 = 0.0;
        locals.var_qdexte_dn2 = 0.0;
        locals.var_qdexte_dn4 = 0.0;
        locals.var_qdexte_dn5 = 0.0;
        locals.var_qdexte_dn6 = 0.0;
        locals.var_qdexte_dn7 = 0.0;
        locals.var_qdexte_dn8 = 0.0;
        locals.var_qdexte_dn9 = 0.0;
        locals.var_qdexte_dn10 = 0.0;
        locals.var_qdexte_dn13 = 0.0;

        locals.var_qsexte = 0.0;
        locals.var_qsexte_dn0 = 0.0;
        locals.var_qsexte_dn2 = 0.0;
        locals.var_qsexte_dn4 = 0.0;
        locals.var_qsexte_dn5 = 0.0;
        locals.var_qsexte_dn6 = 0.0;
        locals.var_qsexte_dn7 = 0.0;
        locals.var_qsexte_dn8 = 0.0;
        locals.var_qsexte_dn9 = 0.0;
        locals.var_qsexte_dn10 = 0.0;
        locals.var_qsexte_dn13 = 0.0;

        locals.var_qgov = 0.0;
        locals.var_qgov_dn0 = 0.0;
        locals.var_qgov_dn2 = 0.0;
        locals.var_qgov_dn4 = 0.0;
        locals.var_qgov_dn5 = 0.0;
        locals.var_qgov_dn6 = 0.0;
        locals.var_qgov_dn7 = 0.0;
        locals.var_qgov_dn8 = 0.0;
        locals.var_qgov_dn9 = 0.0;
        locals.var_qgov_dn10 = 0.0;
        locals.var_qgov_dn13 = 0.0;

        locals.var_qdov = 0.0;
        locals.var_qdov_dn0 = 0.0;
        locals.var_qdov_dn2 = 0.0;
        locals.var_qdov_dn4 = 0.0;
        locals.var_qdov_dn5 = 0.0;
        locals.var_qdov_dn6 = 0.0;
        locals.var_qdov_dn7 = 0.0;
        locals.var_qdov_dn8 = 0.0;
        locals.var_qdov_dn9 = 0.0;
        locals.var_qdov_dn10 = 0.0;
        locals.var_qdov_dn13 = 0.0;

        locals.var_qsov = 0.0;
        locals.var_qsov_dn0 = 0.0;
        locals.var_qsov_dn2 = 0.0;
        locals.var_qsov_dn4 = 0.0;
        locals.var_qsov_dn5 = 0.0;
        locals.var_qsov_dn6 = 0.0;
        locals.var_qsov_dn7 = 0.0;
        locals.var_qsov_dn8 = 0.0;
        locals.var_qsov_dn9 = 0.0;
        locals.var_qsov_dn10 = 0.0;
        locals.var_qsov_dn13 = 0.0;

        locals.var_qdp = 0.0;
        locals.var_qdp_dn0 = 0.0;
        locals.var_qdp_dn2 = 0.0;
        locals.var_qdp_dn6 = 0.0;

        locals.var_qsp = 0.0;
        locals.var_qsp_dn2 = 0.0;
        locals.var_qsp_dn6 = 0.0;

        let assign101810_e153875: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101810_e153875;

        let (assign101820_e153879, assign101820_e153879_d_n0, assign101820_e153879_d_n2, assign101820_e153879_d_n4, assign101820_e153879_d_n5, assign101820_e153879_d_n6, assign101820_e153879_d_n7, assign101820_e153879_d_n8, assign101820_e153879_d_n9, assign101820_e153879_d_n10, assign101820_e153879_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign101820_e153879;
        locals.var_qge_dn0 = assign101820_e153879_d_n0;
        locals.var_qge_dn2 = assign101820_e153879_d_n2;
        locals.var_qge_dn4 = assign101820_e153879_d_n4;
        locals.var_qge_dn5 = assign101820_e153879_d_n5;
        locals.var_qge_dn6 = assign101820_e153879_d_n6;
        locals.var_qge_dn7 = assign101820_e153879_d_n7;
        locals.var_qge_dn8 = assign101820_e153879_d_n8;
        locals.var_qge_dn9 = assign101820_e153879_d_n9;
        locals.var_qge_dn10 = assign101820_e153879_d_n10;
        locals.var_qge_dn13 = assign101820_e153879_d_n13;

        let (assign101830_e153883, assign101830_e153883_d_n0, assign101830_e153883_d_n2, assign101830_e153883_d_n4, assign101830_e153883_d_n5, assign101830_e153883_d_n6, assign101830_e153883_d_n7, assign101830_e153883_d_n8, assign101830_e153883_d_n9, assign101830_e153883_d_n10, assign101830_e153883_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign101830_e153883;
        locals.var_qde_dn0 = assign101830_e153883_d_n0;
        locals.var_qde_dn2 = assign101830_e153883_d_n2;
        locals.var_qde_dn4 = assign101830_e153883_d_n4;
        locals.var_qde_dn5 = assign101830_e153883_d_n5;
        locals.var_qde_dn6 = assign101830_e153883_d_n6;
        locals.var_qde_dn7 = assign101830_e153883_d_n7;
        locals.var_qde_dn8 = assign101830_e153883_d_n8;
        locals.var_qde_dn9 = assign101830_e153883_d_n9;
        locals.var_qde_dn10 = assign101830_e153883_d_n10;
        locals.var_qde_dn13 = assign101830_e153883_d_n13;

        let (assign101840_e153887, assign101840_e153887_d_n0, assign101840_e153887_d_n2, assign101840_e153887_d_n4, assign101840_e153887_d_n5, assign101840_e153887_d_n6, assign101840_e153887_d_n7, assign101840_e153887_d_n8, assign101840_e153887_d_n9, assign101840_e153887_d_n10, assign101840_e153887_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign101840_e153887;
        locals.var_qse_dn0 = assign101840_e153887_d_n0;
        locals.var_qse_dn2 = assign101840_e153887_d_n2;
        locals.var_qse_dn4 = assign101840_e153887_d_n4;
        locals.var_qse_dn5 = assign101840_e153887_d_n5;
        locals.var_qse_dn6 = assign101840_e153887_d_n6;
        locals.var_qse_dn7 = assign101840_e153887_d_n7;
        locals.var_qse_dn8 = assign101840_e153887_d_n8;
        locals.var_qse_dn9 = assign101840_e153887_d_n9;
        locals.var_qse_dn10 = assign101840_e153887_d_n10;
        locals.var_qse_dn13 = assign101840_e153887_d_n13;

        let (assign101850_e153891, assign101850_e153891_d_n0, assign101850_e153891_d_n2, assign101850_e153891_d_n4, assign101850_e153891_d_n5, assign101850_e153891_d_n6, assign101850_e153891_d_n7, assign101850_e153891_d_n8, assign101850_e153891_d_n9, assign101850_e153891_d_n10, assign101850_e153891_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn13,)
    }
};
        locals.var_xd = assign101850_e153891;
        locals.var_xd_dn0 = assign101850_e153891_d_n0;
        locals.var_xd_dn2 = assign101850_e153891_d_n2;
        locals.var_xd_dn4 = assign101850_e153891_d_n4;
        locals.var_xd_dn5 = assign101850_e153891_d_n5;
        locals.var_xd_dn6 = assign101850_e153891_d_n6;
        locals.var_xd_dn7 = assign101850_e153891_d_n7;
        locals.var_xd_dn8 = assign101850_e153891_d_n8;
        locals.var_xd_dn9 = assign101850_e153891_d_n9;
        locals.var_xd_dn10 = assign101850_e153891_d_n10;
        locals.var_xd_dn13 = assign101850_e153891_d_n13;

        let (assign101860_e153897, assign101860_e153897_d_n0, assign101860_e153897_d_n2, assign101860_e153897_d_n4, assign101860_e153897_d_n5, assign101860_e153897_d_n6, assign101860_e153897_d_n7, assign101860_e153897_d_n8, assign101860_e153897_d_n9, assign101860_e153897_d_n10, assign101860_e153897_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101860_e153895: f64 = (locals.var_mfactor * locals.var_qb);
        (assign101860_e153895, (locals.var_mfactor * locals.var_qb_dn0), (locals.var_mfactor * locals.var_qb_dn2), (locals.var_mfactor * locals.var_qb_dn4), (locals.var_mfactor * locals.var_qb_dn5), (locals.var_mfactor * locals.var_qb_dn6), (locals.var_mfactor * locals.var_qb_dn7), (locals.var_mfactor * locals.var_qb_dn8), (locals.var_mfactor * locals.var_qb_dn9), (locals.var_mfactor * locals.var_qb_dn10), (locals.var_mfactor * locals.var_qb_dn13),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn0, locals.var_qbulk_dn2, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn13,)
    }
};
        locals.var_qbulk = assign101860_e153897;
        locals.var_qbulk_dn0 = assign101860_e153897_d_n0;
        locals.var_qbulk_dn2 = assign101860_e153897_d_n2;
        locals.var_qbulk_dn4 = assign101860_e153897_d_n4;
        locals.var_qbulk_dn5 = assign101860_e153897_d_n5;
        locals.var_qbulk_dn6 = assign101860_e153897_d_n6;
        locals.var_qbulk_dn7 = assign101860_e153897_d_n7;
        locals.var_qbulk_dn8 = assign101860_e153897_d_n8;
        locals.var_qbulk_dn9 = assign101860_e153897_d_n9;
        locals.var_qbulk_dn10 = assign101860_e153897_d_n10;
        locals.var_qbulk_dn13 = assign101860_e153897_d_n13;

        let (assign101870_e153903, assign101870_e153903_d_n0, assign101870_e153903_d_n2, assign101870_e153903_d_n4, assign101870_e153903_d_n5, assign101870_e153903_d_n6, assign101870_e153903_d_n7, assign101870_e153903_d_n8, assign101870_e153903_d_n9, assign101870_e153903_d_n10, assign101870_e153903_d_n13,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101870_e153901: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101870_e153901, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn13),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn13,)
    }
};
        locals.var_qi = assign101870_e153903;
        locals.var_qi_dn0 = assign101870_e153903_d_n0;
        locals.var_qi_dn2 = assign101870_e153903_d_n2;
        locals.var_qi_dn4 = assign101870_e153903_d_n4;
        locals.var_qi_dn5 = assign101870_e153903_d_n5;
        locals.var_qi_dn6 = assign101870_e153903_d_n6;
        locals.var_qi_dn7 = assign101870_e153903_d_n7;
        locals.var_qi_dn8 = assign101870_e153903_d_n8;
        locals.var_qi_dn9 = assign101870_e153903_d_n9;
        locals.var_qi_dn10 = assign101870_e153903_d_n10;
        locals.var_qi_dn13 = assign101870_e153903_d_n13;

        let (assign101880_e153913, assign101880_e153913_d_n0, assign101880_e153913_d_n2, assign101880_e153913_d_n4, assign101880_e153913_d_n5, assign101880_e153913_d_n6, assign101880_e153913_d_n7, assign101880_e153913_d_n8, assign101880_e153913_d_n9, assign101880_e153913_d_n10, assign101880_e153913_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101880_e153909: f64 = (locals.var_qb + locals.var_qi);
        let assign101880_e153910: f64 = (-assign101880_e153909);
        let assign101880_e153911: f64 = (locals.var_mfactor * assign101880_e153910);
        (assign101880_e153911, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn13 + locals.var_qi_dn13))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign101880_e153913;
        locals.var_qge_dn0 = assign101880_e153913_d_n0;
        locals.var_qge_dn2 = assign101880_e153913_d_n2;
        locals.var_qge_dn4 = assign101880_e153913_d_n4;
        locals.var_qge_dn5 = assign101880_e153913_d_n5;
        locals.var_qge_dn6 = assign101880_e153913_d_n6;
        locals.var_qge_dn7 = assign101880_e153913_d_n7;
        locals.var_qge_dn8 = assign101880_e153913_d_n8;
        locals.var_qge_dn9 = assign101880_e153913_d_n9;
        locals.var_qge_dn10 = assign101880_e153913_d_n10;
        locals.var_qge_dn13 = assign101880_e153913_d_n13;

        let (assign101890_e153920, assign101890_e153920_d_n0, assign101890_e153920_d_n2, assign101890_e153920_d_n4, assign101890_e153920_d_n5, assign101890_e153920_d_n6, assign101890_e153920_d_n7, assign101890_e153920_d_n8, assign101890_e153920_d_n9, assign101890_e153920_d_n10, assign101890_e153920_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101890_e153918: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101890_e153918, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn13),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign101890_e153920;
        locals.var_qde_dn0 = assign101890_e153920_d_n0;
        locals.var_qde_dn2 = assign101890_e153920_d_n2;
        locals.var_qde_dn4 = assign101890_e153920_d_n4;
        locals.var_qde_dn5 = assign101890_e153920_d_n5;
        locals.var_qde_dn6 = assign101890_e153920_d_n6;
        locals.var_qde_dn7 = assign101890_e153920_d_n7;
        locals.var_qde_dn8 = assign101890_e153920_d_n8;
        locals.var_qde_dn9 = assign101890_e153920_d_n9;
        locals.var_qde_dn10 = assign101890_e153920_d_n10;
        locals.var_qde_dn13 = assign101890_e153920_d_n13;

    }

    pub(super) fn stamp_transient_block_361(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (assign101900_e153929, assign101900_e153929_d_n0, assign101900_e153929_d_n2, assign101900_e153929_d_n4, assign101900_e153929_d_n5, assign101900_e153929_d_n6, assign101900_e153929_d_n7, assign101900_e153929_d_n8, assign101900_e153929_d_n9, assign101900_e153929_d_n10, assign101900_e153929_d_n13,) = {
    if (locals.var_guard2329 == 0.0) {
        let assign101900_e153926: f64 = (locals.var_qi - locals.var_qd);
        let assign101900_e153927: f64 = (locals.var_mfactor * assign101900_e153926);
        (assign101900_e153927, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn13 - locals.var_qd_dn13)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign101900_e153929;
        locals.var_qse_dn0 = assign101900_e153929_d_n0;
        locals.var_qse_dn2 = assign101900_e153929_d_n2;
        locals.var_qse_dn4 = assign101900_e153929_d_n4;
        locals.var_qse_dn5 = assign101900_e153929_d_n5;
        locals.var_qse_dn6 = assign101900_e153929_d_n6;
        locals.var_qse_dn7 = assign101900_e153929_d_n7;
        locals.var_qse_dn8 = assign101900_e153929_d_n8;
        locals.var_qse_dn9 = assign101900_e153929_d_n9;
        locals.var_qse_dn10 = assign101900_e153929_d_n10;
        locals.var_qse_dn13 = assign101900_e153929_d_n13;

        let (assign101910_e153935, assign101910_e153935_d_n0, assign101910_e153935_d_n2, assign101910_e153935_d_n4, assign101910_e153935_d_n5, assign101910_e153935_d_n6, assign101910_e153935_d_n7, assign101910_e153935_d_n8, assign101910_e153935_d_n9, assign101910_e153935_d_n10, assign101910_e153935_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101910_e153933: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101910_e153933, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn13 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn13)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101910_e153935;
        locals.var_mks_dlyov_dn0 = assign101910_e153935_d_n0;
        locals.var_mks_dlyov_dn2 = assign101910_e153935_d_n2;
        locals.var_mks_dlyov_dn4 = assign101910_e153935_d_n4;
        locals.var_mks_dlyov_dn5 = assign101910_e153935_d_n5;
        locals.var_mks_dlyov_dn6 = assign101910_e153935_d_n6;
        locals.var_mks_dlyov_dn7 = assign101910_e153935_d_n7;
        locals.var_mks_dlyov_dn8 = assign101910_e153935_d_n8;
        locals.var_mks_dlyov_dn9 = assign101910_e153935_d_n9;
        locals.var_mks_dlyov_dn10 = assign101910_e153935_d_n10;
        locals.var_mks_dlyov_dn13 = assign101910_e153935_d_n13;

        let (assign101920_e153948, assign101920_e153948_d_n0, assign101920_e153948_d_n2, assign101920_e153948_d_n4, assign101920_e153948_d_n5, assign101920_e153948_d_n6, assign101920_e153948_d_n7, assign101920_e153948_d_n8, assign101920_e153948_d_n9, assign101920_e153948_d_n10, assign101920_e153948_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101920_e153939: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101920_e153942: f64 = (4.0 * 1e-12);
        let assign101920_e153944: f64 = (assign101920_e153942 * 1e-12);
        let assign101920_e153945: f64 = (assign101920_e153939 + assign101920_e153944);
        let assign101920_e153946: f64 = (assign101920_e153945).sqrt();
        (assign101920_e153946, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101920_e153946)), (((locals.var_mks_dlyov_dn13 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn13)) / (2.0 * assign101920_e153946)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign101920_e153948;
        locals.var_tmf2_dn0 = assign101920_e153948_d_n0;
        locals.var_tmf2_dn2 = assign101920_e153948_d_n2;
        locals.var_tmf2_dn4 = assign101920_e153948_d_n4;
        locals.var_tmf2_dn5 = assign101920_e153948_d_n5;
        locals.var_tmf2_dn6 = assign101920_e153948_d_n6;
        locals.var_tmf2_dn7 = assign101920_e153948_d_n7;
        locals.var_tmf2_dn8 = assign101920_e153948_d_n8;
        locals.var_tmf2_dn9 = assign101920_e153948_d_n9;
        locals.var_tmf2_dn10 = assign101920_e153948_d_n10;
        locals.var_tmf2_dn13 = assign101920_e153948_d_n13;

        let (assign101930_e153958, assign101930_e153958_d_n0, assign101930_e153958_d_n2, assign101930_e153958_d_n4, assign101930_e153958_d_n5, assign101930_e153958_d_n6, assign101930_e153958_d_n7, assign101930_e153958_d_n8, assign101930_e153958_d_n9, assign101930_e153958_d_n10, assign101930_e153958_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101930_e153954: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101930_e153955: f64 = (1.0 + assign101930_e153954);
        let assign101930_e153956: f64 = (0.5 * assign101930_e153955);
        (assign101930_e153956, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn13 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101930_e153958;
        locals.var_t0_dn0 = assign101930_e153958_d_n0;
        locals.var_t0_dn2 = assign101930_e153958_d_n2;
        locals.var_t0_dn4 = assign101930_e153958_d_n4;
        locals.var_t0_dn5 = assign101930_e153958_d_n5;
        locals.var_t0_dn6 = assign101930_e153958_d_n6;
        locals.var_t0_dn7 = assign101930_e153958_d_n7;
        locals.var_t0_dn8 = assign101930_e153958_d_n8;
        locals.var_t0_dn9 = assign101930_e153958_d_n9;
        locals.var_t0_dn10 = assign101930_e153958_d_n10;
        locals.var_t0_dn13 = assign101930_e153958_d_n13;

        let (assign101940_e153966, assign101940_e153966_d_n0, assign101940_e153966_d_n2, assign101940_e153966_d_n4, assign101940_e153966_d_n5, assign101940_e153966_d_n6, assign101940_e153966_d_n7, assign101940_e153966_d_n8, assign101940_e153966_d_n9, assign101940_e153966_d_n10, assign101940_e153966_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101940_e153963: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101940_e153964: f64 = (0.5 * assign101940_e153963);
        (assign101940_e153964, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101940_e153966;
        locals.var_mks_dlyov_dn0 = assign101940_e153966_d_n0;
        locals.var_mks_dlyov_dn2 = assign101940_e153966_d_n2;
        locals.var_mks_dlyov_dn4 = assign101940_e153966_d_n4;
        locals.var_mks_dlyov_dn5 = assign101940_e153966_d_n5;
        locals.var_mks_dlyov_dn6 = assign101940_e153966_d_n6;
        locals.var_mks_dlyov_dn7 = assign101940_e153966_d_n7;
        locals.var_mks_dlyov_dn8 = assign101940_e153966_d_n8;
        locals.var_mks_dlyov_dn9 = assign101940_e153966_d_n9;
        locals.var_mks_dlyov_dn10 = assign101940_e153966_d_n10;
        locals.var_mks_dlyov_dn13 = assign101940_e153966_d_n13;

        let assign101950_e153969: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101950_e153969;

        let (assign101960_e153975, assign101960_e153975_d_n0, assign101960_e153975_d_n2, assign101960_e153975_d_n4, assign101960_e153975_d_n5, assign101960_e153975_d_n6, assign101960_e153975_d_n7, assign101960_e153975_d_n8, assign101960_e153975_d_n9, assign101960_e153975_d_n10, assign101960_e153975_d_n13,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn13,)
    }
};
        locals.var_mks_dlyov = assign101960_e153975;
        locals.var_mks_dlyov_dn0 = assign101960_e153975_d_n0;
        locals.var_mks_dlyov_dn2 = assign101960_e153975_d_n2;
        locals.var_mks_dlyov_dn4 = assign101960_e153975_d_n4;
        locals.var_mks_dlyov_dn5 = assign101960_e153975_d_n5;
        locals.var_mks_dlyov_dn6 = assign101960_e153975_d_n6;
        locals.var_mks_dlyov_dn7 = assign101960_e153975_d_n7;
        locals.var_mks_dlyov_dn8 = assign101960_e153975_d_n8;
        locals.var_mks_dlyov_dn9 = assign101960_e153975_d_n9;
        locals.var_mks_dlyov_dn10 = assign101960_e153975_d_n10;
        locals.var_mks_dlyov_dn13 = assign101960_e153975_d_n13;

        let (assign101970_e153981, assign101970_e153981_d_n0, assign101970_e153981_d_n2, assign101970_e153981_d_n4, assign101970_e153981_d_n5, assign101970_e153981_d_n6, assign101970_e153981_d_n7, assign101970_e153981_d_n8, assign101970_e153981_d_n9, assign101970_e153981_d_n10, assign101970_e153981_d_n13,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign101970_e153981;
        locals.var_t0_dn0 = assign101970_e153981_d_n0;
        locals.var_t0_dn2 = assign101970_e153981_d_n2;
        locals.var_t0_dn4 = assign101970_e153981_d_n4;
        locals.var_t0_dn5 = assign101970_e153981_d_n5;
        locals.var_t0_dn6 = assign101970_e153981_d_n6;
        locals.var_t0_dn7 = assign101970_e153981_d_n7;
        locals.var_t0_dn8 = assign101970_e153981_d_n8;
        locals.var_t0_dn9 = assign101970_e153981_d_n9;
        locals.var_t0_dn10 = assign101970_e153981_d_n10;
        locals.var_t0_dn13 = assign101970_e153981_d_n13;

        let (assign101980_e153987, assign101980_e153987_d_n0, assign101980_e153987_d_n2, assign101980_e153987_d_n4, assign101980_e153987_d_n5, assign101980_e153987_d_n6, assign101980_e153987_d_n7, assign101980_e153987_d_n8, assign101980_e153987_d_n9, assign101980_e153987_d_n10, assign101980_e153987_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign101980_e153985: f64 = (locals.var_mks_dlyov * locals.var_cox0);
        (assign101980_e153985, (locals.var_mks_dlyov_dn0 * locals.var_cox0), (locals.var_mks_dlyov_dn2 * locals.var_cox0), (locals.var_mks_dlyov_dn4 * locals.var_cox0), (locals.var_mks_dlyov_dn5 * locals.var_cox0), (locals.var_mks_dlyov_dn6 * locals.var_cox0), (locals.var_mks_dlyov_dn7 * locals.var_cox0), (locals.var_mks_dlyov_dn8 * locals.var_cox0), (locals.var_mks_dlyov_dn9 * locals.var_cox0), (locals.var_mks_dlyov_dn10 * locals.var_cox0), (locals.var_mks_dlyov_dn13 * locals.var_cox0),)
    } else {
        (locals.var_tauov, locals.var_tauov_dn0, locals.var_tauov_dn2, locals.var_tauov_dn4, locals.var_tauov_dn5, locals.var_tauov_dn6, locals.var_tauov_dn7, locals.var_tauov_dn8, locals.var_tauov_dn9, locals.var_tauov_dn10, locals.var_tauov_dn13,)
    }
};
        locals.var_tauov = assign101980_e153987;
        locals.var_tauov_dn0 = assign101980_e153987_d_n0;
        locals.var_tauov_dn2 = assign101980_e153987_d_n2;
        locals.var_tauov_dn4 = assign101980_e153987_d_n4;
        locals.var_tauov_dn5 = assign101980_e153987_d_n5;
        locals.var_tauov_dn6 = assign101980_e153987_d_n6;
        locals.var_tauov_dn7 = assign101980_e153987_d_n7;
        locals.var_tauov_dn8 = assign101980_e153987_d_n8;
        locals.var_tauov_dn9 = assign101980_e153987_d_n9;
        locals.var_tauov_dn10 = assign101980_e153987_d_n10;
        locals.var_tauov_dn13 = assign101980_e153987_d_n13;

        let (assign101990_e153991, assign101990_e153991_d_n0, assign101990_e153991_d_n2, assign101990_e153991_d_n4, assign101990_e153991_d_n5, assign101990_e153991_d_n6, assign101990_e153991_d_n7, assign101990_e153991_d_n8, assign101990_e153991_d_n9, assign101990_e153991_d_n10, assign101990_e153991_d_n13,) = {
    if (p.p29 != 0.0) {
        ((nv13 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    }
};
        locals.var_qbd_nqs = assign101990_e153991;
        locals.var_qbd_nqs_dn0 = assign101990_e153991_d_n0;
        locals.var_qbd_nqs_dn2 = assign101990_e153991_d_n2;
        locals.var_qbd_nqs_dn4 = assign101990_e153991_d_n4;
        locals.var_qbd_nqs_dn5 = assign101990_e153991_d_n5;
        locals.var_qbd_nqs_dn6 = assign101990_e153991_d_n6;
        locals.var_qbd_nqs_dn7 = assign101990_e153991_d_n7;
        locals.var_qbd_nqs_dn8 = assign101990_e153991_d_n8;
        locals.var_qbd_nqs_dn9 = assign101990_e153991_d_n9;
        locals.var_qbd_nqs_dn10 = assign101990_e153991_d_n10;
        locals.var_qbd_nqs_dn13 = assign101990_e153991_d_n13;

        let (assign102000_e153999, assign102000_e153999_d_n0, assign102000_e153999_d_n2, assign102000_e153999_d_n4, assign102000_e153999_d_n5, assign102000_e153999_d_n6, assign102000_e153999_d_n7, assign102000_e153999_d_n8, assign102000_e153999_d_n9, assign102000_e153999_d_n10, assign102000_e153999_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign102000_e153995: f64 = (locals.var_qbd_nqs - locals.var_qbd_qs);
        let assign102000_e153997: f64 = (assign102000_e153995 / locals.var_tauov);
        (assign102000_e153997, ((((locals.var_qbd_nqs_dn0 - locals.var_qbd_qs_dn0) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn0)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn2 - locals.var_qbd_qs_dn2) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn2)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn4 - locals.var_qbd_qs_dn4) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn4)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn5 - locals.var_qbd_qs_dn5) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn5)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn6 - locals.var_qbd_qs_dn6) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn6)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn7 - locals.var_qbd_qs_dn7) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn7)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn8 - locals.var_qbd_qs_dn8) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn8)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn9 - locals.var_qbd_qs_dn9) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn9)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn10 - locals.var_qbd_qs_dn10) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn10)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn13 - locals.var_qbd_qs_dn13) * locals.var_tauov) - (assign102000_e153995 * locals.var_tauov_dn13)) / (locals.var_tauov * locals.var_tauov)),)
    } else {
        (locals.var_ibd_nqs, locals.var_ibd_nqs_dn0, locals.var_ibd_nqs_dn2, locals.var_ibd_nqs_dn4, locals.var_ibd_nqs_dn5, locals.var_ibd_nqs_dn6, locals.var_ibd_nqs_dn7, locals.var_ibd_nqs_dn8, locals.var_ibd_nqs_dn9, locals.var_ibd_nqs_dn10, locals.var_ibd_nqs_dn13,)
    }
};
        locals.var_ibd_nqs = assign102000_e153999;
        locals.var_ibd_nqs_dn0 = assign102000_e153999_d_n0;
        locals.var_ibd_nqs_dn2 = assign102000_e153999_d_n2;
        locals.var_ibd_nqs_dn4 = assign102000_e153999_d_n4;
        locals.var_ibd_nqs_dn5 = assign102000_e153999_d_n5;
        locals.var_ibd_nqs_dn6 = assign102000_e153999_d_n6;
        locals.var_ibd_nqs_dn7 = assign102000_e153999_d_n7;
        locals.var_ibd_nqs_dn8 = assign102000_e153999_d_n8;
        locals.var_ibd_nqs_dn9 = assign102000_e153999_d_n9;
        locals.var_ibd_nqs_dn10 = assign102000_e153999_d_n10;
        locals.var_ibd_nqs_dn13 = assign102000_e153999_d_n13;

        let (assign102010_e154007, assign102010_e154007_d_n0, assign102010_e154007_d_n2, assign102010_e154007_d_n4, assign102010_e154007_d_n5, assign102010_e154007_d_n6, assign102010_e154007_d_n7, assign102010_e154007_d_n8, assign102010_e154007_d_n9, assign102010_e154007_d_n10, assign102010_e154007_d_n13,) = {
    if (p.p29 != 0.0) {
        let assign102010_e154004: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102010_e154005: f64 = (locals.var_qovd - assign102010_e154004);
        (assign102010_e154005, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn13 - (locals.var_qbd_qs_dn13 - locals.var_qbd_nqs_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign102010_e154007;
        locals.var_qovd_dn0 = assign102010_e154007_d_n0;
        locals.var_qovd_dn2 = assign102010_e154007_d_n2;
        locals.var_qovd_dn4 = assign102010_e154007_d_n4;
        locals.var_qovd_dn5 = assign102010_e154007_d_n5;
        locals.var_qovd_dn6 = assign102010_e154007_d_n6;
        locals.var_qovd_dn7 = assign102010_e154007_d_n7;
        locals.var_qovd_dn8 = assign102010_e154007_d_n8;
        locals.var_qovd_dn9 = assign102010_e154007_d_n9;
        locals.var_qovd_dn10 = assign102010_e154007_d_n10;
        locals.var_qovd_dn13 = assign102010_e154007_d_n13;

        let (assign102020_e154011, assign102020_e154011_d_n0, assign102020_e154011_d_n2, assign102020_e154011_d_n4, assign102020_e154011_d_n5, assign102020_e154011_d_n6, assign102020_e154011_d_n7, assign102020_e154011_d_n8, assign102020_e154011_d_n9, assign102020_e154011_d_n10, assign102020_e154011_d_n13,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign102020_e154011;
        locals.var_qbdld_dn0 = assign102020_e154011_d_n0;
        locals.var_qbdld_dn2 = assign102020_e154011_d_n2;
        locals.var_qbdld_dn4 = assign102020_e154011_d_n4;
        locals.var_qbdld_dn5 = assign102020_e154011_d_n5;
        locals.var_qbdld_dn6 = assign102020_e154011_d_n6;
        locals.var_qbdld_dn7 = assign102020_e154011_d_n7;
        locals.var_qbdld_dn8 = assign102020_e154011_d_n8;
        locals.var_qbdld_dn9 = assign102020_e154011_d_n9;
        locals.var_qbdld_dn10 = assign102020_e154011_d_n10;
        locals.var_qbdld_dn13 = assign102020_e154011_d_n13;

        let (assign102030_e154016, assign102030_e154016_d_n0, assign102030_e154016_d_n2, assign102030_e154016_d_n4, assign102030_e154016_d_n5, assign102030_e154016_d_n6, assign102030_e154016_d_n7, assign102030_e154016_d_n8, assign102030_e154016_d_n9, assign102030_e154016_d_n10, assign102030_e154016_d_n13,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn13,)
    }
};
        locals.var_qbd_nqs = assign102030_e154016;
        locals.var_qbd_nqs_dn0 = assign102030_e154016_d_n0;
        locals.var_qbd_nqs_dn2 = assign102030_e154016_d_n2;
        locals.var_qbd_nqs_dn4 = assign102030_e154016_d_n4;
        locals.var_qbd_nqs_dn5 = assign102030_e154016_d_n5;
        locals.var_qbd_nqs_dn6 = assign102030_e154016_d_n6;
        locals.var_qbd_nqs_dn7 = assign102030_e154016_d_n7;
        locals.var_qbd_nqs_dn8 = assign102030_e154016_d_n8;
        locals.var_qbd_nqs_dn9 = assign102030_e154016_d_n9;
        locals.var_qbd_nqs_dn10 = assign102030_e154016_d_n10;
        locals.var_qbd_nqs_dn13 = assign102030_e154016_d_n13;

        let assign102040_e154019: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign102040_e154019;

        let (assign102050_e154033, assign102050_e154033_d_n0, assign102050_e154033_d_n2, assign102050_e154033_d_n4, assign102050_e154033_d_n5, assign102050_e154033_d_n6, assign102050_e154033_d_n7, assign102050_e154033_d_n8, assign102050_e154033_d_n9, assign102050_e154033_d_n10, assign102050_e154033_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102050_e154024: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102050_e154026: f64 = (assign102050_e154024 - locals.var_qovs);
        let assign102050_e154028: f64 = (assign102050_e154026 + locals.var_qgos);
        let assign102050_e154030: f64 = (assign102050_e154028 + locals.var_qgod);
        let assign102050_e154031: f64 = (locals.var_mfactor * assign102050_e154030);
        (assign102050_e154031, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((locals.var_qgbo_dn6 - locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((-locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn13) - locals.var_qovs_dn13) + locals.var_qgos_dn13) + locals.var_qgod_dn13)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn13,)
    }
};
        locals.var_qgov = assign102050_e154033;
        locals.var_qgov_dn0 = assign102050_e154033_d_n0;
        locals.var_qgov_dn2 = assign102050_e154033_d_n2;
        locals.var_qgov_dn4 = assign102050_e154033_d_n4;
        locals.var_qgov_dn5 = assign102050_e154033_d_n5;
        locals.var_qgov_dn6 = assign102050_e154033_d_n6;
        locals.var_qgov_dn7 = assign102050_e154033_d_n7;
        locals.var_qgov_dn8 = assign102050_e154033_d_n8;
        locals.var_qgov_dn9 = assign102050_e154033_d_n9;
        locals.var_qgov_dn10 = assign102050_e154033_d_n10;
        locals.var_qgov_dn13 = assign102050_e154033_d_n13;

        let (assign102060_e154042, assign102060_e154042_d_n0, assign102060_e154042_d_n2, assign102060_e154042_d_n4, assign102060_e154042_d_n5, assign102060_e154042_d_n6, assign102060_e154042_d_n7, assign102060_e154042_d_n8, assign102060_e154042_d_n9, assign102060_e154042_d_n10, assign102060_e154042_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102060_e154037: f64 = locals.var_qbdld;
        let assign102060_e154039: f64 = (assign102060_e154037 - locals.var_qgod);
        let assign102060_e154040: f64 = (locals.var_mfactor * assign102060_e154039);
        (assign102060_e154040, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn13 - locals.var_qgod_dn13)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn13,)
    }
};
        locals.var_qdov = assign102060_e154042;
        locals.var_qdov_dn0 = assign102060_e154042_d_n0;
        locals.var_qdov_dn2 = assign102060_e154042_d_n2;
        locals.var_qdov_dn4 = assign102060_e154042_d_n4;
        locals.var_qdov_dn5 = assign102060_e154042_d_n5;
        locals.var_qdov_dn6 = assign102060_e154042_d_n6;
        locals.var_qdov_dn7 = assign102060_e154042_d_n7;
        locals.var_qdov_dn8 = assign102060_e154042_d_n8;
        locals.var_qdov_dn9 = assign102060_e154042_d_n9;
        locals.var_qdov_dn10 = assign102060_e154042_d_n10;
        locals.var_qdov_dn13 = assign102060_e154042_d_n13;

        let (assign102070_e154051, assign102070_e154051_d_n0, assign102070_e154051_d_n2, assign102070_e154051_d_n4, assign102070_e154051_d_n5, assign102070_e154051_d_n6, assign102070_e154051_d_n7, assign102070_e154051_d_n8, assign102070_e154051_d_n9, assign102070_e154051_d_n10, assign102070_e154051_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102070_e154046: f64 = locals.var_qbsld;
        let assign102070_e154048: f64 = (assign102070_e154046 - locals.var_qgos);
        let assign102070_e154049: f64 = (locals.var_mfactor * assign102070_e154048);
        (assign102070_e154049, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn13 - locals.var_qgos_dn13)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn13,)
    }
};
        locals.var_qsov = assign102070_e154051;
        locals.var_qsov_dn0 = assign102070_e154051_d_n0;
        locals.var_qsov_dn2 = assign102070_e154051_d_n2;
        locals.var_qsov_dn4 = assign102070_e154051_d_n4;
        locals.var_qsov_dn5 = assign102070_e154051_d_n5;
        locals.var_qsov_dn6 = assign102070_e154051_d_n6;
        locals.var_qsov_dn7 = assign102070_e154051_d_n7;
        locals.var_qsov_dn8 = assign102070_e154051_d_n8;
        locals.var_qsov_dn9 = assign102070_e154051_d_n9;
        locals.var_qsov_dn10 = assign102070_e154051_d_n10;
        locals.var_qsov_dn13 = assign102070_e154051_d_n13;

        let (assign102080_e154064, assign102080_e154064_d_n0, assign102080_e154064_d_n2, assign102080_e154064_d_n4, assign102080_e154064_d_n5, assign102080_e154064_d_n6, assign102080_e154064_d_n7, assign102080_e154064_d_n8, assign102080_e154064_d_n9, assign102080_e154064_d_n10, assign102080_e154064_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102080_e154056: f64 = locals.var_qy;
        let assign102080_e154058: f64 = (assign102080_e154056 - locals.var_qovd_add);
        let assign102080_e154060: f64 = (assign102080_e154058 - locals.var_qovs_add);
        let assign102080_e154061: f64 = (locals.var_mfactor * assign102080_e154060);
        let assign102080_e154062: f64 = (locals.var_qge + assign102080_e154061);
        (assign102080_e154062, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn13 + (locals.var_mfactor * ((locals.var_qy_dn13 - locals.var_qovd_add_dn13) - locals.var_qovs_add_dn13))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    }
};
        locals.var_qge = assign102080_e154064;
        locals.var_qge_dn0 = assign102080_e154064_d_n0;
        locals.var_qge_dn2 = assign102080_e154064_d_n2;
        locals.var_qge_dn4 = assign102080_e154064_d_n4;
        locals.var_qge_dn5 = assign102080_e154064_d_n5;
        locals.var_qge_dn6 = assign102080_e154064_d_n6;
        locals.var_qge_dn7 = assign102080_e154064_d_n7;
        locals.var_qge_dn8 = assign102080_e154064_d_n8;
        locals.var_qge_dn9 = assign102080_e154064_d_n9;
        locals.var_qge_dn10 = assign102080_e154064_d_n10;
        locals.var_qge_dn13 = assign102080_e154064_d_n13;

        let (assign102090_e154075, assign102090_e154075_d_n0, assign102090_e154075_d_n2, assign102090_e154075_d_n4, assign102090_e154075_d_n5, assign102090_e154075_d_n6, assign102090_e154075_d_n7, assign102090_e154075_d_n8, assign102090_e154075_d_n9, assign102090_e154075_d_n10, assign102090_e154075_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102090_e154069: f64 = (-locals.var_qy);
        let assign102090_e154071: f64 = (assign102090_e154069 + locals.var_qbdld_add);
        let assign102090_e154072: f64 = (locals.var_mfactor * assign102090_e154071);
        let assign102090_e154073: f64 = (locals.var_qde + assign102090_e154072);
        (assign102090_e154073, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn13 + (locals.var_mfactor * ((-locals.var_qy_dn13) + locals.var_qbdld_add_dn13))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    }
};
        locals.var_qde = assign102090_e154075;
        locals.var_qde_dn0 = assign102090_e154075_d_n0;
        locals.var_qde_dn2 = assign102090_e154075_d_n2;
        locals.var_qde_dn4 = assign102090_e154075_d_n4;
        locals.var_qde_dn5 = assign102090_e154075_d_n5;
        locals.var_qde_dn6 = assign102090_e154075_d_n6;
        locals.var_qde_dn7 = assign102090_e154075_d_n7;
        locals.var_qde_dn8 = assign102090_e154075_d_n8;
        locals.var_qde_dn9 = assign102090_e154075_d_n9;
        locals.var_qde_dn10 = assign102090_e154075_d_n10;
        locals.var_qde_dn13 = assign102090_e154075_d_n13;

        let (assign102100_e154084, assign102100_e154084_d_n0, assign102100_e154084_d_n2, assign102100_e154084_d_n4, assign102100_e154084_d_n5, assign102100_e154084_d_n6, assign102100_e154084_d_n7, assign102100_e154084_d_n8, assign102100_e154084_d_n9, assign102100_e154084_d_n10, assign102100_e154084_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102100_e154080: f64 = locals.var_qbsld_add;
        let assign102100_e154081: f64 = (locals.var_mfactor * assign102100_e154080);
        let assign102100_e154082: f64 = (locals.var_qse + assign102100_e154081);
        (assign102100_e154082, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn13 + (locals.var_mfactor * locals.var_qbsld_add_dn13)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    }
};
        locals.var_qse = assign102100_e154084;
        locals.var_qse_dn0 = assign102100_e154084_d_n0;
        locals.var_qse_dn2 = assign102100_e154084_d_n2;
        locals.var_qse_dn4 = assign102100_e154084_d_n4;
        locals.var_qse_dn5 = assign102100_e154084_d_n5;
        locals.var_qse_dn6 = assign102100_e154084_d_n6;
        locals.var_qse_dn7 = assign102100_e154084_d_n7;
        locals.var_qse_dn8 = assign102100_e154084_d_n8;
        locals.var_qse_dn9 = assign102100_e154084_d_n9;
        locals.var_qse_dn10 = assign102100_e154084_d_n10;
        locals.var_qse_dn13 = assign102100_e154084_d_n13;

        let (assign102110_e154093, assign102110_e154093_d_n0, assign102110_e154093_d_n2, assign102110_e154093_d_n4, assign102110_e154093_d_n5, assign102110_e154093_d_n6, assign102110_e154093_d_n7, assign102110_e154093_d_n8, assign102110_e154093_d_n9, assign102110_e154093_d_n10, assign102110_e154093_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102110_e154088: f64 = (-locals.var_qovdext);
        let assign102110_e154090: f64 = (assign102110_e154088 - locals.var_qovsext);
        let assign102110_e154091: f64 = (locals.var_mfactor * assign102110_e154090);
        (assign102110_e154091, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn13) - locals.var_qovsext_dn13)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn13,)
    }
};
        locals.var_qgexte = assign102110_e154093;
        locals.var_qgexte_dn0 = assign102110_e154093_d_n0;
        locals.var_qgexte_dn2 = assign102110_e154093_d_n2;
        locals.var_qgexte_dn4 = assign102110_e154093_d_n4;
        locals.var_qgexte_dn5 = assign102110_e154093_d_n5;
        locals.var_qgexte_dn6 = assign102110_e154093_d_n6;
        locals.var_qgexte_dn7 = assign102110_e154093_d_n7;
        locals.var_qgexte_dn8 = assign102110_e154093_d_n8;
        locals.var_qgexte_dn9 = assign102110_e154093_d_n9;
        locals.var_qgexte_dn10 = assign102110_e154093_d_n10;
        locals.var_qgexte_dn13 = assign102110_e154093_d_n13;

        let (assign102120_e154099, assign102120_e154099_d_n0, assign102120_e154099_d_n2, assign102120_e154099_d_n4, assign102120_e154099_d_n5, assign102120_e154099_d_n6, assign102120_e154099_d_n7, assign102120_e154099_d_n8, assign102120_e154099_d_n9, assign102120_e154099_d_n10, assign102120_e154099_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102120_e154097: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102120_e154097, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn13),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn13,)
    }
};
        locals.var_qdexte = assign102120_e154099;
        locals.var_qdexte_dn0 = assign102120_e154099_d_n0;
        locals.var_qdexte_dn2 = assign102120_e154099_d_n2;
        locals.var_qdexte_dn4 = assign102120_e154099_d_n4;
        locals.var_qdexte_dn5 = assign102120_e154099_d_n5;
        locals.var_qdexte_dn6 = assign102120_e154099_d_n6;
        locals.var_qdexte_dn7 = assign102120_e154099_d_n7;
        locals.var_qdexte_dn8 = assign102120_e154099_d_n8;
        locals.var_qdexte_dn9 = assign102120_e154099_d_n9;
        locals.var_qdexte_dn10 = assign102120_e154099_d_n10;
        locals.var_qdexte_dn13 = assign102120_e154099_d_n13;

        let (assign102130_e154105, assign102130_e154105_d_n0, assign102130_e154105_d_n2, assign102130_e154105_d_n4, assign102130_e154105_d_n5, assign102130_e154105_d_n6, assign102130_e154105_d_n7, assign102130_e154105_d_n8, assign102130_e154105_d_n9, assign102130_e154105_d_n10, assign102130_e154105_d_n13,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102130_e154103: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102130_e154103, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn13),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn13,)
    }
};
        locals.var_qsexte = assign102130_e154105;
        locals.var_qsexte_dn0 = assign102130_e154105_d_n0;
        locals.var_qsexte_dn2 = assign102130_e154105_d_n2;
        locals.var_qsexte_dn4 = assign102130_e154105_d_n4;
        locals.var_qsexte_dn5 = assign102130_e154105_d_n5;
        locals.var_qsexte_dn6 = assign102130_e154105_d_n6;
        locals.var_qsexte_dn7 = assign102130_e154105_d_n7;
        locals.var_qsexte_dn8 = assign102130_e154105_d_n8;
        locals.var_qsexte_dn9 = assign102130_e154105_d_n9;
        locals.var_qsexte_dn10 = assign102130_e154105_d_n10;
        locals.var_qsexte_dn13 = assign102130_e154105_d_n13;

        let (assign102140_e154116, assign102140_e154116_d_n0, assign102140_e154116_d_n2, assign102140_e154116_d_n6,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102140_e154110: f64 = (-locals.var_qfd);
        let assign102140_e154112: f64 = (assign102140_e154110 - locals.var_qgdo);
        let assign102140_e154113: f64 = (locals.var_mfactor * assign102140_e154112);
        let assign102140_e154114: f64 = (locals.var_qdp + assign102140_e154113);
        (assign102140_e154114, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn6 + (locals.var_mfactor * ((-locals.var_qfd_dn6) - locals.var_qgdo_dn6))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn6,)
    }
};
        locals.var_qdp = assign102140_e154116;
        locals.var_qdp_dn0 = assign102140_e154116_d_n0;
        locals.var_qdp_dn2 = assign102140_e154116_d_n2;
        locals.var_qdp_dn6 = assign102140_e154116_d_n6;

        let (assign102150_e154127, assign102150_e154127_d_n2, assign102150_e154127_d_n6,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign102150_e154121: f64 = (-locals.var_qfs);
        let assign102150_e154123: f64 = (assign102150_e154121 - locals.var_qgso);
        let assign102150_e154124: f64 = (locals.var_mfactor * assign102150_e154123);
        let assign102150_e154125: f64 = (locals.var_qsp + assign102150_e154124);
        (assign102150_e154125, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn6 + (locals.var_mfactor * ((-locals.var_qfs_dn6) - locals.var_qgso_dn6))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn6,)
    }
};
        locals.var_qsp = assign102150_e154127;
        locals.var_qsp_dn2 = assign102150_e154127_d_n2;
        locals.var_qsp_dn6 = assign102150_e154127_d_n6;

    }

    pub(super) fn stamp_transient_block_362(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign102160_e154131: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102160_e154132: f64 = (locals.var_mfactor * assign102160_e154131);
        locals.var_isube = assign102160_e154132;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn13 = (locals.var_mfactor * (locals.var_isub_dn13 + locals.var_isubibpc_dn13));

        let assign102170_e154135: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102170_e154135;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn13 = (locals.var_mfactor * locals.var_isubld_dn13);

        let assign102180_e154138: f64 = (-locals.var_igb);
        let assign102180_e154139: f64 = (locals.var_mfactor * assign102180_e154138);
        locals.var_igbe = assign102180_e154139;
        locals.var_igbe_dn0 = (locals.var_mfactor * (-locals.var_igb_dn0));
        locals.var_igbe_dn2 = (locals.var_mfactor * (-locals.var_igb_dn2));
        locals.var_igbe_dn4 = (locals.var_mfactor * (-locals.var_igb_dn4));
        locals.var_igbe_dn5 = (locals.var_mfactor * (-locals.var_igb_dn5));
        locals.var_igbe_dn6 = (locals.var_mfactor * (-locals.var_igb_dn6));
        locals.var_igbe_dn7 = (locals.var_mfactor * (-locals.var_igb_dn7));
        locals.var_igbe_dn8 = (locals.var_mfactor * (-locals.var_igb_dn8));
        locals.var_igbe_dn9 = (locals.var_mfactor * (-locals.var_igb_dn9));
        locals.var_igbe_dn10 = (locals.var_mfactor * (-locals.var_igb_dn10));
        locals.var_igbe_dn13 = (locals.var_mfactor * (-locals.var_igb_dn13));

        let assign102190_e154142: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2332 = assign102190_e154142;

        let (assign102200_e154152, assign102200_e154152_d_n0, assign102200_e154152_d_n2, assign102200_e154152_d_n4, assign102200_e154152_d_n5, assign102200_e154152_d_n6, assign102200_e154152_d_n7, assign102200_e154152_d_n8, assign102200_e154152_d_n9, assign102200_e154152_d_n10, assign102200_e154152_d_n13,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign102200_e154147: f64 = (p.p252 * locals.var_igate);
        let assign102200_e154149: f64 = (assign102200_e154147 - locals.var_igd);
        let assign102200_e154150: f64 = (locals.var_mfactor * assign102200_e154149);
        (assign102200_e154150, (locals.var_mfactor * ((p.p252 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn4) - locals.var_igd_dn4)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn5) - locals.var_igd_dn5)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn8) - locals.var_igd_dn8)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn9) - locals.var_igd_dn9)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn13) - locals.var_igd_dn13)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn4, locals.var_igde_dn5, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn8, locals.var_igde_dn9, locals.var_igde_dn10, locals.var_igde_dn13,)
    }
};
        locals.var_igde = assign102200_e154152;
        locals.var_igde_dn0 = assign102200_e154152_d_n0;
        locals.var_igde_dn2 = assign102200_e154152_d_n2;
        locals.var_igde_dn4 = assign102200_e154152_d_n4;
        locals.var_igde_dn5 = assign102200_e154152_d_n5;
        locals.var_igde_dn6 = assign102200_e154152_d_n6;
        locals.var_igde_dn7 = assign102200_e154152_d_n7;
        locals.var_igde_dn8 = assign102200_e154152_d_n8;
        locals.var_igde_dn9 = assign102200_e154152_d_n9;
        locals.var_igde_dn10 = assign102200_e154152_d_n10;
        locals.var_igde_dn13 = assign102200_e154152_d_n13;

        let (assign102210_e154165, assign102210_e154165_d_n0, assign102210_e154165_d_n2, assign102210_e154165_d_n4, assign102210_e154165_d_n5, assign102210_e154165_d_n6, assign102210_e154165_d_n7, assign102210_e154165_d_n8, assign102210_e154165_d_n9, assign102210_e154165_d_n10, assign102210_e154165_d_n13,) = {
    if (locals.var_guard2332 == 0.0) {
        let assign102210_e154158: f64 = (1.0 - p.p252);
        let assign102210_e154160: f64 = (assign102210_e154158 * locals.var_igate);
        let assign102210_e154162: f64 = (assign102210_e154160 - locals.var_igs);
        let assign102210_e154163: f64 = (locals.var_mfactor * assign102210_e154162);
        (assign102210_e154163, (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn4) - locals.var_igs_dn4)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn5) - locals.var_igs_dn5)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn8) - locals.var_igs_dn8)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn9) - locals.var_igs_dn9)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign102210_e154158 * locals.var_igate_dn13) - locals.var_igs_dn13)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn4, locals.var_igde_dn5, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn8, locals.var_igde_dn9, locals.var_igde_dn10, locals.var_igde_dn13,)
    }
};
        locals.var_igde = assign102210_e154165;
        locals.var_igde_dn0 = assign102210_e154165_d_n0;
        locals.var_igde_dn2 = assign102210_e154165_d_n2;
        locals.var_igde_dn4 = assign102210_e154165_d_n4;
        locals.var_igde_dn5 = assign102210_e154165_d_n5;
        locals.var_igde_dn6 = assign102210_e154165_d_n6;
        locals.var_igde_dn7 = assign102210_e154165_d_n7;
        locals.var_igde_dn8 = assign102210_e154165_d_n8;
        locals.var_igde_dn9 = assign102210_e154165_d_n9;
        locals.var_igde_dn10 = assign102210_e154165_d_n10;
        locals.var_igde_dn13 = assign102210_e154165_d_n13;

        let assign102220_e154168: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2333 = assign102220_e154168;

        let (assign102230_e154180, assign102230_e154180_d_n0, assign102230_e154180_d_n2, assign102230_e154180_d_n4, assign102230_e154180_d_n5, assign102230_e154180_d_n6, assign102230_e154180_d_n7, assign102230_e154180_d_n8, assign102230_e154180_d_n9, assign102230_e154180_d_n10, assign102230_e154180_d_n13,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102230_e154173: f64 = (1.0 - p.p252);
        let assign102230_e154175: f64 = (assign102230_e154173 * locals.var_igate);
        let assign102230_e154177: f64 = (assign102230_e154175 - locals.var_igs);
        let assign102230_e154178: f64 = (locals.var_mfactor * assign102230_e154177);
        (assign102230_e154178, (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn4) - locals.var_igs_dn4)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn5) - locals.var_igs_dn5)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn8) - locals.var_igs_dn8)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn9) - locals.var_igs_dn9)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign102230_e154173 * locals.var_igate_dn13) - locals.var_igs_dn13)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn4, locals.var_igse_dn5, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn8, locals.var_igse_dn9, locals.var_igse_dn10, locals.var_igse_dn13,)
    }
};
        locals.var_igse = assign102230_e154180;
        locals.var_igse_dn0 = assign102230_e154180_d_n0;
        locals.var_igse_dn2 = assign102230_e154180_d_n2;
        locals.var_igse_dn4 = assign102230_e154180_d_n4;
        locals.var_igse_dn5 = assign102230_e154180_d_n5;
        locals.var_igse_dn6 = assign102230_e154180_d_n6;
        locals.var_igse_dn7 = assign102230_e154180_d_n7;
        locals.var_igse_dn8 = assign102230_e154180_d_n8;
        locals.var_igse_dn9 = assign102230_e154180_d_n9;
        locals.var_igse_dn10 = assign102230_e154180_d_n10;
        locals.var_igse_dn13 = assign102230_e154180_d_n13;

        let (assign102240_e154191, assign102240_e154191_d_n0, assign102240_e154191_d_n2, assign102240_e154191_d_n4, assign102240_e154191_d_n5, assign102240_e154191_d_n6, assign102240_e154191_d_n7, assign102240_e154191_d_n8, assign102240_e154191_d_n9, assign102240_e154191_d_n10, assign102240_e154191_d_n13,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign102240_e154186: f64 = (p.p252 * locals.var_igate);
        let assign102240_e154188: f64 = (assign102240_e154186 - locals.var_igd);
        let assign102240_e154189: f64 = (locals.var_mfactor * assign102240_e154188);
        (assign102240_e154189, (locals.var_mfactor * ((p.p252 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn4) - locals.var_igd_dn4)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn5) - locals.var_igd_dn5)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn8) - locals.var_igd_dn8)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn9) - locals.var_igd_dn9)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn13) - locals.var_igd_dn13)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn4, locals.var_igse_dn5, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn8, locals.var_igse_dn9, locals.var_igse_dn10, locals.var_igse_dn13,)
    }
};
        locals.var_igse = assign102240_e154191;
        locals.var_igse_dn0 = assign102240_e154191_d_n0;
        locals.var_igse_dn2 = assign102240_e154191_d_n2;
        locals.var_igse_dn4 = assign102240_e154191_d_n4;
        locals.var_igse_dn5 = assign102240_e154191_d_n5;
        locals.var_igse_dn6 = assign102240_e154191_d_n6;
        locals.var_igse_dn7 = assign102240_e154191_d_n7;
        locals.var_igse_dn8 = assign102240_e154191_d_n8;
        locals.var_igse_dn9 = assign102240_e154191_d_n9;
        locals.var_igse_dn10 = assign102240_e154191_d_n10;
        locals.var_igse_dn13 = assign102240_e154191_d_n13;

        let assign102250_e154194: f64 = (locals.var_mfactor * locals.var_igidl);
        locals.var_igidle = assign102250_e154194;
        locals.var_igidle_dn0 = (locals.var_mfactor * locals.var_igidl_dn0);
        locals.var_igidle_dn2 = (locals.var_mfactor * locals.var_igidl_dn2);
        locals.var_igidle_dn4 = (locals.var_mfactor * locals.var_igidl_dn4);
        locals.var_igidle_dn5 = (locals.var_mfactor * locals.var_igidl_dn5);
        locals.var_igidle_dn6 = (locals.var_mfactor * locals.var_igidl_dn6);
        locals.var_igidle_dn7 = (locals.var_mfactor * locals.var_igidl_dn7);
        locals.var_igidle_dn8 = (locals.var_mfactor * locals.var_igidl_dn8);
        locals.var_igidle_dn9 = (locals.var_mfactor * locals.var_igidl_dn9);
        locals.var_igidle_dn10 = (locals.var_mfactor * locals.var_igidl_dn10);
        locals.var_igidle_dn13 = (locals.var_mfactor * locals.var_igidl_dn13);

        let assign102260_e154197: f64 = (locals.var_mfactor * locals.var_igisl);
        locals.var_igisle = assign102260_e154197;
        locals.var_igisle_dn0 = (locals.var_mfactor * locals.var_igisl_dn0);
        locals.var_igisle_dn2 = (locals.var_mfactor * locals.var_igisl_dn2);
        locals.var_igisle_dn4 = (locals.var_mfactor * locals.var_igisl_dn4);
        locals.var_igisle_dn5 = (locals.var_mfactor * locals.var_igisl_dn5);
        locals.var_igisle_dn6 = (locals.var_mfactor * locals.var_igisl_dn6);
        locals.var_igisle_dn7 = (locals.var_mfactor * locals.var_igisl_dn7);
        locals.var_igisle_dn8 = (locals.var_mfactor * locals.var_igisl_dn8);
        locals.var_igisle_dn9 = (locals.var_mfactor * locals.var_igisl_dn9);
        locals.var_igisle_dn10 = (locals.var_mfactor * locals.var_igisl_dn10);
        locals.var_igisle_dn13 = (locals.var_mfactor * locals.var_igisl_dn13);

        let assign102290_e154202: f64 = (4.0 * 1.3806226e-23);
        let assign102290_e154204: f64 = (assign102290_e154202 * locals.var_ttemp);
        let assign102290_e154206: f64 = assign102290_e154204;
        locals.var_whi_noise = assign102290_e154206;
        locals.var_whi_noise_dn0 = (assign102290_e154202 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102290_e154202 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102290_e154202 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102290_e154202 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102290_e154202 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102290_e154202 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102290_e154202 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102290_e154202 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102290_e154202 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn13 = (assign102290_e154202 * locals.var_ttemp_dn13);

        let assign102310_e154212: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102310_e154212;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn13 = (locals.var_mfactor * locals.var_nthrml_dn13);

        let assign102320_e154215: f64 = locals.var_qge_dn5;
        locals.var_cgdbd = assign102320_e154215;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;

        let assign102330_e154218: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102330_e154218;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn13 = (p.p87 * locals.var_cgdbd_dn13);

        let assign102340_e154221: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign102340_e154221;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;

        let assign102350_e154224: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102350_e154224;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn13 = (p.p87 * locals.var_cgsbd_dn13);

        let (assign102360_e154230, assign102360_e154230_d_n0, assign102360_e154230_d_n2, assign102360_e154230_d_n4, assign102360_e154230_d_n5, assign102360_e154230_d_n6, assign102360_e154230_d_n7, assign102360_e154230_d_n8, assign102360_e154230_d_n9, assign102360_e154230_d_n10, assign102360_e154230_d_n13,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13,)
    }
};
        locals.var_cgsb = assign102360_e154230;
        locals.var_cgsb_dn0 = assign102360_e154230_d_n0;
        locals.var_cgsb_dn2 = assign102360_e154230_d_n2;
        locals.var_cgsb_dn4 = assign102360_e154230_d_n4;
        locals.var_cgsb_dn5 = assign102360_e154230_d_n5;
        locals.var_cgsb_dn6 = assign102360_e154230_d_n6;
        locals.var_cgsb_dn7 = assign102360_e154230_d_n7;
        locals.var_cgsb_dn8 = assign102360_e154230_d_n8;
        locals.var_cgsb_dn9 = assign102360_e154230_d_n9;
        locals.var_cgsb_dn10 = assign102360_e154230_d_n10;
        locals.var_cgsb_dn13 = assign102360_e154230_d_n13;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn13 = 0.0;

        locals.var_noicross = 0.0;
        locals.var_noicross_dn0 = 0.0;
        locals.var_noicross_dn2 = 0.0;
        locals.var_noicross_dn4 = 0.0;
        locals.var_noicross_dn5 = 0.0;
        locals.var_noicross_dn6 = 0.0;
        locals.var_noicross_dn7 = 0.0;
        locals.var_noicross_dn8 = 0.0;
        locals.var_noicross_dn9 = 0.0;
        locals.var_noicross_dn10 = 0.0;
        locals.var_noicross_dn13 = 0.0;

        let assign102390_e154250: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2334 = assign102390_e154250;

        let (assign102400_e154260, assign102400_e154260_d_n0, assign102400_e154260_d_n2, assign102400_e154260_d_n4, assign102400_e154260_d_n5, assign102400_e154260_d_n6, assign102400_e154260_d_n7, assign102400_e154260_d_n8, assign102400_e154260_d_n9, assign102400_e154260_d_n10, assign102400_e154260_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102400_e154254: f64 = (1e-6 * locals.var_cox);
        let assign102400_e154256: f64 = (assign102400_e154254 * locals.var_weffcv_nf);
        let assign102400_e154258: f64 = (assign102400_e154256 * locals.var_leff);
        (assign102400_e154258, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn13) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign102400_e154260;
        locals.var_t0_dn0 = assign102400_e154260_d_n0;
        locals.var_t0_dn2 = assign102400_e154260_d_n2;
        locals.var_t0_dn4 = assign102400_e154260_d_n4;
        locals.var_t0_dn5 = assign102400_e154260_d_n5;
        locals.var_t0_dn6 = assign102400_e154260_d_n6;
        locals.var_t0_dn7 = assign102400_e154260_d_n7;
        locals.var_t0_dn8 = assign102400_e154260_d_n8;
        locals.var_t0_dn9 = assign102400_e154260_d_n9;
        locals.var_t0_dn10 = assign102400_e154260_d_n10;
        locals.var_t0_dn13 = assign102400_e154260_d_n13;

        let (assign102410_e154266, assign102410_e154266_d_n0, assign102410_e154266_d_n2, assign102410_e154266_d_n4, assign102410_e154266_d_n5, assign102410_e154266_d_n6, assign102410_e154266_d_n7, assign102410_e154266_d_n8, assign102410_e154266_d_n9, assign102410_e154266_d_n10, assign102410_e154266_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102410_e154264: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102410_e154264, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign102410_e154266;
        locals.var_t10_dn0 = assign102410_e154266_d_n0;
        locals.var_t10_dn2 = assign102410_e154266_d_n2;
        locals.var_t10_dn4 = assign102410_e154266_d_n4;
        locals.var_t10_dn5 = assign102410_e154266_d_n5;
        locals.var_t10_dn6 = assign102410_e154266_d_n6;
        locals.var_t10_dn7 = assign102410_e154266_d_n7;
        locals.var_t10_dn8 = assign102410_e154266_d_n8;
        locals.var_t10_dn9 = assign102410_e154266_d_n9;
        locals.var_t10_dn10 = assign102410_e154266_d_n10;
        locals.var_t10_dn13 = assign102410_e154266_d_n13;

        let (assign102420_e154280, assign102420_e154280_d_n0, assign102420_e154280_d_n2, assign102420_e154280_d_n4, assign102420_e154280_d_n5, assign102420_e154280_d_n6, assign102420_e154280_d_n7, assign102420_e154280_d_n8, assign102420_e154280_d_n9, assign102420_e154280_d_n10, assign102420_e154280_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102420_e154270: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102420_e154272: f64 = (assign102420_e154270 * locals.var_beta_inv);
        let assign102420_e154274: f64 = (assign102420_e154272 * locals.var_t10);
        let assign102420_e154276: f64 = (assign102420_e154274 * locals.var_t10);
        let assign102420_e154278: f64 = (assign102420_e154276 / locals.var_gds0_ign);
        (assign102420_e154278, ((((((((assign102420_e154270 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn0)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn2)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn4)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn5)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn6)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn7)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn8)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn9)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn10)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102420_e154270 * locals.var_beta_inv_dn13) * locals.var_t10) + (assign102420_e154272 * locals.var_t10_dn13)) * locals.var_t10) + (assign102420_e154274 * locals.var_t10_dn13)) * locals.var_gds0_ign) - (assign102420_e154276 * locals.var_gds0_ign_dn13)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn13,)
    }
};
        locals.var_nign0 = assign102420_e154280;
        locals.var_nign0_dn0 = assign102420_e154280_d_n0;
        locals.var_nign0_dn2 = assign102420_e154280_d_n2;
        locals.var_nign0_dn4 = assign102420_e154280_d_n4;
        locals.var_nign0_dn5 = assign102420_e154280_d_n5;
        locals.var_nign0_dn6 = assign102420_e154280_d_n6;
        locals.var_nign0_dn7 = assign102420_e154280_d_n7;
        locals.var_nign0_dn8 = assign102420_e154280_d_n8;
        locals.var_nign0_dn9 = assign102420_e154280_d_n9;
        locals.var_nign0_dn10 = assign102420_e154280_d_n10;
        locals.var_nign0_dn13 = assign102420_e154280_d_n13;

        let assign102430_e154284: f64 = (10.0 * 2.220446049250313e-16);
        let assign102430_e154289: f64 = (10.0 * 2.220446049250313e-16);
        let assign102430_e154291: f64 = if ((locals.var_kusai00l > assign102430_e154284) && (locals.var_vds > assign102430_e154289)) { 1.0 } else { 0.0 };
        locals.var_guard2335 = assign102430_e154291;

        let (assign102440_e154299, assign102440_e154299_d_n0, assign102440_e154299_d_n2, assign102440_e154299_d_n4, assign102440_e154299_d_n5, assign102440_e154299_d_n6, assign102440_e154299_d_n7, assign102440_e154299_d_n8, assign102440_e154299_d_n9, assign102440_e154299_d_n10, assign102440_e154299_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102440_e154297: f64 = (locals.var_muun / locals.var_mu);
        (assign102440_e154297, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn13 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn13)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn13,)
    }
};
        locals.var_mumoda = assign102440_e154299;
        locals.var_mumoda_dn0 = assign102440_e154299_d_n0;
        locals.var_mumoda_dn2 = assign102440_e154299_d_n2;
        locals.var_mumoda_dn4 = assign102440_e154299_d_n4;
        locals.var_mumoda_dn5 = assign102440_e154299_d_n5;
        locals.var_mumoda_dn6 = assign102440_e154299_d_n6;
        locals.var_mumoda_dn7 = assign102440_e154299_d_n7;
        locals.var_mumoda_dn8 = assign102440_e154299_d_n8;
        locals.var_mumoda_dn9 = assign102440_e154299_d_n9;
        locals.var_mumoda_dn10 = assign102440_e154299_d_n10;
        locals.var_mumoda_dn13 = assign102440_e154299_d_n13;

        let (assign102450_e154311, assign102450_e154311_d_n0, assign102450_e154311_d_n2, assign102450_e154311_d_n4, assign102450_e154311_d_n5, assign102450_e154311_d_n6, assign102450_e154311_d_n7, assign102450_e154311_d_n8, assign102450_e154311_d_n9, assign102450_e154311_d_n10, assign102450_e154311_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102450_e154305: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102450_e154307: f64 = (assign102450_e154305 - locals.var_mumoda);
        let assign102450_e154309: f64 = (assign102450_e154307 / locals.var_vds);
        (assign102450_e154309, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn13 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn13)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn13) * locals.var_vds) - (assign102450_e154307 * locals.var_vds_dn13)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn13,)
    }
};
        locals.var_mumodb = assign102450_e154311;
        locals.var_mumodb_dn0 = assign102450_e154311_d_n0;
        locals.var_mumodb_dn2 = assign102450_e154311_d_n2;
        locals.var_mumodb_dn4 = assign102450_e154311_d_n4;
        locals.var_mumodb_dn5 = assign102450_e154311_d_n5;
        locals.var_mumodb_dn6 = assign102450_e154311_d_n6;
        locals.var_mumodb_dn7 = assign102450_e154311_d_n7;
        locals.var_mumodb_dn8 = assign102450_e154311_d_n8;
        locals.var_mumodb_dn9 = assign102450_e154311_d_n9;
        locals.var_mumodb_dn10 = assign102450_e154311_d_n10;
        locals.var_mumodb_dn13 = assign102450_e154311_d_n13;

        let (assign102460_e154333, assign102460_e154333_d_n0, assign102460_e154333_d_n2, assign102460_e154333_d_n4, assign102460_e154333_d_n5, assign102460_e154333_d_n6, assign102460_e154333_d_n7, assign102460_e154333_d_n8, assign102460_e154333_d_n9, assign102460_e154333_d_n10, assign102460_e154333_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 != 0.0)) {
        let assign102460_e154318: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign102460_e154322: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign102460_e154323: f64 = (locals.var_kusai00 + assign102460_e154322);
        let assign102460_e154325: f64 = (assign102460_e154323 + locals.var_kusail);
        let assign102460_e154326: f64 = (assign102460_e154318 * assign102460_e154325);
        let assign102460_e154329: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign102460_e154330: f64 = (assign102460_e154326 / assign102460_e154329);
        let assign102460_e154331: f64 = (locals.var_mumoda + assign102460_e154330);
        (assign102460_e154331, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn9 + ((((((0.6666666666666667 * locals.var_mumodb_dn9) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn9 + ((locals.var_vgvt_dn9 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign102460_e154329 * assign102460_e154329))), (locals.var_mumoda_dn13 + ((((((0.6666666666666667 * locals.var_mumodb_dn13) * assign102460_e154325) + (assign102460_e154318 * ((locals.var_kusai00_dn13 + ((locals.var_vgvt_dn13 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn13))) + locals.var_kusail_dn13))) * assign102460_e154329) - (assign102460_e154326 * (locals.var_vgvt_dn13 + locals.var_sqrtkusail_dn13))) / (assign102460_e154329 * assign102460_e154329))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn13,)
    }
};
        locals.var_correct_w1 = assign102460_e154333;
        locals.var_correct_w1_dn0 = assign102460_e154333_d_n0;
        locals.var_correct_w1_dn2 = assign102460_e154333_d_n2;
        locals.var_correct_w1_dn4 = assign102460_e154333_d_n4;
        locals.var_correct_w1_dn5 = assign102460_e154333_d_n5;
        locals.var_correct_w1_dn6 = assign102460_e154333_d_n6;
        locals.var_correct_w1_dn7 = assign102460_e154333_d_n7;
        locals.var_correct_w1_dn8 = assign102460_e154333_d_n8;
        locals.var_correct_w1_dn9 = assign102460_e154333_d_n9;
        locals.var_correct_w1_dn10 = assign102460_e154333_d_n10;
        locals.var_correct_w1_dn13 = assign102460_e154333_d_n13;

        let (assign102470_e154342, assign102470_e154342_d_n0, assign102470_e154342_d_n2, assign102470_e154342_d_n4, assign102470_e154342_d_n5, assign102470_e154342_d_n6, assign102470_e154342_d_n7, assign102470_e154342_d_n8, assign102470_e154342_d_n9, assign102470_e154342_d_n10, assign102470_e154342_d_n13,) = {
    if ((locals.var_guard2334 != 0.0) && (locals.var_guard2335 == 0.0)) {
        let assign102470_e154340: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign102470_e154340, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn13 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn13)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn13,)
    }
};
        locals.var_correct_w1 = assign102470_e154342;
        locals.var_correct_w1_dn0 = assign102470_e154342_d_n0;
        locals.var_correct_w1_dn2 = assign102470_e154342_d_n2;
        locals.var_correct_w1_dn4 = assign102470_e154342_d_n4;
        locals.var_correct_w1_dn5 = assign102470_e154342_d_n5;
        locals.var_correct_w1_dn6 = assign102470_e154342_d_n6;
        locals.var_correct_w1_dn7 = assign102470_e154342_d_n7;
        locals.var_correct_w1_dn8 = assign102470_e154342_d_n8;
        locals.var_correct_w1_dn9 = assign102470_e154342_d_n9;
        locals.var_correct_w1_dn10 = assign102470_e154342_d_n10;
        locals.var_correct_w1_dn13 = assign102470_e154342_d_n13;

        let (assign102480_e154352, assign102480_e154352_d_n0, assign102480_e154352_d_n2, assign102480_e154352_d_n4, assign102480_e154352_d_n5, assign102480_e154352_d_n6, assign102480_e154352_d_n7, assign102480_e154352_d_n8, assign102480_e154352_d_n9, assign102480_e154352_d_n10, assign102480_e154352_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102480_e154346: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign102480_e154348: f64 = (assign102480_e154346 * locals.var_kusai_ig);
        let assign102480_e154350: f64 = (assign102480_e154348 * locals.var_correct_w1);
        (assign102480_e154350, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn9) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn9)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn9)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) + (assign102480_e154346 * locals.var_kusai_ig_dn13)) * locals.var_correct_w1) + (assign102480_e154348 * locals.var_correct_w1_dn13)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102480_e154352;
        locals.var_noiigate_dn0 = assign102480_e154352_d_n0;
        locals.var_noiigate_dn2 = assign102480_e154352_d_n2;
        locals.var_noiigate_dn4 = assign102480_e154352_d_n4;
        locals.var_noiigate_dn5 = assign102480_e154352_d_n5;
        locals.var_noiigate_dn6 = assign102480_e154352_d_n6;
        locals.var_noiigate_dn7 = assign102480_e154352_d_n7;
        locals.var_noiigate_dn8 = assign102480_e154352_d_n8;
        locals.var_noiigate_dn9 = assign102480_e154352_d_n9;
        locals.var_noiigate_dn10 = assign102480_e154352_d_n10;
        locals.var_noiigate_dn13 = assign102480_e154352_d_n13;

        let (assign102490_e154356, assign102490_e154356_d_n0, assign102490_e154356_d_n2, assign102490_e154356_d_n4, assign102490_e154356_d_n5, assign102490_e154356_d_n6, assign102490_e154356_d_n7, assign102490_e154356_d_n8, assign102490_e154356_d_n9, assign102490_e154356_d_n10, assign102490_e154356_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn13,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn13,)
    }
};
        locals.var_noicross = assign102490_e154356;
        locals.var_noicross_dn0 = assign102490_e154356_d_n0;
        locals.var_noicross_dn2 = assign102490_e154356_d_n2;
        locals.var_noicross_dn4 = assign102490_e154356_d_n4;
        locals.var_noicross_dn5 = assign102490_e154356_d_n5;
        locals.var_noicross_dn6 = assign102490_e154356_d_n6;
        locals.var_noicross_dn7 = assign102490_e154356_d_n7;
        locals.var_noicross_dn8 = assign102490_e154356_d_n8;
        locals.var_noicross_dn9 = assign102490_e154356_d_n9;
        locals.var_noicross_dn10 = assign102490_e154356_d_n10;
        locals.var_noicross_dn13 = assign102490_e154356_d_n13;

    }

    pub(super) fn stamp_transient_block_363(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign102500_e154365, assign102500_e154365_d_n0, assign102500_e154365_d_n2, assign102500_e154365_d_n4, assign102500_e154365_d_n5, assign102500_e154365_d_n6, assign102500_e154365_d_n7, assign102500_e154365_d_n8, assign102500_e154365_d_n9, assign102500_e154365_d_n10, assign102500_e154365_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let (assign102500_e154363, assign102500_e154363_d_n0, assign102500_e154363_d_n2, assign102500_e154363_d_n4, assign102500_e154363_d_n5, assign102500_e154363_d_n6, assign102500_e154363_d_n7, assign102500_e154363_d_n8, assign102500_e154363_d_n9, assign102500_e154363_d_n10, assign102500_e154363_d_n13,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
            }
        };
        (assign102500_e154363, assign102500_e154363_d_n0, assign102500_e154363_d_n2, assign102500_e154363_d_n4, assign102500_e154363_d_n5, assign102500_e154363_d_n6, assign102500_e154363_d_n7, assign102500_e154363_d_n8, assign102500_e154363_d_n9, assign102500_e154363_d_n10, assign102500_e154363_d_n13,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102500_e154365;
        locals.var_noiigate_dn0 = assign102500_e154365_d_n0;
        locals.var_noiigate_dn2 = assign102500_e154365_d_n2;
        locals.var_noiigate_dn4 = assign102500_e154365_d_n4;
        locals.var_noiigate_dn5 = assign102500_e154365_d_n5;
        locals.var_noiigate_dn6 = assign102500_e154365_d_n6;
        locals.var_noiigate_dn7 = assign102500_e154365_d_n7;
        locals.var_noiigate_dn8 = assign102500_e154365_d_n8;
        locals.var_noiigate_dn9 = assign102500_e154365_d_n9;
        locals.var_noiigate_dn10 = assign102500_e154365_d_n10;
        locals.var_noiigate_dn13 = assign102500_e154365_d_n13;

        let (assign102510_e154375, assign102510_e154375_d_n0, assign102510_e154375_d_n2, assign102510_e154375_d_n4, assign102510_e154375_d_n5, assign102510_e154375_d_n6, assign102510_e154375_d_n7, assign102510_e154375_d_n8, assign102510_e154375_d_n9, assign102510_e154375_d_n10, assign102510_e154375_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102510_e154368: f64 = (-locals.var_t10);
        let (assign102510_e154373, assign102510_e154373_d_n0, assign102510_e154373_d_n2, assign102510_e154373_d_n4, assign102510_e154373_d_n5, assign102510_e154373_d_n6, assign102510_e154373_d_n7, assign102510_e154373_d_n8, assign102510_e154373_d_n9, assign102510_e154373_d_n10, assign102510_e154373_d_n13,) = {
            if (assign102510_e154368 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102510_e154373, assign102510_e154373_d_n0, assign102510_e154373_d_n2, assign102510_e154373_d_n4, assign102510_e154373_d_n5, assign102510_e154373_d_n6, assign102510_e154373_d_n7, assign102510_e154373_d_n8, assign102510_e154373_d_n9, assign102510_e154373_d_n10, assign102510_e154373_d_n13,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn13,)
    }
};
        locals.var_noiigate = assign102510_e154375;
        locals.var_noiigate_dn0 = assign102510_e154375_d_n0;
        locals.var_noiigate_dn2 = assign102510_e154375_d_n2;
        locals.var_noiigate_dn4 = assign102510_e154375_d_n4;
        locals.var_noiigate_dn5 = assign102510_e154375_d_n5;
        locals.var_noiigate_dn6 = assign102510_e154375_d_n6;
        locals.var_noiigate_dn7 = assign102510_e154375_d_n7;
        locals.var_noiigate_dn8 = assign102510_e154375_d_n8;
        locals.var_noiigate_dn9 = assign102510_e154375_d_n9;
        locals.var_noiigate_dn10 = assign102510_e154375_d_n10;
        locals.var_noiigate_dn13 = assign102510_e154375_d_n13;

        let (assign102520_e154385, assign102520_e154385_d_n0, assign102520_e154385_d_n2, assign102520_e154385_d_n4, assign102520_e154385_d_n5, assign102520_e154385_d_n6, assign102520_e154385_d_n7, assign102520_e154385_d_n8, assign102520_e154385_d_n9, assign102520_e154385_d_n10, assign102520_e154385_d_n13,) = {
    if (locals.var_guard2334 != 0.0) {
        let assign102520_e154378: f64 = (-locals.var_t10);
        let (assign102520_e154383, assign102520_e154383_d_n0, assign102520_e154383_d_n2, assign102520_e154383_d_n4, assign102520_e154383_d_n5, assign102520_e154383_d_n6, assign102520_e154383_d_n7, assign102520_e154383_d_n8, assign102520_e154383_d_n9, assign102520_e154383_d_n10, assign102520_e154383_d_n13,) = {
            if (assign102520_e154378 > locals.var_t0) {
                (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102520_e154383, assign102520_e154383_d_n0, assign102520_e154383_d_n2, assign102520_e154383_d_n4, assign102520_e154383_d_n5, assign102520_e154383_d_n6, assign102520_e154383_d_n7, assign102520_e154383_d_n8, assign102520_e154383_d_n9, assign102520_e154383_d_n10, assign102520_e154383_d_n13,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn13,)
    }
};
        locals.var_noicross = assign102520_e154385;
        locals.var_noicross_dn0 = assign102520_e154385_d_n0;
        locals.var_noicross_dn2 = assign102520_e154385_d_n2;
        locals.var_noicross_dn4 = assign102520_e154385_d_n4;
        locals.var_noicross_dn5 = assign102520_e154385_d_n5;
        locals.var_noicross_dn6 = assign102520_e154385_d_n6;
        locals.var_noicross_dn7 = assign102520_e154385_d_n7;
        locals.var_noicross_dn8 = assign102520_e154385_d_n8;
        locals.var_noicross_dn9 = assign102520_e154385_d_n9;
        locals.var_noicross_dn10 = assign102520_e154385_d_n10;
        locals.var_noicross_dn13 = assign102520_e154385_d_n13;

        let assign102530_e154388: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign102530_e154388;
        locals.var_sid_dn0 = ((locals.var_whi_noise_dn0 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn0));
        locals.var_sid_dn2 = ((locals.var_whi_noise_dn2 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn2));
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = ((locals.var_whi_noise_dn5 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn5));
        locals.var_sid_dn6 = ((locals.var_whi_noise_dn6 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn6));
        locals.var_sid_dn7 = ((locals.var_whi_noise_dn7 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn7));
        locals.var_sid_dn8 = ((locals.var_whi_noise_dn8 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn8));
        locals.var_sid_dn9 = ((locals.var_whi_noise_dn9 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn9));
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn13 = ((locals.var_whi_noise_dn13 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn13));

        locals.var_ci = locals.var_noicross;
        locals.var_ci_dn0 = locals.var_noicross_dn0;
        locals.var_ci_dn2 = locals.var_noicross_dn2;
        locals.var_ci_dn4 = locals.var_noicross_dn4;
        locals.var_ci_dn5 = locals.var_noicross_dn5;
        locals.var_ci_dn6 = locals.var_noicross_dn6;
        locals.var_ci_dn7 = locals.var_noicross_dn7;
        locals.var_ci_dn8 = locals.var_noicross_dn8;
        locals.var_ci_dn9 = locals.var_noicross_dn9;
        locals.var_ci_dn10 = locals.var_noicross_dn10;
        locals.var_ci_dn13 = locals.var_noicross_dn13;

        let (assign102550_e154402, assign102550_e154402_d_n0, assign102550_e154402_d_n2, assign102550_e154402_d_n4, assign102550_e154402_d_n5, assign102550_e154402_d_n6, assign102550_e154402_d_n7, assign102550_e154402_d_n8, assign102550_e154402_d_n9, assign102550_e154402_d_n10, assign102550_e154402_d_n13,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign102550_e154399: f64 = (locals.var_noiigate / locals.var_sid);
        let assign102550_e154400: f64 = (assign102550_e154399).sqrt();
        (assign102550_e154400, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn9 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn9)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)), ((((locals.var_noiigate_dn13 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn13)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102550_e154400)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign102550_e154402;
        locals.var_sigrat_dn0 = assign102550_e154402_d_n0;
        locals.var_sigrat_dn2 = assign102550_e154402_d_n2;
        locals.var_sigrat_dn4 = assign102550_e154402_d_n4;
        locals.var_sigrat_dn5 = assign102550_e154402_d_n5;
        locals.var_sigrat_dn6 = assign102550_e154402_d_n6;
        locals.var_sigrat_dn7 = assign102550_e154402_d_n7;
        locals.var_sigrat_dn8 = assign102550_e154402_d_n8;
        locals.var_sigrat_dn9 = assign102550_e154402_d_n9;
        locals.var_sigrat_dn10 = assign102550_e154402_d_n10;
        locals.var_sigrat_dn13 = assign102550_e154402_d_n13;

        let (assign102560_e154414, assign102560_e154414_d_n0, assign102560_e154414_d_n2, assign102560_e154414_d_n4, assign102560_e154414_d_n5, assign102560_e154414_d_n6, assign102560_e154414_d_n7, assign102560_e154414_d_n8, assign102560_e154414_d_n9, assign102560_e154414_d_n10, assign102560_e154414_d_n13,) = {
    if (locals.var_mode > 0.0) {
        let assign102560_e154409: f64 = (1.0 - locals.var_qdrat);
        let assign102560_e154410: f64 = (locals.var_sigrat * assign102560_e154409);
        (assign102560_e154410, ((locals.var_sigrat_dn0 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn13 * assign102560_e154409) + (locals.var_sigrat * (-locals.var_qdrat_dn13))),)
    } else {
        let assign102560_e154413: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102560_e154413, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn13 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn13)),)
    }
};
        locals.var_sigrat_s = assign102560_e154414;
        locals.var_sigrat_s_dn0 = assign102560_e154414_d_n0;
        locals.var_sigrat_s_dn2 = assign102560_e154414_d_n2;
        locals.var_sigrat_s_dn4 = assign102560_e154414_d_n4;
        locals.var_sigrat_s_dn5 = assign102560_e154414_d_n5;
        locals.var_sigrat_s_dn6 = assign102560_e154414_d_n6;
        locals.var_sigrat_s_dn7 = assign102560_e154414_d_n7;
        locals.var_sigrat_s_dn8 = assign102560_e154414_d_n8;
        locals.var_sigrat_s_dn9 = assign102560_e154414_d_n9;
        locals.var_sigrat_s_dn10 = assign102560_e154414_d_n10;
        locals.var_sigrat_s_dn13 = assign102560_e154414_d_n13;

        let (assign102570_e154426, assign102570_e154426_d_n0, assign102570_e154426_d_n2, assign102570_e154426_d_n4, assign102570_e154426_d_n5, assign102570_e154426_d_n6, assign102570_e154426_d_n7, assign102570_e154426_d_n8, assign102570_e154426_d_n9, assign102570_e154426_d_n10, assign102570_e154426_d_n13,) = {
    if (locals.var_mode > 0.0) {
        let assign102570_e154420: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102570_e154420, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn13 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn13)),)
    } else {
        let assign102570_e154424: f64 = (1.0 - locals.var_qdrat);
        let assign102570_e154425: f64 = (locals.var_sigrat * assign102570_e154424);
        (assign102570_e154425, ((locals.var_sigrat_dn0 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn13 * assign102570_e154424) + (locals.var_sigrat * (-locals.var_qdrat_dn13))),)
    }
};
        locals.var_sigrat_d = assign102570_e154426;
        locals.var_sigrat_d_dn0 = assign102570_e154426_d_n0;
        locals.var_sigrat_d_dn2 = assign102570_e154426_d_n2;
        locals.var_sigrat_d_dn4 = assign102570_e154426_d_n4;
        locals.var_sigrat_d_dn5 = assign102570_e154426_d_n5;
        locals.var_sigrat_d_dn6 = assign102570_e154426_d_n6;
        locals.var_sigrat_d_dn7 = assign102570_e154426_d_n7;
        locals.var_sigrat_d_dn8 = assign102570_e154426_d_n8;
        locals.var_sigrat_d_dn9 = assign102570_e154426_d_n9;
        locals.var_sigrat_d_dn10 = assign102570_e154426_d_n10;
        locals.var_sigrat_d_dn13 = assign102570_e154426_d_n13;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn4 = 0.0;
        locals.var_rsde_dn5 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn8 = 0.0;
        locals.var_rsde_dn9 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn13 = 0.0;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn4 = 0.0;
        locals.var_rdde_dn5 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn8 = 0.0;
        locals.var_rdde_dn9 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn13 = 0.0;

        let assign102600_e154431: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2336 = assign102600_e154431;

        let assign102610_e154434: f64 = if locals.var_flg_rs == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2337 = assign102610_e154434;

        let assign102620_e154441: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102620_e154441;

        let (assign102630_e154457, assign102630_e154457_d_n0, assign102630_e154457_d_n2, assign102630_e154457_d_n4, assign102630_e154457_d_n5, assign102630_e154457_d_n6, assign102630_e154457_d_n7, assign102630_e154457_d_n8, assign102630_e154457_d_n9, assign102630_e154457_d_n10, assign102630_e154457_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let (assign102630_e154455, assign102630_e154455_d_n0, assign102630_e154455_d_n2, assign102630_e154455_d_n4, assign102630_e154455_d_n5, assign102630_e154455_d_n6, assign102630_e154455_d_n7, assign102630_e154455_d_n8, assign102630_e154455_d_n9, assign102630_e154455_d_n10, assign102630_e154455_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102630_e154454: f64 = (locals.var_tratio).powf(p.p416);
                (assign102630_e154454, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn13)) } } else { (assign102630_e154454 * (p.p416 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign102630_e154455, assign102630_e154455_d_n0, assign102630_e154455_d_n2, assign102630_e154455_d_n4, assign102630_e154455_d_n5, assign102630_e154455_d_n6, assign102630_e154455_d_n7, assign102630_e154455_d_n8, assign102630_e154455_d_n9, assign102630_e154455_d_n10, assign102630_e154455_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102630_e154457;
        locals.var_t1_dn0 = assign102630_e154457_d_n0;
        locals.var_t1_dn2 = assign102630_e154457_d_n2;
        locals.var_t1_dn4 = assign102630_e154457_d_n4;
        locals.var_t1_dn5 = assign102630_e154457_d_n5;
        locals.var_t1_dn6 = assign102630_e154457_d_n6;
        locals.var_t1_dn7 = assign102630_e154457_d_n7;
        locals.var_t1_dn8 = assign102630_e154457_d_n8;
        locals.var_t1_dn9 = assign102630_e154457_d_n9;
        locals.var_t1_dn10 = assign102630_e154457_d_n10;
        locals.var_t1_dn13 = assign102630_e154457_d_n13;

        let (assign102640_e154468, assign102640_e154468_d_n0, assign102640_e154468_d_n2, assign102640_e154468_d_n4, assign102640_e154468_d_n5, assign102640_e154468_d_n6, assign102640_e154468_d_n7, assign102640_e154468_d_n8, assign102640_e154468_d_n9, assign102640_e154468_d_n10, assign102640_e154468_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102640_e154466: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign102640_e154466, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn13,)
    }
};
        locals.var_rrdrmues = assign102640_e154468;
        locals.var_rrdrmues_dn0 = assign102640_e154468_d_n0;
        locals.var_rrdrmues_dn2 = assign102640_e154468_d_n2;
        locals.var_rrdrmues_dn4 = assign102640_e154468_d_n4;
        locals.var_rrdrmues_dn5 = assign102640_e154468_d_n5;
        locals.var_rrdrmues_dn6 = assign102640_e154468_d_n6;
        locals.var_rrdrmues_dn7 = assign102640_e154468_d_n7;
        locals.var_rrdrmues_dn8 = assign102640_e154468_d_n8;
        locals.var_rrdrmues_dn9 = assign102640_e154468_d_n9;
        locals.var_rrdrmues_dn10 = assign102640_e154468_d_n10;
        locals.var_rrdrmues_dn13 = assign102640_e154468_d_n13;

        let (assign102650_e154493, assign102650_e154493_d_n0, assign102650_e154493_d_n2, assign102650_e154493_d_n4, assign102650_e154493_d_n5, assign102650_e154493_d_n6, assign102650_e154493_d_n7, assign102650_e154493_d_n8, assign102650_e154493_d_n9, assign102650_e154493_d_n10, assign102650_e154493_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102650_e154478: f64 = (0.4 * locals.var_tratio);
        let assign102650_e154479: f64 = (1.8 + assign102650_e154478);
        let assign102650_e154482: f64 = (0.1 * locals.var_tratio);
        let assign102650_e154484: f64 = (assign102650_e154482 * locals.var_tratio);
        let assign102650_e154485: f64 = (assign102650_e154479 + assign102650_e154484);
        let assign102650_e154489: f64 = (1.0 - locals.var_tratio);
        let assign102650_e154490: f64 = (p.p418 * assign102650_e154489);
        let assign102650_e154491: f64 = (assign102650_e154485 - assign102650_e154490);
        (assign102650_e154491, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign102650_e154482 * locals.var_tratio_dn13))) - (p.p418 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign102650_e154493;
        locals.var_t0_dn0 = assign102650_e154493_d_n0;
        locals.var_t0_dn2 = assign102650_e154493_d_n2;
        locals.var_t0_dn4 = assign102650_e154493_d_n4;
        locals.var_t0_dn5 = assign102650_e154493_d_n5;
        locals.var_t0_dn6 = assign102650_e154493_d_n6;
        locals.var_t0_dn7 = assign102650_e154493_d_n7;
        locals.var_t0_dn8 = assign102650_e154493_d_n8;
        locals.var_t0_dn9 = assign102650_e154493_d_n9;
        locals.var_t0_dn10 = assign102650_e154493_d_n10;
        locals.var_t0_dn13 = assign102650_e154493_d_n13;

        let (assign102660_e154504, assign102660_e154504_d_n0, assign102660_e154504_d_n2, assign102660_e154504_d_n4, assign102660_e154504_d_n5, assign102660_e154504_d_n6, assign102660_e154504_d_n7, assign102660_e154504_d_n8, assign102660_e154504_d_n9, assign102660_e154504_d_n10, assign102660_e154504_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102660_e154502: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign102660_e154502, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn13,)
    }
};
        locals.var_rrdrvmaxs = assign102660_e154504;
        locals.var_rrdrvmaxs_dn0 = assign102660_e154504_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign102660_e154504_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign102660_e154504_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign102660_e154504_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign102660_e154504_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign102660_e154504_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign102660_e154504_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign102660_e154504_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign102660_e154504_d_n10;
        locals.var_rrdrvmaxs_dn13 = assign102660_e154504_d_n13;

        let (assign102670_e154519, assign102670_e154519_d_n0, assign102670_e154519_d_n2, assign102670_e154519_d_n4, assign102670_e154519_d_n5, assign102670_e154519_d_n6, assign102670_e154519_d_n7, assign102670_e154519_d_n8, assign102670_e154519_d_n9, assign102670_e154519_d_n10, assign102670_e154519_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 != 0.0)) {
        let assign102670_e154515: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign102670_e154516: f64 = (p.p439 * assign102670_e154515);
        let assign102670_e154517: f64 = (locals.var_uc_rdrbb_s + assign102670_e154516);
        (assign102670_e154517, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn13 + (p.p439 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn13,)
    }
};
        locals.var_uc_rdrbb_s = assign102670_e154519;
        locals.var_uc_rdrbb_s_dn0 = assign102670_e154519_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign102670_e154519_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign102670_e154519_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign102670_e154519_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign102670_e154519_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign102670_e154519_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign102670_e154519_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign102670_e154519_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign102670_e154519_d_n10;
        locals.var_uc_rdrbb_s_dn13 = assign102670_e154519_d_n13;

        let (assign102680_e154531, assign102680_e154531_d_n0, assign102680_e154531_d_n2, assign102680_e154531_d_n4, assign102680_e154531_d_n5, assign102680_e154531_d_n6, assign102680_e154531_d_n7, assign102680_e154531_d_n8, assign102680_e154531_d_n9, assign102680_e154531_d_n10, assign102680_e154531_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2338 == 0.0)) {
        let assign102680_e154527: f64 = ctx_temp;
        let assign102680_e154529: f64 = (assign102680_e154527 + p.p11);
        (assign102680_e154529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign102680_e154531;
        locals.var_ttemp_dn0 = assign102680_e154531_d_n0;
        locals.var_ttemp_dn2 = assign102680_e154531_d_n2;
        locals.var_ttemp_dn4 = assign102680_e154531_d_n4;
        locals.var_ttemp_dn5 = assign102680_e154531_d_n5;
        locals.var_ttemp_dn6 = assign102680_e154531_d_n6;
        locals.var_ttemp_dn7 = assign102680_e154531_d_n7;
        locals.var_ttemp_dn8 = assign102680_e154531_d_n8;
        locals.var_ttemp_dn9 = assign102680_e154531_d_n9;
        locals.var_ttemp_dn10 = assign102680_e154531_d_n10;
        locals.var_ttemp_dn13 = assign102680_e154531_d_n13;

        let (assign102690_e154540,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102690_e154538: f64 = (locals.var_weff_ld * p.p7);
        (assign102690_e154538,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign102690_e154540;

        let (assign102700_e154547,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (p.p71,)
    } else {
        (locals.var_ldrifte_s,)
    }
};
        locals.var_ldrifte_s = assign102700_e154547;

        let (assign102710_e154554,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_novers,)
    }
};
        locals.var_novers = assign102710_e154554;

        let (assign102720_e154563, assign102720_e154563_d_n0, assign102720_e154563_d_n2, assign102720_e154563_d_n4, assign102720_e154563_d_n5, assign102720_e154563_d_n6, assign102720_e154563_d_n7, assign102720_e154563_d_n8, assign102720_e154563_d_n9, assign102720_e154563_d_n10, assign102720_e154563_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102720_e154561: f64 = (locals.var_rrdrmues * locals.var_rdrmuele);
        (assign102720_e154561, (locals.var_rrdrmues_dn0 * locals.var_rdrmuele), (locals.var_rrdrmues_dn2 * locals.var_rdrmuele), (locals.var_rrdrmues_dn4 * locals.var_rdrmuele), (locals.var_rrdrmues_dn5 * locals.var_rdrmuele), (locals.var_rrdrmues_dn6 * locals.var_rdrmuele), (locals.var_rrdrmues_dn7 * locals.var_rdrmuele), (locals.var_rrdrmues_dn8 * locals.var_rdrmuele), (locals.var_rrdrmues_dn9 * locals.var_rdrmuele), (locals.var_rrdrmues_dn10 * locals.var_rdrmuele), (locals.var_rrdrmues_dn13 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0_s, locals.var_mu0_s_dn0, locals.var_mu0_s_dn2, locals.var_mu0_s_dn4, locals.var_mu0_s_dn5, locals.var_mu0_s_dn6, locals.var_mu0_s_dn7, locals.var_mu0_s_dn8, locals.var_mu0_s_dn9, locals.var_mu0_s_dn10, locals.var_mu0_s_dn13,)
    }
};
        locals.var_mu0_s = assign102720_e154563;
        locals.var_mu0_s_dn0 = assign102720_e154563_d_n0;
        locals.var_mu0_s_dn2 = assign102720_e154563_d_n2;
        locals.var_mu0_s_dn4 = assign102720_e154563_d_n4;
        locals.var_mu0_s_dn5 = assign102720_e154563_d_n5;
        locals.var_mu0_s_dn6 = assign102720_e154563_d_n6;
        locals.var_mu0_s_dn7 = assign102720_e154563_d_n7;
        locals.var_mu0_s_dn8 = assign102720_e154563_d_n8;
        locals.var_mu0_s_dn9 = assign102720_e154563_d_n9;
        locals.var_mu0_s_dn10 = assign102720_e154563_d_n10;
        locals.var_mu0_s_dn13 = assign102720_e154563_d_n13;

        let (assign102730_e154576, assign102730_e154576_d_n0, assign102730_e154576_d_n2, assign102730_e154576_d_n4, assign102730_e154576_d_n5, assign102730_e154576_d_n6, assign102730_e154576_d_n7, assign102730_e154576_d_n8, assign102730_e154576_d_n9, assign102730_e154576_d_n10, assign102730_e154576_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102730_e154570: f64 = (locals.var_rrdrvmaxs * locals.var_rdrvmaxwe);
        let assign102730_e154572: f64 = (assign102730_e154570 * locals.var_rdrvmaxle);
        let assign102730_e154574: f64 = (assign102730_e154572 + 1e-25);
        (assign102730_e154574, ((locals.var_rrdrvmaxs_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn13 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe_s, locals.var_vmaxe_s_dn0, locals.var_vmaxe_s_dn2, locals.var_vmaxe_s_dn4, locals.var_vmaxe_s_dn5, locals.var_vmaxe_s_dn6, locals.var_vmaxe_s_dn7, locals.var_vmaxe_s_dn8, locals.var_vmaxe_s_dn9, locals.var_vmaxe_s_dn10, locals.var_vmaxe_s_dn13,)
    }
};
        locals.var_vmaxe_s = assign102730_e154576;
        locals.var_vmaxe_s_dn0 = assign102730_e154576_d_n0;
        locals.var_vmaxe_s_dn2 = assign102730_e154576_d_n2;
        locals.var_vmaxe_s_dn4 = assign102730_e154576_d_n4;
        locals.var_vmaxe_s_dn5 = assign102730_e154576_d_n5;
        locals.var_vmaxe_s_dn6 = assign102730_e154576_d_n6;
        locals.var_vmaxe_s_dn7 = assign102730_e154576_d_n7;
        locals.var_vmaxe_s_dn8 = assign102730_e154576_d_n8;
        locals.var_vmaxe_s_dn9 = assign102730_e154576_d_n9;
        locals.var_vmaxe_s_dn10 = assign102730_e154576_d_n10;
        locals.var_vmaxe_s_dn13 = assign102730_e154576_d_n13;

        let (assign102740_e154585, assign102740_e154585_d_n2, assign102740_e154585_d_n7,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102740_e154583: f64 = (locals.var_vsps / locals.var_ldrifte_s);
        (assign102740_e154583, (locals.var_vsps_dn2 / locals.var_ldrifte_s), (locals.var_vsps_dn7 / locals.var_ldrifte_s),)
    } else {
        (locals.var_edri_s, locals.var_edri_s_dn2, locals.var_edri_s_dn7,)
    }
};
        locals.var_edri_s = assign102740_e154585;
        locals.var_edri_s_dn2 = assign102740_e154585_d_n2;
        locals.var_edri_s_dn7 = assign102740_e154585_d_n7;

        let (assign102750_e154594, assign102750_e154594_d_n0, assign102750_e154594_d_n2, assign102750_e154594_d_n4, assign102750_e154594_d_n5, assign102750_e154594_d_n6, assign102750_e154594_d_n7, assign102750_e154594_d_n8, assign102750_e154594_d_n9, assign102750_e154594_d_n10, assign102750_e154594_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102750_e154592: f64 = (locals.var_mu0_s * locals.var_edri_s);
        (assign102750_e154592, (locals.var_mu0_s_dn0 * locals.var_edri_s), ((locals.var_mu0_s_dn2 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn2)), (locals.var_mu0_s_dn4 * locals.var_edri_s), (locals.var_mu0_s_dn5 * locals.var_edri_s), (locals.var_mu0_s_dn6 * locals.var_edri_s), ((locals.var_mu0_s_dn7 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn7)), (locals.var_mu0_s_dn8 * locals.var_edri_s), (locals.var_mu0_s_dn9 * locals.var_edri_s), (locals.var_mu0_s_dn10 * locals.var_edri_s), (locals.var_mu0_s_dn13 * locals.var_edri_s),)
    } else {
        (locals.var_vdri_s, locals.var_vdri_s_dn0, locals.var_vdri_s_dn2, locals.var_vdri_s_dn4, locals.var_vdri_s_dn5, locals.var_vdri_s_dn6, locals.var_vdri_s_dn7, locals.var_vdri_s_dn8, locals.var_vdri_s_dn9, locals.var_vdri_s_dn10, locals.var_vdri_s_dn13,)
    }
};
        locals.var_vdri_s = assign102750_e154594;
        locals.var_vdri_s_dn0 = assign102750_e154594_d_n0;
        locals.var_vdri_s_dn2 = assign102750_e154594_d_n2;
        locals.var_vdri_s_dn4 = assign102750_e154594_d_n4;
        locals.var_vdri_s_dn5 = assign102750_e154594_d_n5;
        locals.var_vdri_s_dn6 = assign102750_e154594_d_n6;
        locals.var_vdri_s_dn7 = assign102750_e154594_d_n7;
        locals.var_vdri_s_dn8 = assign102750_e154594_d_n8;
        locals.var_vdri_s_dn9 = assign102750_e154594_d_n9;
        locals.var_vdri_s_dn10 = assign102750_e154594_d_n10;
        locals.var_vdri_s_dn13 = assign102750_e154594_d_n13;

        let assign102760_e154597: f64 = if locals.var_vsps >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102760_e154597;

        let (assign102770_e154608, assign102770_e154608_d_n0, assign102770_e154608_d_n2, assign102770_e154608_d_n4, assign102770_e154608_d_n5, assign102770_e154608_d_n6, assign102770_e154608_d_n7, assign102770_e154608_d_n8, assign102770_e154608_d_n9, assign102770_e154608_d_n10, assign102770_e154608_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2339 != 0.0)) {
        let assign102770_e154606: f64 = (locals.var_vdri_s / locals.var_vmaxe_s);
        (assign102770_e154606, (((locals.var_vdri_s_dn0 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn2 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn4 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn5 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn6 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn7 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn8 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn9 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn10 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn13 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn13)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102770_e154608;
        locals.var_t1_dn0 = assign102770_e154608_d_n0;
        locals.var_t1_dn2 = assign102770_e154608_d_n2;
        locals.var_t1_dn4 = assign102770_e154608_d_n4;
        locals.var_t1_dn5 = assign102770_e154608_d_n5;
        locals.var_t1_dn6 = assign102770_e154608_d_n6;
        locals.var_t1_dn7 = assign102770_e154608_d_n7;
        locals.var_t1_dn8 = assign102770_e154608_d_n8;
        locals.var_t1_dn9 = assign102770_e154608_d_n9;
        locals.var_t1_dn10 = assign102770_e154608_d_n10;
        locals.var_t1_dn13 = assign102770_e154608_d_n13;

        let (assign102780_e154621, assign102780_e154621_d_n0, assign102780_e154621_d_n2, assign102780_e154621_d_n4, assign102780_e154621_d_n5, assign102780_e154621_d_n6, assign102780_e154621_d_n7, assign102780_e154621_d_n8, assign102780_e154621_d_n9, assign102780_e154621_d_n10, assign102780_e154621_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2339 == 0.0)) {
        let assign102780_e154617: f64 = (-locals.var_vdri_s);
        let assign102780_e154619: f64 = (assign102780_e154617 / locals.var_vmaxe_s);
        (assign102780_e154619, ((((-locals.var_vdri_s_dn0) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn2) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn4) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn5) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn6) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn7) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn8) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn9) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn10) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn13) * locals.var_vmaxe_s) - (assign102780_e154617 * locals.var_vmaxe_s_dn13)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102780_e154621;
        locals.var_t1_dn0 = assign102780_e154621_d_n0;
        locals.var_t1_dn2 = assign102780_e154621_d_n2;
        locals.var_t1_dn4 = assign102780_e154621_d_n4;
        locals.var_t1_dn5 = assign102780_e154621_d_n5;
        locals.var_t1_dn6 = assign102780_e154621_d_n6;
        locals.var_t1_dn7 = assign102780_e154621_d_n7;
        locals.var_t1_dn8 = assign102780_e154621_d_n8;
        locals.var_t1_dn9 = assign102780_e154621_d_n9;
        locals.var_t1_dn10 = assign102780_e154621_d_n10;
        locals.var_t1_dn13 = assign102780_e154621_d_n13;

        let assign102790_e154625: f64 = (10.0 * 2.220446049250313e-16);
        let assign102790_e154626: f64 = (1.0 - assign102790_e154625);
        let assign102790_e154633: f64 = (10.0 * 2.220446049250313e-16);
        let assign102790_e154634: f64 = (1.0 + assign102790_e154633);
        let assign102790_e154636: f64 = if ((assign102790_e154626 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102790_e154634)) { 1.0 } else { 0.0 };
        locals.var_guard2340 = assign102790_e154636;

        let (assign102800_e154645, assign102800_e154645_d_n0, assign102800_e154645_d_n2, assign102800_e154645_d_n4, assign102800_e154645_d_n5, assign102800_e154645_d_n6, assign102800_e154645_d_n7, assign102800_e154645_d_n8, assign102800_e154645_d_n9, assign102800_e154645_d_n10, assign102800_e154645_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102800_e154645;
        locals.var_t3_dn0 = assign102800_e154645_d_n0;
        locals.var_t3_dn2 = assign102800_e154645_d_n2;
        locals.var_t3_dn4 = assign102800_e154645_d_n4;
        locals.var_t3_dn5 = assign102800_e154645_d_n5;
        locals.var_t3_dn6 = assign102800_e154645_d_n6;
        locals.var_t3_dn7 = assign102800_e154645_d_n7;
        locals.var_t3_dn8 = assign102800_e154645_d_n8;
        locals.var_t3_dn9 = assign102800_e154645_d_n9;
        locals.var_t3_dn10 = assign102800_e154645_d_n10;
        locals.var_t3_dn13 = assign102800_e154645_d_n13;

    }

    pub(super) fn stamp_transient_block_364(
        locals: &mut StampLocals,
    ) {
        let assign102810_e154649: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154650: f64 = (2.0 - assign102810_e154649);
        let assign102810_e154657: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154658: f64 = (2.0 + assign102810_e154657);
        let assign102810_e154660: f64 = if ((assign102810_e154650 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102810_e154658)) { 1.0 } else { 0.0 };
        locals.var_guard2341 = assign102810_e154660;

        let (assign102820_e154672, assign102820_e154672_d_n0, assign102820_e154672_d_n2, assign102820_e154672_d_n4, assign102820_e154672_d_n5, assign102820_e154672_d_n6, assign102820_e154672_d_n7, assign102820_e154672_d_n8, assign102820_e154672_d_n9, assign102820_e154672_d_n10, assign102820_e154672_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 == 0.0)) && (locals.var_guard2341 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102820_e154672;
        locals.var_t3_dn0 = assign102820_e154672_d_n0;
        locals.var_t3_dn2 = assign102820_e154672_d_n2;
        locals.var_t3_dn4 = assign102820_e154672_d_n4;
        locals.var_t3_dn5 = assign102820_e154672_d_n5;
        locals.var_t3_dn6 = assign102820_e154672_d_n6;
        locals.var_t3_dn7 = assign102820_e154672_d_n7;
        locals.var_t3_dn8 = assign102820_e154672_d_n8;
        locals.var_t3_dn9 = assign102820_e154672_d_n9;
        locals.var_t3_dn10 = assign102820_e154672_d_n10;
        locals.var_t3_dn13 = assign102820_e154672_d_n13;

        let (assign102830_e154689, assign102830_e154689_d_n0, assign102830_e154689_d_n2, assign102830_e154689_d_n4, assign102830_e154689_d_n5, assign102830_e154689_d_n6, assign102830_e154689_d_n7, assign102830_e154689_d_n8, assign102830_e154689_d_n9, assign102830_e154689_d_n10, assign102830_e154689_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2340 == 0.0)) && (locals.var_guard2341 == 0.0)) {
        let assign102830_e154686: f64 = (locals.var_uc_rdrbb_s - 1.0);
        let assign102830_e154687: f64 = (locals.var_t1).powf(assign102830_e154686);
        (assign102830_e154687, if locals.var_uc_rdrbb_s_dn0 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn0)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn0 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn2 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn2)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn2 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn4 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn4)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn4 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn5 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn5)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn5 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn6 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn6)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn6 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn7 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn7)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn7 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn8 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn8)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn8 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn9 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn9)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn9 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn10 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn10)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn10 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn13 == 0.0 && ((assign102830_e154686) as f64).is_finite() && ((assign102830_e154686) as f64).fract() == 0.0 { if assign102830_e154686 == 0.0 { 0.0 } else { (assign102830_e154686 * ((locals.var_t1).powf(assign102830_e154686 - 1.0) * locals.var_t1_dn13)) } } else { (assign102830_e154687 * ((locals.var_uc_rdrbb_s_dn13 * (locals.var_t1).ln()) + (assign102830_e154686 * (locals.var_t1_dn13 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign102830_e154689;
        locals.var_t3_dn0 = assign102830_e154689_d_n0;
        locals.var_t3_dn2 = assign102830_e154689_d_n2;
        locals.var_t3_dn4 = assign102830_e154689_d_n4;
        locals.var_t3_dn5 = assign102830_e154689_d_n5;
        locals.var_t3_dn6 = assign102830_e154689_d_n6;
        locals.var_t3_dn7 = assign102830_e154689_d_n7;
        locals.var_t3_dn8 = assign102830_e154689_d_n8;
        locals.var_t3_dn9 = assign102830_e154689_d_n9;
        locals.var_t3_dn10 = assign102830_e154689_d_n10;
        locals.var_t3_dn13 = assign102830_e154689_d_n13;

        let (assign102840_e154698, assign102840_e154698_d_n0, assign102840_e154698_d_n2, assign102840_e154698_d_n4, assign102840_e154698_d_n5, assign102840_e154698_d_n6, assign102840_e154698_d_n7, assign102840_e154698_d_n8, assign102840_e154698_d_n9, assign102840_e154698_d_n10, assign102840_e154698_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102840_e154696: f64 = (locals.var_t1 * locals.var_t3);
        (assign102840_e154696, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign102840_e154698;
        locals.var_t2_dn0 = assign102840_e154698_d_n0;
        locals.var_t2_dn2 = assign102840_e154698_d_n2;
        locals.var_t2_dn4 = assign102840_e154698_d_n4;
        locals.var_t2_dn5 = assign102840_e154698_d_n5;
        locals.var_t2_dn6 = assign102840_e154698_d_n6;
        locals.var_t2_dn7 = assign102840_e154698_d_n7;
        locals.var_t2_dn8 = assign102840_e154698_d_n8;
        locals.var_t2_dn9 = assign102840_e154698_d_n9;
        locals.var_t2_dn10 = assign102840_e154698_d_n10;
        locals.var_t2_dn13 = assign102840_e154698_d_n13;

        let (assign102850_e154707, assign102850_e154707_d_n0, assign102850_e154707_d_n2, assign102850_e154707_d_n4, assign102850_e154707_d_n5, assign102850_e154707_d_n6, assign102850_e154707_d_n7, assign102850_e154707_d_n8, assign102850_e154707_d_n9, assign102850_e154707_d_n10, assign102850_e154707_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102850_e154705: f64 = (1.0 + locals.var_t2);
        (assign102850_e154705, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign102850_e154707;
        locals.var_t4_dn0 = assign102850_e154707_d_n0;
        locals.var_t4_dn2 = assign102850_e154707_d_n2;
        locals.var_t4_dn4 = assign102850_e154707_d_n4;
        locals.var_t4_dn5 = assign102850_e154707_d_n5;
        locals.var_t4_dn6 = assign102850_e154707_d_n6;
        locals.var_t4_dn7 = assign102850_e154707_d_n7;
        locals.var_t4_dn8 = assign102850_e154707_d_n8;
        locals.var_t4_dn9 = assign102850_e154707_d_n9;
        locals.var_t4_dn10 = assign102850_e154707_d_n10;
        locals.var_t4_dn13 = assign102850_e154707_d_n13;

        let assign102860_e154711: f64 = (10.0 * 2.220446049250313e-16);
        let assign102860_e154712: f64 = (1.0 - assign102860_e154711);
        let assign102860_e154719: f64 = (10.0 * 2.220446049250313e-16);
        let assign102860_e154720: f64 = (1.0 + assign102860_e154719);
        let assign102860_e154722: f64 = if ((assign102860_e154712 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102860_e154720)) { 1.0 } else { 0.0 };
        locals.var_guard2342 = assign102860_e154722;

        let (assign102870_e154733, assign102870_e154733_d_n0, assign102870_e154733_d_n2, assign102870_e154733_d_n4, assign102870_e154733_d_n5, assign102870_e154733_d_n6, assign102870_e154733_d_n7, assign102870_e154733_d_n8, assign102870_e154733_d_n9, assign102870_e154733_d_n10, assign102870_e154733_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102870_e154731: f64 = (1.0 / locals.var_t4);
        (assign102870_e154731, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102870_e154733;
        locals.var_t5_dn0 = assign102870_e154733_d_n0;
        locals.var_t5_dn2 = assign102870_e154733_d_n2;
        locals.var_t5_dn4 = assign102870_e154733_d_n4;
        locals.var_t5_dn5 = assign102870_e154733_d_n5;
        locals.var_t5_dn6 = assign102870_e154733_d_n6;
        locals.var_t5_dn7 = assign102870_e154733_d_n7;
        locals.var_t5_dn8 = assign102870_e154733_d_n8;
        locals.var_t5_dn9 = assign102870_e154733_d_n9;
        locals.var_t5_dn10 = assign102870_e154733_d_n10;
        locals.var_t5_dn13 = assign102870_e154733_d_n13;

        let assign102880_e154737: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154738: f64 = (2.0 - assign102880_e154737);
        let assign102880_e154745: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154746: f64 = (2.0 + assign102880_e154745);
        let assign102880_e154748: f64 = if ((assign102880_e154738 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102880_e154746)) { 1.0 } else { 0.0 };
        locals.var_guard2343 = assign102880_e154748;

        let (assign102890_e154763, assign102890_e154763_d_n0, assign102890_e154763_d_n2, assign102890_e154763_d_n4, assign102890_e154763_d_n5, assign102890_e154763_d_n6, assign102890_e154763_d_n7, assign102890_e154763_d_n8, assign102890_e154763_d_n9, assign102890_e154763_d_n10, assign102890_e154763_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 != 0.0)) {
        let assign102890_e154760: f64 = (locals.var_t4).sqrt();
        let assign102890_e154761: f64 = (1.0 / assign102890_e154760);
        (assign102890_e154761, (-((locals.var_t4_dn0 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn2 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn4 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn5 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn6 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn7 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn8 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn9 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn10 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))), (-((locals.var_t4_dn13 / (2.0 * assign102890_e154760)) / (assign102890_e154760 * assign102890_e154760))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102890_e154763;
        locals.var_t5_dn0 = assign102890_e154763_d_n0;
        locals.var_t5_dn2 = assign102890_e154763_d_n2;
        locals.var_t5_dn4 = assign102890_e154763_d_n4;
        locals.var_t5_dn5 = assign102890_e154763_d_n5;
        locals.var_t5_dn6 = assign102890_e154763_d_n6;
        locals.var_t5_dn7 = assign102890_e154763_d_n7;
        locals.var_t5_dn8 = assign102890_e154763_d_n8;
        locals.var_t5_dn9 = assign102890_e154763_d_n9;
        locals.var_t5_dn10 = assign102890_e154763_d_n10;
        locals.var_t5_dn13 = assign102890_e154763_d_n13;

        let (assign102900_e154788, assign102900_e154788_d_n0, assign102900_e154788_d_n2, assign102900_e154788_d_n4, assign102900_e154788_d_n5, assign102900_e154788_d_n6, assign102900_e154788_d_n7, assign102900_e154788_d_n8, assign102900_e154788_d_n9, assign102900_e154788_d_n10, assign102900_e154788_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let (assign102900_e154786, assign102900_e154786_d_n0, assign102900_e154786_d_n2, assign102900_e154786_d_n4, assign102900_e154786_d_n5, assign102900_e154786_d_n6, assign102900_e154786_d_n7, assign102900_e154786_d_n8, assign102900_e154786_d_n9, assign102900_e154786_d_n10, assign102900_e154786_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102900_e154780: f64 = (-1.0);
                let assign102900_e154782: f64 = (assign102900_e154780 / locals.var_uc_rdrbb_s);
                let assign102900_e154784: f64 = (assign102900_e154782 - 1.0);
                let assign102900_e154785: f64 = (locals.var_t4).powf(assign102900_e154784);
                (assign102900_e154785, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn0)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn2)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn4)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn5)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn6)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn7)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn8)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn9)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn10)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn13) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102900_e154784) as f64).is_finite() && ((assign102900_e154784) as f64).fract() == 0.0 { if assign102900_e154784 == 0.0 { 0.0 } else { (assign102900_e154784 * ((locals.var_t4).powf(assign102900_e154784 - 1.0) * locals.var_t4_dn13)) } } else { (assign102900_e154785 * (((-((assign102900_e154780 * locals.var_uc_rdrbb_s_dn13) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102900_e154784 * (locals.var_t4_dn13 / locals.var_t4)))) },)
            }
        };
        (assign102900_e154786, assign102900_e154786_d_n0, assign102900_e154786_d_n2, assign102900_e154786_d_n4, assign102900_e154786_d_n5, assign102900_e154786_d_n6, assign102900_e154786_d_n7, assign102900_e154786_d_n8, assign102900_e154786_d_n9, assign102900_e154786_d_n10, assign102900_e154786_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign102900_e154788;
        locals.var_t6_dn0 = assign102900_e154788_d_n0;
        locals.var_t6_dn2 = assign102900_e154788_d_n2;
        locals.var_t6_dn4 = assign102900_e154788_d_n4;
        locals.var_t6_dn5 = assign102900_e154788_d_n5;
        locals.var_t6_dn6 = assign102900_e154788_d_n6;
        locals.var_t6_dn7 = assign102900_e154788_d_n7;
        locals.var_t6_dn8 = assign102900_e154788_d_n8;
        locals.var_t6_dn9 = assign102900_e154788_d_n9;
        locals.var_t6_dn10 = assign102900_e154788_d_n10;
        locals.var_t6_dn13 = assign102900_e154788_d_n13;

        let (assign102910_e154803, assign102910_e154803_d_n0, assign102910_e154803_d_n2, assign102910_e154803_d_n4, assign102910_e154803_d_n5, assign102910_e154803_d_n6, assign102910_e154803_d_n7, assign102910_e154803_d_n8, assign102910_e154803_d_n9, assign102910_e154803_d_n10, assign102910_e154803_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let assign102910_e154801: f64 = (locals.var_t4 * locals.var_t6);
        (assign102910_e154801, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign102910_e154803;
        locals.var_t5_dn0 = assign102910_e154803_d_n0;
        locals.var_t5_dn2 = assign102910_e154803_d_n2;
        locals.var_t5_dn4 = assign102910_e154803_d_n4;
        locals.var_t5_dn5 = assign102910_e154803_d_n5;
        locals.var_t5_dn6 = assign102910_e154803_d_n6;
        locals.var_t5_dn7 = assign102910_e154803_d_n7;
        locals.var_t5_dn8 = assign102910_e154803_d_n8;
        locals.var_t5_dn9 = assign102910_e154803_d_n9;
        locals.var_t5_dn10 = assign102910_e154803_d_n10;
        locals.var_t5_dn13 = assign102910_e154803_d_n13;

        let (assign102920_e154812, assign102920_e154812_d_n0, assign102920_e154812_d_n2, assign102920_e154812_d_n4, assign102920_e154812_d_n5, assign102920_e154812_d_n6, assign102920_e154812_d_n7, assign102920_e154812_d_n8, assign102920_e154812_d_n9, assign102920_e154812_d_n10, assign102920_e154812_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102920_e154810: f64 = (locals.var_mu0_s * locals.var_t5);
        (assign102920_e154810, ((locals.var_mu0_s_dn0 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn0)), ((locals.var_mu0_s_dn2 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn2)), ((locals.var_mu0_s_dn4 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn4)), ((locals.var_mu0_s_dn5 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn5)), ((locals.var_mu0_s_dn6 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn6)), ((locals.var_mu0_s_dn7 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn7)), ((locals.var_mu0_s_dn8 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn8)), ((locals.var_mu0_s_dn9 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn9)), ((locals.var_mu0_s_dn10 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn10)), ((locals.var_mu0_s_dn13 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu_s, locals.var_mu_s_dn0, locals.var_mu_s_dn2, locals.var_mu_s_dn4, locals.var_mu_s_dn5, locals.var_mu_s_dn6, locals.var_mu_s_dn7, locals.var_mu_s_dn8, locals.var_mu_s_dn9, locals.var_mu_s_dn10, locals.var_mu_s_dn13,)
    }
};
        locals.var_mu_s = assign102920_e154812;
        locals.var_mu_s_dn0 = assign102920_e154812_d_n0;
        locals.var_mu_s_dn2 = assign102920_e154812_d_n2;
        locals.var_mu_s_dn4 = assign102920_e154812_d_n4;
        locals.var_mu_s_dn5 = assign102920_e154812_d_n5;
        locals.var_mu_s_dn6 = assign102920_e154812_d_n6;
        locals.var_mu_s_dn7 = assign102920_e154812_d_n7;
        locals.var_mu_s_dn8 = assign102920_e154812_d_n8;
        locals.var_mu_s_dn9 = assign102920_e154812_d_n9;
        locals.var_mu_s_dn10 = assign102920_e154812_d_n10;
        locals.var_mu_s_dn13 = assign102920_e154812_d_n13;

        let (assign102930_e154819,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_novers,)
    } else {
        (locals.var_carr_s,)
    }
};
        locals.var_carr_s = assign102930_e154819;

        let (assign102940_e154826,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        (locals.var_xmax_s,)
    } else {
        (locals.var_xov_s,)
    }
};
        locals.var_xov_s = assign102940_e154826;

        let (assign102950_e154835, assign102950_e154835_d_n0, assign102950_e154835_d_n2, assign102950_e154835_d_n4, assign102950_e154835_d_n5, assign102950_e154835_d_n6, assign102950_e154835_d_n7, assign102950_e154835_d_n8, assign102950_e154835_d_n9, assign102950_e154835_d_n10, assign102950_e154835_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102950_e154833: f64 = (1.6021918e-19 / locals.var_ldrifte_s);
        (assign102950_e154833, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign102950_e154835;
        locals.var_t1_dn0 = assign102950_e154835_d_n0;
        locals.var_t1_dn2 = assign102950_e154835_d_n2;
        locals.var_t1_dn4 = assign102950_e154835_d_n4;
        locals.var_t1_dn5 = assign102950_e154835_d_n5;
        locals.var_t1_dn6 = assign102950_e154835_d_n6;
        locals.var_t1_dn7 = assign102950_e154835_d_n7;
        locals.var_t1_dn8 = assign102950_e154835_d_n8;
        locals.var_t1_dn9 = assign102950_e154835_d_n9;
        locals.var_t1_dn10 = assign102950_e154835_d_n10;
        locals.var_t1_dn13 = assign102950_e154835_d_n13;

        let (assign102960_e154848, assign102960_e154848_d_n0, assign102960_e154848_d_n2, assign102960_e154848_d_n4, assign102960_e154848_d_n5, assign102960_e154848_d_n6, assign102960_e154848_d_n7, assign102960_e154848_d_n8, assign102960_e154848_d_n9, assign102960_e154848_d_n10, assign102960_e154848_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102960_e154842: f64 = (locals.var_t1 * locals.var_xov_s);
        let assign102960_e154844: f64 = (assign102960_e154842 * locals.var_mu_s);
        let assign102960_e154846: f64 = (assign102960_e154844 * locals.var_carr_s);
        (assign102960_e154846, ((((locals.var_t1_dn0 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn0)) * locals.var_carr_s), ((((locals.var_t1_dn2 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn2)) * locals.var_carr_s), ((((locals.var_t1_dn4 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn4)) * locals.var_carr_s), ((((locals.var_t1_dn5 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn5)) * locals.var_carr_s), ((((locals.var_t1_dn6 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn6)) * locals.var_carr_s), ((((locals.var_t1_dn7 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn7)) * locals.var_carr_s), ((((locals.var_t1_dn8 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn8)) * locals.var_carr_s), ((((locals.var_t1_dn9 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn9)) * locals.var_carr_s), ((((locals.var_t1_dn10 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn10)) * locals.var_carr_s), ((((locals.var_t1_dn13 * locals.var_xov_s) * locals.var_mu_s) + (assign102960_e154842 * locals.var_mu_s_dn13)) * locals.var_carr_s),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign102960_e154848;
        locals.var_gd_s_dn0 = assign102960_e154848_d_n0;
        locals.var_gd_s_dn2 = assign102960_e154848_d_n2;
        locals.var_gd_s_dn4 = assign102960_e154848_d_n4;
        locals.var_gd_s_dn5 = assign102960_e154848_d_n5;
        locals.var_gd_s_dn6 = assign102960_e154848_d_n6;
        locals.var_gd_s_dn7 = assign102960_e154848_d_n7;
        locals.var_gd_s_dn8 = assign102960_e154848_d_n8;
        locals.var_gd_s_dn9 = assign102960_e154848_d_n9;
        locals.var_gd_s_dn10 = assign102960_e154848_d_n10;
        locals.var_gd_s_dn13 = assign102960_e154848_d_n13;

        let assign102970_e154852: f64 = 1e-25;
        let assign102970_e154857: f64 = if ((locals.var_gd_s < assign102970_e154852) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2344 = assign102970_e154857;

        let (assign102980_e154870, assign102980_e154870_d_n0, assign102980_e154870_d_n2, assign102980_e154870_d_n4, assign102980_e154870_d_n5, assign102980_e154870_d_n6, assign102980_e154870_d_n7, assign102980_e154870_d_n8, assign102980_e154870_d_n9, assign102980_e154870_d_n10, assign102980_e154870_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign102980_e154866: f64 = 1e-25;
        let assign102980_e154868: f64 = (assign102980_e154866 - locals.var_gd_s);
        (assign102980_e154868, (-locals.var_gd_s_dn0), (-locals.var_gd_s_dn2), (-locals.var_gd_s_dn4), (-locals.var_gd_s_dn5), (-locals.var_gd_s_dn6), (-locals.var_gd_s_dn7), (-locals.var_gd_s_dn8), (-locals.var_gd_s_dn9), (-locals.var_gd_s_dn10), (-locals.var_gd_s_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign102980_e154870;
        locals.var_tmf1_dn0 = assign102980_e154870_d_n0;
        locals.var_tmf1_dn2 = assign102980_e154870_d_n2;
        locals.var_tmf1_dn4 = assign102980_e154870_d_n4;
        locals.var_tmf1_dn5 = assign102980_e154870_d_n5;
        locals.var_tmf1_dn6 = assign102980_e154870_d_n6;
        locals.var_tmf1_dn7 = assign102980_e154870_d_n7;
        locals.var_tmf1_dn8 = assign102980_e154870_d_n8;
        locals.var_tmf1_dn9 = assign102980_e154870_d_n9;
        locals.var_tmf1_dn10 = assign102980_e154870_d_n10;
        locals.var_tmf1_dn13 = assign102980_e154870_d_n13;

        let (assign102990_e154881, assign102990_e154881_d_n0, assign102990_e154881_d_n2, assign102990_e154881_d_n4, assign102990_e154881_d_n5, assign102990_e154881_d_n6, assign102990_e154881_d_n7, assign102990_e154881_d_n8, assign102990_e154881_d_n9, assign102990_e154881_d_n10, assign102990_e154881_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign102990_e154879: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign102990_e154879, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign102990_e154881;
        locals.var_x2_dn0 = assign102990_e154881_d_n0;
        locals.var_x2_dn2 = assign102990_e154881_d_n2;
        locals.var_x2_dn4 = assign102990_e154881_d_n4;
        locals.var_x2_dn5 = assign102990_e154881_d_n5;
        locals.var_x2_dn6 = assign102990_e154881_d_n6;
        locals.var_x2_dn7 = assign102990_e154881_d_n7;
        locals.var_x2_dn8 = assign102990_e154881_d_n8;
        locals.var_x2_dn9 = assign102990_e154881_d_n9;
        locals.var_x2_dn10 = assign102990_e154881_d_n10;
        locals.var_x2_dn13 = assign102990_e154881_d_n13;

        let (assign103000_e154892, assign103000_e154892_d_n0, assign103000_e154892_d_n2, assign103000_e154892_d_n4, assign103000_e154892_d_n5, assign103000_e154892_d_n6, assign103000_e154892_d_n7, assign103000_e154892_d_n8, assign103000_e154892_d_n9, assign103000_e154892_d_n10, assign103000_e154892_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103000_e154890: f64 = (1e-25 * 1e-25);
        (assign103000_e154890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign103000_e154892;
        locals.var_xmax2_dn0 = assign103000_e154892_d_n0;
        locals.var_xmax2_dn2 = assign103000_e154892_d_n2;
        locals.var_xmax2_dn4 = assign103000_e154892_d_n4;
        locals.var_xmax2_dn5 = assign103000_e154892_d_n5;
        locals.var_xmax2_dn6 = assign103000_e154892_d_n6;
        locals.var_xmax2_dn7 = assign103000_e154892_d_n7;
        locals.var_xmax2_dn8 = assign103000_e154892_d_n8;
        locals.var_xmax2_dn9 = assign103000_e154892_d_n9;
        locals.var_xmax2_dn10 = assign103000_e154892_d_n10;
        locals.var_xmax2_dn13 = assign103000_e154892_d_n13;

        let (assign103010_e154901, assign103010_e154901_d_n0, assign103010_e154901_d_n2, assign103010_e154901_d_n4, assign103010_e154901_d_n5, assign103010_e154901_d_n6, assign103010_e154901_d_n7, assign103010_e154901_d_n8, assign103010_e154901_d_n9, assign103010_e154901_d_n10, assign103010_e154901_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103010_e154901;
        locals.var_xp_dn0 = assign103010_e154901_d_n0;
        locals.var_xp_dn2 = assign103010_e154901_d_n2;
        locals.var_xp_dn4 = assign103010_e154901_d_n4;
        locals.var_xp_dn5 = assign103010_e154901_d_n5;
        locals.var_xp_dn6 = assign103010_e154901_d_n6;
        locals.var_xp_dn7 = assign103010_e154901_d_n7;
        locals.var_xp_dn8 = assign103010_e154901_d_n8;
        locals.var_xp_dn9 = assign103010_e154901_d_n9;
        locals.var_xp_dn10 = assign103010_e154901_d_n10;
        locals.var_xp_dn13 = assign103010_e154901_d_n13;

        let (assign103020_e154910, assign103020_e154910_d_n0, assign103020_e154910_d_n2, assign103020_e154910_d_n4, assign103020_e154910_d_n5, assign103020_e154910_d_n6, assign103020_e154910_d_n7, assign103020_e154910_d_n8, assign103020_e154910_d_n9, assign103020_e154910_d_n10, assign103020_e154910_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103020_e154910;
        locals.var_xmp_dn0 = assign103020_e154910_d_n0;
        locals.var_xmp_dn2 = assign103020_e154910_d_n2;
        locals.var_xmp_dn4 = assign103020_e154910_d_n4;
        locals.var_xmp_dn5 = assign103020_e154910_d_n5;
        locals.var_xmp_dn6 = assign103020_e154910_d_n6;
        locals.var_xmp_dn7 = assign103020_e154910_d_n7;
        locals.var_xmp_dn8 = assign103020_e154910_d_n8;
        locals.var_xmp_dn9 = assign103020_e154910_d_n9;
        locals.var_xmp_dn10 = assign103020_e154910_d_n10;
        locals.var_xmp_dn13 = assign103020_e154910_d_n13;

        let (assign103030_e154919,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103030_e154919;

        let (assign103040_e154928,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103040_e154928;

        let (assign103050_e154937, assign103050_e154937_d_n0, assign103050_e154937_d_n2, assign103050_e154937_d_n4, assign103050_e154937_d_n5, assign103050_e154937_d_n6, assign103050_e154937_d_n7, assign103050_e154937_d_n8, assign103050_e154937_d_n9, assign103050_e154937_d_n10, assign103050_e154937_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign103050_e154937;
        locals.var_arg_dn0 = assign103050_e154937_d_n0;
        locals.var_arg_dn2 = assign103050_e154937_d_n2;
        locals.var_arg_dn4 = assign103050_e154937_d_n4;
        locals.var_arg_dn5 = assign103050_e154937_d_n5;
        locals.var_arg_dn6 = assign103050_e154937_d_n6;
        locals.var_arg_dn7 = assign103050_e154937_d_n7;
        locals.var_arg_dn8 = assign103050_e154937_d_n8;
        locals.var_arg_dn9 = assign103050_e154937_d_n9;
        locals.var_arg_dn10 = assign103050_e154937_d_n10;
        locals.var_arg_dn13 = assign103050_e154937_d_n13;

        let (assign103060_e154946, assign103060_e154946_d_n0, assign103060_e154946_d_n2, assign103060_e154946_d_n4, assign103060_e154946_d_n5, assign103060_e154946_d_n6, assign103060_e154946_d_n7, assign103060_e154946_d_n8, assign103060_e154946_d_n9, assign103060_e154946_d_n10, assign103060_e154946_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103060_e154946;
        locals.var_dnm_dn0 = assign103060_e154946_d_n0;
        locals.var_dnm_dn2 = assign103060_e154946_d_n2;
        locals.var_dnm_dn4 = assign103060_e154946_d_n4;
        locals.var_dnm_dn5 = assign103060_e154946_d_n5;
        locals.var_dnm_dn6 = assign103060_e154946_d_n6;
        locals.var_dnm_dn7 = assign103060_e154946_d_n7;
        locals.var_dnm_dn8 = assign103060_e154946_d_n8;
        locals.var_dnm_dn9 = assign103060_e154946_d_n9;
        locals.var_dnm_dn10 = assign103060_e154946_d_n10;
        locals.var_dnm_dn13 = assign103060_e154946_d_n13;

        let (assign103070_e154957, assign103070_e154957_d_n0, assign103070_e154957_d_n2, assign103070_e154957_d_n4, assign103070_e154957_d_n5, assign103070_e154957_d_n6, assign103070_e154957_d_n7, assign103070_e154957_d_n8, assign103070_e154957_d_n9, assign103070_e154957_d_n10, assign103070_e154957_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103070_e154955: f64 = (locals.var_xp * locals.var_x2);
        (assign103070_e154955, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103070_e154957;
        locals.var_xp_dn0 = assign103070_e154957_d_n0;
        locals.var_xp_dn2 = assign103070_e154957_d_n2;
        locals.var_xp_dn4 = assign103070_e154957_d_n4;
        locals.var_xp_dn5 = assign103070_e154957_d_n5;
        locals.var_xp_dn6 = assign103070_e154957_d_n6;
        locals.var_xp_dn7 = assign103070_e154957_d_n7;
        locals.var_xp_dn8 = assign103070_e154957_d_n8;
        locals.var_xp_dn9 = assign103070_e154957_d_n9;
        locals.var_xp_dn10 = assign103070_e154957_d_n10;
        locals.var_xp_dn13 = assign103070_e154957_d_n13;

        let (assign103080_e154968, assign103080_e154968_d_n0, assign103080_e154968_d_n2, assign103080_e154968_d_n4, assign103080_e154968_d_n5, assign103080_e154968_d_n6, assign103080_e154968_d_n7, assign103080_e154968_d_n8, assign103080_e154968_d_n9, assign103080_e154968_d_n10, assign103080_e154968_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103080_e154966: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103080_e154966, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103080_e154968;
        locals.var_xmp_dn0 = assign103080_e154968_d_n0;
        locals.var_xmp_dn2 = assign103080_e154968_d_n2;
        locals.var_xmp_dn4 = assign103080_e154968_d_n4;
        locals.var_xmp_dn5 = assign103080_e154968_d_n5;
        locals.var_xmp_dn6 = assign103080_e154968_d_n6;
        locals.var_xmp_dn7 = assign103080_e154968_d_n7;
        locals.var_xmp_dn8 = assign103080_e154968_d_n8;
        locals.var_xmp_dn9 = assign103080_e154968_d_n9;
        locals.var_xmp_dn10 = assign103080_e154968_d_n10;
        locals.var_xmp_dn13 = assign103080_e154968_d_n13;

        let (assign103090_e154979, assign103090_e154979_d_n0, assign103090_e154979_d_n2, assign103090_e154979_d_n4, assign103090_e154979_d_n5, assign103090_e154979_d_n6, assign103090_e154979_d_n7, assign103090_e154979_d_n8, assign103090_e154979_d_n9, assign103090_e154979_d_n10, assign103090_e154979_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103090_e154977: f64 = (locals.var_xp * locals.var_x2);
        (assign103090_e154977, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign103090_e154979;
        locals.var_xp_dn0 = assign103090_e154979_d_n0;
        locals.var_xp_dn2 = assign103090_e154979_d_n2;
        locals.var_xp_dn4 = assign103090_e154979_d_n4;
        locals.var_xp_dn5 = assign103090_e154979_d_n5;
        locals.var_xp_dn6 = assign103090_e154979_d_n6;
        locals.var_xp_dn7 = assign103090_e154979_d_n7;
        locals.var_xp_dn8 = assign103090_e154979_d_n8;
        locals.var_xp_dn9 = assign103090_e154979_d_n9;
        locals.var_xp_dn10 = assign103090_e154979_d_n10;
        locals.var_xp_dn13 = assign103090_e154979_d_n13;

        let (assign103100_e154990, assign103100_e154990_d_n0, assign103100_e154990_d_n2, assign103100_e154990_d_n4, assign103100_e154990_d_n5, assign103100_e154990_d_n6, assign103100_e154990_d_n7, assign103100_e154990_d_n8, assign103100_e154990_d_n9, assign103100_e154990_d_n10, assign103100_e154990_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103100_e154988: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103100_e154988, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign103100_e154990;
        locals.var_xmp_dn0 = assign103100_e154990_d_n0;
        locals.var_xmp_dn2 = assign103100_e154990_d_n2;
        locals.var_xmp_dn4 = assign103100_e154990_d_n4;
        locals.var_xmp_dn5 = assign103100_e154990_d_n5;
        locals.var_xmp_dn6 = assign103100_e154990_d_n6;
        locals.var_xmp_dn7 = assign103100_e154990_d_n7;
        locals.var_xmp_dn8 = assign103100_e154990_d_n8;
        locals.var_xmp_dn9 = assign103100_e154990_d_n9;
        locals.var_xmp_dn10 = assign103100_e154990_d_n10;
        locals.var_xmp_dn13 = assign103100_e154990_d_n13;

    }

    pub(super) fn stamp_transient_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103110_e155001, assign103110_e155001_d_n0, assign103110_e155001_d_n2, assign103110_e155001_d_n4, assign103110_e155001_d_n5, assign103110_e155001_d_n6, assign103110_e155001_d_n7, assign103110_e155001_d_n8, assign103110_e155001_d_n9, assign103110_e155001_d_n10, assign103110_e155001_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103110_e154999: f64 = (locals.var_xp + locals.var_xmp);
        (assign103110_e154999, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign103110_e155001;
        locals.var_arg_dn0 = assign103110_e155001_d_n0;
        locals.var_arg_dn2 = assign103110_e155001_d_n2;
        locals.var_arg_dn4 = assign103110_e155001_d_n4;
        locals.var_arg_dn5 = assign103110_e155001_d_n5;
        locals.var_arg_dn6 = assign103110_e155001_d_n6;
        locals.var_arg_dn7 = assign103110_e155001_d_n7;
        locals.var_arg_dn8 = assign103110_e155001_d_n8;
        locals.var_arg_dn9 = assign103110_e155001_d_n9;
        locals.var_arg_dn10 = assign103110_e155001_d_n10;
        locals.var_arg_dn13 = assign103110_e155001_d_n13;

        let (assign103120_e155010, assign103120_e155010_d_n0, assign103120_e155010_d_n2, assign103120_e155010_d_n4, assign103120_e155010_d_n5, assign103120_e155010_d_n6, assign103120_e155010_d_n7, assign103120_e155010_d_n8, assign103120_e155010_d_n9, assign103120_e155010_d_n10, assign103120_e155010_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103120_e155010;
        locals.var_dnm_dn0 = assign103120_e155010_d_n0;
        locals.var_dnm_dn2 = assign103120_e155010_d_n2;
        locals.var_dnm_dn4 = assign103120_e155010_d_n4;
        locals.var_dnm_dn5 = assign103120_e155010_d_n5;
        locals.var_dnm_dn6 = assign103120_e155010_d_n6;
        locals.var_dnm_dn7 = assign103120_e155010_d_n7;
        locals.var_dnm_dn8 = assign103120_e155010_d_n8;
        locals.var_dnm_dn9 = assign103120_e155010_d_n9;
        locals.var_dnm_dn10 = assign103120_e155010_d_n10;
        locals.var_dnm_dn13 = assign103120_e155010_d_n13;

        let assign103130_e155025: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2345 = assign103130_e155025;

        let assign103140_e155028: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2346 = assign103140_e155028;

        let (assign103150_e155041,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103150_e155041;

        let assign103160_e155044: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2347 = assign103160_e155044;

        let (assign103170_e155060,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103170_e155060;

        let assign103180_e155063: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2348 = assign103180_e155063;

        let (assign103190_e155082,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103190_e155082;

        let assign103200_e155085: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2349 = assign103200_e155085;

        let (assign103210_e155107,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) && (locals.var_guard2348 == 0.0)) && (locals.var_guard2349 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103210_e155107;

        let (assign103220_e155118,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103220_e155118;

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign103230_body0_e155142, assign103230_body0_e155142_d_n0, assign103230_body0_e155142_d_n2, assign103230_body0_e155142_d_n4, assign103230_body0_e155142_d_n5, assign103230_body0_e155142_d_n6, assign103230_body0_e155142_d_n7, assign103230_body0_e155142_d_n8, assign103230_body0_e155142_d_n9, assign103230_body0_e155142_d_n10, assign103230_body0_e155142_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        let assign103230_body0_e155140: f64 = (locals.var_dnm).sqrt();
        (assign103230_body0_e155140, (locals.var_dnm_dn0 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn2 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn4 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn5 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn6 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn7 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn8 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn9 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn10 / (2.0 * assign103230_body0_e155140)), (locals.var_dnm_dn13 / (2.0 * assign103230_body0_e155140)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign103230_body0_e155142;
            locals.var_dnm_dn0 = assign103230_body0_e155142_d_n0;
            locals.var_dnm_dn2 = assign103230_body0_e155142_d_n2;
            locals.var_dnm_dn4 = assign103230_body0_e155142_d_n4;
            locals.var_dnm_dn5 = assign103230_body0_e155142_d_n5;
            locals.var_dnm_dn6 = assign103230_body0_e155142_d_n6;
            locals.var_dnm_dn7 = assign103230_body0_e155142_d_n7;
            locals.var_dnm_dn8 = assign103230_body0_e155142_d_n8;
            locals.var_dnm_dn9 = assign103230_body0_e155142_d_n9;
            locals.var_dnm_dn10 = assign103230_body0_e155142_d_n10;
            locals.var_dnm_dn13 = assign103230_body0_e155142_d_n13;
            let (assign103230_body1_e155155,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 != 0.0)) {
        let assign103230_body1_e155153: f64 = (locals.var_m0 + 1.0);
        (assign103230_body1_e155153,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign103230_body1_e155155;
        }

        let (assign103240_e155178, assign103240_e155178_d_n0, assign103240_e155178_d_n2, assign103240_e155178_d_n4, assign103240_e155178_d_n5, assign103240_e155178_d_n6, assign103240_e155178_d_n7, assign103240_e155178_d_n8, assign103240_e155178_d_n9, assign103240_e155178_d_n10, assign103240_e155178_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) && (locals.var_guard2345 == 0.0)) {
        let (assign103240_e155176, assign103240_e155176_d_n0, assign103240_e155176_d_n2, assign103240_e155176_d_n4, assign103240_e155176_d_n5, assign103240_e155176_d_n6, assign103240_e155176_d_n7, assign103240_e155176_d_n8, assign103240_e155176_d_n9, assign103240_e155176_d_n10, assign103240_e155176_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103240_e155173: f64 = (2.0 * 2.0);
                let assign103240_e155174: f64 = (1.0 / assign103240_e155173);
                let assign103240_e155175: f64 = (locals.var_dnm).powf(assign103240_e155174);
                (assign103240_e155175, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn0)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn2)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn4)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn5)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn6)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn7)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn8)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn9)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn10)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103240_e155174) as f64).is_finite() && ((assign103240_e155174) as f64).fract() == 0.0 { if assign103240_e155174 == 0.0 { 0.0 } else { (assign103240_e155174 * ((locals.var_dnm).powf(assign103240_e155174 - 1.0) * locals.var_dnm_dn13)) } } else { (assign103240_e155175 * (assign103240_e155174 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign103240_e155176, assign103240_e155176_d_n0, assign103240_e155176_d_n2, assign103240_e155176_d_n4, assign103240_e155176_d_n5, assign103240_e155176_d_n6, assign103240_e155176_d_n7, assign103240_e155176_d_n8, assign103240_e155176_d_n9, assign103240_e155176_d_n10, assign103240_e155176_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103240_e155178;
        locals.var_dnm_dn0 = assign103240_e155178_d_n0;
        locals.var_dnm_dn2 = assign103240_e155178_d_n2;
        locals.var_dnm_dn4 = assign103240_e155178_d_n4;
        locals.var_dnm_dn5 = assign103240_e155178_d_n5;
        locals.var_dnm_dn6 = assign103240_e155178_d_n6;
        locals.var_dnm_dn7 = assign103240_e155178_d_n7;
        locals.var_dnm_dn8 = assign103240_e155178_d_n8;
        locals.var_dnm_dn9 = assign103240_e155178_d_n9;
        locals.var_dnm_dn10 = assign103240_e155178_d_n10;
        locals.var_dnm_dn13 = assign103240_e155178_d_n13;

        let (assign103250_e155189, assign103250_e155189_d_n0, assign103250_e155189_d_n2, assign103250_e155189_d_n4, assign103250_e155189_d_n5, assign103250_e155189_d_n6, assign103250_e155189_d_n7, assign103250_e155189_d_n8, assign103250_e155189_d_n9, assign103250_e155189_d_n10, assign103250_e155189_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103250_e155187: f64 = (1.0 / locals.var_dnm);
        (assign103250_e155187, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign103250_e155189;
        locals.var_dnm_dn0 = assign103250_e155189_d_n0;
        locals.var_dnm_dn2 = assign103250_e155189_d_n2;
        locals.var_dnm_dn4 = assign103250_e155189_d_n4;
        locals.var_dnm_dn5 = assign103250_e155189_d_n5;
        locals.var_dnm_dn6 = assign103250_e155189_d_n6;
        locals.var_dnm_dn7 = assign103250_e155189_d_n7;
        locals.var_dnm_dn8 = assign103250_e155189_d_n8;
        locals.var_dnm_dn9 = assign103250_e155189_d_n9;
        locals.var_dnm_dn10 = assign103250_e155189_d_n10;
        locals.var_dnm_dn13 = assign103250_e155189_d_n13;

        let (assign103260_e155202, assign103260_e155202_d_n0, assign103260_e155202_d_n2, assign103260_e155202_d_n4, assign103260_e155202_d_n5, assign103260_e155202_d_n6, assign103260_e155202_d_n7, assign103260_e155202_d_n8, assign103260_e155202_d_n9, assign103260_e155202_d_n10, assign103260_e155202_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103260_e155198: f64 = (locals.var_tmf1 * 1e-25);
        let assign103260_e155200: f64 = (assign103260_e155198 * locals.var_dnm);
        (assign103260_e155200, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-25) * locals.var_dnm) + (assign103260_e155198 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign103260_e155202;
        locals.var_tmf0_dn0 = assign103260_e155202_d_n0;
        locals.var_tmf0_dn2 = assign103260_e155202_d_n2;
        locals.var_tmf0_dn4 = assign103260_e155202_d_n4;
        locals.var_tmf0_dn5 = assign103260_e155202_d_n5;
        locals.var_tmf0_dn6 = assign103260_e155202_d_n6;
        locals.var_tmf0_dn7 = assign103260_e155202_d_n7;
        locals.var_tmf0_dn8 = assign103260_e155202_d_n8;
        locals.var_tmf0_dn9 = assign103260_e155202_d_n9;
        locals.var_tmf0_dn10 = assign103260_e155202_d_n10;
        locals.var_tmf0_dn13 = assign103260_e155202_d_n13;

        let (assign103270_e155217, assign103270_e155217_d_n0, assign103270_e155217_d_n2, assign103270_e155217_d_n4, assign103270_e155217_d_n5, assign103270_e155217_d_n6, assign103270_e155217_d_n7, assign103270_e155217_d_n8, assign103270_e155217_d_n9, assign103270_e155217_d_n10, assign103270_e155217_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103270_e155211: f64 = (1e-25 * locals.var_xmp);
        let assign103270_e155213: f64 = (assign103270_e155211 * locals.var_dnm);
        let assign103270_e155215: f64 = (assign103270_e155213 / locals.var_arg);
        (assign103270_e155215, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn0)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn2)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn4)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn5)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn6)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn7)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn8)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn9)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn10)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn13) * locals.var_dnm) + (assign103270_e155211 * locals.var_dnm_dn13)) * locals.var_arg) - (assign103270_e155213 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103270_e155217;
        locals.var_t0_dn0 = assign103270_e155217_d_n0;
        locals.var_t0_dn2 = assign103270_e155217_d_n2;
        locals.var_t0_dn4 = assign103270_e155217_d_n4;
        locals.var_t0_dn5 = assign103270_e155217_d_n5;
        locals.var_t0_dn6 = assign103270_e155217_d_n6;
        locals.var_t0_dn7 = assign103270_e155217_d_n7;
        locals.var_t0_dn8 = assign103270_e155217_d_n8;
        locals.var_t0_dn9 = assign103270_e155217_d_n9;
        locals.var_t0_dn10 = assign103270_e155217_d_n10;
        locals.var_t0_dn13 = assign103270_e155217_d_n13;

        let (assign103280_e155230, assign103280_e155230_d_n0, assign103280_e155230_d_n2, assign103280_e155230_d_n4, assign103280_e155230_d_n5, assign103280_e155230_d_n6, assign103280_e155230_d_n7, assign103280_e155230_d_n8, assign103280_e155230_d_n9, assign103280_e155230_d_n10, assign103280_e155230_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign103280_e155226: f64 = 1e-25;
        let assign103280_e155228: f64 = (assign103280_e155226 - locals.var_tmf0);
        (assign103280_e155228, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign103280_e155230;
        locals.var_gd_s_dn0 = assign103280_e155230_d_n0;
        locals.var_gd_s_dn2 = assign103280_e155230_d_n2;
        locals.var_gd_s_dn4 = assign103280_e155230_d_n4;
        locals.var_gd_s_dn5 = assign103280_e155230_d_n5;
        locals.var_gd_s_dn6 = assign103280_e155230_d_n6;
        locals.var_gd_s_dn7 = assign103280_e155230_d_n7;
        locals.var_gd_s_dn8 = assign103280_e155230_d_n8;
        locals.var_gd_s_dn9 = assign103280_e155230_d_n9;
        locals.var_gd_s_dn10 = assign103280_e155230_d_n10;
        locals.var_gd_s_dn13 = assign103280_e155230_d_n13;

        let (assign103290_e155239, assign103290_e155239_d_n0, assign103290_e155239_d_n2, assign103290_e155239_d_n4, assign103290_e155239_d_n5, assign103290_e155239_d_n6, assign103290_e155239_d_n7, assign103290_e155239_d_n8, assign103290_e155239_d_n9, assign103290_e155239_d_n10, assign103290_e155239_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103290_e155239;
        locals.var_t0_dn0 = assign103290_e155239_d_n0;
        locals.var_t0_dn2 = assign103290_e155239_d_n2;
        locals.var_t0_dn4 = assign103290_e155239_d_n4;
        locals.var_t0_dn5 = assign103290_e155239_d_n5;
        locals.var_t0_dn6 = assign103290_e155239_d_n6;
        locals.var_t0_dn7 = assign103290_e155239_d_n7;
        locals.var_t0_dn8 = assign103290_e155239_d_n8;
        locals.var_t0_dn9 = assign103290_e155239_d_n9;
        locals.var_t0_dn10 = assign103290_e155239_d_n10;
        locals.var_t0_dn13 = assign103290_e155239_d_n13;

        let (assign103300_e155249, assign103300_e155249_d_n0, assign103300_e155249_d_n2, assign103300_e155249_d_n4, assign103300_e155249_d_n5, assign103300_e155249_d_n6, assign103300_e155249_d_n7, assign103300_e155249_d_n8, assign103300_e155249_d_n9, assign103300_e155249_d_n10, assign103300_e155249_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 == 0.0)) {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn13,)
    }
};
        locals.var_gd_s = assign103300_e155249;
        locals.var_gd_s_dn0 = assign103300_e155249_d_n0;
        locals.var_gd_s_dn2 = assign103300_e155249_d_n2;
        locals.var_gd_s_dn4 = assign103300_e155249_d_n4;
        locals.var_gd_s_dn5 = assign103300_e155249_d_n5;
        locals.var_gd_s_dn6 = assign103300_e155249_d_n6;
        locals.var_gd_s_dn7 = assign103300_e155249_d_n7;
        locals.var_gd_s_dn8 = assign103300_e155249_d_n8;
        locals.var_gd_s_dn9 = assign103300_e155249_d_n9;
        locals.var_gd_s_dn10 = assign103300_e155249_d_n10;
        locals.var_gd_s_dn13 = assign103300_e155249_d_n13;

        let (assign103310_e155259, assign103310_e155259_d_n0, assign103310_e155259_d_n2, assign103310_e155259_d_n4, assign103310_e155259_d_n5, assign103310_e155259_d_n6, assign103310_e155259_d_n7, assign103310_e155259_d_n8, assign103310_e155259_d_n9, assign103310_e155259_d_n10, assign103310_e155259_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2344 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103310_e155259;
        locals.var_t0_dn0 = assign103310_e155259_d_n0;
        locals.var_t0_dn2 = assign103310_e155259_d_n2;
        locals.var_t0_dn4 = assign103310_e155259_d_n4;
        locals.var_t0_dn5 = assign103310_e155259_d_n5;
        locals.var_t0_dn6 = assign103310_e155259_d_n6;
        locals.var_t0_dn7 = assign103310_e155259_d_n7;
        locals.var_t0_dn8 = assign103310_e155259_d_n8;
        locals.var_t0_dn9 = assign103310_e155259_d_n9;
        locals.var_t0_dn10 = assign103310_e155259_d_n10;
        locals.var_t0_dn13 = assign103310_e155259_d_n13;

        let (assign103320_e155268, assign103320_e155268_d_n0, assign103320_e155268_d_n2, assign103320_e155268_d_n4, assign103320_e155268_d_n5, assign103320_e155268_d_n6, assign103320_e155268_d_n7, assign103320_e155268_d_n8, assign103320_e155268_d_n9, assign103320_e155268_d_n10, assign103320_e155268_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103320_e155266: f64 = (1.0 / locals.var_gd_s);
        (assign103320_e155266, (-(locals.var_gd_s_dn0 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn2 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn4 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn5 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn6 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn7 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn8 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn9 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn10 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn13 / (locals.var_gd_s * locals.var_gd_s))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103320_e155268;
        locals.var_rsd_dn0 = assign103320_e155268_d_n0;
        locals.var_rsd_dn2 = assign103320_e155268_d_n2;
        locals.var_rsd_dn4 = assign103320_e155268_d_n4;
        locals.var_rsd_dn5 = assign103320_e155268_d_n5;
        locals.var_rsd_dn6 = assign103320_e155268_d_n6;
        locals.var_rsd_dn7 = assign103320_e155268_d_n7;
        locals.var_rsd_dn8 = assign103320_e155268_d_n8;
        locals.var_rsd_dn9 = assign103320_e155268_d_n9;
        locals.var_rsd_dn10 = assign103320_e155268_d_n10;
        locals.var_rsd_dn13 = assign103320_e155268_d_n13;

        let (assign103330_e155277, assign103330_e155277_d_n0, assign103330_e155277_d_n2, assign103330_e155277_d_n4, assign103330_e155277_d_n5, assign103330_e155277_d_n6, assign103330_e155277_d_n7, assign103330_e155277_d_n8, assign103330_e155277_d_n9, assign103330_e155277_d_n10, assign103330_e155277_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103330_e155275: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign103330_e155275, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn13 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103330_e155277;
        locals.var_rsd_dn0 = assign103330_e155277_d_n0;
        locals.var_rsd_dn2 = assign103330_e155277_d_n2;
        locals.var_rsd_dn4 = assign103330_e155277_d_n4;
        locals.var_rsd_dn5 = assign103330_e155277_d_n5;
        locals.var_rsd_dn6 = assign103330_e155277_d_n6;
        locals.var_rsd_dn7 = assign103330_e155277_d_n7;
        locals.var_rsd_dn8 = assign103330_e155277_d_n8;
        locals.var_rsd_dn9 = assign103330_e155277_d_n9;
        locals.var_rsd_dn10 = assign103330_e155277_d_n10;
        locals.var_rsd_dn13 = assign103330_e155277_d_n13;

        let (assign103340_e155286, assign103340_e155286_d_n0, assign103340_e155286_d_n2, assign103340_e155286_d_n4, assign103340_e155286_d_n5, assign103340_e155286_d_n6, assign103340_e155286_d_n7, assign103340_e155286_d_n8, assign103340_e155286_d_n9, assign103340_e155286_d_n10, assign103340_e155286_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103340_e155284: f64 = (locals.var_rsd + locals.var_rs0);
        (assign103340_e155284, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103340_e155286;
        locals.var_rsd_dn0 = assign103340_e155286_d_n0;
        locals.var_rsd_dn2 = assign103340_e155286_d_n2;
        locals.var_rsd_dn4 = assign103340_e155286_d_n4;
        locals.var_rsd_dn5 = assign103340_e155286_d_n5;
        locals.var_rsd_dn6 = assign103340_e155286_d_n6;
        locals.var_rsd_dn7 = assign103340_e155286_d_n7;
        locals.var_rsd_dn8 = assign103340_e155286_d_n8;
        locals.var_rsd_dn9 = assign103340_e155286_d_n9;
        locals.var_rsd_dn10 = assign103340_e155286_d_n10;
        locals.var_rsd_dn13 = assign103340_e155286_d_n13;

        let assign103380_e155317: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2351 = assign103380_e155317;

        let (assign103390_e155326, assign103390_e155326_d_n0, assign103390_e155326_d_n2, assign103390_e155326_d_n4, assign103390_e155326_d_n5, assign103390_e155326_d_n6, assign103390_e155326_d_n7, assign103390_e155326_d_n8, assign103390_e155326_d_n9, assign103390_e155326_d_n10, assign103390_e155326_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) && (locals.var_guard2351 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign103390_e155326;
        locals.var_rsd_dn0 = assign103390_e155326_d_n0;
        locals.var_rsd_dn2 = assign103390_e155326_d_n2;
        locals.var_rsd_dn4 = assign103390_e155326_d_n4;
        locals.var_rsd_dn5 = assign103390_e155326_d_n5;
        locals.var_rsd_dn6 = assign103390_e155326_d_n6;
        locals.var_rsd_dn7 = assign103390_e155326_d_n7;
        locals.var_rsd_dn8 = assign103390_e155326_d_n8;
        locals.var_rsd_dn9 = assign103390_e155326_d_n9;
        locals.var_rsd_dn10 = assign103390_e155326_d_n10;
        locals.var_rsd_dn13 = assign103390_e155326_d_n13;

        let (assign103400_e155335, assign103400_e155335_d_n0, assign103400_e155335_d_n2, assign103400_e155335_d_n4, assign103400_e155335_d_n5, assign103400_e155335_d_n6, assign103400_e155335_d_n7, assign103400_e155335_d_n8, assign103400_e155335_d_n9, assign103400_e155335_d_n10, assign103400_e155335_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign103400_e155333: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign103400_e155333, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13,)
    }
};
        locals.var_rsde = assign103400_e155335;
        locals.var_rsde_dn0 = assign103400_e155335_d_n0;
        locals.var_rsde_dn2 = assign103400_e155335_d_n2;
        locals.var_rsde_dn4 = assign103400_e155335_d_n4;
        locals.var_rsde_dn5 = assign103400_e155335_d_n5;
        locals.var_rsde_dn6 = assign103400_e155335_d_n6;
        locals.var_rsde_dn7 = assign103400_e155335_d_n7;
        locals.var_rsde_dn8 = assign103400_e155335_d_n8;
        locals.var_rsde_dn9 = assign103400_e155335_d_n9;
        locals.var_rsde_dn10 = assign103400_e155335_d_n10;
        locals.var_rsde_dn13 = assign103400_e155335_d_n13;

        let assign103410_e155338: f64 = if locals.var_flg_rd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2356 = assign103410_e155338;

        let (assign103420_e155345, assign103420_e155345_d_n5, assign103420_e155345_d_n7,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_vdsi, locals.var_vdsi_dn5, locals.var_vdsi_dn7,)
    } else {
        (locals.var_vds__blk2352, locals.var_vds__blk2352_dn5, locals.var_vds__blk2352_dn7,)
    }
};
        locals.var_vds__blk2352 = assign103420_e155345;
        locals.var_vds__blk2352_dn5 = assign103420_e155345_d_n5;
        locals.var_vds__blk2352_dn7 = assign103420_e155345_d_n7;

        let (assign103430_e155352, assign103430_e155352_d_n7, assign103430_e155352_d_n8,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_vbsi, locals.var_vbsi_dn7, locals.var_vbsi_dn8,)
    } else {
        (locals.var_vbs__blk2353, locals.var_vbs__blk2353_dn7, locals.var_vbs__blk2353_dn8,)
    }
};
        locals.var_vbs__blk2353 = assign103430_e155352;
        locals.var_vbs__blk2353_dn7 = assign103430_e155352_d_n7;
        locals.var_vbs__blk2353_dn8 = assign103430_e155352_d_n8;

        let assign103440_e155359: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2357 = assign103440_e155359;

        let (assign103450_e155375, assign103450_e155375_d_n0, assign103450_e155375_d_n2, assign103450_e155375_d_n4, assign103450_e155375_d_n5, assign103450_e155375_d_n6, assign103450_e155375_d_n7, assign103450_e155375_d_n8, assign103450_e155375_d_n9, assign103450_e155375_d_n10, assign103450_e155375_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let (assign103450_e155373, assign103450_e155373_d_n0, assign103450_e155373_d_n2, assign103450_e155373_d_n4, assign103450_e155373_d_n5, assign103450_e155373_d_n6, assign103450_e155373_d_n7, assign103450_e155373_d_n8, assign103450_e155373_d_n9, assign103450_e155373_d_n10, assign103450_e155373_d_n13,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103450_e155372: f64 = (locals.var_tratio).powf(p.p415);
                (assign103450_e155372, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn13)) } } else { (assign103450_e155372 * (p.p415 * (locals.var_tratio_dn13 / locals.var_tratio))) },)
            }
        };
        (assign103450_e155373, assign103450_e155373_d_n0, assign103450_e155373_d_n2, assign103450_e155373_d_n4, assign103450_e155373_d_n5, assign103450_e155373_d_n6, assign103450_e155373_d_n7, assign103450_e155373_d_n8, assign103450_e155373_d_n9, assign103450_e155373_d_n10, assign103450_e155373_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103450_e155375;
        locals.var_t1_dn0 = assign103450_e155375_d_n0;
        locals.var_t1_dn2 = assign103450_e155375_d_n2;
        locals.var_t1_dn4 = assign103450_e155375_d_n4;
        locals.var_t1_dn5 = assign103450_e155375_d_n5;
        locals.var_t1_dn6 = assign103450_e155375_d_n6;
        locals.var_t1_dn7 = assign103450_e155375_d_n7;
        locals.var_t1_dn8 = assign103450_e155375_d_n8;
        locals.var_t1_dn9 = assign103450_e155375_d_n9;
        locals.var_t1_dn10 = assign103450_e155375_d_n10;
        locals.var_t1_dn13 = assign103450_e155375_d_n13;

        let (assign103460_e155386, assign103460_e155386_d_n0, assign103460_e155386_d_n2, assign103460_e155386_d_n4, assign103460_e155386_d_n5, assign103460_e155386_d_n6, assign103460_e155386_d_n7, assign103460_e155386_d_n8, assign103460_e155386_d_n9, assign103460_e155386_d_n10, assign103460_e155386_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103460_e155384: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign103460_e155384, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn13,)
    }
};
        locals.var_rrdrmue = assign103460_e155386;
        locals.var_rrdrmue_dn0 = assign103460_e155386_d_n0;
        locals.var_rrdrmue_dn2 = assign103460_e155386_d_n2;
        locals.var_rrdrmue_dn4 = assign103460_e155386_d_n4;
        locals.var_rrdrmue_dn5 = assign103460_e155386_d_n5;
        locals.var_rrdrmue_dn6 = assign103460_e155386_d_n6;
        locals.var_rrdrmue_dn7 = assign103460_e155386_d_n7;
        locals.var_rrdrmue_dn8 = assign103460_e155386_d_n8;
        locals.var_rrdrmue_dn9 = assign103460_e155386_d_n9;
        locals.var_rrdrmue_dn10 = assign103460_e155386_d_n10;
        locals.var_rrdrmue_dn13 = assign103460_e155386_d_n13;

        let (assign103470_e155411, assign103470_e155411_d_n0, assign103470_e155411_d_n2, assign103470_e155411_d_n4, assign103470_e155411_d_n5, assign103470_e155411_d_n6, assign103470_e155411_d_n7, assign103470_e155411_d_n8, assign103470_e155411_d_n9, assign103470_e155411_d_n10, assign103470_e155411_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103470_e155396: f64 = (0.4 * locals.var_tratio);
        let assign103470_e155397: f64 = (1.8 + assign103470_e155396);
        let assign103470_e155400: f64 = (0.1 * locals.var_tratio);
        let assign103470_e155402: f64 = (assign103470_e155400 * locals.var_tratio);
        let assign103470_e155403: f64 = (assign103470_e155397 + assign103470_e155402);
        let assign103470_e155407: f64 = (1.0 - locals.var_tratio);
        let assign103470_e155408: f64 = (p.p417 * assign103470_e155407);
        let assign103470_e155409: f64 = (assign103470_e155403 - assign103470_e155408);
        (assign103470_e155409, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign103470_e155400 * locals.var_tratio_dn13))) - (p.p417 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign103470_e155411;
        locals.var_t0_dn0 = assign103470_e155411_d_n0;
        locals.var_t0_dn2 = assign103470_e155411_d_n2;
        locals.var_t0_dn4 = assign103470_e155411_d_n4;
        locals.var_t0_dn5 = assign103470_e155411_d_n5;
        locals.var_t0_dn6 = assign103470_e155411_d_n6;
        locals.var_t0_dn7 = assign103470_e155411_d_n7;
        locals.var_t0_dn8 = assign103470_e155411_d_n8;
        locals.var_t0_dn9 = assign103470_e155411_d_n9;
        locals.var_t0_dn10 = assign103470_e155411_d_n10;
        locals.var_t0_dn13 = assign103470_e155411_d_n13;

    }

    pub(super) fn stamp_transient_block_366(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign103480_e155422, assign103480_e155422_d_n0, assign103480_e155422_d_n2, assign103480_e155422_d_n4, assign103480_e155422_d_n5, assign103480_e155422_d_n6, assign103480_e155422_d_n7, assign103480_e155422_d_n8, assign103480_e155422_d_n9, assign103480_e155422_d_n10, assign103480_e155422_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103480_e155420: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign103480_e155420, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn13,)
    }
};
        locals.var_rrdrvmax = assign103480_e155422;
        locals.var_rrdrvmax_dn0 = assign103480_e155422_d_n0;
        locals.var_rrdrvmax_dn2 = assign103480_e155422_d_n2;
        locals.var_rrdrvmax_dn4 = assign103480_e155422_d_n4;
        locals.var_rrdrvmax_dn5 = assign103480_e155422_d_n5;
        locals.var_rrdrvmax_dn6 = assign103480_e155422_d_n6;
        locals.var_rrdrvmax_dn7 = assign103480_e155422_d_n7;
        locals.var_rrdrvmax_dn8 = assign103480_e155422_d_n8;
        locals.var_rrdrvmax_dn9 = assign103480_e155422_d_n9;
        locals.var_rrdrvmax_dn10 = assign103480_e155422_d_n10;
        locals.var_rrdrvmax_dn13 = assign103480_e155422_d_n13;

        let (assign103490_e155437, assign103490_e155437_d_n0, assign103490_e155437_d_n2, assign103490_e155437_d_n4, assign103490_e155437_d_n5, assign103490_e155437_d_n6, assign103490_e155437_d_n7, assign103490_e155437_d_n8, assign103490_e155437_d_n9, assign103490_e155437_d_n10, assign103490_e155437_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) {
        let assign103490_e155433: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign103490_e155434: f64 = (p.p438 * assign103490_e155433);
        let assign103490_e155435: f64 = (locals.var_uc_rdrbb + assign103490_e155434);
        (assign103490_e155435, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn13 + (p.p438 * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign103490_e155437;
        locals.var_uc_rdrbb_dn0 = assign103490_e155437_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103490_e155437_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103490_e155437_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103490_e155437_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103490_e155437_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103490_e155437_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103490_e155437_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103490_e155437_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103490_e155437_d_n10;
        locals.var_uc_rdrbb_dn13 = assign103490_e155437_d_n13;

        let assign103510_e155445: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard2359 = assign103510_e155445;

        let (assign103520_e155456, assign103520_e155456_d_n0, assign103520_e155456_d_n2, assign103520_e155456_d_n4, assign103520_e155456_d_n5, assign103520_e155456_d_n6, assign103520_e155456_d_n7, assign103520_e155456_d_n8, assign103520_e155456_d_n9, assign103520_e155456_d_n10, assign103520_e155456_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 != 0.0)) && (locals.var_guard2359 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn13,)
    }
};
        locals.var_uc_rdrbb = assign103520_e155456;
        locals.var_uc_rdrbb_dn0 = assign103520_e155456_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103520_e155456_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103520_e155456_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103520_e155456_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103520_e155456_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103520_e155456_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103520_e155456_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103520_e155456_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103520_e155456_d_n10;
        locals.var_uc_rdrbb_dn13 = assign103520_e155456_d_n13;

        let (assign103530_e155468, assign103530_e155468_d_n0, assign103530_e155468_d_n2, assign103530_e155468_d_n4, assign103530_e155468_d_n5, assign103530_e155468_d_n6, assign103530_e155468_d_n7, assign103530_e155468_d_n8, assign103530_e155468_d_n9, assign103530_e155468_d_n10, assign103530_e155468_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2357 == 0.0)) {
        let assign103530_e155464: f64 = ctx_temp;
        let assign103530_e155466: f64 = (assign103530_e155464 + p.p11);
        (assign103530_e155466, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign103530_e155468;
        locals.var_ttemp_dn0 = assign103530_e155468_d_n0;
        locals.var_ttemp_dn2 = assign103530_e155468_d_n2;
        locals.var_ttemp_dn4 = assign103530_e155468_d_n4;
        locals.var_ttemp_dn5 = assign103530_e155468_d_n5;
        locals.var_ttemp_dn6 = assign103530_e155468_d_n6;
        locals.var_ttemp_dn7 = assign103530_e155468_d_n7;
        locals.var_ttemp_dn8 = assign103530_e155468_d_n8;
        locals.var_ttemp_dn9 = assign103530_e155468_d_n9;
        locals.var_ttemp_dn10 = assign103530_e155468_d_n10;
        locals.var_ttemp_dn13 = assign103530_e155468_d_n13;

        let (assign103540_e155477,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103540_e155475: f64 = (locals.var_weff_ld * p.p7);
        (assign103540_e155475,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign103540_e155477;

        let (assign103550_e155486,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103550_e155484: f64 = (p.p67 + p.p68);
        (assign103550_e155484,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign103550_e155486;

        let (assign103560_e155495,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103560_e155493: f64 = (locals.var_uc_xldld + 1e-12);
        (assign103560_e155493,)
    } else {
        (locals.var_rd_xldld,)
    }
};
        locals.var_rd_xldld = assign103560_e155495;

        let (assign103570_e155502,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_noverd,)
    }
};
        locals.var_noverd = assign103570_e155502;

        let (assign103580_e155517, assign103580_e155517_d_n0, assign103580_e155517_d_n2, assign103580_e155517_d_n4, assign103580_e155517_d_n5, assign103580_e155517_d_n6, assign103580_e155517_d_n7, assign103580_e155517_d_n8, assign103580_e155517_d_n9, assign103580_e155517_d_n10, assign103580_e155517_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103580_e155512: f64 = (p.p411 * locals.var_vbs__blk2353);
        let assign103580_e155513: f64 = (p.p410 - assign103580_e155512);
        let assign103580_e155514: f64 = (locals.var_vbs__blk2353 * assign103580_e155513);
        let assign103580_e155515: f64 = (1.0 + assign103580_e155514);
        (assign103580_e155515, 0.0, 0.0, 0.0, 0.0, 0.0, ((locals.var_vbs__blk2353_dn7 * assign103580_e155513) + (locals.var_vbs__blk2353 * (-(p.p411 * locals.var_vbs__blk2353_dn7)))), ((locals.var_vbs__blk2353_dn8 * assign103580_e155513) + (locals.var_vbs__blk2353 * (-(p.p411 * locals.var_vbs__blk2353_dn8)))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103580_e155517;
        locals.var_t1_dn0 = assign103580_e155517_d_n0;
        locals.var_t1_dn2 = assign103580_e155517_d_n2;
        locals.var_t1_dn4 = assign103580_e155517_d_n4;
        locals.var_t1_dn5 = assign103580_e155517_d_n5;
        locals.var_t1_dn6 = assign103580_e155517_d_n6;
        locals.var_t1_dn7 = assign103580_e155517_d_n7;
        locals.var_t1_dn8 = assign103580_e155517_d_n8;
        locals.var_t1_dn9 = assign103580_e155517_d_n9;
        locals.var_t1_dn10 = assign103580_e155517_d_n10;
        locals.var_t1_dn13 = assign103580_e155517_d_n13;

        let (assign103590_e155533, assign103590_e155533_d_n0, assign103590_e155533_d_n2, assign103590_e155533_d_n4, assign103590_e155533_d_n5, assign103590_e155533_d_n6, assign103590_e155533_d_n7, assign103590_e155533_d_n8, assign103590_e155533_d_n9, assign103590_e155533_d_n10, assign103590_e155533_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103590_e155524: f64 = (locals.var_t1 * locals.var_t1);
        let assign103590_e155527: f64 = (4.0 * 0.1);
        let assign103590_e155529: f64 = (assign103590_e155527 * 0.1);
        let assign103590_e155530: f64 = (assign103590_e155524 + assign103590_e155529);
        let assign103590_e155531: f64 = (assign103590_e155530).sqrt();
        (assign103590_e155531, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign103590_e155531)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign103590_e155531)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103590_e155533;
        locals.var_tmf2_dn0 = assign103590_e155533_d_n0;
        locals.var_tmf2_dn2 = assign103590_e155533_d_n2;
        locals.var_tmf2_dn4 = assign103590_e155533_d_n4;
        locals.var_tmf2_dn5 = assign103590_e155533_d_n5;
        locals.var_tmf2_dn6 = assign103590_e155533_d_n6;
        locals.var_tmf2_dn7 = assign103590_e155533_d_n7;
        locals.var_tmf2_dn8 = assign103590_e155533_d_n8;
        locals.var_tmf2_dn9 = assign103590_e155533_d_n9;
        locals.var_tmf2_dn10 = assign103590_e155533_d_n10;
        locals.var_tmf2_dn13 = assign103590_e155533_d_n13;

        let (assign103600_e155546, assign103600_e155546_d_n0, assign103600_e155546_d_n2, assign103600_e155546_d_n4, assign103600_e155546_d_n5, assign103600_e155546_d_n6, assign103600_e155546_d_n7, assign103600_e155546_d_n8, assign103600_e155546_d_n9, assign103600_e155546_d_n10, assign103600_e155546_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103600_e155542: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign103600_e155543: f64 = (1.0 + assign103600_e155542);
        let assign103600_e155544: f64 = (0.5 * assign103600_e155543);
        (assign103600_e155544, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103600_e155546;
        locals.var_t2_dn0 = assign103600_e155546_d_n0;
        locals.var_t2_dn2 = assign103600_e155546_d_n2;
        locals.var_t2_dn4 = assign103600_e155546_d_n4;
        locals.var_t2_dn5 = assign103600_e155546_d_n5;
        locals.var_t2_dn6 = assign103600_e155546_d_n6;
        locals.var_t2_dn7 = assign103600_e155546_d_n7;
        locals.var_t2_dn8 = assign103600_e155546_d_n8;
        locals.var_t2_dn9 = assign103600_e155546_d_n9;
        locals.var_t2_dn10 = assign103600_e155546_d_n10;
        locals.var_t2_dn13 = assign103600_e155546_d_n13;

        let (assign103610_e155557, assign103610_e155557_d_n0, assign103610_e155557_d_n2, assign103610_e155557_d_n4, assign103610_e155557_d_n5, assign103610_e155557_d_n6, assign103610_e155557_d_n7, assign103610_e155557_d_n8, assign103610_e155557_d_n9, assign103610_e155557_d_n10, assign103610_e155557_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103610_e155554: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign103610_e155555: f64 = (0.5 * assign103610_e155554);
        (assign103610_e155555, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign103610_e155557;
        locals.var_rdrmuevbs_dn0 = assign103610_e155557_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103610_e155557_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103610_e155557_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103610_e155557_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103610_e155557_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103610_e155557_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103610_e155557_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103610_e155557_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103610_e155557_d_n10;
        locals.var_rdrmuevbs_dn13 = assign103610_e155557_d_n13;

        let assign103620_e155560: f64 = if locals.var_rdrmuevbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2360 = assign103620_e155560;

        let (assign103630_e155569, assign103630_e155569_d_n0, assign103630_e155569_d_n2, assign103630_e155569_d_n4, assign103630_e155569_d_n5, assign103630_e155569_d_n6, assign103630_e155569_d_n7, assign103630_e155569_d_n8, assign103630_e155569_d_n9, assign103630_e155569_d_n10, assign103630_e155569_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2360 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign103630_e155569;
        locals.var_rdrmuevbs_dn0 = assign103630_e155569_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103630_e155569_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103630_e155569_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103630_e155569_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103630_e155569_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103630_e155569_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103630_e155569_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103630_e155569_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103630_e155569_d_n10;
        locals.var_rdrmuevbs_dn13 = assign103630_e155569_d_n13;

        let (assign103640_e155578, assign103640_e155578_d_n0, assign103640_e155578_d_n2, assign103640_e155578_d_n4, assign103640_e155578_d_n5, assign103640_e155578_d_n6, assign103640_e155578_d_n7, assign103640_e155578_d_n8, assign103640_e155578_d_n9, assign103640_e155578_d_n10, assign103640_e155578_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2360 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103640_e155578;
        locals.var_t2_dn0 = assign103640_e155578_d_n0;
        locals.var_t2_dn2 = assign103640_e155578_d_n2;
        locals.var_t2_dn4 = assign103640_e155578_d_n4;
        locals.var_t2_dn5 = assign103640_e155578_d_n5;
        locals.var_t2_dn6 = assign103640_e155578_d_n6;
        locals.var_t2_dn7 = assign103640_e155578_d_n7;
        locals.var_t2_dn8 = assign103640_e155578_d_n8;
        locals.var_t2_dn9 = assign103640_e155578_d_n9;
        locals.var_t2_dn10 = assign103640_e155578_d_n10;
        locals.var_t2_dn13 = assign103640_e155578_d_n13;

        let (assign103650_e155589, assign103650_e155589_d_n0, assign103650_e155589_d_n2, assign103650_e155589_d_n4, assign103650_e155589_d_n5, assign103650_e155589_d_n6, assign103650_e155589_d_n7, assign103650_e155589_d_n8, assign103650_e155589_d_n9, assign103650_e155589_d_n10, assign103650_e155589_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103650_e155585: f64 = (locals.var_rrdrmue * locals.var_rdrmuele);
        let assign103650_e155587: f64 = (assign103650_e155585 * locals.var_rdrmuevbs);
        (assign103650_e155587, (((locals.var_rrdrmue_dn0 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn0)), (((locals.var_rrdrmue_dn2 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn2)), (((locals.var_rrdrmue_dn4 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn4)), (((locals.var_rrdrmue_dn5 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn5)), (((locals.var_rrdrmue_dn6 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn6)), (((locals.var_rrdrmue_dn7 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn7)), (((locals.var_rrdrmue_dn8 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn8)), (((locals.var_rrdrmue_dn9 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn9)), (((locals.var_rrdrmue_dn10 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn10)), (((locals.var_rrdrmue_dn13 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103650_e155585 * locals.var_rdrmuevbs_dn13)),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn8, locals.var_mu0_dn9, locals.var_mu0_dn10, locals.var_mu0_dn13,)
    }
};
        locals.var_mu0 = assign103650_e155589;
        locals.var_mu0_dn0 = assign103650_e155589_d_n0;
        locals.var_mu0_dn2 = assign103650_e155589_d_n2;
        locals.var_mu0_dn4 = assign103650_e155589_d_n4;
        locals.var_mu0_dn5 = assign103650_e155589_d_n5;
        locals.var_mu0_dn6 = assign103650_e155589_d_n6;
        locals.var_mu0_dn7 = assign103650_e155589_d_n7;
        locals.var_mu0_dn8 = assign103650_e155589_d_n8;
        locals.var_mu0_dn9 = assign103650_e155589_d_n9;
        locals.var_mu0_dn10 = assign103650_e155589_d_n10;
        locals.var_mu0_dn13 = assign103650_e155589_d_n13;

        let (assign103660_e155602, assign103660_e155602_d_n0, assign103660_e155602_d_n2, assign103660_e155602_d_n4, assign103660_e155602_d_n5, assign103660_e155602_d_n6, assign103660_e155602_d_n7, assign103660_e155602_d_n8, assign103660_e155602_d_n9, assign103660_e155602_d_n10, assign103660_e155602_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103660_e155596: f64 = (locals.var_rrdrvmax * locals.var_rdrvmaxwe);
        let assign103660_e155598: f64 = (assign103660_e155596 * locals.var_rdrvmaxle);
        let assign103660_e155600: f64 = (assign103660_e155598 + 1e-25);
        (assign103660_e155600, ((locals.var_rrdrvmax_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn13 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk2355, locals.var_vmaxe__blk2355_dn0, locals.var_vmaxe__blk2355_dn2, locals.var_vmaxe__blk2355_dn4, locals.var_vmaxe__blk2355_dn5, locals.var_vmaxe__blk2355_dn6, locals.var_vmaxe__blk2355_dn7, locals.var_vmaxe__blk2355_dn8, locals.var_vmaxe__blk2355_dn9, locals.var_vmaxe__blk2355_dn10, locals.var_vmaxe__blk2355_dn13,)
    }
};
        locals.var_vmaxe__blk2355 = assign103660_e155602;
        locals.var_vmaxe__blk2355_dn0 = assign103660_e155602_d_n0;
        locals.var_vmaxe__blk2355_dn2 = assign103660_e155602_d_n2;
        locals.var_vmaxe__blk2355_dn4 = assign103660_e155602_d_n4;
        locals.var_vmaxe__blk2355_dn5 = assign103660_e155602_d_n5;
        locals.var_vmaxe__blk2355_dn6 = assign103660_e155602_d_n6;
        locals.var_vmaxe__blk2355_dn7 = assign103660_e155602_d_n7;
        locals.var_vmaxe__blk2355_dn8 = assign103660_e155602_d_n8;
        locals.var_vmaxe__blk2355_dn9 = assign103660_e155602_d_n9;
        locals.var_vmaxe__blk2355_dn10 = assign103660_e155602_d_n10;
        locals.var_vmaxe__blk2355_dn13 = assign103660_e155602_d_n13;

        let (assign103670_e155609,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (locals.var_uc_rdrcx,)
    } else {
        (locals.var_cx,)
    }
};
        locals.var_cx = assign103670_e155609;

        let (assign103680_e155616,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        (p.p421,)
    } else {
        (locals.var_car,)
    }
};
        locals.var_car = assign103680_e155616;

        let (assign103690_e155625, assign103690_e155625_d_n0, assign103690_e155625_d_n2, assign103690_e155625_d_n4, assign103690_e155625_d_n5, assign103690_e155625_d_n6, assign103690_e155625_d_n7, assign103690_e155625_d_n8, assign103690_e155625_d_n9, assign103690_e155625_d_n10, assign103690_e155625_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103690_e155623: f64 = (locals.var_mu0 * 10000.0);
        (assign103690_e155623, (locals.var_mu0_dn0 * 10000.0), (locals.var_mu0_dn2 * 10000.0), (locals.var_mu0_dn4 * 10000.0), (locals.var_mu0_dn5 * 10000.0), (locals.var_mu0_dn6 * 10000.0), (locals.var_mu0_dn7 * 10000.0), (locals.var_mu0_dn8 * 10000.0), (locals.var_mu0_dn9 * 10000.0), (locals.var_mu0_dn10 * 10000.0), (locals.var_mu0_dn13 * 10000.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103690_e155625;
        locals.var_t1_dn0 = assign103690_e155625_d_n0;
        locals.var_t1_dn2 = assign103690_e155625_d_n2;
        locals.var_t1_dn4 = assign103690_e155625_d_n4;
        locals.var_t1_dn5 = assign103690_e155625_d_n5;
        locals.var_t1_dn6 = assign103690_e155625_d_n6;
        locals.var_t1_dn7 = assign103690_e155625_d_n7;
        locals.var_t1_dn8 = assign103690_e155625_d_n8;
        locals.var_t1_dn9 = assign103690_e155625_d_n9;
        locals.var_t1_dn10 = assign103690_e155625_d_n10;
        locals.var_t1_dn13 = assign103690_e155625_d_n13;

        let (assign103700_e155634, assign103700_e155634_d_n0, assign103700_e155634_d_n2, assign103700_e155634_d_n4, assign103700_e155634_d_n5, assign103700_e155634_d_n6, assign103700_e155634_d_n7, assign103700_e155634_d_n8, assign103700_e155634_d_n9, assign103700_e155634_d_n10, assign103700_e155634_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103700_e155632: f64 = (locals.var_vmaxe__blk2355 * 100.0);
        (assign103700_e155632, (locals.var_vmaxe__blk2355_dn0 * 100.0), (locals.var_vmaxe__blk2355_dn2 * 100.0), (locals.var_vmaxe__blk2355_dn4 * 100.0), (locals.var_vmaxe__blk2355_dn5 * 100.0), (locals.var_vmaxe__blk2355_dn6 * 100.0), (locals.var_vmaxe__blk2355_dn7 * 100.0), (locals.var_vmaxe__blk2355_dn8 * 100.0), (locals.var_vmaxe__blk2355_dn9 * 100.0), (locals.var_vmaxe__blk2355_dn10 * 100.0), (locals.var_vmaxe__blk2355_dn13 * 100.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103700_e155634;
        locals.var_t2_dn0 = assign103700_e155634_d_n0;
        locals.var_t2_dn2 = assign103700_e155634_d_n2;
        locals.var_t2_dn4 = assign103700_e155634_d_n4;
        locals.var_t2_dn5 = assign103700_e155634_d_n5;
        locals.var_t2_dn6 = assign103700_e155634_d_n6;
        locals.var_t2_dn7 = assign103700_e155634_d_n7;
        locals.var_t2_dn8 = assign103700_e155634_d_n8;
        locals.var_t2_dn9 = assign103700_e155634_d_n9;
        locals.var_t2_dn10 = assign103700_e155634_d_n10;
        locals.var_t2_dn13 = assign103700_e155634_d_n13;

        let assign103730_e155655: f64 = if locals.var_vddp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2363 = assign103730_e155655;

        let (assign103740_e155671, assign103740_e155671_d_n0, assign103740_e155671_d_n2, assign103740_e155671_d_n4, assign103740_e155671_d_n5, assign103740_e155671_d_n6, assign103740_e155671_d_n7, assign103740_e155671_d_n8, assign103740_e155671_d_n9, assign103740_e155671_d_n10, assign103740_e155671_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103740_e155664: f64 = (-locals.var_vddp);
        let assign103740_e155666: f64 = (assign103740_e155664 / 2.0);
        let assign103740_e155667: f64 = (2.0 * assign103740_e155666);
        let assign103740_e155669: f64 = (assign103740_e155667 / p.p262);
        (assign103740_e155669, ((2.0 * ((-locals.var_vddp_dn0) / 2.0)) / p.p262), 0.0, 0.0, ((2.0 * ((-locals.var_vddp_dn5) / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign103740_e155671;
        locals.var_tmf1_dn0 = assign103740_e155671_d_n0;
        locals.var_tmf1_dn2 = assign103740_e155671_d_n2;
        locals.var_tmf1_dn4 = assign103740_e155671_d_n4;
        locals.var_tmf1_dn5 = assign103740_e155671_d_n5;
        locals.var_tmf1_dn6 = assign103740_e155671_d_n6;
        locals.var_tmf1_dn7 = assign103740_e155671_d_n7;
        locals.var_tmf1_dn8 = assign103740_e155671_d_n8;
        locals.var_tmf1_dn9 = assign103740_e155671_d_n9;
        locals.var_tmf1_dn10 = assign103740_e155671_d_n10;
        locals.var_tmf1_dn13 = assign103740_e155671_d_n13;

        let (assign103750_e155716, assign103750_e155716_d_n0, assign103750_e155716_d_n2, assign103750_e155716_d_n4, assign103750_e155716_d_n5, assign103750_e155716_d_n6, assign103750_e155716_d_n7, assign103750_e155716_d_n8, assign103750_e155716_d_n9, assign103750_e155716_d_n10, assign103750_e155716_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103750_e155682: f64 = (1.0 / 2.0);
        let assign103750_e155686: f64 = (1.0 / 6.0);
        let assign103750_e155690: f64 = (1.0 / 24.0);
        let assign103750_e155694: f64 = (1.0 / 120.0);
        let assign103750_e155698: f64 = (1.0 / 720.0);
        let assign103750_e155702: f64 = (1.0 / 5040.0);
        let assign103750_e155703: f64 = (locals.var_tmf1 * assign103750_e155702);
        let assign103750_e155704: f64 = (assign103750_e155698 + assign103750_e155703);
        let assign103750_e155705: f64 = (locals.var_tmf1 * assign103750_e155704);
        let assign103750_e155706: f64 = (assign103750_e155694 + assign103750_e155705);
        let assign103750_e155707: f64 = (locals.var_tmf1 * assign103750_e155706);
        let assign103750_e155708: f64 = (assign103750_e155690 + assign103750_e155707);
        let assign103750_e155709: f64 = (locals.var_tmf1 * assign103750_e155708);
        let assign103750_e155710: f64 = (assign103750_e155686 + assign103750_e155709);
        let assign103750_e155711: f64 = (locals.var_tmf1 * assign103750_e155710);
        let assign103750_e155712: f64 = (assign103750_e155682 + assign103750_e155711);
        let assign103750_e155713: f64 = (locals.var_tmf1 * assign103750_e155712);
        let assign103750_e155714: f64 = (1.0 + assign103750_e155713);
        (assign103750_e155714, ((locals.var_tmf1_dn0 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn2 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn4 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn5 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn6 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn7 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn8 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn9 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn10 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103750_e155702))))))))))), ((locals.var_tmf1_dn13 * assign103750_e155712) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155710) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155708) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155706) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103750_e155704) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103750_e155702))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103750_e155716;
        locals.var_tmf2_dn0 = assign103750_e155716_d_n0;
        locals.var_tmf2_dn2 = assign103750_e155716_d_n2;
        locals.var_tmf2_dn4 = assign103750_e155716_d_n4;
        locals.var_tmf2_dn5 = assign103750_e155716_d_n5;
        locals.var_tmf2_dn6 = assign103750_e155716_d_n6;
        locals.var_tmf2_dn7 = assign103750_e155716_d_n7;
        locals.var_tmf2_dn8 = assign103750_e155716_d_n8;
        locals.var_tmf2_dn9 = assign103750_e155716_d_n9;
        locals.var_tmf2_dn10 = assign103750_e155716_d_n10;
        locals.var_tmf2_dn13 = assign103750_e155716_d_n13;

        let (assign103760_e155757, assign103760_e155757_d_n0, assign103760_e155757_d_n2, assign103760_e155757_d_n4, assign103760_e155757_d_n5, assign103760_e155757_d_n6, assign103760_e155757_d_n7, assign103760_e155757_d_n8, assign103760_e155757_d_n9, assign103760_e155757_d_n10, assign103760_e155757_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103760_e155725: f64 = (1.0 / 2.0);
        let assign103760_e155729: f64 = (1.0 / 3.0);
        let assign103760_e155733: f64 = (1.0 / 8.0);
        let assign103760_e155737: f64 = (1.0 / 30.0);
        let assign103760_e155741: f64 = (1.0 / 144.0);
        let assign103760_e155745: f64 = (1.0 / 840.0);
        let assign103760_e155746: f64 = (locals.var_tmf1 * assign103760_e155745);
        let assign103760_e155747: f64 = (assign103760_e155741 + assign103760_e155746);
        let assign103760_e155748: f64 = (locals.var_tmf1 * assign103760_e155747);
        let assign103760_e155749: f64 = (assign103760_e155737 + assign103760_e155748);
        let assign103760_e155750: f64 = (locals.var_tmf1 * assign103760_e155749);
        let assign103760_e155751: f64 = (assign103760_e155733 + assign103760_e155750);
        let assign103760_e155752: f64 = (locals.var_tmf1 * assign103760_e155751);
        let assign103760_e155753: f64 = (assign103760_e155729 + assign103760_e155752);
        let assign103760_e155754: f64 = (locals.var_tmf1 * assign103760_e155753);
        let assign103760_e155755: f64 = (assign103760_e155725 + assign103760_e155754);
        (assign103760_e155755, ((locals.var_tmf1_dn0 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103760_e155745))))))))), ((locals.var_tmf1_dn2 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103760_e155745))))))))), ((locals.var_tmf1_dn4 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103760_e155745))))))))), ((locals.var_tmf1_dn5 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103760_e155745))))))))), ((locals.var_tmf1_dn6 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103760_e155745))))))))), ((locals.var_tmf1_dn7 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103760_e155745))))))))), ((locals.var_tmf1_dn8 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103760_e155745))))))))), ((locals.var_tmf1_dn9 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103760_e155745))))))))), ((locals.var_tmf1_dn10 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103760_e155745))))))))), ((locals.var_tmf1_dn13 * assign103760_e155753) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155751) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155749) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103760_e155747) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103760_e155745))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign103760_e155757;
        locals.var_tmf3_dn0 = assign103760_e155757_d_n0;
        locals.var_tmf3_dn2 = assign103760_e155757_d_n2;
        locals.var_tmf3_dn4 = assign103760_e155757_d_n4;
        locals.var_tmf3_dn5 = assign103760_e155757_d_n5;
        locals.var_tmf3_dn6 = assign103760_e155757_d_n6;
        locals.var_tmf3_dn7 = assign103760_e155757_d_n7;
        locals.var_tmf3_dn8 = assign103760_e155757_d_n8;
        locals.var_tmf3_dn9 = assign103760_e155757_d_n9;
        locals.var_tmf3_dn10 = assign103760_e155757_d_n10;
        locals.var_tmf3_dn13 = assign103760_e155757_d_n13;

        let (assign103770_e155768, assign103770_e155768_d_n0, assign103770_e155768_d_n2, assign103770_e155768_d_n4, assign103770_e155768_d_n5, assign103770_e155768_d_n6, assign103770_e155768_d_n7, assign103770_e155768_d_n8, assign103770_e155768_d_n9, assign103770_e155768_d_n10, assign103770_e155768_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103770_e155766: f64 = (p.p262 / locals.var_tmf2);
        (assign103770_e155766, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103770_e155768;
        locals.var_vzadd_dn0 = assign103770_e155768_d_n0;
        locals.var_vzadd_dn2 = assign103770_e155768_d_n2;
        locals.var_vzadd_dn4 = assign103770_e155768_d_n4;
        locals.var_vzadd_dn5 = assign103770_e155768_d_n5;
        locals.var_vzadd_dn6 = assign103770_e155768_d_n6;
        locals.var_vzadd_dn7 = assign103770_e155768_d_n7;
        locals.var_vzadd_dn8 = assign103770_e155768_d_n8;
        locals.var_vzadd_dn9 = assign103770_e155768_d_n9;
        locals.var_vzadd_dn10 = assign103770_e155768_d_n10;
        locals.var_vzadd_dn13 = assign103770_e155768_d_n13;

        let (assign103780_e155784, assign103780_e155784_d_n0, assign103780_e155784_d_n2, assign103780_e155784_d_n4, assign103780_e155784_d_n5, assign103780_e155784_d_n6, assign103780_e155784_d_n7, assign103780_e155784_d_n8, assign103780_e155784_d_n9, assign103780_e155784_d_n10, assign103780_e155784_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103780_e155776: f64 = (-2.0);
        let assign103780_e155778: f64 = (assign103780_e155776 * locals.var_tmf3);
        let assign103780_e155781: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103780_e155782: f64 = (assign103780_e155778 / assign103780_e155781);
        (assign103780_e155782, ((((assign103780_e155776 * locals.var_tmf3_dn0) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn2) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn4) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn5) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn6) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn7) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn8) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn9) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn10) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103780_e155781 * assign103780_e155781)), ((((assign103780_e155776 * locals.var_tmf3_dn13) * assign103780_e155781) - (assign103780_e155778 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign103780_e155781 * assign103780_e155781)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103780_e155784;
        locals.var_t2_dn0 = assign103780_e155784_d_n0;
        locals.var_t2_dn2 = assign103780_e155784_d_n2;
        locals.var_t2_dn4 = assign103780_e155784_d_n4;
        locals.var_t2_dn5 = assign103780_e155784_d_n5;
        locals.var_t2_dn6 = assign103780_e155784_d_n6;
        locals.var_t2_dn7 = assign103780_e155784_d_n7;
        locals.var_t2_dn8 = assign103780_e155784_d_n8;
        locals.var_t2_dn9 = assign103780_e155784_d_n9;
        locals.var_t2_dn10 = assign103780_e155784_d_n10;
        locals.var_t2_dn13 = assign103780_e155784_d_n13;

        let assign103790_e155787: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2364 = assign103790_e155787;

    }

    pub(super) fn stamp_transient_block_367(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103800_e155798, assign103800_e155798_d_n0, assign103800_e155798_d_n2, assign103800_e155798_d_n4, assign103800_e155798_d_n5, assign103800_e155798_d_n6, assign103800_e155798_d_n7, assign103800_e155798_d_n8, assign103800_e155798_d_n9, assign103800_e155798_d_n10, assign103800_e155798_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) && (locals.var_guard2364 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103800_e155798;
        locals.var_vzadd_dn0 = assign103800_e155798_d_n0;
        locals.var_vzadd_dn2 = assign103800_e155798_d_n2;
        locals.var_vzadd_dn4 = assign103800_e155798_d_n4;
        locals.var_vzadd_dn5 = assign103800_e155798_d_n5;
        locals.var_vzadd_dn6 = assign103800_e155798_d_n6;
        locals.var_vzadd_dn7 = assign103800_e155798_d_n7;
        locals.var_vzadd_dn8 = assign103800_e155798_d_n8;
        locals.var_vzadd_dn9 = assign103800_e155798_d_n9;
        locals.var_vzadd_dn10 = assign103800_e155798_d_n10;
        locals.var_vzadd_dn13 = assign103800_e155798_d_n13;

        let (assign103810_e155811, assign103810_e155811_d_n0, assign103810_e155811_d_n2, assign103810_e155811_d_n4, assign103810_e155811_d_n5, assign103810_e155811_d_n6, assign103810_e155811_d_n7, assign103810_e155811_d_n8, assign103810_e155811_d_n9, assign103810_e155811_d_n10, assign103810_e155811_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 != 0.0)) {
        let assign103810_e155808: f64 = (2.0 * locals.var_vzadd);
        let assign103810_e155809: f64 = (locals.var_vddp - assign103810_e155808);
        (assign103810_e155809, (locals.var_vddp_dn0 - (2.0 * locals.var_vzadd_dn0)), (-(2.0 * locals.var_vzadd_dn2)), (-(2.0 * locals.var_vzadd_dn4)), (locals.var_vddp_dn5 - (2.0 * locals.var_vzadd_dn5)), (-(2.0 * locals.var_vzadd_dn6)), (-(2.0 * locals.var_vzadd_dn7)), (-(2.0 * locals.var_vzadd_dn8)), (-(2.0 * locals.var_vzadd_dn9)), (-(2.0 * locals.var_vzadd_dn10)), (-(2.0 * locals.var_vzadd_dn13)),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn13,)
    }
};
        locals.var_vddpz = assign103810_e155811;
        locals.var_vddpz_dn0 = assign103810_e155811_d_n0;
        locals.var_vddpz_dn2 = assign103810_e155811_d_n2;
        locals.var_vddpz_dn4 = assign103810_e155811_d_n4;
        locals.var_vddpz_dn5 = assign103810_e155811_d_n5;
        locals.var_vddpz_dn6 = assign103810_e155811_d_n6;
        locals.var_vddpz_dn7 = assign103810_e155811_d_n7;
        locals.var_vddpz_dn8 = assign103810_e155811_d_n8;
        locals.var_vddpz_dn9 = assign103810_e155811_d_n9;
        locals.var_vddpz_dn10 = assign103810_e155811_d_n10;
        locals.var_vddpz_dn13 = assign103810_e155811_d_n13;

        let (assign103820_e155827, assign103820_e155827_d_n0, assign103820_e155827_d_n2, assign103820_e155827_d_n4, assign103820_e155827_d_n5, assign103820_e155827_d_n6, assign103820_e155827_d_n7, assign103820_e155827_d_n8, assign103820_e155827_d_n9, assign103820_e155827_d_n10, assign103820_e155827_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103820_e155822: f64 = (locals.var_vddp / 2.0);
        let assign103820_e155823: f64 = (2.0 * assign103820_e155822);
        let assign103820_e155825: f64 = (assign103820_e155823 / p.p262);
        (assign103820_e155825, ((2.0 * (locals.var_vddp_dn0 / 2.0)) / p.p262), 0.0, 0.0, ((2.0 * (locals.var_vddp_dn5 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign103820_e155827;
        locals.var_tmf1_dn0 = assign103820_e155827_d_n0;
        locals.var_tmf1_dn2 = assign103820_e155827_d_n2;
        locals.var_tmf1_dn4 = assign103820_e155827_d_n4;
        locals.var_tmf1_dn5 = assign103820_e155827_d_n5;
        locals.var_tmf1_dn6 = assign103820_e155827_d_n6;
        locals.var_tmf1_dn7 = assign103820_e155827_d_n7;
        locals.var_tmf1_dn8 = assign103820_e155827_d_n8;
        locals.var_tmf1_dn9 = assign103820_e155827_d_n9;
        locals.var_tmf1_dn10 = assign103820_e155827_d_n10;
        locals.var_tmf1_dn13 = assign103820_e155827_d_n13;

        let (assign103830_e155873, assign103830_e155873_d_n0, assign103830_e155873_d_n2, assign103830_e155873_d_n4, assign103830_e155873_d_n5, assign103830_e155873_d_n6, assign103830_e155873_d_n7, assign103830_e155873_d_n8, assign103830_e155873_d_n9, assign103830_e155873_d_n10, assign103830_e155873_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103830_e155839: f64 = (1.0 / 2.0);
        let assign103830_e155843: f64 = (1.0 / 6.0);
        let assign103830_e155847: f64 = (1.0 / 24.0);
        let assign103830_e155851: f64 = (1.0 / 120.0);
        let assign103830_e155855: f64 = (1.0 / 720.0);
        let assign103830_e155859: f64 = (1.0 / 5040.0);
        let assign103830_e155860: f64 = (locals.var_tmf1 * assign103830_e155859);
        let assign103830_e155861: f64 = (assign103830_e155855 + assign103830_e155860);
        let assign103830_e155862: f64 = (locals.var_tmf1 * assign103830_e155861);
        let assign103830_e155863: f64 = (assign103830_e155851 + assign103830_e155862);
        let assign103830_e155864: f64 = (locals.var_tmf1 * assign103830_e155863);
        let assign103830_e155865: f64 = (assign103830_e155847 + assign103830_e155864);
        let assign103830_e155866: f64 = (locals.var_tmf1 * assign103830_e155865);
        let assign103830_e155867: f64 = (assign103830_e155843 + assign103830_e155866);
        let assign103830_e155868: f64 = (locals.var_tmf1 * assign103830_e155867);
        let assign103830_e155869: f64 = (assign103830_e155839 + assign103830_e155868);
        let assign103830_e155870: f64 = (locals.var_tmf1 * assign103830_e155869);
        let assign103830_e155871: f64 = (1.0 + assign103830_e155870);
        (assign103830_e155871, ((locals.var_tmf1_dn0 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn2 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn4 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn5 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn6 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn7 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn8 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn9 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn10 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103830_e155859))))))))))), ((locals.var_tmf1_dn13 * assign103830_e155869) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155867) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155865) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155863) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103830_e155861) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103830_e155859))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign103830_e155873;
        locals.var_tmf2_dn0 = assign103830_e155873_d_n0;
        locals.var_tmf2_dn2 = assign103830_e155873_d_n2;
        locals.var_tmf2_dn4 = assign103830_e155873_d_n4;
        locals.var_tmf2_dn5 = assign103830_e155873_d_n5;
        locals.var_tmf2_dn6 = assign103830_e155873_d_n6;
        locals.var_tmf2_dn7 = assign103830_e155873_d_n7;
        locals.var_tmf2_dn8 = assign103830_e155873_d_n8;
        locals.var_tmf2_dn9 = assign103830_e155873_d_n9;
        locals.var_tmf2_dn10 = assign103830_e155873_d_n10;
        locals.var_tmf2_dn13 = assign103830_e155873_d_n13;

        let (assign103840_e155915, assign103840_e155915_d_n0, assign103840_e155915_d_n2, assign103840_e155915_d_n4, assign103840_e155915_d_n5, assign103840_e155915_d_n6, assign103840_e155915_d_n7, assign103840_e155915_d_n8, assign103840_e155915_d_n9, assign103840_e155915_d_n10, assign103840_e155915_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103840_e155883: f64 = (1.0 / 2.0);
        let assign103840_e155887: f64 = (1.0 / 3.0);
        let assign103840_e155891: f64 = (1.0 / 8.0);
        let assign103840_e155895: f64 = (1.0 / 30.0);
        let assign103840_e155899: f64 = (1.0 / 144.0);
        let assign103840_e155903: f64 = (1.0 / 840.0);
        let assign103840_e155904: f64 = (locals.var_tmf1 * assign103840_e155903);
        let assign103840_e155905: f64 = (assign103840_e155899 + assign103840_e155904);
        let assign103840_e155906: f64 = (locals.var_tmf1 * assign103840_e155905);
        let assign103840_e155907: f64 = (assign103840_e155895 + assign103840_e155906);
        let assign103840_e155908: f64 = (locals.var_tmf1 * assign103840_e155907);
        let assign103840_e155909: f64 = (assign103840_e155891 + assign103840_e155908);
        let assign103840_e155910: f64 = (locals.var_tmf1 * assign103840_e155909);
        let assign103840_e155911: f64 = (assign103840_e155887 + assign103840_e155910);
        let assign103840_e155912: f64 = (locals.var_tmf1 * assign103840_e155911);
        let assign103840_e155913: f64 = (assign103840_e155883 + assign103840_e155912);
        (assign103840_e155913, ((locals.var_tmf1_dn0 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103840_e155903))))))))), ((locals.var_tmf1_dn2 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103840_e155903))))))))), ((locals.var_tmf1_dn4 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103840_e155903))))))))), ((locals.var_tmf1_dn5 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103840_e155903))))))))), ((locals.var_tmf1_dn6 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103840_e155903))))))))), ((locals.var_tmf1_dn7 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103840_e155903))))))))), ((locals.var_tmf1_dn8 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103840_e155903))))))))), ((locals.var_tmf1_dn9 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103840_e155903))))))))), ((locals.var_tmf1_dn10 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103840_e155903))))))))), ((locals.var_tmf1_dn13 * assign103840_e155911) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155909) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155907) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign103840_e155905) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign103840_e155903))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign103840_e155915;
        locals.var_tmf3_dn0 = assign103840_e155915_d_n0;
        locals.var_tmf3_dn2 = assign103840_e155915_d_n2;
        locals.var_tmf3_dn4 = assign103840_e155915_d_n4;
        locals.var_tmf3_dn5 = assign103840_e155915_d_n5;
        locals.var_tmf3_dn6 = assign103840_e155915_d_n6;
        locals.var_tmf3_dn7 = assign103840_e155915_d_n7;
        locals.var_tmf3_dn8 = assign103840_e155915_d_n8;
        locals.var_tmf3_dn9 = assign103840_e155915_d_n9;
        locals.var_tmf3_dn10 = assign103840_e155915_d_n10;
        locals.var_tmf3_dn13 = assign103840_e155915_d_n13;

        let (assign103850_e155927, assign103850_e155927_d_n0, assign103850_e155927_d_n2, assign103850_e155927_d_n4, assign103850_e155927_d_n5, assign103850_e155927_d_n6, assign103850_e155927_d_n7, assign103850_e155927_d_n8, assign103850_e155927_d_n9, assign103850_e155927_d_n10, assign103850_e155927_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103850_e155925: f64 = (p.p262 / locals.var_tmf2);
        (assign103850_e155925, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103850_e155927;
        locals.var_vzadd_dn0 = assign103850_e155927_d_n0;
        locals.var_vzadd_dn2 = assign103850_e155927_d_n2;
        locals.var_vzadd_dn4 = assign103850_e155927_d_n4;
        locals.var_vzadd_dn5 = assign103850_e155927_d_n5;
        locals.var_vzadd_dn6 = assign103850_e155927_d_n6;
        locals.var_vzadd_dn7 = assign103850_e155927_d_n7;
        locals.var_vzadd_dn8 = assign103850_e155927_d_n8;
        locals.var_vzadd_dn9 = assign103850_e155927_d_n9;
        locals.var_vzadd_dn10 = assign103850_e155927_d_n10;
        locals.var_vzadd_dn13 = assign103850_e155927_d_n13;

        let (assign103860_e155944, assign103860_e155944_d_n0, assign103860_e155944_d_n2, assign103860_e155944_d_n4, assign103860_e155944_d_n5, assign103860_e155944_d_n6, assign103860_e155944_d_n7, assign103860_e155944_d_n8, assign103860_e155944_d_n9, assign103860_e155944_d_n10, assign103860_e155944_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103860_e155936: f64 = (-2.0);
        let assign103860_e155938: f64 = (assign103860_e155936 * locals.var_tmf3);
        let assign103860_e155941: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103860_e155942: f64 = (assign103860_e155938 / assign103860_e155941);
        (assign103860_e155942, ((((assign103860_e155936 * locals.var_tmf3_dn0) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn2) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn4) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn5) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn6) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn7) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn8) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn9) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn10) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103860_e155941 * assign103860_e155941)), ((((assign103860_e155936 * locals.var_tmf3_dn13) * assign103860_e155941) - (assign103860_e155938 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign103860_e155941 * assign103860_e155941)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign103860_e155944;
        locals.var_t2_dn0 = assign103860_e155944_d_n0;
        locals.var_t2_dn2 = assign103860_e155944_d_n2;
        locals.var_t2_dn4 = assign103860_e155944_d_n4;
        locals.var_t2_dn5 = assign103860_e155944_d_n5;
        locals.var_t2_dn6 = assign103860_e155944_d_n6;
        locals.var_t2_dn7 = assign103860_e155944_d_n7;
        locals.var_t2_dn8 = assign103860_e155944_d_n8;
        locals.var_t2_dn9 = assign103860_e155944_d_n9;
        locals.var_t2_dn10 = assign103860_e155944_d_n10;
        locals.var_t2_dn13 = assign103860_e155944_d_n13;

        let assign103870_e155947: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2365 = assign103870_e155947;

        let (assign103880_e155959, assign103880_e155959_d_n0, assign103880_e155959_d_n2, assign103880_e155959_d_n4, assign103880_e155959_d_n5, assign103880_e155959_d_n6, assign103880_e155959_d_n7, assign103880_e155959_d_n8, assign103880_e155959_d_n9, assign103880_e155959_d_n10, assign103880_e155959_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign103880_e155959;
        locals.var_vzadd_dn0 = assign103880_e155959_d_n0;
        locals.var_vzadd_dn2 = assign103880_e155959_d_n2;
        locals.var_vzadd_dn4 = assign103880_e155959_d_n4;
        locals.var_vzadd_dn5 = assign103880_e155959_d_n5;
        locals.var_vzadd_dn6 = assign103880_e155959_d_n6;
        locals.var_vzadd_dn7 = assign103880_e155959_d_n7;
        locals.var_vzadd_dn8 = assign103880_e155959_d_n8;
        locals.var_vzadd_dn9 = assign103880_e155959_d_n9;
        locals.var_vzadd_dn10 = assign103880_e155959_d_n10;
        locals.var_vzadd_dn13 = assign103880_e155959_d_n13;

        let (assign103890_e155973, assign103890_e155973_d_n0, assign103890_e155973_d_n2, assign103890_e155973_d_n4, assign103890_e155973_d_n5, assign103890_e155973_d_n6, assign103890_e155973_d_n7, assign103890_e155973_d_n8, assign103890_e155973_d_n9, assign103890_e155973_d_n10, assign103890_e155973_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2363 == 0.0)) {
        let assign103890_e155970: f64 = (2.0 * locals.var_vzadd);
        let assign103890_e155971: f64 = (locals.var_vddp + assign103890_e155970);
        (assign103890_e155971, (locals.var_vddp_dn0 + (2.0 * locals.var_vzadd_dn0)), (2.0 * locals.var_vzadd_dn2), (2.0 * locals.var_vzadd_dn4), (locals.var_vddp_dn5 + (2.0 * locals.var_vzadd_dn5)), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn13,)
    }
};
        locals.var_vddpz = assign103890_e155973;
        locals.var_vddpz_dn0 = assign103890_e155973_d_n0;
        locals.var_vddpz_dn2 = assign103890_e155973_d_n2;
        locals.var_vddpz_dn4 = assign103890_e155973_d_n4;
        locals.var_vddpz_dn5 = assign103890_e155973_d_n5;
        locals.var_vddpz_dn6 = assign103890_e155973_d_n6;
        locals.var_vddpz_dn7 = assign103890_e155973_d_n7;
        locals.var_vddpz_dn8 = assign103890_e155973_d_n8;
        locals.var_vddpz_dn9 = assign103890_e155973_d_n9;
        locals.var_vddpz_dn10 = assign103890_e155973_d_n10;
        locals.var_vddpz_dn13 = assign103890_e155973_d_n13;

        let (assign103900_e155982, assign103900_e155982_d_n0, assign103900_e155982_d_n2, assign103900_e155982_d_n4, assign103900_e155982_d_n5, assign103900_e155982_d_n6, assign103900_e155982_d_n7, assign103900_e155982_d_n8, assign103900_e155982_d_n9, assign103900_e155982_d_n10, assign103900_e155982_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103900_e155980: f64 = (locals.var_vddpz / locals.var_ldrifte);
        (assign103900_e155980, (locals.var_vddpz_dn0 / locals.var_ldrifte), (locals.var_vddpz_dn2 / locals.var_ldrifte), (locals.var_vddpz_dn4 / locals.var_ldrifte), (locals.var_vddpz_dn5 / locals.var_ldrifte), (locals.var_vddpz_dn6 / locals.var_ldrifte), (locals.var_vddpz_dn7 / locals.var_ldrifte), (locals.var_vddpz_dn8 / locals.var_ldrifte), (locals.var_vddpz_dn9 / locals.var_ldrifte), (locals.var_vddpz_dn10 / locals.var_ldrifte), (locals.var_vddpz_dn13 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn4, locals.var_edri_dn5, locals.var_edri_dn6, locals.var_edri_dn7, locals.var_edri_dn8, locals.var_edri_dn9, locals.var_edri_dn10, locals.var_edri_dn13,)
    }
};
        locals.var_edri = assign103900_e155982;
        locals.var_edri_dn0 = assign103900_e155982_d_n0;
        locals.var_edri_dn2 = assign103900_e155982_d_n2;
        locals.var_edri_dn4 = assign103900_e155982_d_n4;
        locals.var_edri_dn5 = assign103900_e155982_d_n5;
        locals.var_edri_dn6 = assign103900_e155982_d_n6;
        locals.var_edri_dn7 = assign103900_e155982_d_n7;
        locals.var_edri_dn8 = assign103900_e155982_d_n8;
        locals.var_edri_dn9 = assign103900_e155982_d_n9;
        locals.var_edri_dn10 = assign103900_e155982_d_n10;
        locals.var_edri_dn13 = assign103900_e155982_d_n13;

        let (assign103910_e155991, assign103910_e155991_d_n0, assign103910_e155991_d_n2, assign103910_e155991_d_n4, assign103910_e155991_d_n5, assign103910_e155991_d_n6, assign103910_e155991_d_n7, assign103910_e155991_d_n8, assign103910_e155991_d_n9, assign103910_e155991_d_n10, assign103910_e155991_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign103910_e155989: f64 = (locals.var_mu0 * locals.var_edri);
        (assign103910_e155989, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn4 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn4)), ((locals.var_mu0_dn5 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn5)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), ((locals.var_mu0_dn8 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn8)), ((locals.var_mu0_dn9 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn9)), ((locals.var_mu0_dn10 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn10)), ((locals.var_mu0_dn13 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn13)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn8, locals.var_vdri_dn9, locals.var_vdri_dn10, locals.var_vdri_dn13,)
    }
};
        locals.var_vdri = assign103910_e155991;
        locals.var_vdri_dn0 = assign103910_e155991_d_n0;
        locals.var_vdri_dn2 = assign103910_e155991_d_n2;
        locals.var_vdri_dn4 = assign103910_e155991_d_n4;
        locals.var_vdri_dn5 = assign103910_e155991_d_n5;
        locals.var_vdri_dn6 = assign103910_e155991_d_n6;
        locals.var_vdri_dn7 = assign103910_e155991_d_n7;
        locals.var_vdri_dn8 = assign103910_e155991_d_n8;
        locals.var_vdri_dn9 = assign103910_e155991_d_n9;
        locals.var_vdri_dn10 = assign103910_e155991_d_n10;
        locals.var_vdri_dn13 = assign103910_e155991_d_n13;

        let assign103920_e155994: f64 = if locals.var_vddp >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2366 = assign103920_e155994;

        let (assign103930_e156005, assign103930_e156005_d_n0, assign103930_e156005_d_n2, assign103930_e156005_d_n4, assign103930_e156005_d_n5, assign103930_e156005_d_n6, assign103930_e156005_d_n7, assign103930_e156005_d_n8, assign103930_e156005_d_n9, assign103930_e156005_d_n10, assign103930_e156005_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2366 != 0.0)) {
        let assign103930_e156003: f64 = (locals.var_vdri / locals.var_vmaxe__blk2355);
        (assign103930_e156003, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn0)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn2)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn4)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn5)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn6)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn7)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn8)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn9 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn9)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn10)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), (((locals.var_vdri_dn13 * locals.var_vmaxe__blk2355) - (locals.var_vdri * locals.var_vmaxe__blk2355_dn13)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103930_e156005;
        locals.var_t1_dn0 = assign103930_e156005_d_n0;
        locals.var_t1_dn2 = assign103930_e156005_d_n2;
        locals.var_t1_dn4 = assign103930_e156005_d_n4;
        locals.var_t1_dn5 = assign103930_e156005_d_n5;
        locals.var_t1_dn6 = assign103930_e156005_d_n6;
        locals.var_t1_dn7 = assign103930_e156005_d_n7;
        locals.var_t1_dn8 = assign103930_e156005_d_n8;
        locals.var_t1_dn9 = assign103930_e156005_d_n9;
        locals.var_t1_dn10 = assign103930_e156005_d_n10;
        locals.var_t1_dn13 = assign103930_e156005_d_n13;

        let (assign103940_e156018, assign103940_e156018_d_n0, assign103940_e156018_d_n2, assign103940_e156018_d_n4, assign103940_e156018_d_n5, assign103940_e156018_d_n6, assign103940_e156018_d_n7, assign103940_e156018_d_n8, assign103940_e156018_d_n9, assign103940_e156018_d_n10, assign103940_e156018_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2366 == 0.0)) {
        let assign103940_e156014: f64 = (-locals.var_vdri);
        let assign103940_e156016: f64 = (assign103940_e156014 / locals.var_vmaxe__blk2355);
        (assign103940_e156016, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn0)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn2)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn4)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn5)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn6)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn7)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn8)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn9) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn9)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn10)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)), ((((-locals.var_vdri_dn13) * locals.var_vmaxe__blk2355) - (assign103940_e156014 * locals.var_vmaxe__blk2355_dn13)) / (locals.var_vmaxe__blk2355 * locals.var_vmaxe__blk2355)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign103940_e156018;
        locals.var_t1_dn0 = assign103940_e156018_d_n0;
        locals.var_t1_dn2 = assign103940_e156018_d_n2;
        locals.var_t1_dn4 = assign103940_e156018_d_n4;
        locals.var_t1_dn5 = assign103940_e156018_d_n5;
        locals.var_t1_dn6 = assign103940_e156018_d_n6;
        locals.var_t1_dn7 = assign103940_e156018_d_n7;
        locals.var_t1_dn8 = assign103940_e156018_d_n8;
        locals.var_t1_dn9 = assign103940_e156018_d_n9;
        locals.var_t1_dn10 = assign103940_e156018_d_n10;
        locals.var_t1_dn13 = assign103940_e156018_d_n13;

        let assign103950_e156022: f64 = (10.0 * 2.220446049250313e-16);
        let assign103950_e156023: f64 = (1.0 - assign103950_e156022);
        let assign103950_e156030: f64 = (10.0 * 2.220446049250313e-16);
        let assign103950_e156031: f64 = (1.0 + assign103950_e156030);
        let assign103950_e156033: f64 = if ((assign103950_e156023 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103950_e156031)) { 1.0 } else { 0.0 };
        locals.var_guard2367 = assign103950_e156033;

        let (assign103960_e156042, assign103960_e156042_d_n0, assign103960_e156042_d_n2, assign103960_e156042_d_n4, assign103960_e156042_d_n5, assign103960_e156042_d_n6, assign103960_e156042_d_n7, assign103960_e156042_d_n8, assign103960_e156042_d_n9, assign103960_e156042_d_n10, assign103960_e156042_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103960_e156042;
        locals.var_t3_dn0 = assign103960_e156042_d_n0;
        locals.var_t3_dn2 = assign103960_e156042_d_n2;
        locals.var_t3_dn4 = assign103960_e156042_d_n4;
        locals.var_t3_dn5 = assign103960_e156042_d_n5;
        locals.var_t3_dn6 = assign103960_e156042_d_n6;
        locals.var_t3_dn7 = assign103960_e156042_d_n7;
        locals.var_t3_dn8 = assign103960_e156042_d_n8;
        locals.var_t3_dn9 = assign103960_e156042_d_n9;
        locals.var_t3_dn10 = assign103960_e156042_d_n10;
        locals.var_t3_dn13 = assign103960_e156042_d_n13;

        let assign103970_e156046: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156047: f64 = (2.0 - assign103970_e156046);
        let assign103970_e156054: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156055: f64 = (2.0 + assign103970_e156054);
        let assign103970_e156057: f64 = if ((assign103970_e156047 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103970_e156055)) { 1.0 } else { 0.0 };
        locals.var_guard2368 = assign103970_e156057;

        let (assign103980_e156069, assign103980_e156069_d_n0, assign103980_e156069_d_n2, assign103980_e156069_d_n4, assign103980_e156069_d_n5, assign103980_e156069_d_n6, assign103980_e156069_d_n7, assign103980_e156069_d_n8, assign103980_e156069_d_n9, assign103980_e156069_d_n10, assign103980_e156069_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2368 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103980_e156069;
        locals.var_t3_dn0 = assign103980_e156069_d_n0;
        locals.var_t3_dn2 = assign103980_e156069_d_n2;
        locals.var_t3_dn4 = assign103980_e156069_d_n4;
        locals.var_t3_dn5 = assign103980_e156069_d_n5;
        locals.var_t3_dn6 = assign103980_e156069_d_n6;
        locals.var_t3_dn7 = assign103980_e156069_d_n7;
        locals.var_t3_dn8 = assign103980_e156069_d_n8;
        locals.var_t3_dn9 = assign103980_e156069_d_n9;
        locals.var_t3_dn10 = assign103980_e156069_d_n10;
        locals.var_t3_dn13 = assign103980_e156069_d_n13;

        let (assign103990_e156086, assign103990_e156086_d_n0, assign103990_e156086_d_n2, assign103990_e156086_d_n4, assign103990_e156086_d_n5, assign103990_e156086_d_n6, assign103990_e156086_d_n7, assign103990_e156086_d_n8, assign103990_e156086_d_n9, assign103990_e156086_d_n10, assign103990_e156086_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2368 == 0.0)) {
        let assign103990_e156083: f64 = (locals.var_uc_rdrbb - 1.0);
        let assign103990_e156084: f64 = (locals.var_t1).powf(assign103990_e156083);
        (assign103990_e156084, if locals.var_uc_rdrbb_dn0 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn0)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn0 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn2 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn2)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn2 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn4 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn4)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn4 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn5 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn5)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn5 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn6 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn6)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn6 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn7 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn7)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn7 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn8 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn8)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn8 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn9 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn9)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn9 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn10 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn10)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn10 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn13 == 0.0 && ((assign103990_e156083) as f64).is_finite() && ((assign103990_e156083) as f64).fract() == 0.0 { if assign103990_e156083 == 0.0 { 0.0 } else { (assign103990_e156083 * ((locals.var_t1).powf(assign103990_e156083 - 1.0) * locals.var_t1_dn13)) } } else { (assign103990_e156084 * ((locals.var_uc_rdrbb_dn13 * (locals.var_t1).ln()) + (assign103990_e156083 * (locals.var_t1_dn13 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign103990_e156086;
        locals.var_t3_dn0 = assign103990_e156086_d_n0;
        locals.var_t3_dn2 = assign103990_e156086_d_n2;
        locals.var_t3_dn4 = assign103990_e156086_d_n4;
        locals.var_t3_dn5 = assign103990_e156086_d_n5;
        locals.var_t3_dn6 = assign103990_e156086_d_n6;
        locals.var_t3_dn7 = assign103990_e156086_d_n7;
        locals.var_t3_dn8 = assign103990_e156086_d_n8;
        locals.var_t3_dn9 = assign103990_e156086_d_n9;
        locals.var_t3_dn10 = assign103990_e156086_d_n10;
        locals.var_t3_dn13 = assign103990_e156086_d_n13;

        let (assign104000_e156095, assign104000_e156095_d_n0, assign104000_e156095_d_n2, assign104000_e156095_d_n4, assign104000_e156095_d_n5, assign104000_e156095_d_n6, assign104000_e156095_d_n7, assign104000_e156095_d_n8, assign104000_e156095_d_n9, assign104000_e156095_d_n10, assign104000_e156095_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104000_e156093: f64 = (locals.var_t1 * locals.var_t3);
        (assign104000_e156093, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104000_e156095;
        locals.var_t2_dn0 = assign104000_e156095_d_n0;
        locals.var_t2_dn2 = assign104000_e156095_d_n2;
        locals.var_t2_dn4 = assign104000_e156095_d_n4;
        locals.var_t2_dn5 = assign104000_e156095_d_n5;
        locals.var_t2_dn6 = assign104000_e156095_d_n6;
        locals.var_t2_dn7 = assign104000_e156095_d_n7;
        locals.var_t2_dn8 = assign104000_e156095_d_n8;
        locals.var_t2_dn9 = assign104000_e156095_d_n9;
        locals.var_t2_dn10 = assign104000_e156095_d_n10;
        locals.var_t2_dn13 = assign104000_e156095_d_n13;

        let (assign104010_e156104, assign104010_e156104_d_n0, assign104010_e156104_d_n2, assign104010_e156104_d_n4, assign104010_e156104_d_n5, assign104010_e156104_d_n6, assign104010_e156104_d_n7, assign104010_e156104_d_n8, assign104010_e156104_d_n9, assign104010_e156104_d_n10, assign104010_e156104_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104010_e156102: f64 = (1.0 + locals.var_t2);
        (assign104010_e156102, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign104010_e156104;
        locals.var_t4_dn0 = assign104010_e156104_d_n0;
        locals.var_t4_dn2 = assign104010_e156104_d_n2;
        locals.var_t4_dn4 = assign104010_e156104_d_n4;
        locals.var_t4_dn5 = assign104010_e156104_d_n5;
        locals.var_t4_dn6 = assign104010_e156104_d_n6;
        locals.var_t4_dn7 = assign104010_e156104_d_n7;
        locals.var_t4_dn8 = assign104010_e156104_d_n8;
        locals.var_t4_dn9 = assign104010_e156104_d_n9;
        locals.var_t4_dn10 = assign104010_e156104_d_n10;
        locals.var_t4_dn13 = assign104010_e156104_d_n13;

        let assign104020_e156108: f64 = (10.0 * 2.220446049250313e-16);
        let assign104020_e156109: f64 = (1.0 - assign104020_e156108);
        let assign104020_e156116: f64 = (10.0 * 2.220446049250313e-16);
        let assign104020_e156117: f64 = (1.0 + assign104020_e156116);
        let assign104020_e156119: f64 = if ((assign104020_e156109 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104020_e156117)) { 1.0 } else { 0.0 };
        locals.var_guard2369 = assign104020_e156119;

        let (assign104030_e156130, assign104030_e156130_d_n0, assign104030_e156130_d_n2, assign104030_e156130_d_n4, assign104030_e156130_d_n5, assign104030_e156130_d_n6, assign104030_e156130_d_n7, assign104030_e156130_d_n8, assign104030_e156130_d_n9, assign104030_e156130_d_n10, assign104030_e156130_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 != 0.0)) {
        let assign104030_e156128: f64 = (1.0 / locals.var_t4);
        (assign104030_e156128, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104030_e156130;
        locals.var_t5_dn0 = assign104030_e156130_d_n0;
        locals.var_t5_dn2 = assign104030_e156130_d_n2;
        locals.var_t5_dn4 = assign104030_e156130_d_n4;
        locals.var_t5_dn5 = assign104030_e156130_d_n5;
        locals.var_t5_dn6 = assign104030_e156130_d_n6;
        locals.var_t5_dn7 = assign104030_e156130_d_n7;
        locals.var_t5_dn8 = assign104030_e156130_d_n8;
        locals.var_t5_dn9 = assign104030_e156130_d_n9;
        locals.var_t5_dn10 = assign104030_e156130_d_n10;
        locals.var_t5_dn13 = assign104030_e156130_d_n13;

        let assign104040_e156134: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156135: f64 = (2.0 - assign104040_e156134);
        let assign104040_e156142: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156143: f64 = (2.0 + assign104040_e156142);
        let assign104040_e156145: f64 = if ((assign104040_e156135 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104040_e156143)) { 1.0 } else { 0.0 };
        locals.var_guard2370 = assign104040_e156145;

        let (assign104050_e156160, assign104050_e156160_d_n0, assign104050_e156160_d_n2, assign104050_e156160_d_n4, assign104050_e156160_d_n5, assign104050_e156160_d_n6, assign104050_e156160_d_n7, assign104050_e156160_d_n8, assign104050_e156160_d_n9, assign104050_e156160_d_n10, assign104050_e156160_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 != 0.0)) {
        let assign104050_e156157: f64 = (locals.var_t4).sqrt();
        let assign104050_e156158: f64 = (1.0 / assign104050_e156157);
        (assign104050_e156158, (-((locals.var_t4_dn0 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn2 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn4 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn5 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn6 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn7 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn8 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn9 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn10 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))), (-((locals.var_t4_dn13 / (2.0 * assign104050_e156157)) / (assign104050_e156157 * assign104050_e156157))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign104050_e156160;
        locals.var_t5_dn0 = assign104050_e156160_d_n0;
        locals.var_t5_dn2 = assign104050_e156160_d_n2;
        locals.var_t5_dn4 = assign104050_e156160_d_n4;
        locals.var_t5_dn5 = assign104050_e156160_d_n5;
        locals.var_t5_dn6 = assign104050_e156160_d_n6;
        locals.var_t5_dn7 = assign104050_e156160_d_n7;
        locals.var_t5_dn8 = assign104050_e156160_d_n8;
        locals.var_t5_dn9 = assign104050_e156160_d_n9;
        locals.var_t5_dn10 = assign104050_e156160_d_n10;
        locals.var_t5_dn13 = assign104050_e156160_d_n13;

        let (assign104060_e156185, assign104060_e156185_d_n0, assign104060_e156185_d_n2, assign104060_e156185_d_n4, assign104060_e156185_d_n5, assign104060_e156185_d_n6, assign104060_e156185_d_n7, assign104060_e156185_d_n8, assign104060_e156185_d_n9, assign104060_e156185_d_n10, assign104060_e156185_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let (assign104060_e156183, assign104060_e156183_d_n0, assign104060_e156183_d_n2, assign104060_e156183_d_n4, assign104060_e156183_d_n5, assign104060_e156183_d_n6, assign104060_e156183_d_n7, assign104060_e156183_d_n8, assign104060_e156183_d_n9, assign104060_e156183_d_n10, assign104060_e156183_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104060_e156177: f64 = (-1.0);
                let assign104060_e156179: f64 = (assign104060_e156177 / locals.var_uc_rdrbb);
                let assign104060_e156181: f64 = (assign104060_e156179 - 1.0);
                let assign104060_e156182: f64 = (locals.var_t4).powf(assign104060_e156181);
                (assign104060_e156182, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn0)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn2)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn4)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn5)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn6)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn7)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn8)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn9)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn10)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign104060_e156177 * locals.var_uc_rdrbb_dn13) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104060_e156181) as f64).is_finite() && ((assign104060_e156181) as f64).fract() == 0.0 { if assign104060_e156181 == 0.0 { 0.0 } else { (assign104060_e156181 * ((locals.var_t4).powf(assign104060_e156181 - 1.0) * locals.var_t4_dn13)) } } else { (assign104060_e156182 * (((-((assign104060_e156177 * locals.var_uc_rdrbb_dn13) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104060_e156181 * (locals.var_t4_dn13 / locals.var_t4)))) },)
            }
        };
        (assign104060_e156183, assign104060_e156183_d_n0, assign104060_e156183_d_n2, assign104060_e156183_d_n4, assign104060_e156183_d_n5, assign104060_e156183_d_n6, assign104060_e156183_d_n7, assign104060_e156183_d_n8, assign104060_e156183_d_n9, assign104060_e156183_d_n10, assign104060_e156183_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign104060_e156185;
        locals.var_t6_dn0 = assign104060_e156185_d_n0;
        locals.var_t6_dn2 = assign104060_e156185_d_n2;
        locals.var_t6_dn4 = assign104060_e156185_d_n4;
        locals.var_t6_dn5 = assign104060_e156185_d_n5;
        locals.var_t6_dn6 = assign104060_e156185_d_n6;
        locals.var_t6_dn7 = assign104060_e156185_d_n7;
        locals.var_t6_dn8 = assign104060_e156185_d_n8;
        locals.var_t6_dn9 = assign104060_e156185_d_n9;
        locals.var_t6_dn10 = assign104060_e156185_d_n10;
        locals.var_t6_dn13 = assign104060_e156185_d_n13;

    }
}
